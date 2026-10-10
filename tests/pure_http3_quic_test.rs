use widya::web::pure_http3::*;
use std::collections::HashMap;

#[test]
fn test_quic_vint_encoding_and_decoding() {
    let test_values = [0u64, 25, 63, 64, 1500, 16383, 16384, 100_000, 1073741823, 1073741824, 4611686018427387903];

    for &v in &test_values {
        let encoded = QuicVint::encode(v);
        let (decoded, len) = QuicVint::decode(&encoded).expect("Decode VINT harus berhasil");
        assert_eq!(decoded, v, "Nilai decoded harus sama dengan input");
        assert_eq!(len, encoded.len(), "Panjang yang dibaca harus sama dengan ukuran buffer");
    }
}

#[test]
fn test_quic_packet_serialization_and_deserialization() {
    let dcid = ConnectionId::baru(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let scid = ConnectionId::baru(&[8, 7, 6, 5, 4, 3, 2, 1]);
    let payload = b"QUIC_PURE_SOVEREIGN_PAYLOAD".to_vec();

    // 1. Long Header Initial Packet
    let initial_pkt = QuicPacket::LongHeader {
        packet_type: QuicLongPacketType::Initial,
        version: QUIC_VERSION_1,
        dcid: dcid.clone(),
        scid: scid.clone(),
        packet_number: 42,
        payload: payload.clone(),
    };

    let serialized = initial_pkt.serialize();
    let deserialized = QuicPacket::deserialize(&serialized, 8).expect("Deserialisasi Initial packet harus berhasil");

    if let QuicPacket::LongHeader { packet_type, version, dcid: d, scid: s, packet_number, payload: p } = deserialized {
        assert_eq!(packet_type, QuicLongPacketType::Initial);
        assert_eq!(version, QUIC_VERSION_1);
        assert_eq!(d, dcid);
        assert_eq!(s, scid);
        assert_eq!(packet_number, 42);
        assert_eq!(p, payload);
    } else {
        panic!("Ekspektasi LongHeader packet");
    }

    // 2. Short Header 1-RTT Packet
    let short_pkt = QuicPacket::ShortHeader {
        dcid: dcid.clone(),
        packet_number: 100,
        payload: payload.clone(),
    };

    let short_ser = short_pkt.serialize();
    let short_de = QuicPacket::deserialize(&short_ser, 8).expect("Deserialisasi Short packet harus berhasil");

    if let QuicPacket::ShortHeader { dcid: d, packet_number, payload: p } = short_de {
        assert_eq!(d, dcid);
        assert_eq!(packet_number, 100);
        assert_eq!(p, payload);
    } else {
        panic!("Ekspektasi ShortHeader packet");
    }
}

#[test]
fn test_quic_frames_serialization_and_parsing() {
    let stream_frame = QuicFrame::Stream {
        stream_id: 4,
        offset: 128,
        fin: true,
        data: b"H3_FRAME_STREAM_DATA".to_vec(),
    };

    let ack_frame = QuicFrame::Ack {
        largest_acknowledged: 15,
        ack_delay: 25,
    };

    let close_frame = QuicFrame::ConnectionClose {
        error_code: 0x0100,
        reason: "Normal Shutdown".to_string(),
    };

    let mut buf = Vec::new();
    buf.extend_from_slice(&stream_frame.serialize());
    buf.extend_from_slice(&ack_frame.serialize());
    buf.extend_from_slice(&close_frame.serialize());

    let parsed = QuicFrame::parse_frames(&buf).expect("Parse frames harus sukses");
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0], stream_frame);
    assert_eq!(parsed[1], ack_frame);
    assert_eq!(parsed[2], close_frame);
}

#[test]
fn test_http3_frames_and_qpack_compression() {
    let headers = vec![
        H3Header::new(":method", "GET"),
        H3Header::new(":path", "/"),
        H3Header::new(":scheme", "https"),
        H3Header::new(":status", "200"),
        H3Header::new("x-powered-by", "Widya-Lang Sovereign Matrix"),
    ];

    let header_frame = H3Frame::Headers(headers.clone());
    let data_frame = H3Frame::Data(b"<h1>HTTP/3 Berdaulat Berjalan Cepat!</h1>".to_vec());

    let mut settings_map = HashMap::new();
    settings_map.insert(0x06, 65536); // MAX_FIELD_SECTION_SIZE
    let settings_frame = H3Frame::Settings(settings_map);

    let mut stream_buf = Vec::new();
    stream_buf.extend_from_slice(&settings_frame.serialize());
    stream_buf.extend_from_slice(&header_frame.serialize());
    stream_buf.extend_from_slice(&data_frame.serialize());

    // Parse stream buffer
    let (parsed_settings, off1) = H3Frame::parse(&stream_buf).expect("Parse settings harus ok");
    assert!(matches!(parsed_settings, H3Frame::Settings(_)));

    let (parsed_headers, off2) = H3Frame::parse(&stream_buf[off1..]).expect("Parse headers harus ok");
    if let H3Frame::Headers(h_list) = parsed_headers {
        assert_eq!(h_list.len(), 5);
        assert_eq!(h_list[0], H3Header::new(":method", "GET"));
        assert_eq!(h_list[1], H3Header::new(":path", "/"));
        assert_eq!(h_list[4], H3Header::new("x-powered-by", "Widya-Lang Sovereign Matrix"));
    } else {
        panic!("Ekspektasi H3Frame::Headers");
    }

    let (parsed_data, _) = H3Frame::parse(&stream_buf[off1 + off2..]).expect("Parse data harus ok");
    if let H3Frame::Data(d) = parsed_data {
        assert_eq!(d, b"<h1>HTTP/3 Berdaulat Berjalan Cepat!</h1>");
    } else {
        panic!("Ekspektasi H3Frame::Data");
    }
}

#[test]
fn test_pure_http3_udp_roundtrip_communication() {
    // Jalankan server UDP HTTP/3 pada ephemeral port loopback
    let server = PureHttp3Server::bind("127.0.0.1:0").expect("Server harus berhasil bind");
    let server_addr = server.addr;

    // Jalankan thread server untuk merespon 1 request
    let handle = std::thread::spawn(move || {
        let res = server.tangani_satu_paket(|method, path, body| {
            assert_eq!(method, "GET");
            assert_eq!(path, "/api/kedaulatan");
            assert_eq!(body, b"PING_FROM_CLIENT");

            let resp_headers = vec![
                H3Header::new("content-type", "application/json"),
                H3Header::new("server", "Widya-Pure-HTTP3"),
            ];
            let resp_body = b"{\"status\":\"sukses\",\"protokol\":\"HTTP/3 (RFC 9114)\"}".to_vec();

            (200, resp_headers, resp_body)
        });
        assert!(res.is_ok());
    });

    // Klien mengirim request via UDP
    let resp = PureHttp3Client::kirim_permintaan(
        server_addr,
        "GET",
        "/api/kedaulatan",
        b"PING_FROM_CLIENT",
    ).expect("Klien harus berhasil menerima respon HTTP/3");

    handle.join().unwrap();

    assert_eq!(resp.status, 200);
    assert_eq!(resp.headers.get("content-type").map(|s| s.as_str()), Some("application/json"));
    assert_eq!(resp.headers.get("alt-svc").map(|s| s.as_str()), Some("h3=\":443\"; ma=86400"));
    assert!(String::from_utf8_lossy(&resp.body).contains("HTTP/3 (RFC 9114)"));
}
