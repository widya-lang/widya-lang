//! Pure Sovereign Zero-Dependency Geophysical Exploration & Seismic SEG-Y Engine (pure_mining.rs)
//!
//! 100% In-House Implementation:
//! - SEG-Y Rev 1/2 Standard Binary File Reader & Serializer
//! - 3200-Byte EBCDIC <-> ASCII Textual Header Transcoder
//! - 400-Byte Binary Reel Header & 240-Byte Standard Trace Header Decoder
//! - Sovereign IBM 360/370 32-Bit Hexadecimal Floating Point Decoder & Encoder
//! - Subsurface Stratigraphic Acoustic Impedance & Zoeppritz Reflection Coefficient Engine
//! - Ricker Wavelet Synthetic Seismogram Generator
//! - Direct Interoperability Bridge with `SegyDataset` & `SeismicTrace`

use serde::{Deserialize, Serialize};

// ============================================================================
// 1. IBM 360/370 FLOATING POINT CONVERSION ENGINE
// ============================================================================

pub struct IbmFloat;

impl IbmFloat {
    /// Konversi 4 byte format IBM 360/370 floating point ke f32 IEEE 754
    /// Struktur: [Sign (1 bit) | Exponent base 16 (7 bit, excess 64) | Mantissa (24 bit)]
    /// Nilai = (-1)^sign * (mantissa / 2^24) * 16^(exp - 64)
    pub fn to_f32(bytes: [u8; 4]) -> f32 {
        let raw = u32::from_be_bytes(bytes);
        if raw == 0 {
            return 0.0;
        }

        let sign = (raw >> 31) & 0x01;
        let exponent = ((raw >> 24) & 0x7F) as i32 - 64;
        let mantissa = (raw & 0x00FFFFFF) as f64 / 16777216.0; // 2^24

        // 16^exp = 2^(4 * exp)
        let value = mantissa * (16.0f64).powi(exponent);
        let signed_value = if sign == 1 { -value } else { value };
        signed_value as f32
    }

    /// Konversi f32 IEEE 754 ke 4 byte format IBM 360/370 floating point
    pub fn from_f32(val: f32) -> [u8; 4] {
        if val == 0.0 {
            return [0, 0, 0, 0];
        }

        let sign = if val < 0.0 { 1u32 } else { 0u32 };
        let abs_val = (val.abs() as f64).max(1e-35);

        // Cari exponent basis 16: 16^(exp - 64)
        let log16 = abs_val.log(16.0);
        let mut exp = log16.floor() as i32 + 1;
        let mut mantissa = abs_val / (16.0f64).powi(exp);

        while mantissa >= 1.0 {
            exp += 1;
            mantissa /= 16.0;
        }
        while mantissa < (1.0 / 16.0) && exp > -64 {
            exp -= 1;
            mantissa *= 16.0;
        }

        let exp_field = ((exp + 64).clamp(0, 127) as u32) & 0x7F;
        let mantissa_field = ((mantissa * 16777216.0).round() as u32).min(0x00FFFFFF);

        let raw = (sign << 31) | (exp_field << 24) | mantissa_field;
        raw.to_be_bytes()
    }
}

// ============================================================================
// 2. EBCDIC <-> ASCII TRANSCODER
// ============================================================================

pub struct EbcdicTranscoder;

impl EbcdicTranscoder {
    /// Transkode 1 byte EBCDIC (kode IBM lawas) ke karakter ASCII
    pub fn ebcdic_to_ascii_byte(b: u8) -> u8 {
        match b {
            0x40 => b' ',
            0xC1..=0xC9 => b'A' + (b - 0xC1),
            0xD1..=0xD9 => b'J' + (b - 0xD1),
            0xE2..=0xE9 => b'S' + (b - 0xE2),
            0x81..=0x89 => b'a' + (b - 0x81),
            0x91..=0x99 => b'j' + (b - 0x91),
            0xA2..=0xA9 => b's' + (b - 0xA2),
            0xF0..=0xF9 => b'0' + (b - 0xF0),
            0x4B => b'.',
            0x6B => b',',
            0x4E => b'+',
            0x60 => b'-',
            0x61 => b'/',
            0x7D => b'\'',
            0x7E => b'=',
            _ => b' ',
        }
    }

    /// Transkode 1 byte ASCII ke byte EBCDIC
    pub fn ascii_to_ebcdic_byte(b: u8) -> u8 {
        match b {
            b' ' => 0x40,
            b'A'..=b'I' => 0xC1 + (b - b'A'),
            b'J'..=b'R' => 0xD1 + (b - b'J'),
            b'S'..=b'Z' => 0xE2 + (b - b'S'),
            b'a'..=b'i' => 0x81 + (b - b'a'),
            b'j'..=b'r' => 0x91 + (b - b'j'),
            b's'..=b'z' => 0xA2 + (b - b's'),
            b'0'..=b'9' => 0xF0 + (b - b'0'),
            b'.' => 0x4B,
            b',' => 0x6B,
            b'+' => 0x4E,
            b'-' => 0x60,
            b'/' => 0x61,
            _ => 0x40,
        }
    }

    pub fn decode_ebcdic(bytes: &[u8]) -> String {
        let ascii_bytes: Vec<u8> = bytes.iter().map(|&b| Self::ebcdic_to_ascii_byte(b)).collect();
        String::from_utf8_lossy(&ascii_bytes).to_string()
    }

    pub fn encode_ebcdic(text: &str, target_len: usize) -> Vec<u8> {
        let mut out = vec![0x40; target_len]; // 0x40 = EBCDIC space
        for (i, &b) in text.as_bytes().iter().take(target_len).enumerate() {
            out[i] = Self::ascii_to_ebcdic_byte(b);
        }
        out
    }
}

// ============================================================================
// 3. PURE SEG-Y REV 1/2 BINARY FILE PARSER & ENCODER
// ============================================================================

/// Format Sampel Amplitudo Geofisika SEG-Y
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegyDataFormat {
    IbmFloat32,  // Kode format 1: 4-byte IBM floating point
    Int32,       // Kode format 2: 4-byte two's complement integer
    Int16,       // Kode format 3: 2-byte two's complement integer
    IeeeFloat32, // Kode format 5: 4-byte IEEE 754 floating point
}

/// Header Jejak Seismik 240-Byte (Trace Header)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PureTraceHeader {
    pub trace_seq_line: u32,       // Bytes 0..4
    pub trace_seq_file: u32,       // Bytes 4..8
    pub field_record_num: u32,     // Bytes 8..12
    pub trace_num_in_record: u32,  // Bytes 12..16
    pub source_x: i32,             // Bytes 72..76
    pub source_y: i32,             // Bytes 76..80
    pub group_x: i32,              // Bytes 80..84 (Penerima / Geophone)
    pub group_y: i32,              // Bytes 84..88
    pub coord_scalar: i16,         // Bytes 68..70 (Pengali koordinat)
    pub sample_count: u16,         // Bytes 114..116
    pub sample_interval_us: u16,   // Bytes 116..118 (Interval sampling dalam mikrodetik)
}

/// Jejak Seismik Lengkap (Header 240B + Amplitudo Waveform)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PureTrace {
    pub header: PureTraceHeader,
    pub amplitudes: Vec<f32>,
}

/// Berkas Dataset Seismik SEG-Y Murni
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PureSegyFile {
    pub textual_header_ebcdic: Vec<u8>, // 3200 bytes
    pub sample_interval_us: u16,        // 400B Reel Header
    pub samples_per_trace: u16,
    pub format: SegyDataFormat,
    pub traces: Vec<PureTrace>,
}

impl PureSegyFile {
    pub fn new(sample_interval_us: u16, samples_per_trace: u16, format: SegyDataFormat) -> Self {
        let ebcdic = EbcdicTranscoder::encode_ebcdic("C01 WIDYA-LANG SOVEREIGN SEISMIC SURVEY - SEG-Y REV 2.0", 3200);
        Self {
            textual_header_ebcdic: ebcdic,
            sample_interval_us,
            samples_per_trace,
            format,
            traces: Vec::new(),
        }
    }

    /// Serialisasi Lengkap Berkas SEG-Y ke Buffer Biner
    pub fn encode_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        // 1. 3200-Byte Textual Header
        let mut text_header = self.textual_header_ebcdic.clone();
        text_header.resize(3200, 0x40);
        out.extend_from_slice(&text_header);

        // 2. 400-Byte Binary Reel Header
        let mut bin_header = vec![0u8; 400];
        // Offset 16 (0-indexed): Sample Interval (u16 BE)
        bin_header[16..18].copy_from_slice(&self.sample_interval_us.to_be_bytes());
        // Offset 20: Samples Per Trace (u16 BE)
        bin_header[20..22].copy_from_slice(&self.samples_per_trace.to_be_bytes());
        // Offset 24: Data Sample Format Code (u16 BE)
        let fmt_code: u16 = match self.format {
            SegyDataFormat::IbmFloat32 => 1,
            SegyDataFormat::Int32 => 2,
            SegyDataFormat::Int16 => 3,
            SegyDataFormat::IeeeFloat32 => 5,
        };
        bin_header[24..26].copy_from_slice(&fmt_code.to_be_bytes());
        out.extend_from_slice(&bin_header);

        // 3. Traces (Trace Header 240B + Samples)
        for trace in &self.traces {
            let mut tr_hdr = vec![0u8; 240];
            tr_hdr[0..4].copy_from_slice(&trace.header.trace_seq_line.to_be_bytes());
            tr_hdr[4..8].copy_from_slice(&trace.header.trace_seq_file.to_be_bytes());
            tr_hdr[68..70].copy_from_slice(&trace.header.coord_scalar.to_be_bytes());
            tr_hdr[72..76].copy_from_slice(&trace.header.source_x.to_be_bytes());
            tr_hdr[76..80].copy_from_slice(&trace.header.source_y.to_be_bytes());
            tr_hdr[80..84].copy_from_slice(&trace.header.group_x.to_be_bytes());
            tr_hdr[84..88].copy_from_slice(&trace.header.group_y.to_be_bytes());
            tr_hdr[114..116].copy_from_slice(&trace.header.sample_count.to_be_bytes());
            tr_hdr[116..118].copy_from_slice(&trace.header.sample_interval_us.to_be_bytes());
            out.extend_from_slice(&tr_hdr);

            // Tulis Sampel Amplitudo
            for &amp in &trace.amplitudes {
                match self.format {
                    SegyDataFormat::IeeeFloat32 => {
                        out.extend_from_slice(&amp.to_be_bytes());
                    }
                    SegyDataFormat::IbmFloat32 => {
                        let ibm = IbmFloat::from_f32(amp);
                        out.extend_from_slice(&ibm);
                    }
                    _ => {
                        out.extend_from_slice(&amp.to_be_bytes());
                    }
                }
            }
        }

        out
    }

    /// Deserialisasi Berkas SEG-Y dari Buffer Biner
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 3600 {
            return Err("Ukuran data terlalu kecil untuk header SEG-Y (< 3600 byte)".to_string());
        }

        let textual_header_ebcdic = bytes[0..3200].to_vec();
        let bin_hdr = &bytes[3200..3600];

        let sample_interval_us = u16::from_be_bytes([bin_hdr[16], bin_hdr[17]]);
        let samples_per_trace = u16::from_be_bytes([bin_hdr[20], bin_hdr[21]]);
        let fmt_code = u16::from_be_bytes([bin_hdr[24], bin_hdr[25]]);

        let format = match fmt_code {
            1 => SegyDataFormat::IbmFloat32,
            2 => SegyDataFormat::Int32,
            3 => SegyDataFormat::Int16,
            5 => SegyDataFormat::IeeeFloat32,
            _ => SegyDataFormat::IeeeFloat32,
        };

        let mut traces = Vec::new();
        let mut offset = 3600;
        let bytes_per_sample = match format {
            SegyDataFormat::Int16 => 2,
            _ => 4,
        };

        while offset + 240 <= bytes.len() {
            let tr_hdr_bytes = &bytes[offset..offset + 240];
            let trace_seq_line = u32::from_be_bytes([tr_hdr_bytes[0], tr_hdr_bytes[1], tr_hdr_bytes[2], tr_hdr_bytes[3]]);
            let trace_seq_file = u32::from_be_bytes([tr_hdr_bytes[4], tr_hdr_bytes[5], tr_hdr_bytes[6], tr_hdr_bytes[7]]);
            let coord_scalar = i16::from_be_bytes([tr_hdr_bytes[68], tr_hdr_bytes[69]]);
            let source_x = i32::from_be_bytes([tr_hdr_bytes[72], tr_hdr_bytes[73], tr_hdr_bytes[74], tr_hdr_bytes[75]]);
            let source_y = i32::from_be_bytes([tr_hdr_bytes[76], tr_hdr_bytes[77], tr_hdr_bytes[78], tr_hdr_bytes[79]]);
            let group_x = i32::from_be_bytes([tr_hdr_bytes[80], tr_hdr_bytes[81], tr_hdr_bytes[82], tr_hdr_bytes[83]]);
            let group_y = i32::from_be_bytes([tr_hdr_bytes[84], tr_hdr_bytes[85], tr_hdr_bytes[86], tr_hdr_bytes[87]]);
            let mut ns = u16::from_be_bytes([tr_hdr_bytes[114], tr_hdr_bytes[115]]);
            if ns == 0 {
                ns = samples_per_trace;
            }
            let dt = u16::from_be_bytes([tr_hdr_bytes[116], tr_hdr_bytes[117]]);

            let header = PureTraceHeader {
                trace_seq_line,
                trace_seq_file,
                field_record_num: 1,
                trace_num_in_record: 1,
                source_x,
                source_y,
                group_x,
                group_y,
                coord_scalar,
                sample_count: ns,
                sample_interval_us: if dt > 0 { dt } else { sample_interval_us },
            };

            offset += 240;
            let trace_data_len = (ns as usize) * bytes_per_sample;
            if offset + trace_data_len > bytes.len() {
                break;
            }

            let mut amplitudes = Vec::with_capacity(ns as usize);
            for i in 0..ns as usize {
                let sample_offset = offset + i * bytes_per_sample;
                let amp = match format {
                    SegyDataFormat::IeeeFloat32 => {
                        f32::from_be_bytes([
                            bytes[sample_offset],
                            bytes[sample_offset + 1],
                            bytes[sample_offset + 2],
                            bytes[sample_offset + 3],
                        ])
                    }
                    SegyDataFormat::IbmFloat32 => {
                        IbmFloat::to_f32([
                            bytes[sample_offset],
                            bytes[sample_offset + 1],
                            bytes[sample_offset + 2],
                            bytes[sample_offset + 3],
                        ])
                    }
                    _ => 0.0,
                };
                amplitudes.push(amp);
            }

            traces.push(PureTrace { header, amplitudes });
            offset += trace_data_len;
        }

        Ok(Self {
            textual_header_ebcdic,
            sample_interval_us,
            samples_per_trace,
            format,
            traces,
        })
    }
}

// ============================================================================
// 4. SUBSURFACE GEOPHYSICS & SEISMIC REFLECTION MODELING
// ============================================================================

/// Lapisan Stratigrafi Bawah Permukaan Bumi (Subsurface Layer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubsurfaceLayer {
    pub thickness_m: f64,        // Ketebalan lapisan dalam meter
    pub p_wave_velocity_m_s: f64, // Kecepatan gelombang P (Vp) dalam m/s (misal: Sedimen 2000, Batugamping 4000)
    pub density_kg_m3: f64,      // Densitas batuan dalam kg/m³ (misal: 2400 kg/m³)
}

impl SubsurfaceLayer {
    pub fn acoustic_impedance(&self) -> f64 {
        self.p_wave_velocity_m_s * self.density_kg_m3
    }
}

pub struct SeismicModelingEngine;

impl SeismicModelingEngine {
    /// Hitung Koefisien Refleksi Normal Incidence (Zoeppritz Limit): R = (Z2 - Z1) / (Z2 + Z1)
    pub fn reflection_coefficient(layer1: &SubsurfaceLayer, layer2: &SubsurfaceLayer) -> f64 {
        let z1 = layer1.acoustic_impedance();
        let z2 = layer2.acoustic_impedance();
        (z2 - z1) / (z2 + z1)
    }

    /// Evaluasi Amplitudo Wavelet Ricker: R(t) = (1 - 2*pi^2*f^2*t^2) * exp(-pi^2*f^2*t^2)
    pub fn ricker_wavelet(t_sec: f64, dominant_freq_hz: f64) -> f64 {
        let pi = std::f64::consts::PI;
        let pf2 = pi * pi * dominant_freq_hz * dominant_freq_hz;
        let t2 = t_sec * t_sec;
        (1.0 - 2.0 * pf2 * t2) * (-pf2 * t2).exp()
    }

    /// Sintesis Jejak Seismik 1D (Konvolusi Deret Reflektifitas dengan Wavelet Ricker)
    pub fn synthesize_trace(
        layers: &[SubsurfaceLayer],
        sample_interval_ms: f64,
        total_time_ms: f64,
        wavelet_freq_hz: f64,
    ) -> Vec<f32> {
        let num_samples = (total_time_ms / sample_interval_ms).ceil() as usize;
        let mut trace = vec![0.0f32; num_samples];

        // Hitung waktu tempuh bolak-balik (Two-Way Travel Time / TWT) pada batas tiap lapisan
        let mut current_twt_ms = 0.0;
        let mut reflection_series = Vec::new();

        for i in 0..layers.len().saturating_sub(1) {
            let l1 = &layers[i];
            let l2 = &layers[i + 1];
            // One way time = thickness / velocity, Two way time = 2 * thickness / velocity
            let layer_twt_ms = (2.0 * l1.thickness_m / l1.p_wave_velocity_m_s) * 1000.0;
            current_twt_ms += layer_twt_ms;
            let rc = Self::reflection_coefficient(l1, l2);
            reflection_series.push((current_twt_ms, rc));
        }

        // Konvolusi dengan Ricker Wavelet
        let half_win_ms = 50.0; // Window konvolusi +/- 50ms
        for sample_idx in 0..num_samples {
            let t_sample_ms = (sample_idx as f64) * sample_interval_ms;
            let mut amp = 0.0;

            for &(t_event_ms, rc) in &reflection_series {
                let dt_ms = t_sample_ms - t_event_ms;
                if dt_ms.abs() <= half_win_ms {
                    let w = Self::ricker_wavelet(dt_ms / 1000.0, wavelet_freq_hz);
                    amp += rc * w;
                }
            }

            trace[sample_idx] = amp as f32;
        }

        trace
    }
}

// ============================================================================
// 5. BRIDGES DENGAN SUBSISTEM MINING / SEGY LEGACY
// ============================================================================

impl From<PureSegyFile> for crate::mining::seismic_segy::SegyDataset {
    fn from(pure: PureSegyFile) -> Self {
        let mut dataset = crate::mining::seismic_segy::SegyDataset::new(
            pure.sample_interval_us,
            pure.samples_per_trace,
        );

        for tr in pure.traces {
            dataset.add_trace(
                tr.header.trace_seq_line,
                tr.header.source_x,
                tr.header.source_y,
                tr.header.group_x,
                tr.header.group_y,
                tr.amplitudes,
            );
        }

        dataset
    }
}
