use serde::{Deserialize, Serialize};

/// Resource Pasien HL7 FHIR v4.0 (Standar Wajib SatuSehat Kemenkes RI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirPatient {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub active: bool,
    pub identifier: Vec<FhirIdentifier>,
    pub name: Vec<FhirHumanName>,
    pub gender: String, // "male", "female", "other", "unknown"
    #[serde(rename = "birthDate")]
    pub birth_date: String, // "YYYY-MM-DD"
    pub telecom: Vec<FhirContactPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirIdentifier {
    pub system: String, // e.g. "https://fhir.kemkes.go.id/id/nik"
    pub value: String,  // Nomor Induk Kependudukan (NIK 16 digit)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirHumanName {
    #[serde(rename = "use")]
    pub name_use: String, // "official"
    pub text: String,     // Nama lengkap
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirContactPoint {
    pub system: String, // "phone", "email"
    pub value: String,
    #[serde(rename = "use")]
    pub contact_use: String, // "mobile", "home", "work"
}

impl FhirPatient {
    pub fn new_satusehat(nik: &str, nama: &str, gender: &str, birth_date: &str, telepon: &str) -> Self {
        Self {
            resource_type: "Patient".to_string(),
            id: format!("satusehat-{}", nik),
            active: true,
            identifier: vec![FhirIdentifier {
                system: "https://fhir.kemkes.go.id/id/nik".to_string(),
                value: nik.to_string(),
            }],
            name: vec![FhirHumanName {
                name_use: "official".to_string(),
                text: nama.to_string(),
            }],
            gender: gender.to_string(),
            birth_date: birth_date.to_string(),
            telecom: vec![FhirContactPoint {
                system: "phone".to_string(),
                value: telepon.to_string(),
                contact_use: "mobile".to_string(),
            }],
        }
    }
}

/// Resource Kunjungan Pasien (Encounter HL7 FHIR v4.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirEncounter {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub status: String, // "in-progress", "finished", "cancelled"
    #[serde(rename = "class")]
    pub encounter_class: FhirCoding, // "AMB" (Rawat Jalan), "IMP" (Rawat Inap), "EMER" (IGD)
    pub subject: FhirReference,
    pub period_start: String,
    pub period_end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirCoding {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirReference {
    pub reference: String, // "Patient/satusehat-3273..."
    pub display: Option<String>,
}

/// Resource Observasi Tanda Vital / Laboratorium (Observation HL7 FHIR v4.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirObservation {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub status: String, // "final"
    pub category: Vec<FhirCodeableConcept>,
    pub code: FhirCodeableConcept,
    pub subject: FhirReference,
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<FhirQuantity>,
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirCodeableConcept {
    pub coding: Vec<FhirCoding>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirQuantity {
    pub value: f64,
    pub unit: String, // e.g. "mmHg", "Cel", "mg/dL", "/min"
    pub system: String,
    pub code: String,
}

impl FhirObservation {
    /// Membuat Observasi Tekanan Darah Sistolik (LOINC: 8480-6)
    pub fn sistolik(patient_id: &str, value_mmhg: f64, waktu_iso: &str) -> Self {
        Self {
            resource_type: "Observation".to_string(),
            id: format!("obs-sys-{}", uuid::Uuid::new_v4()),
            status: "final".to_string(),
            category: vec![FhirCodeableConcept {
                coding: vec![FhirCoding {
                    system: "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
                    code: "vital-signs".to_string(),
                    display: "Vital Signs".to_string(),
                }],
                text: Some("Tanda Vital".to_string()),
            }],
            code: FhirCodeableConcept {
                coding: vec![FhirCoding {
                    system: "http://loinc.org".to_string(),
                    code: "8480-6".to_string(),
                    display: "Systolic blood pressure".to_string(),
                }],
                text: Some("Tekanan Darah Sistolik".to_string()),
            },
            subject: FhirReference {
                reference: format!("Patient/{}", patient_id),
                display: None,
            },
            value_quantity: Some(FhirQuantity {
                value: value_mmhg,
                unit: "mmHg".to_string(),
                system: "http://unitsofmeasure.org".to_string(),
                code: "mm[Hg]".to_string(),
            }),
            effective_date_time: waktu_iso.to_string(),
        }
    }
}
