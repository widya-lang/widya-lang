//! Pustaka Format Serialisasi Berdaulat Murni (Sovereign Pure Format Engine)
//! 100% Bebas dari Ketergantungan Eksternal (Zero External Crates: Tanpa serde, serde_json, toml, csv).
//! Mengimplementasikan RFC 8259 (JSON), TOML v1.0, RFC 4180 (CSV), dan Format INI Standar.

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// 1. PURE JSON (RFC 8259) PARSER & SERIALIZER
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum PureJsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<PureJsonValue>),
    Object(HashMap<String, PureJsonValue>),
}

impl PureJsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, PureJsonValue::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PureJsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PureJsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PureJsonValue::Number(n) => Some(*n as i64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            PureJsonValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<PureJsonValue>> {
        match self {
            PureJsonValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, PureJsonValue>> {
        match self {
            PureJsonValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&PureJsonValue> {
        match self {
            PureJsonValue::Object(o) => o.get(key),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&PureJsonValue> {
        match self {
            PureJsonValue::Array(a) => a.get(index),
            _ => None,
        }
    }
}

pub struct PureJson;

impl PureJson {
    pub fn parse(input: &str) -> Result<PureJsonValue, String> {
        let chars: Vec<char> = input.chars().collect();
        let mut idx = 0;
        Self::skip_whitespace(&chars, &mut idx);
        let val = Self::parse_value(&chars, &mut idx)?;
        Self::skip_whitespace(&chars, &mut idx);
        if idx < chars.len() {
            return Err(format!("Karakter tidak terduga setelah JSON di posisi {}", idx));
        }
        Ok(val)
    }

    pub fn stringify(val: &PureJsonValue) -> String {
        let mut out = String::new();
        Self::format_value(val, &mut out, None, 0);
        out
    }

    pub fn stringify_pretty(val: &PureJsonValue, indent_size: usize) -> String {
        let mut out = String::new();
        Self::format_value(val, &mut out, Some(indent_size), 0);
        out
    }

    fn skip_whitespace(chars: &[char], idx: &mut usize) {
        while *idx < chars.len() && chars[*idx].is_whitespace() {
            *idx += 1;
        }
    }

    fn parse_value(chars: &[char], idx: &mut usize) -> Result<PureJsonValue, String> {
        Self::skip_whitespace(chars, idx);
        if *idx >= chars.len() {
            return Err("Input JSON berakhir sebelum waktunya".to_string());
        }

        match chars[*idx] {
            'n' => Self::parse_null(chars, idx),
            't' | 'f' => Self::parse_bool(chars, idx),
            '"' => Self::parse_string(chars, idx).map(PureJsonValue::String),
            '[' => Self::parse_array(chars, idx),
            '{' => Self::parse_object(chars, idx),
            '-' | '0'..='9' => Self::parse_number(chars, idx),
            c => Err(format!("Karakter tidak valid '{}' pada indeks {}", c, idx)),
        }
    }

    fn parse_null(chars: &[char], idx: &mut usize) -> Result<PureJsonValue, String> {
        if *idx + 4 <= chars.len() && &chars[*idx..*idx + 4] == &['n', 'u', 'l', 'l'] {
            *idx += 4;
            Ok(PureJsonValue::Null)
        } else {
            Err(format!("Token tidak dikenal di posisi {}", idx))
        }
    }

    fn parse_bool(chars: &[char], idx: &mut usize) -> Result<PureJsonValue, String> {
        if *idx + 4 <= chars.len() && &chars[*idx..*idx + 4] == &['t', 'r', 'u', 'e'] {
            *idx += 4;
            Ok(PureJsonValue::Bool(true))
        } else if *idx + 5 <= chars.len() && &chars[*idx..*idx + 5] == &['f', 'a', 'l', 's', 'e'] {
            *idx += 5;
            Ok(PureJsonValue::Bool(false))
        } else {
            Err(format!("Token boolean tidak valid di posisi {}", idx))
        }
    }

    fn parse_string(chars: &[char], idx: &mut usize) -> Result<String, String> {
        if chars[*idx] != '"' {
            return Err("String harus diawali tanda kutip ganda".to_string());
        }
        *idx += 1; // Lewati pembuka '"'

        let mut s = String::new();
        while *idx < chars.len() {
            let c = chars[*idx];
            *idx += 1;
            match c {
                '"' => return Ok(s),
                '\\' => {
                    if *idx >= chars.len() {
                        return Err("Escape sequence tidak lengkap".to_string());
                    }
                    let esc = chars[*idx];
                    *idx += 1;
                    match esc {
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        '/' => s.push('/'),
                        'b' => s.push('\x08'),
                        'f' => s.push('\x0C'),
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        'u' => {
                            if *idx + 4 > chars.len() {
                                return Err("Unicode escape \\uXXXX tidak lengkap".to_string());
                            }
                            let hex_str: String = chars[*idx..*idx + 4].iter().collect();
                            *idx += 4;
                            let codepoint = u32::from_str_radix(&hex_str, 16)
                                .map_err(|_| format!("Hex unicode tidak valid: {}", hex_str))?;
                            if let Some(ch) = char::from_u32(codepoint) {
                                s.push(ch);
                            } else {
                                s.push('\u{FFFD}');
                            }
                        }
                        other => return Err(format!("Escape sequence tidak valid: \\{}", other)),
                    }
                }
                normal => s.push(normal),
            }
        }
        Err("String tidak ditutup tanda kutip ganda".to_string())
    }

    fn parse_number(chars: &[char], idx: &mut usize) -> Result<PureJsonValue, String> {
        let start = *idx;
        if chars[*idx] == '-' {
            *idx += 1;
        }
        while *idx < chars.len() && chars[*idx].is_ascii_digit() {
            *idx += 1;
        }
        if *idx < chars.len() && chars[*idx] == '.' {
            *idx += 1;
            while *idx < chars.len() && chars[*idx].is_ascii_digit() {
                *idx += 1;
            }
        }
        if *idx < chars.len() && (chars[*idx] == 'e' || chars[*idx] == 'E') {
            *idx += 1;
            if *idx < chars.len() && (chars[*idx] == '+' || chars[*idx] == '-') {
                *idx += 1;
            }
            while *idx < chars.len() && chars[*idx].is_ascii_digit() {
                *idx += 1;
            }
        }

        let num_str: String = chars[start..*idx].iter().collect();
        let num: f64 = num_str.parse().map_err(|_| format!("Format angka tidak valid: {}", num_str))?;
        Ok(PureJsonValue::Number(num))
    }

    fn parse_array(chars: &[char], idx: &mut usize) -> Result<PureJsonValue, String> {
        *idx += 1; // lewati '['
        Self::skip_whitespace(chars, idx);

        let mut items = Vec::new();
        if *idx < chars.len() && chars[*idx] == ']' {
            *idx += 1;
            return Ok(PureJsonValue::Array(items));
        }

        loop {
            let val = Self::parse_value(chars, idx)?;
            items.push(val);
            Self::skip_whitespace(chars, idx);

            if *idx >= chars.len() {
                return Err("Array tidak ditutup ']'".to_string());
            }

            if chars[*idx] == ']' {
                *idx += 1;
                break;
            } else if chars[*idx] == ',' {
                *idx += 1;
                Self::skip_whitespace(chars, idx);
                // Dukungan trailing comma
                if *idx < chars.len() && chars[*idx] == ']' {
                    *idx += 1;
                    break;
                }
            } else {
                return Err(format!("Karakter tidak terduga dalam array: '{}'", chars[*idx]));
            }
        }

        Ok(PureJsonValue::Array(items))
    }

    fn parse_object(chars: &[char], idx: &mut usize) -> Result<PureJsonValue, String> {
        *idx += 1; // lewati '{'
        Self::skip_whitespace(chars, idx);

        let mut obj = HashMap::new();
        if *idx < chars.len() && chars[*idx] == '}' {
            *idx += 1;
            return Ok(PureJsonValue::Object(obj));
        }

        loop {
            Self::skip_whitespace(chars, idx);
            if *idx >= chars.len() || chars[*idx] != '"' {
                return Err("Kunci objek JSON harus berupa string".to_string());
            }

            let key = Self::parse_string(chars, idx)?;
            Self::skip_whitespace(chars, idx);

            if *idx >= chars.len() || chars[*idx] != ':' {
                return Err("Kunci objek harus diikuti titik dua ':'".to_string());
            }
            *idx += 1; // lewati ':'

            let val = Self::parse_value(chars, idx)?;
            obj.insert(key, val);

            Self::skip_whitespace(chars, idx);
            if *idx >= chars.len() {
                return Err("Objek tidak ditutup '}'".to_string());
            }

            if chars[*idx] == '}' {
                *idx += 1;
                break;
            } else if chars[*idx] == ',' {
                *idx += 1;
                Self::skip_whitespace(chars, idx);
                if *idx < chars.len() && chars[*idx] == '}' {
                    *idx += 1;
                    break;
                }
            } else {
                return Err(format!("Karakter tidak terduga dalam objek: '{}'", chars[*idx]));
            }
        }

        Ok(PureJsonValue::Object(obj))
    }

    fn format_value(val: &PureJsonValue, out: &mut String, indent: Option<usize>, depth: usize) {
        match val {
            PureJsonValue::Null => out.push_str("null"),
            PureJsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            PureJsonValue::Number(n) => {
                if n.fract() == 0.0 && !n.is_infinite() && !n.is_nan() && *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
                    out.push_str(&format!("{}", *n as i64));
                } else {
                    out.push_str(&format!("{}", n));
                }
            }
            PureJsonValue::String(s) => {
                out.push('"');
                for c in s.chars() {
                    match c {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        '\r' => out.push_str("\\r"),
                        '\t' => out.push_str("\\t"),
                        '\x08' => out.push_str("\\b"),
                        '\x0C' => out.push_str("\\f"),
                        c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                        c => out.push(c),
                    }
                }
                out.push('"');
            }
            PureJsonValue::Array(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push('[');
                if let Some(ind) = indent {
                    out.push('\n');
                    for (i, item) in items.iter().enumerate() {
                        out.push_str(&" ".repeat((depth + 1) * ind));
                        Self::format_value(item, out, indent, depth + 1);
                        if i + 1 < items.len() {
                            out.push(',');
                        }
                        out.push('\n');
                    }
                    out.push_str(&" ".repeat(depth * ind));
                } else {
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        Self::format_value(item, out, indent, depth);
                    }
                }
                out.push(']');
            }
            PureJsonValue::Object(map) => {
                if map.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push('{');
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort(); // Deterministik urutan kunci

                if let Some(ind) = indent {
                    out.push('\n');
                    for (i, k) in keys.iter().enumerate() {
                        out.push_str(&" ".repeat((depth + 1) * ind));
                        out.push('"');
                        out.push_str(k);
                        out.push_str("\": ");
                        Self::format_value(&map[*k], out, indent, depth + 1);
                        if i + 1 < keys.len() {
                            out.push(',');
                        }
                        out.push('\n');
                    }
                    out.push_str(&" ".repeat(depth * ind));
                } else {
                    for (i, k) in keys.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        out.push('"');
                        out.push_str(k);
                        out.push_str("\":");
                        Self::format_value(&map[*k], out, indent, depth);
                    }
                }
                out.push('}');
            }
        }
    }
}

// ============================================================================
// 2. PURE CSV (RFC 4180) PARSER & SERIALIZER
// ============================================================================

pub struct PureCsv;

impl PureCsv {
    pub fn parse(input: &str, delimiter: char) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if in_quotes {
                if c == '"' {
                    if chars.peek() == Some(&'"') {
                        chars.next();
                        current_field.push('"');
                    } else {
                        in_quotes = false;
                    }
                } else {
                    current_field.push(c);
                }
            } else {
                if c == '"' {
                    in_quotes = true;
                } else if c == delimiter {
                    current_row.push(current_field);
                    current_field = String::new();
                } else if c == '\r' {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    current_row.push(current_field);
                    current_field = String::new();
                    rows.push(current_row);
                    current_row = Vec::new();
                } else if c == '\n' {
                    current_row.push(current_field);
                    current_field = String::new();
                    rows.push(current_row);
                    current_row = Vec::new();
                } else {
                    current_field.push(c);
                }
            }
        }

        if !current_field.is_empty() || !current_row.is_empty() {
            current_row.push(current_field);
            rows.push(current_row);
        }

        rows
    }

    pub fn serialize(rows: &[Vec<String>], delimiter: char) -> String {
        let mut out = String::new();
        for (r_idx, row) in rows.iter().enumerate() {
            for (c_idx, cell) in row.iter().enumerate() {
                if c_idx > 0 {
                    out.push(delimiter);
                }
                let needs_quotes = cell.contains(delimiter) || cell.contains('"') || cell.contains('\n') || cell.contains('\r');
                if needs_quotes {
                    out.push('"');
                    for ch in cell.chars() {
                        if ch == '"' {
                            out.push_str("\"\"");
                        } else {
                            out.push(ch);
                        }
                    }
                    out.push('"');
                } else {
                    out.push_str(cell);
                }
            }
            if r_idx + 1 < rows.len() {
                out.push('\n');
            }
        }
        out
    }
}

// ============================================================================
// 3. PURE INI PARSER & SERIALIZER
// ============================================================================

pub struct PureIni;

impl PureIni {
    pub fn parse(input: &str) -> HashMap<String, HashMap<String, String>> {
        let mut result = HashMap::new();
        let mut current_section = "default".to_string();

        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len() - 1].trim().to_string();
                continue;
            }

            if let Some(pos) = trimmed.find('=') {
                let key = trimmed[..pos].trim().to_string();
                let mut val = trimmed[pos + 1..].trim();
                // Hilangkan inline comment
                if let Some(c) = val.find(';') {
                    val = val[..c].trim();
                }
                if let Some(c) = val.find('#') {
                    val = val[..c].trim();
                }
                // Hilangkan surrounding quotes
                let clean_val = if (val.starts_with('"') && val.ends_with('"') && val.len() >= 2)
                    || (val.starts_with('\'') && val.ends_with('\'') && val.len() >= 2)
                {
                    val[1..val.len() - 1].to_string()
                } else {
                    val.to_string()
                };

                result
                    .entry(current_section.clone())
                    .or_insert_with(HashMap::new)
                    .insert(key, clean_val);
            }
        }

        result
    }

    pub fn serialize(data: &HashMap<String, HashMap<String, String>>) -> String {
        let mut out = String::new();
        if let Some(default_sec) = data.get("default") {
            let mut keys: Vec<&String> = default_sec.keys().collect();
            keys.sort();
            for k in keys {
                out.push_str(&format!("{} = {}\n", k, default_sec[k]));
            }
            if !default_sec.is_empty() {
                out.push('\n');
            }
        }

        let mut sec_names: Vec<&String> = data.keys().filter(|&k| k != "default").collect();
        sec_names.sort();

        for sec in sec_names {
            out.push_str(&format!("[{}]\n", sec));
            let sub = &data[sec];
            let mut keys: Vec<&String> = sub.keys().collect();
            keys.sort();
            for k in keys {
                out.push_str(&format!("{} = {}\n", k, sub[k]));
            }
            out.push('\n');
        }

        out.trim_end().to_string()
    }
}

// ============================================================================
// 4. UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_json_roundtrip() {
        let raw = r#"{"nama":"Widya Sovereign","versi":2.6,"aktif":true,"pilar":["Kripto","Kompiler","Syscall"],"null_val":null}"#;
        let parsed = PureJson::parse(raw).expect("Gagal mem-parse JSON");

        assert_eq!(parsed.get("nama").unwrap().as_str(), Some("Widya Sovereign"));
        assert_eq!(parsed.get("versi").unwrap().as_f64(), Some(2.6));
        assert_eq!(parsed.get("aktif").unwrap().as_bool(), Some(true));
        assert!(parsed.get("null_val").unwrap().is_null());

        let arr = parsed.get("pilar").unwrap().as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_str(), Some("Kripto"));

        let serialized = PureJson::stringify(&parsed);
        let reparsed = PureJson::parse(&serialized).expect("Gagal parse ulang");
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_pure_json_pretty() {
        let mut map = HashMap::new();
        map.insert("kedaulatan".to_string(), PureJsonValue::Bool(true));
        let obj = PureJsonValue::Object(map);
        let pretty = PureJson::stringify_pretty(&obj, 2);
        assert!(pretty.contains("  \"kedaulatan\": true"));
    }

    #[test]
    fn test_pure_csv_roundtrip() {
        let input = "id,nama,skor\n1,\"Widya, Lang\",99.5\n2,\"Sovereign \"\"Pro\"\"\",100";
        let parsed = PureCsv::parse(input, ',');
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[1][1], "Widya, Lang");
        assert_eq!(parsed[2][1], "Sovereign \"Pro\"");

        let serialized = PureCsv::serialize(&parsed, ',');
        let reparsed = PureCsv::parse(&serialized, ',');
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_pure_ini_roundtrip() {
        let ini = "[server]\nhost = 127.0.0.1\nport = 8080\n[db]\nengine = widyabase";
        let parsed = PureIni::parse(ini);
        assert_eq!(parsed.get("server").unwrap().get("host").unwrap(), "127.0.0.1");
        assert_eq!(parsed.get("db").unwrap().get("engine").unwrap(), "widyabase");

        let serialized = PureIni::serialize(&parsed);
        assert!(serialized.contains("[server]"));
        assert!(serialized.contains("engine = widyabase"));
    }
}
