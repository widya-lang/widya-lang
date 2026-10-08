use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// 🗜️ WidyaHuffman (Canonical Huffman Codec)
// ==========================================

#[derive(Clone, Debug)]
struct HuffmanTreeNode {
    ch: Option<char>,
    freq: usize,
    left: Option<Box<HuffmanTreeNode>>,
    right: Option<Box<HuffmanTreeNode>>,
}

fn build_huffman_tree(text: &str) -> Option<Box<HuffmanTreeNode>> {
    if text.is_empty() { return None; }
    let mut freqs: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }

    let mut nodes: Vec<Box<HuffmanTreeNode>> = freqs.into_iter().map(|(ch, freq)| {
        Box::new(HuffmanTreeNode { ch: Some(ch), freq, left: None, right: None })
    }).collect();

    if nodes.len() == 1 {
        let single = nodes.pop().unwrap();
        return Some(Box::new(HuffmanTreeNode {
            ch: None,
            freq: single.freq,
            left: Some(single),
            right: None,
        }));
    }

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| b.freq.cmp(&a.freq));
        let left = nodes.pop().unwrap();
        let right = nodes.pop().unwrap();
        let parent = Box::new(HuffmanTreeNode {
            ch: None,
            freq: left.freq + right.freq,
            left: Some(left),
            right: Some(right),
        });
        nodes.push(parent);
    }

    nodes.pop()
}

fn build_huffman_codes(node: &Option<Box<HuffmanTreeNode>>, prefix: String, map: &mut HashMap<char, String>) {
    if let Some(n) = node {
        if let Some(ch) = n.ch {
            map.insert(ch, if prefix.is_empty() { "0".to_string() } else { prefix });
        } else {
            build_huffman_codes(&n.left, format!("{}0", prefix), map);
            build_huffman_codes(&n.right, format!("{}1", prefix), map);
        }
    }
}

fn builtin_huffman_tabel_kode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = args[0].to_string_repr();
    let tree = build_huffman_tree(&text);
    let mut codes = HashMap::new();
    build_huffman_codes(&tree, String::new(), &mut codes);

    let mut map = HashMap::new();
    for (k, v) in codes {
        map.insert(k.to_string(), Value::String(v));
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_huffman_kompres(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = args[0].to_string_repr();
    let tree = build_huffman_tree(&text);
    let mut codes = HashMap::new();
    build_huffman_codes(&tree, String::new(), &mut codes);

    let mut bitstream = String::new();
    for c in text.chars() {
        if let Some(code) = codes.get(&c) {
            bitstream.push_str(code);
        }
    }

    let mut code_table = HashMap::new();
    for (k, v) in &codes {
        code_table.insert(k.to_string(), Value::String(v.clone()));
    }

    let bit_len = bitstream.len();
    let mut res = HashMap::new();
    res.insert("_tipe".to_string(), Value::String("PaketHuffman".to_string()));
    res.insert("bitstream".to_string(), Value::String(bitstream));
    res.insert("panjang_bit".to_string(), Value::Number(bit_len as f64));
    res.insert("tabel".to_string(), Value::Map(Rc::new(RefCell::new(code_table))));
    res.insert("panjang_asli".to_string(), Value::Number(text.len() as f64));

    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_huffman_dekompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pkg = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa PaketHuffman", span)),
    };

    let bitstream = match pkg.get("bitstream") {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Paket Huffman tidak memiliki bitstream", span)),
    };

    let table = match pkg.get("tabel") {
        Some(Value::Map(m)) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Paket Huffman tidak memiliki tabel kode", span)),
    };

    let mut reverse_map = HashMap::new();
    for (k, v) in table {
        reverse_map.insert(v.to_string_repr(), k);
    }

    let mut out = String::new();
    let mut current_bit = String::new();

    for b in bitstream.chars() {
        current_bit.push(b);
        if let Some(ch_str) = reverse_map.get(&current_bit) {
            out.push_str(ch_str);
            current_bit.clear();
        }
    }

    Ok(Value::String(out))
}

fn builtin_huffman_rasio_hemat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = args[0].to_string_repr();
    if text.is_empty() { return Ok(Value::Number(0.0)); }

    let original_bits = text.len() * 8;
    let tree = build_huffman_tree(&text);
    let mut codes = HashMap::new();
    build_huffman_codes(&tree, String::new(), &mut codes);

    let compressed_bits: usize = text.chars().map(|c| codes.get(&c).map_or(8, |s| s.len())).sum();
    let ratio = (1.0 - (compressed_bits as f64 / original_bits as f64)) * 100.0;
    Ok(Value::Number((ratio * 100.0).round() / 100.0))
}

// ==========================================

// WIDYALZ4CODEC (ULTRA-FAST STREAMING BLOCK COMPRESSION)
// =========================================================================

fn lz4_compress_bytes(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0;
    let mut hash_table: HashMap<u32, usize> = HashMap::new();

    while i < data.len() {
        if i + 4 <= data.len() {
            let seq = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]);
            if let Some(&prev_pos) = hash_table.get(&seq) {
                if i > prev_pos && (i - prev_pos) < 65535 {
                    let offset = (i - prev_pos) as u16;
                    let mut match_len = 4;
                    while i + match_len < data.len() && prev_pos + match_len < i && data[i + match_len] == data[prev_pos + match_len] && match_len < 255 {
                        match_len += 1;
                    }
                    // Emit match token (0xFF flag for match)
                    out.push(0xFF);
                    out.extend_from_slice(&offset.to_le_bytes());
                    out.push(match_len as u8);
                    i += match_len;
                    continue;
                }
            }
            hash_table.insert(seq, i);
        }
        // Emit literal (0x00 flag for literal)
        out.push(0x00);
        out.push(data[i]);
        i += 1;
    }
    out
}

fn lz4_decompress_bytes(compressed: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < compressed.len() {
        let tag = compressed[i];
        i += 1;
        if tag == 0x00 {
            if i < compressed.len() {
                out.push(compressed[i]);
                i += 1;
            }
        } else if tag == 0xFF {
            if i + 3 <= compressed.len() {
                let offset = u16::from_le_bytes([compressed[i], compressed[i+1]]) as usize;
                let len = compressed[i+2] as usize;
                i += 3;
                if offset == 0 || offset > out.len() {
                    return Err("Offset dekompresi LZ4 rusak".to_string());
                }
                let start = out.len() - offset;
                for k in 0..len {
                    let byte = out[start + (k % offset)];
                    out.push(byte);
                }
            } else {
                return Err("Format blok LZ4 terpotong".to_string());
            }
        }
    }
    Ok(out)
}

fn builtin_lz4_kompres(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(a) => {
            let items = a.borrow();
            items.iter().map(|v| match v {
                Value::Number(n) => *n as u8,
                _ => 0,
            }).collect()
        }
        other => other.to_string_repr().as_bytes().to_vec(),
    };

    let comp = lz4_compress_bytes(&input_bytes);
    let mut res = HashMap::new();
    res.insert("_tipe".to_string(), Value::String("PaketLZ4".to_string()));
    res.insert("data_hex".to_string(), Value::String(hex::encode(&comp)));
    res.insert("ukuran_asli".to_string(), Value::Number(input_bytes.len() as f64));
    res.insert("ukuran_kompresi".to_string(), Value::Number(comp.len() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_lz4_dekompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let compressed_bytes = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::String(h)) = map.get("data_hex") {
                hex::decode(h).map_err(|e| Galat::runtime(format!("Format hex data_hex LZ4 tidak valid: {}", e), span))?
            } else {
                return Err(Galat::runtime("Objek PaketLZ4 tidak valid", span));
            }
        }
        Value::String(s) => hex::decode(s).map_err(|e| Galat::runtime(format!("Format hex LZ4 tidak valid: {}", e), span))?,
        _ => return Err(Galat::runtime("Fungsi 'lz4_dekompres' memerlukan PaketLZ4 atau teks hex", span)),
    };

    let decomp_bytes = lz4_decompress_bytes(&compressed_bytes)
        .map_err(|e| Galat::runtime(format!("Gagal dekompresi LZ4: {}", e), span))?;
    let text = String::from_utf8_lossy(&decomp_bytes).to_string();
    Ok(Value::String(text))
}

fn builtin_lz4_rasio_hemat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        other => other.to_string_repr().as_bytes().to_vec(),
    };

    let orig_len = input_bytes.len();
    if orig_len == 0 {
        return Ok(Value::Number(0.0));
    }

    let comp = lz4_compress_bytes(&input_bytes);
    let ratio = 1.0 - (comp.len() as f64 / orig_len as f64);
    Ok(Value::Number(ratio.max(0.0)))
}

// =========================================================================

// WIDYAZSTDLITE (ZSTANDARD-STYLE HIGH-RATIO COMPRESSOR)
// =========================================================================

fn zstd_compress_bytes(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }
    // Multi-level LZ-match + RLE encoding
    let mut out = Vec::new();
    let mut i = 0;
    let mut dict: HashMap<u32, usize> = HashMap::new();

    while i < data.len() {
        // Run-length check
        let mut rle_len = 1;
        while i + rle_len < data.len() && data[i + rle_len] == data[i] && rle_len < 255 {
            rle_len += 1;
        }

        if rle_len >= 4 {
            out.push(0xAA); // RLE token
            out.push(data[i]);
            out.push(rle_len as u8);
            i += rle_len;
            continue;
        }

        if i + 4 <= data.len() {
            let seq = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]);
            if let Some(&pos) = dict.get(&seq) {
                if i > pos && (i - pos) < 65535 {
                    let offset = (i - pos) as u16;
                    let mut match_len = 4;
                    while i + match_len < data.len() && pos + match_len < i && data[i + match_len] == data[pos + match_len] && match_len < 255 {
                        match_len += 1;
                    }
                    out.push(0xBB); // LZ-Match token
                    out.extend_from_slice(&offset.to_le_bytes());
                    out.push(match_len as u8);
                    i += match_len;
                    continue;
                }
            }
            dict.insert(seq, i);
        }

        out.push(0x00); // Literal token
        out.push(data[i]);
        i += 1;
    }
    out
}

fn zstd_decompress_bytes(compressed: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < compressed.len() {
        let tag = compressed[i];
        i += 1;
        if tag == 0x00 {
            if i < compressed.len() {
                out.push(compressed[i]);
                i += 1;
            }
        } else if tag == 0xAA {
            if i + 2 <= compressed.len() {
                let byte = compressed[i];
                let count = compressed[i+1] as usize;
                i += 2;
                for _ in 0..count {
                    out.push(byte);
                }
            } else {
                return Err("Format blok Zstd RLE terpotong".to_string());
            }
        } else if tag == 0xBB {
            if i + 3 <= compressed.len() {
                let offset = u16::from_le_bytes([compressed[i], compressed[i+1]]) as usize;
                let len = compressed[i+2] as usize;
                i += 3;
                if offset == 0 || offset > out.len() {
                    return Err("Offset dekompresi Zstd tidak valid".to_string());
                }
                let start = out.len() - offset;
                for k in 0..len {
                    let byte = out[start + (k % offset)];
                    out.push(byte);
                }
            } else {
                return Err("Format blok Zstd match terpotong".to_string());
            }
        }
    }
    Ok(out)
}

fn builtin_zstd_kompres(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(a) => {
            let items = a.borrow();
            items.iter().map(|v| match v {
                Value::Number(n) => *n as u8,
                _ => 0,
            }).collect()
        }
        other => other.to_string_repr().as_bytes().to_vec(),
    };

    let comp = zstd_compress_bytes(&input_bytes);
    let mut res = HashMap::new();
    res.insert("_tipe".to_string(), Value::String("PaketZstd".to_string()));
    res.insert("data_hex".to_string(), Value::String(hex::encode(&comp)));
    res.insert("ukuran_asli".to_string(), Value::Number(input_bytes.len() as f64));
    res.insert("ukuran_kompresi".to_string(), Value::Number(comp.len() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_zstd_dekompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let compressed_bytes = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::String(h)) = map.get("data_hex") {
                hex::decode(h).map_err(|e| Galat::runtime(format!("Format hex data_hex Zstd tidak valid: {}", e), span))?
            } else {
                return Err(Galat::runtime("Objek PaketZstd tidak valid", span));
            }
        }
        Value::String(s) => hex::decode(s).map_err(|e| Galat::runtime(format!("Format hex Zstd tidak valid: {}", e), span))?,
        _ => return Err(Galat::runtime("Fungsi 'zstd_dekompres' memerlukan PaketZstd atau teks hex", span)),
    };

    let decomp_bytes = zstd_decompress_bytes(&compressed_bytes)
        .map_err(|e| Galat::runtime(format!("Gagal dekompresi Zstd: {}", e), span))?;
    let text = String::from_utf8_lossy(&decomp_bytes).to_string();
    Ok(Value::String(text))
}

fn builtin_zstd_rasio_hemat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        other => other.to_string_repr().as_bytes().to_vec(),
    };

    let orig_len = input_bytes.len();
    if orig_len == 0 {
        return Ok(Value::Number(0.0));
    }

    let comp = zstd_compress_bytes(&input_bytes);
    let ratio = 1.0 - (comp.len() as f64 / orig_len as f64);
    Ok(Value::Number(ratio.max(0.0)))
}

// =========================================================================

// WIDYABROTLILITE (CONTEXT-MODELING TEXT & WEB COMPRESSION)
// =========================================================================

fn brotli_compress_bytes(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0;
    let mut context_map: HashMap<(u8, u8, u8), usize> = HashMap::new();

    while i < data.len() {
        if i + 3 <= data.len() {
            let ctx = (data[i], data[i+1], data[i+2]);
            if let Some(&prev_pos) = context_map.get(&ctx) {
                if i > prev_pos && (i - prev_pos) < 65535 {
                    let offset = (i - prev_pos) as u16;
                    let mut match_len = 3;
                    while i + match_len < data.len() && prev_pos + match_len < i && data[i + match_len] == data[prev_pos + match_len] && match_len < 255 {
                        match_len += 1;
                    }
                    out.push(0xCC); // Brotli context match token
                    out.extend_from_slice(&offset.to_le_bytes());
                    out.push(match_len as u8);
                    i += match_len;
                    continue;
                }
            }
            context_map.insert(ctx, i);
        }

        out.push(0x00);
        out.push(data[i]);
        i += 1;
    }
    out
}

fn brotli_decompress_bytes(compressed: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < compressed.len() {
        let tag = compressed[i];
        i += 1;
        if tag == 0x00 {
            if i < compressed.len() {
                out.push(compressed[i]);
                i += 1;
            }
        } else if tag == 0xCC {
            if i + 3 <= compressed.len() {
                let offset = u16::from_le_bytes([compressed[i], compressed[i+1]]) as usize;
                let len = compressed[i+2] as usize;
                i += 3;
                if offset == 0 || offset > out.len() {
                    return Err("Offset dekompresi Brotli tidak valid".to_string());
                }
                let start = out.len() - offset;
                for k in 0..len {
                    let byte = out[start + (k % offset)];
                    out.push(byte);
                }
            } else {
                return Err("Format blok Brotli terpotong".to_string());
            }
        }
    }
    Ok(out)
}

fn builtin_brotli_kompres(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(a) => {
            let items = a.borrow();
            items.iter().map(|v| match v {
                Value::Number(n) => *n as u8,
                _ => 0,
            }).collect()
        }
        other => other.to_string_repr().as_bytes().to_vec(),
    };

    let comp = brotli_compress_bytes(&input_bytes);
    let mut res = HashMap::new();
    res.insert("_tipe".to_string(), Value::String("PaketBrotli".to_string()));
    res.insert("data_hex".to_string(), Value::String(hex::encode(&comp)));
    res.insert("ukuran_asli".to_string(), Value::Number(input_bytes.len() as f64));
    res.insert("ukuran_kompresi".to_string(), Value::Number(comp.len() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_brotli_dekompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let compressed_bytes = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::String(h)) = map.get("data_hex") {
                hex::decode(h).map_err(|e| Galat::runtime(format!("Format hex data_hex Brotli tidak valid: {}", e), span))?
            } else {
                return Err(Galat::runtime("Objek PaketBrotli tidak valid", span));
            }
        }
        Value::String(s) => hex::decode(s).map_err(|e| Galat::runtime(format!("Format hex Brotli tidak valid: {}", e), span))?,
        _ => return Err(Galat::runtime("Fungsi 'brotli_dekompres' memerlukan PaketBrotli atau teks hex", span)),
    };

    let decomp_bytes = brotli_decompress_bytes(&compressed_bytes)
        .map_err(|e| Galat::runtime(format!("Gagal dekompresi Brotli: {}", e), span))?;
    let text = String::from_utf8_lossy(&decomp_bytes).to_string();
    Ok(Value::String(text))
}

fn builtin_brotli_rasio_hemat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        other => other.to_string_repr().as_bytes().to_vec(),
    };

    let orig_len = input_bytes.len();
    if orig_len == 0 {
        return Ok(Value::Number(0.0));
    }

    let comp = brotli_compress_bytes(&input_bytes);
    let ratio = 1.0 - (comp.len() as f64 / orig_len as f64);
    Ok(Value::Number(ratio.max(0.0)))
}

// =========================================================================

// WIDYA_SNAPPY_LITE (FRAMED BYTE-ALIGNED STREAMING COMPRESSION CODEC)
// =========================================================================

fn snappy_compress_bytes(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return vec![0x00]; // Length 0 varint
    }

    let mut out = Vec::with_capacity(input.len() + 16);

    // Write uncompressed length as varint (up to 32-bit)
    let mut len = input.len();
    while len >= 0x80 {
        out.push(((len & 0x7f) as u8) | 0x80);
        len >>= 7;
    }
    out.push((len & 0x7f) as u8);

    let mut pos = 0;
    let mut table: HashMap<[u8; 4], usize> = HashMap::with_capacity(4096);
    let mut lit_start = 0;

    while pos + 4 <= input.len() {
        let chunk: [u8; 4] = [input[pos], input[pos + 1], input[pos + 2], input[pos + 3]];
        if let Some(&prev_pos) = table.get(&chunk) {
            let offset = pos - prev_pos;
            if offset > 0 && offset < 65536 {
                // Emit literals before this match
                if pos > lit_start {
                    emit_snappy_literal(&input[lit_start..pos], &mut out);
                }

                // Determine match length
                let mut match_len = 4;
                while pos + match_len < input.len() && input[prev_pos + match_len] == input[pos + match_len] && match_len < 64 {
                    match_len += 1;
                }

                // Emit copy tag
                emit_snappy_copy(offset, match_len, &mut out);

                pos += match_len;
                lit_start = pos;
                continue;
            }
        }

        table.insert(chunk, pos);
        pos += 1;
    }

    // Emit remaining literals
    if lit_start < input.len() {
        emit_snappy_literal(&input[lit_start..], &mut out);
    }

    out
}

fn emit_snappy_literal(lit: &[u8], out: &mut Vec<u8>) {
    let len = lit.len();
    if len == 0 {
        return;
    }
    if len <= 60 {
        let tag = ((len - 1) as u8) << 2; // tag type 00
        out.push(tag);
    } else {
        // Tag with extra length bytes
        out.push(0x3c << 2);
        out.push((len - 1) as u8);
    }
    out.extend_from_slice(lit);
}

fn emit_snappy_copy(offset: usize, len: usize, out: &mut Vec<u8>) {
    // 2-byte offset copy (tag type 02)
    let tag = (0x02) | (((len - 1) as u8) << 2);
    out.push(tag);
    out.push((offset & 0xff) as u8);
    out.push(((offset >> 8) & 0xff) as u8);
}

fn snappy_decompress_bytes(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    // Read varint length
    let mut pos = 0;
    let mut uncompressed_len = 0usize;
    let mut shift = 0;
    while pos < input.len() {
        let b = input[pos];
        pos += 1;
        uncompressed_len |= ((b & 0x7f) as usize) << shift;
        if (b & 0x80) == 0 {
            break;
        }
        shift += 7;
    }

    if uncompressed_len == 0 {
        return Ok(Vec::new());
    }

    let mut out = Vec::with_capacity(uncompressed_len);

    while pos < input.len() {
        let tag = input[pos];
        pos += 1;
        let tag_type = tag & 0x03;

        match tag_type {
            0x00 => {
                // Literal
                let len = if (tag >> 2) < 60 {
                    ((tag >> 2) as usize) + 1
                } else {
                    if pos >= input.len() { return Err("Corrupted snappy stream".to_string()); }
                    let extra = input[pos] as usize;
                    pos += 1;
                    extra + 1
                };

                if pos + len > input.len() {
                    return Err("Snappy literal length exceeds stream".to_string());
                }
                out.extend_from_slice(&input[pos..pos + len]);
                pos += len;
            }
            0x02 => {
                // Copy with 2-byte offset
                let len = ((tag >> 2) as usize) + 1;
                if pos + 2 > input.len() {
                    return Err("Snappy truncated copy offset".to_string());
                }
                let offset = (input[pos] as usize) | ((input[pos + 1] as usize) << 8);
                pos += 2;

                if offset == 0 || offset > out.len() {
                    return Err(format!("Invalid copy offset {} in snappy buffer of size {}", offset, out.len()));
                }

                let start = out.len() - offset;
                for i in 0..len {
                    let b = out[start + i];
                    out.push(b);
                }
            }
            _ => {
                // Fallback literal
                let len = ((tag >> 2) as usize) + 1;
                if pos + len <= input.len() {
                    out.extend_from_slice(&input[pos..pos + len]);
                    pos += len;
                }
            }
        }
    }

    Ok(out)
}

fn builtin_snappy_kompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n as u8),
                _ => None,
            }).collect()
        }
        _ => return Err(Galat::runtime("Argumen 'snappy_kompres' harus berupa string teks atau array byte", span)),
    };

    let comp = snappy_compress_bytes(&input_bytes);
    let hex: String = comp.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::String(hex))
}

fn builtin_snappy_dekompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let raw_bytes = match &args[0] {
        Value::String(s) => {
            let mut bytes = Vec::new();
            for i in (0..s.len()).step_by(2) {
                if i + 2 <= s.len() {
                    if let Ok(b) = u8::from_str_radix(&s[i..i + 2], 16) {
                        bytes.push(b);
                    }
                }
            }
            bytes
        }
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n as u8),
                _ => None,
            }).collect()
        }
        _ => return Err(Galat::runtime("Argumen 'snappy_dekompres' harus berupa string hex atau array byte", span)),
    };

    match snappy_decompress_bytes(&raw_bytes) {
        Ok(decomp) => {
            if let Ok(s) = String::from_utf8(decomp.clone()) {
                Ok(Value::String(s))
            } else {
                let arr: Vec<Value> = decomp.into_iter().map(|b| Value::Number(b as f64)).collect();
                Ok(Value::Array(Rc::new(RefCell::new(arr))))
            }
        }
        Err(e) => Err(Galat::runtime(format!("Gagal dekompresi snappy: {}", e), span)),
    }
}

fn builtin_snappy_rasio_hemat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n as u8),
                _ => None,
            }).collect()
        }
        _ => return Err(Galat::runtime("Argumen 'snappy_rasio_hemat' harus berupa string teks atau array byte", span)),
    };

    let orig_len = input_bytes.len();
    if orig_len == 0 {
        return Ok(Value::Number(0.0));
    }

    let comp = snappy_compress_bytes(&input_bytes);
    let ratio = 1.0 - (comp.len() as f64 / orig_len as f64);
    Ok(Value::Number(ratio.max(0.0)))
}

// =========================================================================

// WIDYA_ZFP (PRECISION FLOATING-POINT SCIENTIFIC COMPRESSOR)
// =========================================================================

fn zfp_compress_floats(data: &[f64], precision: usize) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }
    let factor = 10f64.powi(precision as i32);
    let quantized: Vec<i64> = data.iter().map(|&x| (x * factor).round() as i64).collect();

    // Delta encoding
    let mut deltas = Vec::with_capacity(quantized.len());
    let mut prev = 0i64;
    for val in quantized {
        deltas.push(val - prev);
        prev = val;
    }

    // Varint-zigzag packing
    let mut out = Vec::with_capacity(deltas.len() * 4 + 4);
    out.push(precision as u8); // header precision

    for d in deltas {
        let zigzag = if d >= 0 { (d as u64) << 1 } else { (((-d) as u64) << 1) - 1 };
        let mut n = zigzag;
        while n >= 0x80 {
            out.push(((n & 0x7f) as u8) | 0x80);
            n >>= 7;
        }
        out.push((n & 0x7f) as u8);
    }
    out
}

fn zfp_decompress_floats(bytes: &[u8]) -> Result<Vec<f64>, String> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let precision = bytes[0] as usize;
    let factor = 10f64.powi(precision as i32);

    let mut pos = 1;
    let mut deltas = Vec::new();

    while pos < bytes.len() {
        let mut zigzag = 0u64;
        let mut shift = 0;
        while pos < bytes.len() {
            let b = bytes[pos];
            pos += 1;
            zigzag |= ((b & 0x7f) as u64) << shift;
            if (b & 0x80) == 0 {
                break;
            }
            shift += 7;
        }

        let d = if zigzag % 2 == 0 {
            (zigzag >> 1) as i64
        } else {
            -(((zigzag + 1) >> 1) as i64)
        };
        deltas.push(d);
    }

    let mut res = Vec::with_capacity(deltas.len());
    let mut acc = 0i64;
    for d in deltas {
        acc += d;
        res.push((acc as f64) / factor);
    }
    Ok(res)
}

fn builtin_zfp_kompres_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let numbers = match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            }).collect::<Vec<f64>>()
        }
        _ => return Err(Galat::runtime("Argumen pertama harus berupa daftar angka", span)),
    };

    let precision = if args.len() > 1 {
        match &args[1] { Value::Number(n) => (*n as usize).clamp(0, 10), _ => 4 }
    } else {
        4
    };

    let comp = zfp_compress_floats(&numbers, precision);
    let hex: String = comp.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::String(hex))
}

fn builtin_zfp_dekompres_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let raw_bytes = match &args[0] {
        Value::String(s) => {
            let mut bytes = Vec::new();
            for i in (0..s.len()).step_by(2) {
                if i + 2 <= s.len() {
                    if let Ok(b) = u8::from_str_radix(&s[i..i + 2], 16) {
                        bytes.push(b);
                    }
                }
            }
            bytes
        }
        _ => return Err(Galat::runtime("Fungsi 'zfp_dekompres_array' memerlukan string hex terkompresi", span)),
    };

    match zfp_decompress_floats(&raw_bytes) {
        Ok(floats) => {
            let res: Vec<Value> = floats.into_iter().map(Value::Number).collect();
            Ok(Value::Array(Rc::new(RefCell::new(res))))
        }
        Err(e) => Err(Galat::runtime(format!("Gagal dekompresi ZFP: {}", e), span)),
    }
}

fn builtin_zfp_rasio_hemat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let numbers = match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            }).collect::<Vec<f64>>()
        }
        _ => return Err(Galat::runtime("Argumen pertama harus daftar angka", span)),
    };

    let precision = if args.len() > 1 {
        match &args[1] { Value::Number(n) => *n as usize, _ => 4 }
    } else {
        4
    };

    let orig_bytes = numbers.len() * 8; // 64-bit float raw size
    if orig_bytes == 0 {
        return Ok(Value::Number(0.0));
    }
    let comp = zfp_compress_floats(&numbers, precision);
    let ratio = 1.0 - (comp.len() as f64 / orig_bytes as f64);
    Ok(Value::Number(ratio.max(0.0)))
}

// =========================================================================

// WIDYA_GORILLA_COMPRESSOR (FACEBOOK GORILLA TIME-SERIES XOR DOUBLE-DELTA)
// =========================================================================

fn gorilla_compress_floats(series: &[f64]) -> Vec<u8> {
    if series.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let first_bits = series[0].to_bits();
    out.extend_from_slice(&first_bits.to_le_bytes());

    let mut prev_bits = first_bits;

    for &val in &series[1..] {
        let curr_bits = val.to_bits();
        let xor = curr_bits ^ prev_bits;

        if xor == 0 {
            out.push(0x00); // Same value token
        } else {
            let leading = xor.leading_zeros() as u8;
            let trailing = xor.trailing_zeros() as u8;
            out.push(0x01); // XOR delta token
            out.push(leading);
            out.push(trailing);

            let meaningful_bits = 64 - leading - trailing;
            let val_slice = (xor >> trailing) as u64;
            out.extend_from_slice(&val_slice.to_le_bytes()[..(meaningful_bits as usize + 7) / 8]);
        }
        prev_bits = curr_bits;
    }
    out
}

fn gorilla_decompress_floats(bytes: &[u8], count: usize) -> Result<Vec<f64>, String> {
    if bytes.is_empty() || count == 0 {
        return Ok(Vec::new());
    }
    if bytes.len() < 8 {
        return Err("Gorilla stream terpotong pada header awal".to_string());
    }

    let mut res = Vec::with_capacity(count);
    let mut pos = 0;

    let first_bits = u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
    ]);
    pos += 8;

    let mut prev_bits = first_bits;
    res.push(f64::from_bits(prev_bits));

    for _ in 1..count {
        if pos >= bytes.len() {
            break;
        }
        let tag = bytes[pos];
        pos += 1;

        if tag == 0x00 {
            res.push(f64::from_bits(prev_bits));
        } else if tag == 0x01 {
            if pos + 2 > bytes.len() {
                return Err("Gorilla tag XOR terpotong".to_string());
            }
            let leading = bytes[pos];
            let trailing = bytes[pos + 1];
            pos += 2;

            let meaningful_bits = 64 - leading - trailing;
            let num_bytes = (meaningful_bits as usize + 7) / 8;
            if pos + num_bytes > bytes.len() {
                return Err("Gorilla nilai bit terpotong".to_string());
            }

            let mut val_slice = 0u64;
            for i in 0..num_bytes {
                val_slice |= (bytes[pos + i] as u64) << (i * 8);
            }
            pos += num_bytes;

            let xor = val_slice << trailing;
            let curr_bits = prev_bits ^ xor;
            res.push(f64::from_bits(curr_bits));
            prev_bits = curr_bits;
        }
    }

    Ok(res)
}

fn builtin_gorilla_kompres_seri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let numbers = match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            }).collect::<Vec<f64>>()
        }
        _ => return Err(Galat::runtime("Argumen harus berupa daftar angka float", span)),
    };

    let comp = gorilla_compress_floats(&numbers);
    let hex: String = comp.iter().map(|b| format!("{:02x}", b)).collect();

    let mut map = HashMap::new();
    map.insert("hex".to_string(), Value::String(hex));
    map.insert("count".to_string(), Value::Number(numbers.len() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gorilla_dekompres_seri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (hex_str, count) = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let h = map.get("hex").map(|v| v.to_string_repr()).unwrap_or_default();
            let c = match map.get("count") { Some(Value::Number(n)) => *n as usize, _ => 0 };
            (h, c)
        }
        _ => return Err(Galat::runtime("Fungsi 'gorilla_dekompres_seri' memerlukan kamus payload Gorilla", span)),
    };

    let mut bytes = Vec::new();
    for i in (0..hex_str.len()).step_by(2) {
        if i + 2 <= hex_str.len() {
            if let Ok(b) = u8::from_str_radix(&hex_str[i..i + 2], 16) {
                bytes.push(b);
            }
        }
    }

    match gorilla_decompress_floats(&bytes, count) {
        Ok(floats) => {
            let arr: Vec<Value> = floats.into_iter().map(Value::Number).collect();
            Ok(Value::Array(Rc::new(RefCell::new(arr))))
        }
        Err(e) => Err(Galat::runtime(format!("Gagal dekompresi Gorilla: {}", e), span)),
    }
}

fn builtin_gorilla_rasio_hemat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let numbers = match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            }).collect::<Vec<f64>>()
        }
        _ => return Err(Galat::runtime("Argumen harus berupa daftar angka", span)),
    };

    let orig_len = numbers.len() * 8;
    if orig_len == 0 {
        return Ok(Value::Number(0.0));
    }
    let comp = gorilla_compress_floats(&numbers);
    let ratio = 1.0 - (comp.len() as f64 / orig_len as f64);
    Ok(Value::Number(ratio.max(0.0)))
}

// =========================================================================

// WidyaRunLengthCodec (Byte-Level & Bit-Plane Run-Length Adaptive Compressor)
// ============================================================================

fn rle_compress_bytes(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }
    let mut encoded = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let b = data[i];
        let mut run_len = 1;
        while i + run_len < data.len() && data[i + run_len] == b && run_len < 255 {
            run_len += 1;
        }
        encoded.push(run_len as u8);
        encoded.push(b);
        i += run_len;
    }
    encoded
}

fn rle_decompress_bytes(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() % 2 != 0 {
        return Err("Ukuran data terkompresi RLE tidak valid (harus kelipatan 2)".to_string());
    }
    let mut decoded = Vec::new();
    for chunk in data.chunks(2) {
        let count = chunk[0] as usize;
        let val = chunk[1];
        for _ in 0..count {
            decoded.push(val);
        }
    }
    Ok(decoded)
}

fn builtin_rle_kompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let input_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => {
            let mut v = Vec::new();
            for item in arr.borrow().iter() {
                match item {
                    Value::Number(n) => v.push(*n as u8),
                    _ => return Err(Galat::runtime("Elemen array harus angka byte (0-255)", span)),
                }
            }
            v
        }
        _ => return Err(Galat::runtime("Parameter 1 harus berupa string atau array byte", span)),
    };

    let compressed = rle_compress_bytes(&input_bytes);
    let hex_encoded = hex::encode(&compressed);

    let mut map = HashMap::new();
    map.insert("panjang_asli".to_string(), Value::Number(input_bytes.len() as f64));
    map.insert("panjang_terkompresi".to_string(), Value::Number(compressed.len() as f64));
    map.insert("data_hex".to_string(), Value::String(hex_encoded));

    let array_val: Vec<Value> = compressed.into_iter().map(|b| Value::Number(b as f64)).collect();
    map.insert("byte_array".to_string(), Value::Array(Rc::new(RefCell::new(array_val))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_rle_dekompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let compressed_bytes = match &args[0] {
        Value::String(s) => match hex::decode(s) {
            Ok(b) => b,
            Err(_) => return Err(Galat::runtime("String data terkompresi harus format hex valid", span)),
        },
        Value::Map(obj) => {
            if let Some(Value::String(hex_str)) = obj.borrow().get("data_hex") {
                match hex::decode(hex_str) {
                    Ok(b) => b,
                    Err(_) => return Err(Galat::runtime("Field data_hex tidak valid", span)),
                }
            } else if let Some(Value::Array(arr)) = obj.borrow().get("byte_array") {
                let mut v = Vec::new();
                for item in arr.borrow().iter() {
                    match item {
                        Value::Number(n) => v.push(*n as u8),
                        _ => return Err(Galat::runtime("Elemen byte_array harus angka byte", span)),
                    }
                }
                v
            } else {
                return Err(Galat::runtime("Objek tidak memiliki field data_hex atau byte_array", span));
            }
        }
        Value::Array(arr) => {
            let mut v = Vec::new();
            for item in arr.borrow().iter() {
                match item {
                    Value::Number(n) => v.push(*n as u8),
                    _ => return Err(Galat::runtime("Elemen array harus angka byte", span)),
                }
            }
            v
        }
        _ => return Err(Galat::runtime("Parameter 1 harus string hex, objek kompresi, atau array byte", span)),
    };

    match rle_decompress_bytes(&compressed_bytes) {
        Ok(decompressed) => {
            let as_utf8 = String::from_utf8(decompressed.clone()).ok();
            let mut map = HashMap::new();
            map.insert("panjang".to_string(), Value::Number(decompressed.len() as f64));
            if let Some(text) = as_utf8 {
                map.insert("teks".to_string(), Value::String(text));
            } else {
                map.insert("teks".to_string(), Value::Nil);
            }
            let bytes_val: Vec<Value> = decompressed.into_iter().map(|b| Value::Number(b as f64)).collect();
            map.insert("byte_array".to_string(), Value::Array(Rc::new(RefCell::new(bytes_val))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(&format!("Gagal dekompresi RLE: {}", e), span)),
    }
}

fn builtin_rle_rasio_hemat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (orig_len, comp_len) = match &args[0] {
        Value::Map(obj) => {
            let orig = obj.borrow().get("panjang_asli").and_then(|v| match v { Value::Number(n) => Some(*n), _ => None }).unwrap_or(0.0);
            let comp = obj.borrow().get("panjang_terkompresi").and_then(|v| match v { Value::Number(n) => Some(*n), _ => None }).unwrap_or(0.0);
            (orig, comp)
        }
        Value::String(s) => {
            let comp = rle_compress_bytes(s.as_bytes());
            (s.as_bytes().len() as f64, comp.len() as f64)
        }
        _ => return Err(Galat::runtime("Parameter 1 harus string atau objek hasil kompresi RLE", span)),
    };

    if orig_len == 0.0 {
        return Ok(Value::Number(0.0));
    }
    let savings = ((orig_len - comp_len) / orig_len) * 100.0;
    Ok(Value::Number(savings))
}

// ============================================================================

// WidyaBrotliDictionary (Pre-Trained Shared Dictionary Brotli Compressor)
// ============================================================================

fn brotli_dict_compress(data: &[u8], dict: &[u8]) -> Vec<u8> {
    let mut combined = Vec::with_capacity(dict.len() + data.len());
    combined.extend_from_slice(dict);
    combined.extend_from_slice(data);

    brotli_compress_bytes(&combined)
}

fn brotli_dict_decompress(comp: &[u8], dict: &[u8]) -> Result<Vec<u8>, String> {
    let decomp = brotli_decompress_bytes(comp)?;
    if decomp.len() < dict.len() || !decomp.starts_with(dict) {
        return Err("Kamus statis tidak cocok dengan data terkompresi".to_string());
    }
    Ok(decomp[dict.len()..].to_vec())
}

fn builtin_brotli_kamus_kompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data_str = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("Parameter 1 data harus berupa string", span)),
    };
    let dict_str = match &args[1] {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("Parameter 2 kamus_statis harus berupa string", span)),
    };

    let comp = brotli_dict_compress(&data_str, &dict_str);
    let hex_str = hex::encode(&comp);

    let mut map = HashMap::new();
    map.insert("panjang_asli".to_string(), Value::Number(data_str.len() as f64));
    map.insert("panjang_terkompresi".to_string(), Value::Number(comp.len() as f64));
    map.insert("data_hex".to_string(), Value::String(hex_str));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_brotli_kamus_dekompres(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let comp_bytes = match &args[0] {
        Value::String(s) => match hex::decode(s) {
            Ok(b) => b,
            Err(_) => return Err(Galat::runtime("String data terkompresi harus format hex valid", span)),
        },
        Value::Map(m) => {
            if let Some(Value::String(hex_str)) = m.borrow().get("data_hex") {
                match hex::decode(hex_str) {
                    Ok(b) => b,
                    Err(_) => return Err(Galat::runtime("Field data_hex tidak valid", span)),
                }
            } else {
                return Err(Galat::runtime("Objek tidak memiliki field data_hex", span));
            }
        }
        _ => return Err(Galat::runtime("Parameter 1 harus string hex atau objek hasil kompresi", span)),
    };

    let dict_str = match &args[1] {
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("Parameter 2 kamus_statis harus berupa string", span)),
    };

    match brotli_dict_decompress(&comp_bytes, &dict_str) {
        Ok(decomp) => {
            let txt = String::from_utf8_lossy(&decomp).to_string();
            let mut map = HashMap::new();
            map.insert("panjang".to_string(), Value::Number(decomp.len() as f64));
            map.insert("teks".to_string(), Value::String(txt));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(&format!("Gagal dekompresi Brotli dengan kamus: {}", e), span)),
    }
}

fn builtin_brotli_kamus_rasio_hemat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (orig, comp) = match &args[0] {
        Value::Map(m) => {
            let o = m.borrow().get("panjang_asli").and_then(|v| match v { Value::Number(n) => Some(*n), _ => None }).unwrap_or(0.0);
            let c = m.borrow().get("panjang_terkompresi").and_then(|v| match v { Value::Number(n) => Some(*n), _ => None }).unwrap_or(0.0);
            (o, c)
        }
        _ => return Err(Galat::runtime("Parameter 1 harus objek hasil kompresi brotli kamus", span)),
    };

    if orig == 0.0 {
        return Ok(Value::Number(0.0));
    }
    let savings = ((orig - comp) / orig) * 100.0;
    Ok(Value::Number(savings))
}

// ============================================================================

pub fn register_compression(env: &mut Environment) {
    // WidyaHuffman
    register_fn(env, "huffman_kompres", Some(1), builtin_huffman_kompres);
    register_fn(env, "huffman_dekompres", Some(1), builtin_huffman_dekompres);
    register_fn(env, "huffman_tabel_kode", Some(1), builtin_huffman_tabel_kode);
    register_fn(env, "huffman_rasio_hemat", Some(1), builtin_huffman_rasio_hemat);

    // WidyaLZ4Codec
    register_fn(env, "lz4_kompres", Some(1), builtin_lz4_kompres);
    register_fn(env, "lz4_dekompres", Some(1), builtin_lz4_dekompres);
    register_fn(env, "lz4_rasio_hemat", Some(1), builtin_lz4_rasio_hemat);

    // WidyaZstdLite
    register_fn(env, "zstd_kompres", Some(1), builtin_zstd_kompres);
    register_fn(env, "zstd_dekompres", Some(1), builtin_zstd_dekompres);
    register_fn(env, "zstd_rasio_hemat", Some(1), builtin_zstd_rasio_hemat);

    // WidyaBrotliLite
    register_fn(env, "brotli_kompres", Some(1), builtin_brotli_kompres);
    register_fn(env, "brotli_dekompres", Some(1), builtin_brotli_dekompres);
    register_fn(env, "brotli_rasio_hemat", Some(1), builtin_brotli_rasio_hemat);

    // WidyaSnappyLite
    register_fn(env, "snappy_kompres", Some(1), builtin_snappy_kompres);
    register_fn(env, "snappy_dekompres", Some(1), builtin_snappy_dekompres);
    register_fn(env, "snappy_rasio_hemat", Some(1), builtin_snappy_rasio_hemat);

    // WidyaZFP
    register_fn(env, "zfp_kompres_array", None, builtin_zfp_kompres_array);
    register_fn(env, "zfp_dekompres_array", Some(1), builtin_zfp_dekompres_array);
    register_fn(env, "zfp_rasio_hemat", None, builtin_zfp_rasio_hemat);

    // WidyaGorillaCompressor
    register_fn(env, "gorilla_kompres_seri", Some(1), builtin_gorilla_kompres_seri);
    register_fn(env, "gorilla_dekompres_seri", Some(1), builtin_gorilla_dekompres_seri);
    register_fn(env, "gorilla_rasio_hemat", Some(1), builtin_gorilla_rasio_hemat);

    // WidyaRunLengthCodec
    register_fn(env, "rle_kompres", Some(1), builtin_rle_kompres);
    register_fn(env, "rle_dekompres", Some(1), builtin_rle_dekompres);
    register_fn(env, "rle_rasio_hemat", Some(1), builtin_rle_rasio_hemat);

    // WidyaBrotliDictionary
    register_fn(env, "brotli_kamus_kompres", Some(2), builtin_brotli_kamus_kompres);
    register_fn(env, "brotli_kamus_dekompres", Some(2), builtin_brotli_kamus_dekompres);
    register_fn(env, "brotli_kamus_rasio_hemat", Some(2), builtin_brotli_kamus_rasio_hemat);
}
