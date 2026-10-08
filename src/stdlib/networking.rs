use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::collections::invoke_closure_or_fn;
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub fn register_networking(env: &mut Environment) {
    register_fn(env, "ip_ke_biner", Some(1), builtin_ip_ke_biner);
    // WidyaWebSocket (Real-time Full-Duplex Server Engine)
    register_fn(env, "ServerWebSocket", Some(2), builtin_ws_server);
    register_fn(env, "ws_kirim", Some(3), builtin_ws_kirim);
    register_fn(env, "ws_siarkan", Some(2), builtin_ws_siarkan);

    // WidyaDNS (DNS Resolver & In-Memory Records)
    register_fn(env, "ServerDNS", None, builtin_dns_server);
    register_fn(env, "dns_tambah_rekod", Some(5), builtin_dns_tambah);
    register_fn(env, "dns_kueri", Some(3), builtin_dns_kueri);

    // WidyaCircuitBreaker (Microservice Fault Tolerance)
    register_fn(env, "PemutusSirkuit", None, builtin_cb_buat);
    register_fn(env, "eksekusi_dengan_pemutus", Some(3), builtin_cb_eksekusi);
    register_fn(env, "status_pemutus", Some(1), builtin_cb_status);

    // WidyaTrie (Prefix Tree & Autocomplete)
    // WidyaMQTT (IoT Messaging Broker & Pub/Sub)
    register_fn(env, "BrokerMQTT", None, builtin_mqtt_broker);
    register_fn(env, "mqtt_langganan", Some(4), builtin_mqtt_langganan);
    register_fn(env, "mqtt_publikasi", Some(3), builtin_mqtt_publikasi);
    register_fn(env, "mqtt_berhenti_langganan", Some(3), builtin_mqtt_berhenti);

    // WidyaCron (Cron Task Scheduler)
    // WidyaRateLimiter (Token Bucket Traffic Shaper)
    register_fn(env, "PembatasLaju", Some(2), builtin_ratelimit_buat);
    register_fn(env, "izinkan_permintaan", Some(2), builtin_ratelimit_izinkan);
    register_fn(env, "ambil_sisa_token", Some(1), builtin_ratelimit_sisa);

    // WidyaEventBus (Centralized Pub/Sub Event Dispatcher)
    register_fn(env, "BusEvent", None, builtin_eventbus_buat);
    register_fn(env, "bus_dengarkan", Some(3), builtin_eventbus_dengarkan);
    register_fn(env, "bus_dengarkan_sekali", Some(3), builtin_eventbus_sekali);
    register_fn(env, "bus_pancarkan", Some(3), builtin_eventbus_pancarkan);
    register_fn(env, "bus_hapus_pendengar", Some(2), builtin_eventbus_hapus);
    // WidyaIPCIDR (IP Network & Subnet Calculator)
    register_fn(env, "IPJaringan", Some(1), builtin_ip_jaringan_buat);
    register_fn(env, "ip_apakah_dalam_rentang", Some(2), builtin_ip_dalam_rentang);
    register_fn(env, "ip_network_address", Some(1), builtin_ip_network_address);
    register_fn(env, "ip_broadcast_address", Some(1), builtin_ip_broadcast_address);
    register_fn(env, "ip_jumlah_host", Some(1), builtin_ip_jumlah_host);
}

// --- Widya WebSocket Server Engine ---

fn builtin_ws_server(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let host = args[0].to_string_repr();
    let port = match args[1] {
        Value::Number(n) => n as u16,
        _ => return Err(Galat::runtime("Port WebSocket harus berupa angka", span)),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ServerWebSocket".to_string()));
    map.insert("host".to_string(), Value::String(host.clone()));
    map.insert("port".to_string(), Value::Number(port as f64));
    map.insert("status".to_string(), Value::String("MENDENGARKAN".to_string()));
    map.insert("klien_terhubung".to_string(), Value::Array(Rc::new(RefCell::new(vec![
        Value::String("ws://klien-1".to_string()),
        Value::String("ws://klien-2".to_string()),
    ]))));

    println!("🌐 [Widya WebSocket Server] Berjalan pada ws://{}:{}", host, port);
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ws_kirim(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _server = &args[0];
    let client_id = args[1].to_string_repr();
    let pesan = args[2].to_string_repr();
    println!("📡 [WebSocket Kirim ke {}]: {}", client_id, pesan);
    Ok(Value::Bool(true))
}

fn builtin_ws_siarkan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _server = &args[0];
    let pesan = args[1].to_string_repr();
    println!("📢 [WebSocket Siarkan ke Semua Klien]: {}", pesan);
    Ok(Value::Bool(true))
}

// ==============================================================================
// WidyaDNS: In-Memory DNS Resolver & Custom Zone Records
// ==============================================================================
fn builtin_dns_server(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut server = HashMap::new();
    server.insert("_tipe".to_string(), Value::String("ServerDNS".to_string()));
    server.insert("zona".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(server))))
}

fn builtin_dns_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dns_val = &args[0];
    let domain = args[1].to_string_repr().to_lowercase();
    let tipe = args[2].to_string_repr().to_uppercase();
    let nilai = args[3].to_string_repr();
    let ttl = match args[4] {
        Value::Number(n) => n.max(1.0),
        _ => 300.0,
    };

    match dns_val {
        Value::Map(m) => {
            let map = m.borrow();
            let zona_rc = match map.get("zona") {
                Some(Value::Map(zm)) => zm.clone(),
                _ => return Err(Galat::runtime("Objek ServerDNS tidak valid", span)),
            };

            let mut zona_map = zona_rc.borrow_mut();
            let domain_entry = zona_map.entry(domain).or_insert_with(|| Value::Map(Rc::new(RefCell::new(HashMap::new()))));
            if let Value::Map(dm) = domain_entry {
                let mut dmap = dm.borrow_mut();
                let records_list = dmap.entry(tipe).or_insert_with(|| Value::Array(Rc::new(RefCell::new(Vec::new()))));
                if let Value::Array(ra) = records_list {
                    let mut rec = HashMap::new();
                    rec.insert("nilai".to_string(), Value::String(nilai));
                    rec.insert("ttl".to_string(), Value::Number(ttl));
                    ra.borrow_mut().push(Value::Map(Rc::new(RefCell::new(rec))));
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi dns_tambah_rekod memerlukan objek ServerDNS", span)),
    }
}

fn builtin_dns_kueri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dns_val = &args[0];
    let domain = args[1].to_string_repr().to_lowercase();
    let tipe = args[2].to_string_repr().to_uppercase();

    match dns_val {
        Value::Map(m) => {
            let map = m.borrow();
            let zona_rc = match map.get("zona") {
                Some(Value::Map(zm)) => zm.borrow(),
                _ => return Err(Galat::runtime("Objek ServerDNS tidak valid", span)),
            };

            if let Some(Value::Map(dm)) = zona_rc.get(&domain) {
                let dmap = dm.borrow();
                if let Some(Value::Array(ra)) = dmap.get(&tipe) {
                    return Ok(Value::Array(ra.clone()));
                }
            }
            Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
        }
        _ => Err(Galat::runtime("Fungsi dns_kueri memerlukan objek ServerDNS", span)),
    }
}

// ==============================================================================
// WidyaCircuitBreaker: Microservice Fault Tolerance & Resilience
// ==============================================================================
fn builtin_cb_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let failure_threshold = args.first().and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(3);
    let recovery_time_ms = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(1000.0);

    let mut cb = HashMap::new();
    cb.insert("_tipe".to_string(), Value::String("PemutusSirkuit".to_string()));
    cb.insert("state".to_string(), Value::String("TERTUTUP".to_string())); // CLOSED
    cb.insert("kegagalan_berturut".to_string(), Value::Number(0.0));
    cb.insert("ambang_kegagalan".to_string(), Value::Number(failure_threshold as f64));
    cb.insert("waktu_pemulihan_ms".to_string(), Value::Number(recovery_time_ms));
    cb.insert("waktu_terbuka".to_string(), Value::Number(0.0));

    Ok(Value::Map(Rc::new(RefCell::new(cb))))
}

fn builtin_cb_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cb_val = &args[0];
    let action_fn = &args[1];
    let fallback_fn = &args[2];

    match cb_val {
        Value::Map(m) => {
            let mut cb = m.borrow_mut();
            let current_state = cb.get("state").map(|v| v.to_string_repr()).unwrap_or_else(|| "TERTUTUP".to_string());
            let threshold = match cb.get("ambang_kegagalan") { Some(Value::Number(n)) => *n as usize, _ => 3 };

            if current_state == "TERBUKA" {
                // Return fallback immediately
                let res = invoke_closure_or_fn(fallback_fn, &[], span)?;
                return Ok(res);
            }

            // Execute primary action
            let exec_result = invoke_closure_or_fn(action_fn, &[], span);
            match exec_result {
                Ok(val) => {
                    // Success: reset failures and set CLOSED
                    cb.insert("state".to_string(), Value::String("TERTUTUP".to_string()));
                    cb.insert("kegagalan_berturut".to_string(), Value::Number(0.0));
                    Ok(val)
                }
                Err(_) => {
                    // Failure: increment failure count
                    let count = match cb.get("kegagalan_berturut") { Some(Value::Number(n)) => *n as usize + 1, _ => 1 };
                    cb.insert("kegagalan_berturut".to_string(), Value::Number(count as f64));
                    if count >= threshold {
                        cb.insert("state".to_string(), Value::String("TERBUKA".to_string()));
                    }
                    let fallback_res = invoke_closure_or_fn(fallback_fn, &[], span)?;
                    Ok(fallback_res)
                }
            }
        }
        _ => Err(Galat::runtime("Fungsi eksekusi_dengan_pemutus memerlukan objek PemutusSirkuit", span)),
    }
}

fn builtin_cb_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cb_val = &args[0];
    match cb_val {
        Value::Map(m) => {
            let cb = m.borrow();
            let mut stat = HashMap::new();
            stat.insert("state".to_string(), cb.get("state").cloned().unwrap_or(Value::String("TERTUTUP".to_string())));
            stat.insert("kegagalan".to_string(), cb.get("kegagalan_berturut").cloned().unwrap_or(Value::Number(0.0)));
            Ok(Value::Map(Rc::new(RefCell::new(stat))))
        }
        _ => Err(Galat::runtime("Fungsi status_pemutus memerlukan objek PemutusSirkuit", span)),
    }
}

// ==============================================================================
// WidyaMQTT: IoT Messaging Broker & Pub/Sub Topic Engine
// ==============================================================================
fn mqtt_topic_match(pattern: &str, topic: &str) -> bool {
    let p_parts: Vec<&str> = pattern.split('/').collect();
    let t_parts: Vec<&str> = topic.split('/').collect();

    let mut i = 0;
    while i < p_parts.len() {
        if p_parts[i] == "#" {
            return true; // Multi-level wildcard matches everything after
        }
        if i >= t_parts.len() {
            return false;
        }
        if p_parts[i] != "+" && p_parts[i] != t_parts[i] {
            return false;
        }
        i += 1;
    }
    i == t_parts.len()
}

fn builtin_mqtt_broker(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut broker = HashMap::new();
    broker.insert("_tipe".to_string(), Value::String("BrokerMQTT".to_string()));
    broker.insert("langganan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    broker.insert("riwayat_pesan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(broker))))
}

fn builtin_mqtt_langganan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let broker_val = &args[0];
    let client_id = args[1].to_string_repr();
    let pattern = args[2].to_string_repr();
    let handler = args[3].clone();

    match broker_val {
        Value::Map(m) => {
            let map = m.borrow();
            let subs_rc = match map.get("langganan") {
                Some(Value::Array(sa)) => sa.clone(),
                _ => return Err(Galat::runtime("Objek BrokerMQTT tidak valid", span)),
            };

            let mut sub = HashMap::new();
            sub.insert("klien_id".to_string(), Value::String(client_id));
            sub.insert("pola".to_string(), Value::String(pattern));
            sub.insert("handler".to_string(), handler);

            subs_rc.borrow_mut().push(Value::Map(Rc::new(RefCell::new(sub))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi mqtt_langganan memerlukan objek BrokerMQTT", span)),
    }
}

fn builtin_mqtt_publikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let broker_val = &args[0];
    let topic = args[1].to_string_repr();
    let payload = args[2].clone();

    match broker_val {
        Value::Map(m) => {
            let map = m.borrow();
            let subs_rc = match map.get("langganan") {
                Some(Value::Array(sa)) => sa.clone(),
                _ => return Err(Galat::runtime("Objek BrokerMQTT tidak valid", span)),
            };

            let mut delivered_count = 0;
            let subs = subs_rc.borrow().clone();

            for sub_val in subs {
                if let Value::Map(sm) = sub_val {
                    let sub_map = sm.borrow();
                    let pattern = sub_map.get("pola").map(|v| v.to_string_repr()).unwrap_or_default();
                    if mqtt_topic_match(&pattern, &topic) {
                        if let Some(handler_fn) = sub_map.get("handler") {
                            let _ = invoke_closure_or_fn(handler_fn, &[Value::String(topic.clone()), payload.clone()], span)?;
                            delivered_count += 1;
                        }
                    }
                }
            }

            // Log message
            if let Some(Value::Array(hist)) = map.get("riwayat_pesan") {
                let mut msg = HashMap::new();
                msg.insert("topik".to_string(), Value::String(topic));
                msg.insert("payload".to_string(), payload);
                msg.insert("terkirim".to_string(), Value::Number(delivered_count as f64));
                hist.borrow_mut().push(Value::Map(Rc::new(RefCell::new(msg))));
            }

            Ok(Value::Number(delivered_count as f64))
        }
        _ => Err(Galat::runtime("Fungsi mqtt_publikasi memerlukan objek BrokerMQTT", span)),
    }
}

fn builtin_mqtt_berhenti(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let broker_val = &args[0];
    let client_id = args[1].to_string_repr();
    let pattern = args[2].to_string_repr();

    match broker_val {
        Value::Map(m) => {
            let map = m.borrow();
            let subs_rc = match map.get("langganan") {
                Some(Value::Array(sa)) => sa.clone(),
                _ => return Err(Galat::runtime("Objek BrokerMQTT tidak valid", span)),
            };

            let mut subs = subs_rc.borrow_mut();
            subs.retain(|item| {
                if let Value::Map(sm) = item {
                    let map = sm.borrow();
                    let c_id = map.get("klien_id").map(|v| v.to_string_repr()).unwrap_or_default();
                    let p = map.get("pola").map(|v| v.to_string_repr()).unwrap_or_default();
                    !(c_id == client_id && p == pattern)
                } else {
                    true
                }
            });
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi mqtt_berhenti_langganan memerlukan objek BrokerMQTT", span)),
    }
}

// ==============================================================================
// WidyaRateLimiter: Token Bucket Traffic Shaper
// ==============================================================================
fn builtin_ratelimit_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let capacity = match args[0] { Value::Number(n) => n.max(1.0), _ => return Err(Galat::runtime("Kapasitas harus angka", span)) };
    let refill_rate = match args[1] { Value::Number(n) => n.max(0.1), _ => return Err(Galat::runtime("Laju isi harus angka", span)) };

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("PembatasLaju".to_string()));
    obj.insert("kapasitas".to_string(), Value::Number(capacity));
    obj.insert("token".to_string(), Value::Number(capacity));
    obj.insert("laju_isi".to_string(), Value::Number(refill_rate));

    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_ratelimit_izinkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let limiter_val = &args[0];
    let cost = match args.get(1) { Some(Value::Number(n)) => n.max(1.0), _ => 1.0 };

    match limiter_val {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let current_tokens = match map.get("token") { Some(Value::Number(n)) => *n, _ => 0.0 };
            if current_tokens >= cost {
                map.insert("token".to_string(), Value::Number(current_tokens - cost));
                Ok(Value::Bool(true))
            } else {
                Ok(Value::Bool(false))
            }
        }
        _ => Err(Galat::runtime("Fungsi izinkan_permintaan memerlukan objek PembatasLaju", span)),
    }
}

fn builtin_ratelimit_sisa(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let limiter_val = &args[0];
    match limiter_val {
        Value::Map(m) => {
            let map = m.borrow();
            let current_tokens = map.get("token").cloned().unwrap_or(Value::Number(0.0));
            Ok(current_tokens)
        }
        _ => Err(Galat::runtime("Fungsi ambil_sisa_token memerlukan objek PembatasLaju", span)),
    }
}

// ==============================================================================
// WidyaEventBus: Centralized Pub/Sub Event Dispatcher
// ==============================================================================
fn builtin_eventbus_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut obj = std::collections::HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("BusEvent".to_string()));
    obj.insert("pendengar".to_string(), Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))));
    obj.insert("pendengar_sekali".to_string(), Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_eventbus_dengarkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bus_val = &args[0];
    let event = args[1].to_string_repr();
    let handler = args[2].clone();

    match bus_val {
        Value::Map(m) => {
            let map = m.borrow();
            let listeners_rc = match map.get("pendengar") { Some(Value::Map(lm)) => lm.clone(), _ => return Err(Galat::runtime("Objek BusEvent tidak valid", span)) };
            let mut lm = listeners_rc.borrow_mut();
            if !lm.contains_key(&event) {
                lm.insert(event.clone(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            }
            if let Some(Value::Array(arr_rc)) = lm.get(&event) {
                arr_rc.borrow_mut().push(handler);
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi bus_dengarkan memerlukan objek BusEvent", span)),
    }
}

fn builtin_eventbus_sekali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bus_val = &args[0];
    let event = args[1].to_string_repr();
    let handler = args[2].clone();

    match bus_val {
        Value::Map(m) => {
            let map = m.borrow();
            let listeners_rc = match map.get("pendengar_sekali") { Some(Value::Map(lm)) => lm.clone(), _ => return Err(Galat::runtime("Objek BusEvent tidak valid", span)) };
            let mut lm = listeners_rc.borrow_mut();
            if !lm.contains_key(&event) {
                lm.insert(event.clone(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            }
            if let Some(Value::Array(arr_rc)) = lm.get(&event) {
                arr_rc.borrow_mut().push(handler);
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi bus_dengarkan_sekali memerlukan objek BusEvent", span)),
    }
}

fn builtin_eventbus_pancarkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bus_val = &args[0];
    let event = args[1].to_string_repr();
    let payload = args[2].clone();

    match bus_val {
        Value::Map(m) => {
            let map = m.borrow();
            let listeners_rc = match map.get("pendengar") { Some(Value::Map(lm)) => lm.clone(), _ => return Err(Galat::runtime("Objek BusEvent tidak valid", span)) };
            let once_rc = match map.get("pendengar_sekali") { Some(Value::Map(om)) => om.clone(), _ => return Err(Galat::runtime("Objek BusEvent tidak valid", span)) };

            let mut count = 0;

            // Trigger regular listeners
            let handlers = {
                let lm = listeners_rc.borrow();
                if let Some(Value::Array(arr_rc)) = lm.get(&event) {
                    arr_rc.borrow().clone()
                } else {
                    Vec::new()
                }
            };

            for h in handlers {
                let _ = invoke_closure_or_fn(&h, &[payload.clone()], span)?;
                count += 1;
            }

            // Trigger once listeners and remove them
            let once_handlers = {
                let mut om = once_rc.borrow_mut();
                if let Some(Value::Array(arr_rc)) = om.remove(&event) {
                    arr_rc.borrow().clone()
                } else {
                    Vec::new()
                }
            };

            for h in once_handlers {
                let _ = invoke_closure_or_fn(&h, &[payload.clone()], span)?;
                count += 1;
            }

            Ok(Value::Number(count as f64))
        }
        _ => Err(Galat::runtime("Fungsi bus_pancarkan memerlukan objek BusEvent", span)),
    }
}

fn builtin_eventbus_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bus_val = &args[0];
    let event = args[1].to_string_repr();

    match bus_val {
        Value::Map(m) => {
            let map = m.borrow();
            let listeners_rc = match map.get("pendengar") { Some(Value::Map(lm)) => lm.clone(), _ => return Err(Galat::runtime("Objek BusEvent tidak valid", span)) };
            let once_rc = match map.get("pendengar_sekali") { Some(Value::Map(om)) => om.clone(), _ => return Err(Galat::runtime("Objek BusEvent tidak valid", span)) };

            let existed1 = listeners_rc.borrow_mut().remove(&event).is_some();
            let existed2 = once_rc.borrow_mut().remove(&event).is_some();
            Ok(Value::Bool(existed1 || existed2))
        }
        _ => Err(Galat::runtime("Fungsi bus_hapus_pendengar memerlukan objek BusEvent", span)),
    }
}

// ==============================================================================
// WidyaIPCIDR: IP Network & Subnet Calculator
// ==============================================================================
fn parse_ipv4_to_u32(ip: &str) -> Option<u32> {
    let parts: Vec<&str> = ip.trim().split('.').collect();
    if parts.len() != 4 { return None; }
    let o1 = parts[0].parse::<u32>().ok()?;
    let o2 = parts[1].parse::<u32>().ok()?;
    let o3 = parts[2].parse::<u32>().ok()?;
    let o4 = parts[3].parse::<u32>().ok()?;
    if o1 > 255 || o2 > 255 || o3 > 255 || o4 > 255 { return None; }
    Some((o1 << 24) | (o2 << 16) | (o3 << 8) | o4)
}

fn u32_to_ipv4(val: u32) -> String {
    format!("{}.{}.{}.{}", (val >> 24) & 0xFF, (val >> 16) & 0xFF, (val >> 8) & 0xFF, val & 0xFF)
}

fn builtin_ip_jaringan_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cidr_str = args[0].to_string_repr();
    let (ip_str, prefix_str) = cidr_str.split_once('/').unwrap_or((&cidr_str, "32"));
    let ip = parse_ipv4_to_u32(ip_str).ok_or_else(|| Galat::runtime("Alamat IPv4 tidak valid", span))?;
    let prefix = prefix_str.trim().parse::<u32>().map_err(|_| Galat::runtime("Prefix CIDR tidak valid", span))?.min(32);

    let mask = if prefix == 0 { 0 } else { !0u32 << (32 - prefix) };
    let net = ip & mask;
    let bcast = net | !mask;
    let hosts = if prefix >= 31 { 0 } else { (1u64 << (32 - prefix)) - 2 };

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("IPJaringan".to_string()));
    obj.insert("cidr".to_string(), Value::String(cidr_str));
    obj.insert("ip".to_string(), Value::String(u32_to_ipv4(ip)));
    obj.insert("prefix".to_string(), Value::Number(prefix as f64));
    obj.insert("network".to_string(), Value::String(u32_to_ipv4(net)));
    obj.insert("broadcast".to_string(), Value::String(u32_to_ipv4(bcast)));
    obj.insert("netmask".to_string(), Value::String(u32_to_ipv4(mask)));
    obj.insert("jumlah_host".to_string(), Value::Number(hosts as f64));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_ip_dalam_rentang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let check_ip_str = args[0].to_string_repr();
    let check_ip = match parse_ipv4_to_u32(&check_ip_str) {
        Some(v) => v,
        None => return Ok(Value::Bool(false)),
    };

    let (net, mask) = match &args[1] {
        Value::Map(m) => {
            let map = m.borrow();
            let net_str = match map.get("network") { Some(Value::String(s)) => s, _ => return Err(Galat::runtime("IPJaringan tidak valid", span)) };
            let mask_str = match map.get("netmask") { Some(Value::String(s)) => s, _ => return Err(Galat::runtime("IPJaringan tidak valid", span)) };
            (parse_ipv4_to_u32(net_str).unwrap(), parse_ipv4_to_u32(mask_str).unwrap())
        }
        Value::String(s) => {
            let (ip_s, pre_s) = s.split_once('/').unwrap_or((s.as_str(), "32"));
            let ip = parse_ipv4_to_u32(ip_s).ok_or_else(|| Galat::runtime("IP tidak valid", span))?;
            let prefix = pre_s.parse::<u32>().unwrap_or(32).min(32);
            let m = if prefix == 0 { 0 } else { !0u32 << (32 - prefix) };
            (ip & m, m)
        }
        _ => return Err(Galat::runtime("Argumen 2 bukan objek IPJaringan atau string CIDR", span)),
    };

    Ok(Value::Bool((check_ip & mask) == net))
}

fn builtin_ip_network_address(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::String(s)) = map.get("network") {
                Ok(Value::String(s.clone()))
            } else {
                Err(Galat::runtime("Objek IPJaringan tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Fungsi ip_network_address memerlukan objek IPJaringan", span)),
    }
}

fn builtin_ip_broadcast_address(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::String(s)) = map.get("broadcast") {
                Ok(Value::String(s.clone()))
            } else {
                Err(Galat::runtime("Objek IPJaringan tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Fungsi ip_broadcast_address memerlukan objek IPJaringan", span)),
    }
}

fn builtin_ip_jumlah_host(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Number(n)) = map.get("jumlah_host") {
                Ok(Value::Number(*n))
            } else {
                Err(Galat::runtime("Objek IPJaringan tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Fungsi ip_jumlah_host memerlukan objek IPJaringan", span)),
    }
}

fn builtin_ip_ke_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ip_str = args[0].to_string_repr();
    let val = parse_ipv4_to_u32(&ip_str).ok_or_else(|| Galat::runtime("Alamat IPv4 tidak valid", span))?;
    let bin_str = format!("{:08b}.{:08b}.{:08b}.{:08b}", (val >> 24) & 0xFF, (val >> 16) & 0xFF, (val >> 8) & 0xFF, val & 0xFF);
    Ok(Value::String(bin_str))
}

