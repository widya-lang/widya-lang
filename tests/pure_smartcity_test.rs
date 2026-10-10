//! Unit & Integration Test: WidyaSmartCity Pure Urban Geospatial & Systems Engine
//! Memverifikasi kalkulasi debit puncak limpasan banjir (metode Rasional),
//! optimasi durasi siklus dan lampu hijau lalu lintas Webster,
//! potensi pembangkitan energi PLTS Atap (Rooftop Solar PV), serta jembatan koordinat GIS.

use widya::smartcity::pure_smartcity::{
    LandCoverType, RooftopSolarEngine, TrafficApproachPhase, UrbanCatchmentZone,
    UrbanHydrologyEngine, WebsterSignalOptimizer,
};
use widya::smartcity::TitikKoordinat;

#[test]
fn test_urban_hydrology_flood_runoff_and_retention_basin() {
    // Zona Perkotaan Komersial & Jalan Aspal (Luas 2.0 km², C = 0.90)
    let catchment = UrbanCatchmentZone {
        zone_id: "ZONE_KEMANG_01".to_string(),
        area_km2: 2.0,
        land_cover: LandCoverType::AsphaltRoadOrRoof,
        elevation_m: 12.5,
    };

    // Hujan lebat intensitas 50 mm/jam
    // Q = 0.278 * 0.90 * 50 * 2.0 = 25.02 m³/detik
    let peak_q = UrbanHydrologyEngine::calculate_peak_runoff_m3_s(&catchment, 50.0);
    assert!((peak_q - 25.02).abs() < 1e-2);

    // Kapasitas pompa pembuangan = 15.0 m³/detik
    // Selisih debit masuk - keluar = 25.02 - 15.0 = 10.02 m³/detik
    // Durasi hujan = 2 jam (7200 detik)
    // Volume kolam retensi yang dibutuhkan = 10.02 * 7200 = 72,144 m³
    let ret_vol = UrbanHydrologyEngine::calculate_retention_volume_m3(peak_q, 15.0, 2.0);
    assert!((ret_vol - 72_144.0).abs() < 10.0);
}

#[test]
fn test_webster_smart_traffic_signal_optimization() {
    let optimizer = WebsterSignalOptimizer::new(4.0); // 4 detik waktu hilang per fase

    // Simpang 4 Lengan: Utara-Selatan (Fase 1) dan Timur-Barat (Fase 2)
    let phases = vec![
        TrafficApproachPhase {
            approach_name: "Utara-Selatan".to_string(),
            actual_flow_veh_per_hr: 900.0,
            saturation_flow_veh_per_hr: 2400.0, // y1 = 900/2400 = 0.375
        },
        TrafficApproachPhase {
            approach_name: "Timur-Barat".to_string(),
            actual_flow_veh_per_hr: 600.0,
            saturation_flow_veh_per_hr: 2000.0, // y2 = 600/2000 = 0.300
        },
    ];

    // Y = 0.375 + 0.300 = 0.675
    // L = 2 fase * 4 detik = 8 detik
    // C_0 = (1.5 * 8 + 5) / (1 - 0.675) = (12 + 5) / 0.325 = 17 / 0.325 ≈ 52.3 detik
    let c_opt = optimizer.calculate_optimum_cycle_time(&phases).expect("Perhitungan siklus harus berhasil");
    assert!((c_opt - 52.3).abs() < 0.5);

    // Alokasi lampu hijau:
    // Waktu hijau efektif = 52.3 - 8 = 44.3 detik
    // g1 = (0.375 / 0.675) * 44.3 ≈ 24.61 detik
    // g2 = (0.300 / 0.675) * 44.3 ≈ 19.69 detik
    let splits = optimizer.allocate_green_splits(&phases, c_opt);
    assert_eq!(splits.len(), 2);
    assert!((splits[0] - 24.61).abs() < 0.5);
    assert!((splits[1] - 19.69).abs() < 0.5);
    assert!((splits[0] + splits[1] - (c_opt - 8.0)).abs() < 1e-2);
}

#[test]
fn test_rooftop_solar_pv_yield_and_carbon_offset() {
    // Luas atap gedung perkantoran = 500 m²
    // Radiasi matahari rata-rata = 4.8 kWh/m²/hari
    // Efisiensi panel surya = 21% (0.21)
    // Performance ratio (PR) = 80% (0.80)
    // E = 500 * 4.8 * 0.21 * 0.80 = 403.2 kWh/hari
    let daily_yield = RooftopSolarEngine::estimate_daily_pv_yield_kwh(500.0, 4.8, 0.21, 0.80);
    assert!((daily_yield - 403.2).abs() < 0.1);

    // Pengurangan emisi CO2 per tahun:
    // 403.2 * 365 hari * 0.78 kg/kWh = 147,168 * 0.78 ≈ 114,791 kg CO2e
    let annual_co2 = RooftopSolarEngine::calculate_annual_co2_reduction_kg(daily_yield);
    assert!((annual_co2 - 114_791.0).abs() < 100.0);
}

#[test]
fn test_smartcity_legacy_gis_bridge() {
    let zone = UrbanCatchmentZone {
        zone_id: "ZONE_KOTA_TUA".to_string(),
        area_km2: 1.5,
        land_cover: LandCoverType::CommercialConcrete,
        elevation_m: 8.0,
    };

    let coord: TitikKoordinat = zone.into();
    assert_eq!(coord.lintang, -6.2088);
    assert_eq!(coord.bujur, 106.8456);
}
