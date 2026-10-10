//! Pure Sovereign Zero-Dependency Maritime AIS Decoder & Collision Avoidance (pure_ais.rs)
//!
//! 100% In-House Implementation:
//! - NMEA 0183 Sentence Parsing & Checksum XOR Verification
//! - 6-Bit Armored ASCII Bitstream Unpacking per ITU-R M.1371
//! - AIS Type 1, 2, 3 (Position Reports) & Type 5 (Static & Voyage Data) Decoders
//! - Maritime CPA (Closest Point of Approach) & TCPA (Time to CPA) Collision Prediction

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Status Navigasi Kapal (ITU-R M.1371)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigationalStatus {
    UnderWayUsingEngine,
    AtAnchor,
    NotUnderCommand,
    RestrictedManoeuvrability,
    ConstrainedByHerDraught,
    Moored,
    Aground,
    EngagedInFishing,
    UnderWaySailing,
    Other(u8),
}

impl NavigationalStatus {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::UnderWayUsingEngine,
            1 => Self::AtAnchor,
            2 => Self::NotUnderCommand,
            3 => Self::RestrictedManoeuvrability,
            4 => Self::ConstrainedByHerDraught,
            5 => Self::Moored,
            6 => Self::Aground,
            7 => Self::EngagedInFishing,
            8 => Self::UnderWaySailing,
            other => Self::Other(other),
        }
    }
}

/// Laporan Posisi AIS (Tipe 1, 2, dan 3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AisPositionReport {
    pub message_type: u8,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub nav_status: NavigationalStatus,
    pub rate_of_turn: i8,
    pub speed_over_ground_knots: f64,
    pub position_accuracy: bool,
    pub longitude: f64,
    pub latitude: f64,
    pub course_over_ground_deg: f64,
    pub true_heading_deg: Option<u16>,
    pub timestamp_sec: u8,
}

/// Data Statis & Perjalanan Kapal (Tipe 5)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AisStaticVoyageData {
    pub mmsi: u32,
    pub imo_number: u32,
    pub callsign: String,
    pub vessel_name: String,
    pub ship_type: u8,
    pub destination: String,
}

/// Reader Bitstream 6-bit Armored ASCII
pub struct AisBitReader {
    bits: Vec<bool>,
    cursor: usize,
}

impl AisBitReader {
    pub fn from_armored_ascii(payload: &str) -> Result<Self, String> {
        let mut bits = Vec::with_capacity(payload.len() * 6);
        for c in payload.chars() {
            let b = c as u8;
            if b < 48 || b > 119 || (b > 87 && b < 96) {
                return Err(format!("Karakter ASCII lapis 6-bit tidak valid: '{}'", c));
            }
            let mut val = b - 48;
            if val > 40 {
                val -= 8;
            }
            for i in (0..6).rev() {
                bits.push(((val >> i) & 1) != 0);
            }
        }
        Ok(Self { bits, cursor: 0 })
    }

    pub fn read_u32(&mut self, n_bits: usize) -> Result<u32, String> {
        if self.cursor + n_bits > self.bits.len() {
            return Err("Bitstream terpotong".to_string());
        }
        let mut res = 0u32;
        for _ in 0..n_bits {
            res = (res << 1) | (self.bits[self.cursor] as u32);
            self.cursor += 1;
        }
        Ok(res)
    }

    pub fn read_i32(&mut self, n_bits: usize) -> Result<i32, String> {
        let raw = self.read_u32(n_bits)?;
        let sign_mask = 1 << (n_bits - 1);
        if (raw & sign_mask) != 0 {
            let neg = (raw as i32) - (1 << n_bits);
            Ok(neg)
        } else {
            Ok(raw as i32)
        }
    }

    pub fn read_string(&mut self, n_chars: usize) -> Result<String, String> {
        let mut s = String::with_capacity(n_chars);
        for _ in 0..n_chars {
            let val = self.read_u32(6)? as u8;
            let ch = if val < 32 {
                (val + 64) as char
            } else {
                val as char
            };
            if ch != '@' {
                s.push(ch);
            }
        }
        Ok(s.trim().to_string())
    }
}

pub struct AisDecoder;

impl AisDecoder {
    /// Konstruktor helper laporan posisi AIS langsung
    pub fn decode_position_report(mmsi: u32, lat: f64, lon: f64, sog: f64, cog: f64, hdg: u16) -> AisPositionReport {
        AisPositionReport {
            message_type: 1,
            repeat_indicator: 0,
            mmsi,
            nav_status: NavigationalStatus::UnderWayUsingEngine,
            rate_of_turn: 0,
            speed_over_ground_knots: sog,
            position_accuracy: true,
            longitude: lon,
            latitude: lat,
            course_over_ground_deg: cog,
            true_heading_deg: Some(hdg),
            timestamp_sec: 35,
        }
    }

    /// Validasi checksum NMEA 0183 (XOR antar karakter antara '!' dan '*')
    pub fn verify_nmea_checksum(sentence: &str) -> bool {
        let trimmed = sentence.trim();
        if !trimmed.starts_with('!') && !trimmed.starts_with('$') {
            return false;
        }
        let parts: Vec<&str> = trimmed.split('*').collect();
        if parts.len() != 2 {
            return false;
        }

        let body = &parts[0][1..]; // Lewati prefix '!' atau '$'
        let mut xor_sum: u8 = 0;
        for b in body.as_bytes() {
            xor_sum ^= *b;
        }

        if let Ok(expected) = u8::from_str_radix(parts[1].trim(), 16) {
            xor_sum == expected
        } else {
            false
        }
    }

    /// Ekstraksi dan dekode kalimat NMEA AIVDM single-sentence
    pub fn decode_sentence(sentence: &str) -> Result<AisMessage, String> {
        if !Self::verify_nmea_checksum(sentence) {
            return Err("Checksum NMEA 0183 tidak cocok".to_string());
        }

        let clean = sentence.trim();
        let star_pos = clean.find('*').ok_or("Tidak ada tanda checksum '*")?;
        let payload_part = &clean[..star_pos];
        let fields: Vec<&str> = payload_part.split(',').collect();

        if fields.len() < 6 {
            return Err("Format kalimat NMEA AIVDM kurang dari 6 field".to_string());
        }

        let armored_payload = fields[5];
        let mut reader = AisBitReader::from_armored_ascii(armored_payload)?;
        let msg_type = reader.read_u32(6)? as u8;

        match msg_type {
            1 | 2 | 3 => {
                let repeat = reader.read_u32(2)? as u8;
                let mmsi = reader.read_u32(30)?;
                let status = reader.read_u32(4)? as u8;
                let rot = reader.read_i32(8)? as i8;
                let sog_raw = reader.read_u32(10)?;
                let sog = (sog_raw as f64) / 10.0;
                let accuracy = reader.read_u32(1)? == 1;

                let lon_raw = reader.read_i32(28)?;
                let lon = (lon_raw as f64) / 600000.0;

                let lat_raw = reader.read_i32(27)?;
                let lat = (lat_raw as f64) / 600000.0;

                let cog_raw = reader.read_u32(12)?;
                let cog = (cog_raw as f64) / 10.0;

                let hdg_raw = reader.read_u32(9)?;
                let true_heading = if hdg_raw <= 359 { Some(hdg_raw as u16) } else { None };
                let ts = reader.read_u32(6)? as u8;

                Ok(AisMessage::PositionReport(AisPositionReport {
                    message_type: msg_type,
                    repeat_indicator: repeat,
                    mmsi,
                    nav_status: NavigationalStatus::from_u8(status),
                    rate_of_turn: rot,
                    speed_over_ground_knots: sog,
                    position_accuracy: accuracy,
                    longitude: lon,
                    latitude: lat,
                    course_over_ground_deg: cog,
                    true_heading_deg: true_heading,
                    timestamp_sec: ts,
                }))
            }
            5 => {
                let _repeat = reader.read_u32(2)?;
                let mmsi = reader.read_u32(30)?;
                let _ais_ver = reader.read_u32(2)?;
                let imo = reader.read_u32(30)?;
                let callsign = reader.read_string(7)?;
                let name = reader.read_string(20)?;
                let ship_type = reader.read_u32(8)? as u8;
                let _dim = reader.read_u32(30)?;
                let _fix_type = reader.read_u32(4)?;
                let _eta = reader.read_u32(20)?;
                let _draught = reader.read_u32(8)?;
                let destination = reader.read_string(20)?;

                Ok(AisMessage::StaticVoyage(AisStaticVoyageData {
                    mmsi,
                    imo_number: imo,
                    callsign,
                    vessel_name: name,
                    ship_type,
                    destination,
                }))
            }
            other => Err(format!("Tipe pesan AIS {} belum didukung", other)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AisMessage {
    PositionReport(AisPositionReport),
    StaticVoyage(AisStaticVoyageData),
}

// ============================================================================
// 2. MARINE COLLISION PREDICTION ENGINE (CPA / TCPA)
// ============================================================================

/// Keadaan Navigasi Kinematik Kapal
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VesselKinematics {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub speed_knots: f64,
    pub course_deg: f64,
}

/// Hasil Perhitungan Closest Point of Approach (CPA)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CpaCalculation {
    pub cpa_distance_nm: f64,    // Jarak terdekat dalam Nautical Miles (NM)
    pub tcpa_minutes: f64,        // Waktu menuju CPA dalam Menit (negatif = sudah lewat)
    pub is_diverging: bool,
    pub is_collision_risk: bool,  // true jika CPA < safe_distance dan TCPA > 0
}

pub struct MarineKinematicsEngine;

impl MarineKinematicsEngine {
    /// Hitung CPA dan TCPA antara Kapal Sendiri (Own Ship) dan Kapal Target
    pub fn compute_cpa_tcpa(
        own: &VesselKinematics,
        target: &VesselKinematics,
        safe_distance_nm: f64,
        time_horizon_min: f64,
    ) -> CpaCalculation {
        let mean_lat_rad = ((own.lat_deg + target.lat_deg) / 2.0).to_radians();

        // 1. Hitung jarak relatif spasial dalam Nautical Miles (1 menit busur = 1 NM)
        let dx_nm = (target.lon_deg - own.lon_deg) * mean_lat_rad.cos() * 60.0;
        let dy_nm = (target.lat_deg - own.lat_deg) * 60.0;

        // 2. Hitung vektor kecepatan dalam knot (sumbu X: Timur, sumbu Y: Utara)
        let own_c_rad = own.course_deg.to_radians();
        let tgt_c_rad = target.course_deg.to_radians();

        let v_own_x = own.speed_knots * own_c_rad.sin();
        let v_own_y = own.speed_knots * own_c_rad.cos();

        let v_tgt_x = target.speed_knots * tgt_c_rad.sin();
        let v_tgt_y = target.speed_knots * tgt_c_rad.cos();

        let v_rel_x = v_tgt_x - v_own_x;
        let v_rel_y = v_tgt_y - v_own_y;

        let v_rel_sq = v_rel_x * v_rel_x + v_rel_y * v_rel_y;

        if v_rel_sq < 1e-6 {
            // Kecepatan sama persis (jarak relatif konstan)
            let current_dist = (dx_nm * dx_nm + dy_nm * dy_nm).sqrt();
            return CpaCalculation {
                cpa_distance_nm: current_dist,
                tcpa_minutes: 0.0,
                is_diverging: false,
                is_collision_risk: current_dist < safe_distance_nm,
            };
        }

        // 3. TCPA = - (d . v_rel) / |v_rel|^2 (dalam jam)
        let tcpa_hours = -(dx_nm * v_rel_x + dy_nm * v_rel_y) / v_rel_sq;
        let tcpa_minutes = tcpa_hours * 60.0;

        // 4. CPA = Posisi relatif saat t = TCPA
        let cpa_nm = if tcpa_hours > 0.0 {
            let cpa_x = dx_nm + v_rel_x * tcpa_hours;
            let cpa_y = dy_nm + v_rel_y * tcpa_hours;
            (cpa_x * cpa_x + cpa_y * cpa_y).sqrt()
        } else {
            // Jika sudah lewat (menjauh), CPA adalah jarak saat ini
            (dx_nm * dx_nm + dy_nm * dy_nm).sqrt()
        };

        let is_diverging = tcpa_minutes < 0.0;
        let is_risk = !is_diverging && cpa_nm <= safe_distance_nm && tcpa_minutes <= time_horizon_min;

        CpaCalculation {
            cpa_distance_nm: cpa_nm,
            tcpa_minutes,
            is_diverging,
            is_collision_risk: is_risk,
        }
    }
}
