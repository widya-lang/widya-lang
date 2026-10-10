use widya::telecom::pure_5g::{
    Pure5gSession, Pure5gUpfRouter, PureGtpuCodec, PurePfcpCodec, PFCP_MSG_HEARTBEAT_REQ,
    PFCP_MSG_SESSION_ESTABLISH_RESP,
};

#[test]
fn test_gtpu_encapsulation_and_decapsulation_with_5g_qfi() {
    let inner_ip = vec![
        0x45, 0x00, 0x00, 0x28, // IPv4 Header
        0x12, 0x34, 0x40, 0x00,
        0x40, 0x06, 0x00, 0x00,
        10, 45, 0, 2, // Src IP
        8, 8, 8, 8,   // Dst IP
        0x04, 0xD2, 0x00, 0x50, // Port 1234 -> 80
    ];

    let teid = 0xCAFE1234;
    let qfi = 9u8; // 5G QoS Flow Identifier 9

    // 1. Enkapsulasi ke GTP-U
    let gtpu_bytes = PureGtpuCodec::encapsulate(teid, Some(qfi), &inner_ip);
    assert!(gtpu_bytes.len() > inner_ip.len());
    assert_eq!(gtpu_bytes[1], 255); // T-PDU

    // 2. Dekapsulasi dari GTP-U
    let (header, extracted_payload) = PureGtpuCodec::decapsulate(&gtpu_bytes)
        .expect("Failed to decapsulate GTP-U packet");

    assert_eq!(header.version, 1);
    assert_eq!(header.message_type, 255);
    assert_eq!(header.teid, teid);
    assert_eq!(header.qfi, Some(9));
    assert_eq!(extracted_payload, inner_ip);
}

#[test]
fn test_pfcp_protocol_messages() {
    // 1. Heartbeat Request
    let hb_bytes = PurePfcpCodec::build_heartbeat_req(105);
    let hb_header = PurePfcpCodec::parse_header(&hb_bytes).expect("Failed to parse PFCP Heartbeat");
    assert_eq!(hb_header.version, 1);
    assert_eq!(hb_header.message_type, PFCP_MSG_HEARTBEAT_REQ);
    assert!(!hb_header.has_seid);
    assert_eq!(hb_header.sequence_number, 105);

    // 2. Session Establishment Response
    let seid = 0xABCD1234567890EF;
    let resp_bytes = PurePfcpCodec::build_session_establish_resp(seid, 106, true);
    let resp_header = PurePfcpCodec::parse_header(&resp_bytes).expect("Failed to parse PFCP Session Resp");
    assert_eq!(resp_header.message_type, PFCP_MSG_SESSION_ESTABLISH_RESP);
    assert!(resp_header.has_seid);
    assert_eq!(resp_header.seid, Some(seid));
    assert_eq!(resp_header.sequence_number, 106);

    // Verify Cause IE in payload
    assert!(resp_bytes.len() >= 16 + 5);
    let ie_type = u16::from_be_bytes([resp_bytes[16], resp_bytes[17]]);
    assert_eq!(ie_type, 19); // Cause IE Type
    assert_eq!(resp_bytes[20], 1); // 1 = Request Accepted
}

#[test]
fn test_pure_5g_upf_router_uplink_and_downlink() {
    let mut upf = Pure5gUpfRouter::new();

    let session = Pure5gSession {
        seid: 1001,
        teid_uplink: 0x11112222,
        teid_downlink: 0x33334444,
        ue_ip: [10, 45, 0, 5],
        qfi: 5,
    };
    upf.register_session(session);

    let inner_packet = b"PING-PONG-5G-PAYLOAD".to_vec();

    // 1. Test Uplink: Incoming from gNodeB
    let incoming_gtpu = PureGtpuCodec::encapsulate(0x11112222, Some(5), &inner_packet);
    let routed_uplink = upf.route_uplink(&incoming_gtpu).expect("Uplink routing failed");
    assert_eq!(routed_uplink, inner_packet);

    // 2. Test Downlink: Incoming from Internet towards UE IP 10.45.0.5
    let routed_downlink = upf.route_downlink([10, 45, 0, 5], &inner_packet).expect("Downlink routing failed");
    let (dl_header, dl_payload) = PureGtpuCodec::decapsulate(&routed_downlink).unwrap();
    assert_eq!(dl_header.teid, 0x33334444);
    assert_eq!(dl_header.qfi, Some(5));
    assert_eq!(dl_payload, inner_packet);
}
