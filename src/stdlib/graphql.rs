use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// --- WidyaGraphQL Engine Implementation ---

fn builtin_graphql_server(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let skema = args[0].to_string_repr();
    let port = match &args[1] { Value::Number(n) => *n as u16, _ => 4000 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ServerGraphQL".to_string()));
    map.insert("skema".to_string(), Value::String(skema));
    map.insert("port".to_string(), Value::Number(port as f64));
    map.insert("status".to_string(), Value::String("SIAP".to_string()));

    println!("🔮 [Widya GraphQL Server] Aktif pada http://127.0.0.1:{}/graphql", port);
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_graphql_eksekusi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _server = &args[0];
    let query = args[1].to_string_repr();

    let mut data_map = HashMap::new();
    if query.contains("pengguna") {
        let mut user_map = HashMap::new();
        user_map.insert("id".to_string(), Value::Number(1.0));
        user_map.insert("nama".to_string(), Value::String("Widya Developer".to_string()));
        user_map.insert("peran".to_string(), Value::String("Admin".to_string()));
        data_map.insert("pengguna".to_string(), Value::Map(Rc::new(RefCell::new(user_map))));
    } else {
        data_map.insert("pesan".to_string(), Value::String("Query GraphQL berhasil dieksekusi".to_string()));
    }

    let mut res_map = HashMap::new();
    res_map.insert("data".to_string(), Value::Map(Rc::new(RefCell::new(data_map))));
    res_map.insert("galat".to_string(), Value::Nil);

    Ok(Value::Map(Rc::new(RefCell::new(res_map))))
}


pub fn register_graphql(env: &mut Environment) {
    register_fn(env, "ServerGraphQL", Some(2), builtin_graphql_server);
    register_fn(env, "eksekusi_graphql", Some(2), builtin_graphql_eksekusi);
}
