//! Unit & Integration Test: WidyaAutomotive Pure UDS Diagnostics & EV BMS Engine
//! Memverifikasi segmentasi/reassembly transport ISO-TP (ISO 15765-2),
//! state machine diagnostik UDS (ISO 14229-1), protokol keamanan Seed/Key,
//! estimasi State-of-Charge (SoC) Coulomb counting EV BMS, dan deteksi thermal runaway.

use widya::automotive::pure_automotive::{
    BatterySafetyStatus, DiagnosticSessionType, EcuDiagnosticServer, EvBatteryTelemetry,
    EvBmsEngine, IsoTpTransport,
};
use widya::automotive::Obd2Telemetry;

#[test]
fn test_iso_tp_single_frame_and_multiframe_roundtrip() {
    let can_id = 0x7E0;

    // 1. Single Frame (panjang <= 7 byte)
    let payload_short = vec![0x22, 0xF1, 0x90]; // Read DID VIN
    let frames_sf = IsoTpTransport::segment_message(can_id, &payload_short);
    assert_eq!(frames_sf.len(), 1);
    assert_eq!(frames_sf[0].data[0], 0x03); // SF panjang 3 byte
    assert_eq!(&frames_sf[0].data[1..4], &[0x22, 0xF1, 0x90]);

    let reassembled_sf = IsoTpTransport::reassemble_frames(&frames_sf).expect("Reassembly SF gagal");
    assert_eq!(reassembled_sf, payload_short);

    // 2. Multi-Frame (First Frame + Consecutive Frames, misal 20 byte)
    let payload_long = b"INDONESIA_EV_NUSANTARA_2026".to_vec(); // 27 bytes
    let frames_mf = IsoTpTransport::segment_message(can_id, &payload_long);
    assert!(frames_mf.len() > 1);

    // Frame pertama harus bertipe First Frame (PCI 0x10)
    assert_eq!((frames_mf[0].data[0] >> 4) & 0x0F, 1);
    let total_len = (((frames_mf[0].data[0] & 0x0F) as usize) << 8) | (frames_mf[0].data[1] as usize);
    assert_eq!(total_len, 27);

    // Frame berikutnya harus bertipe Consecutive Frame (PCI 0x20)
    assert_eq!((frames_mf[1].data[0] >> 4) & 0x0F, 2);
    assert_eq!(frames_mf[1].data[0] & 0x0F, 1); // Sequence Number = 1

    let reassembled_mf = IsoTpTransport::reassemble_frames(&frames_mf).expect("Reassembly MF gagal");
    assert_eq!(reassembled_mf, payload_long);
}

#[test]
fn test_uds_diagnostic_session_and_security_access() {
    let mut server = EcuDiagnosticServer::new("INA-EV-2026-X99");
    assert_eq!(server.session, DiagnosticSessionType::DefaultSession);
    assert!(!server.is_security_unlocked);

    // 1. Coba masuk ke Programming Session tanpa membuka Security -> Harus Ditolak (NRC 0x33 SecurityAccessDenied)
    let req_prog = vec![0x10, 0x02];
    let resp_prog_denied = server.handle_request(&req_prog);
    assert_eq!(resp_prog_denied, vec![0x7F, 0x10, 0x33]);

    // 2. Buka Extended Diagnostic Session (0x10 0x03)
    let req_ext = vec![0x10, 0x03];
    let resp_ext = server.handle_request(&req_ext);
    assert_eq!(resp_ext[0], 0x50); // Positive response 0x10 + 0x40
    assert_eq!(server.session, DiagnosticSessionType::ExtendedDiagnosticSession);

    // 3. Request Seed (0x27 0x01)
    let req_seed = vec![0x27, 0x01];
    let resp_seed = server.handle_request(&req_seed);
    assert_eq!(resp_seed[0], 0x67);
    assert_eq!(resp_seed[1], 0x01);
    let seed = u32::from_be_bytes([resp_seed[2], resp_seed[3], resp_seed[4], resp_seed[5]]);
    assert_eq!(seed, server.current_seed);

    // 4. Hitung Key dan Kirim (0x27 0x02)
    let calculated_key = (seed ^ 0xA5A5A5A5).wrapping_add(0x1234);
    let key_bytes = calculated_key.to_be_bytes();
    let req_key = vec![0x27, 0x02, key_bytes[0], key_bytes[1], key_bytes[2], key_bytes[3]];
    let resp_key = server.handle_request(&req_key);
    assert_eq!(resp_key, vec![0x67, 0x02]);
    assert!(server.is_security_unlocked);

    // 5. Sekarang masuk ke Programming Session (0x10 0x02) harus berhasil!
    let resp_prog_ok = server.handle_request(&req_prog);
    assert_eq!(resp_prog_ok[0], 0x50);
    assert_eq!(server.session, DiagnosticSessionType::ProgrammingSession);

    // 6. Baca DID VIN (0x22 0xF1 0x90)
    let req_vin = vec![0x22, 0xF1, 0x90];
    let resp_vin = server.handle_request(&req_vin);
    assert_eq!(&resp_vin[0..3], &[0x62, 0xF1, 0x90]);
    assert_eq!(&resp_vin[3..], b"INA-EV-2026-X99");
}

#[test]
fn test_ev_bms_coulomb_counting_and_safety_guard() {
    // Paket Baterai EV 60 kWh / 150 Ah, awal SoC 80%
    let mut bms = EvBmsEngine::new(150.0, 80.0);

    // 1. Uji Discharge Arus 75 Ampere selama 1 jam (3600 detik)
    // Delta Ah = 75 Ah -> Pengurangan SoC = (75 / 150) * 100% = 50%
    // Jadi SoC sisa = 80% - 50% = 30%
    let soc_after = bms.update_coulomb_counting(75.0, 3600.0);
    assert!((soc_after - 30.0).abs() < 1e-2);

    // 2. Evaluasi Status Keamanan Kondisi Normal
    let normal_cells = vec![3650, 3645, 3652, 3648]; // Tegangan sel rata (~3.65V)
    let safety_norm = bms.evaluate_safety(&normal_cells, 32.0, 0.05);
    assert_eq!(safety_norm, BatterySafetyStatus::Normal);

    // 3. Evaluasi Cell Imbalance (selisih > 250 mV)
    let unbalanced_cells = vec![3700, 3420, 3680, 3690]; // 3700 - 3420 = 280 mV
    let safety_imbalance = bms.evaluate_safety(&unbalanced_cells, 34.0, 0.05);
    assert_eq!(safety_imbalance, BatterySafetyStatus::CellImbalanceWarning);

    // 4. Deteksi Bahaya Thermal Runaway (Suhu > 65°C atau laju kenaikan > 1.2°C/detik)
    let safety_thermal = bms.evaluate_safety(&normal_cells, 72.0, 1.5);
    assert_eq!(safety_thermal, BatterySafetyStatus::ThermalRunawayHazard);
}

#[test]
fn test_automotive_legacy_obd2_bridge() {
    let telemetry = EvBatteryTelemetry {
        pack_voltage_v: 400.0,
        current_draw_a: 45.0,
        state_of_charge_pct: 68.5,
        cell_voltages_mv: vec![3650; 96],
        temperature_celsius: 35.0,
        safety_status: BatterySafetyStatus::Normal,
    };

    let legacy_obd2: Obd2Telemetry = (&telemetry).into();
    assert_eq!(legacy_obd2.coolant_temp_celsius, 35);
    assert_eq!(legacy_obd2.ev_battery_soc_pct, 68.5);
}
