use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Tag-Length-Value (TLV) structure untuk QRIS EMVCo
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QrisTlvNode {
    pub tag: String,
    pub length: usize,
    pub value: String,
    pub sub_tags: Option<BTreeMap<String, QrisTlvNode>>,
}

/// Representasi Data Payload QRIS Lengkap (EMVCo MPM - Merchant-Presented Mode)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrisPayload {
    pub payload_format_indicator: String, // Tag 00 (Default: "01")
    pub point_of_initiation: String,      // Tag 01 ("11" = Statis, "12" = Dinamis)
    pub merchant_category_code: String,   // Tag 52 (e.g. "5411")
    pub currency_code: String,            // Tag 53 ("360" untuk IDR)
    pub transaction_amount: Option<f64>,  // Tag 54 (Nominal jika Dinamis)
    pub country_code: String,             // Tag 58 ("ID")
    pub merchant_name: String,            // Tag 59
    pub merchant_city: String,            // Tag 60
    pub postal_code: Option<String>,      // Tag 61
    pub national_merchant_id: Option<String>, // Tag 51 (NMID)
    pub additional_data: Option<QrisAdditionalData>, // Tag 62
    pub merchant_accounts: BTreeMap<String, String>, // Tag 26-45
    pub raw_tlv_map: BTreeMap<String, String>,
    pub crc16: String,                    // Tag 63
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QrisAdditionalData {
    pub bill_number: Option<String>,      // Sub-tag 01
    pub mobile_number: Option<String>,    // Sub-tag 02
    pub store_label: Option<String>,      // Sub-tag 03
    pub loyalty_number: Option<String>,   // Sub-tag 04
    pub reference_id: Option<String>,     // Sub-tag 05
    pub customer_label: Option<String>,   // Sub-tag 06
    pub terminal_label: Option<String>,   // Sub-tag 07
    pub purpose: Option<String>,          // Sub-tag 08
    pub additional_data: Option<String>,  // Sub-tag 99
}

/// Menghitung Checksum CRC-16/CCITT-FALSE (Polinomial 0x1021, Initial 0xFFFF)
/// Standar resmi EMVCo / ASPI QRIS
pub fn hitung_qris_crc16(data: &str) -> String {
    let mut crc: u16 = 0xFFFF;
    for byte in data.bytes() {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    format!("{:04X}", crc)
}

/// Validasi string QRIS utuh termasuk checksum CRC16
pub fn validasi_qris_crc16(raw_qris: &str) -> bool {
    let s = raw_qris.trim();
    if s.len() < 8 {
        return false;
    }
    // Tag 63 harus berada di akhir: "6304XXXX"
    if let Some(pos) = s.rfind("6304") {
        if pos + 8 != s.len() {
            return false;
        }
        let data_to_check = &s[..pos + 4];
        let expected_crc = &s[pos + 4..];
        let calculated = hitung_qris_crc16(data_to_check);
        return calculated.eq_ignore_ascii_case(expected_crc);
    }
    false
}

/// Format TLV helper: format!("{}{:02}{}", tag, value.len(), value)
pub fn format_tlv(tag: &str, value: &str) -> String {
    format!("{}{:02}{}", tag, value.len(), value)
}

/// Parser mentah TLV ke Map
pub fn parse_raw_tlv(raw: &str) -> Result<BTreeMap<String, String>, String> {
    let mut map = BTreeMap::new();
    let chars: Vec<char> = raw.chars().collect();
    let mut idx = 0;

    while idx < chars.len() {
        if idx + 4 > chars.len() {
            break;
        }
        let tag: String = chars[idx..idx + 2].iter().collect();
        let len_str: String = chars[idx + 2..idx + 4].iter().collect();
        let len: usize = len_str.parse().map_err(|e| format!("Panjang TLV tidak valid di tag {}: {}", tag, e))?;

        idx += 4;
        if idx + len > chars.len() {
            return Err(format!("Data TLV terpotong di tag {}", tag));
        }
        let value: String = chars[idx..idx + len].iter().collect();
        map.insert(tag, value);
        idx += len;
    }

    Ok(map)
}

/// Parser QRIS String menjadi QrisPayload Terstruktur
pub fn parse_qris(raw_qris: &str) -> Result<QrisPayload, String> {
    if !validasi_qris_crc16(raw_qris) {
        return Err("Checksum CRC16 QRIS tidak sah atau korup".to_string());
    }

    let tlv = parse_raw_tlv(raw_qris)?;

    let payload_format = tlv.get("00").cloned().unwrap_or_else(|| "01".to_string());
    let poi = tlv.get("01").cloned().unwrap_or_else(|| "11".to_string());
    let mcc = tlv.get("52").cloned().unwrap_or_else(|| "0000".to_string());
    let currency = tlv.get("53").cloned().unwrap_or_else(|| "360".to_string());
    let country = tlv.get("58").cloned().unwrap_or_else(|| "ID".to_string());
    let merchant_name = tlv.get("59").cloned().unwrap_or_else(|| "".to_string());
    let merchant_city = tlv.get("60").cloned().unwrap_or_else(|| "".to_string());
    let postal = tlv.get("61").cloned();
    let nmid = tlv.get("51").cloned();
    let crc16 = tlv.get("63").cloned().unwrap_or_default();

    let amount = if let Some(amt_str) = tlv.get("54") {
        amt_str.parse::<f64>().ok()
    } else {
        None
    };

    let mut merchant_accounts = BTreeMap::new();
    for (tag, val) in &tlv {
        if let Ok(num) = tag.parse::<u32>() {
            if (26..=45).contains(&num) {
                merchant_accounts.insert(tag.clone(), val.clone());
            }
        }
    }

    let additional_data = if let Some(tag62_raw) = tlv.get("62") {
        if let Ok(sub_tlv) = parse_raw_tlv(tag62_raw) {
            Some(QrisAdditionalData {
                bill_number: sub_tlv.get("01").cloned(),
                mobile_number: sub_tlv.get("02").cloned(),
                store_label: sub_tlv.get("03").cloned(),
                loyalty_number: sub_tlv.get("04").cloned(),
                reference_id: sub_tlv.get("05").cloned(),
                customer_label: sub_tlv.get("06").cloned(),
                terminal_label: sub_tlv.get("07").cloned(),
                purpose: sub_tlv.get("08").cloned(),
                additional_data: sub_tlv.get("99").cloned(),
            })
        } else {
            None
        }
    } else {
        None
    };

    Ok(QrisPayload {
        payload_format_indicator: payload_format,
        point_of_initiation: poi,
        merchant_category_code: mcc,
        currency_code: currency,
        transaction_amount: amount,
        country_code: country,
        merchant_name,
        merchant_city,
        postal_code: postal,
        national_merchant_id: nmid,
        additional_data,
        merchant_accounts,
        raw_tlv_map: tlv,
        crc16,
    })
}

/// Builder untuk membuat QRIS Dinamis atau Statis
#[derive(Debug, Clone)]
pub struct QrisBuilder {
    pub is_dynamic: bool,
    pub merchant_name: String,
    pub merchant_city: String,
    pub postal_code: Option<String>,
    pub mcc: String,
    pub nmid: Option<String>,
    pub merchant_pan: Option<String>, // e.g. ID.CO.QRIS.WWW
    pub amount: Option<f64>,
    pub invoice_id: Option<String>,
    pub terminal_id: Option<String>,
}

impl Default for QrisBuilder {
    fn default() -> Self {
        Self {
            is_dynamic: true,
            merchant_name: "WIDYA STORE".to_string(),
            merchant_city: "JAKARTA".to_string(),
            postal_code: Some("12950".to_string()),
            mcc: "5411".to_string(),
            nmid: Some("ID1020000000000".to_string()),
            merchant_pan: Some("936000000000000000".to_string()),
            amount: None,
            invoice_id: None,
            terminal_id: Some("W01".to_string()),
        }
    }
}

impl QrisBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dynamic(mut self, is_dyn: bool) -> Self {
        self.is_dynamic = is_dyn;
        self
    }

    pub fn merchant(mut self, name: &str, city: &str, postal: Option<&str>) -> Self {
        self.merchant_name = name.to_uppercase();
        self.merchant_city = city.to_uppercase();
        self.postal_code = postal.map(|s| s.to_string());
        self
    }

    pub fn amount(mut self, nominal: f64) -> Self {
        self.amount = Some(nominal);
        self
    }

    pub fn invoice(mut self, inv: &str) -> Self {
        self.invoice_id = Some(inv.to_string());
        self
    }

    pub fn terminal(mut self, tid: &str) -> Self {
        self.terminal_id = Some(tid.to_string());
        self
    }

    /// Merakit String QRIS Lengkap dengan Tag 63 CRC16
    pub fn build(&self) -> String {
        let mut buffer = String::new();

        // 00: Payload Format Indicator
        buffer.push_str(&format_tlv("00", "01"));

        // 01: Point of Initiation (11 = Statis, 12 = Dinamis)
        let poi = if self.is_dynamic { "12" } else { "11" };
        buffer.push_str(&format_tlv("01", poi));

        // 26: Merchant Account Information (GPN/Acquirer)
        let mut sub26 = String::new();
        sub26.push_str(&format_tlv("00", "ID.CO.QRIS.WWW"));
        if let Some(pan) = &self.merchant_pan {
            sub26.push_str(&format_tlv("01", pan));
        }
        sub26.push_str(&format_tlv("02", "01"));
        sub26.push_str(&format_tlv("03", "UMI"));
        buffer.push_str(&format_tlv("26", &sub26));

        // 51: National Merchant ID (NMID)
        if let Some(nmid) = &self.nmid {
            let mut sub51 = String::new();
            sub51.push_str(&format_tlv("00", "ID.OR.GPN.WWW"));
            sub51.push_str(&format_tlv("02", nmid));
            buffer.push_str(&format_tlv("51", &sub51));
        }

        // 52: Merchant Category Code
        buffer.push_str(&format_tlv("52", &self.mcc));

        // 53: Transaction Currency (360 = IDR)
        buffer.push_str(&format_tlv("53", "360"));

        // 54: Transaction Amount (Jika Dinamis / Ada Nominal)
        if let Some(amt) = self.amount {
            let amt_str = format!("{:.0}", amt);
            buffer.push_str(&format_tlv("54", &amt_str));
        }

        // 58: Country Code
        buffer.push_str(&format_tlv("58", "ID"));

        // 59: Merchant Name
        buffer.push_str(&format_tlv("59", &self.merchant_name));

        // 60: Merchant City
        buffer.push_str(&format_tlv("60", &self.merchant_city));

        // 61: Postal Code
        if let Some(postal) = &self.postal_code {
            buffer.push_str(&format_tlv("61", postal));
        }

        // 62: Additional Data Field (Invoice / Terminal)
        let mut sub62 = String::new();
        if let Some(inv) = &self.invoice_id {
            sub62.push_str(&format_tlv("01", inv)); // Bill Number
            sub62.push_str(&format_tlv("05", inv)); // Reference ID
        }
        if let Some(tid) = &self.terminal_id {
            sub62.push_str(&format_tlv("07", tid)); // Terminal Label
        }
        if !sub62.is_empty() {
            buffer.push_str(&format_tlv("62", &sub62));
        }

        // 63: CRC16 Checksum
        buffer.push_str("6304");
        let crc = hitung_qris_crc16(&buffer);
        buffer.push_str(&crc);

        buffer
    }
}

/// Mengubah QRIS Statis menjadi QRIS Dinamis dengan menyuntikkan nominal dan ID Tagihan
pub fn konversi_statis_ke_dinamis(static_qris: &str, nominal: f64, no_tagihan: &str) -> Result<String, String> {
    let payload = parse_qris(static_qris)?;
    let mut builder = QrisBuilder::new()
        .dynamic(true)
        .merchant(&payload.merchant_name, &payload.merchant_city, payload.postal_code.as_deref())
        .amount(nominal)
        .invoice(no_tagihan);
    builder.mcc = payload.merchant_category_code;
    builder.nmid = payload.national_merchant_id;
    Ok(builder.build())
}
