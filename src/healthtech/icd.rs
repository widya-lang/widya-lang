use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalDiagnosis {
    pub icd10_code: String,
    pub icd11_code: Option<String>,
    pub nama_diagnosis_id: String,
    pub english_display: String,
    pub is_bpjs_covered: bool,
}

/// Kamus Diagnosa Medis ICD-10 Standar Pelayanan Kesehatan Primer & BPJS
pub fn cari_diagnosis_icd10(kata_kunci: &str) -> Vec<MedicalDiagnosis> {
    let database = [
        MedicalDiagnosis {
            icd10_code: "A09.0".to_string(),
            icd11_code: Some("1A40.0".to_string()),
            nama_diagnosis_id: "Gastroenteritis dan kolitis infeksius (Diare Akut)".to_string(),
            english_display: "Other and unspecified gastroenteritis and colitis of infectious origin".to_string(),
            is_bpjs_covered: true,
        },
        MedicalDiagnosis {
            icd10_code: "I10".to_string(),
            icd11_code: Some("BA00".to_string()),
            nama_diagnosis_id: "Hipertensi Esensial (Tekanan Darah Tinggi Primer)".to_string(),
            english_display: "Essential (primary) hypertension".to_string(),
            is_bpjs_covered: true,
        },
        MedicalDiagnosis {
            icd10_code: "E11.9".to_string(),
            icd11_code: Some("5A11".to_string()),
            nama_diagnosis_id: "Diabetes Melitus Tipe 2 Tanpa Komplikasi".to_string(),
            english_display: "Type 2 diabetes mellitus without complications".to_string(),
            is_bpjs_covered: true,
        },
        MedicalDiagnosis {
            icd10_code: "J00".to_string(),
            icd11_code: Some("CA00".to_string()),
            nama_diagnosis_id: "Nasofaringitis Akut (Flu Biasa / Common Cold)".to_string(),
            english_display: "Acute nasopharyngitis [common cold]".to_string(),
            is_bpjs_covered: true,
        },
        MedicalDiagnosis {
            icd10_code: "J45.9".to_string(),
            icd11_code: Some("CA23".to_string()),
            nama_diagnosis_id: "Asma Bronkial Tak Terspesifikasi".to_string(),
            english_display: "Asthma, unspecified".to_string(),
            is_bpjs_covered: true,
        },
        MedicalDiagnosis {
            icd10_code: "K29.7".to_string(),
            icd11_code: Some("DA42".to_string()),
            nama_diagnosis_id: "Gastritis Tak Terspesifikasi (Sakit Maag)".to_string(),
            english_display: "Gastritis, unspecified".to_string(),
            is_bpjs_covered: true,
        },
        MedicalDiagnosis {
            icd10_code: "Z00.0".to_string(),
            icd11_code: Some("QA00".to_string()),
            nama_diagnosis_id: "Pemeriksaan Medis Umum (Medical Check-Up Rutin)".to_string(),
            english_display: "General medical examination".to_string(),
            is_bpjs_covered: false,
        },
    ];

    let query = kata_kunci.to_lowercase();
    database
        .iter()
        .filter(|d| {
            d.icd10_code.to_lowercase().contains(&query)
                || d.nama_diagnosis_id.to_lowercase().contains(&query)
                || d.english_display.to_lowercase().contains(&query)
        })
        .cloned()
        .collect()
}
