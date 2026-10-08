// ============================================================================
// WIDYABASE PGWIRE v3.0 NETWORK DAEMON (server.rs)
// ============================================================================
// Server TCP mandiri berkinerja tinggi yang mengimplementasikan PostgreSQL
// Wire Protocol v3.0 (Port default 5432).
// Mengizinkan psql, DBeaver, Prisma, atau driver PG standar terhubung langsung!
// ============================================================================

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use crate::planner::{ExecutionEngine, QueryResult};
use crate::sql_parser::{SqlLexer, SqlParser, SqlValue};

pub struct PgwireServer {
    engine: Arc<ExecutionEngine>,
    port: u16,
}

impl PgwireServer {
    pub fn new(engine: Arc<ExecutionEngine>, port: u16) -> Self {
        PgwireServer { engine, port }
    }

    pub fn start(&self) -> std::io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr)?;
        println!("🚀 [WidyaBase] Server aktif mendengarkan di {} (pgwire v3.0)", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(socket) => {
                    let eng = Arc::clone(&self.engine);
                    thread::spawn(move || {
                        if let Err(e) = handle_client(socket, eng) {
                            eprintln!("⚠️ [WidyaBase Client Error] {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("⚠️ [WidyaBase Connection Error] {}", e);
                }
            }
        }
        Ok(())
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PreparedStatement {
    pub name: String,
    pub query: String,
    pub param_types: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct Portal {
    pub name: String,
    pub statement_name: String,
    pub query: String,
    pub param_values: Vec<Option<Vec<u8>>>,
    pub param_formats: Vec<i16>,
    pub result_formats: Vec<i16>,
}

fn handle_client(mut stream: TcpStream, engine: Arc<ExecutionEngine>) -> std::io::Result<()> {
    let mut session_vars: HashMap<String, String> = HashMap::new();
    session_vars.insert("client_encoding".to_string(), "UTF8".to_string());
    session_vars.insert("server_version".to_string(), "15.0 (WidyaBase 1.0)".to_string());
    session_vars.insert("server_encoding".to_string(), "UTF8".to_string());
    session_vars.insert("standard_conforming_strings".to_string(), "on".to_string());
    session_vars.insert("TimeZone".to_string(), "UTC".to_string());
    session_vars.insert("integer_datetimes".to_string(), "on".to_string());
    session_vars.insert("search_path".to_string(), "public".to_string());

    // 1. Baca Paket Inisialisasi Pertama
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let msg_len = i32::from_be_bytes(len_buf) as usize;

    let mut body = vec![0u8; msg_len - 4];
    stream.read_exact(&mut body)?;

    let proto_ver = i32::from_be_bytes(body[0..4].try_into().unwrap());

    // Cek SSL Request (80877103)
    let startup_body = if proto_ver == 80877103 {
        // Balas 'N' (Tidak mendukung SSL plain socket)
        stream.write_all(b"N")?;
        stream.flush()?;

        // Baca StartupMessage berikutnya
        stream.read_exact(&mut len_buf)?;
        let startup_len = i32::from_be_bytes(len_buf) as usize;
        let mut s_body = vec![0u8; startup_len - 4];
        stream.read_exact(&mut s_body)?;
        s_body
    } else if proto_ver == 80877102 {
        // Cancel Request: abaikan dan tutup koneksi
        return Ok(());
    } else {
        body
    };

    // Ekstrak parameter dari StartupMessage (user, database, application_name, dll.)
    if startup_body.len() > 4 {
        let params_slice = &startup_body[4..];
        let mut idx = 0;
        while idx < params_slice.len() {
            let mut key_end = idx;
            while key_end < params_slice.len() && params_slice[key_end] != 0 {
                key_end += 1;
            }
            if key_end >= params_slice.len() || key_end == idx {
                break;
            }
            let key = String::from_utf8_lossy(&params_slice[idx..key_end]).to_string();
            idx = key_end + 1;

            let mut val_end = idx;
            while val_end < params_slice.len() && params_slice[val_end] != 0 {
                val_end += 1;
            }
            let val = String::from_utf8_lossy(&params_slice[idx..val_end]).to_string();
            idx = val_end + 1;

            session_vars.insert(key, val);
        }
    }

    // 2. Kirim AuthenticationOk (Pesan 'R', length 8, code 0)
    let auth_ok = [b'R', 0, 0, 0, 8, 0, 0, 0, 0];
    stream.write_all(&auth_ok)?;

    // 3. Kirim BackendKeyData (Pesan 'K', length 12: process_id i32 + secret_key i32)
    let pid = (std::process::id() as i32).to_be_bytes();
    let secret = 42424242i32.to_be_bytes();
    let mut backend_key = vec![b'K', 0, 0, 0, 12];
    backend_key.extend_from_slice(&pid);
    backend_key.extend_from_slice(&secret);
    stream.write_all(&backend_key)?;

    // 4. Kirim ParameterStatus lengkap standar PostgreSQL
    send_parameter_status(&mut stream, "server_version", "15.0 (WidyaBase 1.0)")?;
    send_parameter_status(&mut stream, "server_encoding", "UTF8")?;
    send_parameter_status(&mut stream, "client_encoding", "UTF8")?;
    send_parameter_status(&mut stream, "is_superuser", "on")?;
    send_parameter_status(&mut stream, "session_authorization", session_vars.get("user").map(|s| s.as_str()).unwrap_or("widya"))?;
    send_parameter_status(&mut stream, "standard_conforming_strings", "on")?;
    send_parameter_status(&mut stream, "TimeZone", "UTC")?;
    send_parameter_status(&mut stream, "integer_datetimes", "on")?;
    if let Some(app) = session_vars.get("application_name") {
        send_parameter_status(&mut stream, "application_name", app)?;
    }

    // 5. Kirim ReadyForQuery ('Z', length 5, status 'I' = Idle)
    send_ready_for_query(&mut stream)?;

    // 6. State Manajemen Prepared Statements & Portals untuk Extended Query Protocol
    let mut prepared_statements: HashMap<String, PreparedStatement> = HashMap::new();
    let mut portals: HashMap<String, Portal> = HashMap::new();

    // 7. Query Processing Loop (Mendukung Simple Query 'Q' dan Extended Protocol 'P','B','D','E','S','H','C')
    loop {
        let mut type_buf = [0u8; 1];
        if stream.read_exact(&mut type_buf).is_err() {
            break; // Klien menutup koneksi
        }
        let msg_type = type_buf[0];

        stream.read_exact(&mut len_buf)?;
        let q_len = i32::from_be_bytes(len_buf) as usize;
        let mut q_body = vec![0u8; q_len - 4];
        stream.read_exact(&mut q_body)?;

        match msg_type {
            b'Q' => {
                // Simple Query Protocol
                let query_str = String::from_utf8_lossy(&q_body)
                    .trim_matches('\0')
                    .to_string();

                if query_str.trim().is_empty() || query_str.trim() == ";" {
                    send_ready_for_query(&mut stream)?;
                    continue;
                }

                execute_and_respond(&mut stream, &engine, &query_str, &[0], &mut session_vars)?;
                send_ready_for_query(&mut stream)?;
            }
            b'P' => {
                // Extended Query: Parse ('P')
                let mut cursor = 0;
                let stmt_name = read_null_terminated_string(&q_body, &mut cursor);
                let query = read_null_terminated_string(&q_body, &mut cursor);

                let mut param_types = Vec::new();
                if cursor + 2 <= q_body.len() {
                    let num_params = i16::from_be_bytes(q_body[cursor..cursor + 2].try_into().unwrap()) as usize;
                    cursor += 2;
                    for _ in 0..num_params {
                        if cursor + 4 <= q_body.len() {
                            let oid = i32::from_be_bytes(q_body[cursor..cursor + 4].try_into().unwrap());
                            param_types.push(oid);
                            cursor += 4;
                        }
                    }
                }

                prepared_statements.insert(
                    stmt_name.clone(),
                    PreparedStatement {
                        name: stmt_name,
                        query,
                        param_types,
                    },
                );

                // Kirim ParseComplete ('1', length 4)
                stream.write_all(&[b'1', 0, 0, 0, 4])?;
                stream.flush()?;
            }
            b'B' => {
                // Extended Query: Bind ('B')
                let mut cursor = 0;
                let portal_name = read_null_terminated_string(&q_body, &mut cursor);
                let stmt_name = read_null_terminated_string(&q_body, &mut cursor);

                let mut param_formats = Vec::new();
                if cursor + 2 <= q_body.len() {
                    let num_formats = i16::from_be_bytes(q_body[cursor..cursor + 2].try_into().unwrap()) as usize;
                    cursor += 2;
                    for _ in 0..num_formats {
                        if cursor + 2 <= q_body.len() {
                            let fmt = i16::from_be_bytes(q_body[cursor..cursor + 2].try_into().unwrap());
                            param_formats.push(fmt);
                            cursor += 2;
                        }
                    }
                }

                let mut param_values = Vec::new();
                if cursor + 2 <= q_body.len() {
                    let num_params = i16::from_be_bytes(q_body[cursor..cursor + 2].try_into().unwrap()) as usize;
                    cursor += 2;
                    for _ in 0..num_params {
                        if cursor + 4 <= q_body.len() {
                            let plen = i32::from_be_bytes(q_body[cursor..cursor + 4].try_into().unwrap());
                            cursor += 4;
                            if plen == -1 {
                                param_values.push(None); // NULL
                            } else if plen >= 0 {
                                let len_usize = plen as usize;
                                if cursor + len_usize <= q_body.len() {
                                    let val_bytes = q_body[cursor..cursor + len_usize].to_vec();
                                    param_values.push(Some(val_bytes));
                                    cursor += len_usize;
                                }
                            }
                        }
                    }
                }

                let mut result_formats = Vec::new();
                if cursor + 2 <= q_body.len() {
                    let num_r_formats = i16::from_be_bytes(q_body[cursor..cursor + 2].try_into().unwrap()) as usize;
                    cursor += 2;
                    for _ in 0..num_r_formats {
                        if cursor + 2 <= q_body.len() {
                            let fmt = i16::from_be_bytes(q_body[cursor..cursor + 2].try_into().unwrap());
                            result_formats.push(fmt);
                            cursor += 2;
                        }
                    }
                }

                let query = prepared_statements
                    .get(&stmt_name)
                    .map(|s| s.query.clone())
                    .unwrap_or_default();

                portals.insert(
                    portal_name.clone(),
                    Portal {
                        name: portal_name,
                        statement_name: stmt_name,
                        query,
                        param_values,
                        param_formats,
                        result_formats,
                    },
                );

                // Kirim BindComplete ('2', length 4)
                stream.write_all(&[b'2', 0, 0, 0, 4])?;
                stream.flush()?;
            }
            b'D' => {
                // Extended Query: Describe ('D')
                if !q_body.is_empty() {
                    let desc_type = q_body[0];
                    let mut cursor = 1;
                    let target_name = read_null_terminated_string(&q_body, &mut cursor);

                    if desc_type == b'S' {
                        // Describe Statement
                        if let Some(stmt) = prepared_statements.get(&target_name) {
                            // Kirim ParameterDescription ('t')
                            let mut payload = Vec::new();
                            payload.extend_from_slice(&(stmt.param_types.len() as i16).to_be_bytes());
                            for &oid in &stmt.param_types {
                                payload.extend_from_slice(&oid.to_be_bytes());
                            }
                            let plen = (payload.len() + 4) as i32;
                            stream.write_all(b"t")?;
                            stream.write_all(&plen.to_be_bytes())?;
                            stream.write_all(&payload)?;

                            // RowDescription atau NoData
                            let q_upper = stmt.query.trim().to_uppercase();
                            if q_upper.starts_with("SELECT") || q_upper.starts_with("PILIH") || q_upper.starts_with("SHOW") {
                                send_row_description(&mut stream, &["result".to_string()], &[0])?;
                            } else {
                                stream.write_all(&[b'n', 0, 0, 0, 4])?; // NoData
                            }
                        } else {
                            send_error_response(&mut stream, "Prepared statement tidak ditemukan")?;
                        }
                    } else if desc_type == b'P' {
                        // Describe Portal
                        if let Some(portal) = portals.get(&target_name) {
                            let q_upper = portal.query.trim().to_uppercase();
                            if q_upper.starts_with("SELECT") || q_upper.starts_with("PILIH") || q_upper.starts_with("SHOW") {
                                send_row_description(&mut stream, &["result".to_string()], &portal.result_formats)?;
                            } else {
                                stream.write_all(&[b'n', 0, 0, 0, 4])?; // NoData
                            }
                        } else {
                            send_error_response(&mut stream, "Portal tidak ditemukan")?;
                        }
                    }
                    stream.flush()?;
                }
            }
            b'E' => {
                // Extended Query: Execute ('E')
                let mut cursor = 0;
                let portal_name = read_null_terminated_string(&q_body, &mut cursor);
                let _max_rows = if cursor + 4 <= q_body.len() {
                    i32::from_be_bytes(q_body[cursor..cursor + 4].try_into().unwrap())
                } else {
                    0
                };

                if let Some(portal) = portals.get(&portal_name).cloned() {
                    // Substitusi nilai parameter ($1, $2, dst.) ke kueri
                    let mut final_query = portal.query.clone();
                    for (i, p_val) in portal.param_values.iter().enumerate() {
                        let param_placeholder = format!("${}", i + 1);
                        let val_str = match p_val {
                            Some(bytes) => {
                                let is_binary = portal.param_formats.get(i).copied().unwrap_or(0) == 1;
                                if is_binary && bytes.len() == 4 {
                                    i32::from_be_bytes(bytes[..4].try_into().unwrap()).to_string()
                                } else if is_binary && bytes.len() == 8 {
                                    i64::from_be_bytes(bytes[..8].try_into().unwrap()).to_string()
                                } else {
                                    let s = String::from_utf8_lossy(bytes);
                                    format!("'{}'", s.replace('\'', "''"))
                                }
                            }
                            None => "NULL".to_string(),
                        };
                        final_query = final_query.replace(&param_placeholder, &val_str);
                    }

                    execute_and_respond(&mut stream, &engine, &final_query, &portal.result_formats, &mut session_vars)?;
                } else {
                    send_error_response(&mut stream, &format!("Portal '{}' tidak ditemukan", portal_name))?;
                }
            }
            b'S' => {
                // Extended Query: Sync ('S')
                send_ready_for_query(&mut stream)?;
            }
            b'H' => {
                // Extended Query: Flush ('H')
                stream.flush()?;
            }
            b'C' => {
                // Extended Query: Close ('C')
                if !q_body.is_empty() {
                    let close_type = q_body[0];
                    let mut cursor = 1;
                    let target_name = read_null_terminated_string(&q_body, &mut cursor);
                    if close_type == b'S' {
                        prepared_statements.remove(&target_name);
                    } else if close_type == b'P' {
                        portals.remove(&target_name);
                    }
                    // Kirim CloseComplete ('3', length 4)
                    stream.write_all(&[b'3', 0, 0, 0, 4])?;
                    stream.flush()?;
                }
            }
            b'X' => {
                // Terminate
                break;
            }
            _ => {
                // Tipe pesan lain yang tidak dikenal, abaikan dan kirim ReadyForQuery
                send_ready_for_query(&mut stream)?;
            }
        }
    }

    Ok(())
}

fn read_null_terminated_string(buf: &[u8], cursor: &mut usize) -> String {
    let start = *cursor;
    while *cursor < buf.len() && buf[*cursor] != 0 {
        *cursor += 1;
    }
    let s = String::from_utf8_lossy(&buf[start..*cursor]).to_string();
    if *cursor < buf.len() {
        *cursor += 1; // lewati byte null
    }
    s
}

/// Mengeksekusi query SQL dengan dukungan introspeksi driver (DBeaver, Prisma, Grafana, dll.)
fn execute_and_respond(
    stream: &mut TcpStream,
    engine: &Arc<ExecutionEngine>,
    query_str: &str,
    format_codes: &[i16],
    session_vars: &mut HashMap<String, String>,
) -> std::io::Result<()> {
    let trimmed = query_str.trim().trim_end_matches(';').trim();
    let upper = trimmed.to_uppercase();

    // 1. Tangani perintah konfigurasi sesi SET
    if upper.starts_with("SET ") || upper.starts_with("ATUR ") {
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let key = parts[1].trim_matches(|c| c == '"' || c == '\'');
            let val = if parts.len() >= 4 && (parts[2] == "=" || parts[2].eq_ignore_ascii_case("TO")) {
                parts[3..].join(" ").trim_matches(|c| c == '"' || c == '\'').to_string()
            } else {
                "on".to_string()
            };
            session_vars.insert(key.to_string(), val);
        }
        send_command_complete(stream, "SET")?;
        return Ok(());
    }

    // 2. Tangani kueri SHOW
    if upper.starts_with("SHOW ") || upper.starts_with("TAMPILKAN ") {
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let var_name = parts.get(1).map(|s| s.trim_matches(';')).unwrap_or("search_path");
        let val = session_vars.get(var_name).cloned().unwrap_or_else(|| match var_name.to_lowercase().as_str() {
            "search_path" => "public".to_string(),
            "client_encoding" => "UTF8".to_string(),
            "server_version" => "15.0 (WidyaBase 1.0)".to_string(),
            "transaction_isolation" | "transaction isolation level" => "read committed".to_string(),
            "standard_conforming_strings" => "on".to_string(),
            "max_identifier_length" => "63".to_string(),
            "timezone" => "UTC".to_string(),
            _ => "default".to_string(),
        });

        let columns = vec![var_name.to_string()];
        let row = vec![SqlValue::Text(val)];
        send_row_description(stream, &columns, format_codes)?;
        send_data_row(stream, &row, format_codes)?;
        send_command_complete(stream, "SHOW")?;
        return Ok(());
    }

    // 3. Tangani transaksi blok
    if upper == "BEGIN" || upper == "START TRANSACTION" {
        send_command_complete(stream, "BEGIN")?;
        return Ok(());
    }
    if upper == "COMMIT" {
        send_command_complete(stream, "COMMIT")?;
        return Ok(());
    }
    if upper == "ROLLBACK" {
        send_command_complete(stream, "ROLLBACK")?;
        return Ok(());
    }
    if upper.starts_with("DISCARD") {
        send_command_complete(stream, "DISCARD ALL")?;
        return Ok(());
    }

    // 4. Tangani introspeksi umum (SELECT version(), current_schema(), 1, dll.)
    if upper == "SELECT 1" || upper == "SELECT 1;" {
        send_row_description(stream, &["?column?".to_string()], format_codes)?;
        send_data_row(stream, &[SqlValue::Integer(1)], format_codes)?;
        send_command_complete(stream, "SELECT 1")?;
        return Ok(());
    }
    if upper.contains("VERSION()") {
        send_row_description(stream, &["version".to_string()], format_codes)?;
        send_data_row(stream, &[SqlValue::Text("PostgreSQL 15.0 (WidyaBase 1.0.0)".to_string())], format_codes)?;
        send_command_complete(stream, "SELECT 1")?;
        return Ok(());
    }
    if upper.contains("CURRENT_SCHEMA()") || upper.contains("CURRENT_SCHEMA") {
        send_row_description(stream, &["current_schema".to_string()], format_codes)?;
        send_data_row(stream, &[SqlValue::Text("public".to_string())], format_codes)?;
        send_command_complete(stream, "SELECT 1")?;
        return Ok(());
    }
    if upper.contains("CURRENT_DATABASE()") {
        send_row_description(stream, &["current_database".to_string()], format_codes)?;
        send_data_row(stream, &[SqlValue::Text("widyabase".to_string())], format_codes)?;
        send_command_complete(stream, "SELECT 1")?;
        return Ok(());
    }
    if upper.contains("PG_BACKEND_PID()") {
        send_row_description(stream, &["pg_backend_pid".to_string()], format_codes)?;
        send_data_row(stream, &[SqlValue::Integer(std::process::id() as i64)], format_codes)?;
        send_command_complete(stream, "SELECT 1")?;
        return Ok(());
    }

    // 5. Eksekusi Kueri Normal melalui Engine
    let mut lexer = SqlLexer::new(query_str);
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = SqlParser::new(tokens);
            match parser.parse_statement() {
                Ok(stmt) => match engine.execute(stmt, 1) {
                    Ok(res) => match res {
                        QueryResult::Message(msg) => {
                            send_command_complete(stream, &msg)?;
                        }
                        QueryResult::Rows { columns, rows, affected } => {
                            send_row_description(stream, &columns, format_codes)?;
                            for row in &rows {
                                send_data_row(stream, row, format_codes)?;
                            }
                            send_command_complete(stream, &format!("SELECT {}", affected))?;
                        }
                    },
                    Err(err) => {
                        send_error_response(stream, &err)?;
                    }
                },
                Err(parse_err) => {
                    send_error_response(stream, &parse_err)?;
                }
            }
        }
        Err(lex_err) => {
            send_error_response(stream, &lex_err)?;
        }
    }

    Ok(())
}

fn send_parameter_status(stream: &mut TcpStream, name: &str, value: &str) -> std::io::Result<()> {
    let mut payload = Vec::new();
    payload.extend_from_slice(name.as_bytes());
    payload.push(0);
    payload.extend_from_slice(value.as_bytes());
    payload.push(0);

    let len = (payload.len() + 4) as i32;
    stream.write_all(b"S")?;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&payload)?;
    stream.flush()?;
    Ok(())
}

fn send_ready_for_query(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write_all(&[b'Z', 0, 0, 0, 5, b'I'])?;
    stream.flush()?;
    Ok(())
}

fn send_command_complete(stream: &mut TcpStream, tag: &str) -> std::io::Result<()> {
    let mut payload = tag.as_bytes().to_vec();
    payload.push(0);
    let len = (payload.len() + 4) as i32;
    stream.write_all(b"C")?;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&payload)?;
    stream.flush()?;
    Ok(())
}

fn send_error_response(stream: &mut TcpStream, message: &str) -> std::io::Result<()> {
    let mut payload = Vec::new();
    payload.push(b'S'); // Severity
    payload.extend_from_slice(b"ERROR\0");
    payload.push(b'C'); // Code
    payload.extend_from_slice(b"42000\0");
    payload.push(b'M'); // Message
    payload.extend_from_slice(message.as_bytes());
    payload.push(0);
    payload.push(0); // Terminator

    let len = (payload.len() + 4) as i32;
    stream.write_all(b"E")?;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&payload)?;
    stream.flush()?;
    Ok(())
}

fn send_row_description(stream: &mut TcpStream, columns: &[String], format_codes: &[i16]) -> std::io::Result<()> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&(columns.len() as i16).to_be_bytes());

    for (idx, col) in columns.iter().enumerate() {
        let fmt = if format_codes.len() == 1 {
            format_codes[0]
        } else if idx < format_codes.len() {
            format_codes[idx]
        } else {
            0
        };
        payload.extend_from_slice(col.as_bytes());
        payload.push(0); // null term
        payload.extend_from_slice(&0i32.to_be_bytes()); // table oid
        payload.extend_from_slice(&((idx + 1) as i16).to_be_bytes()); // column attr num
        payload.extend_from_slice(&25i32.to_be_bytes()); // data type OID (25 = TEXT)
        payload.extend_from_slice(&(-1i16).to_be_bytes()); // data type size (-1 varlen)
        payload.extend_from_slice(&0i32.to_be_bytes()); // type modifier
        payload.extend_from_slice(&fmt.to_be_bytes()); // format code (0 = text, 1 = binary)
    }

    let len = (payload.len() + 4) as i32;
    stream.write_all(b"T")?;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&payload)?;
    stream.flush()?;
    Ok(())
}

fn send_data_row(stream: &mut TcpStream, row: &[SqlValue], format_codes: &[i16]) -> std::io::Result<()> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&(row.len() as i16).to_be_bytes());

    for (idx, val) in row.iter().enumerate() {
        let fmt = if format_codes.len() == 1 {
            format_codes[0]
        } else if idx < format_codes.len() {
            format_codes[idx]
        } else {
            0
        };

        match val {
            SqlValue::Null => {
                payload.extend_from_slice(&(-1i32).to_be_bytes()); // NULL indicator
            }
            SqlValue::Integer(i) => {
                if fmt == 1 {
                    // Binary format: 8-byte big-endian int64
                    payload.extend_from_slice(&8i32.to_be_bytes());
                    payload.extend_from_slice(&i.to_be_bytes());
                } else {
                    let s = i.to_string();
                    let bytes = s.as_bytes();
                    payload.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
                    payload.extend_from_slice(bytes);
                }
            }
            SqlValue::Float(f) => {
                if fmt == 1 {
                    // Binary format: 8-byte IEEE 754 float64
                    payload.extend_from_slice(&8i32.to_be_bytes());
                    payload.extend_from_slice(&f.to_bits().to_be_bytes());
                } else {
                    let s = f.to_string();
                    let bytes = s.as_bytes();
                    payload.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
                    payload.extend_from_slice(bytes);
                }
            }
            SqlValue::Boolean(b) => {
                if fmt == 1 {
                    // Binary format: 1 byte
                    payload.extend_from_slice(&1i32.to_be_bytes());
                    payload.push(if *b { 1 } else { 0 });
                } else {
                    let s = if *b { "t" } else { "f" };
                    payload.extend_from_slice(&(s.len() as i32).to_be_bytes());
                    payload.extend_from_slice(s.as_bytes());
                }
            }
            SqlValue::Text(t) => {
                let bytes = t.as_bytes();
                payload.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
                payload.extend_from_slice(bytes);
            }
        }
    }

    let len = (payload.len() + 4) as i32;
    stream.write_all(b"D")?;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&payload)?;
    stream.flush()?;
    Ok(())
}

// ============================================================================
// WIDYABASE RESP (REDIS SERIALIZATION PROTOCOL) NETWORK DAEMON
// ============================================================================
// Melayani klien Redis (redis-cli, ioredis, redis-py, jedis) di Port 6379.
// ============================================================================

pub struct RespServer {
    engine: Arc<ExecutionEngine>,
    port: u16,
}

impl RespServer {
    pub fn new(engine: Arc<ExecutionEngine>, port: u16) -> Self {
        RespServer { engine, port }
    }

    pub fn start(&self) -> std::io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr)?;
        println!("🚀 [WidyaBase] Redis-compatible Cache Server aktif di {} (RESP v2/v3)", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(socket) => {
                    let cache = Arc::clone(&self.engine.cache_engine);
                    thread::spawn(move || {
                        if let Err(e) = handle_resp_client(socket, cache) {
                            eprintln!("⚠️ [WidyaBase Redis Client Error] {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("⚠️ [WidyaBase Redis Connection Error] {}", e);
                }
            }
        }
        Ok(())
    }
}

fn handle_resp_client(mut stream: TcpStream, cache: Arc<crate::cache::CacheEngine>) -> std::io::Result<()> {
    use std::io::BufRead;
    let mut reader = std::io::BufReader::new(stream.try_clone()?);

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break; // Klien terputus
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let args = if trimmed.starts_with('*') {
            // RESP Array (*<count>\r\n$<len>\r\n<arg>\r\n...)
            let num_args: usize = trimmed[1..].parse().unwrap_or(0);
            let mut parsed_args = Vec::new();

            for _ in 0..num_args {
                let mut len_line = String::new();
                if reader.read_line(&mut len_line)? == 0 { break; }
                let len_trim = len_line.trim();
                if len_trim.starts_with('$') {
                    let arg_len: usize = len_trim[1..].parse().unwrap_or(0);
                    let mut arg_buf = vec![0u8; arg_len];
                    reader.read_exact(&mut arg_buf)?;
                    let mut crlf = [0u8; 2];
                    reader.read_exact(&mut crlf)?;
                    parsed_args.push(String::from_utf8_lossy(&arg_buf).to_string());
                }
            }
            parsed_args
        } else {
            // Inline command (misal: "PING" atau "SET key value")
            trimmed.split_whitespace().map(|s| s.to_string()).collect()
        };

        if !args.is_empty() {
            let cmd_str = args.join(" ");
            let resp_str = match cache.execute_redis_command(&cmd_str) {
                Ok(res) => res,
                Err(err) => crate::cache::CacheEngine::format_resp_error(&err),
            };
            stream.write_all(resp_str.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
