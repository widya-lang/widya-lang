//! Sovereign WidyaBase Wire Protocol & Client (Pilar IV: Akses Basis Data Murni Bebas Driver C-FFI)
//! 100% Bebas Ketergantungan Eksternal (Zero External Crates: Tanpa postgres, rusqlite, mysql, duckdb).
//! Mengimplementasikan Wire Protocol v3.0 berbasis raw `std::net::TcpStream` untuk komunikasi klien-server WidyaBase.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct WireRow {
    pub values: Vec<Option<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WireQueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<WireRow>,
    pub tag: String,
    pub affected_rows: usize,
}

pub struct WidyaWireClient {
    stream: TcpStream,
    pub user: String,
    pub database: String,
}

impl WidyaWireClient {
    /// Hubungkan ke server WidyaBase melalui soket TCP mentah
    pub fn connect(host: &str, port: u16, user: &str, database: &str) -> Result<Self, String> {
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect_timeout(
            &addr.to_socket_addrs().map_err(|e| e.to_string())?.next().ok_or("Host tidak valid")?,
            Duration::from_secs(5),
        ).map_err(|e| format!("Gagal menghubungi peladen WidyaBase pada {}: {}", addr, e))?;

        stream.set_read_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;
        stream.set_write_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;

        // 1. Kirim StartupMessage PGWire v3.0
        // Format: Length (i32) + Protocol Version 196608 (i32) + Key/Value strings + '\0'
        let mut body = Vec::new();
        body.extend_from_slice(&196608i32.to_be_bytes()); // Versi protokol 3.0
        body.extend_from_slice(b"user\0");
        body.extend_from_slice(user.as_bytes());
        body.push(0);
        body.extend_from_slice(b"database\0");
        body.extend_from_slice(database.as_bytes());
        body.push(0);
        body.extend_from_slice(b"client_encoding\0UTF8\0");
        body.push(0); // Terminasi akhir

        let total_len = (body.len() + 4) as i32;
        let mut packet = Vec::new();
        packet.extend_from_slice(&total_len.to_be_bytes());
        packet.extend_from_slice(&body);

        stream.write_all(&packet).map_err(|e| format!("Gagal mengirim startup packet: {}", e))?;
        stream.flush().map_err(|e| e.to_string())?;

        // 2. Baca handshake balasan sampai pesan ReadyForQuery ('Z')
        loop {
            let mut msg_type = [0u8; 1];
            if stream.read_exact(&mut msg_type).is_err() {
                break;
            }
            let mut len_bytes = [0u8; 4];
            stream.read_exact(&mut len_bytes).map_err(|e| e.to_string())?;
            let len = (i32::from_be_bytes(len_bytes) - 4) as usize;
            let mut payload = vec![0u8; len];
            stream.read_exact(&mut payload).map_err(|e| e.to_string())?;

            match msg_type[0] {
                b'R' => {
                    // AuthenticationOk atau Auth request
                }
                b'S' => {
                    // ParameterStatus
                }
                b'K' => {
                    // BackendKeyData
                }
                b'Z' => {
                    // ReadyForQuery: handshake selesai, server siap menerima kueri!
                    break;
                }
                b'E' => {
                    let err_msg = String::from_utf8_lossy(&payload);
                    return Err(format!("Galat handshake WidyaBase: {}", err_msg));
                }
                _ => {}
            }
        }

        Ok(Self {
            stream,
            user: user.to_string(),
            database: database.to_string(),
        })
    }

    /// Eksekusi SQL Query murni melalui format Query ('Q')
    pub fn query(&mut self, sql: &str) -> Result<WireQueryResult, String> {
        // Kirim Query frame: 'Q' + (len + 4) + sql + '\0'
        let mut body = sql.as_bytes().to_vec();
        body.push(0);
        let len = (body.len() + 4) as i32;

        let mut frame = Vec::with_capacity(body.len() + 5);
        frame.push(b'Q');
        frame.extend_from_slice(&len.to_be_bytes());
        frame.extend_from_slice(&body);

        self.stream.write_all(&frame).map_err(|e| format!("Gagal mengirim kueri SQL: {}", e))?;
        self.stream.flush().map_err(|e| e.to_string())?;

        let mut columns = Vec::new();
        let mut rows = Vec::new();
        let mut command_tag = String::new();
        let mut affected = 0usize;

        // Baca balasan frame dari peladen
        loop {
            let mut msg_type = [0u8; 1];
            self.stream.read_exact(&mut msg_type).map_err(|e| e.to_string())?;
            let mut len_bytes = [0u8; 4];
            self.stream.read_exact(&mut len_bytes).map_err(|e| e.to_string())?;
            let payload_len = (i32::from_be_bytes(len_bytes) - 4) as usize;
            let mut payload = vec![0u8; payload_len];
            self.stream.read_exact(&mut payload).map_err(|e| e.to_string())?;

            match msg_type[0] {
                b'T' => {
                    // RowDescription
                    columns = Self::parse_row_description(&payload)?;
                }
                b'D' => {
                    // DataRow
                    let row = Self::parse_data_row(&payload)?;
                    rows.push(row);
                }
                b'C' => {
                    // CommandComplete
                    command_tag = String::from_utf8_lossy(&payload).trim_matches('\0').to_string();
                    if let Some(last_word) = command_tag.split_whitespace().last() {
                        affected = last_word.parse::<usize>().unwrap_or(rows.len());
                    }
                }
                b'E' => {
                    // ErrorResponse
                    return Err(format!("Galat Eksekusi WidyaBase: {}", String::from_utf8_lossy(&payload)));
                }
                b'Z' => {
                    // ReadyForQuery: selesai
                    break;
                }
                _ => {}
            }
        }

        Ok(WireQueryResult {
            columns,
            rows,
            tag: command_tag,
            affected_rows: affected,
        })
    }

    fn parse_row_description(payload: &[u8]) -> Result<Vec<String>, String> {
        if payload.len() < 2 {
            return Ok(Vec::new());
        }
        let col_count = u16::from_be_bytes([payload[0], payload[1]]) as usize;
        let mut cols = Vec::with_capacity(col_count);
        let mut offset = 2;

        for _ in 0..col_count {
            let mut end = offset;
            while end < payload.len() && payload[end] != 0 {
                end += 1;
            }
            let name = String::from_utf8_lossy(&payload[offset..end]).to_string();
            cols.push(name);
            offset = end + 1 + 18; // Lewati field metadata (table_oid, col_attr, type_oid, type_len, type_mod, format)
        }
        Ok(cols)
    }

    fn parse_data_row(payload: &[u8]) -> Result<WireRow, String> {
        if payload.len() < 2 {
            return Ok(WireRow { values: Vec::new() });
        }
        let col_count = u16::from_be_bytes([payload[0], payload[1]]) as usize;
        let mut values = Vec::with_capacity(col_count);
        let mut offset = 2;

        for _ in 0..col_count {
            if offset + 4 > payload.len() {
                break;
            }
            let val_len = i32::from_be_bytes([
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            ]);
            offset += 4;

            if val_len == -1 {
                values.push(None); // Nilai NULL
            } else {
                let len = val_len as usize;
                if offset + len <= payload.len() {
                    let val_str = String::from_utf8_lossy(&payload[offset..offset + len]).to_string();
                    values.push(Some(val_str));
                    offset += len;
                } else {
                    values.push(None);
                }
            }
        }
        Ok(WireRow { values })
    }
}

// ============================================================================
// UNIT TESTS UNTUK PROTOKOL WIDYA-WIRE
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row_description() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&2u16.to_be_bytes()); // 2 kolom

        // Kolom 1: "id"
        payload.extend_from_slice(b"id\0");
        payload.extend_from_slice(&[0u8; 18]); // metadata

        // Kolom 2: "nama"
        payload.extend_from_slice(b"nama\0");
        payload.extend_from_slice(&[0u8; 18]); // metadata

        let cols = WidyaWireClient::parse_row_description(&payload).unwrap();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0], "id");
        assert_eq!(cols[1], "nama");
    }

    #[test]
    fn test_parse_data_row() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&2u16.to_be_bytes()); // 2 kolom

        // Nilai 1: "101"
        let val1 = b"101";
        payload.extend_from_slice(&(val1.len() as i32).to_be_bytes());
        payload.extend_from_slice(val1);

        // Nilai 2: "Budi Santoso"
        let val2 = "Budi Santoso".as_bytes();
        payload.extend_from_slice(&(val2.len() as i32).to_be_bytes());
        payload.extend_from_slice(val2);

        let row = WidyaWireClient::parse_data_row(&payload).unwrap();
        assert_eq!(row.values.len(), 2);
        assert_eq!(row.values[0].as_deref(), Some("101"));
        assert_eq!(row.values[1].as_deref(), Some("Budi Santoso"));
    }
}
