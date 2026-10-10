// Projects/arenasilat/src/kategori_and_drawing.rs
// ==============================================================================
// ARENA SILAT SAAS — PEMBAGIAN KELAS & DRAWING KONTINGEN (IPSI 2026)
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::pendaftaran::{AtletRegistrasi, KategoriUsia, JenisKelamin, JenisKompetisi};
use super::bracket_generator::AtletPeserta;

/// Rentang Berat Badan Kelas Tanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KelasTandingDefinisi {
    pub kode_kelas: String, // "A", "B", "C", "D", "E", "Bebas"
    pub min_kg: f32,
    pub max_kg: f32,
    pub kategori_usia: KategoriUsia,
    pub jenis_kelamin: JenisKelamin,
}

/// Mesin Klasifikasi Kategori & Pembagian Pool Pertandingan
pub struct CategoryDrawingEngine;

impl CategoryDrawingEngine {
    /// Dapatkan Definisi Kelas Tanding Dewasa Putra Resmi IPSI
    pub fn kelas_dewasa_putra() -> Vec<KelasTandingDefinisi> {
        vec![
            KelasTandingDefinisi { kode_kelas: "A".to_string(), min_kg: 45.0, max_kg: 50.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "B".to_string(), min_kg: 50.0, max_kg: 55.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "C".to_string(), min_kg: 55.0, max_kg: 60.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "D".to_string(), min_kg: 60.0, max_kg: 65.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "E".to_string(), min_kg: 65.0, max_kg: 70.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "F".to_string(), min_kg: 70.0, max_kg: 75.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "G".to_string(), min_kg: 75.0, max_kg: 80.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "H".to_string(), min_kg: 80.0, max_kg: 85.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "I".to_string(), min_kg: 85.0, max_kg: 90.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "J".to_string(), min_kg: 90.0, max_kg: 95.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
            KelasTandingDefinisi { kode_kelas: "Bebas".to_string(), min_kg: 95.0, max_kg: 140.0, kategori_usia: KategoriUsia::Dewasa, jenis_kelamin: JenisKelamin::Putra },
        ]
    }

    /// Evaluasi Kesesuaian Berat Badan Atlet dengan Kelas Tanding yang Didaftarkan
    pub fn validasi_berat_badan(berat_aktual: f32, min_kg: f32, max_kg: f32) -> bool {
        // Berdasarkan regulasi resmi IPSI 2026: Toleransi Timbangan adalah 0.00 kg
        berat_aktual >= min_kg && berat_aktual <= max_kg
    }

    /// Kelompokkan Daftar Atlet Terverifikasi ke dalam Kategori Kelas
    pub fn kelompokkan_atlet(atlet_list: &[AtletRegistrasi]) -> HashMap<String, Vec<AtletPeserta>> {
        let mut grouped: HashMap<String, Vec<AtletPeserta>> = HashMap::new();

        for a in atlet_list {
            if !a.terverifikasi {
                continue;
            }

            let key = match &a.kompetisi {
                JenisKompetisi::Tanding(kelas) => {
                    format!("{:?} - {:?} - {}", a.kategori_usia, a.jenis_kelamin, kelas)
                },
                JenisKompetisi::SeniTunggal => {
                    format!("{:?} - {:?} - Seni Tunggal", a.kategori_usia, a.jenis_kelamin)
                },
                JenisKompetisi::SeniGanda => {
                    format!("{:?} - {:?} - Seni Ganda", a.kategori_usia, a.jenis_kelamin)
                },
                JenisKompetisi::SeniRegu => {
                    format!("{:?} - {:?} - Seni Regu", a.kategori_usia, a.jenis_kelamin)
                },
                JenisKompetisi::SoloKreatif => {
                    format!("{:?} - {:?} - Solo Kreatif", a.kategori_usia, a.jenis_kelamin)
                },
            };

            let peserta = AtletPeserta {
                id: a.id.clone(),
                nama: a.nama_lengkap.clone(),
                kontingen: a.kontingen_id.clone(),
                kelas_tanding: key.clone(),
                berat_badan_kg: a.berat_badan_kg,
                unggulan_seed: a.seed_unggulan,
            };

            grouped.entry(key).or_insert_with(Vec::new).push(peserta);
        }

        grouped
    }

    /// Lakukan Pengundian / Drawing Adil dengan Pemisahan Kontingen (Fair Contingent Drawing)
    pub fn lakukan_drawing_kategori(daftar_peserta: &[AtletPeserta]) -> (Vec<AtletPeserta>, Vec<AtletPeserta>) {
        let mut pool_atas: Vec<AtletPeserta> = Vec::new();
        let mut pool_bawah: Vec<AtletPeserta> = Vec::new();
        let mut tracker_kontingen: HashMap<String, u32> = HashMap::new();

        // 1. Tempatkan unggulan seed 1 (Pool Atas) dan seed 2 (Pool Bawah)
        for p in daftar_peserta {
            if let Some(seed) = p.unggulan_seed {
                if seed == 1 {
                    pool_atas.push(p.clone());
                    *tracker_kontingen.entry(p.kontingen.clone()).or_insert(0) += 1;
                } else if seed == 2 {
                    pool_bawah.push(p.clone());
                    *tracker_kontingen.entry(p.kontingen.clone()).or_insert(0) += 1;
                }
            }
        }

        // 2. Distribusikan sisa peserta secara bergantian dengan memperhatikan kontingen
        for p in daftar_peserta {
            if p.unggulan_seed == Some(1) || p.unggulan_seed == Some(2) {
                continue;
            }

            let hitung = tracker_kontingen.entry(p.kontingen.clone()).or_insert(0);
            if *hitung % 2 == 0 {
                pool_atas.push(p.clone());
            } else {
                pool_bawah.push(p.clone());
            }
            *hitung += 1;
        }

        (pool_atas, pool_bawah)
    }
}
