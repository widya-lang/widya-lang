//! Pure Sovereign Zero-Dependency Medical Imaging & DICOM v3.0 Engine (pure_dicom.rs)
//!
//! 100% In-House Implementation:
//! - DICOM (Digital Imaging and Communications in Medicine) PS 3.10 File Format Parser & Encoder
//! - 128-byte Preamble + "DICM" Prefix Validation
//! - Explicit VR Little Endian (1.2.840.10008.1.2.1) & Implicit VR Little Endian Tag Reader
//! - Computed Tomography (CT) & Magnetic Resonance (MR) Hounsfield Unit (HU) Transformator:
//!   HU = RawPixel * RescaleSlope + RescaleIntercept
//! - Contrast Windowing Engine (Window Center / Level & Window Width) dengan Preset Klinis:
//!   Soft Tissue, Bone, Lung, dan Brain
//! - Direct Export to Sovereign `ImageBuffer` (RgbaPixel) tanpa DCMTK, GDCM, atau pustaka C++ luar.

use crate::multimedia::image::{ImageBuffer, RgbaPixel};
use serde::{Deserialize, Serialize};

pub const DICOM_PREAMBLE_LEN: usize = 128;
pub const DICOM_MAGIC: [u8; 4] = [b'D', b'I', b'C', b'M'];

// Transfer Syntaxes
pub const UID_EXPLICIT_VR_LITTLE_ENDIAN: &str = "1.2.840.10008.1.2.1";
pub const UID_IMPLICIT_VR_LITTLE_ENDIAN: &str = "1.2.840.10008.1.2";
pub const UID_CT_IMAGE_STORAGE: &str          = "1.2.840.10008.5.1.4.1.1.2";
pub const UID_MR_IMAGE_STORAGE: &str          = "1.2.840.10008.5.1.4.1.1.4";

/// Preset Window Kontras Radiologi Klinis
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DicomWindowPreset {
    SoftTissue, // Abdomen/Organ Lunak: C = 40, W = 400
    Bone,       // Tulang: C = 400, W = 1800
    Lung,       // Paru-Paru: C = -600, W = 1500
    Brain,      // Otak: C = 40, W = 80
    Custom { center: f64, width: f64 },
}

impl DicomWindowPreset {
    pub fn center_and_width(&self) -> (f64, f64) {
        match self {
            Self::SoftTissue => (40.0, 400.0),
            Self::Bone => (400.0, 1800.0),
            Self::Lung => (-600.0, 1500.0),
            Self::Brain => (40.0, 80.0),
            Self::Custom { center, width } => (*center, *width),
        }
    }
}

/// Berkas Citra Medis DICOM v3.0 Berdaulat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PureDicomFile {
    pub patient_name: String,
    pub patient_id: String,
    pub study_date: String,
    pub modality: String, // "CT", "MR", "CR", "DX", "US", "OT"
    pub rows: u16,
    pub columns: u16,
    pub bits_allocated: u16,       // Typically 16
    pub bits_stored: u16,          // Typically 12 or 16
    pub high_bit: u16,             // Typically 11 or 15
    pub pixel_representation: u16, // 0 = unsigned, 1 = 2's complement signed
    pub rescale_intercept: f64,    // CT Intercept (e.g. -1024.0)
    pub rescale_slope: f64,        // CT Slope (e.g. 1.0)
    pub window_center: f64,        // Level (e.g. 40.0)
    pub window_width: f64,         // Width (e.g. 400.0)
    pub pixel_data: Vec<i16>,      // Raw 16-bit pixel values
}

impl PureDicomFile {
    /// Konstruktor citra CT scan sintetis/akuisisi baru
    pub fn new_ct(
        patient_name: &str,
        patient_id: &str,
        rows: u16,
        columns: u16,
        intercept: f64,
        slope: f64,
        pixels: Vec<i16>,
    ) -> Self {
        Self {
            patient_name: patient_name.to_string(),
            patient_id: patient_id.to_string(),
            study_date: "20261011".to_string(),
            modality: "CT".to_string(),
            rows,
            columns,
            bits_allocated: 16,
            bits_stored: 16,
            high_bit: 15,
            pixel_representation: 1, // Signed 16-bit
            rescale_intercept: intercept,
            rescale_slope: slope,
            window_center: 40.0,
            window_width: 400.0,
            pixel_data: pixels,
        }
    }

    /// Konversi nilai piksel mentah ke skala densitas radiologi Hounsfield Unit (HU)
    #[inline]
    pub fn get_hu(&self, raw_value: i16) -> f64 {
        (raw_value as f64) * self.rescale_slope + self.rescale_intercept
    }

    /// Terapkan Contrast Windowing dan ekspor langsung ke `ImageBuffer` RGBA 8-bit
    pub fn to_image_buffer(&self, preset: DicomWindowPreset) -> ImageBuffer {
        let (window_center, window_width) = preset.center_and_width();
        let total = (self.rows as usize) * (self.columns as usize);
        let mut pixels = Vec::with_capacity(total);

        let half_width = window_width * 0.5;
        let min_val = window_center - half_width;
        let max_val = window_center + half_width;

        for &raw in &self.pixel_data {
            let hu = self.get_hu(raw);
            let gray = if hu <= min_val {
                0u8
            } else if hu >= max_val {
                255u8
            } else {
                (((hu - min_val) / window_width) * 255.0).clamp(0.0, 255.0).round() as u8
            };
            pixels.push(RgbaPixel::new(gray, gray, gray, 255));
        }

        ImageBuffer {
            width: self.columns as u32,
            height: self.rows as u32,
            pixels,
        }
    }

    /// Serialisasi biner ke format file biner DICOM Part 10 standar (.dcm)
    pub fn encode_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        // 1. 128 bytes preamble (0x00) + 4 bytes "DICM"
        out.resize(DICOM_PREAMBLE_LEN, 0x00);
        out.extend_from_slice(&DICOM_MAGIC);

        // Helper untuk menulis elemen data Explicit VR Little Endian
        let mut write_element = |group: u16, element: u16, vr: &[u8; 2], value: &[u8]| {
            out.extend_from_slice(&group.to_le_bytes());
            out.extend_from_slice(&element.to_le_bytes());
            out.extend_from_slice(vr);

            let pad = value.len() % 2 != 0;
            let full_len = if pad { value.len() + 1 } else { value.len() };

            // Jika VR adalah OB, OW, OF, SQ, UT, UN: 2 bytes padding nol + 4 bytes length
            let is_32bit_len = matches!(vr, b"OB" | b"OW" | b"OF" | b"SQ" | b"UT" | b"UN");
            if is_32bit_len {
                out.extend_from_slice(&[0, 0]); // Reserved
                out.extend_from_slice(&(full_len as u32).to_le_bytes());
            } else {
                out.extend_from_slice(&(full_len as u16).to_le_bytes());
            }
            out.extend_from_slice(value);
            if pad {
                out.push(0x20); // Pad with space if odd length
            }
        };


        // File Meta Information
        write_element(0x0002, 0x0002, b"UI", UID_CT_IMAGE_STORAGE.as_bytes());
        write_element(0x0002, 0x0003, b"UI", b"1.2.826.0.1.3680043.2.1125.1");
        write_element(0x0002, 0x0010, b"UI", UID_EXPLICIT_VR_LITTLE_ENDIAN.as_bytes());

        // Dataset Elements
        write_element(0x0008, 0x0020, b"DA", self.study_date.as_bytes());
        write_element(0x0008, 0x0060, b"CS", self.modality.as_bytes());
        write_element(0x0010, 0x0010, b"PN", self.patient_name.as_bytes());
        write_element(0x0010, 0x0020, b"LO", self.patient_id.as_bytes());

        // Geometry & Dimensions
        write_element(0x0028, 0x0002, b"US", &1u16.to_le_bytes());
        write_element(0x0028, 0x0004, b"CS", b"MONOCHROME2");
        write_element(0x0028, 0x0010, b"US", &self.rows.to_le_bytes());
        write_element(0x0028, 0x0011, b"US", &self.columns.to_le_bytes());
        write_element(0x0028, 0x0100, b"US", &self.bits_allocated.to_le_bytes());
        write_element(0x0028, 0x0101, b"US", &self.bits_stored.to_le_bytes());
        write_element(0x0028, 0x0102, b"US", &self.high_bit.to_le_bytes());
        write_element(0x0028, 0x0103, b"US", &self.pixel_representation.to_le_bytes());

        // Radiometry Rescale
        let intercept_str = format!("{:.2}", self.rescale_intercept);
        let slope_str = format!("{:.2}", self.rescale_slope);
        let center_str = format!("{:.2}", self.window_center);
        let width_str = format!("{:.2}", self.window_width);

        write_element(0x0028, 0x1050, b"DS", center_str.as_bytes());
        write_element(0x0028, 0x1051, b"DS", width_str.as_bytes());
        write_element(0x0028, 0x1052, b"DS", intercept_str.as_bytes());
        write_element(0x0028, 0x1053, b"DS", slope_str.as_bytes());

        // Pixel Data (7FE0, 0010)
        let mut raw_bytes = Vec::with_capacity(self.pixel_data.len() * 2);
        for &px in &self.pixel_data {
            raw_bytes.extend_from_slice(&px.to_le_bytes());
        }
        write_element(0x7FE0, 0x0010, b"OW", &raw_bytes);

        out
    }

    /// Deserialisasi biner dari berkas DICOM Part 10 standar
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 132 {
            return Err("Ukuran berkas terlalu kecil untuk format DICOM (minimal 132 byte)".to_string());
        }

        // Cek Magic "DICM"
        if &bytes[128..132] != &DICOM_MAGIC {
            return Err("Magic header 'DICM' tidak ditemukan pada offset 128".to_string());
        }

        let mut patient_name = String::new();
        let mut patient_id = String::new();
        let mut study_date = String::new();
        let mut modality = "OT".to_string();
        let mut rows = 0u16;
        let mut columns = 0u16;
        let mut bits_allocated = 16u16;
        let mut bits_stored = 16u16;
        let mut high_bit = 15u16;
        let mut pixel_representation = 0u16;
        let mut rescale_intercept = 0.0f64;
        let mut rescale_slope = 1.0f64;
        let mut window_center = 40.0f64;
        let mut window_width = 400.0f64;
        let mut pixel_data = Vec::new();

        let mut offset = 132;
        while offset + 8 <= bytes.len() {
            let group = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            let element = u16::from_le_bytes([bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;

            // Cek apakah Explicit VR (karakter huruf besar ASCII 2 byte)
            let vr0 = bytes[offset];
            let vr1 = bytes[offset + 1];
            let is_explicit = vr0.is_ascii_uppercase() && vr1.is_ascii_uppercase();

            let length: usize;
            if is_explicit {
                let vr = [vr0, vr1];
                offset += 2;
                let is_32bit = matches!(&vr, b"OB" | b"OW" | b"OF" | b"SQ" | b"UT" | b"UN");
                if is_32bit {
                    if offset + 6 > bytes.len() { break; }
                    offset += 2; // skip reserved 2 bytes
                    length = u32::from_le_bytes([
                        bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3],
                    ]) as usize;
                    offset += 4;
                } else {
                    if offset + 2 > bytes.len() { break; }
                    length = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]) as usize;
                    offset += 2;
                }
            } else {
                // Implicit VR: 4 bytes length
                length = u32::from_le_bytes([
                    bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3],
                ]) as usize;
                offset += 4;
            }

            if offset + length > bytes.len() {
                // Potong jika melebihi panjang berkas
                break;
            }

            let val_bytes = &bytes[offset..offset + length];

            match (group, element) {
                (0x0008, 0x0020) => study_date = String::from_utf8_lossy(val_bytes).trim().to_string(),
                (0x0008, 0x0060) => modality = String::from_utf8_lossy(val_bytes).trim().to_string(),
                (0x0010, 0x0010) => patient_name = String::from_utf8_lossy(val_bytes).trim_matches('\0').trim().to_string(),
                (0x0010, 0x0020) => patient_id = String::from_utf8_lossy(val_bytes).trim_matches('\0').trim().to_string(),
                (0x0028, 0x0010) => if val_bytes.len() >= 2 { rows = u16::from_le_bytes([val_bytes[0], val_bytes[1]]); },
                (0x0028, 0x0011) => if val_bytes.len() >= 2 { columns = u16::from_le_bytes([val_bytes[0], val_bytes[1]]); },
                (0x0028, 0x0100) => if val_bytes.len() >= 2 { bits_allocated = u16::from_le_bytes([val_bytes[0], val_bytes[1]]); },
                (0x0028, 0x0101) => if val_bytes.len() >= 2 { bits_stored = u16::from_le_bytes([val_bytes[0], val_bytes[1]]); },
                (0x0028, 0x0102) => if val_bytes.len() >= 2 { high_bit = u16::from_le_bytes([val_bytes[0], val_bytes[1]]); },
                (0x0028, 0x0103) => if val_bytes.len() >= 2 { pixel_representation = u16::from_le_bytes([val_bytes[0], val_bytes[1]]); },
                (0x0028, 0x1050) => {
                    if let Ok(s) = std::str::from_utf8(val_bytes) {
                        window_center = s.trim().parse().unwrap_or(40.0);
                    }
                }
                (0x0028, 0x1051) => {
                    if let Ok(s) = std::str::from_utf8(val_bytes) {
                        window_width = s.trim().parse().unwrap_or(400.0);
                    }
                }
                (0x0028, 0x1052) => {
                    if let Ok(s) = std::str::from_utf8(val_bytes) {
                        rescale_intercept = s.trim().parse().unwrap_or(0.0);
                    }
                }
                (0x0028, 0x1053) => {
                    if let Ok(s) = std::str::from_utf8(val_bytes) {
                        rescale_slope = s.trim().parse().unwrap_or(1.0);
                    }
                }
                (0x7FE0, 0x0010) => {
                    // Pixel Data
                    let num_pixels = length / 2;
                    pixel_data = Vec::with_capacity(num_pixels);
                    for chunk in val_bytes.chunks_exact(2) {
                        let px = i16::from_le_bytes([chunk[0], chunk[1]]);
                        pixel_data.push(px);
                    }
                }
                _ => {}
            }

            offset += length;
        }

        Ok(Self {
            patient_name,
            patient_id,
            study_date,
            modality,
            rows,
            columns,
            bits_allocated,
            bits_stored,
            high_bit,
            pixel_representation,
            rescale_intercept,
            rescale_slope,
            window_center,
            window_width,
            pixel_data,
        })
    }
}

// ============================================================================
// BRIDGES DENGAN SUBSISTEM HEALTHTECH LEGACY
// ============================================================================

impl From<PureDicomFile> for crate::healthtech::dicom::DicomMetadata {
    fn from(p: PureDicomFile) -> Self {
        Self {
            is_valid_dicom: true,
            patient_name: Some(p.patient_name),
            patient_id: Some(p.patient_id),
            modality: p.modality,
            study_date: Some(p.study_date),
            rows: p.rows as u32,
            columns: p.columns as u32,
            bits_allocated: p.bits_allocated,
            samples_per_pixel: 1,
            photometric_interpretation: "MONOCHROME2".to_string(),
        }
    }
}
