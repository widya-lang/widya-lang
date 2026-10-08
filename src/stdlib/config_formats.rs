use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ==========================================
// 📄 WidyaIniConfig (INI Parser & Serializer)
// ==========================================

fn parse_ini_value(v: &str) -> Value {
    let trimmed = v.trim();
    if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("benar") || trimmed.eq_ignore_ascii_case("ya") {
        return Value::Bool(true);
    }
    if trimmed.eq_ignore_ascii_case("false") || trimmed.eq_ignore_ascii_case("salah") || trimmed.eq_ignore_ascii_case("tidak") {
        return Value::Bool(false);
    }
    if let Ok(n) = trimmed.parse::<f64>() {
        return Value::Number(n);
    }
    // Remove surrounding quotes if present
    if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2) ||
       (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2) {
        return Value::String(trimmed[1..trimmed.len()-1].to_string());
    }
    Value::String(trimmed.to_string())
}

fn builtin_ini_parse(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("Argumen INI harus berupa string teks", span)),
    };

    let mut sections: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
    let mut current_section = "default".to_string();
    let mut current_map: std::collections::HashMap<String, Value> = std::collections::HashMap::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if !current_map.is_empty() || current_section != "default" {
                sections.insert(current_section.clone(), Value::Map(Rc::new(RefCell::new(current_map))));
                current_map = std::collections::HashMap::new();
            }
            current_section = trimmed[1..trimmed.len()-1].trim().to_string();
            continue;
        }

        if let Some(pos) = trimmed.find('=') {
            let key = trimmed[..pos].trim().to_string();
            let mut val_str = trimmed[pos+1..].trim();
            // check inline comment
            if let Some(c_pos) = val_str.find(';') {
                val_str = val_str[..c_pos].trim();
            }
            if let Some(c_pos) = val_str.find('#') {
                val_str = val_str[..c_pos].trim();
            }
            current_map.insert(key, parse_ini_value(val_str));
        } else if let Some(pos) = trimmed.find(':') {
            let key = trimmed[..pos].trim().to_string();
            let mut val_str = trimmed[pos+1..].trim();
            if let Some(c_pos) = val_str.find(';') {
                val_str = val_str[..c_pos].trim();
            }
            if let Some(c_pos) = val_str.find('#') {
                val_str = val_str[..c_pos].trim();
            }
            current_map.insert(key, parse_ini_value(val_str));
        }
    }
    sections.insert(current_section, Value::Map(Rc::new(RefCell::new(current_map))));

    Ok(Value::Map(Rc::new(RefCell::new(sections))))
}

fn builtin_ini_serialize(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let obj = match &args[0] {
        Value::Map(map) => map.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa objek map section INI", span)),
    };

    let mut out = String::new();

    // If default section exists, write it first
    if let Some(Value::Map(map)) = obj.get("default") {
        for (k, v) in map.borrow().iter() {
            out.push_str(&format!("{} = {}\n", k, v.to_string_repr()));
        }
        if !map.borrow().is_empty() {
            out.push('\n');
        }
    }

    for (sec, val) in &obj {
        if sec == "default" { continue; }
        out.push_str(&format!("[{}]\n", sec));
        if let Value::Map(m) = val {
            for (k, v) in m.borrow().iter() {
                out.push_str(&format!("{} = {}\n", k, v.to_string_repr()));
            }
        }
        out.push('\n');
    }

    Ok(Value::String(out.trim_end().to_string()))
}

fn builtin_ini_ambil_nilai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime("ini_ambil_nilai memerlukan minimal 3 argumen: objek, section, key, [default]", span));
    }
    let obj = match &args[0] {
        Value::Map(map) => map.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa objek map INI", span)),
    };
    let section = args[1].to_string_repr();
    let key = args[2].to_string_repr();
    let default_val = if args.len() > 3 { args[3].clone() } else { Value::Nil };

    if let Some(Value::Map(sec_map)) = obj.get(&section) {
        if let Some(v) = sec_map.borrow().get(&key) {
            return Ok(v.clone());
        }
    }
    Ok(default_val)
}

// ==========================================

// 📝 WidyaTOML (Native TOML Parser & Serializer)
// ==========================================

fn parse_toml_primitive(v: &str) -> Value {
    let trimmed = v.trim();
    if trimmed == "true" || trimmed == "benar" || trimmed == "ya" {
        return Value::Bool(true);
    }
    if trimmed == "false" || trimmed == "salah" || trimmed == "tidak" {
        return Value::Bool(false);
    }
    if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2) ||
       (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2) {
        return Value::String(trimmed[1..trimmed.len()-1].to_string());
    }
    if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed.len() >= 2 {
        let inner = &trimmed[1..trimmed.len()-1];
        let mut items = Vec::new();
        for elem in inner.split(',') {
            let el = elem.trim();
            if !el.is_empty() {
                items.push(parse_toml_primitive(el));
            }
        }
        return Value::Array(Rc::new(RefCell::new(items)));
    }
    if let Ok(n) = trimmed.parse::<f64>() {
        return Value::Number(n);
    }
    Value::String(trimmed.to_string())
}

fn builtin_toml_parse(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let toml_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("Argumen TOML harus berupa string teks", span)),
    };

    let root_map: Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut current_table_path: Vec<String> = Vec::new();

    for line in toml_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Table header: [table] or [table.sub]
        if trimmed.starts_with('[') && trimmed.ends_with(']') && !trimmed.starts_with("[[") {
            let header = &trimmed[1..trimmed.len()-1].trim();
            current_table_path = header.split('.').map(|s| s.trim().to_string()).collect();
            continue;
        }

        // Key = Value
        if let Some(pos) = trimmed.find('=') {
            let key = trimmed[..pos].trim().to_string();
            let val_str = trimmed[pos+1..].trim();
            let val = parse_toml_primitive(val_str);

            let mut target_map_rc = root_map.clone();
            for part in &current_table_path {
                let next_rc = {
                    let mut tm = target_map_rc.borrow_mut();
                    if let Some(Value::Map(existing)) = tm.get(part) {
                        existing.clone()
                    } else {
                        let new_m = Rc::new(RefCell::new(HashMap::new()));
                        tm.insert(part.clone(), Value::Map(new_m.clone()));
                        new_m
                    }
                };
                target_map_rc = next_rc;
            }

            target_map_rc.borrow_mut().insert(key, val);
        }
    }

    Ok(Value::Map(root_map))
}

fn serialize_toml_rec(val: &Value, prefix: &str, out: &mut String) {
    if let Value::Map(m) = val {
        let map = m.borrow();
        let mut primitives = Vec::new();
        let mut tables = Vec::new();

        for (k, v) in map.iter() {
            if k.starts_with('_') { continue; }
            if let Value::Map(_) = v {
                tables.push((k.clone(), v.clone()));
            } else {
                primitives.push((k.clone(), v.clone()));
            }
        }

        primitives.sort_by(|a, b| a.0.cmp(&b.0));
        tables.sort_by(|a, b| a.0.cmp(&b.0));

        for (k, v) in primitives {
            match &v {
                Value::String(s) => out.push_str(&format!("{} = \"{}\"\n", k, s)),
                Value::Bool(b) => out.push_str(&format!("{} = {}\n", k, b)),
                Value::Number(n) => out.push_str(&format!("{} = {}\n", k, n)),
                Value::Array(a) => {
                    let items: Vec<String> = a.borrow().iter().map(|item| item.to_string_repr()).collect();
                    out.push_str(&format!("{} = [{}]\n", k, items.join(", ")));
                }
                _ => out.push_str(&format!("{} = \"{}\"\n", k, v.to_string_repr())),
            }
        }

        for (k, v) in tables {
            let next_prefix = if prefix.is_empty() { k.clone() } else { format!("{}.{}", prefix, k) };
            out.push_str(&format!("\n[{}]\n", next_prefix));
            serialize_toml_rec(&v, &next_prefix, out);
        }
    }
}

fn builtin_toml_serialize(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let widya_val = &args[0];
    if let Value::Map(_) = widya_val {
        let mut out = String::new();
        serialize_toml_rec(widya_val, "", &mut out);
        Ok(Value::String(out.trim().to_string()))
    } else {
        Err(Galat::runtime("Argumen toml_serialize harus berupa objek map", span))
    }
}

fn builtin_toml_ambil_nilai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("toml_ambil_nilai memerlukan: objek, path_kunci, [default]", span));
    }
    let mut current = args[0].clone();
    let path = args[1].to_string_repr();
    let default_val = if args.len() > 2 { args[2].clone() } else { Value::Nil };

    for part in path.split('.') {
        let next = match &current {
            Value::Map(m) => m.borrow().get(part).cloned(),
            _ => None,
        };
        if let Some(v) = next {
            current = v;
        } else {
            return Ok(default_val);
        }
    }

    Ok(current)
}

// ==========================================

// WIDYAPARQUETLITE (COLUMNAR STORAGE & FAST ANALYTICS AGGREGATION)
// =========================================================================

fn builtin_kolumnar_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let schema_val = &args[0];
    let columns = match schema_val {
        Value::Array(a) => {
            let items = a.borrow();
            items.iter().map(|v| v.to_string_repr()).collect::<Vec<String>>()
        }
        _ => return Err(Galat::runtime("Fungsi 'TabelKolumnar' memerlukan daftar nama kolom", span)),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("TabelKolumnar".to_string()));
    
    let mut schema_map = HashMap::new();
    let mut data_map = HashMap::new();
    for col in columns {
        schema_map.insert(col.clone(), Value::String("any".to_string()));
        data_map.insert(col, Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }
    map.insert("skema".to_string(), Value::Map(Rc::new(RefCell::new(schema_map))));
    map.insert("kolom".to_string(), Value::Map(Rc::new(RefCell::new(data_map))));
    map.insert("jumlah_baris".to_string(), Value::Number(0.0));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kolumnar_tambah_baris(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let table_val = &args[0];
    let row_val = &args[1];

    match (table_val, row_val) {
        (Value::Map(m), Value::Map(row)) => {
            let mut tbl = m.borrow_mut();
            let row_data = row.borrow();
            let mut curr_rows = match tbl.get("jumlah_baris") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };

            if let Some(Value::Map(cols_map)) = tbl.get("kolom") {
                let cols = cols_map.borrow();
                for (col_name, col_vec_val) in cols.iter() {
                    if let Value::Array(arr) = col_vec_val {
                        let val = row_data.get(col_name).cloned().unwrap_or(Value::Nil);
                        arr.borrow_mut().push(val);
                    }
                }
            }
            curr_rows += 1;
            tbl.insert("jumlah_baris".to_string(), Value::Number(curr_rows as f64));
            Ok(Value::Number(curr_rows as f64))
        }
        _ => Err(Galat::runtime("Fungsi 'kolumnar_tambah_baris' memerlukan (TabelKolumnar, kamus_baris)", span)),
    }
}

fn builtin_kolumnar_kueri_agregasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let table_val = &args[0];
    let col_name = args[1].to_string_repr();
    let op = args[2].to_string_repr().to_uppercase();

    match table_val {
        Value::Map(m) => {
            let tbl = m.borrow();
            if let Some(Value::Map(cols_map)) = tbl.get("kolom") {
                if let Some(Value::Array(arr)) = cols_map.borrow().get(&col_name) {
                    let items = arr.borrow();
                    if items.is_empty() {
                        return Ok(Value::Nil);
                    }

                    match op.as_str() {
                        "SUM" | "JUMLAH" => {
                            let mut sum = 0.0;
                            for it in items.iter() {
                                if let Value::Number(n) = it {
                                    sum += *n;
                                }
                            }
                            return Ok(Value::Number(sum));
                        }
                        "AVG" | "RATA_RATA" | "RATA" => {
                            let mut sum = 0.0;
                            let mut count = 0.0;
                            for it in items.iter() {
                                if let Value::Number(n) = it {
                                    sum += *n;
                                    count += 1.0;
                                }
                            }
                            return if count > 0.0 {
                                Ok(Value::Number(sum / count))
                            } else {
                                Ok(Value::Number(0.0))
                            };
                        }
                        "MIN" | "MINIMUM" => {
                            let mut min_val = f64::INFINITY;
                            for it in items.iter() {
                                if let Value::Number(n) = it {
                                    if *n < min_val {
                                        min_val = *n;
                                    }
                                }
                            }
                            return if min_val.is_infinite() {
                                Ok(Value::Nil)
                            } else {
                                Ok(Value::Number(min_val))
                            };
                        }
                        "MAX" | "MAKSIMUM" | "MAKS" => {
                            let mut max_val = f64::NEG_INFINITY;
                            for it in items.iter() {
                                if let Value::Number(n) = it {
                                    if *n > max_val {
                                        max_val = *n;
                                    }
                                }
                            }
                            return if max_val.is_infinite() {
                                Ok(Value::Nil)
                            } else {
                                Ok(Value::Number(max_val))
                            };
                        }
                        "COUNT" | "HITUNG" => {
                            return Ok(Value::Number(items.len() as f64));
                        }
                        _ => return Err(Galat::runtime(format!("Operasi agregasi '{}' tidak dikenal (pilihan: SUM, AVG, MIN, MAX, COUNT)", op), span)),
                    }
                } else {
                    return Err(Galat::runtime(format!("Kolom '{}' tidak ditemukan pada TabelKolumnar", col_name), span));
                }
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi 'kolumnar_kueri_agregasi' memerlukan objek TabelKolumnar", span)),
    }
}

fn builtin_kolumnar_saring(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let table_val = &args[0];
    let col_name = args[1].to_string_repr();
    let op = args[2].to_string_repr();
    let target_val = &args[3];

    match table_val {
        Value::Map(m) => {
            let tbl = m.borrow();
            let total_rows = match tbl.get("jumlah_baris") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };

            if let Some(Value::Map(cols_map)) = tbl.get("kolom") {
                let cols = cols_map.borrow();
                let target_col = match cols.get(&col_name) {
                    Some(Value::Array(a)) => a.borrow(),
                    _ => return Err(Galat::runtime(format!("Kolom '{}' tidak ditemukan", col_name), span)),
                };

                let mut matched_indices = Vec::new();
                for i in 0..total_rows.min(target_col.len()) {
                    let cell = &target_col[i];
                    let matches = match op.as_str() {
                        "==" | "=" => cell == target_val,
                        "!=" => cell != target_val,
                        ">" => match (cell, target_val) {
                            (Value::Number(a), Value::Number(b)) => a > b,
                            _ => false,
                        },
                        ">=" => match (cell, target_val) {
                            (Value::Number(a), Value::Number(b)) => a >= b,
                            _ => false,
                        },
                        "<" => match (cell, target_val) {
                            (Value::Number(a), Value::Number(b)) => a < b,
                            _ => false,
                        },
                        "<=" => match (cell, target_val) {
                            (Value::Number(a), Value::Number(b)) => a <= b,
                            _ => false,
                        },
                        _ => false,
                    };
                    if matches {
                        matched_indices.push(i);
                    }
                }

                let mut rows_result = Vec::new();
                for &idx in &matched_indices {
                    let mut row = HashMap::new();
                    for (c_name, c_arr_val) in cols.iter() {
                        if let Value::Array(arr) = c_arr_val {
                            let arr_ref = arr.borrow();
                            if idx < arr_ref.len() {
                                row.insert(c_name.clone(), arr_ref[idx].clone());
                            }
                        }
                    }
                    rows_result.push(Value::Map(Rc::new(RefCell::new(row))));
                }

                return Ok(Value::Array(Rc::new(RefCell::new(rows_result))));
            }
            Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
        }
        _ => Err(Galat::runtime("Fungsi 'kolumnar_saring' memerlukan objek TabelKolumnar", span)),
    }
}

fn builtin_kolumnar_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let table_val = &args[0];
    match table_val {
        Value::Map(m) => {
            let tbl = m.borrow();
            let total_rows = match tbl.get("jumlah_baris") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };

            let mut res = Vec::new();
            if let Some(Value::Map(cols_map)) = tbl.get("kolom") {
                let cols = cols_map.borrow();
                for i in 0..total_rows {
                    let mut row = HashMap::new();
                    for (c_name, c_arr_val) in cols.iter() {
                        if let Value::Array(arr) = c_arr_val {
                            let arr_ref = arr.borrow();
                            if i < arr_ref.len() {
                                row.insert(c_name.clone(), arr_ref[i].clone());
                            }
                        }
                    }
                    res.push(Value::Map(Rc::new(RefCell::new(row))));
                }
            }
            Ok(Value::Array(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'kolumnar_ke_array' memerlukan objek TabelKolumnar", span)),
    }
}

// =========================================================================

pub fn register_config_formats(env: &mut Environment) {
    // WidyaIniConfig
    register_fn(env, "ini_parse", Some(1), builtin_ini_parse);
    register_fn(env, "ini_serialize", Some(1), builtin_ini_serialize);
    register_fn(env, "ini_ambil_nilai", None, builtin_ini_ambil_nilai);

    // WidyaTOML
    register_fn(env, "toml_parse", Some(1), builtin_toml_parse);
    register_fn(env, "toml_serialize", Some(1), builtin_toml_serialize);
    register_fn(env, "toml_ambil_nilai", None, builtin_toml_ambil_nilai);

    // WidyaParquetLite
    register_fn(env, "TabelKolumnar", Some(1), builtin_kolumnar_buat);
    register_fn(env, "kolumnar_tambah_baris", Some(2), builtin_kolumnar_tambah_baris);
    register_fn(env, "kolumnar_kueri_agregasi", Some(3), builtin_kolumnar_kueri_agregasi);
    register_fn(env, "kolumnar_saring", Some(4), builtin_kolumnar_saring);
    register_fn(env, "kolumnar_ke_array", Some(1), builtin_kolumnar_ke_array);
}
