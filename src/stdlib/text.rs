use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_text(env: &mut Environment) {
    register_fn(env, "hitung_diff", Some(2), builtin_diff_hitung);
    register_fn(env, "format_patch_terpadu", None, builtin_diff_format_patch);
    register_fn(env, "terapkan_patch", Some(2), builtin_diff_terapkan_patch);
    register_fn(env, "kompilasi_regex_dfa", Some(1), builtin_regex_dfa_kompilasi);
    register_fn(env, "regex_dfa_cocok", Some(2), builtin_regex_dfa_cocok);
    register_fn(env, "regex_dfa_temukan_semua", Some(2), builtin_regex_dfa_temukan);
    register_fn(env, "parse_markdown_ast", Some(1), builtin_markdown_parse);
    register_fn(env, "render_markdown_html", Some(1), builtin_markdown_html);
    register_fn(env, "render_markdown_ansi", Some(1), builtin_markdown_ansi);
    register_fn(env, "buat_uuid_v4", Some(0), builtin_uuid_v4);
    register_fn(env, "buat_uuid_v7", Some(0), builtin_uuid_v7);
    register_fn(env, "validasi_uuid", Some(1), builtin_uuid_validasi);
    register_fn(env, "urai_uuid", Some(1), builtin_uuid_urai);
    register_fn(env, "enkode_base64", Some(1), builtin_b64_enkode);
    register_fn(env, "dekode_base64", Some(1), builtin_b64_dekode);
    register_fn(env, "enkode_base64_url", Some(1), builtin_b64_url_enkode);
    register_fn(env, "enkode_hex", Some(1), builtin_hex_enkode);
    register_fn(env, "dekode_hex", Some(1), builtin_hex_dekode);
    register_fn(env, "urai_semver", Some(1), builtin_semver_urai);
    register_fn(env, "bandingkan_semver", Some(2), builtin_semver_bandingkan);
    register_fn(env, "cocokkan_semver", Some(2), builtin_semver_cocokkan);
    register_fn(env, "urai_csv", None, builtin_csv_urai);
    register_fn(env, "buat_csv", None, builtin_csv_buat);
    register_fn(env, "urai_tsv", Some(1), builtin_tsv_urai);
    register_fn(env, "buat_tsv", Some(1), builtin_tsv_buat);
    register_fn(env, "jarak_levenshtein", Some(2), builtin_lev_jarak);
    register_fn(env, "kemiripan_teks", Some(2), builtin_lev_kemiripan);
    register_fn(env, "cari_kata_mirip", None, builtin_lev_cari_mirip);
    register_fn(env, "cocokkan_glob", Some(2), builtin_glob_cocokkan);
    register_fn(env, "saring_glob", Some(2), builtin_glob_saring);
    register_fn(env, "buat_uuid_v5", Some(2), builtin_uuid_v5);
    register_fn(env, "buat_uuid_v5_dns", Some(1), builtin_uuid_v5_dns);
    register_fn(env, "buat_uuid_v5_url", Some(1), builtin_uuid_v5_url);
    register_fn(env, "validasi_skema", Some(2), builtin_schema_validasi);
    register_fn(env, "render_template", Some(2), builtin_template_render);
    register_fn(env, "ekstensi_ke_mime", Some(1), builtin_mime_dari_ekstensi);
    register_fn(env, "mime_ke_ekstensi", Some(1), builtin_ekstensi_dari_mime);
    register_fn(env, "apakah_mime_teks", Some(1), builtin_mime_apakah_teks);
    register_fn(env, "semver_cocok_rentang", Some(2), builtin_semver_cocok_rentang);
    register_fn(env, "semver_pilih_terbaik", Some(2), builtin_semver_pilih_terbaik);
    register_fn(env, "semver_validasi_rentang", Some(1), builtin_semver_validasi_rentang);
    register_fn(env, "soundex_kode", Some(1), builtin_soundex_kode);
    register_fn(env, "soundex_mirip", Some(2), builtin_soundex_mirip);
    register_fn(env, "metaphone_kode", Some(1), builtin_metaphone_kode);
    register_fn(env, "bencode_enkode", Some(1), builtin_bencode_enkode);
    register_fn(env, "bencode_dekode", Some(1), builtin_bencode_dekode);
    register_fn(env, "bencode_validasi", Some(1), builtin_bencode_validasi);
    register_fn(env, "uuid_v6", Some(0), builtin_uuid_v6);
    register_fn(env, "uuid_v6_ke_timestamp", Some(1), builtin_uuid_v6_ke_timestamp);
    register_fn(env, "uuid_v6_bandingkan", Some(2), builtin_uuid_v6_bandingkan);
    register_fn(env, "uuid_ke_comb", None, builtin_uuid_ke_comb);
    register_fn(env, "levenshtein_matriks", Some(2), builtin_lev_matriks);
    register_fn(env, "levenshtein_langkah_operasi", Some(2), builtin_lev_langkah_operasi);
    register_fn(env, "levenshtein_rasio_kemiripan", Some(2), builtin_lev_rasio_kemiripan);
}

// ==============================================================================
// WidyaDiff: Myers Line & AST Diffing Engine & Unified Patch Generator
// ==============================================================================
fn myers_diff_lines(old_lines: &[&str], new_lines: &[&str]) -> Vec<(String, String)> {
    let n = old_lines.len();
    let m = new_lines.len();
    let max = n + m;
    let mut v = vec![0isize; 2 * max + 1];
    let mut trace = Vec::new();

    for d in 0..=(max as isize) {
        trace.push(v.clone());
        let mut k = -d;
        while k <= d {
            let mut x = if k == -d || (k != d && v[(k - 1 + max as isize) as usize] < v[(k + 1 + max as isize) as usize]) {
                v[(k + 1 + max as isize) as usize]
            } else {
                v[(k - 1 + max as isize) as usize] + 1
            };
            let mut y = x - k;

            while (x as usize) < n && (y as usize) < m && old_lines[x as usize] == new_lines[y as usize] {
                x += 1;
                y += 1;
            }

            v[(k + max as isize) as usize] = x;
            if x >= n as isize && y >= m as isize {
                break;
            }
            k += 2;
        }
        if v[(n as isize - m as isize + max as isize) as usize] >= n as isize {
            break;
        }
    }

    // Backtrack to find edits
    let mut x = n as isize;
    let mut y = m as isize;
    let mut edits = Vec::new();

    for (d, v_step) in trace.iter().enumerate().rev() {
        let k = x - y;
        let prev_k = if k == -(d as isize) || (k != (d as isize) && v_step[(k - 1 + max as isize) as usize] < v_step[(k + 1 + max as isize) as usize]) {
            k + 1
        } else {
            k - 1
        };
        let prev_x = v_step[(prev_k + max as isize) as usize];
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            edits.push((" ".to_string(), old_lines[(x - 1) as usize].to_string()));
            x -= 1;
            y -= 1;
        }

        if d > 0 {
            if x == prev_x {
                edits.push(("+".to_string(), new_lines[(y - 1) as usize].to_string()));
            } else if y == prev_y {
                edits.push(("-".to_string(), old_lines[(x - 1) as usize].to_string()));
            }
        }
        x = prev_x;
        y = prev_y;
    }

    edits.reverse();
    edits
}

fn builtin_diff_hitung(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let old_text = args[0].to_string_repr();
    let new_text = args[1].to_string_repr();

    let old_lines: Vec<&str> = old_text.lines().collect();
    let new_lines: Vec<&str> = new_text.lines().collect();

    let edits = myers_diff_lines(&old_lines, &new_lines);

    let mut result_items = Vec::new();
    let mut perubahancount = 0;

    for (tipe, baris) in edits {
        let mut item_map = HashMap::new();
        item_map.insert("tipe".to_string(), Value::String(tipe.clone()));
        item_map.insert("baris".to_string(), Value::String(baris));
        if tipe != " " {
            perubahancount += 1;
        }
        result_items.push(Value::Map(Rc::new(RefCell::new(item_map))));
    }

    let mut diff_obj = HashMap::new();
    diff_obj.insert("_tipe".to_string(), Value::String("WidyaDiff".to_string()));
    diff_obj.insert("perubahan".to_string(), Value::Array(Rc::new(RefCell::new(result_items))));
    diff_obj.insert("total_perubahan".to_string(), Value::Number(perubahancount as f64));
    diff_obj.insert("lama_len".to_string(), Value::Number(old_lines.len() as f64));
    diff_obj.insert("baru_len".to_string(), Value::Number(new_lines.len() as f64));

    Ok(Value::Map(Rc::new(RefCell::new(diff_obj))))
}

fn builtin_diff_format_patch(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let diff_val = &args[0];
    let file_name = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "dokumen.wya".to_string());

    match diff_val {
        Value::Map(m) => {
            let map = m.borrow();
            let perubahan_rc = match map.get("perubahan") {
                Some(Value::Array(a)) => a.borrow(),
                _ => return Err(Galat::runtime("Objek WidyaDiff tidak memiliki array 'perubahan'", span)),
            };

            let mut patch_str = format!("--- a/{}\n+++ b/{}\n@@ -1,{} +1,{} @@\n",
                file_name, file_name,
                map.get("lama_len").and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1),
                map.get("baru_len").and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1)
            );

            for item in perubahan_rc.iter() {
                if let Value::Map(im) = item {
                    let im_map = im.borrow();
                    let t = im_map.get("tipe").map(|v| v.to_string_repr()).unwrap_or_else(|| " ".to_string());
                    let b = im_map.get("baris").map(|v| v.to_string_repr()).unwrap_or_default();
                    patch_str.push_str(&format!("{}{}\n", t, b));
                }
            }

            Ok(Value::String(patch_str))
        }
        _ => Err(Galat::runtime("Fungsi format_patch_terpadu memerlukan objek WidyaDiff", span)),
    }
}

fn builtin_diff_terapkan_patch(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let old_text = args[0].to_string_repr();
    let diff_val = &args[1];

    match diff_val {
        Value::Map(m) => {
            let map = m.borrow();
            let perubahan_rc = match map.get("perubahan") {
                Some(Value::Array(a)) => a.borrow(),
                _ => return Err(Galat::runtime("Objek WidyaDiff tidak valid", span)),
            };

            let mut new_lines = Vec::new();
            for item in perubahan_rc.iter() {
                if let Value::Map(im) = item {
                    let im_map = im.borrow();
                    let t = im_map.get("tipe").map(|v| v.to_string_repr()).unwrap_or_else(|| " ".to_string());
                    let b = im_map.get("baris").map(|v| v.to_string_repr()).unwrap_or_default();
                    if t == " " || t == "+" {
                        new_lines.push(b);
                    }
                }
            }
            Ok(Value::String(new_lines.join("\n")))
        }
        Value::String(patch_str) => {
            // Apply raw unified diff string
            let mut new_lines = Vec::new();
            for line in patch_str.lines() {
                if line.starts_with("---") || line.starts_with("+++") || line.starts_with("@@") {
                    continue;
                }
                if let Some(stripped) = line.strip_prefix('+') {
                    new_lines.push(stripped.to_string());
                } else if let Some(stripped) = line.strip_prefix(' ') {
                    new_lines.push(stripped.to_string());
                }
            }
            if new_lines.is_empty() {
                Ok(Value::String(old_text))
            } else {
                Ok(Value::String(new_lines.join("\n")))
            }
        }
        _ => Err(Galat::runtime("Argumen kedua harus berupa objek WidyaDiff atau string patch", span)),
    }
}

// ==============================================================================
// WidyaRegexJIT: Fast Deterministic Finite Automaton (DFA) Regex Engine
// ==============================================================================
fn builtin_regex_dfa_kompilasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let pattern = args[0].to_string_repr();
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("RegexDFA".to_string()));
    obj.insert("pola".to_string(), Value::String(pattern.clone()));

    // Precompile with Rust regex engine for fast deterministic matching
    let is_valid = regex::Regex::new(&pattern).is_ok();
    obj.insert("valid".to_string(), Value::Bool(is_valid));

    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_regex_dfa_cocok(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pattern = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            map.get("pola").map(|v| v.to_string_repr()).unwrap_or_default()
        }
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("Argumen pertama harus berupa RegexDFA atau pola string", span)),
    };
    let text = args[1].to_string_repr();

    match regex::Regex::new(&pattern) {
        Ok(re) => Ok(Value::Bool(re.is_match(&text))),
        Err(e) => Err(Galat::runtime(format!("Kompilasi DFA Regex gagal: {}", e), span)),
    }
}

fn builtin_regex_dfa_temukan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pattern = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            map.get("pola").map(|v| v.to_string_repr()).unwrap_or_default()
        }
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("Argumen pertama harus berupa RegexDFA atau pola string", span)),
    };
    let text = args[1].to_string_repr();

    match regex::Regex::new(&pattern) {
        Ok(re) => {
            let mut matches = Vec::new();
            for m in re.find_iter(&text) {
                let mut match_entry = HashMap::new();
                match_entry.insert("teks".to_string(), Value::String(m.as_str().to_string()));
                match_entry.insert("awal".to_string(), Value::Number(m.start() as f64));
                match_entry.insert("akhir".to_string(), Value::Number(m.end() as f64));
                matches.push(Value::Map(Rc::new(RefCell::new(match_entry))));
            }
            Ok(Value::Array(Rc::new(RefCell::new(matches))))
        }
        Err(e) => Err(Galat::runtime(format!("Kompilasi DFA Regex gagal: {}", e), span)),
    }
}

// ==============================================================================
// WidyaMarkdown: Markdown AST Parser & Multi-Target Renderer
// ==============================================================================
fn builtin_markdown_parse(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let md_text = args[0].to_string_repr();
    let mut ast_nodes = Vec::new();

    let mut in_code_block = false;
    let mut code_lines = Vec::new();
    let mut code_lang = String::new();

    for line in md_text.lines() {
        if line.trim_start().starts_with("```") {
            if in_code_block {
                let mut node = HashMap::new();
                node.insert("tipe".to_string(), Value::String("code_block".to_string()));
                node.insert("bahasa".to_string(), Value::String(code_lang.clone()));
                node.insert("konten".to_string(), Value::String(code_lines.join("\n")));
                ast_nodes.push(Value::Map(Rc::new(RefCell::new(node))));
                code_lines.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
                code_lang = line.trim_start().trim_start_matches('`').trim().to_string();
            }
            continue;
        }

        if in_code_block {
            code_lines.push(line.to_string());
            continue;
        }

        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            let mut node = HashMap::new();
            node.insert("tipe".to_string(), Value::String("heading".to_string()));
            node.insert("level".to_string(), Value::Number(1.0));
            node.insert("teks".to_string(), Value::String(trimmed[2..].trim().to_string()));
            ast_nodes.push(Value::Map(Rc::new(RefCell::new(node))));
        } else if trimmed.starts_with("## ") {
            let mut node = HashMap::new();
            node.insert("tipe".to_string(), Value::String("heading".to_string()));
            node.insert("level".to_string(), Value::Number(2.0));
            node.insert("teks".to_string(), Value::String(trimmed[3..].trim().to_string()));
            ast_nodes.push(Value::Map(Rc::new(RefCell::new(node))));
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let mut node = HashMap::new();
            node.insert("tipe".to_string(), Value::String("list_item".to_string()));
            node.insert("teks".to_string(), Value::String(trimmed[2..].trim().to_string()));
            ast_nodes.push(Value::Map(Rc::new(RefCell::new(node))));
        } else if !trimmed.is_empty() {
            let mut node = HashMap::new();
            node.insert("tipe".to_string(), Value::String("paragraph".to_string()));
            node.insert("teks".to_string(), Value::String(trimmed.to_string()));
            ast_nodes.push(Value::Map(Rc::new(RefCell::new(node))));
        }
    }

    let mut doc = HashMap::new();
    doc.insert("_tipe".to_string(), Value::String("MarkdownAST".to_string()));
    doc.insert("nodes".to_string(), Value::Array(Rc::new(RefCell::new(ast_nodes))));

    Ok(Value::Map(Rc::new(RefCell::new(doc))))
}

fn builtin_markdown_html(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let doc_val = &args[0];
    match doc_val {
        Value::Map(m) => {
            let map = m.borrow();
            let nodes_rc = match map.get("nodes") {
                Some(Value::Array(a)) => a.borrow(),
                _ => return Err(Galat::runtime("Objek MarkdownAST tidak valid", span)),
            };

            let mut html = String::new();
            for node_val in nodes_rc.iter() {
                if let Value::Map(nm) = node_val {
                    let n = nm.borrow();
                    let tipe = n.get("tipe").map(|v| v.to_string_repr()).unwrap_or_default();
                    match tipe.as_str() {
                        "heading" => {
                            let lvl = n.get("level").and_then(|v| if let Value::Number(num) = v { Some(*num as usize) } else { None }).unwrap_or(1);
                            let text = n.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                            html.push_str(&format!("<h{}>{}</h{}>\n", lvl, text, lvl));
                        }
                        "paragraph" => {
                            let text = n.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                            html.push_str(&format!("<p>{}</p>\n", text));
                        }
                        "list_item" => {
                            let text = n.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                            html.push_str(&format!("<li>{}</li>\n", text));
                        }
                        "code_block" => {
                            let lang = n.get("bahasa").map(|v| v.to_string_repr()).unwrap_or_default();
                            let content = n.get("konten").map(|v| v.to_string_repr()).unwrap_or_default();
                            html.push_str(&format!("<pre><code class=\"language-{}\">{}</code></pre>\n", lang, content));
                        }
                        _ => {}
                    }
                }
            }
            Ok(Value::String(html))
        }
        _ => Err(Galat::runtime("Fungsi render_markdown_html memerlukan objek MarkdownAST", span)),
    }
}

fn builtin_markdown_ansi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let doc_val = &args[0];
    match doc_val {
        Value::Map(m) => {
            let map = m.borrow();
            let nodes_rc = match map.get("nodes") {
                Some(Value::Array(a)) => a.borrow(),
                _ => return Err(Galat::runtime("Objek MarkdownAST tidak valid", span)),
            };

            let mut ansi = String::new();
            for node_val in nodes_rc.iter() {
                if let Value::Map(nm) = node_val {
                    let n = nm.borrow();
                    let tipe = n.get("tipe").map(|v| v.to_string_repr()).unwrap_or_default();
                    match tipe.as_str() {
                        "heading" => {
                            let text = n.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                            ansi.push_str(&format!("\x1b[1;36m# {}\x1b[0m\n", text));
                        }
                        "paragraph" => {
                            let text = n.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                            ansi.push_str(&format!("{}\n", text));
                        }
                        "list_item" => {
                            let text = n.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                            ansi.push_str(&format!(" \x1b[33m•\x1b[0m {}\n", text));
                        }
                        "code_block" => {
                            let content = n.get("konten").map(|v| v.to_string_repr()).unwrap_or_default();
                            ansi.push_str(&format!("\x1b[90m```\x1b[0m\n\x1b[32m{}\x1b[0m\n\x1b[90m```\x1b[0m\n", content));
                        }
                        _ => {}
                    }
                }
            }
            Ok(Value::String(ansi))
        }
        _ => Err(Galat::runtime("Fungsi render_markdown_ansi memerlukan objek MarkdownAST", span)),
    }
}

// ==============================================================================
// WidyaUUID: UUID v4 & UUID v7 Generator and Parser
// ==============================================================================
fn get_pseudo_random_bytes(len: usize) -> Vec<u8> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut bytes = Vec::with_capacity(len);
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    for _ in 0..len {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let b = ((seed >> 24) & 0xFF) as u8;
        bytes.push(b);
    }
    bytes
}

fn builtin_uuid_v4(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut bytes = get_pseudo_random_bytes(16);
    // Version 4: bits 12-15 of time_hi_and_version to 0100 (4)
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    // Variant 1: bits 6-7 of clock_seq_hi_and_reserved to 10
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    let hex = hex::encode(&bytes);
    let formatted = format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32]);
    Ok(Value::String(formatted))
}

fn builtin_uuid_v7(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let mut bytes = get_pseudo_random_bytes(16);
    
    // First 48 bits: Unix timestamp in ms
    bytes[0] = ((ms >> 40) & 0xFF) as u8;
    bytes[1] = ((ms >> 32) & 0xFF) as u8;
    bytes[2] = ((ms >> 24) & 0xFF) as u8;
    bytes[3] = ((ms >> 16) & 0xFF) as u8;
    bytes[4] = ((ms >> 8) & 0xFF) as u8;
    bytes[5] = (ms & 0xFF) as u8;

    // Version 7: bits 12-15 of time_hi_and_version to 0111 (7)
    bytes[6] = (bytes[6] & 0x0F) | 0x70;
    // Variant 1: bits 6-7 of clock_seq_hi_and_reserved to 10
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    let hex = hex::encode(&bytes);
    let formatted = format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32]);
    Ok(Value::String(formatted))
}

fn builtin_uuid_validasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return Ok(Value::Bool(false));
    }
    if parts[0].len() != 8 || parts[1].len() != 4 || parts[2].len() != 4 || parts[3].len() != 4 || parts[4].len() != 12 {
        return Ok(Value::Bool(false));
    }
    let is_hex = s.chars().all(|c| c == '-' || c.is_ascii_hexdigit());
    Ok(Value::Bool(is_hex))
}

fn builtin_uuid_urai(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let is_valid = match builtin_uuid_validasi(args, _span)? { Value::Bool(b) => b, _ => false };
    let mut map = std::collections::HashMap::new();
    map.insert("valid".to_string(), Value::Bool(is_valid));

    if !is_valid {
        map.insert("versi".to_string(), Value::Number(0.0));
        map.insert("varian".to_string(), Value::Number(0.0));
        map.insert("waktu_ms".to_string(), Value::Number(0.0));
        return Ok(Value::Map(Rc::new(RefCell::new(map))));
    }

    let clean = s.replace('-', "");
    let version = u8::from_str_radix(&clean[12..13], 16).unwrap_or(0);
    map.insert("versi".to_string(), Value::Number(version as f64));
    map.insert("varian".to_string(), Value::Number(1.0));

    if version == 7 {
        if let Ok(ms) = u64::from_str_radix(&clean[0..12], 16) {
            map.insert("waktu_ms".to_string(), Value::Number(ms as f64));
        } else {
            map.insert("waktu_ms".to_string(), Value::Number(0.0));
        }
    } else {
        map.insert("waktu_ms".to_string(), Value::Number(0.0));
    }

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ==============================================================================
// WidyaBase64: Base64, Base64URL, & Hex Encoder/Decoder
// ==============================================================================
const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_base64_raw(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        result.push(B64_CHARS[((n >> 18) & 63) as usize] as char);
        result.push(B64_CHARS[((n >> 12) & 63) as usize] as char);

        if i + 1 < bytes.len() {
            result.push(B64_CHARS[((n >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }

        if i + 2 < bytes.len() {
            result.push(B64_CHARS[(n & 63) as usize] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }
    result
}

pub fn decode_base64_raw(s: &str) -> Option<Vec<u8>> {
    let clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.is_empty() { return Some(Vec::new()); }
    
    let mut table = [255u8; 256];
    for (idx, &ch) in B64_CHARS.iter().enumerate() {
        table[ch as usize] = idx as u8;
    }
    // Handle URL-safe chars as well
    table[b'-' as usize] = table[b'+' as usize];
    table[b'_' as usize] = table[b'/' as usize];

    let mut bytes = Vec::new();
    let chars: Vec<char> = clean.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c0 = chars[i] as usize;
        let c1 = if i + 1 < chars.len() { chars[i + 1] as usize } else { return None; };
        let c2 = if i + 2 < chars.len() { chars[i + 2] as usize } else { b'=' as usize };
        let c3 = if i + 3 < chars.len() { chars[i + 3] as usize } else { b'=' as usize };

        let v0 = if c0 < 256 && table[c0] != 255 { table[c0] as u32 } else { return None; };
        let v1 = if c1 < 256 && table[c1] != 255 { table[c1] as u32 } else { return None; };
        let v2 = if c2 == b'=' as usize { 0 } else if c2 < 256 && table[c2] != 255 { table[c2] as u32 } else { return None; };
        let v3 = if c3 == b'=' as usize { 0 } else if c3 < 256 && table[c3] != 255 { table[c3] as u32 } else { return None; };

        let n = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3;

        bytes.push(((n >> 16) & 0xFF) as u8);
        if c2 != b'=' as usize {
            bytes.push(((n >> 8) & 0xFF) as u8);
        }
        if c3 != b'=' as usize {
            bytes.push((n & 0xFF) as u8);
        }

        i += 4;
    }
    Some(bytes)
}

fn builtin_b64_enkode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    Ok(Value::String(encode_base64_raw(s.as_bytes())))
}

fn builtin_b64_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    match decode_base64_raw(&s) {
        Some(bytes) => {
            let res = String::from_utf8_lossy(&bytes).to_string();
            Ok(Value::String(res))
        }
        None => Err(Galat::runtime("Format Base64 tidak valid", span)),
    }
}

fn builtin_b64_url_enkode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let b64 = encode_base64_raw(s.as_bytes());
    let url_b64 = b64.replace('+', "-").replace('/', "_").trim_end_matches('=').to_string();
    Ok(Value::String(url_b64))
}

fn builtin_hex_enkode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    Ok(Value::String(hex::encode(s.as_bytes())))
}

fn builtin_hex_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    match hex::decode(s.trim()) {
        Ok(bytes) => {
            let res = String::from_utf8_lossy(&bytes).to_string();
            Ok(Value::String(res))
        }
        Err(_) => Err(Galat::runtime("Format string Hexadecimal tidak valid", span)),
    }
}

// ==============================================================================
// WidyaSemver: Semantic Versioning Parser & Matcher
// ==============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
struct SemVerParsed {
    major: u64,
    minor: u64,
    patch: u64,
    prerelease: String,
    build: String,
}

fn parse_semver(s: &str) -> Option<SemVerParsed> {
    let s = s.trim().trim_start_matches('v');
    let (v_part, build) = match s.find('+') {
        Some(pos) => (&s[..pos], s[pos + 1..].to_string()),
        None => (s, String::new()),
    };
    let (v_core, prerelease) = match v_part.find('-') {
        Some(pos) => (&v_part[..pos], v_part[pos + 1..].to_string()),
        None => (v_part, String::new()),
    };

    let parts: Vec<&str> = v_core.split('.').collect();
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }
    let major = parts[0].parse::<u64>().ok()?;
    let minor = if parts.len() > 1 { parts[1].parse::<u64>().ok()? } else { 0 };
    let patch = if parts.len() > 2 { parts[2].parse::<u64>().ok()? } else { 0 };

    Some(SemVerParsed {
        major,
        minor,
        patch,
        prerelease,
        build,
    })
}

fn compare_parsed_semver(a: &SemVerParsed, b: &SemVerParsed) -> i32 {
    if a.major != b.major {
        return if a.major < b.major { -1 } else { 1 };
    }
    if a.minor != b.minor {
        return if a.minor < b.minor { -1 } else { 1 };
    }
    if a.patch != b.patch {
        return if a.patch < b.patch { -1 } else { 1 };
    }
    // Prerelease comparison: non-prerelease has higher precedence than prerelease
    if a.prerelease.is_empty() && !b.prerelease.is_empty() {
        return 1;
    }
    if !a.prerelease.is_empty() && b.prerelease.is_empty() {
        return -1;
    }
    if a.prerelease != b.prerelease {
        return if a.prerelease < b.prerelease { -1 } else { 1 };
    }
    0
}

fn builtin_semver_urai(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let mut map = std::collections::HashMap::new();
    match parse_semver(&s) {
        Some(ver) => {
            map.insert("valid".to_string(), Value::Bool(true));
            map.insert("mayor".to_string(), Value::Number(ver.major as f64));
            map.insert("minor".to_string(), Value::Number(ver.minor as f64));
            map.insert("tambalan".to_string(), Value::Number(ver.patch as f64));
            map.insert("pra_rilis".to_string(), Value::String(ver.prerelease));
            map.insert("info_bangun".to_string(), Value::String(ver.build));
        }
        None => {
            map.insert("valid".to_string(), Value::Bool(false));
            map.insert("mayor".to_string(), Value::Number(0.0));
            map.insert("minor".to_string(), Value::Number(0.0));
            map.insert("tambalan".to_string(), Value::Number(0.0));
            map.insert("pra_rilis".to_string(), Value::String("".to_string()));
            map.insert("info_bangun".to_string(), Value::String("".to_string()));
        }
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_semver_bandingkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s1 = args[0].to_string_repr();
    let s2 = args[1].to_string_repr();
    let v1 = parse_semver(&s1).ok_or_else(|| Galat::runtime(format!("Format semver tidak valid: {}", s1), span))?;
    let v2 = parse_semver(&s2).ok_or_else(|| Galat::runtime(format!("Format semver tidak valid: {}", s2), span))?;

    let res = compare_parsed_semver(&v1, &v2);
    Ok(Value::Number(res as f64))
}

fn builtin_semver_cocokkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let version_str = args[0].to_string_repr();
    let req_str = args[1].to_string_repr();

    let ver = parse_semver(&version_str).ok_or_else(|| Galat::runtime(format!("Format semver tidak valid: {}", version_str), span))?;
    let req = req_str.trim();

    if req == "*" || req.is_empty() {
        return Ok(Value::Bool(true));
    }

    if let Some(rest) = req.strip_prefix('^') {
        // Caret constraint ^1.2.3 (compatible with >= 1.2.3 and < 2.0.0)
        let base = parse_semver(rest).ok_or_else(|| Galat::runtime(format!("Constraint caret tidak valid: {}", req), span))?;
        if compare_parsed_semver(&ver, &base) < 0 {
            return Ok(Value::Bool(false));
        }
        if base.major > 0 {
            return Ok(Value::Bool(ver.major == base.major));
        } else if base.minor > 0 {
            return Ok(Value::Bool(ver.major == 0 && ver.minor == base.minor));
        } else {
            return Ok(Value::Bool(ver.major == 0 && ver.minor == 0 && ver.patch == base.patch));
        }
    }

    if let Some(rest) = req.strip_prefix('~') {
        // Tilde constraint ~1.2.3 (compatible with >= 1.2.3 and < 1.3.0)
        let base = parse_semver(rest).ok_or_else(|| Galat::runtime(format!("Constraint tilde tidak valid: {}", req), span))?;
        if compare_parsed_semver(&ver, &base) < 0 {
            return Ok(Value::Bool(false));
        }
        return Ok(Value::Bool(ver.major == base.major && ver.minor == base.minor));
    }

    if let Some(rest) = req.strip_prefix(">=") {
        let base = parse_semver(rest).ok_or_else(|| Galat::runtime("Constraint >= tidak valid", span))?;
        return Ok(Value::Bool(compare_parsed_semver(&ver, &base) >= 0));
    }

    if let Some(rest) = req.strip_prefix("<=") {
        let base = parse_semver(rest).ok_or_else(|| Galat::runtime("Constraint <= tidak valid", span))?;
        return Ok(Value::Bool(compare_parsed_semver(&ver, &base) <= 0));
    }

    if let Some(rest) = req.strip_prefix('>') {
        let base = parse_semver(rest).ok_or_else(|| Galat::runtime("Constraint > tidak valid", span))?;
        return Ok(Value::Bool(compare_parsed_semver(&ver, &base) > 0));
    }

    if let Some(rest) = req.strip_prefix('<') {
        let base = parse_semver(rest).ok_or_else(|| Galat::runtime("Constraint < tidak valid", span))?;
        return Ok(Value::Bool(compare_parsed_semver(&ver, &base) < 0));
    }

    if let Some(rest) = req.strip_prefix("==").or_else(|| req.strip_prefix('=')) {
        let base = parse_semver(rest).ok_or_else(|| Galat::runtime("Constraint == tidak valid", span))?;
        return Ok(Value::Bool(compare_parsed_semver(&ver, &base) == 0));
    }

    let base = parse_semver(req).ok_or_else(|| Galat::runtime(format!("Constraint semver tidak valid: {}", req), span))?;
    Ok(Value::Bool(compare_parsed_semver(&ver, &base) == 0))
}

// ==============================================================================
// WidyaCSV: RFC 4180 CSV & TSV Parser and Serializer
// ==============================================================================
fn parse_csv_lines(text: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;

    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if in_quotes {
            if c == '"' {
                if i + 1 < chars.len() && chars[i + 1] == '"' {
                    current_field.push('"');
                    i += 1;
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
                current_row.push(current_field.clone());
                current_field.clear();
            } else if c == '\r' {
                if i + 1 < chars.len() && chars[i + 1] == '\n' {
                    i += 1;
                }
                current_row.push(current_field.clone());
                current_field.clear();
                rows.push(current_row.clone());
                current_row.clear();
            } else if c == '\n' {
                current_row.push(current_field.clone());
                current_field.clear();
                rows.push(current_row.clone());
                current_row.clear();
            } else {
                current_field.push(c);
            }
        }
        i += 1;
    }

    if !current_field.is_empty() || !current_row.is_empty() {
        current_row.push(current_field);
        rows.push(current_row);
    }

    rows
}

fn builtin_csv_urai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let text = match args.first() {
        Some(v) => v.to_string_repr(),
        None => return Err(Galat::runtime("Fungsi urai_csv memerlukan string teks CSV", span)),
    };
    let delim = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| ",".to_string()).chars().next().unwrap_or(',');
    let has_header = args.get(2).and_then(|v| if let Value::Bool(b) = v { Some(*b) } else { None }).unwrap_or(true);

    let raw_rows = parse_csv_lines(&text, delim);
    if raw_rows.is_empty() {
        return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    if has_header && raw_rows.len() > 1 {
        let headers = &raw_rows[0];
        let mut list = Vec::new();
        for row in &raw_rows[1..] {
            let mut map = std::collections::HashMap::new();
            for (idx, h) in headers.iter().enumerate() {
                let cell_val = row.get(idx).cloned().unwrap_or_default();
                map.insert(h.clone(), Value::String(cell_val));
            }
            list.push(Value::Map(Rc::new(RefCell::new(map))));
        }
        Ok(Value::Array(Rc::new(RefCell::new(list))))
    } else {
        let mut list = Vec::new();
        for row in raw_rows {
            let row_vals: Vec<Value> = row.into_iter().map(Value::String).collect();
            list.push(Value::Array(Rc::new(RefCell::new(row_vals))));
        }
        Ok(Value::Array(Rc::new(RefCell::new(list))))
    }
}

fn builtin_csv_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data_val = match args.first() {
        Some(v) => v,
        None => return Err(Galat::runtime("Fungsi buat_csv memerlukan array data", span)),
    };
    let delim = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| ",".to_string());

    let escape_cell = |s: &str| -> String {
        if s.contains(&delim) || s.contains('"') || s.contains('\n') || s.contains('\r') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    };

    match data_val {
        Value::Array(rows_rc) => {
            let rows = rows_rc.borrow();
            let mut out = String::new();

            for (r_idx, row_val) in rows.iter().enumerate() {
                match row_val {
                    Value::Array(cell_rc) => {
                        let cells = cell_rc.borrow();
                        let line: Vec<String> = cells.iter().map(|c| escape_cell(&c.to_string_repr())).collect();
                        out.push_str(&line.join(&delim));
                        if r_idx + 1 < rows.len() { out.push('\n'); }
                    }
                    Value::Map(m) => {
                        let map = m.borrow();
                        let line: Vec<String> = map.values().map(|c| escape_cell(&c.to_string_repr())).collect();
                        out.push_str(&line.join(&delim));
                        if r_idx + 1 < rows.len() { out.push('\n'); }
                    }
                    _ => {}
                }
            }
            Ok(Value::String(out))
        }
        _ => Err(Galat::runtime("Fungsi buat_csv memerlukan array baris", span)),
    }
}

fn builtin_tsv_urai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let text = args[0].to_string_repr();
    let dummy_args = vec![Value::String(text), Value::String("\t".to_string()), Value::Bool(true)];
    builtin_csv_urai(&dummy_args, span)
}

fn builtin_tsv_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dummy_args = vec![args[0].clone(), Value::String("\t".to_string())];
    builtin_csv_buat(&dummy_args, span)
}

// ==============================================================================
// WidyaLevenshtein: Fuzzy Matching & Edit Distance
// ==============================================================================
fn calc_levenshtein(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    let len1 = v1.len();
    let len2 = v2.len();

    let mut dp = vec![vec![0usize; len2 + 1]; len1 + 1];

    for i in 0..=len1 { dp[i][0] = i; }
    for j in 0..=len2 { dp[0][j] = j; }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if v1[i - 1] == v2[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[len1][len2]
}

fn builtin_lev_jarak(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1 = args[0].to_string_repr();
    let s2 = args[1].to_string_repr();
    let dist = calc_levenshtein(&s1, &s2);
    Ok(Value::Number(dist as f64))
}

fn builtin_lev_kemiripan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1 = args[0].to_string_repr();
    let s2 = args[1].to_string_repr();
    let max_len = s1.chars().count().max(s2.chars().count());
    if max_len == 0 {
        return Ok(Value::Number(1.0));
    }
    let dist = calc_levenshtein(&s1, &s2);
    let ratio = 1.0 - (dist as f64 / max_len as f64);
    let rounded = (ratio * 1000.0).round() / 1000.0;
    Ok(Value::Number(rounded))
}

fn builtin_lev_cari_mirip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let target = match args.first() {
        Some(v) => v.to_string_repr(),
        None => return Err(Galat::runtime("Fungsi cari_kata_mirip memerlukan kata target", span)),
    };
    let candidates_val = match args.get(1) {
        Some(v) => v,
        None => return Err(Galat::runtime("Fungsi cari_kata_mirip memerlukan daftar kandidat kata", span)),
    };
    let min_score = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.4);

    match candidates_val {
        Value::Array(cands_rc) => {
            let cands = cands_rc.borrow();
            let mut matches = Vec::new();

            for cand_val in cands.iter() {
                let cand_str = cand_val.to_string_repr();
                let max_len = target.chars().count().max(cand_str.chars().count());
                let ratio = if max_len == 0 {
                    1.0
                } else {
                    let dist = calc_levenshtein(&target, &cand_str);
                    1.0 - (dist as f64 / max_len as f64)
                };

                if ratio >= min_score {
                    let mut entry = std::collections::HashMap::new();
                    entry.insert("kata".to_string(), Value::String(cand_str));
                    entry.insert("skor".to_string(), Value::Number((ratio * 1000.0).round() / 1000.0));
                    matches.push((ratio, entry));
                }
            }

            matches.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            let result_arr: Vec<Value> = matches.into_iter().map(|(_, e)| Value::Map(Rc::new(RefCell::new(e)))).collect();
            Ok(Value::Array(Rc::new(RefCell::new(result_arr))))
        }
        _ => Err(Galat::runtime("Daftar kandidat harus berupa Array", span)),
    }
}

// ==============================================================================
// WidyaGlob: Wildcard File Path Pattern Matcher
// ==============================================================================
fn glob_match(pattern: &str, path: &str) -> bool {
    let mut regex_pattern = String::from("^");
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    if i + 2 < chars.len() && chars[i + 2] == '/' {
                        regex_pattern.push_str("(?:.*/)?");
                        i += 3;
                        continue;
                    } else {
                        regex_pattern.push_str(".*");
                        i += 2;
                        continue;
                    }
                } else {
                    regex_pattern.push_str("[^/]*");
                }
            }
            '?' => regex_pattern.push_str("[^/]"),
            '.' | '(' | ')' | '+' | '|' | '^' | '$' | '@' | '%' => {
                regex_pattern.push('\\');
                regex_pattern.push(chars[i]);
            }
            c => regex_pattern.push(c),
        }
        i += 1;
    }
    regex_pattern.push('$');

    if let Ok(re) = regex::Regex::new(&regex_pattern) {
        re.is_match(path)
    } else {
        false
    }
}

fn builtin_glob_cocokkan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let pattern = args[0].to_string_repr();
    let path = args[1].to_string_repr();
    Ok(Value::Bool(glob_match(&pattern, &path)))
}

fn builtin_glob_saring(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let list_val = &args[0];
    let pattern = args[1].to_string_repr();

    match list_val {
        Value::Array(arr_rc) => {
            let arr = arr_rc.borrow();
            let mut matches = Vec::new();
            for item in arr.iter() {
                let s = item.to_string_repr();
                if glob_match(&pattern, &s) {
                    matches.push(Value::String(s));
                }
            }
            Ok(Value::Array(Rc::new(RefCell::new(matches))))
        }
        _ => Err(Galat::runtime("Fungsi saring_glob memerlukan Array jalur berkas", span)),
    }
}

// ==============================================================================
// WidyaUUIDv5: Deterministic Namespace UUID Engine (RFC 4122)
// ==============================================================================
fn generate_uuid_v5_raw(namespace_bytes: &[u8; 16], name: &str) -> String {
    use sha2::{Digest, Sha256}; // Deterministic 16-byte slice with version/variant bits
    let mut hasher = Sha256::new();
    hasher.update(namespace_bytes);
    hasher.update(name.as_bytes());
    let hash = hasher.finalize();

    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&hash[0..16]);

    // Version 5 (0101)
    bytes[6] = (bytes[6] & 0x0F) | 0x50;
    // Variant 1 (10)
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    let hex = hex::encode(bytes);
    format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32])
}

const NS_DNS: [u8; 16] = [0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8];
const NS_URL: [u8; 16] = [0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8];

fn parse_uuid_bytes(s: &str) -> Option<[u8; 16]> {
    let clean = s.replace('-', "");
    if clean.len() != 32 { return None; }
    let raw = hex::decode(clean).ok()?;
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&raw[0..16]);
    Some(bytes)
}

fn builtin_uuid_v5(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ns_str = args[0].to_string_repr();
    let name = args[1].to_string_repr();
    let ns_bytes = parse_uuid_bytes(&ns_str).ok_or_else(|| Galat::runtime(format!("Namespace UUID tidak valid: {}", ns_str), span))?;
    Ok(Value::String(generate_uuid_v5_raw(&ns_bytes, &name)))
}

fn builtin_uuid_v5_dns(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let domain = args[0].to_string_repr();
    Ok(Value::String(generate_uuid_v5_raw(&NS_DNS, &domain)))
}

fn builtin_uuid_v5_url(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let url = args[0].to_string_repr();
    Ok(Value::String(generate_uuid_v5_raw(&NS_URL, &url)))
}

// ==============================================================================
// WidyaSchemaValidator: Data Shape & Schema Validation
// ==============================================================================
fn builtin_schema_validasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data_val = &args[0];
    let schema_val = &args[1];

    let mut errors = Vec::new();

    if let (Value::Map(data_m), Value::Map(schema_m)) = (data_val, schema_val) {
        let d = data_m.borrow();
        let s = schema_m.borrow();

        for (field_name, rule_val) in s.iter() {
            if let Value::Map(rule_m) = rule_val {
                let r = rule_m.borrow();
                let is_required = r.get("wajib").map(|v| v.is_truthy()).unwrap_or(false);
                let expected_type = r.get("tipe").map(|v| v.to_string_repr()).unwrap_or_default();

                match d.get(field_name) {
                    None | Some(Value::Nil) => {
                        if is_required {
                            errors.push(Value::String(format!("Bidang '{}' wajib diisi", field_name)));
                        }
                    }
                    Some(val) => {
                        if !expected_type.is_empty() {
                            let actual_type = match val {
                                Value::Number(_) => "angka",
                                Value::String(_) => "teks",
                                Value::Bool(_) => "bool",
                                Value::Array(_) => "array",
                                Value::Map(_) => "map",
                                _ => "lainnya",
                            };
                            if actual_type != expected_type && expected_type != "apapun" {
                                errors.push(Value::String(format!("Bidang '{}' harus bertipe '{}', tetapi bernilai '{}'", field_name, expected_type, actual_type)));
                            }
                        }

                        // Numeric constraints
                        if let Value::Number(n) = val {
                            if let Some(Value::Number(min_val)) = r.get("min") {
                                if *n < *min_val {
                                    errors.push(Value::String(format!("Bidang '{}' bernilai {} lebih kecil dari batas minimum {}", field_name, n, min_val)));
                                }
                            }
                            if let Some(Value::Number(max_val)) = r.get("maks") {
                                if *n > *max_val {
                                    errors.push(Value::String(format!("Bidang '{}' bernilai {} lebih besar dari batas maksimum {}", field_name, n, max_val)));
                                }
                            }
                        }

                        // String length constraints
                        if let Value::String(s_val) = val {
                            if let Some(Value::Number(min_len)) = r.get("min_panjang") {
                                if s_val.chars().count() < *min_len as usize {
                                    errors.push(Value::String(format!("Panjang teks '{}' kurang dari {}", field_name, min_len)));
                                }
                            }
                            if let Some(Value::Number(max_len)) = r.get("maks_panjang") {
                                if s_val.chars().count() > *max_len as usize {
                                    errors.push(Value::String(format!("Panjang teks '{}' melebihi {}", field_name, max_len)));
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        return Err(Galat::runtime("Fungsi validasi_skema memerlukan objek data dan objek skema bertipe Map", span));
    }

    let is_valid = errors.is_empty();
    let mut res = std::collections::HashMap::new();
    res.insert("valid".to_string(), Value::Bool(is_valid));
    res.insert("galat".to_string(), Value::Array(Rc::new(RefCell::new(errors))));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

// ==============================================================================
// WidyaTemplate: Text Interpolation & Template Rendering
// ==============================================================================
fn builtin_template_render(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let template = args[0].to_string_repr();
    let ctx_val = &args[1];

    let ctx_map = match ctx_val {
        Value::Map(m) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Konteks template harus berupa Map", span)),
    };

    let mut out = String::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{' && i + 1 < chars.len() && chars[i + 1] == '{' {
            // Placeholder {{ ... }}
            if let Some(end_idx) = template[i + 2..].find("}}") {
                let tag = template[i + 2..i + 2 + end_idx].trim();
                let val_str = match ctx_map.get(tag) {
                    Some(v) => v.to_string_repr(),
                    None => String::new(),
                };
                out.push_str(&val_str);
                i = i + 2 + end_idx + 2;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }

    Ok(Value::String(out))
}

// ==============================================================================
// WidyaMime: MIME Type Detection & File Extension Mapper
// ==============================================================================
fn get_mime_map() -> &'static [(&'static str, &'static str, bool)] {
    &[
        ("html", "text/html", true),
        ("htm", "text/html", true),
        ("css", "text/css", true),
        ("js", "application/javascript", true),
        ("json", "application/json", true),
        ("xml", "application/xml", true),
        ("txt", "text/plain", true),
        ("md", "text/markdown", true),
        ("csv", "text/csv", true),
        ("png", "image/png", false),
        ("jpg", "image/jpeg", false),
        ("jpeg", "image/jpeg", false),
        ("gif", "image/gif", false),
        ("svg", "image/svg+xml", true),
        ("webp", "image/webp", false),
        ("pdf", "application/pdf", false),
        ("zip", "application/zip", false),
        ("tar", "application/x-tar", false),
        ("wasm", "application/wasm", false),
        ("mp3", "audio/mpeg", false),
        ("wav", "audio/wav", false),
        ("mp4", "video/mp4", false),
        ("wya", "text/x-widya", true),
    ]
}

fn builtin_mime_dari_ekstensi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let filename = args[0].to_string_repr().to_lowercase();
    let ext = filename.split('.').last().unwrap_or(&filename).trim();
    for &(e, m, _) in get_mime_map() {
        if e == ext {
            return Ok(Value::String(m.to_string()));
        }
    }
    Ok(Value::String("application/octet-stream".to_string()))
}

fn builtin_ekstensi_dari_mime(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mime = args[0].to_string_repr().to_lowercase();
    let clean = mime.split(';').next().unwrap_or(&mime).trim();
    for &(e, m, _) in get_mime_map() {
        if m == clean {
            return Ok(Value::String(e.to_string()));
        }
    }
    Ok(Value::String("bin".to_string()))
}

fn builtin_mime_apakah_teks(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mime = args[0].to_string_repr().to_lowercase();
    let clean = mime.split(';').next().unwrap_or(&mime).trim();
    for &(_, m, is_text) in get_mime_map() {
        if m == clean {
            return Ok(Value::Bool(is_text));
        }
    }
    Ok(Value::Bool(clean.starts_with("text/")))
}

// ==============================================================================
// WidyaVersionRange: SemVer Constraint & Version Range Matching
// ==============================================================================
fn parse_semver_parts(v: &str) -> Option<(u64, u64, u64)> {
    let clean = v.trim().trim_start_matches('v').trim_start_matches('=');
    let base = clean.split('-').next().unwrap_or(clean);
    let parts: Vec<&str> = base.split('.').collect();
    if parts.is_empty() { return None; }
    let major = parts.get(0)?.parse::<u64>().ok()?;
    let minor = parts.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
    Some((major, minor, patch))
}

fn check_semver_clause(target: (u64, u64, u64), clause: &str) -> bool {
    let c = clause.trim();
    if c == "*" || c.is_empty() || c == "x" || c == "X" {
        return true;
    }

    // Caret ^ (Compatible with version: ^1.2.3 -> >=1.2.3 <2.0.0, ^0.2.3 -> >=0.2.3 <0.3.0)
    if let Some(rest) = c.strip_prefix('^') {
        if let Some((maj, min, pat)) = parse_semver_parts(rest) {
            if target < (maj, min, pat) { return false; }
            if maj > 0 {
                return target < (maj + 1, 0, 0);
            } else if min > 0 {
                return target < (0, min + 1, 0);
            } else {
                return target < (0, 0, pat + 1);
            }
        }
    }

    // Tilde ~ (Patch updates: ~1.2.3 -> >=1.2.3 <1.3.0)
    if let Some(rest) = c.strip_prefix('~') {
        if let Some((maj, min, pat)) = parse_semver_parts(rest) {
            if target < (maj, min, pat) { return false; }
            return target < (maj, min + 1, 0);
        }
    }

    // Comparison operators
    if let Some(rest) = c.strip_prefix(">=") {
        if let Some(v) = parse_semver_parts(rest) { return target >= v; }
    }
    if let Some(rest) = c.strip_prefix("<=") {
        if let Some(v) = parse_semver_parts(rest) { return target <= v; }
    }
    if let Some(rest) = c.strip_prefix('>') {
        if let Some(v) = parse_semver_parts(rest) { return target > v; }
    }
    if let Some(rest) = c.strip_prefix('<') {
        if let Some(v) = parse_semver_parts(rest) { return target < v; }
    }
    if let Some(rest) = c.strip_prefix('=') {
        if let Some(v) = parse_semver_parts(rest) { return target == v; }
    }

    // Exact match
    if let Some(v) = parse_semver_parts(c) {
        return target == v;
    }

    false
}

fn semver_matches_range_internal(version: &str, range_str: &str) -> bool {
    let target = match parse_semver_parts(version) {
        Some(v) => v,
        None => return false,
    };

    // Split OR ranges: ">=1.0.0 <2.0.0 || >=3.0.0"
    for or_clause in range_str.split("||") {
        let trimmed_or = or_clause.trim();
        if trimmed_or.is_empty() { continue; }

        // AND clauses: space or comma separated
        let and_clauses: Vec<&str> = trimmed_or.split(|c: char| c.is_whitespace() || c == ',').filter(|s| !s.is_empty()).collect();
        let all_pass = and_clauses.iter().all(|clause| check_semver_clause(target, clause));
        if all_pass {
            return true;
        }
    }
    false
}

fn builtin_semver_cocok_rentang(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let ver = args[0].to_string_repr();
    let range = args[1].to_string_repr();
    Ok(Value::Bool(semver_matches_range_internal(&ver, &range)))
}

fn builtin_semver_pilih_terbaik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let range = args[1].to_string_repr();
    let versions_val = &args[0];

    let mut matched_versions = Vec::new();
    match versions_val {
        Value::Array(arr) => {
            for v_val in arr.borrow().iter() {
                let v_str = v_val.to_string_repr();
                if semver_matches_range_internal(&v_str, &range) {
                    if let Some(parsed) = parse_semver_parts(&v_str) {
                        matched_versions.push((parsed, v_str));
                    }
                }
            }
        }
        _ => return Err(Galat::runtime("Fungsi semver_pilih_terbaik memerlukan daftar string versi", span)),
    }

    if matched_versions.is_empty() {
        return Ok(Value::Nil);
    }

    matched_versions.sort_by(|a, b| a.0.cmp(&b.0));
    let best = matched_versions.last().unwrap().1.clone();
    Ok(Value::String(best))
}

fn builtin_semver_validasi_rentang(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let range = args[0].to_string_repr();
    let is_valid = !range.trim().is_empty() && (range.contains('^') || range.contains('~') || range.contains('>') || range.contains('<') || range.contains('=') || range.contains('*') || parse_semver_parts(&range).is_some());
    Ok(Value::Bool(is_valid))
}

// ==============================================================================
// WidyaSoundex: Phonetic String Matching & Name Comparison
// ==============================================================================
fn get_soundex_digit(c: char) -> char {
    match c.to_ascii_uppercase() {
        'B' | 'F' | 'P' | 'V' => '1',
        'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => '2',
        'D' | 'T' => '3',
        'L' => '4',
        'M' | 'N' => '5',
        'R' => '6',
        _ => '0',
    }
}

fn compute_soundex(s: &str) -> String {
    let clean: String = s.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if clean.is_empty() {
        return "0000".to_string();
    }

    let mut chars = clean.chars();
    let first = chars.next().unwrap().to_ascii_uppercase();
    let mut code = String::with_capacity(4);
    code.push(first);

    let mut last_code = get_soundex_digit(first);

    for c in chars {
        let digit = get_soundex_digit(c);
        if digit != '0' && digit != last_code {
            code.push(digit);
            last_code = digit;
            if code.len() == 4 {
                break;
            }
        } else if digit == '0' {
            last_code = '0';
        }
    }

    while code.len() < 4 {
        code.push('0');
    }
    code
}

fn builtin_soundex_kode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    Ok(Value::String(compute_soundex(&s)))
}

fn builtin_soundex_mirip(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1 = args[0].to_string_repr();
    let s2 = args[1].to_string_repr();
    let c1 = compute_soundex(&s1);
    let c2 = compute_soundex(&s2);
    Ok(Value::Bool(c1 == c2))
}

fn builtin_metaphone_kode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let clean: String = s.to_ascii_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).collect();
    // Simplified phonetic transformation
    let mut res = String::new();
    for c in clean.chars() {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' => if res.is_empty() { res.push(c); },
            'B' => res.push('B'),
            'C' | 'K' | 'Q' => res.push('K'),
            'D' | 'T' => res.push('T'),
            'F' | 'V' | 'P' => res.push('F'),
            'G' | 'J' => res.push('J'),
            'L' => res.push('L'),
            'M' => res.push('M'),
            'N' => res.push('N'),
            'R' => res.push('R'),
            'S' | 'Z' | 'X' => res.push('S'),
            _ => {},
        }
    }
    if res.is_empty() { res.push('0'); }
    Ok(Value::String(res))
}

// ==============================================================================
// WidyaBencode: BitTorrent Bencoding RFC 1077 Serializer & Deserializer
// ==============================================================================
fn bencode_encode_val(v: &Value) -> String {
    match v {
        Value::Number(n) => format!("i{}e", n.round() as i64),
        Value::String(s) => format!("{}:{}", s.len(), s),
        Value::Bool(b) => if *b { "i1e".to_string() } else { "i0e".to_string() },
        Value::Array(arr) => {
            let mut s = String::from("l");
            for item in arr.borrow().iter() {
                s.push_str(&bencode_encode_val(item));
            }
            s.push('e');
            s
        }
        Value::Map(m) => {
            let mut s = String::from("d");
            let map = m.borrow();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for k in keys {
                if k.starts_with('_') { continue; } // ignore internal meta fields
                s.push_str(&format!("{}:{}", k.len(), k));
                s.push_str(&bencode_encode_val(map.get(k).unwrap()));
            }
            s.push('e');
            s
        }
        Value::Nil => "0:".to_string(),
        _ => "0:".to_string(),
    }
}

fn bencode_decode_internal(bytes: &[u8], pos: &mut usize) -> Result<Value, String> {
    if *pos >= bytes.len() { return Err("Unexpected EOF".to_string()); }

    match bytes[*pos] {
        b'i' => {
            *pos += 1;
            let start = *pos;
            while *pos < bytes.len() && bytes[*pos] != b'e' {
                *pos += 1;
            }
            if *pos >= bytes.len() { return Err("Unterminated integer".to_string()); }
            let num_str = std::str::from_utf8(&bytes[start..*pos]).map_err(|e| e.to_string())?;
            *pos += 1; // skip 'e'
            let num: f64 = num_str.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
            Ok(Value::Number(num))
        }
        b'l' => {
            *pos += 1;
            let mut list = Vec::new();
            while *pos < bytes.len() && bytes[*pos] != b'e' {
                list.push(bencode_decode_internal(bytes, pos)?);
            }
            if *pos >= bytes.len() { return Err("Unterminated list".to_string()); }
            *pos += 1; // skip 'e'
            Ok(Value::Array(Rc::new(RefCell::new(list))))
        }
        b'd' => {
            *pos += 1;
            let mut map = HashMap::new();
            while *pos < bytes.len() && bytes[*pos] != b'e' {
                let key_val = bencode_decode_internal(bytes, pos)?;
                let key_str = match key_val {
                    Value::String(s) => s,
                    _ => return Err("Dict key must be string".to_string()),
                };
                let val = bencode_decode_internal(bytes, pos)?;
                map.insert(key_str, val);
            }
            if *pos >= bytes.len() { return Err("Unterminated dict".to_string()); }
            *pos += 1; // skip 'e'
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        b'0'..=b'9' => {
            let start = *pos;
            while *pos < bytes.len() && bytes[*pos] != b':' {
                *pos += 1;
            }
            if *pos >= bytes.len() { return Err("Malformed string length".to_string()); }
            let len_str = std::str::from_utf8(&bytes[start..*pos]).map_err(|e| e.to_string())?;
            let len: usize = len_str.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
            *pos += 1; // skip ':'
            if *pos + len > bytes.len() { return Err("String length out of bounds".to_string()); }
            let s = std::str::from_utf8(&bytes[*pos..*pos + len]).map_err(|e| e.to_string())?;
            *pos += len;
            Ok(Value::String(s.to_string()))
        }
        _ => Err(format!("Invalid bencode byte: {}", bytes[*pos])),
    }
}

fn builtin_bencode_enkode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String(bencode_encode_val(&args[0])))
}

fn builtin_bencode_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let mut pos = 0;
    let res = bencode_decode_internal(s.as_bytes(), &mut pos).map_err(|e| Galat::runtime(format!("Gagal dekode Bencode: {}", e), span))?;
    Ok(res)
}

fn builtin_bencode_validasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let mut pos = 0;
    let ok = bencode_decode_internal(s.as_bytes(), &mut pos).is_ok() && pos == s.len();
    Ok(Value::Bool(ok))
}

// ==============================================================================
// WidyaUUIDv6: Reordered Gregorian Time UUID & Comb Engine
// ==============================================================================
fn generate_uuid_v6() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    use rand::Rng;

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    // Gregorian offset in 100-ns intervals between 1582-10-15 and 1970-01-01
    let g_offset = 0x01B21DD213814000u64;
    let intervals = (now.as_secs() * 10_000_000 + (now.subsec_nanos() as u64 / 100)) + g_offset;

    // UUID v6: time_high (32) - time_mid (16) - 0x6 time_low (12) - clock_seq (16) - node (48)
    let time_high = (intervals >> 28) as u32;
    let time_mid = ((intervals >> 12) & 0xFFFF) as u16;
    let time_low_and_version = ((intervals & 0x0FFF) as u16) | 0x6000;

    let mut rng = rand::thread_rng();
    let clock_seq_and_variant: u16 = (rng.r#gen::<u16>() & 0x3FFF) | 0x8000;
    let node: u64 = rng.r#gen::<u64>() & 0x0000FFFFFFFFFFFF;

    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        time_high, time_mid, time_low_and_version, clock_seq_and_variant, node
    )
}

fn builtin_uuid_v6(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String(generate_uuid_v6()))
}

fn builtin_uuid_v6_ke_timestamp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr().replace('-', "");
    if s.len() < 32 { return Err(Galat::runtime("UUID v6 tidak valid", span)); }

    let time_high = u64::from_str_radix(&s[0..8], 16).map_err(|_| Galat::runtime("Invalid time_high", span))?;
    let time_mid = u64::from_str_radix(&s[8..12], 16).map_err(|_| Galat::runtime("Invalid time_mid", span))?;
    let time_low = u64::from_str_radix(&s[13..16], 16).map_err(|_| Galat::runtime("Invalid time_low", span))?;

    let intervals = (time_high << 28) | (time_mid << 12) | time_low;
    let g_offset = 0x01B21DD213814000u64;
    let epoch_intervals = intervals.saturating_sub(g_offset);
    let ms = epoch_intervals / 10_000;
    Ok(Value::Number(ms as f64))
}

fn builtin_uuid_v6_bandingkan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let u1 = args[0].to_string_repr();
    let u2 = args[1].to_string_repr();
    let cmp = match u1.cmp(&u2) {
        std::cmp::Ordering::Greater => 1.0,
        std::cmp::Ordering::Equal => 0.0,
        std::cmp::Ordering::Less => -1.0,
    };
    Ok(Value::Number(cmp))
}

fn builtin_uuid_ke_comb(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let base_uuid = match args.first() {
        Some(v) => v.to_string_repr().replace('-', ""),
        None => format!("{:032x}", rand::random::<u128>()),
    };
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = match args.get(1) {
        Some(Value::Number(n)) => *n as u64,
        _ => SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
    };
    let clean = if base_uuid.len() >= 20 { &base_uuid[..20] } else { &base_uuid };
    let comb = format!("{}{:012x}", clean, ms & 0x0000FFFFFFFFFFFF);
    let formatted = format!("{}-{}-{}-{}-{}", &comb[0..8], &comb[8..12], &comb[12..16], &comb[16..20], &comb[20..32]);
    Ok(Value::String(formatted))
}

// ==============================================================================
// WidyaLevenshteinMatrix: Dynamic Alignment & Diff Matrix Engine
// ==============================================================================
fn compute_lev_matrix(s1: &[char], s2: &[char]) -> Vec<Vec<usize>> {
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp
}

fn builtin_lev_matriks(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1: Vec<char> = args[0].to_string_repr().chars().collect();
    let s2: Vec<char> = args[1].to_string_repr().chars().collect();
    let dp = compute_lev_matrix(&s1, &s2);

    let mut rows = Vec::new();
    for r in dp {
        let cols: Vec<Value> = r.into_iter().map(|n| Value::Number(n as f64)).collect();
        rows.push(Value::Array(Rc::new(RefCell::new(cols))));
    }
    Ok(Value::Array(Rc::new(RefCell::new(rows))))
}

fn builtin_lev_langkah_operasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1: Vec<char> = args[0].to_string_repr().chars().collect();
    let s2: Vec<char> = args[1].to_string_repr().chars().collect();
    let dp = compute_lev_matrix(&s1, &s2);

    let mut i = s1.len();
    let mut j = s2.len();
    let mut ops = Vec::new();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && s1[i - 1] == s2[j - 1] {
            ops.push(format!("SAMA: '{}'", s1[i - 1]));
            i -= 1;
            j -= 1;
        } else if i > 0 && j > 0 && dp[i][j] == dp[i - 1][j - 1] + 1 {
            ops.push(format!("GANTI: '{}' -> '{}'", s1[i - 1], s2[j - 1]));
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i][j] == dp[i - 1][j] + 1 {
            ops.push(format!("HAPUS: '{}'", s1[i - 1]));
            i -= 1;
        } else if j > 0 && dp[i][j] == dp[i][j - 1] + 1 {
            ops.push(format!("SISIP: '{}'", s2[j - 1]));
            j -= 1;
        } else {
            break;
        }
    }
    ops.reverse();
    let val_ops: Vec<Value> = ops.into_iter().map(Value::String).collect();
    Ok(Value::Array(Rc::new(RefCell::new(val_ops))))
}

fn builtin_lev_rasio_kemiripan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1: Vec<char> = args[0].to_string_repr().chars().collect();
    let s2: Vec<char> = args[1].to_string_repr().chars().collect();
    let max_len = s1.len().max(s2.len());
    if max_len == 0 { return Ok(Value::Number(1.0)); }

    let dp = compute_lev_matrix(&s1, &s2);
    let dist = dp[s1.len()][s2.len()];
    let ratio = 1.0 - (dist as f64 / max_len as f64);
    Ok(Value::Number((ratio * 10000.0).round() / 10000.0))
}

