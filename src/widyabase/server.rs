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
use crate::widyabase::planner::{ExecutionEngine, QueryResult};
use crate::widyabase::sql_parser::{SqlLexer, SqlParser, SqlValue};

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

fn handle_client(mut stream: TcpStream, engine: Arc<ExecutionEngine>) -> std::io::Result<()> {
    // 1. Baca Paket Inisialisasi Pertama
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let msg_len = i32::from_be_bytes(len_buf) as usize;

    let mut body = vec![0u8; msg_len - 4];
    stream.read_exact(&mut body)?;

    let proto_ver = i32::from_be_bytes(body[0..4].try_into().unwrap());

    // Cek SSL Request (80877103)
    if proto_ver == 80877103 {
        // Balas 'N' (Tidak mendukung SSL plain socket)
        stream.write_all(b"N")?;
        stream.flush()?;

        // Baca StartupMessage berikutnya
        stream.read_exact(&mut len_buf)?;
        let startup_len = i32::from_be_bytes(len_buf) as usize;
        let mut startup_body = vec![0u8; startup_len - 4];
        stream.read_exact(&mut startup_body)?;
    }

    // 2. Kirim AuthenticationOk (Pesan 'R', length 8, code 0)
    let auth_ok = [b'R', 0, 0, 0, 8, 0, 0, 0, 0];
    stream.write_all(&auth_ok)?;

    // 3. Kirim ParameterStatus (server_version, client_encoding)
    send_parameter_status(&mut stream, "server_version", "1.0.0-widyabase")?;
    send_parameter_status(&mut stream, "client_encoding", "UTF8")?;

    // 4. Kirim ReadyForQuery ('Z', length 5, status 'I' = Idle)
    send_ready_for_query(&mut stream)?;

    // 5. Query Processing Loop
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
                // Simple Query
                let query_str = String::from_utf8_lossy(&q_body)
                    .trim_matches('\0')
                    .to_string();

                if query_str.trim().is_empty() || query_str.trim() == ";" {
                    send_ready_for_query(&mut stream)?;
                    continue;
                }

                // Tokenize & Parse
                let mut lexer = SqlLexer::new(&query_str);
                match lexer.tokenize() {
                    Ok(tokens) => {
                        let mut parser = SqlParser::new(tokens);
                        match parser.parse_statement() {
                            Ok(stmt) => {
                                match engine.execute(stmt, 1) {
                                    Ok(res) => match res {
                                        QueryResult::Message(msg) => {
                                            send_command_complete(&mut stream, &msg)?;
                                        }
                                        QueryResult::Rows { columns, rows, affected } => {
                                            send_row_description(&mut stream, &columns)?;
                                            for row in &rows {
                                                send_data_row(&mut stream, row)?;
                                            }
                                            send_command_complete(&mut stream, &format!("SELECT {}", affected))?;
                                        }
                                    },
                                    Err(err) => {
                                        send_error_response(&mut stream, &err)?;
                                    }
                                }
                            }
                            Err(parse_err) => {
                                send_error_response(&mut stream, &parse_err)?;
                            }
                        }
                    }
                    Err(lex_err) => {
                        send_error_response(&mut stream, &lex_err)?;
                    }
                }

                send_ready_for_query(&mut stream)?;
            }
            b'X' => {
                // Terminate
                break;
            }
            _ => {
                // Unhandled message type, send ReadyForQuery
                send_ready_for_query(&mut stream)?;
            }
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

fn send_row_description(stream: &mut TcpStream, columns: &[String]) -> std::io::Result<()> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&(columns.len() as i16).to_be_bytes());

    for col in columns {
        payload.extend_from_slice(col.as_bytes());
        payload.push(0); // null term
        payload.extend_from_slice(&0i32.to_be_bytes()); // table oid
        payload.extend_from_slice(&0i16.to_be_bytes()); // column attr num
        payload.extend_from_slice(&25i32.to_be_bytes()); // data type OID (25 = TEXT)
        payload.extend_from_slice(&(-1i16).to_be_bytes()); // data type size (-1 varlen)
        payload.extend_from_slice(&0i32.to_be_bytes()); // type modifier
        payload.extend_from_slice(&0i16.to_be_bytes()); // format code (0 = text)
    }

    let len = (payload.len() + 4) as i32;
    stream.write_all(b"T")?;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&payload)?;
    stream.flush()?;
    Ok(())
}

fn send_data_row(stream: &mut TcpStream, row: &[SqlValue]) -> std::io::Result<()> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&(row.len() as i16).to_be_bytes());

    for val in row {
        let s = match val {
            SqlValue::Null => {
                payload.extend_from_slice(&(-1i32).to_be_bytes()); // NULL indicator
                continue;
            }
            SqlValue::Boolean(b) => if *b { "t" } else { "f" }.to_string(),
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Text(t) => t.clone(),
        };
        let bytes = s.as_bytes();
        payload.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
        payload.extend_from_slice(bytes);
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

fn handle_resp_client(mut stream: TcpStream, cache: Arc<crate::widyabase::cache::CacheEngine>) -> std::io::Result<()> {
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
                Err(err) => crate::widyabase::cache::CacheEngine::format_resp_error(&err),
            };
            stream.write_all(resp_str.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
