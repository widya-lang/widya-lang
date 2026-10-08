// ============================================================================
// WIDYABASE TIME-SERIES HYPERTABLE ENGINE (timeseries.rs)
// ============================================================================
// Menyediakan partisi waktu otomatis (Hypertables), kompresi Gorilla
// (XOR float & Delta-of-Delta timestamp), time_bucket downsampling,
// continuous aggregates (OHLCV, Min, Max, Avg), dan lifecycle retention.
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Record individual dari suatu pengukuran time-series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesRecord {
    pub timestamp_ms: u64,
    pub series_key: String,
    pub value_numeric: f64,
    pub tags: HashMap<String, String>,
}

/// Baris OHLCV (Open, High, Low, Close, Volume) untuk data finansial/telemetri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcvBar {
    pub bucket_timestamp_ms: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub sample_count: usize,
}

/// Tipe agregasi untuk kueri time-bucket
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeBucketAgg {
    Avg,
    Sum,
    Min,
    Max,
    Count,
}

/// Hasil dari time-bucket agregasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBucketResult {
    pub bucket_timestamp_ms: u64,
    pub series_key: String,
    pub aggregated_value: f64,
    pub sample_count: usize,
}

// ============================================================================
// GORILLA COMPRESSOR (XOR FLOAT & DELTA-OF-DELTA TIMESTAMP)
// ============================================================================

/// Bit-writer pembantu untuk kompresi Gorilla
pub struct BitWriter {
    bytes: Vec<u8>,
    current_byte: u8,
    bits_in_byte: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            current_byte: 0,
            bits_in_byte: 0,
        }
    }

    pub fn write_bit(&mut self, bit: bool) {
        if bit {
            self.current_byte |= 1 << (7 - self.bits_in_byte);
        }
        self.bits_in_byte += 1;
        if self.bits_in_byte == 8 {
            self.bytes.push(self.current_byte);
            self.current_byte = 0;
            self.bits_in_byte = 0;
        }
    }

    pub fn write_bits(&mut self, value: u64, count: u8) {
        for i in (0..count).rev() {
            let bit = ((value >> i) & 1) != 0;
            self.write_bit(bit);
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.bits_in_byte > 0 {
            self.bytes.push(self.current_byte);
        }
        self.bytes
    }
}

/// Bit-reader pembantu untuk dekompresi Gorilla
pub struct BitReader<'a> {
    bytes: &'a [u8],
    byte_index: usize,
    bits_read: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            byte_index: 0,
            bits_read: 0,
        }
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        if self.byte_index >= self.bytes.len() {
            return None;
        }
        let bit = ((self.bytes[self.byte_index] >> (7 - self.bits_read)) & 1) != 0;
        self.bits_read += 1;
        if self.bits_read == 8 {
            self.bits_read = 0;
            self.byte_index += 1;
        }
        Some(bit)
    }

    pub fn read_bits(&mut self, count: u8) -> Option<u64> {
        let mut value = 0u64;
        for _ in 0..count {
            let bit = self.read_bit()?;
            value = (value << 1) | (bit as u64);
        }
        Some(value)
    }
}

/// Kompresi dan dekompresi data deret waktu berbasis algoritma Facebook Gorilla
pub struct GorillaCodec;

impl GorillaCodec {
    /// Mengompresi array pasangan (timestamp_ms, value_f64)
    pub fn compress(data: &[(u64, f64)]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut writer = BitWriter::new();
        let total_count = data.len() as u32;
        writer.write_bits(total_count as u64, 32);

        // Header timestamp pertama (64-bit)
        let first_t = data[0].0;
        writer.write_bits(first_t, 64);

        // Header float pertama (64-bit IEEE 754)
        let first_v = data[0].1.to_bits();
        writer.write_bits(first_v, 64);

        if data.len() == 1 {
            return writer.finish();
        }

        // Simpan delta pertama (32-bit)
        let mut prev_t = first_t;
        let mut prev_delta = (data[1].0 as i64) - (first_t as i64);
        writer.write_bits(prev_delta.unsigned_abs(), 32);

        let mut prev_v_bits = first_v;
        let mut prev_leading_zeros = 64u8;
        let mut prev_trailing_zeros = 64u8;

        for (t, v) in &data[1..] {
            // 1. Delta-of-delta untuk timestamp
            let cur_delta = (*t as i64) - (prev_t as i64);
            let d_o_d = cur_delta - prev_delta;
            prev_delta = cur_delta;
            prev_t = *t;

            if d_o_d == 0 {
                writer.write_bit(false); // bit 0
            } else if (-63..=64).contains(&d_o_d) {
                writer.write_bits(0b10, 2);
                writer.write_bits((d_o_d + 63) as u64, 7);
            } else if (-255..=256).contains(&d_o_d) {
                writer.write_bits(0b110, 3);
                writer.write_bits((d_o_d + 255) as u64, 9);
            } else if (-2047..=2048).contains(&d_o_d) {
                writer.write_bits(0b1110, 4);
                writer.write_bits((d_o_d + 2047) as u64, 12);
            } else {
                writer.write_bits(0b1111, 4);
                writer.write_bits(d_o_d as u64, 32);
            }

            // 2. XOR float value compression
            let v_bits = v.to_bits();
            let xor = v_bits ^ prev_v_bits;
            prev_v_bits = v_bits;

            if xor == 0 {
                writer.write_bit(false); // value sama persis
            } else {
                writer.write_bit(true);
                let leading = xor.leading_zeros() as u8;
                let trailing = xor.trailing_zeros() as u8;

                if leading >= prev_leading_zeros && trailing >= prev_trailing_zeros {
                    writer.write_bit(false); // gunakan rentang sebelumnya
                    let meaningful_len = 64 - prev_leading_zeros - prev_trailing_zeros;
                    let meaningful_bits = (xor >> prev_trailing_zeros) & ((1 << meaningful_len) - 1);
                    writer.write_bits(meaningful_bits, meaningful_len);
                } else {
                    writer.write_bit(true); // deklarasikan rentang baru
                    prev_leading_zeros = leading.min(31);
                    prev_trailing_zeros = trailing.min(31);
                    let meaningful_len = 64 - prev_leading_zeros - prev_trailing_zeros;

                    writer.write_bits(prev_leading_zeros as u64, 5);
                    writer.write_bits(meaningful_len as u64, 6);
                    let meaningful_bits = (xor >> prev_trailing_zeros) & ((1 << meaningful_len) - 1);
                    writer.write_bits(meaningful_bits, meaningful_len);
                }
            }
        }

        writer.finish()
    }

    /// Mendekompresi byte Gorilla kembali ke array (timestamp_ms, value_f64)
    pub fn decompress(compressed: &[u8]) -> Result<Vec<(u64, f64)>, String> {
        if compressed.is_empty() {
            return Ok(Vec::new());
        }

        let mut reader = BitReader::new(compressed);
        let count = reader.read_bits(32).ok_or("Gagal membaca panjang data")? as usize;
        if count == 0 {
            return Ok(Vec::new());
        }

        let first_t = reader.read_bits(64).ok_or("Gagal membaca timestamp pertama")?;
        let first_v = f64::from_bits(reader.read_bits(64).ok_or("Gagal membaca float pertama")?);

        let mut results = Vec::with_capacity(count);
        results.push((first_t, first_v));

        if count == 1 {
            return Ok(results);
        }

        let first_delta = reader.read_bits(32).ok_or("Gagal membaca delta pertama")? as i64;
        let mut prev_t = first_t;
        let mut prev_delta = first_delta;
        let mut prev_v_bits = first_v.to_bits();
        let mut prev_leading = 0u8;
        let mut prev_meaningful = 64u8;

        for _ in 1..count {
            // 1. Baca timestamp
            let d_o_d = if !reader.read_bit().ok_or("EOF timestamp bit")? {
                0i64
            } else if !reader.read_bit().ok_or("EOF timestamp bit 2")? {
                // Prefix 10
                (reader.read_bits(7).ok_or("EOF d_o_d 7-bit")? as i64) - 63
            } else if !reader.read_bit().ok_or("EOF timestamp bit 3")? {
                // Prefix 110
                (reader.read_bits(9).ok_or("EOF d_o_d 9-bit")? as i64) - 255
            } else if !reader.read_bit().ok_or("EOF timestamp bit 4")? {
                // Prefix 1110
                (reader.read_bits(12).ok_or("EOF d_o_d 12-bit")? as i64) - 2047
            } else {
                // Prefix 1111
                reader.read_bits(32).ok_or("EOF d_o_d 32-bit")? as i32 as i64
            };

            let cur_delta = prev_delta + d_o_d;
            let cur_t = ((prev_t as i64) + cur_delta) as u64;
            prev_delta = cur_delta;
            prev_t = cur_t;

            // 2. Baca float value
            let cur_v = if !reader.read_bit().ok_or("EOF float bit")? {
                f64::from_bits(prev_v_bits)
            } else {
                let change_range = reader.read_bit().ok_or("EOF float range bit")?;
                if change_range {
                    prev_leading = reader.read_bits(5).ok_or("EOF leading zeros")? as u8;
                    prev_meaningful = reader.read_bits(6).ok_or("EOF meaningful len")? as u8;
                }
                let meaningful_bits = reader.read_bits(prev_meaningful).ok_or("EOF meaningful bits")?;
                let trailing = 64 - prev_leading - prev_meaningful;
                let xor = meaningful_bits << trailing;
                let cur_bits = prev_v_bits ^ xor;
                prev_v_bits = cur_bits;
                f64::from_bits(cur_bits)
            };

            results.push((cur_t, cur_v));
        }

        Ok(results)
    }
}

// ============================================================================
// TEMPORAL CHUNKS & HYPERTABLE DATA STRUCTURE
// ============================================================================

/// Partisi waktu (Chunk) yang menampung data rentang waktu tertentu
pub struct TimeChunk {
    pub chunk_id: String,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
    pub records: Vec<TimeSeriesRecord>,
    pub is_compressed: bool,
    pub compressed_data: Vec<u8>,
}

impl TimeChunk {
    pub fn new(id: &str, start_ms: u64, end_ms: u64) -> Self {
        Self {
            chunk_id: id.to_string(),
            start_time_ms: start_ms,
            end_time_ms: end_ms,
            records: Vec::new(),
            is_compressed: false,
            compressed_data: Vec::new(),
        }
    }

    /// Menambahkan record ke dalam chunk
    pub fn insert(&mut self, record: TimeSeriesRecord) {
        if self.is_compressed {
            self.decompress_in_place();
        }
        self.records.push(record);
    }

    /// Mengompresi chunk menggunakan Gorilla compression
    pub fn compress_gorilla(&mut self) -> usize {
        if self.is_compressed || self.records.is_empty() {
            return self.compressed_data.len();
        }

        // Urutkan berdasarkan timestamp sebelum dikompresi
        self.records.sort_by_key(|r| r.timestamp_ms);
        let pairs: Vec<(u64, f64)> = self.records.iter().map(|r| (r.timestamp_ms, r.value_numeric)).collect();
        let bytes = GorillaCodec::compress(&pairs);
        self.compressed_data = bytes;
        self.is_compressed = true;
        self.records.clear(); // Bebaskan RAM dari objek record mentah
        self.compressed_data.len()
    }

    /// Mendekompresi kembali ke RAM jika diperlukan mutasi/kueri detail
    pub fn decompress_in_place(&mut self) {
        if !self.is_compressed {
            return;
        }
        if let Ok(pairs) = GorillaCodec::decompress(&self.compressed_data) {
            self.records = pairs
                .into_iter()
                .map(|(t, v)| TimeSeriesRecord {
                    timestamp_ms: t,
                    series_key: "default".to_string(),
                    value_numeric: v,
                    tags: HashMap::new(),
                })
                .collect();
        }
        self.compressed_data.clear();
        self.is_compressed = false;
    }

    /// Membaca semua pasangan (timestamp, value) baik dalam kondisi terkompresi maupun mentah
    pub fn get_pairs(&self) -> Vec<(u64, f64)> {
        if self.is_compressed {
            GorillaCodec::decompress(&self.compressed_data).unwrap_or_default()
        } else {
            self.records.iter().map(|r| (r.timestamp_ms, r.value_numeric)).collect()
        }
    }
}

/// Hypertable yang secara otomatis mempartisi data time-series ke dalam chunk-chunk temporal
pub struct Hypertable {
    pub table_name: String,
    pub chunk_interval_ms: u64,
    pub retention_period_ms: Option<u64>,
    pub chunks: Arc<RwLock<Vec<Arc<RwLock<TimeChunk>>>>>,
}

impl Hypertable {
    pub fn new(name: &str, chunk_interval_ms: u64, retention_ms: Option<u64>) -> Self {
        Self {
            table_name: name.to_string(),
            chunk_interval_ms: chunk_interval_ms.max(1000), // minimal 1 detik
            retention_period_ms: retention_ms,
            chunks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Menyisipkan record ke partisi chunk temporal yang tepat
    pub fn insert(&self, record: TimeSeriesRecord) -> Result<(), String> {
        let t = record.timestamp_ms;
        let mut chunks = self.chunks.write().unwrap();

        // Cari chunk yang rentang waktunya cocok
        for chunk_arc in chunks.iter() {
            let chunk = chunk_arc.read().unwrap();
            if t >= chunk.start_time_ms && t < chunk.end_time_ms {
                drop(chunk);
                chunk_arc.write().unwrap().insert(record);
                return Ok(());
            }
        }

        // Jika belum ada chunk untuk rentang waktu ini, alokasikan chunk baru
        let start_ms = (t / self.chunk_interval_ms) * self.chunk_interval_ms;
        let end_ms = start_ms + self.chunk_interval_ms;
        let chunk_id = format!("{}_{}_{}", self.table_name, start_ms, end_ms);

        let mut new_chunk = TimeChunk::new(&chunk_id, start_ms, end_ms);
        new_chunk.insert(record);
        chunks.push(Arc::new(RwLock::new(new_chunk)));

        Ok(())
    }

    /// Kueri rentang data berdasarkan waktu
    pub fn query_range(&self, series_key: Option<&str>, start_ms: u64, end_ms: u64) -> Vec<TimeSeriesRecord> {
        let chunks = self.chunks.read().unwrap();
        let mut results = Vec::new();

        for chunk_arc in chunks.iter() {
            let chunk = chunk_arc.read().unwrap();
            // Lewati chunk yang tidak beririsan
            if chunk.end_time_ms <= start_ms || chunk.start_time_ms >= end_ms {
                continue;
            }

            if chunk.is_compressed {
                let pairs = chunk.get_pairs();
                for (t, v) in pairs {
                    if t >= start_ms && t < end_ms {
                        results.push(TimeSeriesRecord {
                            timestamp_ms: t,
                            series_key: series_key.unwrap_or("default").to_string(),
                            value_numeric: v,
                            tags: HashMap::new(),
                        });
                    }
                }
            } else {
                for r in &chunk.records {
                    if r.timestamp_ms >= start_ms && r.timestamp_ms < end_ms {
                        if let Some(key) = series_key {
                            if r.series_key != key {
                                continue;
                            }
                        }
                        results.push(r.clone());
                    }
                }
            }
        }

        results.sort_by_key(|r| r.timestamp_ms);
        results
    }

    /// Melakukan downsampling data deret waktu ke dalam ember waktu (time_bucket)
    pub fn time_bucket(
        &self,
        bucket_size_ms: u64,
        series_key: &str,
        start_ms: u64,
        end_ms: u64,
        agg: TimeBucketAgg,
    ) -> Vec<TimeBucketResult> {
        let raw = self.query_range(Some(series_key), start_ms, end_ms);
        let mut buckets: HashMap<u64, Vec<f64>> = HashMap::new();

        for r in raw {
            let bucket_start = (r.timestamp_ms / bucket_size_ms) * bucket_size_ms;
            buckets.entry(bucket_start).or_default().push(r.value_numeric);
        }

        let mut bucket_keys: Vec<u64> = buckets.keys().cloned().collect();
        bucket_keys.sort();

        let mut results = Vec::new();
        for b_time in bucket_keys {
            let vals = &buckets[&b_time];
            let count = vals.len();
            if count == 0 {
                continue;
            }

            let agg_val = match agg {
                TimeBucketAgg::Sum => vals.iter().sum(),
                TimeBucketAgg::Avg => vals.iter().sum::<f64>() / (count as f64),
                TimeBucketAgg::Min => vals.iter().cloned().fold(f64::INFINITY, f64::min),
                TimeBucketAgg::Max => vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                TimeBucketAgg::Count => count as f64,
            };

            results.push(TimeBucketResult {
                bucket_timestamp_ms: b_time,
                series_key: series_key.to_string(),
                aggregated_value: agg_val,
                sample_count: count,
            });
        }

        results
    }

    /// Menghitung continuous aggregate OHLCV (Open-High-Low-Close-Volume)
    pub fn calculate_ohlcv(
        &self,
        bucket_size_ms: u64,
        series_key: &str,
        start_ms: u64,
        end_ms: u64,
    ) -> Vec<OhlcvBar> {
        let raw = self.query_range(Some(series_key), start_ms, end_ms);
        let mut buckets: HashMap<u64, Vec<f64>> = HashMap::new();

        for r in raw {
            let bucket_start = (r.timestamp_ms / bucket_size_ms) * bucket_size_ms;
            buckets.entry(bucket_start).or_default().push(r.value_numeric);
        }

        let mut bucket_keys: Vec<u64> = buckets.keys().cloned().collect();
        bucket_keys.sort();

        let mut bars = Vec::new();
        for b_time in bucket_keys {
            let vals = &buckets[&b_time];
            if vals.is_empty() {
                continue;
            }

            let open = vals[0];
            let close = vals[vals.len() - 1];
            let high = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let low = vals.iter().cloned().fold(f64::INFINITY, f64::min);
            let volume = vals.iter().sum::<f64>();

            bars.push(OhlcvBar {
                bucket_timestamp_ms: b_time,
                open,
                high,
                low,
                close,
                volume,
                sample_count: vals.len(),
            });
        }

        bars
    }

    /// Mengompresi chunk historis yang umurnya lebih tua dari rentang tertentu
    pub fn compress_historical_chunks(&self, older_than_ms: u64) -> usize {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let threshold = now_ms.saturating_sub(older_than_ms);
        let chunks = self.chunks.read().unwrap();
        let mut total_compressed = 0;

        for chunk_arc in chunks.iter() {
            let mut chunk = chunk_arc.write().unwrap();
            if chunk.end_time_ms < threshold && !chunk.is_compressed {
                chunk.compress_gorilla();
                total_compressed += 1;
            }
        }

        total_compressed
    }

    /// Menjalankan kebijakan retensi data (menghapus partisi waktu yang telah kedaluwarsa)
    pub fn apply_retention_policy(&self) -> usize {
        let retention = match self.retention_period_ms {
            Some(r) => r,
            None => return 0,
        };

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let threshold = now_ms.saturating_sub(retention);
        let mut chunks = self.chunks.write().unwrap();
        let initial_len = chunks.len();

        chunks.retain(|chunk_arc| {
            let chunk = chunk_arc.read().unwrap();
            chunk.end_time_ms >= threshold
        });

        initial_len - chunks.len()
    }

    /// Mendapatkan statistik ukuran dan status chunk
    pub fn get_chunk_stats(&self) -> Vec<(String, u64, u64, usize, bool)> {
        let chunks = self.chunks.read().unwrap();
        chunks
            .iter()
            .map(|c| {
                let chunk = c.read().unwrap();
                (
                    chunk.chunk_id.clone(),
                    chunk.start_time_ms,
                    chunk.end_time_ms,
                    if chunk.is_compressed { chunk.compressed_data.len() } else { chunk.records.len() },
                    chunk.is_compressed,
                )
            })
            .collect()
    }
}

/// Mesin Utama Pengelola Koleksi Hypertable Time-Series di WidyaBase
pub struct TimeSeriesEngine {
    pub hypertables: Arc<RwLock<HashMap<String, Arc<Hypertable>>>>,
}

impl TimeSeriesEngine {
    pub fn new() -> Self {
        Self {
            hypertables: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Membuat hypertable baru dengan interval chunk (ms) dan masa retensi opsional (ms)
    pub fn create_hypertable(
        &self,
        name: &str,
        chunk_interval_ms: u64,
        retention_ms: Option<u64>,
    ) -> Arc<Hypertable> {
        let mut tables = self.hypertables.write().unwrap();
        let table = Arc::new(Hypertable::new(name, chunk_interval_ms, retention_ms));
        tables.insert(name.to_uppercase(), Arc::clone(&table));
        table
    }

    /// Mengambil referensi hypertable yang ada
    pub fn get_hypertable(&self, name: &str) -> Option<Arc<Hypertable>> {
        let tables = self.hypertables.read().unwrap();
        tables.get(&name.to_uppercase()).cloned()
    }
}

impl Default for TimeSeriesEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gorilla_compression_and_decompression() {
        let mut raw_data = Vec::new();
        let base_t = 1700000000000u64;
        let mut val = 120.5;

        for i in 0..100 {
            let t = base_t + (i * 1000); // 1 detik per interval
            val += (i as f64 * 0.1).sin();
            raw_data.push((t, val));
        }

        let compressed = GorillaCodec::compress(&raw_data);
        assert!(!compressed.is_empty());
        assert!(compressed.len() < raw_data.len() * 16); // Efisien vs 16 bytes mentah per titik

        let decompressed = GorillaCodec::decompress(&compressed).expect("Gagal dekompresi");
        assert_eq!(decompressed.len(), raw_data.len());

        for i in 0..raw_data.len() {
            assert_eq!(decompressed[i].0, raw_data[i].0);
            assert!((decompressed[i].1 - raw_data[i].1).abs() < 1e-9);
        }
    }

    #[test]
    fn test_hypertable_insert_and_time_bucket() {
        let engine = TimeSeriesEngine::new();
        let ht = engine.create_hypertable("sensor_gelanggang", 60_000, Some(86400_000)); // chunk 1 menit

        let base_t = 1200_000u64;
        for i in 0..120 {
            let t = base_t + (i * 1000); // data tiap 1 detik selama 2 menit
            ht.insert(TimeSeriesRecord {
                timestamp_ms: t,
                series_key: "gyro_pitch".to_string(),
                value_numeric: 10.0 + (i as f64),
                tags: HashMap::new(),
            }).unwrap();
        }

        // Kueri range
        let records = ht.query_range(Some("gyro_pitch"), base_t, base_t + 120_000);
        assert_eq!(records.len(), 120);

        // Kueri time_bucket (interval 30 detik)
        let buckets = ht.time_bucket(30_000, "gyro_pitch", base_t, base_t + 120_000, TimeBucketAgg::Avg);
        assert_eq!(buckets.len(), 4);
        assert!(buckets[0].aggregated_value < buckets[1].aggregated_value);

        // Kueri OHLCV
        let ohlcv = ht.calculate_ohlcv(60_000, "gyro_pitch", base_t, base_t + 120_000);
        assert_eq!(ohlcv.len(), 2);
        assert_eq!(ohlcv[0].open, 10.0);
        assert_eq!(ohlcv[0].sample_count, 60);

        // Uji kompresi Gorilla pada chunk
        let stats = ht.get_chunk_stats();
        assert_eq!(stats.len(), 2);
        let compressed_count = ht.compress_historical_chunks(0);
        assert_eq!(compressed_count, 2);

        // Data tetap dapat diakses setelah kompresi
        let post_records = ht.query_range(Some("gyro_pitch"), base_t, base_t + 10_000);
        assert_eq!(post_records.len(), 10);
    }
}
