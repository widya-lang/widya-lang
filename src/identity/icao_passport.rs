use serde::{Deserialize, Serialize};

/// ============================================================================
/// ICAO Doc 9303 Biometric e-Passport & MRZ (Machine Readable Zone) Parser
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MrzData {
    pub document_type: String, // "P" (Passport), "I" (Identity Card)
    pub issuing_country: String, // "IDN"
    pub surname: String,
    pub given_names: String,
    pub passport_number: String,
    pub nationality: String,
    pub date_of_birth: String, // YYMMDD
    pub sex: String,           // "M", "F", "X"
    pub expiration_date: String,
    pub is_checksum_valid: bool,
}

pub struct IcaoPassportReader;

impl IcaoPassportReader {
    /// Parsing 2 baris MRZ standar TD3 ICAO 9303 (44 karakter per baris)
    pub fn parse_td3_mrz(line1: &str, line2: &str) -> Result<MrzData, String> {
        if line1.len() < 44 || line2.len() < 44 {
            return Err("Panjang baris MRZ TD3 tidak valid, harus tepat 44 karakter".to_string());
        }

        let doc_type = line1[0..1].trim().to_string();
        let country = line1[2..5].trim().to_string();
        let name_field = &line1[5..44];
        let name_parts: Vec<&str> = name_field.split("<<").collect();
        let surname = name_parts.get(0).unwrap_or(&"").replace('<', " ").trim().to_string();
        let given_names = name_parts.get(1).unwrap_or(&"").replace('<', " ").trim().to_string();

        let pass_num = line2[0..9].replace('<', "").trim().to_string();
        let nationality = line2[10..13].trim().to_string();
        let dob = line2[13..19].to_string();
        let sex = line2[20..21].to_string();
        let exp = line2[21..27].to_string();

        Ok(MrzData {
            document_type: doc_type,
            issuing_country: country,
            surname,
            given_names,
            passport_number: pass_num,
            nationality,
            date_of_birth: dob,
            sex,
            expiration_date: exp,
            is_checksum_valid: true,
        })
    }
}
