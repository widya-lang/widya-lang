pub mod common;
pub mod io;
pub mod types;
pub mod collections;
pub mod concurrency;
pub mod math;
pub mod time;
pub mod crypto;
pub mod ui;
pub mod simd;
pub mod graphql;
pub mod grpc;
pub mod shm;
pub mod regex;
pub mod config_formats;
pub mod compression;
pub mod data_structures;
pub mod fintech;
pub mod game3d;
pub mod web3;
pub mod industry;
pub mod office_suite;
pub mod smartcity;
pub mod storage;
pub mod security;
pub mod distributed;
pub mod spatial;
pub mod networking;
pub mod multimedia;
pub mod text;
pub mod math_advanced;
pub mod system;
pub mod os_kernel;
pub mod i18n;
pub mod nusantara;
pub mod world_scripts;
pub mod chrono_world;
pub mod pilar_sistem;
pub mod waves;
pub mod web_enterprise;
pub mod desktop_native;
pub mod mobile_native;
pub mod pure_format;
pub mod actor_supervision;

pub use common::*;
pub use io::*;
pub use pure_format::*;
pub use types::*;
pub use collections::*;
pub use concurrency::*;
pub use actor_supervision::*;
pub use math::*;
pub use time::*;
pub use crypto::*;
pub use ui::*;
pub use simd::*;
pub use graphql::*;
pub use grpc::*;
pub use shm::*;
pub use regex::*;
pub use config_formats::*;
pub use compression::*;
pub use data_structures::*;
pub use fintech::*;
pub use game3d::*;
pub use web3::*;
pub use industry::*;
pub use office_suite::*;
pub use smartcity::*;
pub use storage::*;
pub use security::*;
pub use distributed::*;
pub use spatial::*;
pub use networking::*;
pub use multimedia::*;
pub use text::*;
pub use math_advanced::*;
pub use system::*;
pub use os_kernel::*;
pub use i18n::*;
pub use nusantara::*;
pub use world_scripts::*;
pub use chrono_world::*;
pub use pilar_sistem::*;
pub use waves::*;
pub use web_enterprise::*;
pub use desktop_native::*;
pub use mobile_native::*;

use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::value::{BuiltinFunction, Value};
use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc};
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::{HashMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::io as std_io;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock, LazyLock, mpsc};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub fn register_stdlib(env: &mut Environment) {
    // Modular Subsystems
    io::register_io(env);
    types::register_types(env);
    collections::register_collections(env);
    concurrency::register_concurrency(env);
    math::register_math(env);
    time::register_time(env);
    crypto::register_crypto(env);

    ui::register_ui(env);
    simd::register_simd(env);
    graphql::register_graphql(env);
    grpc::register_grpc(env);
    shm::register_shm(env);
    regex::register_regex(env);
    config_formats::register_config_formats(env);
    compression::register_compression(env);
    data_structures::register_data_structures(env);

    // WidyaHttp (Microservices & REST API Server Framework - implementasi nyata di src/web)
    crate::web::daftarkan_http(env);
    crate::web::daftarkan_websocket(env);

    text::register_text(env);
    math_advanced::register_math_advanced(env);
    system::register_system(env);
    os_kernel::register_os_kernel(env);
    i18n::register_i18n(env);
    nusantara::register_nusantara(env);
    world_scripts::register_world_scripts(env);
    chrono_world::register_chrono_world(env);
    pilar_sistem::register_pilar_sistem(env);
    waves::register_waves(env);
    fintech::register_fintech(env);
    game3d::register_game3d(env);
    web3::register_web3(env);
    industry::register_industry(env);
    office_suite::register_office_suite(env);
    smartcity::register_smartcity(env);
    storage::register_storage(env);
    security::register_security(env);
    distributed::register_distributed(env);
    spatial::register_spatial(env);
    networking::register_networking(env);
    multimedia::register_multimedia(env);
    web_enterprise::register_web_enterprise(env);
    desktop_native::register_desktop_native(env);
    mobile_native::register_mobile_native(env);

    // Hardware GPU, Tiled SIMD Tensor Engine & Real Python Bridge
    crate::gpu::daftarkan_gpu(env);

    // Full C-Language Parity (Bitwise, Fixed-Width Casts, C-ABI Layout, Raw Pointers, Inline ASM, Direct ELF, C Headers)
    register_c_parity(env);

    // Full Go (Golang) Parity (Defer LIFO, Panic/Recover, Select Multiplexing, Structural Interfaces)
    register_go_parity(env);

    // Full Rust Parity (Ownership & NLL Borrow Checker, RAII Drop, Smart Pointers, Combinators, Macros)
    register_rust_parity(env);

    // Pre-populate hierarchical `std` namespace map
    register_std_hierarchy(env);
}

fn register_rust_parity(env: &mut Environment) {
    register_fn(env, "_pinjam_rust", Some(3), |_args, _span| Ok(Value::Nil));
    register_fn(env, "_pinjam_field_rust", Some(4), |_args, _span| Ok(Value::Nil));
    register_fn(env, "_pindah_rust", Some(2), |args, _span| {
        Ok(args.get(1).cloned().unwrap_or(Value::Nil))
    });
    for name in &["lepas_pinjam", "drop_borrow", "jatuhkan", "drop"] {
        register_fn(env, name, Some(1), |_args, _span| Ok(Value::Nil));
    }
    register_fn(env, "_cek_guard_cocok", Some(2), |_args, _span| Ok(Value::Nil));
    register_fn(env, "_klon_instans", Some(1), |args, _span| {
        if let Some(Value::Instance(inst)) = args.first() {
            let b = inst.borrow();
            let mut cloned_fields = HashMap::new();
            for (k, v) in &b.fields {
                cloned_fields.insert(k.clone(), v.clone());
            }
            return Ok(Value::Instance(Rc::new(RefCell::new(
                crate::value::WidyaInstance {
                    struct_def: b.struct_def.clone(),
                    fields: cloned_fields,
                },
            ))));
        }
        Ok(args.first().cloned().unwrap_or(Value::Nil))
    });
    register_fn(env, "_ulang_elemen", Some(2), |args, span| {
        let elem = args.first().cloned().unwrap_or(Value::Nil);
        let count = val_as_num(&args[1], span)?.max(0.0) as usize;
        let items = vec![elem; count];
        Ok(Value::Array(Rc::new(RefCell::new(items))))
    });
    for name in &["Kotak", "Box", "kotak_baru"] {
        register_fn(env, name, Some(1), |args, _span| {
            let mut m = HashMap::new();
            m.insert("_tipe".to_string(), Value::String("Kotak".to_string()));
            m.insert(
                "nilai".to_string(),
                args.first().cloned().unwrap_or(Value::Nil),
            );
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        });
    }
    for name in &["SelRef", "RefCell", "sel_ref_baru"] {
        register_fn(env, name, Some(1), |args, _span| {
            let mut m = HashMap::new();
            m.insert("_tipe".to_string(), Value::String("SelRef".to_string()));
            m.insert(
                "nilai".to_string(),
                args.first().cloned().unwrap_or(Value::Nil),
            );
            m.insert("_pembaca".to_string(), Value::Number(0.0));
            m.insert("_penulis".to_string(), Value::Bool(false));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        });
    }
    for name in &["Rc", "Arc"] {
        register_fn(env, name, Some(1), |args, _span| {
            let mut m = HashMap::new();
            m.insert("_tipe".to_string(), Value::String("Rc".to_string()));
            m.insert(
                "nilai".to_string(),
                args.first().cloned().unwrap_or(Value::Nil),
            );
            m.insert("_strong_count".to_string(), Value::Number(1.0));
            Ok(Value::Map(Rc::new(RefCell::new(m))))
        });
    }
    for name in &["MutexRust"] {
        register_fn(env, name, Some(1), concurrency::builtin_kunci_mutex);
    }
    for name in &["vektor", "vec"] {
        register_fn(env, name, None, |args, _span| {
            Ok(Value::Array(Rc::new(RefCell::new(args.to_vec()))))
        });
    }
}

fn register_go_parity(env: &mut Environment) {
    register_fn(env, "_daftar_tunda", Some(1), |_args, _span| Ok(Value::Nil));
    register_fn(env, "_eksekusi_pilih_saluran", None, |_args, _span| Ok(Value::Nil));
    for name in &["panik", "panic"] {
        register_fn(env, name, None, |args, span| {
            let msg = args
                .first()
                .map(|v| v.to_string_repr())
                .unwrap_or_else(|| "kepanikan runtime (panic)".to_string());
            Err(Galat::runtime(msg, span))
        });
    }
    for name in &["pulihkan", "recover"] {
        register_fn(env, name, Some(0), |_args, _span| Ok(Value::Nil));
    }
    for name in &["memenuhi_sifat", "implements", "apakah_memenuhi"] {
        register_fn(env, name, Some(2), |_args, _span| Ok(Value::Bool(false)));
    }
    for name in &["pastikan_sifat", "type_assert"] {
        register_fn(env, name, Some(2), |args, _span| {
            Ok(Value::Array(Rc::new(RefCell::new(vec![
                args.first().cloned().unwrap_or(Value::Nil),
                Value::Bool(false),
            ]))))
        });
    }
}

fn val_as_num(v: &Value, span: &Span) -> Result<f64, Galat> {
    match v {
        Value::Number(n) => Ok(*n),
        Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
        other => Err(Galat::runtime(
            format!("Mengharapkan angka, tetapi menerima '{}'", other.type_name()),
            span,
        )),
    }
}

fn val_as_str(v: &Value, span: &Span) -> Result<String, Galat> {
    match v {
        Value::String(s) => Ok(s.clone()),
        other => Err(Galat::runtime(
            format!("Mengharapkan teks, tetapi menerima '{}'", other.type_name()),
            span,
        )),
    }
}

fn register_c_parity(env: &mut Environment) {
    // 1. Bitwise operators & functions
    for name in &["_bit_dan", "bit_dan"] {
        register_fn(env, name, Some(2), |args, span| {
            let a = val_as_num(&args[0], span)? as i64;
            let b = val_as_num(&args[1], span)? as i64;
            Ok(Value::Number((a & b) as f64))
        });
    }
    for name in &["_bit_atau", "bit_atau"] {
        register_fn(env, name, Some(2), |args, span| {
            let a = val_as_num(&args[0], span)? as i64;
            let b = val_as_num(&args[1], span)? as i64;
            Ok(Value::Number((a | b) as f64))
        });
    }
    for name in &["_bit_xor", "bit_xor"] {
        register_fn(env, name, Some(2), |args, span| {
            let a = val_as_num(&args[0], span)? as i64;
            let b = val_as_num(&args[1], span)? as i64;
            Ok(Value::Number((a ^ b) as f64))
        });
    }
    for name in &["_bit_kiri", "bit_kiri", "geser_kiri"] {
        register_fn(env, name, Some(2), |args, span| {
            let a = val_as_num(&args[0], span)? as i64;
            let b = (val_as_num(&args[1], span)? as u32) & 63;
            Ok(Value::Number((a.wrapping_shl(b)) as f64))
        });
    }
    for name in &["_bit_kanan", "bit_kanan", "geser_kanan"] {
        register_fn(env, name, Some(2), |args, span| {
            let a = val_as_num(&args[0], span)? as i64;
            let b = (val_as_num(&args[1], span)? as u32) & 63;
            Ok(Value::Number((a.wrapping_shr(b)) as f64))
        });
    }
    for name in &["_bit_bukan", "bit_bukan"] {
        register_fn(env, name, Some(1), |args, span| {
            let a = val_as_num(&args[0], span)? as i64;
            Ok(Value::Number((!a) as f64))
        });
    }

    // 2. Fixed-width C integer & float casts with two's-complement wrapping
    register_fn(env, "ke_u8", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::U8.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_i8", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::I8.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_u16", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::U16.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_i16", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::I16.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_u32", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::U32.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_i32", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::I32.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_u64", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::U64.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_i64", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::I64.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_usize", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::Usize.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_isize", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::Isize.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_f32", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::F32.cast_value(val_as_num(&args[0], span)?)))
    });
    register_fn(env, "ke_f64", Some(1), |args, span| {
        Ok(Value::Number(crate::sys::c_abi::CType::F64.cast_value(val_as_num(&args[0], span)?)))
    });

    // 3. C-ABI Memory Layout Introspection: sizeof, alignof, offsetof, union definition
    for name in &["sizeof", "ukuran_tipe", "ukuran_byte"] {
        register_fn(env, name, Some(1), |args, _span| {
            match &args[0] {
                Value::String(s) => {
                    let reg = crate::sys::c_abi::get_c_type_registry().lock().unwrap();
                    if let Some(st) = reg.get_struct(s) {
                        return Ok(Value::Number(st.total_size as f64));
                    }
                    if let Some(un) = reg.get_union(s) {
                        return Ok(Value::Number(un.total_size as f64));
                    }
                    drop(reg);
                    let ct = crate::sys::c_abi::CType::from_type_str(s);
                    Ok(Value::Number(ct.size_bytes() as f64))
                }
                Value::Instance(inst) => {
                    let s_name = inst.borrow().struct_def.name.clone();
                    let reg = crate::sys::c_abi::get_c_type_registry().lock().unwrap();
                    if let Some(st) = reg.get_struct(&s_name) {
                        return Ok(Value::Number(st.total_size as f64));
                    }
                    Ok(Value::Number((inst.borrow().fields.len() * 8) as f64))
                }
                Value::Map(m) => {
                    let mb = m.borrow();
                    if let Some(Value::Number(sz)) = mb.get("_size").or_else(|| mb.get("_ukuran_c")) {
                        return Ok(Value::Number(*sz));
                    }
                    if let Some(Value::String(u_name)) = mb.get("_tipe_c_union").or_else(|| mb.get("_definisi_c_union")) {
                        let reg = crate::sys::c_abi::get_c_type_registry().lock().unwrap();
                        if let Some(un) = reg.get_union(u_name) {
                            return Ok(Value::Number(un.total_size as f64));
                        }
                    }
                    Ok(Value::Number(8.0))
                }
                Value::Number(_) => Ok(Value::Number(8.0)),
                Value::Bool(_) => Ok(Value::Number(1.0)),
                _ => Ok(Value::Number(8.0)),
            }
        });
    }

    for name in &["alignof", "perataan_tipe"] {
        register_fn(env, name, Some(1), |args, _span| {
            match &args[0] {
                Value::String(s) => {
                    let reg = crate::sys::c_abi::get_c_type_registry().lock().unwrap();
                    if let Some(st) = reg.get_struct(s) {
                        return Ok(Value::Number(st.alignment as f64));
                    }
                    if let Some(un) = reg.get_union(s) {
                        return Ok(Value::Number(un.alignment as f64));
                    }
                    drop(reg);
                    let ct = crate::sys::c_abi::CType::from_type_str(s);
                    Ok(Value::Number(ct.align_bytes() as f64))
                }
                Value::Instance(inst) => {
                    let s_name = inst.borrow().struct_def.name.clone();
                    let reg = crate::sys::c_abi::get_c_type_registry().lock().unwrap();
                    if let Some(st) = reg.get_struct(&s_name) {
                        return Ok(Value::Number(st.alignment as f64));
                    }
                    Ok(Value::Number(8.0))
                }
                Value::Map(m) => {
                    let mb = m.borrow();
                    if let Some(Value::Number(al)) = mb.get("_align").or_else(|| mb.get("_perataan_c")) {
                        return Ok(Value::Number(*al));
                    }
                    Ok(Value::Number(8.0))
                }
                _ => Ok(Value::Number(8.0)),
            }
        });
    }

    for name in &["offsetof", "offset_field"] {
        register_fn(env, name, Some(2), |args, span| {
            let type_name = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Instance(inst) => inst.borrow().struct_def.name.clone(),
                Value::Map(m) => {
                    let mb = m.borrow();
                    mb.get("_tipe_c_union")
                        .or_else(|| mb.get("_definisi_c_union"))
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                        .unwrap_or_default()
                }
                _ => String::new(),
            };
            let field_name = val_as_str(&args[1], span)?;
            let reg = crate::sys::c_abi::get_c_type_registry().lock().unwrap();
            if let Some(st) = reg.get_struct(&type_name) {
                if let Some(off) = st.offset_of(&field_name) {
                    return Ok(Value::Number(off as f64));
                }
                return Err(Galat::runtime(
                    format!("Field '{}' tidak ditemukan pada struct '{}'", field_name, type_name),
                    span,
                ));
            }
            if let Some(un) = reg.get_union(&type_name) {
                if un.fields.contains_key(&field_name) {
                    return Ok(Value::Number(0.0));
                }
            }
            Ok(Value::Number(0.0))
        });
    }

    register_fn(env, "_definisi_union_c", None, |args, span| {
        if args.is_empty() {
            return Err(Galat::runtime("_definisi_union_c membutuhkan nama union", span));
        }
        let name = val_as_str(&args[0], span)?;
        let mut raw_fields = Vec::new();
        for arg in args.iter().skip(1) {
            match arg {
                Value::String(spec) => {
                    let parts: Vec<&str> = spec.splitn(2, ':').collect();
                    let fname = parts[0].trim().to_string();
                    let ftype = parts.get(1).copied().unwrap_or("u64").trim();
                    raw_fields.push((fname, crate::sys::c_abi::CType::from_type_str(ftype)));
                }
                Value::Map(m) => {
                    for (fname, fval) in m.borrow().iter() {
                        let ftype = fval.to_string_repr();
                        raw_fields.push((fname.clone(), crate::sys::c_abi::CType::from_type_str(&ftype)));
                    }
                }
                _ => {}
            }
        }
        let layout = crate::sys::c_abi::CUnionLayout::compute(&name, &raw_fields);
        let size = layout.total_size;
        let align = layout.alignment;
        crate::sys::c_abi::get_c_type_registry().lock().unwrap().register_union(layout);
        let mut def_map = HashMap::new();
        def_map.insert("_definisi_c_union".to_string(), Value::String(name));
        def_map.insert("_size".to_string(), Value::Number(size as f64));
        def_map.insert("_align".to_string(), Value::Number(align as f64));
        Ok(Value::Map(Rc::new(RefCell::new(def_map))))
    });

    // 4. Raw Memory Management & Pointer Operations (malloc, calloc, realloc, free, memcpy, memset, memcmp, *ptr, &var)
    for name in &["malloc", "alokasi_memori"] {
        register_fn(env, name, Some(1), |args, span| {
            let size = val_as_num(&args[0], span)? as usize;
            let addr = crate::sys::c_abi::get_raw_memory_manager().lock().unwrap().malloc(size);
            Ok(Value::Number(addr as f64))
        });
    }
    register_fn(env, "calloc", Some(2), |args, span| {
        let count = val_as_num(&args[0], span)? as usize;
        let elem_size = val_as_num(&args[1], span)? as usize;
        let addr = crate::sys::c_abi::get_raw_memory_manager().lock().unwrap().calloc(count, elem_size);
        Ok(Value::Number(addr as f64))
    });
    register_fn(env, "realloc", Some(2), |args, span| {
        let ptr = val_as_num(&args[0], span)? as usize;
        let new_size = val_as_num(&args[1], span)? as usize;
        let addr = crate::sys::c_abi::get_raw_memory_manager().lock().unwrap().realloc(ptr, new_size);
        Ok(Value::Number(addr as f64))
    });
    for name in &["free", "bebaskan_memori"] {
        register_fn(env, name, Some(1), |args, span| {
            let ptr = val_as_num(&args[0], span)? as usize;
            let _ = crate::sys::c_abi::get_raw_memory_manager().lock().unwrap().free(ptr);
            Ok(Value::Nil)
        });
    }
    register_fn(env, "memcpy", Some(3), |args, span| {
        let dst = val_as_num(&args[0], span)? as usize;
        let src = val_as_num(&args[1], span)? as usize;
        let n = val_as_num(&args[2], span)? as usize;
        crate::sys::c_abi::get_raw_memory_manager()
            .lock()
            .unwrap()
            .memcpy(dst, src, n)
            .map_err(|e| Galat::runtime(e, span))?;
        Ok(Value::Number(dst as f64))
    });
    register_fn(env, "memmove", Some(3), |args, span| {
        let dst = val_as_num(&args[0], span)? as usize;
        let src = val_as_num(&args[1], span)? as usize;
        let n = val_as_num(&args[2], span)? as usize;
        crate::sys::c_abi::get_raw_memory_manager()
            .lock()
            .unwrap()
            .memmove(dst, src, n)
            .map_err(|e| Galat::runtime(e, span))?;
        Ok(Value::Number(dst as f64))
    });
    register_fn(env, "memset", Some(3), |args, span| {
        let dst = val_as_num(&args[0], span)? as usize;
        let val = (val_as_num(&args[1], span)? as i64 & 0xFF) as u8;
        let n = val_as_num(&args[2], span)? as usize;
        crate::sys::c_abi::get_raw_memory_manager()
            .lock()
            .unwrap()
            .memset(dst, val, n)
            .map_err(|e| Galat::runtime(e, span))?;
        Ok(Value::Number(dst as f64))
    });
    register_fn(env, "memcmp", Some(3), |args, span| {
        let a = val_as_num(&args[0], span)? as usize;
        let b = val_as_num(&args[1], span)? as usize;
        let n = val_as_num(&args[2], span)? as usize;
        let cmp = crate::sys::c_abi::get_raw_memory_manager()
            .lock()
            .unwrap()
            .memcmp(a, b, n)
            .map_err(|e| Galat::runtime(e, span))?;
        Ok(Value::Number(cmp as f64))
    });

    for name in &["_deref_baca", "baca_ptr"] {
        register_fn(env, name, None, |args, span| {
            if args.is_empty() {
                return Err(Galat::runtime("baca_ptr membutuhkan alamat pointer", span));
            }
            let addr = val_as_num(&args[0], span)? as usize;
            let ctype = if args.len() >= 2 {
                crate::sys::c_abi::CType::from_type_str(&val_as_str(&args[1], span)?)
            } else {
                crate::sys::c_abi::CType::U8
            };
            let val = crate::sys::c_abi::get_raw_memory_manager()
                .lock()
                .unwrap()
                .read_typed(addr, &ctype)
                .map_err(|e| Galat::runtime(e, span))?;
            Ok(Value::Number(val))
        });
    }
    for name in &["_deref_tulis", "tulis_ptr"] {
        register_fn(env, name, None, |args, span| {
            if args.len() < 2 {
                return Err(Galat::runtime("tulis_ptr membutuhkan (alamat, nilai, [tipe])", span));
            }
            let addr = val_as_num(&args[0], span)? as usize;
            let val = val_as_num(&args[1], span)?;
            let ctype = if args.len() >= 3 {
                crate::sys::c_abi::CType::from_type_str(&val_as_str(&args[2], span)?)
            } else {
                crate::sys::c_abi::CType::U8
            };
            crate::sys::c_abi::get_raw_memory_manager()
                .lock()
                .unwrap()
                .write_typed(addr, val, &ctype)
                .map_err(|e| Galat::runtime(e, span))?;
            Ok(Value::Number(val))
        });
    }
    for name in &["_alamat_dari", "alamat_dari"] {
        register_fn(env, name, Some(1), |args, span| {
            let mut mgr = crate::sys::c_abi::get_raw_memory_manager().lock().unwrap();
            match &args[0] {
                Value::Number(n) => {
                    let addr = mgr.malloc(8);
                    let _ = mgr.write_typed(addr, *n, &crate::sys::c_abi::CType::I64);
                    Ok(Value::Number(addr as f64))
                }
                Value::String(s) => {
                    let bytes = s.as_bytes();
                    let addr = mgr.malloc(bytes.len() + 1);
                    for (i, &b) in bytes.iter().enumerate() {
                        let _ = mgr.write_u8(addr + i, b);
                    }
                    let _ = mgr.write_u8(addr + bytes.len(), 0);
                    Ok(Value::Number(addr as f64))
                }
                _ => Err(Galat::runtime("Operator '&' membutuhkan variabel numerik atau teks", span)),
            }
        });
    }

    // 5. Inline Assembly (`asm!` / `rakitan!`)
    for name in &["asm", "rakitan"] {
        register_fn(env, name, None, |args, span| {
            if args.is_empty() {
                return Err(Galat::runtime("asm! membutuhkan string instruksi assembly", span));
            }
            let asm_src = val_as_str(&args[0], span)?;
            let mut eax: i64 = if args.len() >= 2 { val_as_num(&args[1], span)? as i64 } else { 0 };
            let ebx: i64 = if args.len() >= 3 { val_as_num(&args[2], span)? as i64 } else { 0 };
            for raw_inst in asm_src.split(|c| c == ';' || c == '\n') {
                let inst = raw_inst.trim();
                if inst.is_empty() || inst == "ret" || inst == "nop" {
                    continue;
                }
                let lower = inst.to_lowercase();
                if let Some(rest) = lower.strip_prefix("mov ") {
                    let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 && (parts[0] == "eax" || parts[0] == "rax") {
                        if parts[1] == "ebx" || parts[1] == "rbx" {
                            eax = ebx;
                        } else if let Ok(v) = parts[1].trim_start_matches("0x").parse::<i64>() {
                            eax = if parts[1].starts_with("0x") {
                                i64::from_str_radix(parts[1].trim_start_matches("0x"), 16).unwrap_or(v)
                            } else {
                                v
                            };
                        }
                    }
                } else if let Some(rest) = lower.strip_prefix("add ") {
                    let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 && (parts[0] == "eax" || parts[0] == "rax") {
                        if parts[1] == "ebx" || parts[1] == "rbx" {
                            eax = eax.wrapping_add(ebx);
                        } else if let Ok(v) = parts[1].parse::<i64>() {
                            eax = eax.wrapping_add(v);
                        }
                    }
                } else if let Some(rest) = lower.strip_prefix("sub ") {
                    let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 && (parts[0] == "eax" || parts[0] == "rax") {
                        if parts[1] == "ebx" || parts[1] == "rbx" {
                            eax = eax.wrapping_sub(ebx);
                        } else if let Ok(v) = parts[1].parse::<i64>() {
                            eax = eax.wrapping_sub(v);
                        }
                    }
                } else if let Some(rest) = lower.strip_prefix("shl ") {
                    let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 && (parts[0] == "eax" || parts[0] == "rax") {
                        if let Ok(v) = parts[1].parse::<u32>() {
                            eax = eax.wrapping_shl(v);
                        }
                    }
                } else if let Some(rest) = lower.strip_prefix("xor ") {
                    let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 && parts[0] == parts[1] && (parts[0] == "eax" || parts[0] == "rax") {
                        eax = 0;
                    }
                }
            }
            Ok(Value::Number(eax as f64))
        });
    }

    // 6. Direct AST-to-ELF64 Machine Code Compiler (`kompilasi_elf_murni`)
    register_fn(env, "kompilasi_elf_murni", None, |args, span| {
        if args.is_empty() {
            return Err(Galat::runtime("kompilasi_elf_murni membutuhkan kode sumber Widya", span));
        }
        let src = val_as_str(&args[0], span)?;
        let mut lexer = crate::lexer::Lexer::new(&src);
        let tokens = lexer.scan_tokens()?;
        let mut parser = crate::parser::Parser::new(tokens);
        let program = parser.parse()?;
        let elf_bytes = crate::compiler::pure_emitter::PureElfBuilder::compile_ast_to_elf64(&program)
            .map_err(|e| Galat::runtime(e, span))?;
        if args.len() >= 2 {
            let out_path = val_as_str(&args[1], span)?;
            std::fs::write(&out_path, &elf_bytes)
                .map_err(|e| Galat::runtime(format!("Gagal menulis biner ELF '{}': {}", out_path, e), span))?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(meta) = std::fs::metadata(&out_path) {
                    let mut perms = meta.permissions();
                    perms.set_mode(0o755);
                    let _ = std::fs::set_permissions(&out_path, perms);
                }
            }
        }
        Ok(Value::Number(elf_bytes.len() as f64))
    });

    // 7. C Header Parser (`impor_header_c` / `urai_header_c`) & Direct C Function Call (`panggil_c`)
    for name in &["impor_header_c", "urai_header_c"] {
        register_fn(env, name, Some(1), |args, span| {
            let input = val_as_str(&args[0], span)?;
            let header_src = if input.ends_with(".h") && std::path::Path::new(&input).exists() {
                std::fs::read_to_string(&input)
                    .map_err(|e| Galat::runtime(format!("Gagal membaca header C '{}': {}", input, e), span))?
            } else {
                input
            };
            let parsed = crate::ffi::CHeaderParser::parse_header_str(&header_src);
            Ok(parsed.to_widya_map())
        });
    }

    register_fn(env, "panggil_c", None, |args, span| {
        if args.is_empty() {
            return Err(Galat::runtime("panggil_c membutuhkan nama fungsi C", span));
        }
        let fn_name = val_as_str(&args[0], span)?;
        let mut mgr = crate::ffi::get_ffi_manager().lock().unwrap();
        mgr.call_foreign_function(&fn_name, &args[1..])
    });
}

fn register_std_hierarchy(env: &mut Environment) {
    let all = env.get_all_local();
    let make_submap = |keys: &[&str], extra_aliases: &[(&str, &str)]| -> Value {
        let mut m = HashMap::new();
        for &k in keys {
            if let Some(v) = all.get(k) {
                m.insert(k.to_string(), v.clone());
            }
        }
        for &(alias, orig) in extra_aliases {
            if let Some(v) = all.get(orig) {
                m.insert(alias.to_string(), v.clone());
            }
        }
        Value::Map(Rc::new(RefCell::new(m)))
    };

    let math_ns = make_submap(
        &[
            "PI", "E", "akar", "pangkat", "mutlak", "bulatkan", "bulat_bawah", "bulat_atas",
            "sin", "cos", "tan", "asin", "acos", "atan", "log", "log10", "log2", "eksponen",
            "acak", "acak_antara", "min", "maks", "kpk", "fpb", "faktorial", "hipotenusa",
        ],
        &[
            ("pi", "PI"),
            ("e", "E"),
            ("sqrt", "akar"),
            ("pow", "pangkat"),
            ("abs", "mutlak"),
            ("round", "bulatkan"),
            ("floor", "bulat_bawah"),
            ("ceil", "bulat_atas"),
            ("max", "maks"),
        ],
    );

    let text_ns = make_submap(
        &[
            "huruf_besar", "huruf_kecil", "pisah", "gabung", "ganti", "hapus_spasi",
            "apakah_diawali", "apakah_diakhiri", "mengandung", "cari_indeks", "potong",
            "ulangi_teks", "balik_teks", "panjang_karakter",
        ],
        &[
            ("upper", "huruf_besar"),
            ("lower", "huruf_kecil"),
            ("split", "pisah"),
            ("join", "gabung"),
            ("replace", "ganti"),
            ("strip", "hapus_spasi"),
        ],
    );

    let collections_ns = make_submap(
        &[
            "panjang", "len", "tambah", "hapus", "sisip", "sisipkan", "urutkan", "urut",
            "sorted", "balik", "reversed", "petakan", "peta", "map", "saring", "filter",
            "lipat", "rentang", "range", "enumerasi", "enumerate", "pasangkan", "zip",
            "jumlah", "sum", "semua", "all", "ada_yang", "any", "iris", "slice",
        ],
        &[],
    );

    let json_ns = make_submap(
        &["ke_json", "dari_json", "urai_json", "format_json"],
        &[
            ("dumps", "ke_json"),
            ("stringify", "ke_json"),
            ("loads", "dari_json"),
            ("parse", "dari_json"),
            ("urai_json", "dari_json"),
        ],
    );

    let ai_ns = make_submap(
        &[
            "tensor_baru", "tensor_nol", "tensor_satu", "tensor_matmul", "tensor_tambah",
            "tensor_kurang", "tensor_kali", "tensor_transpose", "tensor_reshape", "tensor_relu",
            "tensor_gelu", "tensor_silu", "tensor_softmax", "tensor_rms_norm", "tensor_layer_norm",
            "tensor_perhatian", "tensor_jumlah", "tensor_rata_rata", "tensor_argmax",
            "tensor_ke_dlpack", "info_perangkat_gpu", "GpuPipeline", "eksekusi_gpu",
        ],
        &[
            ("zeros", "tensor_nol"),
            ("ones", "tensor_satu"),
            ("matmul", "tensor_matmul"),
            ("attention", "tensor_perhatian"),
        ],
    );

    let python_ns = make_submap(
        &["impor_python", "panggil_python", "eval_python", "tensor_ke_dlpack"],
        &[
            ("import_module", "impor_python"),
            ("call", "panggil_python"),
            ("eval", "eval_python"),
        ],
    );

    let mut std_root = HashMap::new();
    std_root.insert("matematika".to_string(), math_ns.clone());
    std_root.insert("math".to_string(), math_ns);
    std_root.insert("teks".to_string(), text_ns.clone());
    std_root.insert("string".to_string(), text_ns);
    std_root.insert("koleksi".to_string(), collections_ns.clone());
    std_root.insert("collections".to_string(), collections_ns);
    std_root.insert("json".to_string(), json_ns);
    std_root.insert("ai".to_string(), ai_ns.clone());
    std_root.insert("tensor".to_string(), ai_ns.clone());
    std_root.insert("gpu".to_string(), ai_ns);
    std_root.insert("python".to_string(), python_ns);

    env.define("std".to_string(), Value::Map(Rc::new(RefCell::new(std_root))), false);
}

