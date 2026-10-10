//! Unit & Integration Test: WidyaAgri Pure Multispectral Drone & ISOBUS Engine
//! Memverifikasi kalkulasi indeks vegetasi kanopi (NDVI, NDRE, SAVI, EVI),
//! klasifikasi vigor tanaman, rekomendasi dosis pupuk N,
//! serialisasi ISO-XML TaskData, serta enkoding/dekoding biner TLG.BIN ISOBUS.

use widya::agri::pure_agri::{
    CanopyVigorCategory, IsoTaskData, IsoXmlTaskStatus, IsobusBinaryGrid, MultispectralEngine,
    MultispectralSample,
};
use widya::agri::TaskControllerGeo;

#[test]
fn test_multispectral_vegetation_indices_and_vigor_classification() {
    // 1. Sampel Kanopi Lebat & Sehat (Dense And Healthy)
    // Reflektansi NIR tinggi (~0.75), Red rendah (~0.05), RedEdge (~0.35), Blue (~0.03)
    let sample_healthy = MultispectralSample {
        blue: 0.03,
        green: 0.12,
        red: 0.05,
        red_edge: 0.35,
        nir: 0.75,
    };
    let indices_healthy = MultispectralEngine::compute_indices(&sample_healthy);

    // NDVI = (0.75 - 0.05) / (0.75 + 0.05) = 0.70 / 0.80 = 0.875
    assert!((indices_healthy.ndvi - 0.875).abs() < 1e-3);
    assert_eq!(indices_healthy.category, CanopyVigorCategory::DenseAndHealthy);
    assert!(indices_healthy.ndre > 0.30);
    assert!(indices_healthy.savi > 0.60);
    assert!(indices_healthy.evi > 0.80);

    // Dosis pupuk Nitrogen pemeliharaan untuk kanopi subur
    let rate_healthy = MultispectralEngine::recommend_nitrogen_rate_kg_ha(indices_healthy.ndvi);
    assert_eq!(rate_healthy, 50.0);

    // 2. Sampel Tanaman Cekaman / Defisiensi Tinggi (Stressed Or Deficient)
    // Reflektansi NIR rendah (~0.22), Red agak tinggi (~0.18)
    let sample_stressed = MultispectralSample {
        blue: 0.06,
        green: 0.08,
        red: 0.18,
        red_edge: 0.20,
        nir: 0.22,
    };
    let indices_stressed = MultispectralEngine::compute_indices(&sample_stressed);

    // NDVI = (0.22 - 0.18) / (0.22 + 0.18) = 0.04 / 0.40 = 0.10
    assert!((indices_stressed.ndvi - 0.10).abs() < 1e-3);
    assert_eq!(indices_stressed.category, CanopyVigorCategory::StressedOrDeficient);

    // Dosis pupuk Nitrogen tinggi untuk pemulihan tanaman stres
    let rate_stressed = MultispectralEngine::recommend_nitrogen_rate_kg_ha(indices_stressed.ndvi);
    assert_eq!(rate_stressed, 160.0);

    // 3. Sampel Tanah Kosong / Air
    let sample_barren = MultispectralSample {
        blue: 0.05,
        green: 0.05,
        red: 0.05,
        red_edge: 0.05,
        nir: 0.05,
    };
    let indices_barren = MultispectralEngine::compute_indices(&sample_barren);
    assert_eq!(indices_barren.category, CanopyVigorCategory::BarrenSoilOrWater);
    let rate_barren = MultispectralEngine::recommend_nitrogen_rate_kg_ha(indices_barren.ndvi);
    assert_eq!(rate_barren, 0.0);
}

#[test]
fn test_isobus_binary_grid_roundtrip() {
    let cols = 3;
    let rows = 2;
    let cell_size_mm = 10_000; // 10 meter grid

    let mut grid = IsobusBinaryGrid::new(cols, rows, cell_size_mm, 50.0);
    grid.set_cell(0, 0, 120.5); // Sel (0,0) diberi 120.5 kg/ha
    grid.set_cell(1, 0, 85.0);
    grid.set_cell(2, 0, 0.0);
    grid.set_cell(0, 1, 160.0);
    grid.set_cell(1, 1, 110.2);
    grid.set_cell(2, 1, 75.8);

    assert_eq!(grid.get_cell(0, 0), Some(120.5));
    assert_eq!(grid.get_cell(1, 1), Some(110.2));
    assert_eq!(grid.get_cell(5, 5), None);

    // Enkoding ke biner TLG.BIN
    let bin = grid.encode_binary();
    assert_eq!(bin.len(), 16 + (cols as usize * rows as usize * 2));
    assert_eq!(bin[0], 1); // Grid Type 1

    // Dekoding kembali
    let parsed = IsobusBinaryGrid::parse_binary(&bin).expect("Parse ISOBUS TLG.BIN failed");
    assert_eq!(parsed.columns, 3);
    assert_eq!(parsed.rows, 2);
    assert_eq!(parsed.cell_width_mm, 10_000);
    assert_eq!(parsed.cell_height_mm, 10_000);

    // Verifikasi nilai rate dengan toleransi desimal 0.1
    let val_0_0 = parsed.get_cell(0, 0).unwrap();
    assert!((val_0_0 - 120.5).abs() < 0.15);
    let val_1_1 = parsed.get_cell(1, 1).unwrap();
    assert!((val_1_1 - 110.2).abs() < 0.15);
}

#[test]
fn test_iso_xml_task_data_generation() {
    let task = IsoTaskData::new(
        "TSK_JAGUNG_01",
        "Aplikasi Pupuk VRA Blok Selatan",
        "PFD_NUSANTARA_01",
        125.0,
    );

    let xml = task.to_iso_xml();
    assert!(xml.contains("ISO11783_TaskData"));
    assert!(xml.contains("TSK_JAGUNG_01"));
    assert!(xml.contains("Aplikasi Pupuk VRA Blok Selatan"));
    assert!(xml.contains("125.00"));
}

#[test]
fn test_agri_legacy_task_controller_bridge() {
    let mut grid = IsobusBinaryGrid::new(2, 2, 5_000, 100.0);
    grid.set_cell(0, 0, 140.0);
    grid.set_cell(1, 1, 80.0);

    let geo_tc: TaskControllerGeo = grid.into();
    // Verifikasi bahwa zona preskripsi terbentuk
    assert_eq!(geo_tc.prescription_map.len(), 4);
    assert_eq!(geo_tc.prescription_map[0].target_nitrogen_kg_ha, 140.0);
}
