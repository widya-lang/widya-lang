use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::{baca_memori_volatil, emit_baris, emit_teks, register_fn, tulis_memori_volatil};
use crate::value::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

pub fn builtin_cetak(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let parts: Vec<String> = args.iter().map(|a| a.to_string_repr()).collect();
    emit_baris(parts.join(" "));
    Ok(Value::Nil)
}

pub fn builtin_tulis(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let parts: Vec<String> = args.iter().map(|a| a.to_string_repr()).collect();
    emit_teks(parts.join(" "));
    Ok(Value::Nil)
}

pub fn builtin_baca(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    if let Some(prompt) = args.first() {
        print!("{}", prompt.to_string_repr());
        let _ = io::stdout().flush();
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| Galat::runtime(format!("Gagal membaca input: {}", e), _span))?;
    let trimmed = input.trim_end_matches(&['\r', '\n'][..]).to_string();
    Ok(Value::String(trimmed))
}

pub fn builtin_baca_berkas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(path) => match fs::read_to_string(path) {
            Ok(content) => Ok(Value::String(content)),
            Err(e) => Err(Galat::runtime(format!("Gagal membaca berkas '{}': {}", path, e), span)),
        },
        _ => Err(Galat::runtime("Fungsi 'baca_berkas' memerlukan path berkas berupa teks", span)),
    }
}

pub fn builtin_tulis_berkas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => match fs::write(path, content) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(e) => Err(Galat::runtime(format!("Gagal menulis berkas '{}': {}", path, e), span)),
        },
        _ => Err(Galat::runtime("Fungsi 'tulis_berkas' memerlukan (path_teks, isi_teks)", span)),
    }
}

pub fn builtin_tulis_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::String(path), Value::Array(bytes_arr)) => {
            let bytes: Result<Vec<u8>, Galat> = bytes_arr.borrow().iter().map(|v| match v {
                Value::Number(n) => Ok(*n as u8),
                _ => Err(Galat::runtime("Array byte harus berisi angka (0-255)", span)),
            }).collect();
            let bytes = bytes?;
            match fs::write(path, &bytes) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(Galat::runtime(format!("Gagal menulis berkas biner '{}': {}", path, e), span)),
            }
        }
        _ => Err(Galat::runtime("Fungsi 'tulis_biner' memerlukan (path_teks, daftar_byte)", span)),
    }
}

pub fn builtin_baca_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(path) => match fs::read(path) {
            Ok(bytes) => {
                let values: Vec<Value> = bytes.into_iter().map(|b| Value::Number(b as f64)).collect();
                Ok(Value::Array(Rc::new(RefCell::new(values))))
            }
            Err(e) => Err(Galat::runtime(format!("Gagal membaca berkas biner '{}': {}", path, e), span)),
        },
        _ => Err(Galat::runtime("Fungsi 'baca_biner' memerlukan path berkas berupa teks", span)),
    }
}

pub fn builtin_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(cmd) => {
            let output = if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(["/C", cmd])
                    .output()
            } else {
                std::process::Command::new("sh")
                    .args(["-c", cmd])
                    .output()
            };

            match output {
                Ok(out) => {
                    let mut res = HashMap::new();
                    res.insert("sukses".to_string(), Value::Bool(out.status.success()));
                    res.insert("kode_keluar".to_string(), Value::Number(out.status.code().unwrap_or(-1) as f64));
                    res.insert("stdout".to_string(), Value::String(String::from_utf8_lossy(&out.stdout).to_string()));
                    res.insert("stderr".to_string(), Value::String(String::from_utf8_lossy(&out.stderr).to_string()));
                    Ok(Value::Map(Rc::new(RefCell::new(res))))
                }
                Err(e) => Err(Galat::runtime(format!("Gagal mengeksekusi perintah '{}': {}", cmd, e), span)),
            }
        }
        _ => Err(Galat::runtime("Fungsi 'eksekusi' memerlukan teks perintah shell", span)),
    }
}

pub fn builtin_baca_env(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(k) => match std::env::var(k) {
            Ok(v) => Ok(Value::String(v)),
            Err(_) => Ok(Value::Nil),
        },
        _ => Err(Galat::runtime("Fungsi 'baca_env' memerlukan nama variabel berupa teks", span)),
    }
}

pub fn builtin_tulis_env(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::String(k), Value::String(v)) => {
            unsafe { std::env::set_var(k, v); }
            Ok(Value::Bool(true))
        },
        (Value::String(k), other) => {
            let val = other.to_string_repr();
            unsafe { std::env::set_var(k, val); }
            Ok(Value::Bool(true))
        },
        _ => Err(Galat::runtime("Fungsi 'tulis_env' memerlukan (nama_variabel, nilai)", span)),
    }
}

pub fn builtin_berkas_ada(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(path) => Ok(Value::Bool(std::path::Path::new(path).exists())),
        _ => Err(Galat::runtime("Fungsi 'berkas_ada' memerlukan path berkas berupa teks", span)),
    }
}

pub fn builtin_hapus_berkas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(path) => match std::fs::remove_file(path) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(_) => Ok(Value::Bool(false)),
        },
        _ => Err(Galat::runtime("Fungsi 'hapus_berkas' memerlukan path berkas berupa teks", span)),
    }
}

pub fn builtin_baca_volatil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let addr = match args[0] {
        Value::Number(n) => n as usize,
        _ => return Err(Galat::runtime("Alamat memori harus berupa angka", span)),
    };
    baca_memori_volatil(addr, span)
}

pub fn builtin_tulis_volatil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let addr = match args[0] {
        Value::Number(n) => n as usize,
        _ => return Err(Galat::runtime("Alamat memori harus berupa angka", span)),
    };
    tulis_memori_volatil(addr, &args[1], span)
}

pub fn builtin_alokasi_buffer_mentah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ukuran = match args[0] {
        Value::Number(n) => n as usize,
        _ => return Err(Galat::runtime("Ukuran buffer harus berupa angka", span)),
    };
    let base_addr = 0x20000000 + (ukuran * 4);
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PenunjukMentah".to_string()));
    map.insert("alamat".to_string(), Value::Number(base_addr as f64));
    map.insert("ukuran_byte".to_string(), Value::Number(ukuran as f64));
    map.insert("alamat_basis".to_string(), Value::Number(base_addr as f64));
    map.insert("panjang_bytes".to_string(), Value::Number(ukuran as f64));
    map.insert("jenis_memori".to_string(), Value::String("SRAM_NO_GC_DIRECT_MMIO".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn register_io(env: &mut Environment) {
    register_fn(env, "cetak", None, builtin_cetak);
    register_fn(env, "tulis", None, builtin_tulis);
    register_fn(env, "baca", None, builtin_baca);

    register_fn(env, "baca_berkas", Some(1), builtin_baca_berkas);
    register_fn(env, "tulis_berkas", Some(2), builtin_tulis_berkas);
    register_fn(env, "tulis_biner", Some(2), builtin_tulis_biner);
    register_fn(env, "baca_biner", Some(1), builtin_baca_biner);
    register_fn(env, "eksekusi", Some(1), builtin_eksekusi);
    register_fn(env, "baca_env", Some(1), builtin_baca_env);
    register_fn(env, "tulis_env", Some(2), builtin_tulis_env);
    register_fn(env, "berkas_ada", Some(1), builtin_berkas_ada);
    register_fn(env, "hapus_berkas", Some(1), builtin_hapus_berkas);

    register_fn(env, "baca_volatil", Some(1), builtin_baca_volatil);
    register_fn(env, "tulis_volatil", Some(2), builtin_tulis_volatil);
    register_fn(env, "alokasi_buffer_mentah", Some(1), builtin_alokasi_buffer_mentah);
}
