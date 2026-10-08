use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::collections::invoke_closure_or_fn;
use crate::stdlib::common::*;
use crate::value::Value;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_math_advanced(env: &mut Environment) {
    register_fn(env, "DeretWaktu", Some(1), builtin_timeseries_buat);
    register_fn(env, "tambah_titik_waktu", Some(3), builtin_timeseries_tambah);
    register_fn(env, "rata_rata_bergerak", Some(2), builtin_timeseries_sma);
    register_fn(env, "prediksi_eksponensial", None, builtin_timeseries_prediksi);
    register_fn(env, "deteksi_anomali_deret", None, builtin_timeseries_anomali);
    register_fn(env, "FilterBloom", None, builtin_bloom_buat);
    register_fn(env, "bloom_tambah", Some(2), builtin_bloom_tambah);
    register_fn(env, "bloom_mungkin_ada", Some(2), builtin_bloom_cek);
    register_fn(env, "HyperLogLog", None, builtin_hll_buat);
    register_fn(env, "hll_tambah", Some(2), builtin_hll_tambah);
    register_fn(env, "hll_hitung_unik", Some(1), builtin_hll_hitung);
    register_fn(env, "GrafJaringan", None, builtin_graf_buat);
    register_fn(env, "tambah_simpul", None, builtin_graf_tambah_simpul);
    register_fn(env, "tambah_sisi", None, builtin_graf_tambah_sisi);
    register_fn(env, "jalur_terpendek_dijkstra", Some(3), builtin_graf_dijkstra);
    register_fn(env, "hitung_pagerank", None, builtin_graf_pagerank);
    register_fn(env, "inisialisasi_populasi_ga", Some(2), builtin_ga_inisialisasi);
    register_fn(env, "evolusi_generasi_ga", Some(4), builtin_ga_evolusi);
    register_fn(env, "optimasi_ga", Some(5), builtin_ga_optimasi);
    register_fn(env, "PohonTrie", None, builtin_trie_buat);
    register_fn(env, "trie_sisip", None, builtin_trie_sisip);
    register_fn(env, "trie_apakah_ada", Some(2), builtin_trie_ada);
    register_fn(env, "trie_saran_awalan", Some(3), builtin_trie_saran);
    register_fn(env, "HimpunanBit", Some(1), builtin_bitset_buat);
    register_fn(env, "bit_pasang", Some(3), builtin_bitset_pasang);
    register_fn(env, "bit_ambil", Some(2), builtin_bitset_ambil);
    register_fn(env, "bit_hitung_aktif", Some(1), builtin_bitset_hitung_aktif);
    register_fn(env, "bit_operasi_dan", Some(2), builtin_bitset_dan);
    register_fn(env, "bit_operasi_atau", Some(2), builtin_bitset_atau);
    register_fn(env, "bit_operasi_xor", Some(2), builtin_bitset_xor);
    register_fn(env, "kompres_rle", Some(1), builtin_compress_rle);
    register_fn(env, "dekompres_rle", Some(1), builtin_decompress_rle);
    register_fn(env, "kompres_lzw", Some(1), builtin_compress_lzw);
    register_fn(env, "dekompres_lzw", Some(1), builtin_decompress_lzw);
    register_fn(env, "rasio_kompresi", Some(2), builtin_compress_rasio);
    register_fn(env, "CountingBloomFilter", None, builtin_cbloom_buat);
    register_fn(env, "cbloom_tambah", Some(2), builtin_cbloom_tambah);
    register_fn(env, "cbloom_hapus", Some(2), builtin_cbloom_hapus);
    register_fn(env, "cbloom_mungkin_ada", Some(2), builtin_cbloom_ada);
    register_fn(env, "Matriks2D", None, builtin_matrix_buat);
    register_fn(env, "matriks_dari_array", Some(1), builtin_matrix_dari_array);
    register_fn(env, "matriks_kali", Some(2), builtin_matrix_kali);
    register_fn(env, "matriks_transpose", Some(1), builtin_matrix_transpose);
    register_fn(env, "matriks_determinan", Some(1), builtin_matrix_determinan);
    register_fn(env, "matriks_invers", Some(1), builtin_matrix_invers);
    register_fn(env, "matriks_ke_array", Some(1), builtin_matrix_ke_array);
    register_fn(env, "RingBuffer", Some(1), builtin_ringbuffer_buat);
    register_fn(env, "ring_dorong", Some(2), builtin_ringbuffer_dorong);
    register_fn(env, "ring_tarik", Some(1), builtin_ringbuffer_tarik);
    register_fn(env, "ring_lihat", Some(1), builtin_ringbuffer_lihat);
    register_fn(env, "ring_ke_array", Some(1), builtin_ringbuffer_ke_array);
    register_fn(env, "ring_panjang", Some(1), builtin_ringbuffer_panjang);
    register_fn(env, "ring_apakah_penuh", Some(1), builtin_ringbuffer_penuh);
    register_fn(env, "bit_rotasi_kiri", Some(2), builtin_bit_rotasi_kiri);
    register_fn(env, "bit_rotasi_kanan", Some(2), builtin_bit_rotasi_kanan);
    register_fn(env, "bit_hitung_satu", Some(1), builtin_bit_hitung_satu);
    register_fn(env, "bit_hitung_nol_depan", Some(1), builtin_bit_hitung_nol_depan);
    register_fn(env, "bit_hitung_nol_belakang", Some(1), builtin_bit_hitung_nol_belakang);
    register_fn(env, "bit_balik_urutan", Some(1), builtin_bit_balik_urutan);
    register_fn(env, "bit_ekstrak", Some(3), builtin_bit_ekstrak);
    register_fn(env, "HyperLogLog", None, builtin_hll_buat);
    register_fn(env, "hll_tambah", Some(2), builtin_hll_tambah);
    register_fn(env, "hll_estimasi", Some(1), builtin_hll_estimasi);
    register_fn(env, "hll_gabung", Some(2), builtin_hll_gabung);
    register_fn(env, "BigInt", Some(1), builtin_bigint_buat);
    register_fn(env, "bigint_tambah", Some(2), builtin_bigint_tambah);
    register_fn(env, "bigint_kurang", Some(2), builtin_bigint_kurang);
    register_fn(env, "bigint_kali", Some(2), builtin_bigint_kali);
    register_fn(env, "bigint_pangkat", Some(2), builtin_bigint_pangkat);
    register_fn(env, "bigint_bandingkan", Some(2), builtin_bigint_bandingkan);
    register_fn(env, "bigint_ke_string", Some(1), builtin_bigint_ke_string);
    register_fn(env, "statistik_rata_rata", Some(1), builtin_stats_rata_rata);
    register_fn(env, "statistik_median", Some(1), builtin_stats_median);
    register_fn(env, "statistik_modus", Some(1), builtin_stats_modus);
    register_fn(env, "statistik_variansi", Some(1), builtin_stats_variansi);
    register_fn(env, "statistik_standar_deviasi", Some(1), builtin_stats_standar_deviasi);
    register_fn(env, "statistik_kuartil", Some(2), builtin_stats_kuartil);
    register_fn(env, "statistik_zscore", Some(1), builtin_stats_zscore);
    register_fn(env, "RantaiMarkov", None, builtin_markov_buat);
    register_fn(env, "markov_latih_teks", Some(2), builtin_markov_latih_teks);
    register_fn(env, "markov_latih_status", Some(4), builtin_markov_latih_status);
    register_fn(env, "markov_hasilkan_teks", None, builtin_markov_hasilkan_teks);
    register_fn(env, "markov_status_berikutnya", Some(2), builtin_markov_status_berikutnya);
    register_fn(env, "PohonPrefix", Some(0), builtin_trie_prefix_buat);
    register_fn(env, "trie_prefix_sisip", None, builtin_trie_prefix_sisip);
    register_fn(env, "trie_prefix_cari", Some(2), builtin_trie_cari);
    register_fn(env, "trie_prefix_cocok_awalan", None, builtin_trie_cocok_awalan);
    register_fn(env, "trie_prefix_jumlah_kata", Some(1), builtin_trie_jumlah_kata);
    register_fn(env, "trie_cari", Some(2), builtin_trie_cari);
    register_fn(env, "trie_cocok_awalan", None, builtin_trie_cocok_awalan);
    register_fn(env, "trie_jumlah_kata", Some(1), builtin_trie_jumlah_kata);
    register_fn(env, "BitBuffer", Some(0), builtin_bitbuf_buat);
    register_fn(env, "bitbuf_tulis_bit", Some(3), builtin_bitbuf_tulis_bit);
    register_fn(env, "bitbuf_baca_bit", Some(2), builtin_bitbuf_baca_bit);
    register_fn(env, "bitbuf_ke_bytes", Some(1), builtin_bitbuf_ke_bytes);
    register_fn(env, "bitbuf_dari_bytes", Some(1), builtin_bitbuf_dari_bytes);
    register_fn(env, "bitbuf_panjang_bit", Some(1), builtin_bitbuf_panjang_bit);
    register_fn(env, "Titik2D", Some(2), builtin_geo2d_titik);
    register_fn(env, "Garis2D", Some(4), builtin_geo2d_garis);
    register_fn(env, "geo2d_jarak_titik", Some(2), builtin_geo2d_jarak_titik);
    register_fn(env, "geo2d_apakah_berpotongan", Some(2), builtin_geo2d_apakah_berpotongan);
    register_fn(env, "geo2d_titik_potong", Some(2), builtin_geo2d_titik_potong);
    register_fn(env, "geo2d_luas_poligon", Some(1), builtin_geo2d_luas_poligon);
    register_fn(env, "geo2d_convex_hull", Some(1), builtin_geo2d_convex_hull);
    register_fn(env, "murmur3_32", None, builtin_murmur3_32);
    register_fn(env, "murmur3_128", None, builtin_murmur3_128);
    register_fn(env, "murmur3_ke_angka", Some(1), builtin_murmur3_ke_angka);
    register_fn(env, "Pecahan", None, builtin_pecahan_buat);
    register_fn(env, "pecahan_tambah", Some(2), builtin_pecahan_tambah);
    register_fn(env, "pecahan_kurang", Some(2), builtin_pecahan_kurang);
    register_fn(env, "pecahan_kali", Some(2), builtin_pecahan_kali);
    register_fn(env, "pecahan_bagi", Some(2), builtin_pecahan_bagi);
    register_fn(env, "pecahan_ke_desimal", Some(1), builtin_pecahan_ke_desimal);
    register_fn(env, "pecahan_ke_string", Some(1), builtin_pecahan_ke_string);
    register_fn(env, "geohash_enkode", None, builtin_geohash_enkode);
    register_fn(env, "geohash_dekode", Some(1), builtin_geohash_dekode);
    register_fn(env, "geohash_tetangga", Some(1), builtin_geohash_tetangga);
    register_fn(env, "geohash_bounding_box", Some(1), builtin_geohash_bounding_box);
    register_fn(env, "CincinHash", None, builtin_cincinhash_buat);
    register_fn(env, "cincin_tambah_node", Some(2), builtin_cincinhash_tambah_node);
    register_fn(env, "cincin_hapus_node", Some(2), builtin_cincinhash_hapus_node);
    register_fn(env, "cincin_dapatkan_node", Some(2), builtin_cincinhash_dapatkan_node);
    register_fn(env, "cincin_daftar_node", Some(1), builtin_cincinhash_daftar_node);
    register_fn(env, "BarycentricInterpolator", Some(2), builtin_barycentric_buat);
    register_fn(env, "barycentric_evaluasi", Some(2), builtin_barycentric_evaluasi);
    register_fn(env, "barycentric_tambah_titik", Some(3), builtin_barycentric_tambah_titik);
    register_fn(env, "TDigest", None, builtin_tdigest_buat);
    register_fn(env, "tdigest_tambah", Some(2), builtin_tdigest_tambah);
    register_fn(env, "tdigest_persentil", Some(2), builtin_tdigest_persentil);
    register_fn(env, "tdigest_cdf", Some(2), builtin_tdigest_cdf);
    register_fn(env, "tdigest_gabung", Some(2), builtin_tdigest_gabung);
}

// =========================================================================
// WidyaTimeSeries: Time-Series Analytics & Forecasting Engine
// =========================================================================

fn builtin_timeseries_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args[0].to_string_repr();
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DeretWaktu".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("data_points".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_timeseries_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ts = &args[0];
    let time = match args[1] { Value::Number(n) => n, _ => 0.0 };
    let val = match args[2] { Value::Number(n) => n, _ => 0.0 };

    match ts {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("data_points") {
                let mut node = HashMap::new();
                node.insert("waktu".to_string(), Value::Number(time));
                node.insert("nilai".to_string(), Value::Number(val));
                arr.borrow_mut().push(Value::Map(Rc::new(RefCell::new(node))));
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("DeretWaktu tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek DeretWaktu", span)),
    }
}

fn builtin_timeseries_sma(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let points = match &args[0] {
        Value::Array(a) => a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect::<Vec<f64>>(),
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(a)) = map.get("data_points") {
                a.borrow().iter().map(|it| {
                    if let Value::Map(node) = it {
                        match node.borrow().get("nilai") { Some(Value::Number(n)) => *n, _ => 0.0 }
                    } else if let Value::Number(n) = it {
                        *n
                    } else {
                        0.0
                    }
                }).collect()
            } else {
                Vec::new()
            }
        }
        _ => return Err(Galat::runtime("Diharapkan objek DeretWaktu atau daftar angka", span)),
    };

    let window = match args[1] { Value::Number(n) => (n as usize).max(1), _ => 3 };
    let mut sma = Vec::new();

    for i in 0..points.len() {
        if i + 1 >= window {
            let start = i + 1 - window;
            let sum: f64 = points[start..=i].iter().sum();
            sma.push(Value::Number(sum / window as f64));
        } else {
            let sum: f64 = points[0..=i].iter().sum();
            sma.push(Value::Number(sum / (i + 1) as f64));
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(sma))))
}

fn builtin_timeseries_prediksi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let points: Vec<f64> = match &args[0] {
        Value::Array(a) => a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(a)) = map.get("data_points") {
                a.borrow().iter().map(|it| {
                    if let Value::Map(node) = it {
                        match node.borrow().get("nilai") { Some(Value::Number(n)) => *n, _ => 0.0 }
                    } else if let Value::Number(n) = it {
                        *n
                    } else {
                        0.0
                    }
                }).collect()
            } else {
                Vec::new()
            }
        }
        _ => return Err(Galat::runtime("Diharapkan objek DeretWaktu", span)),
    };

    let alpha = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.3);
    let horizon = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(3);

    if points.is_empty() {
        return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    // Single Exponential Smoothing
    let mut smoothed = points[0];
    for &p in points.iter().skip(1) {
        smoothed = alpha * p + (1.0 - alpha) * smoothed;
    }

    let forecasts: Vec<Value> = (0..horizon).map(|_| Value::Number(smoothed)).collect();
    Ok(Value::Array(Rc::new(RefCell::new(forecasts))))
}

fn builtin_timeseries_anomali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let points: Vec<f64> = match &args[0] {
        Value::Array(a) => a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Diharapkan daftar data numerik", span)),
    };

    let z_threshold = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(2.0);
    if points.len() < 2 {
        return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    let mean = points.iter().sum::<f64>() / points.len() as f64;
    let variance = points.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (points.len() - 1) as f64;
    let std_dev = variance.sqrt().max(1e-6);

    let mut anomalies = Vec::new();
    for (idx, &val) in points.iter().enumerate() {
        let z = ((val - mean) / std_dev).abs();
        if z >= z_threshold {
            let mut node = HashMap::new();
            node.insert("indeks".to_string(), Value::Number(idx as f64));
            node.insert("nilai".to_string(), Value::Number(val));
            node.insert("z_score".to_string(), Value::Number(z));
            anomalies.push(Value::Map(Rc::new(RefCell::new(node))));
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(anomalies))))
}

// =========================================================================
// WidyaBloom: Bloom Filter & HyperLogLog Probabilistic Big Data Engine
// =========================================================================

fn builtin_bloom_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let capacity = args.first().and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1000);
    let size = (capacity * 10).max(64);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("FilterBloom".to_string()));
    map.insert("ukuran_bit".to_string(), Value::Number(size as f64));
    map.insert("bitset".to_string(), Value::Array(Rc::new(RefCell::new(vec![Value::Bool(false); size]))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_bloom_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let item = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let size = match map.get("ukuran_bit") { Some(Value::Number(n)) => *n as usize, _ => 64 };
            if let Some(Value::Array(bits_rc)) = map.get("bitset") {
                let mut bits = bits_rc.borrow_mut();
                for seed in [1337u64, 4242u64, 9999u64] {
                    let mut hasher = Sha256::new();
                    hasher.update(format!("{}:{}", seed, item).as_bytes());
                    let h = u64::from_le_bytes(hasher.finalize()[0..8].try_into().unwrap());
                    let idx = (h as usize) % size;
                    bits[idx] = Value::Bool(true);
                }
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("FilterBloom tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek FilterBloom", span)),
    }
}

fn builtin_bloom_cek(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let item = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let size = match map.get("ukuran_bit") { Some(Value::Number(n)) => *n as usize, _ => 64 };
            if let Some(Value::Array(bits_rc)) = map.get("bitset") {
                let bits = bits_rc.borrow();
                for seed in [1337u64, 4242u64, 9999u64] {
                    let mut hasher = Sha256::new();
                    hasher.update(format!("{}:{}", seed, item).as_bytes());
                    let h = u64::from_le_bytes(hasher.finalize()[0..8].try_into().unwrap());
                    let idx = (h as usize) % size;
                    if !bits[idx].is_truthy() {
                        return Ok(Value::Bool(false)); // Pasti tidak ada
                    }
                }
                Ok(Value::Bool(true)) // Mungkin ada
            } else {
                Err(Galat::runtime("FilterBloom tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek FilterBloom", span)),
    }
}

fn builtin_hll_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let p = args.first().and_then(|v| if let Value::Number(n) = v { Some((*n as usize).clamp(4, 16)) } else { None }).unwrap_or(10);
    let m = 1 << p;

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("HyperLogLog".to_string()));
    map.insert("p".to_string(), Value::Number(p as f64));
    map.insert("m".to_string(), Value::Number(m as f64));
    map.insert("registers".to_string(), Value::Array(Rc::new(RefCell::new(vec![Value::Number(0.0); m]))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hll_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let item_str = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m_rc) => {
            let map = m_rc.borrow();
            let p = match map.get("p") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let m = 1 << p;
            let regs_rc = match map.get("registers") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek HyperLogLog tidak valid", span)) };

            let mut hasher = Sha256::new();
            hasher.update(item_str.as_bytes());
            let hash_bytes = hasher.finalize();
            let hash_val = u64::from_le_bytes(hash_bytes[0..8].try_into().unwrap());

            let idx = (hash_val >> (64 - p)) as usize % m;
            let w = (hash_val << p) | (1 << (p - 1));
            let rank = (w.leading_zeros() + 1).min(64 - p as u32 + 1) as f64;

            let mut regs = regs_rc.borrow_mut();
            if let Some(Value::Number(curr)) = regs.get(idx) {
                if rank > *curr {
                    regs[idx] = Value::Number(rank);
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi hll_tambah memerlukan objek HyperLogLog", span)),
    }
}

fn builtin_hll_estimasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m_rc) => {
            let map = m_rc.borrow();
            let p = match map.get("p") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let m = (1 << p) as f64;
            let regs_rc = match map.get("registers") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek HyperLogLog tidak valid", span)) };
            let regs = regs_rc.borrow();

            let alpha = match p {
                4 => 0.673,
                5 => 0.697,
                6 => 0.709,
                _ => 0.7213 / (1.0 + 1.079 / m),
            };

            let mut sum = 0.0;
            let mut zero_count = 0.0;
            for r in regs.iter() {
                let val = match r { Value::Number(n) => *n, _ => 0.0 };
                if val == 0.0 {
                    zero_count += 1.0;
                }
                sum += 2.0f64.powf(-val);
            }

            let mut raw_estimate = alpha * m * m / sum;
            if raw_estimate <= 2.5 * m && zero_count > 0.0 {
                raw_estimate = m * (m / zero_count).ln();
            }

            Ok(Value::Number(raw_estimate.round().max(0.0)))
        }
        _ => Err(Galat::runtime("Fungsi hll_estimasi memerlukan objek HyperLogLog", span)),
    }
}

fn builtin_hll_hitung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    builtin_hll_estimasi(args, span)
}

fn builtin_hll_gabung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (p1, regs1) = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let p = match map.get("p") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let a = match map.get("registers") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("HLL 1 tidak valid", span)) };
            (p, a)
        }
        _ => return Err(Galat::runtime("Argumen 1 bukan HyperLogLog", span)),
    };
    let (p2, regs2) = match &args[1] {
        Value::Map(m) => {
            let map = m.borrow();
            let p = match map.get("p") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let a = match map.get("registers") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("HLL 2 tidak valid", span)) };
            (p, a)
        }
        _ => return Err(Galat::runtime("Argumen 2 bukan HyperLogLog", span)),
    };

    if p1 != p2 {
        return Err(Galat::runtime("Dua objek HyperLogLog harus memiliki presisi p yang sama untuk digabung", span));
    }

    let m = 1 << p1;
    let r1 = regs1.borrow();
    let r2 = regs2.borrow();
    let mut new_regs = Vec::with_capacity(m);

    for i in 0..m {
        let v1 = match r1.get(i) { Some(Value::Number(n)) => *n, _ => 0.0 };
        let v2 = match r2.get(i) { Some(Value::Number(n)) => *n, _ => 0.0 };
        new_regs.push(Value::Number(v1.max(v2)));
    }

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("HyperLogLog".to_string()));
    obj.insert("p".to_string(), Value::Number(p1 as f64));
    obj.insert("m".to_string(), Value::Number(m as f64));
    obj.insert("registers".to_string(), Value::Array(Rc::new(RefCell::new(new_regs))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

// =========================================================================
// WidyaGraphEngine: Graph Theory, Topology & Pathfinding
// =========================================================================

fn builtin_graf_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let berarah = args.first().map(|v| v.is_truthy()).unwrap_or(false);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("GrafJaringan".to_string()));
    map.insert("berarah".to_string(), Value::Bool(berarah));
    map.insert("simpul".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("tetangga".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_graf_tambah_simpul(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let id = args[1].to_string_repr();
    let meta = args.get(2).cloned().unwrap_or(Value::Nil);

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let (Some(Value::Map(nodes_rc)), Some(Value::Map(adj_rc))) = (map.get("simpul"), map.get("tetangga")) {
                nodes_rc.borrow_mut().insert(id.clone(), meta);
                if !adj_rc.borrow().contains_key(&id) {
                    adj_rc.borrow_mut().insert(id, Value::Array(Rc::new(RefCell::new(Vec::new()))));
                }
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("GrafJaringan tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek GrafJaringan", span)),
    }
}

fn builtin_graf_tambah_sisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let u = args[1].to_string_repr();
    let v = args[2].to_string_repr();
    let bobot = args.get(3).and_then(|val| if let Value::Number(n) = val { Some(*n) } else { None }).unwrap_or(1.0);

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let berarah = map.get("berarah").map(|val| val.is_truthy()).unwrap_or(false);
            if let Some(Value::Map(adj_rc)) = map.get("tetangga") {
                let mut adj = adj_rc.borrow_mut();

                // Add u -> v
                let edge_uv = vec![Value::String(v.clone()), Value::Number(bobot)];
                adj.entry(u.clone())
                    .or_insert_with(|| Value::Array(Rc::new(RefCell::new(Vec::new()))));
                if let Some(Value::Array(a)) = adj.get(&u) {
                    a.borrow_mut().push(Value::Array(Rc::new(RefCell::new(edge_uv))));
                }

                // If undirected, add v -> u
                if !berarah {
                    let edge_vu = vec![Value::String(u.clone()), Value::Number(bobot)];
                    adj.entry(v.clone())
                        .or_insert_with(|| Value::Array(Rc::new(RefCell::new(Vec::new()))));
                    if let Some(Value::Array(a)) = adj.get(&v) {
                        a.borrow_mut().push(Value::Array(Rc::new(RefCell::new(edge_vu))));
                    }
                }
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("GrafJaringan tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek GrafJaringan", span)),
    }
}

fn builtin_graf_dijkstra(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let start = args[1].to_string_repr();
    let target = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let adj_rc = match map.get("tetangga") {
                Some(Value::Map(m)) => m.borrow(),
                _ => return Err(Galat::runtime("GrafJaringan tidak valid", span)),
            };

            let mut dist: HashMap<String, f64> = HashMap::new();
            let mut prev: HashMap<String, String> = HashMap::new();
            let mut visited: HashMap<String, bool> = HashMap::new();

            for k in adj_rc.keys() {
                dist.insert(k.clone(), f64::INFINITY);
            }
            dist.insert(start.clone(), 0.0);

            loop {
                // Find unvisited node with min dist
                let mut min_node: Option<String> = None;
                let mut min_d = f64::INFINITY;

                for (node, &d) in dist.iter() {
                    if !visited.get(node).copied().unwrap_or(false) && d < min_d {
                        min_d = d;
                        min_node = Some(node.clone());
                    }
                }

                let current = match min_node {
                    Some(c) => c,
                    None => break,
                };

                if current == target || min_d == f64::INFINITY {
                    break;
                }

                visited.insert(current.clone(), true);

                if let Some(Value::Array(neighbors)) = adj_rc.get(&current) {
                    for edge in neighbors.borrow().iter() {
                        if let Value::Array(e_arr) = edge {
                            let e_items = e_arr.borrow();
                            let neighbor_id = e_items[0].to_string_repr();
                            let weight = match e_items[1] { Value::Number(n) => n, _ => 1.0 };

                            let alt = min_d + weight;
                            let cur_dist = dist.get(&neighbor_id).copied().unwrap_or(f64::INFINITY);
                            if alt < cur_dist {
                                dist.insert(neighbor_id.clone(), alt);
                                prev.insert(neighbor_id, current.clone());
                            }
                        }
                    }
                }
            }

            // Reconstruct path
            let total_cost = dist.get(&target).copied().unwrap_or(f64::INFINITY);
            let mut path = Vec::new();
            let mut curr = target.clone();

            if total_cost != f64::INFINITY {
                path.push(Value::String(curr.clone()));
                while let Some(p) = prev.get(&curr) {
                    path.push(Value::String(p.clone()));
                    curr = p.clone();
                }
                path.reverse();
            }

            let mut res = HashMap::new();
            res.insert("jarak_total".to_string(), Value::Number(if total_cost.is_infinite() { -1.0 } else { total_cost }));
            res.insert("jalur".to_string(), Value::Array(Rc::new(RefCell::new(path))));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Diharapkan objek GrafJaringan", span)),
    }
}

fn builtin_graf_pagerank(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let iterations = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(20);
    let damping = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.85);

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let adj_rc = match map.get("tetangga") {
                Some(Value::Map(m)) => m.borrow(),
                _ => return Err(Galat::runtime("GrafJaringan tidak valid", span)),
            };

            let nodes: Vec<String> = adj_rc.keys().cloned().collect();
            let n = nodes.len();
            if n == 0 {
                return Ok(Value::Map(Rc::new(RefCell::new(HashMap::new()))));
            }

            let mut ranks: HashMap<String, f64> = nodes.iter().map(|k| (k.clone(), 1.0 / n as f64)).collect();

            for _ in 0..iterations {
                let mut next_ranks: HashMap<String, f64> = HashMap::new();
                for node in &nodes {
                    let mut incoming_score = 0.0;
                    for other in &nodes {
                        if let Some(Value::Array(edges)) = adj_rc.get(other) {
                            let out_degree = edges.borrow().len();
                            let is_connected = edges.borrow().iter().any(|e| {
                                if let Value::Array(e_arr) = e {
                                    e_arr.borrow()[0].to_string_repr() == *node
                                } else {
                                    false
                                }
                            });
                            if is_connected && out_degree > 0 {
                                incoming_score += ranks[other] / out_degree as f64;
                            }
                        }
                    }
                    next_ranks.insert(node.clone(), (1.0 - damping) / n as f64 + damping * incoming_score);
                }
                ranks = next_ranks;
            }

            let mut out = HashMap::new();
            for (k, v) in ranks {
                out.insert(k, Value::Number(v));
            }
            Ok(Value::Map(Rc::new(RefCell::new(out))))
        }
        _ => Err(Galat::runtime("Diharapkan objek GrafJaringan", span)),
    }
}

// ==============================================================================
// WidyaGenetic: Evolutionary Computing & Genetic Optimization
// ==============================================================================
fn builtin_ga_inisialisasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pop_size = match args[0] {
        Value::Number(n) => n.max(2.0) as usize,
        _ => return Err(Galat::runtime("Ukuran populasi harus angka", span)),
    };
    let chromosome_len = match args[1] {
        Value::Number(n) => n.max(1.0) as usize,
        _ => return Err(Galat::runtime("Panjang kromosom harus angka", span)),
    };

    let mut rng = rand::thread_rng();
    let mut population = Vec::new();

    for _ in 0..pop_size {
        let genes: Vec<Value> = (0..chromosome_len)
            .map(|_| Value::Number(if rng.r#gen::<bool>() { 1.0 } else { 0.0 }))
            .collect();
        population.push(Value::Array(Rc::new(RefCell::new(genes))));
    }

    let mut ga_obj = HashMap::new();
    ga_obj.insert("_tipe".to_string(), Value::String("WidyaGenetic".to_string()));
    ga_obj.insert("populasi".to_string(), Value::Array(Rc::new(RefCell::new(population))));
    ga_obj.insert("ukuran_populasi".to_string(), Value::Number(pop_size as f64));
    ga_obj.insert("panjang_kromosom".to_string(), Value::Number(chromosome_len as f64));
    ga_obj.insert("generasi".to_string(), Value::Number(0.0));

    Ok(Value::Map(Rc::new(RefCell::new(ga_obj))))
}

fn builtin_ga_evolusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ga_val = &args[0];
    let fitness_fn = &args[1];
    let mutation_rate = match args.get(2) {
        Some(Value::Number(n)) => *n,
        _ => 0.05,
    };
    let crossover_rate = match args.get(3) {
        Some(Value::Number(n)) => *n,
        _ => 0.8,
    };

    match ga_val {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let pop_rc = match map.get("populasi") {
                Some(Value::Array(a)) => a.clone(),
                _ => return Err(Galat::runtime("Populasi GA tidak valid", span)),
            };

            let current_pop: Vec<Value> = pop_rc.borrow().clone();
            let pop_size = current_pop.len();
            if pop_size == 0 {
                return Ok(ga_val.clone());
            }

            // Calculate fitness scores
            let mut scored: Vec<(f64, Vec<f64>)> = Vec::new();
            for individual in &current_pop {
                let genes: Vec<f64> = if let Value::Array(arr) = individual {
                    arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect()
                } else {
                    vec![]
                };

                let fitness_res = invoke_closure_or_fn(fitness_fn, &[individual.clone()], span)?;
                let fit_score = match fitness_res {
                    Value::Number(n) => n,
                    _ => 0.0,
                };
                scored.push((fit_score, genes));
            }

            // Sort descending by fitness
            scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            let best_fitness = scored[0].0;
            let best_genes = scored[0].1.clone();

            let mut rng = rand::thread_rng();
            let mut next_pop = Vec::new();
            // Elitism: retain best individual
            let elite_vals: Vec<Value> = best_genes.iter().map(|&g| Value::Number(g)).collect();
            next_pop.push(Value::Array(Rc::new(RefCell::new(elite_vals))));

            let chromosome_len = scored[0].1.len();

            // Breed until full
            while next_pop.len() < pop_size {
                // Tournament selection (size 3)
                let mut pick = || {
                    let mut best_i = rng.gen_range(0..pop_size);
                    for _ in 0..2 {
                        let cand = rng.gen_range(0..pop_size);
                        if scored[cand].0 > scored[best_i].0 {
                            best_i = cand;
                        }
                    }
                    scored[best_i].1.clone()
                };

                let p1 = pick();
                let p2 = pick();

                // Crossover
                let mut child = if rng.r#gen::<f64>() < crossover_rate && chromosome_len > 1 {
                    let point = rng.gen_range(1..chromosome_len);
                    let mut c = p1[0..point].to_vec();
                    c.extend_from_slice(&p2[point..]);
                    c
                } else {
                    p1
                };

                // Mutation
                for gene in &mut child {
                    if rng.r#gen::<f64>() < mutation_rate {
                        *gene = if *gene == 0.0 { 1.0 } else { 0.0 };
                    }
                }

                let child_vals: Vec<Value> = child.into_iter().map(Value::Number).collect();
                next_pop.push(Value::Array(Rc::new(RefCell::new(child_vals))));
            }

            let gen_count = map.get("generasi").and_then(|v| if let Value::Number(n) = v { Some(*n + 1.0) } else { None }).unwrap_or(1.0);
            map.insert("populasi".to_string(), Value::Array(Rc::new(RefCell::new(next_pop))));
            map.insert("generasi".to_string(), Value::Number(gen_count));
            map.insert("fitness_terbaik".to_string(), Value::Number(best_fitness));
            let best_individual_vals: Vec<Value> = best_genes.into_iter().map(Value::Number).collect();
            map.insert("kromosom_terbaik".to_string(), Value::Array(Rc::new(RefCell::new(best_individual_vals))));

            Ok(ga_val.clone())
        }
        _ => Err(Galat::runtime("Fungsi evolusi_generasi_ga memerlukan objek WidyaGenetic", span)),
    }
}

fn builtin_ga_optimasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pop_size = match args[0] {
        Value::Number(n) => n.max(2.0) as usize,
        _ => return Err(Galat::runtime("Ukuran populasi harus angka", span)),
    };
    let chromosome_len = match args[1] {
        Value::Number(n) => n.max(1.0) as usize,
        _ => return Err(Galat::runtime("Panjang kromosom harus angka", span)),
    };
    let max_generations = match args[2] {
        Value::Number(n) => n.max(1.0) as usize,
        _ => 50,
    };
    let fitness_fn = &args[3];
    let target_fitness = args.get(4).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None });

    let ga_obj = builtin_ga_inisialisasi(&[Value::Number(pop_size as f64), Value::Number(chromosome_len as f64)], span)?;

    for _ in 0..max_generations {
        builtin_ga_evolusi(&[ga_obj.clone(), fitness_fn.clone(), Value::Number(0.05), Value::Number(0.8)], span)?;
        if let Value::Map(m) = &ga_obj {
            let map = m.borrow();
            if let Some(target) = target_fitness {
                if let Some(Value::Number(current_fit)) = map.get("fitness_terbaik") {
                    if *current_fit >= target {
                        break;
                    }
                }
            }
        }
    }

    Ok(ga_obj)
}

// ==============================================================================
// WidyaTrie: Prefix Tree & Autocomplete Dictionary Engine
// ==============================================================================
fn builtin_trie_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut node = HashMap::new();
    node.insert("_tipe".to_string(), Value::String("PohonTrie".to_string()));
    node.insert("anak".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    node.insert("adalah_akhir".to_string(), Value::Bool(false));
    node.insert("bobot".to_string(), Value::Number(0.0));

    Ok(Value::Map(Rc::new(RefCell::new(node))))
}

fn builtin_trie_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let trie_val = &args[0];

    // If it's a PohonPrefix object, route to builtin_trie_prefix_sisip
    if let Value::Map(m) = trie_val {
        let is_prefix = {
            let map = m.borrow();
            map.get("_tipe").map(|v| v.to_string_repr() == "PohonPrefix").unwrap_or(false)
        };
        if is_prefix {
            return builtin_trie_prefix_sisip(args, span);
        }
    }

    let kata = args[1].to_string_repr().to_lowercase();
    let bobot = match args.get(2) { Some(Value::Number(n)) => *n, _ => 1.0 };

    match trie_val {
        Value::Map(m) => {
            let mut curr = m.clone();
            for ch in kata.chars() {
                let ch_str = ch.to_string();
                let next_node = {
                    let map = curr.borrow();
                    let anak_rc = match map.get("anak") {
                        Some(Value::Map(am)) => am.clone(),
                        _ => return Err(Galat::runtime("PohonTrie rusak", span)),
                    };
                    let mut anak_map = anak_rc.borrow_mut();
                    if let Some(Value::Map(existing)) = anak_map.get(&ch_str) {
                        existing.clone()
                    } else {
                        let mut new_n = HashMap::new();
                        new_n.insert("anak".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
                        new_n.insert("adalah_akhir".to_string(), Value::Bool(false));
                        new_n.insert("bobot".to_string(), Value::Number(0.0));
                        let new_rc = Rc::new(RefCell::new(new_n));
                        anak_map.insert(ch_str, Value::Map(new_rc.clone()));
                        new_rc
                    }
                };
                curr = next_node;
            }
            curr.borrow_mut().insert("adalah_akhir".to_string(), Value::Bool(true));
            curr.borrow_mut().insert("bobot".to_string(), Value::Number(bobot));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi trie_sisip memerlukan objek PohonTrie", span)),
    }
}

fn builtin_trie_ada(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let trie_val = &args[0];
    let kata = args[1].to_string_repr().to_lowercase();

    match trie_val {
        Value::Map(m) => {
            let mut curr = m.clone();
            for ch in kata.chars() {
                let ch_str = ch.to_string();
                let next_opt = {
                    let map = curr.borrow();
                    let anak_rc = match map.get("anak") {
                        Some(Value::Map(am)) => am.borrow(),
                        _ => return Err(Galat::runtime("PohonTrie rusak", span)),
                    };
                    anak_rc.get(&ch_str).and_then(|v| if let Value::Map(m) = v { Some(m.clone()) } else { None })
                };
                if let Some(next) = next_opt {
                    curr = next;
                } else {
                    return Ok(Value::Bool(false));
                }
            }
            let is_end = curr.borrow().get("adalah_akhir").and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None }).unwrap_or(false);
            Ok(Value::Bool(is_end))
        }
        _ => Err(Galat::runtime("Fungsi trie_apakah_ada memerlukan objek PohonTrie", span)),
    }
}

fn collect_trie_words(node_rc: &Rc<RefCell<HashMap<String, Value>>>, prefix: &str, out: &mut Vec<(String, f64)>) {
    let map = node_rc.borrow();
    if let Some(Value::Bool(true)) = map.get("adalah_akhir") {
        let weight = match map.get("bobot") { Some(Value::Number(n)) => *n, _ => 1.0 };
        out.push((prefix.to_string(), weight));
    }
    if let Some(Value::Map(anak_rc)) = map.get("anak") {
        for (ch, child_val) in anak_rc.borrow().iter() {
            if let Value::Map(child_rc) = child_val {
                collect_trie_words(child_rc, &format!("{}{}", prefix, ch), out);
            }
        }
    }
}

fn builtin_trie_saran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let trie_val = &args[0];
    let awalan = args[1].to_string_repr().to_lowercase();
    let limit = match args[2] { Value::Number(n) => n.max(1.0) as usize, _ => 5 };

    match trie_val {
        Value::Map(m) => {
            let mut curr = m.clone();
            for ch in awalan.chars() {
                let ch_str = ch.to_string();
                let next_opt = {
                    let map = curr.borrow();
                    let anak_rc = match map.get("anak") {
                        Some(Value::Map(am)) => am.borrow(),
                        _ => return Err(Galat::runtime("PohonTrie rusak", span)),
                    };
                    anak_rc.get(&ch_str).and_then(|v| if let Value::Map(m) = v { Some(m.clone()) } else { None })
                };
                if let Some(next) = next_opt {
                    curr = next;
                } else {
                    return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
                }
            }

            let mut candidates = Vec::new();
            collect_trie_words(&curr, &awalan, &mut candidates);
            // Sort by weight descending
            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            let res: Vec<Value> = candidates.into_iter().take(limit).map(|(w, _)| Value::String(w)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi trie_saran_awalan memerlukan objek PohonTrie", span)),
    }
}

// ==============================================================================
// WidyaBitSet: Compact Bit Array & Bitwise Operations
// ==============================================================================
fn builtin_bitset_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let size = match args.first() { Some(Value::Number(n)) => (*n as usize).max(1), _ => 64 };
    let num_words = (size + 63) / 64;
    let words = vec![0u64; num_words];

    let mut obj = std::collections::HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("HimpunanBit".to_string()));
    obj.insert("ukuran".to_string(), Value::Number(size as f64));
    let word_vals: Vec<Value> = words.into_iter().map(|w| Value::Number(w as f64)).collect();
    obj.insert("kata".to_string(), Value::Array(Rc::new(RefCell::new(word_vals))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_bitset_pasang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bs_val = &args[0];
    let idx = match args.get(1) { Some(Value::Number(n)) => *n as usize, _ => 0 };
    let bit_val = match args.get(2) { Some(v) => v.is_truthy(), _ => true };

    match bs_val {
        Value::Map(m) => {
            let map = m.borrow();
            let size = match map.get("ukuran") { Some(Value::Number(n)) => *n as usize, _ => 64 };
            if idx >= size {
                return Err(Galat::runtime(format!("Indeks bit {} di luar batas ukuran {}", idx, size), span));
            }
            let words_rc = match map.get("kata") { Some(Value::Array(wa)) => wa.clone(), _ => return Err(Galat::runtime("Objek HimpunanBit tidak valid", span)) };

            let word_idx = idx / 64;
            let bit_offset = idx % 64;
            let mut wa = words_rc.borrow_mut();

            let mut word = match wa.get(word_idx) { Some(Value::Number(n)) => *n as u64, _ => 0 };
            if bit_val {
                word |= 1u64 << bit_offset;
            } else {
                word &= !(1u64 << bit_offset);
            }
            wa[word_idx] = Value::Number(word as f64);
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi bit_pasang memerlukan objek HimpunanBit", span)),
    }
}

fn builtin_bitset_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bs_val = &args[0];
    let idx = match args.get(1) { Some(Value::Number(n)) => *n as usize, _ => 0 };

    match bs_val {
        Value::Map(m) => {
            let map = m.borrow();
            let size = match map.get("ukuran") { Some(Value::Number(n)) => *n as usize, _ => 64 };
            if idx >= size {
                return Err(Galat::runtime(format!("Indeks bit {} di luar batas ukuran {}", idx, size), span));
            }
            let words_rc = match map.get("kata") { Some(Value::Array(wa)) => wa.clone(), _ => return Err(Galat::runtime("Objek HimpunanBit tidak valid", span)) };

            let word_idx = idx / 64;
            let bit_offset = idx % 64;
            let wa = words_rc.borrow();
            let word = match wa.get(word_idx) { Some(Value::Number(n)) => *n as u64, _ => 0 };
            let is_set = (word & (1u64 << bit_offset)) != 0;
            Ok(Value::Bool(is_set))
        }
        _ => Err(Galat::runtime("Fungsi bit_ambil memerlukan objek HimpunanBit", span)),
    }
}

fn builtin_bitset_hitung_aktif(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bs_val = &args[0];
    match bs_val {
        Value::Map(m) => {
            let map = m.borrow();
            let words_rc = match map.get("kata") { Some(Value::Array(wa)) => wa.clone(), _ => return Err(Galat::runtime("Objek HimpunanBit tidak valid", span)) };
            let mut count = 0u32;
            for w_val in words_rc.borrow().iter() {
                if let Value::Number(n) = w_val {
                    count += (*n as u64).count_ones();
                }
            }
            Ok(Value::Number(count as f64))
        }
        _ => Err(Galat::runtime("Fungsi bit_hitung_aktif memerlukan objek HimpunanBit", span)),
    }
}

fn builtin_bitset_dan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    bitwise_combine(&args[0], &args[1], |a, b| a & b, span)
}

fn builtin_bitset_atau(args: &[Value], span: &Span) -> Result<Value, Galat> {
    bitwise_combine(&args[0], &args[1], |a, b| a | b, span)
}

fn builtin_bitset_xor(args: &[Value], span: &Span) -> Result<Value, Galat> {
    bitwise_combine(&args[0], &args[1], |a, b| a ^ b, span)
}

fn bitwise_combine<F>(v1: &Value, v2: &Value, op: F, span: &Span) -> Result<Value, Galat>
where
    F: Fn(u64, u64) -> u64,
{
    if let (Value::Map(m1), Value::Map(m2)) = (v1, v2) {
        let map1 = m1.borrow();
        let map2 = m2.borrow();
        let size1 = match map1.get("ukuran") { Some(Value::Number(n)) => *n as usize, _ => 64 };
        let size2 = match map2.get("ukuran") { Some(Value::Number(n)) => *n as usize, _ => 64 };
        let final_size = size1.min(size2);

        let w1_rc = match map1.get("kata") { Some(Value::Array(wa)) => wa.clone(), _ => return Err(Galat::runtime("HimpunanBit 1 tidak valid", span)) };
        let w2_rc = match map2.get("kata") { Some(Value::Array(wa)) => wa.clone(), _ => return Err(Galat::runtime("HimpunanBit 2 tidak valid", span)) };

        let w1 = w1_rc.borrow();
        let w2 = w2_rc.borrow();
        let num_words = (final_size + 63) / 64;

        let mut res_words = Vec::new();
        for i in 0..num_words {
            let n1 = match w1.get(i) { Some(Value::Number(n)) => *n as u64, _ => 0 };
            let n2 = match w2.get(i) { Some(Value::Number(n)) => *n as u64, _ => 0 };
            res_words.push(Value::Number(op(n1, n2) as f64));
        }

        let mut out = std::collections::HashMap::new();
        out.insert("_tipe".to_string(), Value::String("HimpunanBit".to_string()));
        out.insert("ukuran".to_string(), Value::Number(final_size as f64));
        out.insert("kata".to_string(), Value::Array(Rc::new(RefCell::new(res_words))));
        Ok(Value::Map(Rc::new(RefCell::new(out))))
    } else {
        Err(Galat::runtime("Operasi bitwise memerlukan dua objek HimpunanBit", span))
    }
}

// ==============================================================================
// WidyaCompress: In-Memory Compression (RLE & LZW)
// ==============================================================================
fn builtin_compress_rle(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = args[0].to_string_repr();
    if text.is_empty() { return Ok(Value::String(String::new())); }

    let chars: Vec<char> = text.chars().collect();
    let mut compressed = String::new();
    let mut i = 0;

    while i < chars.len() {
        let mut count = 1;
        while i + 1 < chars.len() && chars[i] == chars[i + 1] {
            count += 1;
            i += 1;
        }
        if count > 1 {
            compressed.push_str(&format!("{}{}", count, chars[i]));
        } else {
            compressed.push(chars[i]);
        }
        i += 1;
    }

    Ok(Value::String(compressed))
}

fn builtin_decompress_rle(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = args[0].to_string_repr();
    let mut decompressed = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut count_str = String::new();

    for c in chars {
        if c.is_ascii_digit() {
            count_str.push(c);
        } else {
            let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>().unwrap_or(1) };
            for _ in 0..count {
                decompressed.push(c);
            }
            count_str.clear();
        }
    }

    Ok(Value::String(decompressed))
}

fn builtin_compress_lzw(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let uncompressed = args[0].to_string_repr();
    let mut dictionary = std::collections::HashMap::new();
    for i in 0..256 {
        dictionary.insert((i as u8 as char).to_string(), i as u32);
    }

    let mut w = String::new();
    let mut result = Vec::new();
    let mut dict_size = 256u32;

    for c in uncompressed.chars() {
        let mut wc = w.clone();
        wc.push(c);
        if dictionary.contains_key(&wc) {
            w = wc;
        } else {
            result.push(Value::Number(*dictionary.get(&w).unwrap() as f64));
            dictionary.insert(wc, dict_size);
            dict_size += 1;
            w = c.to_string();
        }
    }

    if !w.is_empty() {
        result.push(Value::Number(*dictionary.get(&w).unwrap() as f64));
    }

    Ok(Value::Array(Rc::new(RefCell::new(result))))
}

fn builtin_decompress_lzw(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let compressed_val = &args[0];
    let compressed: Vec<u32> = match compressed_val {
        Value::Array(arr_rc) => arr_rc.borrow().iter().map(|v| match v { Value::Number(n) => *n as u32, _ => 0 }).collect(),
        _ => return Err(Galat::runtime("Fungsi dekompres_lzw memerlukan array kode numerik", span)),
    };

    if compressed.is_empty() {
        return Ok(Value::String(String::new()));
    }

    let mut dictionary = std::collections::HashMap::new();
    for i in 0..256 {
        dictionary.insert(i as u32, (i as u8 as char).to_string());
    }

    let mut w = dictionary.get(&compressed[0]).cloned().unwrap_or_default();
    let mut result = w.clone();
    let mut dict_size = 256u32;

    for &k in &compressed[1..] {
        let entry = if let Some(s) = dictionary.get(&k) {
            s.clone()
        } else if k == dict_size {
            let mut s = w.clone();
            s.push(w.chars().next().unwrap());
            s
        } else {
            return Err(Galat::runtime("LZW compressed stream rusak", span));
        };

        result.push_str(&entry);

        let mut new_entry = w.clone();
        new_entry.push(entry.chars().next().unwrap());
        dictionary.insert(dict_size, new_entry);
        dict_size += 1;

        w = entry;
    }

    Ok(Value::String(result))
}

fn builtin_compress_rasio(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let original = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.0 };
    let compressed = match args.get(1) { Some(Value::Number(n)) => *n, _ => 0.0 };

    if original <= 0.0 {
        return Ok(Value::Number(0.0));
    }

    let saving = (1.0 - (compressed / original)) * 100.0;
    let rounded = (saving * 10.0).round() / 10.0;
    Ok(Value::Number(rounded))
}

// ==============================================================================
// WidyaCountingBloom: Counting & Dynamic Deletable Bloom Filter
// ==============================================================================
fn get_cbloom_hashes(key: &str, num_hashes: usize, capacity: usize) -> Vec<usize> {
    let mut hashes = Vec::with_capacity(num_hashes);
    use sha2::{Digest, Sha256};
    for i in 0..num_hashes {
        let mut hasher = Sha256::new();
        hasher.update((i as u64).to_le_bytes());
        hasher.update(key.as_bytes());
        let digest = hasher.finalize();
        let val = u64::from_le_bytes(digest[0..8].try_into().unwrap()) as usize;
        hashes.push(val % capacity);
    }
    hashes
}

fn builtin_cbloom_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let capacity = match args.first() { Some(Value::Number(n)) => (*n as usize).max(8), _ => 128 };
    let num_hashes = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(3).max(1);

    let mut obj = std::collections::HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("CountingBloomFilter".to_string()));
    obj.insert("kapasitas".to_string(), Value::Number(capacity as f64));
    obj.insert("jumlah_hash".to_string(), Value::Number(num_hashes as f64));
    let counts = vec![Value::Number(0.0); capacity];
    obj.insert("wadah".to_string(), Value::Array(Rc::new(RefCell::new(counts))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_cbloom_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cb_val = &args[0];
    let key = args[1].to_string_repr();

    match cb_val {
        Value::Map(m) => {
            let map = m.borrow();
            let capacity = match map.get("kapasitas") { Some(Value::Number(n)) => *n as usize, _ => 128 };
            let num_hashes = match map.get("jumlah_hash") { Some(Value::Number(n)) => *n as usize, _ => 3 };
            let counts_rc = match map.get("wadah") { Some(Value::Array(ca)) => ca.clone(), _ => return Err(Galat::runtime("Objek CountingBloomFilter tidak valid", span)) };

            let hashes = get_cbloom_hashes(&key, num_hashes, capacity);
            let mut ca = counts_rc.borrow_mut();
            for idx in hashes {
                let current = match ca.get(idx) { Some(Value::Number(n)) => *n, _ => 0.0 };
                ca[idx] = Value::Number(current + 1.0);
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi cbloom_tambah memerlukan objek CountingBloomFilter", span)),
    }
}

fn builtin_cbloom_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cb_val = &args[0];
    let key = args[1].to_string_repr();

    match cb_val {
        Value::Map(m) => {
            let map = m.borrow();
            let capacity = match map.get("kapasitas") { Some(Value::Number(n)) => *n as usize, _ => 128 };
            let num_hashes = match map.get("jumlah_hash") { Some(Value::Number(n)) => *n as usize, _ => 3 };
            let counts_rc = match map.get("wadah") { Some(Value::Array(ca)) => ca.clone(), _ => return Err(Galat::runtime("Objek CountingBloomFilter tidak valid", span)) };

            let hashes = get_cbloom_hashes(&key, num_hashes, capacity);
            let mut ca = counts_rc.borrow_mut();

            // Only decrement if all hash positions are > 0
            let all_present = hashes.iter().all(|&idx| match ca.get(idx) { Some(Value::Number(n)) => *n > 0.0, _ => false });
            if all_present {
                for idx in hashes {
                    let current = match ca.get(idx) { Some(Value::Number(n)) => *n, _ => 0.0 };
                    ca[idx] = Value::Number((current - 1.0).max(0.0));
                }
                Ok(Value::Bool(true))
            } else {
                Ok(Value::Bool(false))
            }
        }
        _ => Err(Galat::runtime("Fungsi cbloom_hapus memerlukan objek CountingBloomFilter", span)),
    }
}

fn builtin_cbloom_ada(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cb_val = &args[0];
    let key = args[1].to_string_repr();

    match cb_val {
        Value::Map(m) => {
            let map = m.borrow();
            let capacity = match map.get("kapasitas") { Some(Value::Number(n)) => *n as usize, _ => 128 };
            let num_hashes = match map.get("jumlah_hash") { Some(Value::Number(n)) => *n as usize, _ => 3 };
            let counts_rc = match map.get("wadah") { Some(Value::Array(ca)) => ca.clone(), _ => return Err(Galat::runtime("Objek CountingBloomFilter tidak valid", span)) };

            let hashes = get_cbloom_hashes(&key, num_hashes, capacity);
            let ca = counts_rc.borrow();
            let all_present = hashes.iter().all(|&idx| match ca.get(idx) { Some(Value::Number(n)) => *n > 0.0, _ => false });
            Ok(Value::Bool(all_present))
        }
        _ => Err(Galat::runtime("Fungsi cbloom_mungkin_ada memerlukan objek CountingBloomFilter", span)),
    }
}

// ==============================================================================
// WidyaMatrixMath: 2D Matrix Linear Algebra & Transformations
// ==============================================================================
fn matrix_create_val(rows: usize, cols: usize, data: Vec<f64>) -> Value {
    let mut map = std::collections::HashMap::new();
    map.insert("_tipe".to_string(), Value::String("Matriks2D".to_string()));
    map.insert("baris".to_string(), Value::Number(rows as f64));
    map.insert("kolom".to_string(), Value::Number(cols as f64));
    let cell_vals: Vec<Value> = data.into_iter().map(Value::Number).collect();
    map.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(cell_vals))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn matrix_extract(v: &Value) -> Option<(usize, usize, Vec<f64>)> {
    if let Value::Map(m) = v {
        let map = m.borrow();
        if let (Some(Value::Number(r)), Some(Value::Number(c)), Some(Value::Array(da))) = (map.get("baris"), map.get("kolom"), map.get("data")) {
            let data: Vec<f64> = da.borrow().iter().map(|c| match c { Value::Number(n) => *n, _ => 0.0 }).collect();
            return Some((*r as usize, *c as usize, data));
        }
        // Fallback for AI tensor Matriks (where data is 2D array: [[..], [..]])
        if let Some(Value::Array(rows_rc)) = map.get("data") {
            let rows = rows_rc.borrow();
            let num_rows = rows.len();
            if num_rows == 0 {
                return Some((0, 0, Vec::new()));
            }
            let mut num_cols = 0;
            let mut data = Vec::new();
            for (r_idx, r_val) in rows.iter().enumerate() {
                if let Value::Array(cols_rc) = r_val {
                    let cols = cols_rc.borrow();
                    if r_idx == 0 {
                        num_cols = cols.len();
                    }
                    for c_val in cols.iter() {
                        match c_val {
                            Value::Number(n) => data.push(*n),
                            _ => data.push(0.0),
                        }
                    }
                }
            }
            return Some((num_rows, num_cols, data));
        }
    }
    None
}

fn builtin_matrix_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let rows = match args.get(0) { Some(Value::Number(n)) => (*n as usize).max(1), _ => 2 };
    let cols = match args.get(1) { Some(Value::Number(n)) => (*n as usize).max(1), _ => 2 };
    let init_val = match args.get(2) { Some(Value::Number(n)) => *n, _ => 0.0 };
    let data = vec![init_val; rows * cols];
    Ok(matrix_create_val(rows, cols, data))
}

fn builtin_matrix_dari_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let arr_val = &args[0];
    match arr_val {
        Value::Array(rows_rc) => {
            let rows = rows_rc.borrow();
            let num_rows = rows.len();
            if num_rows == 0 {
                return Ok(matrix_create_val(0, 0, Vec::new()));
            }
            let mut num_cols = 0;
            let mut data = Vec::new();

            for (r_idx, r_val) in rows.iter().enumerate() {
                if let Value::Array(cols_rc) = r_val {
                    let cols = cols_rc.borrow();
                    if r_idx == 0 {
                        num_cols = cols.len();
                    }
                    for c_val in cols.iter() {
                        let n = match c_val { Value::Number(num) => *num, _ => 0.0 };
                        data.push(n);
                    }
                }
            }

            Ok(matrix_create_val(num_rows, num_cols, data))
        }
        _ => Err(Galat::runtime("Fungsi matriks_dari_array memerlukan array 2D", span)),
    }
}

fn builtin_matrix_kali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (r1, c1, d1) = matrix_extract(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 bukan Matriks2D valid", span))?;
    let (r2, c2, d2) = matrix_extract(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 bukan Matriks2D valid", span))?;

    if c1 != r2 {
        return Err(Galat::runtime(format!("Dimensi matriks tidak cocok untuk perkalian: {}x{} vs {}x{}", r1, c1, r2, c2), span));
    }

    let mut res = vec![0.0; r1 * c2];
    for i in 0..r1 {
        for j in 0..c2 {
            let mut sum = 0.0;
            for k in 0..c1 {
                sum += d1[i * c1 + k] * d2[k * c2 + j];
            }
            res[i * c2 + j] = sum;
        }
    }

    // If input was legacy AI Matriks (has no 'baris' key or _tipe == "Matriks"), return legacy AI format
    let is_ai_matrix = if let Value::Map(m) = &args[0] {
        let map = m.borrow();
        map.get("_tipe").and_then(|v| if let Value::String(s) = v { Some(s == "Matriks") } else { None }).unwrap_or(false)
            || !map.contains_key("baris")
    } else {
        false
    };

    if is_ai_matrix {
        let mut row_vals = Vec::new();
        for i in 0..r1 {
            let mut row = Vec::new();
            for j in 0..c2 {
                row.push(Value::Number(res[i * c2 + j]));
            }
            row_vals.push(Value::Array(Rc::new(RefCell::new(row))));
        }
        let mut obj = HashMap::new();
        obj.insert("_tipe".to_string(), Value::String("Matriks".to_string()));
        obj.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(row_vals))));
        return Ok(Value::Map(Rc::new(RefCell::new(obj))));
    }

    Ok(matrix_create_val(r1, c2, res))
}

fn builtin_matrix_transpose(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (r, c, d) = matrix_extract(&args[0]).ok_or_else(|| Galat::runtime("Fungsi matriks_transpose memerlukan objek Matriks2D", span))?;
    let mut res = vec![0.0; c * r];
    for i in 0..r {
        for j in 0..c {
            res[j * r + i] = d[i * c + j];
        }
    }
    Ok(matrix_create_val(c, r, res))
}

fn calc_det(n: usize, data: &[f64]) -> f64 {
    if n == 1 {
        return data[0];
    }
    if n == 2 {
        return data[0] * data[3] - data[1] * data[2];
    }
    let mut det = 0.0;
    for c in 0..n {
        let mut sub = Vec::with_capacity((n - 1) * (n - 1));
        for i in 1..n {
            for j in 0..n {
                if j != c {
                    sub.push(data[i * n + j]);
                }
            }
        }
        let sign = if c % 2 == 0 { 1.0 } else { -1.0 };
        det += sign * data[c] * calc_det(n - 1, &sub);
    }
    det
}

fn builtin_matrix_determinan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (r, c, d) = matrix_extract(&args[0]).ok_or_else(|| Galat::runtime("Fungsi matriks_determinan memerlukan objek Matriks2D", span))?;
    if r != c {
        return Err(Galat::runtime("Determinan hanya berlaku untuk matriks persegi (NxN)", span));
    }
    let det = calc_det(r, &d);
    Ok(Value::Number((det * 10000.0).round() / 10000.0))
}

fn builtin_matrix_invers(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (r, c, d) = matrix_extract(&args[0]).ok_or_else(|| Galat::runtime("Fungsi matriks_invers memerlukan objek Matriks2D", span))?;
    if r != c {
        return Err(Galat::runtime("Invers hanya berlaku untuk matriks persegi (NxN)", span));
    }
    let det = calc_det(r, &d);
    if det.abs() < 1e-9 {
        return Err(Galat::runtime("Matriks singular (determinan = 0) tidak memiliki invers", span));
    }

    if r == 2 {
        let inv = vec![
            d[3] / det,
            -d[1] / det,
            -d[2] / det,
            d[0] / det,
        ];
        return Ok(matrix_create_val(2, 2, inv));
    }

    let mut cofactors = vec![0.0; r * r];
    for i in 0..r {
        for j in 0..r {
            let mut sub = Vec::with_capacity((r - 1) * (r - 1));
            for row in 0..r {
                if row == i { continue; }
                for col in 0..r {
                    if col == j { continue; }
                    sub.push(d[row * r + col]);
                }
            }
            let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            cofactors[j * r + i] = sign * calc_det(r - 1, &sub) / det; // Transpose of cofactor matrix
        }
    }

    Ok(matrix_create_val(r, r, cofactors))
}

fn builtin_matrix_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (r, c, d) = matrix_extract(&args[0]).ok_or_else(|| Galat::runtime("Fungsi matriks_ke_array memerlukan objek Matriks2D", span))?;
    let mut rows = Vec::with_capacity(r);
    for i in 0..r {
        let mut row = Vec::with_capacity(c);
        for j in 0..c {
            let rounded = (d[i * c + j] * 1000.0).round() / 1000.0;
            row.push(Value::Number(rounded));
        }
        rows.push(Value::Array(Rc::new(RefCell::new(row))));
    }
    Ok(Value::Array(Rc::new(RefCell::new(rows))))
}

// ==============================================================================
// WidyaRingBuffer: Circular Stream Buffer & Fixed Queue
// ==============================================================================
fn builtin_ringbuffer_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let cap = match args.first() {
        Some(Value::Number(n)) => (*n as usize).max(1),
        _ => 16,
    };
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("RingBuffer".to_string()));
    obj.insert("kapasitas".to_string(), Value::Number(cap as f64));
    obj.insert("wadah".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    obj.insert("kepala".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_ringbuffer_dorong(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let item = args[1].clone();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let cap = match map.get("kapasitas") { Some(Value::Number(n)) => *n as usize, _ => 16 };
            let arr_rc = match map.get("wadah") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek RingBuffer tidak valid", span)) };
            let mut arr = arr_rc.borrow_mut();
            if arr.len() >= cap {
                arr.remove(0); // overwrite / FIFO pop oldest
            }
            arr.push(item);
            Ok(Value::Number(arr.len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi ring_dorong memerlukan objek RingBuffer", span)),
    }
}

fn builtin_ringbuffer_tarik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let arr_rc = match map.get("wadah") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek RingBuffer tidak valid", span)) };
            let mut arr = arr_rc.borrow_mut();
            if arr.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(arr.remove(0))
            }
        }
        _ => Err(Galat::runtime("Fungsi ring_tarik memerlukan objek RingBuffer", span)),
    }
}

fn builtin_ringbuffer_lihat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let arr_rc = match map.get("wadah") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek RingBuffer tidak valid", span)) };
            let arr = arr_rc.borrow();
            if arr.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(arr[0].clone())
            }
        }
        _ => Err(Galat::runtime("Fungsi ring_lihat memerlukan objek RingBuffer", span)),
    }
}

fn builtin_ringbuffer_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let arr_rc = match map.get("wadah") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek RingBuffer tidak valid", span)) };
            let arr = arr_rc.borrow().clone();
            Ok(Value::Array(Rc::new(RefCell::new(arr))))
        }
        _ => Err(Galat::runtime("Fungsi ring_ke_array memerlukan objek RingBuffer", span)),
    }
}

fn builtin_ringbuffer_panjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let arr_rc = match map.get("wadah") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek RingBuffer tidak valid", span)) };
            Ok(Value::Number(arr_rc.borrow().len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi ring_panjang memerlukan objek RingBuffer", span)),
    }
}

fn builtin_ringbuffer_penuh(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let cap = match map.get("kapasitas") { Some(Value::Number(n)) => *n as usize, _ => 16 };
            let arr_rc = match map.get("wadah") { Some(Value::Array(a)) => a.clone(), _ => return Err(Galat::runtime("Objek RingBuffer tidak valid", span)) };
            Ok(Value::Bool(arr_rc.borrow().len() >= cap))
        }
        _ => Err(Galat::runtime("Fungsi ring_apakah_penuh memerlukan objek RingBuffer", span)),
    }
}

// ==============================================================================
// WidyaBitwiseOps: Low-Level Bit Manipulation & Binary Packing
// ==============================================================================
fn builtin_bit_rotasi_kiri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num = match &args[0] { Value::Number(n) => *n as u32, _ => return Err(Galat::runtime("bit_rotasi_kiri memerlukan angka", span)) };
    let shift = match &args[1] { Value::Number(n) => (*n as u32) % 32, _ => 0 };
    Ok(Value::Number(num.rotate_left(shift) as f64))
}

fn builtin_bit_rotasi_kanan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num = match &args[0] { Value::Number(n) => *n as u32, _ => return Err(Galat::runtime("bit_rotasi_kanan memerlukan angka", span)) };
    let shift = match &args[1] { Value::Number(n) => (*n as u32) % 32, _ => 0 };
    Ok(Value::Number(num.rotate_right(shift) as f64))
}

fn builtin_bit_hitung_satu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num = match &args[0] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("bit_hitung_satu memerlukan angka", span)) };
    Ok(Value::Number(num.count_ones() as f64))
}

fn builtin_bit_hitung_nol_depan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num = match &args[0] { Value::Number(n) => *n as u32, _ => return Err(Galat::runtime("bit_hitung_nol_depan memerlukan angka", span)) };
    Ok(Value::Number(num.leading_zeros() as f64))
}

fn builtin_bit_hitung_nol_belakang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num = match &args[0] { Value::Number(n) => *n as u32, _ => return Err(Galat::runtime("bit_hitung_nol_belakang memerlukan angka", span)) };
    Ok(Value::Number(num.trailing_zeros() as f64))
}

fn builtin_bit_balik_urutan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num = match &args[0] { Value::Number(n) => *n as u32, _ => return Err(Galat::runtime("bit_balik_urutan memerlukan angka", span)) };
    Ok(Value::Number(num.reverse_bits() as f64))
}

fn builtin_bit_ekstrak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num = match &args[0] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("bit_ekstrak memerlukan angka", span)) };
    let pos = match &args[1] { Value::Number(n) => (*n as u64).min(63), _ => 0 };
    let len = match &args[2] { Value::Number(n) => (*n as u64).min(64 - pos), _ => 1 };

    let mask = if len >= 64 { u64::MAX } else { (1u64 << len) - 1 };
    let extracted = (num >> pos) & mask;
    Ok(Value::Number(extracted as f64))
}

// ==============================================================================
// WidyaBigInt: Arbitrary-Precision Big Integer Arithmetic
// ==============================================================================
fn parse_bigint_str(s: &str) -> (bool, Vec<u32>) {
    let clean = s.trim();
    let (is_neg, digits_str) = if let Some(rest) = clean.strip_prefix('-') {
        (true, rest)
    } else {
        (false, clean.strip_prefix('+').unwrap_or(clean))
    };

    let digits: Vec<u32> = digits_str.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.is_empty() {
        (false, vec![0])
    } else {
        (is_neg, digits)
    }
}

fn bigint_add_digits(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut res = Vec::new();
    let mut carry = 0;
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let d1 = if i >= 0 { a[i as usize] } else { 0 };
        let d2 = if j >= 0 { b[j as usize] } else { 0 };
        let sum = d1 + d2 + carry;
        res.push(sum % 10);
        carry = sum / 10;
        i -= 1;
        j -= 1;
    }
    res.reverse();
    res
}

fn bigint_cmp_digits(a: &[u32], b: &[u32]) -> i32 {
    let a_trim = trim_leading_zeros(a);
    let b_trim = trim_leading_zeros(b);
    if a_trim.len() != b_trim.len() {
        return if a_trim.len() > b_trim.len() { 1 } else { -1 };
    }
    for (d1, d2) in a_trim.iter().zip(b_trim.iter()) {
        if d1 != d2 {
            return if d1 > d2 { 1 } else { -1 };
        }
    }
    0
}

fn trim_leading_zeros(a: &[u32]) -> &[u32] {
    let mut idx = 0;
    while idx < a.len() - 1 && a[idx] == 0 {
        idx += 1;
    }
    &a[idx..]
}

fn bigint_sub_digits(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut res = Vec::new();
    let mut borrow = 0;
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;

    while i >= 0 {
        let d1 = a[i as usize] as i32 - borrow;
        let d2 = if j >= 0 { b[j as usize] as i32 } else { 0 };
        if d1 < d2 {
            res.push((d1 + 10 - d2) as u32);
            borrow = 1;
        } else {
            res.push((d1 - d2) as u32);
            borrow = 0;
        }
        i -= 1;
        j -= 1;
    }
    res.reverse();
    let trimmed = trim_leading_zeros(&res);
    trimmed.to_vec()
}

fn bigint_mul_digits(a: &[u32], b: &[u32]) -> Vec<u32> {
    if (a.len() == 1 && a[0] == 0) || (b.len() == 1 && b[0] == 0) {
        return vec![0];
    }
    let mut res = vec![0u32; a.len() + b.len()];
    for (i, &d1) in a.iter().rev().enumerate() {
        let mut carry = 0;
        for (j, &d2) in b.iter().rev().enumerate() {
            let sum = res[i + j] + d1 * d2 + carry;
            res[i + j] = sum % 10;
            carry = sum / 10;
        }
        if carry > 0 {
            res[i + b.len()] += carry;
        }
    }
    while res.len() > 1 && *res.last().unwrap() == 0 {
        res.pop();
    }
    res.reverse();
    res
}

fn bigint_format(is_neg: bool, digits: &[u32]) -> String {
    let trimmed = trim_leading_zeros(digits);
    if trimmed.len() == 1 && trimmed[0] == 0 {
        return "0".to_string();
    }
    let s: String = trimmed.iter().map(|d| char::from_digit(*d, 10).unwrap()).collect();
    if is_neg { format!("-{}", s) } else { s }
}

fn make_bigint_val(is_neg: bool, digits: Vec<u32>) -> Value {
    let s = bigint_format(is_neg, &digits);
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("BigInt".to_string()));
    obj.insert("nilai".to_string(), Value::String(s));
    Value::Map(Rc::new(RefCell::new(obj)))
}

fn extract_bigint(v: &Value) -> Option<(bool, Vec<u32>)> {
    match v {
        Value::Map(m) => {
            let map = m.borrow();
            let s = match map.get("nilai") { Some(Value::String(s)) => s.clone(), _ => return None };
            Some(parse_bigint_str(&s))
        }
        Value::String(s) => Some(parse_bigint_str(s)),
        Value::Number(n) => Some(parse_bigint_str(&n.round().to_string())),
        _ => None,
    }
}

fn builtin_bigint_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let (is_neg, digits) = parse_bigint_str(&s);
    Ok(make_bigint_val(is_neg, digits))
}

fn builtin_bigint_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (neg1, d1) = extract_bigint(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 bukan BigInt valid", span))?;
    let (neg2, d2) = extract_bigint(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 bukan BigInt valid", span))?;

    if neg1 == neg2 {
        let res = bigint_add_digits(&d1, &d2);
        Ok(make_bigint_val(neg1, res))
    } else {
        let cmp = bigint_cmp_digits(&d1, &d2);
        if cmp == 0 {
            Ok(make_bigint_val(false, vec![0]))
        } else if cmp > 0 {
            let res = bigint_sub_digits(&d1, &d2);
            Ok(make_bigint_val(neg1, res))
        } else {
            let res = bigint_sub_digits(&d2, &d1);
            Ok(make_bigint_val(neg2, res))
        }
    }
}

fn builtin_bigint_kurang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (neg2, d2) = extract_bigint(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 bukan BigInt valid", span))?;
    let inv_arg2 = make_bigint_val(!neg2, d2);
    builtin_bigint_tambah(&[args[0].clone(), inv_arg2], span)
}

fn builtin_bigint_kali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (neg1, d1) = extract_bigint(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 bukan BigInt valid", span))?;
    let (neg2, d2) = extract_bigint(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 bukan BigInt valid", span))?;

    let res = bigint_mul_digits(&d1, &d2);
    let is_neg = neg1 != neg2;
    Ok(make_bigint_val(is_neg, res))
}

fn builtin_bigint_pangkat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (neg1, d1) = extract_bigint(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 bukan BigInt valid", span))?;
    let exp = match &args[1] { Value::Number(n) => (*n as usize).min(1000), _ => return Err(Galat::runtime("Eksponen harus berupa angka integer", span)) };

    if exp == 0 {
        return Ok(make_bigint_val(false, vec![1]));
    }

    let mut base = d1;
    let mut result = vec![1u32];
    let mut e = exp;

    while e > 0 {
        if e % 2 == 1 {
            result = bigint_mul_digits(&result, &base);
        }
        base = bigint_mul_digits(&base, &base);
        e /= 2;
    }

    let is_neg = neg1 && (exp % 2 == 1);
    Ok(make_bigint_val(is_neg, result))
}

fn builtin_bigint_bandingkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (neg1, d1) = extract_bigint(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 bukan BigInt valid", span))?;
    let (neg2, d2) = extract_bigint(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 bukan BigInt valid", span))?;

    if neg1 && !neg2 {
        return Ok(Value::Number(-1.0));
    }
    if !neg1 && neg2 {
        return Ok(Value::Number(1.0));
    }

    let cmp = bigint_cmp_digits(&d1, &d2);
    let final_cmp = if neg1 { -cmp } else { cmp };
    Ok(Value::Number(final_cmp as f64))
}

fn builtin_bigint_ke_string(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (neg, d) = extract_bigint(&args[0]).ok_or_else(|| Galat::runtime("Fungsi bigint_ke_string memerlukan objek BigInt", span))?;
    Ok(Value::String(bigint_format(neg, &d)))
}

// ==============================================================================
// WidyaStats: Descriptive Statistics & Data Analysis
// ==============================================================================
fn extract_numbers(v: &Value) -> Option<Vec<f64>> {
    match v {
        Value::Array(arr) => {
            let mut res = Vec::new();
            for item in arr.borrow().iter() {
                if let Value::Number(n) = item {
                    res.push(*n);
                } else if let Ok(n) = item.to_string_repr().parse::<f64>() {
                    res.push(n);
                }
            }
            Some(res)
        }
        _ => None,
    }
}

fn builtin_stats_rata_rata(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nums = extract_numbers(&args[0]).ok_or_else(|| Galat::runtime("Fungsi statistik_rata_rata memerlukan array angka", span))?;
    if nums.is_empty() { return Ok(Value::Number(0.0)); }
    let sum: f64 = nums.iter().sum();
    let avg = sum / (nums.len() as f64);
    Ok(Value::Number((avg * 10000.0).round() / 10000.0))
}

fn builtin_stats_median(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut nums = extract_numbers(&args[0]).ok_or_else(|| Galat::runtime("Fungsi statistik_median memerlukan array angka", span))?;
    if nums.is_empty() { return Ok(Value::Number(0.0)); }
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = nums.len();
    let med = if len % 2 == 1 {
        nums[len / 2]
    } else {
        (nums[len / 2 - 1] + nums[len / 2]) / 2.0
    };
    Ok(Value::Number((med * 10000.0).round() / 10000.0))
}

fn builtin_stats_modus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nums = extract_numbers(&args[0]).ok_or_else(|| Galat::runtime("Fungsi statistik_modus memerlukan array angka", span))?;
    if nums.is_empty() { return Ok(Value::Nil); }
    let mut freq = HashMap::new();
    for &n in &nums {
        let key = (n * 1000.0).round() as i64;
        *freq.entry(key).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut mode_key = 0;
    for (&k, &count) in &freq {
        if count > max_count {
            max_count = count;
            mode_key = k;
        }
    }
    Ok(Value::Number(mode_key as f64 / 1000.0))
}

fn builtin_stats_variansi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nums = extract_numbers(&args[0]).ok_or_else(|| Galat::runtime("Fungsi statistik_variansi memerlukan array angka", span))?;
    if nums.len() < 2 { return Ok(Value::Number(0.0)); }
    let mean: f64 = nums.iter().sum::<f64>() / (nums.len() as f64);
    let sq_diff: f64 = nums.iter().map(|&x| (x - mean).powi(2)).sum();
    let variance = sq_diff / (nums.len() - 1) as f64;
    Ok(Value::Number((variance * 10000.0).round() / 10000.0))
}

fn builtin_stats_standar_deviasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let var_val = builtin_stats_variansi(args, span)?;
    if let Value::Number(v) = var_val {
        Ok(Value::Number(((v.sqrt()) * 10000.0).round() / 10000.0))
    } else {
        Ok(Value::Number(0.0))
    }
}

fn builtin_stats_kuartil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut nums = extract_numbers(&args[0]).ok_or_else(|| Galat::runtime("Fungsi statistik_kuartil memerlukan array angka", span))?;
    if nums.is_empty() { return Ok(Value::Number(0.0)); }
    let q = match args.get(1) { Some(Value::Number(n)) => (*n as usize).clamp(1, 3), _ => 2 };
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let len = nums.len();
    let pos = (q as f64) * (len as f64 + 1.0) / 4.0 - 1.0;
    let idx = pos.max(0.0) as usize;
    let val = if idx >= len {
        nums[len - 1]
    } else if idx + 1 < len {
        let frac = pos - (idx as f64);
        nums[idx] + frac * (nums[idx + 1] - nums[idx])
    } else {
        nums[idx]
    };
    Ok(Value::Number((val * 10000.0).round() / 10000.0))
}

fn builtin_stats_zscore(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nums = extract_numbers(&args[0]).ok_or_else(|| Galat::runtime("Fungsi statistik_zscore memerlukan array angka", span))?;
    if nums.is_empty() { return Ok(Value::Array(Rc::new(RefCell::new(Vec::new())))); }
    let mean: f64 = nums.iter().sum::<f64>() / (nums.len() as f64);
    let sq_diff: f64 = nums.iter().map(|&x| (x - mean).powi(2)).sum();
    let std_dev = if nums.len() > 1 { (sq_diff / (nums.len() - 1) as f64).sqrt() } else { 1.0 };

    let z_scores: Vec<Value> = nums.into_iter().map(|x| {
        let z = if std_dev > 0.0 { (x - mean) / std_dev } else { 0.0 };
        Value::Number((z * 1000.0).round() / 1000.0)
    }).collect();

    Ok(Value::Array(Rc::new(RefCell::new(z_scores))))
}

// ==============================================================================
// WidyaMarkov: Markov Chain Generative & State Modeler
// ==============================================================================
fn builtin_markov_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let order = match args.first() { Some(Value::Number(n)) => (*n as usize).clamp(1, 4), _ => 1 };
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("RantaiMarkov".to_string()));
    obj.insert("orde".to_string(), Value::Number(order as f64));
    obj.insert("transisi".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_markov_latih_teks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let text = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let trans_rc = match map.get("transisi") { Some(Value::Map(tm)) => tm.clone(), _ => return Err(Galat::runtime("Objek RantaiMarkov tidak valid", span)) };
            let words: Vec<&str> = text.split_whitespace().collect();
            if words.len() < 2 { return Ok(Value::Bool(false)); }

            let mut trans = trans_rc.borrow_mut();
            for i in 0..words.len() - 1 {
                let current = words[i].to_string();
                let next = words[i + 1].to_string();

                let entry_rc = trans.entry(current).or_insert_with(|| Value::Array(Rc::new(RefCell::new(Vec::new()))));
                if let Value::Array(arr_rc) = entry_rc {
                    arr_rc.borrow_mut().push(Value::String(next));
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi markov_latih_teks memerlukan objek RantaiMarkov", span)),
    }
}

fn builtin_markov_latih_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let from_state = args[1].to_string_repr();
    let to_state = args[2].to_string_repr();
    let weight = match args.get(3) { Some(Value::Number(n)) => (*n as usize).max(1), _ => 1 };

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let trans_rc = match map.get("transisi") { Some(Value::Map(tm)) => tm.clone(), _ => return Err(Galat::runtime("Objek RantaiMarkov tidak valid", span)) };
            let mut trans = trans_rc.borrow_mut();
            let entry_rc = trans.entry(from_state).or_insert_with(|| Value::Array(Rc::new(RefCell::new(Vec::new()))));
            if let Value::Array(arr_rc) = entry_rc {
                let mut arr = arr_rc.borrow_mut();
                for _ in 0..weight {
                    arr.push(Value::String(to_state.clone()));
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi markov_latih_status memerlukan objek RantaiMarkov", span)),
    }
}

fn builtin_markov_status_berikutnya(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let current_state = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let trans_rc = match map.get("transisi") { Some(Value::Map(tm)) => tm.clone(), _ => return Err(Galat::runtime("Objek RantaiMarkov tidak valid", span)) };
            let trans = trans_rc.borrow();
            if let Some(Value::Array(arr_rc)) = trans.get(&current_state) {
                let arr = arr_rc.borrow();
                if arr.is_empty() { return Ok(Value::Nil); }
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0..arr.len());
                Ok(arr[idx].clone())
            } else {
                Ok(Value::Nil)
            }
        }
        _ => Err(Galat::runtime("Fungsi markov_status_berikutnya memerlukan objek RantaiMarkov", span)),
    }
}

fn builtin_markov_hasilkan_teks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let start_word = match args.get(1) { Some(v) => v.to_string_repr(), None => "saya".to_string() };
    let max_length = match args.get(2) { Some(Value::Number(n)) => (*n as usize).clamp(2, 200), _ => 15 };

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let trans_rc = match map.get("transisi") { Some(Value::Map(tm)) => tm.clone(), _ => return Err(Galat::runtime("Objek RantaiMarkov tidak valid", span)) };
            let trans = trans_rc.borrow();

            let mut generated = vec![start_word.clone()];
            let mut curr = start_word;

            use rand::Rng;
            let mut rng = rand::thread_rng();

            for _ in 0..max_length {
                if let Some(Value::Array(arr_rc)) = trans.get(&curr) {
                    let arr = arr_rc.borrow();
                    if arr.is_empty() { break; }
                    let idx = rng.gen_range(0..arr.len());
                    let next_word = arr[idx].to_string_repr();
                    generated.push(next_word.clone());
                    curr = next_word;
                } else {
                    break;
                }
            }
            Ok(Value::String(generated.join(" ")))
        }
        _ => Err(Galat::runtime("Fungsi markov_hasilkan_teks memerlukan objek RantaiMarkov", span)),
    }
}

// ==============================================================================
// WidyaTriePrefix: Compact Radix Prefix Tree
// ==============================================================================
fn builtin_trie_prefix_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("PohonPrefix".to_string()));
    obj.insert("kata".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    obj.insert("total".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_trie_prefix_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let word = args[1].to_string_repr();
    let val = match args.get(2) { Some(v) => v.clone(), None => Value::Bool(true) };

    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let words_rc = match map.get("kata") { Some(Value::Map(km)) => km.clone(), _ => return Err(Galat::runtime("Objek PohonPrefix tidak valid", span)) };
            let mut words = words_rc.borrow_mut();
            let is_new = !words.contains_key(&word);
            words.insert(word, val);
            if is_new {
                let curr_tot = match map.get("total") { Some(Value::Number(n)) => *n, _ => 0.0 };
                map.insert("total".to_string(), Value::Number(curr_tot + 1.0));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi trie_sisip memerlukan objek PohonPrefix", span)),
    }
}

fn builtin_trie_cari(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let word = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let words_rc = match map.get("kata") { Some(Value::Map(km)) => km.clone(), _ => return Err(Galat::runtime("Objek PohonPrefix tidak valid", span)) };
            let words = words_rc.borrow();
            match words.get(&word) {
                Some(v) => Ok(v.clone()),
                None => Ok(Value::Nil),
            }
        }
        _ => Err(Galat::runtime("Fungsi trie_cari memerlukan objek PohonPrefix", span)),
    }
}

fn builtin_trie_cocok_awalan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let prefix = args[1].to_string_repr();
    let limit = match args.get(2) { Some(Value::Number(n)) => (*n as usize).max(1), _ => 20 };

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let words_rc = match map.get("kata") { Some(Value::Map(km)) => km.clone(), _ => return Err(Galat::runtime("Objek PohonPrefix tidak valid", span)) };
            let words = words_rc.borrow();

            let mut matches: Vec<String> = words.keys().filter(|k| k.starts_with(&prefix)).cloned().collect();
            matches.sort();
            matches.truncate(limit);

            let res_vals: Vec<Value> = matches.into_iter().map(Value::String).collect();
            Ok(Value::Array(Rc::new(RefCell::new(res_vals))))
        }
        _ => Err(Galat::runtime("Fungsi trie_cocok_awalan memerlukan objek PohonPrefix", span)),
    }
}

fn builtin_trie_jumlah_kata(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let total = match map.get("total") { Some(Value::Number(n)) => *n, _ => 0.0 };
            Ok(Value::Number(total))
        }
        _ => Err(Galat::runtime("Fungsi trie_jumlah_kata memerlukan objek PohonPrefix", span)),
    }
}

// ==============================================================================
// WidyaBitBuffer: Arbitrary Bit-Level Stream Reader & Writer
// ==============================================================================
fn builtin_bitbuf_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("BitBuffer".to_string()));
    obj.insert("bits".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    obj.insert("pos_baca".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_bitbuf_tulis_bit(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let val = match &args[1] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("Nilai bit harus angka", span)) };
    let num_bits = match &args[2] { Value::Number(n) => (*n as usize).clamp(1, 64), _ => 1 };

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let bits_rc = match map.get("bits") { Some(Value::Array(ba)) => ba.clone(), _ => return Err(Galat::runtime("Objek BitBuffer tidak valid", span)) };
            let mut bits = bits_rc.borrow_mut();

            for i in (0..num_bits).rev() {
                let bit = (val >> i) & 1;
                bits.push(Value::Number(bit as f64));
            }
            Ok(Value::Number(bits.len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi bitbuf_tulis_bit memerlukan objek BitBuffer", span)),
    }
}

fn builtin_bitbuf_baca_bit(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let num_bits = match &args[1] { Value::Number(n) => (*n as usize).clamp(1, 64), _ => 1 };

    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let bits_rc = match map.get("bits") { Some(Value::Array(ba)) => ba.clone(), _ => return Err(Galat::runtime("Objek BitBuffer tidak valid", span)) };
            let pos = match map.get("pos_baca") { Some(Value::Number(n)) => *n as usize, _ => 0 };
            let bits = bits_rc.borrow();

            if pos + num_bits > bits.len() {
                return Ok(Value::Nil); // EOF
            }

            let mut result = 0u64;
            for i in 0..num_bits {
                let bit_val = match bits[pos + i] { Value::Number(n) => n as u64, _ => 0 };
                result = (result << 1) | (bit_val & 1);
            }

            map.insert("pos_baca".to_string(), Value::Number((pos + num_bits) as f64));
            Ok(Value::Number(result as f64))
        }
        _ => Err(Galat::runtime("Fungsi bitbuf_baca_bit memerlukan objek BitBuffer", span)),
    }
}

fn builtin_bitbuf_ke_bytes(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let bits_rc = match map.get("bits") { Some(Value::Array(ba)) => ba.clone(), _ => return Err(Galat::runtime("Objek BitBuffer tidak valid", span)) };
            let bits = bits_rc.borrow();

            let mut bytes = Vec::new();
            let mut curr_byte = 0u8;
            let mut count = 0;

            for b in bits.iter() {
                let bit = match b { Value::Number(n) => *n as u8 & 1, _ => 0 };
                curr_byte = (curr_byte << 1) | bit;
                count += 1;
                if count == 8 {
                    bytes.push(Value::Number(curr_byte as f64));
                    curr_byte = 0;
                    count = 0;
                }
            }
            if count > 0 {
                curr_byte <<= 8 - count;
                bytes.push(Value::Number(curr_byte as f64));
            }
            Ok(Value::Array(Rc::new(RefCell::new(bytes))))
        }
        _ => Err(Galat::runtime("Fungsi bitbuf_ke_bytes memerlukan objek BitBuffer", span)),
    }
}

fn builtin_bitbuf_dari_bytes(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            let mut bits = Vec::new();
            for item in items.iter() {
                let byte_val = match item { Value::Number(n) => *n as u8, _ => 0 };
                for i in (0..8).rev() {
                    let bit = (byte_val >> i) & 1;
                    bits.push(Value::Number(bit as f64));
                }
            }
            let mut obj = HashMap::new();
            obj.insert("_tipe".to_string(), Value::String("BitBuffer".to_string()));
            obj.insert("bits".to_string(), Value::Array(Rc::new(RefCell::new(bits))));
            obj.insert("pos_baca".to_string(), Value::Number(0.0));
            Ok(Value::Map(Rc::new(RefCell::new(obj))))
        }
        _ => Err(Galat::runtime("Fungsi bitbuf_dari_bytes memerlukan array angka bytes", span)),
    }
}

fn builtin_bitbuf_panjang_bit(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let bits_rc = match map.get("bits") { Some(Value::Array(ba)) => ba.clone(), _ => return Err(Galat::runtime("Objek BitBuffer tidak valid", span)) };
            Ok(Value::Number(bits_rc.borrow().len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi bitbuf_panjang_bit memerlukan objek BitBuffer", span)),
    }
}

// ==========================================
// 📐 WidyaGeometry2D (Computational Geometry 2D)
// ==========================================

fn get_point_xy(val: &Value) -> Option<(f64, f64)> {
    if let Value::Map(map) = val {
        let m = map.borrow();
        let x = match m.get("x") {
            Some(Value::Number(n)) => *n,
            _ => return None,
        };
        let y = match m.get("y") {
            Some(Value::Number(n)) => *n,
            _ => return None,
        };
        return Some((x, y));
    }
    None
}

fn builtin_geo2d_titik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Koordinat X harus berupa angka", span)),
    };
    let y = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Koordinat Y harus berupa angka", span)),
    };
    let mut map = std::collections::HashMap::new();
    map.insert("tipe".to_string(), Value::String("Titik2D".to_string()));
    map.insert("x".to_string(), Value::Number(x));
    map.insert("y".to_string(), Value::Number(y));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geo2d_garis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x1 = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("x1 harus berupa angka", span)),
    };
    let y1 = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("y1 harus berupa angka", span)),
    };
    let x2 = match &args[2] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("x2 harus berupa angka", span)),
    };
    let y2 = match &args[3] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("y2 harus berupa angka", span)),
    };

    let mut map = std::collections::HashMap::new();
    map.insert("tipe".to_string(), Value::String("Garis2D".to_string()));
    map.insert("x1".to_string(), Value::Number(x1));
    map.insert("y1".to_string(), Value::Number(y1));
    map.insert("x2".to_string(), Value::Number(x2));
    map.insert("y2".to_string(), Value::Number(y2));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geo2d_jarak_titik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let p1 = get_point_xy(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa Titik2D", span))?;
    let p2 = get_point_xy(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 harus berupa Titik2D", span))?;
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    let d = (dx * dx + dy * dy).sqrt();
    Ok(Value::Number(d))
}

fn ccw(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> f64 {
    (p2.0 - p1.0) * (p3.1 - p1.1) - (p2.1 - p1.1) * (p3.0 - p1.0)
}

fn on_segment(p: (f64, f64), q: (f64, f64), r: (f64, f64)) -> bool {
    q.0 <= p.0.max(r.0) && q.0 >= p.0.min(r.0) && q.1 <= p.1.max(r.1) && q.1 >= p.1.min(r.1)
}

fn lines_intersect(p1: (f64, f64), q1: (f64, f64), p2: (f64, f64), q2: (f64, f64)) -> bool {
    let d1 = ccw(p2, q2, p1);
    let d2 = ccw(p2, q2, q1);
    let d3 = ccw(p1, q1, p2);
    let d4 = ccw(p1, q1, q2);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0)) &&
       ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0)) {
        return true;
    }

    if d1.abs() < 1e-9 && on_segment(p2, p1, q2) { return true; }
    if d2.abs() < 1e-9 && on_segment(p2, q1, q2) { return true; }
    if d3.abs() < 1e-9 && on_segment(p1, p2, q1) { return true; }
    if d4.abs() < 1e-9 && on_segment(p1, q2, q1) { return true; }

    false
}

fn get_line_endpoints(val: &Value) -> Option<((f64, f64), (f64, f64))> {
    if let Value::Map(map) = val {
        let m = map.borrow();
        let x1 = match m.get("x1") { Some(Value::Number(n)) => *n, _ => return None };
        let y1 = match m.get("y1") { Some(Value::Number(n)) => *n, _ => return None };
        let x2 = match m.get("x2") { Some(Value::Number(n)) => *n, _ => return None };
        let y2 = match m.get("y2") { Some(Value::Number(n)) => *n, _ => return None };
        return Some(((x1, y1), (x2, y2)));
    }
    None
}

fn builtin_geo2d_apakah_berpotongan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (p1, q1) = get_line_endpoints(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa Garis2D", span))?;
    let (p2, q2) = get_line_endpoints(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 harus berupa Garis2D", span))?;
    Ok(Value::Bool(lines_intersect(p1, q1, p2, q2)))
}

fn builtin_geo2d_titik_potong(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (p1, q1) = get_line_endpoints(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa Garis2D", span))?;
    let (p2, q2) = get_line_endpoints(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 harus berupa Garis2D", span))?;

    let a1 = q1.1 - p1.1;
    let b1 = p1.0 - q1.0;
    let c1 = a1 * p1.0 + b1 * p1.1;

    let a2 = q2.1 - p2.1;
    let b2 = p2.0 - q2.0;
    let c2 = a2 * p2.0 + b2 * p2.1;

    let det = a1 * b2 - a2 * b1;
    if det.abs() < 1e-9 {
        return Ok(Value::Nil);
    }

    let x = (b2 * c1 - b1 * c2) / det;
    let y = (a1 * c2 - a2 * c1) / det;

    let mut map = std::collections::HashMap::new();
    map.insert("tipe".to_string(), Value::String("Titik2D".to_string()));
    map.insert("x".to_string(), Value::Number(x));
    map.insert("y".to_string(), Value::Number(y));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geo2d_luas_poligon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let points = match &args[0] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa array daftar Titik2D", span)),
    };
    if points.len() < 3 {
        return Ok(Value::Number(0.0));
    }
    let mut coords = Vec::new();
    for p in &points {
        if let Some(c) = get_point_xy(p) {
            coords.push(c);
        } else {
            return Err(Galat::runtime("Setiap elemen poligon harus berupa Titik2D", span));
        }
    }

    let mut area = 0.0;
    let n = coords.len();
    for i in 0..n {
        let j = (i + 1) % n;
        area += coords[i].0 * coords[j].1;
        area -= coords[j].0 * coords[i].1;
    }
    area = (area.abs()) / 2.0;
    Ok(Value::Number(area))
}

fn builtin_geo2d_convex_hull(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let points = match &args[0] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa array daftar Titik2D", span)),
    };
    let mut pts = Vec::new();
    for p in &points {
        if let Some(c) = get_point_xy(p) {
            pts.push(c);
        } else {
            return Err(Galat::runtime("Setiap elemen harus berupa Titik2D", span));
        }
    }

    if pts.len() <= 2 {
        return Ok(Value::Array(Rc::new(RefCell::new(points))));
    }

    // Andrew's Monotone Chain
    pts.sort_by(|a, b| {
        if (a.0 - b.0).abs() < 1e-9 {
            a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
        }
    });
    pts.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9);

    let mut lower = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 && ccw(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }

    let mut upper = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 && ccw(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);

    let res: Vec<Value> = lower.into_iter().map(|(x, y)| {
        let mut map = std::collections::HashMap::new();
        map.insert("tipe".to_string(), Value::String("Titik2D".to_string()));
        map.insert("x".to_string(), Value::Number(x));
        map.insert("y".to_string(), Value::Number(y));
        Value::Map(Rc::new(RefCell::new(map)))
    }).collect();

    Ok(Value::Array(Rc::new(RefCell::new(res))))
}

// ==========================================
// ⚡ WidyaMurmurHash (MurmurHash3 32 & 128 bit)
// ==========================================

pub fn murmur3_32_raw(data: &[u8], seed: u32) -> u32 {
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    let mut h1: u32 = seed;

    let nblocks = data.len() / 4;
    for i in 0..nblocks {
        let offset = i * 4;
        let mut k1 = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);

        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let tail = &data[nblocks * 4..];
    let mut k1: u32 = 0;
    if tail.len() >= 3 {
        k1 ^= (tail[2] as u32) << 16;
    }
    if tail.len() >= 2 {
        k1 ^= (tail[1] as u32) << 8;
    }
    if tail.len() >= 1 {
        k1 ^= tail[0] as u32;
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    h1 ^= data.len() as u32;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;
    h1
}

fn murmur3_128_raw(data: &[u8], seed: u32) -> (u64, u64) {
    let c1: u64 = 0x87c37b91114253d5;
    let c2: u64 = 0x4cf5ad432745937f;
    let mut h1: u64 = seed as u64;
    let mut h2: u64 = seed as u64;

    let nblocks = data.len() / 16;
    for i in 0..nblocks {
        let offset = i * 16;
        let mut k1 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        let mut k2 = u64::from_le_bytes(data[offset + 8..offset + 16].try_into().unwrap());

        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
        h1 = h1.rotate_left(27);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(5).wrapping_add(0x52dce729);

        k2 = k2.wrapping_mul(c2);
        k2 = k2.rotate_left(33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
        h2 = h2.rotate_left(31);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(5).wrapping_add(0x38495ab5);
    }

    let tail = &data[nblocks * 16..];
    let mut k1: u64 = 0;
    let mut k2: u64 = 0;

    let len = tail.len();
    if len > 8 {
        for i in (8..len).rev() {
            k2 = (k2 << 8) | (tail[i] as u64);
        }
        k2 = k2.wrapping_mul(c2);
        k2 = k2.rotate_left(33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
    }
    if len > 0 {
        let lim = len.min(8);
        for i in (0..lim).rev() {
            k1 = (k1 << 8) | (tail[i] as u64);
        }
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    h1 ^= data.len() as u64;
    h2 ^= data.len() as u64;

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    // fmix64
    let fmix64 = |mut k: u64| -> u64 {
        k ^= k >> 33;
        k = k.wrapping_mul(0xff51afd7ed558ccd);
        k ^= k >> 33;
        k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
        k ^= k >> 33;
        k
    };

    h1 = fmix64(h1);
    h2 = fmix64(h2);

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    (h1, h2)
}

fn builtin_murmur3_32(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Ok(Value::String("00000000".to_string()));
    }
    let input = args[0].to_string_repr();
    let seed = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => *n as u32,
            _ => 0,
        }
    } else {
        0
    };
    let hash = murmur3_32_raw(input.as_bytes(), seed);
    Ok(Value::String(format!("{:08x}", hash)))
}

fn builtin_murmur3_128(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Ok(Value::String("00000000000000000000000000000000".to_string()));
    }
    let input = args[0].to_string_repr();
    let seed = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => *n as u32,
            _ => 0,
        }
    } else {
        0
    };
    let (h1, h2) = murmur3_128_raw(input.as_bytes(), seed);
    Ok(Value::String(format!("{:016x}{:016x}", h1, h2)))
}

fn builtin_murmur3_ke_angka(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let hex_str = match &args[0] {
        Value::String(s) => s.trim().to_string(),
        _ => return Err(Galat::runtime("Argumen harus berupa string hex hash murmur3", span)),
    };
    if hex_str.len() <= 8 {
        let val = u32::from_str_radix(&hex_str, 16).unwrap_or(0);
        Ok(Value::Number(val as f64))
    } else {
        // take first 8 bytes / 13 hex chars for safe float
        let sub = if hex_str.len() > 13 { &hex_str[..13] } else { &hex_str };
        let val = u64::from_str_radix(sub, 16).unwrap_or(0);
        Ok(Value::Number(val as f64))
    }
}

// ==========================================
// ➗ WidyaBigRat (Arbitrary-Precision Rational Numbers)
// ==========================================

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn make_fraction(mut num: i64, mut den: i64, span: &Span) -> Result<Value, Galat> {
    if den == 0 {
        return Err(Galat::runtime("Penyebut pecahan tidak boleh nol (Division by Zero)", span));
    }
    if den < 0 {
        num = -num;
        den = -den;
    }
    let g = gcd(num, den);
    num /= g;
    den /= g;

    let mut map = std::collections::HashMap::new();
    map.insert("tipe".to_string(), Value::String("Pecahan".to_string()));
    map.insert("pembilang".to_string(), Value::Number(num as f64));
    map.insert("penyebut".to_string(), Value::Number(den as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn get_fraction_num_den(val: &Value) -> Option<(i64, i64)> {
    if let Value::Map(map) = val {
        let m = map.borrow();
        let num = match m.get("pembilang") {
            Some(Value::Number(n)) => *n as i64,
            _ => return None,
        };
        let den = match m.get("penyebut") {
            Some(Value::Number(n)) => *n as i64,
            _ => return None,
        };
        return Some((num, den));
    }
    None
}

fn builtin_pecahan_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return make_fraction(0, 1, span);
    }
    if args.len() == 1 {
        match &args[0] {
            Value::Number(n) => {
                let s = format!("{}", n);
                if let Some(pos) = s.find('.') {
                    let dec_places = s.len() - pos - 1;
                    let den = 10i64.pow(dec_places as u32);
                    let num = (n * den as f64).round() as i64;
                    return make_fraction(num, den, span);
                } else {
                    return make_fraction(*n as i64, 1, span);
                }
            }
            Value::String(s) => {
                if s.contains('/') {
                    let parts: Vec<&str> = s.split('/').collect();
                    if parts.len() == 2 {
                        let num: i64 = parts[0].trim().parse().map_err(|_| Galat::runtime("Pembilang pecahan tidak valid", span))?;
                        let den: i64 = parts[1].trim().parse().map_err(|_| Galat::runtime("Penyebut pecahan tidak valid", span))?;
                        return make_fraction(num, den, span);
                    }
                }
                let n: f64 = s.trim().parse().map_err(|_| Galat::runtime("String pecahan tidak valid", span))?;
                return builtin_pecahan_buat(&[Value::Number(n)], span);
            }
            _ => return Err(Galat::runtime("Argumen tidak valid untuk membuat Pecahan", span)),
        }
    }
    let num = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => return Err(Galat::runtime("Pembilang harus angka", span)),
    };
    let den = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => return Err(Galat::runtime("Penyebut harus angka", span)),
    };
    make_fraction(num, den, span)
}

fn builtin_pecahan_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (n1, d1) = get_fraction_num_den(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus Pecahan", span))?;
    let (n2, d2) = get_fraction_num_den(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 harus Pecahan", span))?;
    let num = n1 * d2 + n2 * d1;
    let den = d1 * d2;
    make_fraction(num, den, span)
}

fn builtin_pecahan_kurang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (n1, d1) = get_fraction_num_den(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus Pecahan", span))?;
    let (n2, d2) = get_fraction_num_den(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 harus Pecahan", span))?;
    let num = n1 * d2 - n2 * d1;
    let den = d1 * d2;
    make_fraction(num, den, span)
}

fn builtin_pecahan_kali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (n1, d1) = get_fraction_num_den(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus Pecahan", span))?;
    let (n2, d2) = get_fraction_num_den(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 harus Pecahan", span))?;
    let num = n1 * n2;
    let den = d1 * d2;
    make_fraction(num, den, span)
}

fn builtin_pecahan_bagi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (n1, d1) = get_fraction_num_den(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus Pecahan", span))?;
    let (n2, d2) = get_fraction_num_den(&args[1]).ok_or_else(|| Galat::runtime("Argumen 2 harus Pecahan", span))?;
    let num = n1 * d2;
    let den = d1 * n2;
    make_fraction(num, den, span)
}

fn builtin_pecahan_ke_desimal(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (num, den) = get_fraction_num_den(&args[0]).ok_or_else(|| Galat::runtime("Argumen harus berupa Pecahan", span))?;
    Ok(Value::Number(num as f64 / den as f64))
}

fn builtin_pecahan_ke_string(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (num, den) = get_fraction_num_den(&args[0]).ok_or_else(|| Galat::runtime("Argumen harus berupa Pecahan", span))?;
    if den == 1 {
        Ok(Value::String(format!("{}", num)))
    } else {
        Ok(Value::String(format!("{}/{}", num, den)))
    }
}

// ==========================================
// 🌍 WidyaGeoHash (Spatial Geohashing)
// ==========================================

const BASE32_GEO: &[u8] = b"0123456789bcdefghjkmnpqrstuvwxyz";

fn geohash_encode_raw(mut lat: f64, mut lon: f64, precision: usize) -> String {
    let mut lat_range = (-90.0, 90.0);
    let mut lon_range = (-180.0, 180.0);
    lat = lat.clamp(-90.0, 90.0);
    lon = lon.clamp(-180.0, 180.0);

    let mut is_even = true;
    let mut bit = 0;
    let mut ch_idx = 0;
    let mut geohash = String::new();

    while geohash.len() < precision {
        if is_even {
            let mid = (lon_range.0 + lon_range.1) / 2.0;
            if lon >= mid {
                ch_idx = (ch_idx << 1) | 1;
                lon_range.0 = mid;
            } else {
                ch_idx = ch_idx << 1;
                lon_range.1 = mid;
            }
        } else {
            let mid = (lat_range.0 + lat_range.1) / 2.0;
            if lat >= mid {
                ch_idx = (ch_idx << 1) | 1;
                lat_range.0 = mid;
            } else {
                ch_idx = ch_idx << 1;
                lat_range.1 = mid;
            }
        }
        is_even = !is_even;
        bit += 1;

        if bit == 5 {
            geohash.push(BASE32_GEO[ch_idx] as char);
            bit = 0;
            ch_idx = 0;
        }
    }

    geohash
}

fn geohash_decode_bbox(hash: &str) -> ((f64, f64), (f64, f64)) {
    let mut lat_range = (-90.0, 90.0);
    let mut lon_range = (-180.0, 180.0);
    let mut is_even = true;

    for c in hash.chars() {
        let cd = BASE32_GEO.iter().position(|&b| b == c as u8).unwrap_or(0);
        for mask in [16, 8, 4, 2, 1] {
            if is_even {
                let mid = (lon_range.0 + lon_range.1) / 2.0;
                if (cd & mask) != 0 {
                    lon_range.0 = mid;
                } else {
                    lon_range.1 = mid;
                }
            } else {
                let mid = (lat_range.0 + lat_range.1) / 2.0;
                if (cd & mask) != 0 {
                    lat_range.0 = mid;
                } else {
                    lat_range.1 = mid;
                }
            }
            is_even = !is_even;
        }
    }

    (lat_range, lon_range)
}

pub fn builtin_geohash_enkode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("geohash_enkode memerlukan argumen: lintang, bujur, [presisi]", span));
    }
    let lat = match &args[0] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Lintang (Latitude) harus angka", span)) };
    let lon = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Bujur (Longitude) harus angka", span)) };
    let prec = if args.len() > 2 {
        match &args[2] { Value::Number(n) => (*n as usize).clamp(1, 12), _ => 9 }
    } else {
        9
    };

    Ok(Value::String(geohash_encode_raw(lat, lon, prec)))
}

pub fn builtin_geohash_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let hash = args[0].to_string_repr().to_lowercase();
    if hash.is_empty() {
        return Err(Galat::runtime("String Geohash tidak boleh kosong", span));
    }
    let (lat_range, lon_range) = geohash_decode_bbox(&hash);
    let lat = (lat_range.0 + lat_range.1) / 2.0;
    let lon = (lon_range.0 + lon_range.1) / 2.0;
    let lat_err = (lat_range.1 - lat_range.0) / 2.0;
    let lon_err = (lon_range.1 - lon_range.0) / 2.0;

    let mut map = HashMap::new();
    map.insert("lintang".to_string(), Value::Number(lat));
    map.insert("bujur".to_string(), Value::Number(lon));
    map.insert("galat_lintang".to_string(), Value::Number(lat_err));
    map.insert("galat_bujur".to_string(), Value::Number(lon_err));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geohash_bounding_box(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let hash = args[0].to_string_repr().to_lowercase();
    let (lat_range, lon_range) = geohash_decode_bbox(&hash);

    let mut map = HashMap::new();
    map.insert("min_lintang".to_string(), Value::Number(lat_range.0));
    map.insert("max_lintang".to_string(), Value::Number(lat_range.1));
    map.insert("min_bujur".to_string(), Value::Number(lon_range.0));
    map.insert("max_bujur".to_string(), Value::Number(lon_range.1));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geohash_tetangga(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let hash = args[0].to_string_repr().to_lowercase();
    let (lat_range, lon_range) = geohash_decode_bbox(&hash);
    let lat = (lat_range.0 + lat_range.1) / 2.0;
    let lon = (lon_range.0 + lon_range.1) / 2.0;
    let d_lat = lat_range.1 - lat_range.0;
    let d_lon = lon_range.1 - lon_range.0;
    let len = hash.len();

    let directions = [
        ("utara", lat + d_lat, lon),
        ("timur_laut", lat + d_lat, lon + d_lon),
        ("timur", lat, lon + d_lon),
        ("tenggara", lat - d_lat, lon + d_lon),
        ("selatan", lat - d_lat, lon),
        ("barat_daya", lat - d_lat, lon - d_lon),
        ("barat", lat, lon - d_lon),
        ("barat_laut", lat + d_lat, lon - d_lon),
    ];

    let mut map = HashMap::new();
    for (dir, n_lat, n_lon) in directions {
        map.insert(dir.to_string(), Value::String(geohash_encode_raw(n_lat, n_lon, len)));
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ==========================================
// 🔄 WidyaConsistentHash (Distributed Hash Ring)
// ==========================================

fn builtin_cincinhash_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let vnodes = if let Some(Value::Number(n)) = args.first() {
        (*n as usize).max(1)
    } else {
        100
    };

    let mut ring = HashMap::new();
    ring.insert("_tipe".to_string(), Value::String("CincinHash".to_string()));
    ring.insert("vnodes".to_string(), Value::Number(vnodes as f64));
    ring.insert("nodes".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    ring.insert("ring".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(ring))))
}

fn builtin_cincinhash_tambah_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ring_val = &args[0];
    let node_id = args[1].to_string_repr();

    match ring_val {
        Value::Map(m) => {
            let map = m.borrow();
            let vnodes = match map.get("vnodes") { Some(Value::Number(n)) => *n as usize, _ => 100 };
            let nodes_rc = match map.get("nodes") { Some(Value::Array(na)) => na.clone(), _ => return Err(Galat::runtime("CincinHash tidak valid", span)) };
            let ring_rc = match map.get("ring") { Some(Value::Array(ra)) => ra.clone(), _ => return Err(Galat::runtime("CincinHash tidak valid", span)) };

            // Add node to list if not already
            let mut nodes = nodes_rc.borrow_mut();
            if !nodes.iter().any(|v| v.to_string_repr() == node_id) {
                nodes.push(Value::String(node_id.clone()));
            }

            // Populate virtual nodes
            let mut ring = ring_rc.borrow_mut();
            for i in 0..vnodes {
                let vnode_key = format!("{}#{}", node_id, i);
                let hash_val = murmur3_32_raw(vnode_key.as_bytes(), 0);
                let mut pair = HashMap::new();
                pair.insert("hash".to_string(), Value::Number(hash_val as f64));
                pair.insert("node".to_string(), Value::String(node_id.clone()));
                ring.push(Value::Map(Rc::new(RefCell::new(pair))));
            }

            // Sort ring by hash
            ring.sort_by(|a, b| {
                let h1 = if let Value::Map(m1) = a { match m1.borrow().get("hash") { Some(Value::Number(n)) => *n as u64, _ => 0 } } else { 0 };
                let h2 = if let Value::Map(m2) = b { match m2.borrow().get("hash") { Some(Value::Number(n)) => *n as u64, _ => 0 } } else { 0 };
                h1.cmp(&h2)
            });

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi cincin_tambah_node memerlukan objek CincinHash", span)),
    }
}

fn builtin_cincinhash_hapus_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ring_val = &args[0];
    let node_id = args[1].to_string_repr();

    match ring_val {
        Value::Map(m) => {
            let map = m.borrow();
            let nodes_rc = match map.get("nodes") { Some(Value::Array(na)) => na.clone(), _ => return Err(Galat::runtime("CincinHash tidak valid", span)) };
            let ring_rc = match map.get("ring") { Some(Value::Array(ra)) => ra.clone(), _ => return Err(Galat::runtime("CincinHash tidak valid", span)) };

            nodes_rc.borrow_mut().retain(|v| v.to_string_repr() != node_id);
            ring_rc.borrow_mut().retain(|v| {
                if let Value::Map(m) = v {
                    m.borrow().get("node").map(|n| n.to_string_repr() != node_id).unwrap_or(true)
                } else {
                    true
                }
            });
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi cincin_hapus_node memerlukan objek CincinHash", span)),
    }
}

fn builtin_cincinhash_dapatkan_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ring_val = &args[0];
    let key = args[1].to_string_repr();

    match ring_val {
        Value::Map(m) => {
            let map = m.borrow();
            let ring_rc = match map.get("ring") { Some(Value::Array(ra)) => ra.clone(), _ => return Err(Galat::runtime("CincinHash tidak valid", span)) };
            let ring = ring_rc.borrow();
            if ring.is_empty() {
                return Ok(Value::Nil);
            }

            let key_hash = murmur3_32_raw(key.as_bytes(), 0) as f64;

            // Binary search / lower bound
            for item in ring.iter() {
                if let Value::Map(m) = item {
                    if let Some(Value::Number(h)) = m.borrow().get("hash") {
                        if *h >= key_hash {
                            return Ok(m.borrow().get("node").cloned().unwrap_or(Value::Nil));
                        }
                    }
                }
            }

            // Wrap around to first node
            if let Some(Value::Map(first)) = ring.first() {
                return Ok(first.borrow().get("node").cloned().unwrap_or(Value::Nil));
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi cincin_dapatkan_node memerlukan objek CincinHash", span)),
    }
}

fn builtin_cincinhash_daftar_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ring_val = &args[0];
    match ring_val {
        Value::Map(m) => {
            let map = m.borrow();
            let nodes_rc = match map.get("nodes") { Some(Value::Array(na)) => na.clone(), _ => return Err(Galat::runtime("CincinHash tidak valid", span)) };
            Ok(Value::Array(nodes_rc))
        }
        _ => Err(Galat::runtime("Fungsi cincin_daftar_node memerlukan objek CincinHash", span)),
    }
}

// =========================================================================
// WIDYA_BARYCENTRIC_INTERPOLATION (STABLE RATIONAL/POLYNOMIAL INTERPOLATION)
// =========================================================================

#[derive(Clone, Debug)]
struct BarycentricEngine {
    xs: Vec<f64>,
    ys: Vec<f64>,
    weights: Vec<f64>,
}

impl BarycentricEngine {
    fn new(xs: Vec<f64>, ys: Vec<f64>) -> Self {
        let n = xs.len();
        let mut weights = vec![1.0; n];
        for j in 0..n {
            let mut w = 1.0;
            for k in 0..n {
                if j != k {
                    w *= xs[j] - xs[k];
                }
            }
            weights[j] = 1.0 / w;
        }
        Self { xs, ys, weights }
    }

    fn evaluate(&self, x: f64) -> f64 {
        let n = self.xs.len();
        for i in 0..n {
            if (x - self.xs[i]).abs() < 1e-12 {
                return self.ys[i];
            }
        }

        let mut num = 0.0;
        let mut den = 0.0;
        for i in 0..n {
            let diff = x - self.xs[i];
            let term = self.weights[i] / diff;
            num += term * self.ys[i];
            den += term;
        }

        if den.abs() < 1e-15 {
            0.0
        } else {
            num / den
        }
    }
}

fn builtin_barycentric_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let xs = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect::<Vec<f64>>(),
        _ => return Err(Galat::runtime("Argumen pertama xs harus array angka", span)),
    };
    let ys = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect::<Vec<f64>>(),
        _ => return Err(Galat::runtime("Argumen kedua ys harus array angka", span)),
    };

    if xs.len() != ys.len() || xs.is_empty() {
        return Err(Galat::runtime("Array xs dan ys harus berukuran sama dan tidak kosong", span));
    }

    let engine = BarycentricEngine::new(xs, ys);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaBarycentricInterpolator".to_string()));
    let xs_val: Vec<Value> = engine.xs.into_iter().map(Value::Number).collect();
    let ys_val: Vec<Value> = engine.ys.into_iter().map(Value::Number).collect();
    let w_val: Vec<Value> = engine.weights.into_iter().map(Value::Number).collect();
    map.insert("xs".to_string(), Value::Array(Rc::new(RefCell::new(xs_val))));
    map.insert("ys".to_string(), Value::Array(Rc::new(RefCell::new(ys_val))));
    map.insert("weights".to_string(), Value::Array(Rc::new(RefCell::new(w_val))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_barycentric_evaluasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("x evaluasi harus angka", span)) };
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let xs = match map.get("xs") {
                Some(Value::Array(a)) => a.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect::<Vec<f64>>(),
                _ => Vec::new(),
            };
            let ys = match map.get("ys") {
                Some(Value::Array(a)) => a.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect::<Vec<f64>>(),
                _ => Vec::new(),
            };
            let weights = match map.get("weights") {
                Some(Value::Array(a)) => a.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect::<Vec<f64>>(),
                _ => Vec::new(),
            };

            let engine = BarycentricEngine { xs, ys, weights };
            Ok(Value::Number(engine.evaluate(x)))
        }
        _ => Err(Galat::runtime("Fungsi 'barycentric_evaluasi' memerlukan instance BarycentricInterpolator", span)),
    }
}

fn builtin_barycentric_tambah_titik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let new_x = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("x baru harus angka", span)) };
    let new_y = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("y baru harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let mut xs = match map.get("xs") {
                Some(Value::Array(a)) => a.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect::<Vec<f64>>(),
                _ => Vec::new(),
            };
            let mut ys = match map.get("ys") {
                Some(Value::Array(a)) => a.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect::<Vec<f64>>(),
                _ => Vec::new(),
            };

            xs.push(new_x);
            ys.push(new_y);

            let engine = BarycentricEngine::new(xs, ys);

            let xs_val: Vec<Value> = engine.xs.into_iter().map(Value::Number).collect();
            let ys_val: Vec<Value> = engine.ys.into_iter().map(Value::Number).collect();
            let w_val: Vec<Value> = engine.weights.into_iter().map(Value::Number).collect();
            map.insert("xs".to_string(), Value::Array(Rc::new(RefCell::new(xs_val))));
            map.insert("ys".to_string(), Value::Array(Rc::new(RefCell::new(ys_val))));
            map.insert("weights".to_string(), Value::Array(Rc::new(RefCell::new(w_val))));

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'barycentric_tambah_titik' memerlukan instance BarycentricInterpolator", span)),
    }
}

// =========================================================================
// WIDYA_TDIGEST (STREAMING QUANTILES & ACCURATE PERCENTILE SKETCH)
// =========================================================================

#[derive(Clone, Debug)]
struct Centroid {
    mean: f64,
    weight: f64,
}

#[derive(Clone, Debug)]
struct TDigestSketch {
    centroids: Vec<Centroid>,
    max_centroids: usize,
    total_weight: f64,
}

impl TDigestSketch {
    fn new(max_centroids: usize) -> Self {
        Self {
            centroids: Vec::with_capacity(max_centroids),
            max_centroids,
            total_weight: 0.0,
        }
    }

    fn add(&mut self, val: f64, weight: f64) {
        if self.centroids.is_empty() {
            self.centroids.push(Centroid { mean: val, weight });
            self.total_weight += weight;
            return;
        }

        // Find closest centroid
        let mut min_diff = f64::MAX;
        let mut best_idx = 0;
        for (i, c) in self.centroids.iter().enumerate() {
            let diff = (c.mean - val).abs();
            if diff < min_diff {
                min_diff = diff;
                best_idx = i;
            }
        }

        if self.centroids.len() < self.max_centroids {
            self.centroids.push(Centroid { mean: val, weight });
            self.centroids.sort_by(|a, b| a.mean.partial_cmp(&b.mean).unwrap_or(std::cmp::Ordering::Equal));
        } else {
            let c = &mut self.centroids[best_idx];
            c.mean = (c.mean * c.weight + val * weight) / (c.weight + weight);
            c.weight += weight;
        }
        self.total_weight += weight;
    }

    fn quantile(&self, q: f64) -> f64 {
        if self.centroids.is_empty() {
            return 0.0;
        }
        if q <= 0.0 {
            return self.centroids.first().unwrap().mean;
        }
        if q >= 1.0 {
            return self.centroids.last().unwrap().mean;
        }

        let target_weight = q * self.total_weight;
        let mut cum_weight = 0.0;

        for (i, c) in self.centroids.iter().enumerate() {
            let prev_cum = cum_weight;
            cum_weight += c.weight;
            if cum_weight >= target_weight {
                if i == 0 {
                    return c.mean;
                }
                let prev = &self.centroids[i - 1];
                let frac = (target_weight - prev_cum) / (c.weight).max(1e-12);
                return prev.mean + frac * (c.mean - prev.mean);
            }
        }
        self.centroids.last().unwrap().mean
    }

    fn cdf(&self, val: f64) -> f64 {
        if self.centroids.is_empty() || self.total_weight == 0.0 {
            return 0.0;
        }
        let mut cum_weight = 0.0;
        for c in &self.centroids {
            if c.mean <= val {
                cum_weight += c.weight;
            } else {
                break;
            }
        }
        (cum_weight / self.total_weight).clamp(0.0, 1.0)
    }

    fn merge(&mut self, other: &TDigestSketch) {
        for c in &other.centroids {
            self.add(c.mean, c.weight);
        }
    }
}

fn tdigest_to_value(td: &TDigestSketch) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaTDigest".to_string()));
    map.insert("max_centroids".to_string(), Value::Number(td.max_centroids as f64));
    map.insert("total_weight".to_string(), Value::Number(td.total_weight));

    let c_arr: Vec<Value> = td.centroids.iter().map(|c| {
        let mut cm = HashMap::new();
        cm.insert("mean".to_string(), Value::Number(c.mean));
        cm.insert("weight".to_string(), Value::Number(c.weight));
        Value::Map(Rc::new(RefCell::new(cm)))
    }).collect();
    map.insert("centroids".to_string(), Value::Array(Rc::new(RefCell::new(c_arr))));

    Value::Map(Rc::new(RefCell::new(map)))
}

fn value_to_tdigest(val: &Value) -> Option<TDigestSketch> {
    match val {
        Value::Map(m) => {
            let map = m.borrow();
            let max_centroids = match map.get("max_centroids") { Some(Value::Number(n)) => *n as usize, _ => 100 };
            let total_weight = match map.get("total_weight") { Some(Value::Number(n)) => *n, _ => 0.0 };

            let mut centroids = Vec::new();
            if let Some(Value::Array(arr)) = map.get("centroids") {
                for item in arr.borrow().iter() {
                    if let Value::Map(cm) = item {
                        let cmap = cm.borrow();
                        let mean = match cmap.get("mean") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let weight = match cmap.get("weight") { Some(Value::Number(n)) => *n, _ => 1.0 };
                        centroids.push(Centroid { mean, weight });
                    }
                }
            }
            Some(TDigestSketch { centroids, max_centroids, total_weight })
        }
        _ => None,
    }
}

fn builtin_tdigest_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let max_centroids = if !args.is_empty() {
        match &args[0] { Value::Number(n) => (*n as usize).max(10), _ => 100 }
    } else {
        100
    };
    let td = TDigestSketch::new(max_centroids);
    Ok(tdigest_to_value(&td))
}

fn builtin_tdigest_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let val = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Nilai harus angka", span)) };
    let weight = if args.len() > 2 {
        match &args[2] { Value::Number(n) => *n, _ => 1.0 }
    } else {
        1.0
    };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut td) = value_to_tdigest(&args[0]) {
                td.add(val, weight);
                let updated = tdigest_to_value(&td);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek TDigest tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'tdigest_tambah' memerlukan instance TDigest", span)),
    }
}

fn builtin_tdigest_persentil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let q = match &args[1] {
        Value::Number(n) => {
            if *n > 1.0 { *n / 100.0 } else { *n }
        }
        _ => return Err(Galat::runtime("Quantile q harus angka", span)),
    };

    if let Some(td) = value_to_tdigest(&args[0]) {
        return Ok(Value::Number(td.quantile(q)));
    }
    Err(Galat::runtime("Fungsi 'tdigest_persentil' memerlukan instance TDigest", span))
}

fn builtin_tdigest_cdf(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let val = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Nilai evaluasi CDF harus angka", span)) };
    if let Some(td) = value_to_tdigest(&args[0]) {
        return Ok(Value::Number(td.cdf(val)));
    }
    Err(Galat::runtime("Fungsi 'tdigest_cdf' memerlukan instance TDigest", span))
}

fn builtin_tdigest_gabung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let other_td = match value_to_tdigest(&args[1]) {
        Some(t) => t,
        None => return Err(Galat::runtime("Argumen kedua harus berupa instance TDigest", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut td) = value_to_tdigest(&args[0]) {
                td.merge(&other_td);
                let updated = tdigest_to_value(&td);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek TDigest tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'tdigest_gabung' memerlukan instance TDigest", span)),
    }
}

