use widya::space::pure_spacecraft::{
    calculate_ccsds_crc16, CcsdsTmTransferFrame, SatelliteTle, CCSDS_ASM,
};

#[test]
fn test_ccsds_crc16_and_frame_roundtrip() {
    let payload = b"WIDYA-SPACE-PAYLOAD-DATA-001".to_vec();
    let frame = CcsdsTmTransferFrame::new(42, 1, 10, 5, payload.clone());

    let bytes = frame.to_bytes();
    assert!(bytes.len() >= 12);
    // ASM marker
    assert_eq!(&bytes[0..4], &CCSDS_ASM);

    // Decode back
    let decoded = CcsdsTmTransferFrame::from_bytes(&bytes).expect("Failed to decode TM frame");
    assert_eq!(decoded.spacecraft_id, 42);
    assert_eq!(decoded.virtual_channel_id, 1);
    assert_eq!(decoded.master_frame_count, 10);
    assert_eq!(decoded.vc_frame_count, 5);
    assert_eq!(decoded.data_payload, payload);

    // Corrupted CRC must fail
    let mut bad_crc = bytes.clone();
    let last = bad_crc.len() - 1;
    bad_crc[last] ^= 0xFF;
    assert!(CcsdsTmTransferFrame::from_bytes(&bad_crc).is_err());

    // Corrupted ASM must fail
    let mut bad_asm = bytes.clone();
    bad_asm[0] = 0x00;
    assert!(CcsdsTmTransferFrame::from_bytes(&bad_asm).is_err());
}

#[test]
fn test_tle_parser_and_orbit_propagation() {
    // Real LEO Satellite TLE (ISS / Nusantara Research Sat)
    let l1 = "1 25544U 98067A   24001.50000000  .00016717  00000-0  10270-3 0  9003";
    let l2 = "2 25544  51.6400 120.0000 0005000  45.0000  90.0000 15.50000000400000";

    let tle = SatelliteTle::parse_tle("NUSANTARA-RESEARCH-1", l1, l2).expect("Failed to parse TLE");
    assert_eq!(tle.sat_number, 25544);
    assert_eq!(tle.inclination_deg, 51.64);
    assert_eq!(tle.mean_motion_rev_per_day, 15.5);

    // Propagate at epoch (0 seconds)
    let pos0 = tle.propagate(0.0);
    assert!(pos0.altitude_km > 350.0 && pos0.altitude_km < 450.0); // Typical LEO orbit altitude
    assert!(pos0.lat_deg >= -90.0 && pos0.lat_deg <= 90.0);
    assert!(pos0.lon_deg >= -180.0 && pos0.lon_deg <= 180.0);

    // Propagate 1 orbit period later (~90 minutes = 5400s)
    let period_s = 86400.0 / tle.mean_motion_rev_per_day;
    let pos_1orbit = tle.propagate(period_s);
    assert!(pos_1orbit.altitude_km > 350.0 && pos_1orbit.altitude_km < 450.0);

    // Earth rotated beneath the satellite during the orbit!
    // Longitude should have shifted westwards
    assert!((pos0.lon_deg - pos_1orbit.lon_deg).abs() > 1.0);
}

#[test]
fn test_ccsds_crc16_known_vector() {
    let test_data = b"123456789";
    let crc = calculate_ccsds_crc16(test_data);
    // Standar CCITT (polinomial 0x1021, init 0xFFFF) untuk b"123456789" adalah 0x29B1
    assert_eq!(crc, 0x29B1);
}
