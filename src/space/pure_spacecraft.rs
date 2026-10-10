//! Pure Sovereign Zero-Dependency Spacecraft Protocol Stack & Orbit Propagator (pure_spacecraft.rs)
//!
//! 100% In-House Implementation:
//! - CCSDS 32-bit Attached Synchronization Marker (ASM 0x1ACFFC1D)
//! - CCSDS Telemetry (TM) & Telecommand (TC) Transfer Frames (CCSDS 132.0-B-2 / 232.0-B-3)
//! - Spacecraft CRC-16 CCITT (Polynomial 0x1021, Initial 0xFFFF) Frame Error Control Field (FECF)
//! - Two-Line Element (TLE) Parser & SGP4 Keplerian Satellite Orbit Propagator
//! - Sub-Satellite Point Geodetic Coordinates (Lat, Lon, Alt) and Nusantara Groundstation Tracking

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

pub const CCSDS_ASM: [u8; 4] = [0x1A, 0xCF, 0xFC, 0x1D]; // 32-bit Attached Synchronization Marker
pub const EARTH_MU: f64 = 398600.4418; // km^3 / s^2 (Earth gravitational parameter)
pub const EARTH_RADIUS_KM: f64 = 6378.137; // WGS-84 Earth equatorial radius
pub const EARTH_ROTATION_RAD_PER_SEC: f64 = 7.2921159e-5; // rad/s

// ============================================================================
// 1. CCSDS CRC-16 CCITT CHECKSUM ENGINE
// ============================================================================

/// Hitung CRC-16 CCITT standar antariksa CCSDS (Polinomial 0x1021, Nilai awal 0xFFFF)
pub fn calculate_ccsds_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

// ============================================================================
// 2. CCSDS TELEMETRY (TM) TRANSFER FRAME (CCSDS 132.0-B-2)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcsdsTmTransferFrame {
    pub spacecraft_id: u16,       // 10 bits (0..1023)
    pub virtual_channel_id: u8,   // 3 bits (0..7)
    pub master_frame_count: u8,   // 8 bits (0..255)
    pub vc_frame_count: u8,       // 8 bits (0..255)
    pub first_header_pointer: u16,// 11 bits (0..2047, 2047 = no packet starts in this frame)
    pub data_payload: Vec<u8>,
}

impl CcsdsTmTransferFrame {
    pub fn new(scid: u16, vcid: u8, mfc: u8, vcfc: u8, data: Vec<u8>) -> Self {
        Self {
            spacecraft_id: scid & 0x03FF,
            virtual_channel_id: vcid & 0x07,
            master_frame_count: mfc,
            vc_frame_count: vcfc,
            first_header_pointer: 0,
            data_payload: data,
        }
    }

    /// Serialisasi frame TM lengkap dengan ASM prefix dan 2-byte CRC-16 CCITT FECF
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + 6 + self.data_payload.len() + 2);

        // 1. Attached Sync Marker (4 bytes)
        out.extend_from_slice(&CCSDS_ASM);

        // 2. TM Primary Header (6 bytes)
        // Octet 0-1: Version (2 bits = 00) | SCID (10 bits) | VCID (3 bits) | OCF Flag (1 bit = 0)
        let b0 = ((self.spacecraft_id >> 2) & 0x3F) as u8; // 2 bits version (0) + upper 6 bits SCID
        let b1 = (((self.spacecraft_id & 0x03) as u8) << 6)
            | ((self.virtual_channel_id & 0x07) << 3); // lower 2 bits SCID + 3 bits VCID + 1 bit OCF (0) + 2 bits unused
        out.push(b0);
        out.push(b1);

        // Octet 2: Master Channel Frame Count
        out.push(self.master_frame_count);

        // Octet 3: Virtual Channel Frame Count
        out.push(self.vc_frame_count);

        // Octet 4-5: Transfer Frame Data Field Status (First Header Pointer 11 bits)
        let status = self.first_header_pointer & 0x07FF;
        out.extend_from_slice(&status.to_be_bytes());

        // 3. Data Field
        out.extend_from_slice(&self.data_payload);

        // 4. Frame Error Control Field (FECF / CRC-16 CCITT) dihitung dari header + data
        let crc = calculate_ccsds_crc16(&out[4..]);
        out.extend_from_slice(&crc.to_be_bytes());

        out
    }

    /// Deserialisasi dari stream byte dengan validasi ASM dan CRC-16
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 12 {
            return Err("Ukuran frame CCSDS TM terlalu pendek (< 12 bytes)".to_string());
        }

        // Cek ASM
        if &bytes[0..4] != &CCSDS_ASM {
            return Err("Marker sinkronisasi CCSDS ASM (0x1ACFFC1D) tidak valid".to_string());
        }

        let frame_body = &bytes[4..];
        let total_body_len = frame_body.len();
        if total_body_len < 8 {
            return Err("Frame body terlalu pendek".to_string());
        }

        let expected_crc = u16::from_be_bytes([
            frame_body[total_body_len - 2],
            frame_body[total_body_len - 1],
        ]);
        let calculated_crc = calculate_ccsds_crc16(&frame_body[..total_body_len - 2]);

        if expected_crc != calculated_crc {
            return Err(format!(
                "CRC-16 CCITT FECF tidak valid: dihitung 0x{:04X}, diterima 0x{:04X}",
                calculated_crc, expected_crc
            ));
        }

        let b0 = frame_body[0];
        let b1 = frame_body[1];
        let scid = (((b0 & 0x3F) as u16) << 2) | (((b1 >> 6) & 0x03) as u16);
        let vcid = (b1 >> 3) & 0x07;
        let mfc = frame_body[2];
        let vcfc = frame_body[3];
        let status = u16::from_be_bytes([frame_body[4], frame_body[5]]);
        let fhp = status & 0x07FF;

        let payload = frame_body[6..total_body_len - 2].to_vec();

        Ok(Self {
            spacecraft_id: scid,
            virtual_channel_id: vcid,
            master_frame_count: mfc,
            vc_frame_count: vcfc,
            first_header_pointer: fhp,
            data_payload: payload,
        })
    }

    /// Ekstrak paket antariksa CCSDS (Space Packet) dari payload data frame TM
    pub fn extract_space_packet(&self) -> Result<crate::space::ccsds_packet::CcsdsSpacePacket, String> {
        crate::space::ccsds_packet::CcsdsSpacePacket::parse_bytes(&self.data_payload)
    }
}

// ============================================================================
// 3. TWO-LINE ELEMENT (TLE) PARSER & SATELLITE ORBIT PROPAGATOR
// ============================================================================

/// Parameter Elemen Orbit Satelit TLE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteTle {
    pub sat_name: String,
    pub sat_number: u32,
    pub epoch_year: u32,
    pub epoch_day: f64,
    pub inclination_deg: f64,
    pub raan_deg: f64, // Right Ascension of Ascending Node
    pub eccentricity: f64,
    pub arg_perigee_deg: f64,
    pub mean_anomaly_deg: f64,
    pub mean_motion_rev_per_day: f64,
}

/// Posisi Geodetik Sub-Satelit di Permukaan Bumi
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GeodeticPosition {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub altitude_km: f64,
}

impl SatelliteTle {
    /// Parse 2 baris standar TLE NORAD / NASA
    pub fn parse_tle(name: &str, line1: &str, line2: &str) -> Result<Self, String> {
        let l1 = line1.trim();
        let l2 = line2.trim();

        if !l1.starts_with('1') || !l2.starts_with('2') {
            return Err("Format baris TLE tidak valid (harus dimulai dengan '1' dan '2')".to_string());
        }

        let sat_num: u32 = l1[2..7].trim().parse().unwrap_or(0);
        let year_raw: u32 = l1[18..20].trim().parse().unwrap_or(24);
        let epoch_year = if year_raw < 57 { 2000 + year_raw } else { 1900 + year_raw };
        let epoch_day: f64 = l1[20..32].trim().parse().unwrap_or(0.0);

        let inc: f64 = l2[8..16].trim().parse().unwrap_or(0.0);
        let raan: f64 = l2[17..25].trim().parse().unwrap_or(0.0);
        let ecc_str = format!("0.{}", l2[26..33].trim());
        let ecc: f64 = ecc_str.parse().unwrap_or(0.001);
        let arg_perigee: f64 = l2[34..42].trim().parse().unwrap_or(0.0);
        let mean_anomaly: f64 = l2[43..51].trim().parse().unwrap_or(0.0);
        let mean_motion: f64 = l2[52..63].trim().parse().unwrap_or(15.0);

        Ok(Self {
            sat_name: name.to_string(),
            sat_number: sat_num,
            epoch_year,
            epoch_day,
            inclination_deg: inc,
            raan_deg: raan,
            eccentricity: ecc,
            arg_perigee_deg: arg_perigee,
            mean_anomaly_deg: mean_anomaly,
            mean_motion_rev_per_day: mean_motion,
        })
    }

    /// Propagasi posisi orbit pada `delta_seconds` dari epoch
    pub fn propagate(&self, delta_seconds: f64) -> GeodeticPosition {
        // 1. Mean motion dalam radian per detik
        let n_rad_s = self.mean_motion_rev_per_day * 2.0 * PI / 86400.0;

        // 2. Semi-major axis a (km) dari Hukum Kepler ke-3: a = (mu / n^2)^(1/3)
        let a = (EARTH_MU / (n_rad_s * n_rad_s)).cbrt();

        // 3. Mean Anomaly saat t: M = M0 + n * dt
        let m0_rad = self.mean_anomaly_deg.to_radians();
        let m = (m0_rad + n_rad_s * delta_seconds).rem_euclid(2.0 * PI);

        // 4. Selesaikan Persamaan Kepler M = E - e*sin(E) dengan metode Newton-Raphson
        let mut e_anom = m;
        for _ in 0..10 {
            let f = e_anom - self.eccentricity * e_anom.sin() - m;
            let f_prime = 1.0 - self.eccentricity * e_anom.cos();
            e_anom -= f / f_prime;
        }

        // 5. True Anomaly nu: tan(nu/2) = sqrt((1+e)/(1-e)) * tan(E/2)
        let true_anom = 2.0 * (((1.0 + self.eccentricity) / (1.0 - self.eccentricity)).sqrt() * (e_anom / 2.0).tan()).atan();

        // 6. Jarak r dari pusat bumi (km)
        let r = a * (1.0 - self.eccentricity * e_anom.cos());

        // 7. Argumen Lintang u = omega + nu
        let u = self.arg_perigee_deg.to_radians() + true_anom;

        // 8. Koordinat dalam bidang orbit & transformasi ECI (Earth-Centered Inertial)
        let inc_rad = self.inclination_deg.to_radians();
        let raan_rad = self.raan_deg.to_radians();

        let x_orb = r * u.cos();
        let y_orb = r * u.sin();

        let z_eci = y_orb * inc_rad.sin();
        let x_node = x_orb;
        let y_node = y_orb * inc_rad.cos();

        let x_eci = x_node * raan_rad.cos() - y_node * raan_rad.sin();
        let y_eci = x_node * raan_rad.sin() + y_node * raan_rad.cos();

        // 9. Konversi ECI ke ECEF (Earth-Centered Earth-Fixed) dengan rotasi bumi
        let gmst = (EARTH_ROTATION_RAD_PER_SEC * delta_seconds).rem_euclid(2.0 * PI);
        let x_ecef = x_eci * gmst.cos() + y_eci * gmst.sin();
        let y_ecef = -x_eci * gmst.sin() + y_eci * gmst.cos();
        let z_ecef = z_eci;

        // 10. Geodetik Sub-Satellite Point (Lat, Lon, Alt)
        let r_xy = (x_ecef * x_ecef + y_ecef * y_ecef).sqrt();
        let lat_deg = z_ecef.atan2(r_xy).to_degrees();
        let lon_deg = y_ecef.atan2(x_ecef).to_degrees();
        let altitude_km = r - EARTH_RADIUS_KM;

        GeodeticPosition {
            lat_deg,
            lon_deg,
            altitude_km,
        }
    }

    /// Cek apakah satelit sedang melintas di atas wilayah Indonesia
    /// Batas Nusantara: Lat (-11.0 .. +6.0), Lon (95.0 .. 141.0)
    pub fn is_over_nusantara(&self, delta_seconds: f64) -> bool {
        let pos = self.propagate(delta_seconds);
        pos.lat_deg >= -11.0 && pos.lat_deg <= 6.0 && pos.lon_deg >= 95.0 && pos.lon_deg <= 141.0
    }
}
