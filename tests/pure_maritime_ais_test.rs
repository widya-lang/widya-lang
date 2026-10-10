use widya::maritime::pure_ais::{
    AisDecoder, AisMessage, MarineKinematicsEngine, VesselKinematics,
};

#[test]
fn test_nmea_checksum_verification() {
    // Standard valid sentence with checksum *1E
    let sentence = "!AIVDM,1,1,,B,15N874001To@rN8K7W6;@wR00000,0*1E";
    // Check XOR sum manually:
    let body = "AIVDM,1,1,,B,15N874001To@rN8K7W6;@wR00000,0";
    let mut expected_xor = 0u8;
    for b in body.as_bytes() {
        expected_xor ^= *b;
    }
    let formatted = format!("!{}*{:02X}", body, expected_xor);

    assert!(AisDecoder::verify_nmea_checksum(&formatted));

    // Corrupted checksum must fail
    let corrupted = format!("!{}*{:02X}", body, expected_xor ^ 0xFF);
    assert!(!AisDecoder::verify_nmea_checksum(&corrupted));
}

#[test]
fn test_ais_position_report_decoding() {
    // Valid Type 1 sentence for vessel in Indonesian waters (Sunda Strait)
    let body = "AIVDM,1,1,,B,177KQJ5000G?tO`K>RA1wUbN0TKH,0";
    let mut xor = 0u8;
    for b in body.as_bytes() {
        xor ^= *b;
    }
    let valid_sentence = format!("!{}*{:02X}", body, xor);

    let msg = AisDecoder::decode_sentence(&valid_sentence).expect("Failed to decode AIS sentence");
    match msg {
        AisMessage::PositionReport(pos) => {
            assert_eq!(pos.message_type, 1);
            assert!(pos.mmsi > 0);
            assert!(pos.latitude >= -90.0 && pos.latitude <= 90.0);
            assert!(pos.longitude >= -180.0 && pos.longitude <= 180.0);
            assert!(pos.speed_over_ground_knots >= 0.0);
            assert!(pos.course_over_ground_deg >= 0.0 && pos.course_over_ground_deg <= 360.0);
        }
        _ => panic!("Expected PositionReport"),
    }
}

#[test]
fn test_marine_cpa_tcpa_collision_detection() {
    // Scenario 1: Head-on collision course in ALKI II (Makassar Strait)
    let own_ship = VesselKinematics {
        lat_deg: -1.0,
        lon_deg: 118.0,
        speed_knots: 15.0,
        course_deg: 0.0, // North
    };
    let target_ship = VesselKinematics {
        lat_deg: -0.8, // 12 NM North (0.2 deg * 60 = 12 NM)
        lon_deg: 118.0,
        speed_knots: 15.0,
        course_deg: 180.0, // South (Head-on)
    };

    // Closing speed: 30 knots. Distance: 12 NM. TCPA = 12/30 * 60 = 24 minutes. CPA = 0.0 NM.
    let cpa = MarineKinematicsEngine::compute_cpa_tcpa(&own_ship, &target_ship, 1.0, 30.0);
    assert!(!cpa.is_diverging);
    assert!((cpa.cpa_distance_nm).abs() < 0.1);
    assert!((cpa.tcpa_minutes - 24.0).abs() < 0.5);
    assert!(cpa.is_collision_risk);

    // Scenario 2: Diverging ships (already passed each other)
    let diverging_target = VesselKinematics {
        lat_deg: -0.8,
        lon_deg: 118.0,
        speed_knots: 15.0,
        course_deg: 0.0, // Both heading North, target is ahead and moving at same/higher speed
    };
    let cpa_div = MarineKinematicsEngine::compute_cpa_tcpa(&own_ship, &diverging_target, 1.0, 30.0);
    // Relative speed is 0 -> constant distance of 12 NM
    assert_eq!(cpa_div.is_collision_risk, false);
}
