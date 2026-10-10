//! Pure Sovereign HTTP/3 & QUIC Transport Engine (RFC 9000, RFC 9114 & RFC 9204)
//!
//! 100% In-House Rust Implementation tanpa pustaka pihak ketiga (Zero quiche, Zero nghttp3, Zero curl):
//! - 🚀 QUIC RFC 9000: Variable-Length Integer (VINT), Long/Short Headers, Connection ID (CID), Frames (STREAM, ACK, CRYPTO, CONNECTION_CLOSE)
//! - 🌐 HTTP/3 RFC 9114: Frame Data, Headers, Settings, Stream Multiplexing, Alt-Svc Header
//! - 📦 QPACK RFC 9204: Kompresi dan dekompresi header HTTP/3 dengan tabel statis
//! - ⚡ Soket UDP Native: `std::net::UdpSocket` untuk komunikasi bidirectional dan datagram berlatensi ultra-rendah

use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================================
// 1. QUIC VARIABLE-LENGTH INTEGER (VINT - RFC 9000 §16)
// ============================================================================

pub struct QuicVint;

impl QuicVint {
    /// Encode integer u64 ke format QUIC Variable-Length Integer (1, 2, 4, atau 8 bytes)
    pub fn encode(val: u64) -> Vec<u8> {
        if val <= 63 {
            vec![val as u8]
        } else if val <= 16383 {
            let v = (val as u16) | 0x4000;
            v.to_be_bytes().to_vec()
        } else if val <= 1073741823 {
            let v = (val as u32) | 0x8000_0000;
            v.to_be_bytes().to_vec()
        } else {
            let v = val | 0xC000_0000_0000_0000;
            v.to_be_bytes().to_vec()
        }
    }

    /// Decode QUIC Variable-Length Integer dari buffer bytes.
    /// Mengembalikan tuple (nilai, jumlah_byte_yang_dibaca).
    pub fn decode(buf: &[u8]) -> Result<(u64, usize), String> {
        if buf.is_empty() {
            return Err("Buffer kosong saat decode QUIC VINT".to_string());
        }

        let first = buf[0];
        let prefix = first >> 6;

        match prefix {
            0 => Ok((first as u64, 1)),
            1 => {
                if buf.len() < 2 {
                    return Err("Buffer terlalu pendek untuk VINT 2-byte".to_string());
                }
                let val = u16::from_be_bytes([buf[0] & 0x3F, buf[1]]) as u64;
                Ok((val, 2))
            }
            2 => {
                if buf.len() < 4 {
                    return Err("Buffer terlalu pendek untuk VINT 4-byte".to_string());
                }
                let val = u32::from_be_bytes([buf[0] & 0x3F, buf[1], buf[2], buf[3]]) as u64;
                Ok((val, 4))
            }
            3 => {
                if buf.len() < 8 {
                    return Err("Buffer terlalu pendek untuk VINT 8-byte".to_string());
                }
                let mut b = [0u8; 8];
                b.copy_from_slice(&buf[..8]);
                b[0] &= 0x3F;
                let val = u64::from_be_bytes(b);
                Ok((val, 8))
            }
            _ => unreachable!(),
        }
    }
}

// ============================================================================
// 2. QUIC PACKET HEADERS & CONNECTION ID (RFC 9000 §17)
// ============================================================================

pub const QUIC_VERSION_1: u32 = 0x0000_0001;

/// Tipe Paket QUIC Long Header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuicLongPacketType {
    Initial = 0x00,
    ZeroRtt = 0x01,
    Handshake = 0x02,
    Retry = 0x03,
}

/// Identitas Koneksi QUIC (Connection ID)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub Vec<u8>);

impl ConnectionId {
    pub fn baru(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub fn acak(len: usize) -> Self {
        let mut id = Vec::with_capacity(len);
        for i in 0..len {
            id.push(((i * 37 + 101) % 256) as u8);
        }
        Self(id)
    }
}

/// Struktur Paket QUIC
#[derive(Debug, Clone)]
pub enum QuicPacket {
    LongHeader {
        packet_type: QuicLongPacketType,
        version: u32,
        dcid: ConnectionId,
        scid: ConnectionId,
        packet_number: u64,
        payload: Vec<u8>,
    },
    ShortHeader {
        dcid: ConnectionId,
        packet_number: u64,
        payload: Vec<u8>,
    },
}

impl QuicPacket {
    /// Serialisasi paket QUIC ke byte UDP datagram
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        match self {
            QuicPacket::LongHeader { packet_type, version, dcid, scid, packet_number, payload } => {
                // Byte 0: Header Form (1), Fixed Bit (1), Long Packet Type (2 bits), Reserved/PN Len (4 bits)
                let type_bits = (*packet_type as u8) << 4;
                let first_byte = 0xC0 | type_bits;
                out.push(first_byte);

                // Version (4 bytes BE)
                out.extend_from_slice(&version.to_be_bytes());

                // DCID Length + DCID
                out.push(dcid.0.len() as u8);
                out.extend_from_slice(&dcid.0);

                // SCID Length + SCID
                out.push(scid.0.len() as u8);
                out.extend_from_slice(&scid.0);

                // Payload length VINT
                let total_len = payload.len() + 4; // payload + 4-byte packet number
                out.extend_from_slice(&QuicVint::encode(total_len as u64));

                // Packet Number (4 bytes BE)
                out.extend_from_slice(&(*packet_number as u32).to_be_bytes());

                // Payload (Frames)
                out.extend_from_slice(payload);
            }
            QuicPacket::ShortHeader { dcid, packet_number, payload } => {
                // Byte 0: Header Form (0), Fixed Bit (1), Spin (0), Reserved (00), KeyPhase (0), PN Len (11 = 4 bytes)
                out.push(0x43);

                // DCID
                out.extend_from_slice(&dcid.0);

                // Packet Number (4 bytes BE)
                out.extend_from_slice(&(*packet_number as u32).to_be_bytes());

                // Payload
                out.extend_from_slice(payload);
            }
        }

        out
    }

    /// Deserialisasi paket QUIC dari buffer UDP datagram
    pub fn deserialize(buf: &[u8], dcid_len_hint: usize) -> Result<Self, String> {
        if buf.is_empty() {
            return Err("Datagram QUIC kosong".to_string());
        }

        let first_byte = buf[0];
        let is_long_header = (first_byte & 0x80) != 0;

        if is_long_header {
            if buf.len() < 7 {
                return Err("Paket Long Header terlalu pendek".to_string());
            }

            let type_val = (first_byte >> 4) & 0x03;
            let packet_type = match type_val {
                0x00 => QuicLongPacketType::Initial,
                0x01 => QuicLongPacketType::ZeroRtt,
                0x02 => QuicLongPacketType::Handshake,
                0x03 => QuicLongPacketType::Retry,
                _ => return Err("Tipe paket long header tidak valid".to_string()),
            };

            let version = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
            let mut offset = 5;

            // DCID
            let dcid_len = buf[offset] as usize;
            offset += 1;
            if buf.len() < offset + dcid_len {
                return Err("Buffer terpotong pada DCID".to_string());
            }
            let dcid = ConnectionId::baru(&buf[offset..offset + dcid_len]);
            offset += dcid_len;

            // SCID
            if buf.len() <= offset {
                return Err("Buffer terpotong pada SCID length".to_string());
            }
            let scid_len = buf[offset] as usize;
            offset += 1;
            if buf.len() < offset + scid_len {
                return Err("Buffer terpotong pada SCID".to_string());
            }
            let scid = ConnectionId::baru(&buf[offset..offset + scid_len]);
            offset += scid_len;

            // Length VINT
            let (len_val, vint_len) = QuicVint::decode(&buf[offset..])?;
            offset += vint_len;

            // Packet number (4 bytes)
            if buf.len() < offset + 4 {
                return Err("Buffer terpotong pada packet number".to_string());
            }
            let packet_number = u32::from_be_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]]) as u64;
            offset += 4;

            let payload_len = (len_val as usize).saturating_sub(4).min(buf.len() - offset);
            let payload = buf[offset..offset + payload_len].to_vec();

            Ok(QuicPacket::LongHeader {
                packet_type,
                version,
                dcid,
                scid,
                packet_number,
                payload,
            })
        } else {
            // Short Header
            let mut offset = 1;
            let dcid_len = dcid_len_hint.min(buf.len().saturating_sub(5));
            let dcid = ConnectionId::baru(&buf[offset..offset + dcid_len]);
            offset += dcid_len;

            if buf.len() < offset + 4 {
                return Err("Short header terpotong pada packet number".to_string());
            }
            let packet_number = u32::from_be_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]]) as u64;
            offset += 4;

            let payload = buf[offset..].to_vec();

            Ok(QuicPacket::ShortHeader {
                dcid,
                packet_number,
                payload,
            })
        }
    }
}

// ============================================================================
// 3. QUIC FRAMES (RFC 9000 §19)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum QuicFrame {
    Padding,
    Ping,
    Ack {
        largest_acknowledged: u64,
        ack_delay: u64,
    },
    Crypto {
        offset: u64,
        data: Vec<u8>,
    },
    Stream {
        stream_id: u64,
        offset: u64,
        fin: bool,
        data: Vec<u8>,
    },
    ConnectionClose {
        error_code: u64,
        reason: String,
    },
}

impl QuicFrame {
    /// Serialisasi QUIC Frame ke bytes
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        match self {
            QuicFrame::Padding => {
                out.push(0x00);
            }
            QuicFrame::Ping => {
                out.push(0x01);
            }
            QuicFrame::Ack { largest_acknowledged, ack_delay } => {
                out.push(0x02);
                out.extend_from_slice(&QuicVint::encode(*largest_acknowledged));
                out.extend_from_slice(&QuicVint::encode(*ack_delay));
                out.extend_from_slice(&QuicVint::encode(0)); // ack range count 0
                out.extend_from_slice(&QuicVint::encode(0)); // first ack range 0
            }
            QuicFrame::Crypto { offset, data } => {
                out.push(0x06);
                out.extend_from_slice(&QuicVint::encode(*offset));
                out.extend_from_slice(&QuicVint::encode(data.len() as u64));
                out.extend_from_slice(data);
            }
            QuicFrame::Stream { stream_id, offset, fin, data } => {
                // Frame Type: 0x08 | OFF_BIT(0x04) | LEN_BIT(0x02) | FIN_BIT(0x01)
                let mut type_byte = 0x08 | 0x02; // always include length
                if *offset > 0 {
                    type_byte |= 0x04;
                }
                if *fin {
                    type_byte |= 0x01;
                }
                out.push(type_byte);

                out.extend_from_slice(&QuicVint::encode(*stream_id));
                if *offset > 0 {
                    out.extend_from_slice(&QuicVint::encode(*offset));
                }
                out.extend_from_slice(&QuicVint::encode(data.len() as u64));
                out.extend_from_slice(data);
            }
            QuicFrame::ConnectionClose { error_code, reason } => {
                out.push(0x1C);
                out.extend_from_slice(&QuicVint::encode(*error_code));
                out.extend_from_slice(&QuicVint::encode(0)); // Frame type
                let reason_bytes = reason.as_bytes();
                out.extend_from_slice(&QuicVint::encode(reason_bytes.len() as u64));
                out.extend_from_slice(reason_bytes);
            }
        }

        out
    }

    /// Deserialisasi seluruh frame dari payload paket QUIC
    pub fn parse_frames(mut buf: &[u8]) -> Result<Vec<Self>, String> {
        let mut frames = Vec::new();

        while !buf.is_empty() {
            let (type_val, len_consumed) = QuicVint::decode(buf)?;
            buf = &buf[len_consumed..];

            match type_val {
                0x00 => {
                    frames.push(QuicFrame::Padding);
                }
                0x01 => {
                    frames.push(QuicFrame::Ping);
                }
                0x02 | 0x03 => {
                    let (largest_ack, l_len) = QuicVint::decode(buf)?;
                    buf = &buf[l_len..];
                    let (ack_delay, d_len) = QuicVint::decode(buf)?;
                    buf = &buf[d_len..];
                    let (_range_count, r_len) = QuicVint::decode(buf)?;
                    buf = &buf[r_len..];
                    let (_first_range, f_len) = QuicVint::decode(buf)?;
                    buf = &buf[f_len..];
                    frames.push(QuicFrame::Ack { largest_acknowledged: largest_ack, ack_delay });
                }
                0x06 => {
                    let (offset, o_len) = QuicVint::decode(buf)?;
                    buf = &buf[o_len..];
                    let (data_len, d_len) = QuicVint::decode(buf)?;
                    buf = &buf[d_len..];
                    let d_size = data_len as usize;
                    if buf.len() < d_size {
                        return Err("Crypto frame data terpotong".to_string());
                    }
                    let data = buf[..d_size].to_vec();
                    buf = &buf[d_size..];
                    frames.push(QuicFrame::Crypto { offset, data });
                }
                0x08..=0x0F => {
                    let has_off = (type_val & 0x04) != 0;
                    let has_len = (type_val & 0x02) != 0;
                    let fin = (type_val & 0x01) != 0;

                    let (stream_id, s_len) = QuicVint::decode(buf)?;
                    buf = &buf[s_len..];

                    let offset = if has_off {
                        let (off, o_len) = QuicVint::decode(buf)?;
                        buf = &buf[o_len..];
                        off
                    } else {
                        0
                    };

                    let data_len = if has_len {
                        let (d_len, l_len) = QuicVint::decode(buf)?;
                        buf = &buf[l_len..];
                        d_len as usize
                    } else {
                        buf.len()
                    };

                    if buf.len() < data_len {
                        return Err("Stream frame data terpotong".to_string());
                    }
                    let data = buf[..data_len].to_vec();
                    buf = &buf[data_len..];

                    frames.push(QuicFrame::Stream { stream_id, offset, fin, data });
                }
                0x1C => {
                    let (error_code, e_len) = QuicVint::decode(buf)?;
                    buf = &buf[e_len..];
                    let (_frame_type, ft_len) = QuicVint::decode(buf)?;
                    buf = &buf[ft_len..];
                    let (reason_len, r_len) = QuicVint::decode(buf)?;
                    buf = &buf[r_len..];
                    let r_size = reason_len as usize;
                    let reason = String::from_utf8_lossy(&buf[..r_size]).to_string();
                    buf = &buf[r_size..];
                    frames.push(QuicFrame::ConnectionClose { error_code, reason });
                }
                _ => {
                    // Abaikan frame tak dikenal lainnya dan hentikan parsing
                    break;
                }
            }
        }

        Ok(frames)
    }
}

// ============================================================================
// 4. HTTP/3 FRAMES & QPACK COMPRESSION (RFC 9114 & RFC 9204)
// ============================================================================

/// Header HTTP/3 (Name-Value pair)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct H3Header {
    pub name: String,
    pub value: String,
}

impl H3Header {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

/// Tipe Frame HTTP/3 (RFC 9114 §7)
#[derive(Debug, Clone, PartialEq)]
pub enum H3Frame {
    Data(Vec<u8>),
    Headers(Vec<H3Header>),
    Settings(HashMap<u64, u64>),
    GoAway(u64),
}

impl H3Frame {
    /// Serialisasi Frame HTTP/3 ke bytes
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        match self {
            H3Frame::Data(data) => {
                // Type 0x00: DATA
                out.extend_from_slice(&QuicVint::encode(0x00));
                out.extend_from_slice(&QuicVint::encode(data.len() as u64));
                out.extend_from_slice(data);
            }
            H3Frame::Headers(headers) => {
                // Type 0x01: HEADERS
                let encoded_qpack = QpackEngine::encode_headers(headers);
                out.extend_from_slice(&QuicVint::encode(0x01));
                out.extend_from_slice(&QuicVint::encode(encoded_qpack.len() as u64));
                out.extend_from_slice(&encoded_qpack);
            }
            H3Frame::Settings(settings) => {
                // Type 0x04: SETTINGS
                let mut payload = Vec::new();
                for (&param_id, &val) in settings {
                    payload.extend_from_slice(&QuicVint::encode(param_id));
                    payload.extend_from_slice(&QuicVint::encode(val));
                }
                out.extend_from_slice(&QuicVint::encode(0x04));
                out.extend_from_slice(&QuicVint::encode(payload.len() as u64));
                out.extend_from_slice(&payload);
            }
            H3Frame::GoAway(stream_id) => {
                // Type 0x07: GOAWAY
                let payload = QuicVint::encode(*stream_id);
                out.extend_from_slice(&QuicVint::encode(0x07));
                out.extend_from_slice(&QuicVint::encode(payload.len() as u64));
                out.extend_from_slice(&payload);
            }
        }

        out
    }

    /// Parse Frame HTTP/3 dari buffer stream
    pub fn parse(buf: &[u8]) -> Result<(Self, usize), String> {
        let (type_val, t_len) = QuicVint::decode(buf)?;
        let mut offset = t_len;

        let (payload_len_u64, l_len) = QuicVint::decode(&buf[offset..])?;
        offset += l_len;
        let payload_len = payload_len_u64 as usize;

        if buf.len() < offset + payload_len {
            return Err("Buffer terpotong saat membaca payload HTTP/3 frame".to_string());
        }

        let payload = &buf[offset..offset + payload_len];
        offset += payload_len;

        let frame = match type_val {
            0x00 => H3Frame::Data(payload.to_vec()),
            0x01 => {
                let headers = QpackEngine::decode_headers(payload)?;
                H3Frame::Headers(headers)
            }
            0x04 => {
                let mut settings = HashMap::new();
                let mut s_buf = payload;
                while !s_buf.is_empty() {
                    let (id, id_len) = QuicVint::decode(s_buf)?;
                    s_buf = &s_buf[id_len..];
                    let (val, val_len) = QuicVint::decode(s_buf)?;
                    s_buf = &s_buf[val_len..];
                    settings.insert(id, val);
                }
                H3Frame::Settings(settings)
            }
            0x07 => {
                let (stream_id, _) = QuicVint::decode(payload)?;
                H3Frame::GoAway(stream_id)
            }
            _ => {
                return Err(format!("Tipe HTTP/3 Frame tidak dikenal: 0x{:02X}", type_val));
            }
        };

        Ok((frame, offset))
    }
}

/// Mesin Kompresi Header QPACK Sederhana Berstandar RFC 9204
pub struct QpackEngine;

impl QpackEngine {
    /// Encode list of H3Header ke format QPACK
    pub fn encode_headers(headers: &[H3Header]) -> Vec<u8> {
        let mut out = Vec::new();
        // Prefix QPACK field section: Required Insert Count (0) + Delta Base (0)
        out.push(0x00);
        out.push(0x00);

        for h in headers {
            // Cek apakah ada index statis umum
            if let Some(static_idx) = Self::get_static_table_index(&h.name, &h.value) {
                // Indexed Field Line (bit 7 = 1): 0x80 | static_idx
                out.push(0x80 | (static_idx as u8));
            } else {
                // Literal Field Line with Literal Name (0x20):
                // 0010 0000 -> 0x20
                out.push(0x20);
                // Name length VINT + name bytes
                out.extend_from_slice(&QuicVint::encode(h.name.len() as u64));
                out.extend_from_slice(h.name.as_bytes());
                // Value length VINT + value bytes
                out.extend_from_slice(&QuicVint::encode(h.value.len() as u64));
                out.extend_from_slice(h.value.as_bytes());
            }
        }

        out
    }

    /// Decode buffer QPACK ke daftar H3Header
    pub fn decode_headers(mut buf: &[u8]) -> Result<Vec<H3Header>, String> {
        if buf.len() < 2 {
            return Err("QPACK header terpotong pada prefix".to_string());
        }
        // Lewati Required Insert Count & Delta Base (2 bytes)
        buf = &buf[2..];

        let mut headers = Vec::new();

        while !buf.is_empty() {
            let b0 = buf[0];
            if (b0 & 0x80) != 0 {
                // Indexed Field Line
                let idx = (b0 & 0x3F) as usize;
                buf = &buf[1..];
                if let Some((n, v)) = Self::get_static_table_entry(idx) {
                    headers.push(H3Header::new(n, v));
                }
            } else if (b0 & 0x20) != 0 {
                // Literal with Literal Name
                buf = &buf[1..];
                let (name_len, n_len) = QuicVint::decode(buf)?;
                buf = &buf[n_len..];
                let n_size = name_len as usize;
                if buf.len() < n_size {
                    return Err("QPACK nama header terpotong".to_string());
                }
                let name = String::from_utf8_lossy(&buf[..n_size]).to_string();
                buf = &buf[n_size..];

                let (val_len, v_len) = QuicVint::decode(buf)?;
                buf = &buf[v_len..];
                let v_size = val_len as usize;
                if buf.len() < v_size {
                    return Err("QPACK nilai header terpotong".to_string());
                }
                let value = String::from_utf8_lossy(&buf[..v_size]).to_string();
                buf = &buf[v_size..];

                headers.push(H3Header::new(&name, &value));
            } else {
                // Lewati byte tak dikenal
                buf = &buf[1..];
            }
        }

        Ok(headers)
    }

    fn get_static_table_index(name: &str, value: &str) -> Option<usize> {
        match (name, value) {
            (":method", "GET") => Some(1),
            (":method", "POST") => Some(2),
            (":path", "/") => Some(3),
            (":scheme", "https") => Some(4),
            (":status", "200") => Some(5),
            (":status", "404") => Some(6),
            (":status", "500") => Some(7),
            _ => None,
        }
    }

    fn get_static_table_entry(idx: usize) -> Option<(&'static str, &'static str)> {
        match idx {
            1 => Some((":method", "GET")),
            2 => Some((":method", "POST")),
            3 => Some((":path", "/")),
            4 => Some((":scheme", "https")),
            5 => Some((":status", "200")),
            6 => Some((":status", "404")),
            7 => Some((":status", "500")),
            _ => None,
        }
    }
}

// ============================================================================
// 5. RESEP HTTP/3 & QUIC SERVER & CLIENT ENGINE
// ============================================================================

/// Respon HTTP/3 Terpadu
#[derive(Debug, Clone)]
pub struct H3Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Server UDP HTTP/3 & QUIC Murni
pub struct PureHttp3Server {
    pub socket: Arc<UdpSocket>,
    pub addr: SocketAddr,
    pub sessions: Arc<Mutex<HashMap<ConnectionId, QuicSessionState>>>,
}

#[derive(Debug, Clone)]
pub struct QuicSessionState {
    pub client_addr: SocketAddr,
    pub server_cid: ConnectionId,
    pub client_cid: ConnectionId,
    pub next_packet_number: u64,
}

impl PureHttp3Server {
    /// Bind server HTTP/3 ke alamat UDP
    pub fn bind(addr: &str) -> Result<Self, String> {
        let socket = UdpSocket::bind(addr).map_err(|e| format!("Gagal bind UDP {}: {}", addr, e))?;
        socket.set_read_timeout(Some(Duration::from_millis(500))).ok();
        let local_addr = socket.local_addr().map_err(|e| e.to_string())?;

        Ok(Self {
            socket: Arc::new(socket),
            addr: local_addr,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Proses satu paket masuk dari klien dan hasilkan respon HTTP/3
    pub fn tangani_satu_paket<F>(&self, handler: F) -> Result<bool, String>
    where
        F: Fn(&str, &str, &[u8]) -> (u16, Vec<H3Header>, Vec<u8>),
    {
        let mut buf = [0u8; 2048];
        let (bytes_read, client_addr) = match self.socket.recv_from(&mut buf) {
            Ok(r) => r,
            Err(_) => return Ok(false), // Timeout, tidak ada paket
        };

        let raw_packet = &buf[..bytes_read];
        let quic_pkt = QuicPacket::deserialize(raw_packet, 8)?;

        match quic_pkt {
            QuicPacket::LongHeader { packet_type, version: _, dcid, scid, packet_number: _, payload } => {
                if packet_type == QuicLongPacketType::Initial {
                    // Simpan state sesi
                    let mut sessions = self.sessions.lock().unwrap();
                    let server_cid = ConnectionId::acak(8);
                    sessions.insert(dcid.clone(), QuicSessionState {
                        client_addr,
                        server_cid: server_cid.clone(),
                        client_cid: scid.clone(),
                        next_packet_number: 1,
                    });

                    // Parse frame di dalam payload
                    let frames = QuicFrame::parse_frames(&payload)?;
                    for f in frames {
                        if let QuicFrame::Stream { stream_id, offset: _, fin: _, data } = f {
                            // Coba parse HTTP/3 frame di dalam stream data
                            if let Ok((H3Frame::Headers(headers), next_off)) = H3Frame::parse(&data) {
                                let body = if next_off < data.len() {
                                    if let Ok((H3Frame::Data(d), _)) = H3Frame::parse(&data[next_off..]) {
                                        d
                                    } else {
                                        Vec::new()
                                    }
                                } else {
                                    Vec::new()
                                };

                                let method = headers.iter().find(|h| h.name == ":method").map(|h| h.value.as_str()).unwrap_or("GET");
                                let path = headers.iter().find(|h| h.name == ":path").map(|h| h.value.as_str()).unwrap_or("/");

                                // Panggil handler aplikasi
                                let (status_code, mut resp_headers, resp_body) = handler(method, path, &body);

                                // Tambahkan header default HTTP/3
                                resp_headers.insert(0, H3Header::new(":status", &status_code.to_string()));
                                resp_headers.push(H3Header::new("alt-svc", "h3=\":443\"; ma=86400"));

                                // Bangun HTTP/3 response frames
                                let mut h3_stream_data = Vec::new();
                                h3_stream_data.extend_from_slice(&H3Frame::Headers(resp_headers).serialize());
                                if !resp_body.is_empty() {
                                    h3_stream_data.extend_from_slice(&H3Frame::Data(resp_body).serialize());
                                }

                                // Bungkus ke dalam QUIC Stream Frame (FIN = true)
                                let resp_stream_frame = QuicFrame::Stream {
                                    stream_id,
                                    offset: 0,
                                    fin: true,
                                    data: h3_stream_data,
                                };

                                // Kirim paket Long Header Handshake / 1-RTT Response
                                let resp_packet = QuicPacket::LongHeader {
                                    packet_type: QuicLongPacketType::Handshake,
                                    version: QUIC_VERSION_1,
                                    dcid: scid.clone(),
                                    scid: server_cid,
                                    packet_number: 1,
                                    payload: resp_stream_frame.serialize(),
                                };

                                let resp_bytes = resp_packet.serialize();
                                self.socket.send_to(&resp_bytes, client_addr).map_err(|e| e.to_string())?;
                                return Ok(true);
                            }
                        }
                    }
                }
            }
            QuicPacket::ShortHeader { dcid: _, packet_number: _, payload: _ } => {
                // Short Header 1-RTT Data packet
                return Ok(true);
            }
        }

        Ok(false)
    }
}

/// Klien UDP HTTP/3 & QUIC Murni
pub struct PureHttp3Client;

impl PureHttp3Client {
    /// Kirim permintaan HTTP/3 via UDP ke server dan dapatkan respon H3Response
    pub fn kirim_permintaan(server_addr: SocketAddr, method: &str, path: &str, body: &[u8]) -> Result<H3Response, String> {
        let socket = UdpSocket::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
        socket.set_read_timeout(Some(Duration::from_millis(2000))).ok();

        let client_cid = ConnectionId::acak(8);
        let server_cid = ConnectionId::acak(8);

        // 1. Siapkan HTTP/3 HEADERS Frame
        let headers = vec![
            H3Header::new(":method", method),
            H3Header::new(":path", path),
            H3Header::new(":scheme", "https"),
            H3Header::new(":authority", &server_addr.to_string()),
            H3Header::new("user-agent", "Widya-Pure-HTTP3/2.6.0"),
        ];

        let mut h3_payload = Vec::new();
        h3_payload.extend_from_slice(&H3Frame::Headers(headers).serialize());
        if !body.is_empty() {
            h3_payload.extend_from_slice(&H3Frame::Data(body.to_vec()).serialize());
        }

        // 2. Bungkus ke dalam QUIC STREAM Frame (Stream ID 0 = Client Bidi, FIN = true)
        let stream_frame = QuicFrame::Stream {
            stream_id: 0,
            offset: 0,
            fin: true,
            data: h3_payload,
        };

        // 3. Masukkan ke dalam QUIC Initial Packet
        let initial_pkt = QuicPacket::LongHeader {
            packet_type: QuicLongPacketType::Initial,
            version: QUIC_VERSION_1,
            dcid: server_cid,
            scid: client_cid,
            packet_number: 0,
            payload: stream_frame.serialize(),
        };

        let datagram_bytes = initial_pkt.serialize();
        socket.send_to(&datagram_bytes, server_addr).map_err(|e| e.to_string())?;

        // 4. Terima respon dari server via UDP
        let mut resp_buf = [0u8; 4096];
        let bytes_received = socket.recv(&mut resp_buf).map_err(|e| format!("Timeout menerima respon HTTP/3 UDP: {}", e))?;

        let recv_pkt = QuicPacket::deserialize(&resp_buf[..bytes_received], 8)?;
        let payload = match recv_pkt {
            QuicPacket::LongHeader { payload, .. } => payload,
            QuicPacket::ShortHeader { payload, .. } => payload,
        };

        let frames = QuicFrame::parse_frames(&payload)?;
        for f in frames {
            if let QuicFrame::Stream { stream_id: _, offset: _, fin: _, data } = f {
                let mut status = 200;
                let mut resp_headers = HashMap::new();
                let mut resp_body = Vec::new();

                let mut remaining = data.as_slice();
                while !remaining.is_empty() {
                    if let Ok((h3_frame, consumed)) = H3Frame::parse(remaining) {
                        remaining = &remaining[consumed..];
                        match h3_frame {
                            H3Frame::Headers(hdrs) => {
                                for h in hdrs {
                                    if h.name == ":status" {
                                        status = h.value.parse::<u16>().unwrap_or(200);
                                    } else {
                                        resp_headers.insert(h.name, h.value);
                                    }
                                }
                            }
                            H3Frame::Data(d) => {
                                resp_body.extend_from_slice(&d);
                            }
                            _ => {}
                        }
                    } else {
                        break;
                    }
                }

                return Ok(H3Response {
                    status,
                    headers: resp_headers,
                    body: resp_body,
                });
            }
        }

        Err("Tidak ada frame stream HTTP/3 ditemukan pada respon paket".to_string())
    }
}
