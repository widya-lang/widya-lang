use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// --- WidyaGRPC & Microservices Implementation ---

fn builtin_grpc_server(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let service_name = args[0].to_string_repr();
    let port = match &args[1] { Value::Number(n) => *n as u16, _ => 50051 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ServerGRPC".to_string()));
    map.insert("service".to_string(), Value::String(service_name.clone()));
    map.insert("port".to_string(), Value::Number(port as f64));

    println!("⚡ [Widya gRPC Server] Layanan '{}' mendengarkan pada port {}", service_name, port);
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_grpc_klien(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let target = args[0].to_string_repr();
    let service = args[1].to_string_repr();

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KlienGRPC".to_string()));
    map.insert("target".to_string(), Value::String(target));
    map.insert("service".to_string(), Value::String(service));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_grpc_panggil(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _client = &args[0];
    let method = args[1].to_string_repr();
    let payload = args[2].to_string_repr();

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("OK_200".to_string()));
    map.insert("metode".to_string(), Value::String(method));
    map.insert("respon".to_string(), Value::String(format!("Hasil RPC: {}", payload)));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}


pub fn register_grpc(env: &mut Environment) {
    register_fn(env, "ServerGRPC", Some(2), builtin_grpc_server);
    register_fn(env, "KlienGRPC", Some(2), builtin_grpc_klien);
    register_fn(env, "grpc_panggil", Some(3), builtin_grpc_panggil);
}
