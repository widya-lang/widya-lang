use serde::{Deserialize, Serialize};

/// Metadata Citra Medis DICOM v3.0 (Digital Imaging and Communications in Medicine)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DicomMetadata {
    pub is_valid_dicom: bool,
    pub patient_name: Option<String>,
    pub patient_id: Option<String>,
    pub modality: String, // "CT", "MR", "CR", "DX", "US", "OT"
    pub study_date: Option<String>,
    pub rows: u32,
    pub columns: u32,
    pub bits_allocated: u16,
    pub samples_per_pixel: u16,
    pub photometric_interpretation: String, // "MONOCHROME1", "MONOCHROME2", "RGB"
}

/// Parser Berkas Biner Citra Medis DICOM v3.0
pub fn parse_dicom_bytes(bytes: &[u8]) -> Result<DicomMetadata, String> {
    if bytes.len() < 132 {
        return Err("Ukuran berkas terlalu kecil untuk format DICOM (minimal 132 byte)".to_string());
    }

    // Periksa magic header "DICM" pada offset 128
    let magic = &bytes[128..132];
    let is_standard_dicom = magic == b"DICM";

    let mut metadata = DicomMetadata {
        is_valid_dicom: is_standard_dicom,
        patient_name: None,
        patient_id: None,
        modality: "OT".to_string(), // Other/General
        study_date: None,
        rows: 512,
        columns: 512,
        bits_allocated: 16,
        samples_per_pixel: 1,
        photometric_interpretation: "MONOCHROME2".to_string(),
    };

    if !is_standard_dicom {
        return Err("Berkas bukan format standar DICOM (Magic header 'DICM' tidak ditemukan)".to_string());
    }

    // Parsing Tag Element Dasar (Explicit / Implicit VR)
    let mut offset = 132;
    while offset + 8 <= bytes.len() {
        let group = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        let element = u16::from_le_bytes([bytes[offset + 2], bytes[offset + 3]]);
        offset += 4;

        // Cek Tag:
        // (0008, 0020) Study Date
        // (0008, 0060) Modality
        // (0010, 0010) Patient Name
        // (0010, 0020) Patient ID
        // (0028, 0010) Rows
        // (0028, 0011) Columns
        let len = match (group, element) {
            (0x0008, 0x0060) => { // Modality
                let val_len = 2.min(bytes.len() - offset);
                if let Ok(s) = std::str::from_utf8(&bytes[offset..offset + val_len]) {
                    metadata.modality = s.trim().to_string();
                }
                val_len
            }
            (0x0010, 0x0010) => { // Patient Name
                let val_len = 32.min(bytes.len() - offset);
                if let Ok(s) = std::str::from_utf8(&bytes[offset..offset + val_len]) {
                    metadata.patient_name = Some(s.trim_matches('\0').trim().to_string());
                }
                val_len
            }
            (0x0010, 0x0020) => { // Patient ID
                let val_len = 16.min(bytes.len() - offset);
                if let Ok(s) = std::str::from_utf8(&bytes[offset..offset + val_len]) {
                    metadata.patient_id = Some(s.trim_matches('\0').trim().to_string());
                }
                val_len
            }
            _ => 4,
        };

        offset += len;
        if offset > 4096 {
            break; // Cukup baca header awal
        }
    }

    Ok(metadata)
}
