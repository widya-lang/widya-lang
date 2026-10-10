//! Unit & Integration Test: WidyaEnergy Pure IEC 61850 Wire Engine
//! Memverifikasi enkapsulasi/dekapsulasi Layer 2 Ethernet GOOSE dan Sampled Values (SV),
//! ASN.1 BER encoding, kalkulasi True RMS, serta sub-3ms tripping protection state machine.

use widya::energy::pure_energy::{
    BreakerStatus, PureGooseMessage, PureSvMessage, SubstationProtectionRelay,
    ETHERTYPE_GOOSE, ETHERTYPE_SAMPLED_VALUES,
};
use widya::energy::{GooseControlBlock, SampledValueMeasurement};

#[test]
fn test_iec61850_goose_frame_roundtrip() {
    let msg = PureGooseMessage::new_trip(
        "Substation1/LLN0$GO$gcb01",
        0x0001,
        true, // Trip Command = TRUE
        12,   // StNum = 12
        0,    // SqNum = 0
    );

    let frame_bytes = msg.encode_frame();

    // Verifikasi Header Ethernet
    assert!(frame_bytes.len() >= 22);
    assert_eq!(&frame_bytes[0..6], &[0x01, 0x0C, 0xCD, 0x01, 0x00, 0x01]); // Multicast MAC
    let ethertype = u16::from_be_bytes([frame_bytes[12], frame_bytes[13]]);
    assert_eq!(ethertype, ETHERTYPE_GOOSE);
    let app_id = u16::from_be_bytes([frame_bytes[14], frame_bytes[15]]);
    assert_eq!(app_id, 0x0001);

    // Dekode kembali
    let decoded = PureGooseMessage::decode_frame(&frame_bytes).expect("Decode GOOSE failed");
    assert_eq!(decoded.app_id, 0x0001);
    assert_eq!(decoded.gocb_ref, "Substation1/LLN0$GO$gcb01");
    assert_eq!(decoded.st_num, 12);
    assert_eq!(decoded.sq_num, 0);
    assert!(decoded.trip_signal);
}

#[test]
fn test_iec61850_sampled_values_frame_roundtrip() {
    let sv = PureSvMessage::new(
        "MU01_FEEDER",
        0x4000,
        1250, // smpCnt
        450_000, -225_000, -225_000, 0, // Ia, Ib, Ic, In (mA)
        20_000_000, -10_000_000, -10_000_000, 0, // Va, Vb, Vc, Vn (mV)
    );

    let frame_bytes = sv.encode_frame();

    // Verifikasi Header Ethernet
    assert!(frame_bytes.len() >= 22);
    assert_eq!(&frame_bytes[0..6], &[0x01, 0x0C, 0xCD, 0x04, 0x00, 0x01]); // SV Multicast MAC
    let ethertype = u16::from_be_bytes([frame_bytes[12], frame_bytes[13]]);
    assert_eq!(ethertype, ETHERTYPE_SAMPLED_VALUES);
    let app_id = u16::from_be_bytes([frame_bytes[14], frame_bytes[15]]);
    assert_eq!(app_id, 0x4000);

    // Dekode kembali
    let decoded = PureSvMessage::decode_frame(&frame_bytes).expect("Decode SV failed");
    assert_eq!(decoded.sv_id, "MU01_FEEDER");
    assert_eq!(decoded.smp_cnt, 1250);
    assert_eq!(decoded.current_a_ma, 450_000);
    assert_eq!(decoded.current_b_ma, -225_000);
    assert_eq!(decoded.voltage_a_mv, 20_000_000);
}

#[test]
fn test_substation_protection_overcurrent_trip() {
    // Threshold proteksi: 1000 Ampere = 1,000,000 mA
    // Ukuran window: 8 sampel
    let mut relay = SubstationProtectionRelay::new("RELAY_OC_51", 1_000_000, 8);
    assert_eq!(relay.breaker_state, BreakerStatus::Closed);

    // Masukkan 7 sampel arus normal (200 A = 200,000 mA)
    for i in 0..7 {
        let sv = PureSvMessage::new(
            "MU01", 0x4000, i as u16,
            200_000, -100_000, -100_000, 0,
            20_000_000, 0, 0, 0,
        );
        let trip = relay.ingest_sv(&sv);
        assert!(trip.is_none());
        assert_eq!(relay.breaker_state, BreakerStatus::Closed);
    }

    // Masukkan sampel ke-8 dengan arus gangguan hubung singkat (1500 A = 1,500,000 mA)
    let fault_sv = PureSvMessage::new(
        "MU01", 0x4000, 7,
        3_500_000, -1_750_000, -1_750_000, 0, // Gangguan besar sehingga RMS > 1,000,000 mA
        20_000_000, 0, 0, 0,
    );
    let trip_opt = relay.ingest_sv(&fault_sv);

    assert!(trip_opt.is_some(), "Relay harus memicu trip saat arus melebihi threshold");
    let trip_goose = trip_opt.unwrap();
    assert!(trip_goose.trip_signal);
    assert_eq!(relay.breaker_state, BreakerStatus::Tripping);
    assert_eq!(trip_goose.st_num, 2);
}

#[test]
fn test_energy_legacy_bridges() {
    let legacy_gcb = GooseControlBlock::new_trip("Bay1/LLN0$GO$trip", 0x0002, true, 5);
    let pure_msg: PureGooseMessage = legacy_gcb.clone().into();
    assert_eq!(pure_msg.app_id, 0x0002);
    assert_eq!(pure_msg.st_num, 5);
    assert!(pure_msg.trip_signal);

    let roundtrip_gcb: GooseControlBlock = pure_msg.into();
    assert_eq!(roundtrip_gcb.app_id, legacy_gcb.app_id);
    assert_eq!(roundtrip_gcb.state_num, legacy_gcb.state_num);

    let legacy_sv = SampledValueMeasurement {
        sv_id: "MU_TEST".to_string(),
        current_phase_a: 120.5,
        current_phase_b: -60.25,
        current_phase_c: -60.25,
        current_neutral: 0.0,
        voltage_phase_a: 115000.0,
        voltage_phase_b: 0.0,
        voltage_phase_c: 0.0,
        smp_count: 50,
    };
    let pure_sv: PureSvMessage = legacy_sv.clone().into();
    assert_eq!(pure_sv.current_a_ma, 120500);
    assert_eq!(pure_sv.smp_cnt, 50);

    let roundtrip_sv: SampledValueMeasurement = pure_sv.into();
    assert!((roundtrip_sv.current_phase_a - legacy_sv.current_phase_a).abs() < 1e-2);
}
