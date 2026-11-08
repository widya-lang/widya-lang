// ============================================================================
// Integration Test: Pure Sovereign SRT & RTMP Streaming Protocol Engine
// ============================================================================
// Menguji jabat tangan RTMP (C0/C1/C2), AMF0 serialization, RTMP Chunk Type 0,
// serta paket kontrol & data SRT over UDP tanpa FFmpeg atau OBS C++.
// ============================================================================

use widya::web::{
    Amf0Engine, RtmpChunkEngine, RtmpHandshake, SrtPacketEngine, SrtPacketType,
    RTMP_HANDSHAKE_SIZE, RTMP_MSG_VIDEO, RTMP_VERSION, SRT_CONTROL_HANDSHAKE,
    SRT_CONTROL_KEEPALIVE,
};

#[test]
fn test_rtmp_handshake_c0_c1_s0_s1_c2() {
    let client_ts = 1000u32;
    // 1. Klien membuat paket C0 + C1 (1537 bytes)
    let c0_c1 = RtmpHandshake::buat_c0_c1(client_ts);
    assert_eq!(c0_c1.len(), 1 + RTMP_HANDSHAKE_SIZE);
    assert_eq!(c0_c1[0], RTMP_VERSION, "C0 harus 0x03");

    // 2. Simulasi balasan peladen S0 + S1 (1537 bytes)
    let mut s0_s1 = Vec::with_capacity(1 + RTMP_HANDSHAKE_SIZE);
    s0_s1.push(RTMP_VERSION);
    s0_s1.extend_from_slice(&2000u32.to_be_bytes()); // Server TS
    s0_s1.extend_from_slice(&[0, 0, 0, 0]);
    for i in 0..1528 {
        s0_s1.push((i % 256) as u8);
    }

    // 3. Klien memverifikasi S0+S1 dan membuat paket balasan C2
    let c2 = RtmpHandshake::verifikasi_s0_s1_dan_buat_c2(&s0_s1)
        .expect("Jabat tangan C2 harus berhasil");

    assert_eq!(c2.len(), RTMP_HANDSHAKE_SIZE);
    // Verifikasi bahwa 4 byte pertama C2 adalah timestamp S1 asli
    assert_eq!(&c2[0..4], &2000u32.to_be_bytes());
}

#[test]
fn test_amf0_command_serialization_and_parsing() {
    let cmd_name = "connect";
    let trans_id = 1.0f64;
    let params = [
        ("app", "live"),
        ("tcUrl", "rtmp://siaran.widya.id/live"),
        ("flashVer", "WidyaBroadcaster 0.1.0"),
    ];

    // 1. Serialisasi perintah AMF0
    let amf0_bytes = Amf0Engine::encode_command(cmd_name, trans_id, &params);
    assert!(amf0_bytes.len() > 20);

    // 2. Parsing perintah AMF0
    let (decoded_cmd, decoded_trans_id) = Amf0Engine::decode_command(&amf0_bytes)
        .expect("Parsing AMF0 harus berhasil");

    assert_eq!(decoded_cmd, "connect");
    assert_eq!(decoded_trans_id, 1.0);
}

#[test]
fn test_rtmp_chunk_type_0_encoding_and_decoding() {
    let cs_id = 4u8; // Chunk stream ID video
    let timestamp_ms = 4500u32;
    let msg_type = RTMP_MSG_VIDEO; // 9
    let stream_id = 1u32;
    let payload = b"\x17\x01\x00\x00\x00\x00\x00\x00\x02\x65Pure Widya AVC Video NALU Data";

    // 1. Encode Chunk Type 0
    let chunk_bytes = RtmpChunkEngine::encode_chunk_type_0(
        cs_id,
        timestamp_ms,
        msg_type,
        stream_id,
        payload,
    );

    // 12 bytes header (1 basic + 11 message) + payload
    assert_eq!(chunk_bytes.len(), 12 + payload.len());

    // 2. Decode Chunk Type 0
    let (d_cs_id, d_ts, d_msg_type, d_stream_id, d_payload) =
        RtmpChunkEngine::decode_chunk_type_0(&chunk_bytes)
            .expect("Decoding chunk Type 0 harus berhasil");

    assert_eq!(d_cs_id, cs_id);
    assert_eq!(d_ts, timestamp_ms);
    assert_eq!(d_msg_type, msg_type);
    assert_eq!(d_stream_id, stream_id);
    assert_eq!(d_payload, payload);
}

#[test]
fn test_srt_handshake_and_control_packets() {
    let socket_id = 0x12345678u32;
    let syn_cookie = 0xABCDEF01u32;
    let isn = 1000u32;
    let ts_us = 450000u32;

    // 1. Bangun paket SRT Handshake (Control Type 0x0000)
    let hs_packet = SrtPacketEngine::build_handshake_packet(socket_id, syn_cookie, isn, ts_us);
    assert_eq!(hs_packet.len(), 16 + 48);

    // 2. Parse paket SRT Handshake
    let parsed = SrtPacketEngine::parse_packet(&hs_packet)
        .expect("Parsing paket SRT harus berhasil");

    match parsed {
        SrtPacketType::Control {
            control_type,
            socket_id: s_id,
            timestamp_us,
            cif_data,
            ..
        } => {
            assert_eq!(control_type, SRT_CONTROL_HANDSHAKE);
            assert_eq!(s_id, socket_id);
            assert_eq!(timestamp_us, ts_us);
            assert_eq!(cif_data.len(), 48); // CIF Handshake size
        }
        _ => panic!("Harus berupa paket SRT Control!"),
    }

    // 3. Bangun dan parse paket SRT Keepalive
    let ka_packet = SrtPacketEngine::build_keepalive_packet(socket_id, ts_us + 1000);
    let parsed_ka = SrtPacketEngine::parse_packet(&ka_packet).unwrap();
    match parsed_ka {
        SrtPacketType::Control { control_type, .. } => {
            assert_eq!(control_type, SRT_CONTROL_KEEPALIVE);
        }
        _ => panic!("Harus berupa paket SRT Keepalive!"),
    }
}

#[test]
fn test_srt_data_packet_streaming() {
    let seq_num = 50001u32;
    let msg_num = 12u32;
    let ts_us = 998877u32;
    let socket_id = 0xCAFEBABE;
    let video_payload = b"AV1_KEYFRAME_VIDEO_DATAGRAM_SOVEREIGN_STREAM";

    // 1. Bangun paket data video SRT
    let data_packet = SrtPacketEngine::build_data_packet(
        seq_num,
        msg_num,
        ts_us,
        socket_id,
        video_payload,
    );

    assert_eq!(data_packet.len(), 16 + video_payload.len());

    // 2. Parse paket data SRT
    let parsed_data = SrtPacketEngine::parse_packet(&data_packet)
        .expect("Parsing paket data SRT harus berhasil");

    match parsed_data {
        SrtPacketType::Data {
            seq_num: s_num,
            msg_num: m_num,
            timestamp_us,
            socket_id: s_id,
            payload,
        } => {
            assert_eq!(s_num, seq_num);
            assert_eq!(m_num, msg_num);
            assert_eq!(timestamp_us, ts_us);
            assert_eq!(s_id, socket_id);
            assert_eq!(payload, video_payload);
        }
        _ => panic!("Harus berupa paket SRT Data!"),
    }
}
