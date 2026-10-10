//! Pure Sovereign Zero-Dependency Precision Agriculture & ISOBUS Engine (pure_agri.rs)
//!
//! 100% In-House Implementation:
//! - Drone Multispectral Vegetation Indices: NDVI, NDRE, SAVI, and EVI
//! - Canopy Chlorophyll & Water Stress Vigor Classifier
//! - Variable Rate Application (VRA) Prescription Map Synthesizer
//! - ISO 11783-10 (ISOBUS) Task Controller Binary Grid (TLG.BIN) & ISO-XML Engine
//! - Tractor Telemetry & Precision Farming Guidance

use serde::{Deserialize, Serialize};

// ============================================================================
// 1. MULTISPECTRAL CANOPY VEGETATION INDICES
// ============================================================================

/// Sampel Reflektansi Multispektral 5-Kanal (Drone / Satelit Sentinel-2)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MultispectralSample {
    pub blue: f64,     // ~475 nm
    pub green: f64,    // ~560 nm
    pub red: f64,      // ~668 nm
    pub red_edge: f64, // ~717 nm
    pub nir: f64,      // ~840 nm (Near Infrared)
}

/// Kategori Kesehatan & Kebugaran Kanopi Tanaman
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanopyVigorCategory {
    DenseAndHealthy,     // NDVI > 0.65: Kanopi lebat & fotosintesis optimal
    ModerateGrowth,      // NDVI 0.40..0.65: Pertumbuhan vegetatif aktif
    EarlyGrowthOrSparse, // NDVI 0.20..0.40: Tanaman muda / kanopi jarang
    StressedOrDeficient, // NDVI 0.05..0.20: Cekaman air / defisiensi nitrogen berat
    BarrenSoilOrWater,   // NDVI <= 0.05: Tanah terbuka / genangan air
}

/// Indeks Vegetasi Multispektral Terhitung
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VegetationIndices {
    pub ndvi: f64, // Normalized Difference Vegetation Index: [-1.0 .. 1.0]
    pub ndre: f64, // Normalized Difference Red Edge Index: [-1.0 .. 1.0]
    pub savi: f64, // Soil Adjusted Vegetation Index (L = 0.5)
    pub evi: f64,  // Enhanced Vegetation Index
    pub category: CanopyVigorCategory,
}

pub struct MultispectralEngine;

impl MultispectralEngine {
    /// Hitung seluruh indeks vegetasi kanopi dari sampel spektral
    pub fn compute_indices(sample: &MultispectralSample) -> VegetationIndices {
        // 1. NDVI: (NIR - Red) / (NIR + Red)
        let ndvi_denom = sample.nir + sample.red;
        let ndvi = if ndvi_denom.abs() > 1e-6 {
            ((sample.nir - sample.red) / ndvi_denom).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        // 2. NDRE: (NIR - RedEdge) / (NIR + RedEdge) (sensitif pada kanopi lebat jenuh)
        let ndre_denom = sample.nir + sample.red_edge;
        let ndre = if ndre_denom.abs() > 1e-6 {
            ((sample.nir - sample.red_edge) / ndre_denom).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        // 3. SAVI (Soil Adjusted, L = 0.5): ((NIR - Red) * 1.5) / (NIR + Red + 0.5)
        let savi_l = 0.5;
        let savi_denom = sample.nir + sample.red + savi_l;
        let savi = if savi_denom.abs() > 1e-6 {
            (((sample.nir - sample.red) * (1.0 + savi_l)) / savi_denom).clamp(-1.0, 1.5)
        } else {
            0.0
        };

        // 4. EVI: 2.5 * (NIR - Red) / (NIR + 6*Red - 7.5*Blue + 1.0)
        let evi_denom = sample.nir + 6.0 * sample.red - 7.5 * sample.blue + 1.0;
        let evi = if evi_denom.abs() > 1e-6 {
            (2.5 * (sample.nir - sample.red) / evi_denom).clamp(-1.0, 2.0)
        } else {
            0.0
        };

        // Klasifikasi Kanopi
        let category = if ndvi > 0.65 {
            CanopyVigorCategory::DenseAndHealthy
        } else if ndvi > 0.40 {
            CanopyVigorCategory::ModerateGrowth
        } else if ndvi > 0.20 {
            CanopyVigorCategory::EarlyGrowthOrSparse
        } else if ndvi > 0.05 {
            CanopyVigorCategory::StressedOrDeficient
        } else {
            CanopyVigorCategory::BarrenSoilOrWater
        };

        VegetationIndices {
            ndvi,
            ndre,
            savi,
            evi,
            category,
        }
    }

    /// Sintesis dosis pemupukan Nitrogen presisi (kg/ha) berdasarkan NDVI
    /// - Tanaman sehat lebat butuh dosis pemeliharaan (misal 50 kg/ha)
    /// - Tanaman dengan pertumbuhan sedang/stres defisiensi butuh pemulihan (misal 150 kg/ha)
    /// - Tanah kosong/tanpa tanaman tidak diberi pupuk (0 kg/ha)
    pub fn recommend_nitrogen_rate_kg_ha(ndvi: f64) -> f64 {
        if ndvi <= 0.10 {
            0.0 // Tidak ada tanaman
        } else if ndvi <= 0.35 {
            160.0 // Defisiensi tinggi, butuh pupuk intensif
        } else if ndvi <= 0.60 {
            110.0 // Pertumbuhan aktif, dosis sedang
        } else {
            50.0 // Kanopi sudah sangat subur, hindari lodging / pemupukan berlebih
        }
    }
}

// ============================================================================
// 2. ISO 11783-10 (ISOBUS) TASK CONTROLLER ENGINE
// ============================================================================

/// Tipe Elemen Tugas Pertanian ISO-XML
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsoXmlTaskStatus {
    Planned,
    Running,
    Paused,
    Completed,
}

/// Deskriptor Tugas Pertanian Presisi (ISO-XML Part 10 Task Data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsoTaskData {
    pub task_id: String,           // e.g. "TSK1"
    pub task_designator: String,   // e.g. "Pemupukan Presisi Jagung Blok A"
    pub customer_id: String,       // e.g. "CTR1" (Kelompok Tani / Koperasi)
    pub farm_id: String,           // e.g. "FRM1"
    pub partfield_id: String,      // e.g. "PFD1" (Kode Lahan)
    pub status: IsoXmlTaskStatus,
    pub target_rate_ddi: u16,      // 0x0006 = Actual / Target Work Rate (mg/m2 or kg/ha)
    pub default_rate: f64,
}

impl IsoTaskData {
    pub fn new(task_id: &str, designator: &str, partfield: &str, default_n_rate: f64) -> Self {
        Self {
            task_id: task_id.to_string(),
            task_designator: designator.to_string(),
            customer_id: "CTR_NUSANTARA".to_string(),
            farm_id: "FRM_NUSANTARA_01".to_string(),
            partfield_id: partfield.to_string(),
            status: IsoXmlTaskStatus::Planned,
            target_rate_ddi: 0x0006,
            default_rate: default_n_rate,
        }
    }

    /// Serialisasi ringkasan dokumen ISO-XML `TASKDATA.XML`
    pub fn to_iso_xml(&self) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
            <ISO11783_TaskData VersionMajor=\"4\" VersionMinor=\"2\" DataTransferLogo=\"1\">\n  \
              <CTR A=\"{}\" B=\"Pusat Pertanian Presisi\"/>\n  \
              <FRM A=\"{}\" B=\"Kebun Mandiri Nusantara\"/>\n  \
              <PFD A=\"{}\" C=\"Lahan Pertanian Berdaulat\"/>\n  \
              <TSK A=\"{}\" B=\"{}\" G=\"{:?}\">\n    \
                <TZN A=\"1\">\n      \
                  <PDV A=\"{:04X}\" B=\"{:.2}\"/>\n    \
                </TZN>\n  \
              </TSK>\n\
            </ISO11783_TaskData>",
            self.customer_id,
            self.farm_id,
            self.partfield_id,
            self.task_id,
            self.task_designator,
            self.status,
            self.target_rate_ddi,
            self.default_rate,
        )
    }
}

/// Kisi Preskripsi Raster Biner ISOBUS (File Format ISO 11783-10 TLG*.BIN)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsobusBinaryGrid {
    pub grid_type: u8,           // 1 = Grid Type 1 (2-byte cell entries), 2 = Grid Type 2 (4-byte)
    pub cell_width_mm: u32,      // Lebar sel dalam milimeter
    pub cell_height_mm: u32,     // Tinggi sel dalam milimeter
    pub columns: u16,
    pub rows: u16,
    pub rates_kg_ha: Vec<f32>,   // Array 1D baris-major (rows * columns)
}

impl IsobusBinaryGrid {
    pub fn new(cols: u16, rows: u16, cell_size_mm: u32, default_rate: f32) -> Self {
        let total = (cols as usize) * (rows as usize);
        Self {
            grid_type: 1, // 16-bit integer representation (10x scaling)
            cell_width_mm: cell_size_mm,
            cell_height_mm: cell_size_mm,
            columns: cols,
            rows,
            rates_kg_ha: vec![default_rate; total],
        }
    }

    #[inline]
    pub fn set_cell(&mut self, col: u16, row: u16, rate: f32) {
        if col < self.columns && row < self.rows {
            let idx = (row as usize) * (self.columns as usize) + (col as usize);
            self.rates_kg_ha[idx] = rate;
        }
    }

    #[inline]
    pub fn get_cell(&self, col: u16, row: u16) -> Option<f32> {
        if col < self.columns && row < self.rows {
            let idx = (row as usize) * (self.columns as usize) + (col as usize);
            Some(self.rates_kg_ha[idx])
        } else {
            None
        }
    }

    /// Serialisasi ke format biner spesifikasi ISOBUS TLG.BIN
    pub fn encode_binary(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(16 + self.rates_kg_ha.len() * 2);

        // Header (12 bytes):
        // Type (1B), Reserved (3B), CellWidth (4B LE), CellHeight (4B LE)
        out.push(self.grid_type);
        out.extend_from_slice(&[0, 0, 0]); // Reserved
        out.extend_from_slice(&self.cell_width_mm.to_le_bytes());
        out.extend_from_slice(&self.cell_height_mm.to_le_bytes());

        // Dimensions (4 bytes): Columns (2B LE), Rows (2B LE)
        out.extend_from_slice(&self.columns.to_le_bytes());
        out.extend_from_slice(&self.rows.to_le_bytes());

        // Cell Values (2 bytes per cell, scaled x10 integer: 120.5 kg/ha -> 1205)
        for &r in &self.rates_kg_ha {
            let raw_val = (r * 10.0).clamp(0.0, 65535.0) as u16;
            out.extend_from_slice(&raw_val.to_le_bytes());
        }

        out
    }

    /// Deserialisasi dari buffer biner ISOBUS TLG.BIN
    pub fn parse_binary(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 16 {
            return Err("Ukuran berkas terlalu pendek untuk header ISOBUS TLG.BIN (< 16 bytes)".to_string());
        }

        let grid_type = bytes[0];
        let cell_width = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let cell_height = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let cols = u16::from_le_bytes([bytes[12], bytes[13]]);
        let rows = u16::from_le_bytes([bytes[14], bytes[15]]);

        let expected_cells = (cols as usize) * (rows as usize);
        let expected_bytes = 16 + expected_cells * 2;
        if bytes.len() < expected_bytes {
            return Err(format!(
                "Ukuran data grid tidak mencukupi (diharapkan {} bytes, ditemukan {})",
                expected_bytes, bytes.len()
            ));
        }

        let mut rates = Vec::with_capacity(expected_cells);
        let mut offset = 16;
        for _ in 0..expected_cells {
            let val = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            rates.push((val as f32) / 10.0);
            offset += 2;
        }

        Ok(Self {
            grid_type,
            cell_width_mm: cell_width,
            cell_height_mm: cell_height,
            columns: cols,
            rows,
            rates_kg_ha: rates,
        })
    }
}

// ============================================================================
// 3. BRIDGES DENGAN SUBSISTEM TASK CONTROLLER LEGACY
// ============================================================================

impl From<IsobusBinaryGrid> for crate::agri::task_controller::TaskControllerGeo {
    fn from(grid: IsobusBinaryGrid) -> Self {
        let mut tc = crate::agri::task_controller::TaskControllerGeo::new();
        // Konversi setiap sel grid menjadi prescription zone geografis estimasi
        let lat_step = (grid.cell_height_mm as f64) / 111_000_000.0; // ~111 km per deg
        let lon_step = (grid.cell_width_mm as f64) / 111_000_000.0;

        let base_lat = -7.0;
        let base_lon = 110.0;

        for r in 0..grid.rows {
            for c in 0..grid.columns {
                if let Some(rate) = grid.get_cell(c, r) {
                    let min_lat = base_lat + (r as f64) * lat_step;
                    let max_lat = min_lat + lat_step;
                    let min_lon = base_lon + (c as f64) * lon_step;
                    let max_lon = min_lon + lon_step;

                    tc.add_zone(min_lat, max_lat, min_lon, max_lon, rate as f64, 25.0);
                }
            }
        }
        tc
    }
}

