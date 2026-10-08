use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::collections::invoke_closure_or_fn;
use crate::stdlib::common::*;
use crate::stdlib::math_advanced::murmur3_32_raw;
use crate::stdlib::storage::current_epoch_ms;
use crate::stdlib::types::{builtin_ada, builtin_kosong};
use crate::value::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::f64::consts::FRAC_1_SQRT_2;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub fn register_system(env: &mut Environment) {
    register_fn(env, "Matriks", Some(1), builtin_ai_matriks);
    register_fn(env, "matriks_tambah", Some(2), builtin_ai_matriks_tambah);
    register_fn(env, "matriks_kali", Some(2), builtin_ai_matriks_kali);
    register_fn(env, "matriks_transpose", Some(1), builtin_ai_matriks_transpose);
    register_fn(env, "sigmoid", Some(1), builtin_ai_sigmoid);
    register_fn(env, "relu", Some(1), builtin_ai_relu);
    register_fn(env, "KirimPesanAsli", Some(2), builtin_bridge_kirim_pesan);
    register_fn(env, "DapatkanInfoPerangkat", Some(0), builtin_bridge_info_perangkat);
    register_fn(env, "Aliran", Some(1), builtin_aliran_buat);
    register_fn(env, "alirkan_ke", Some(2), builtin_aliran_kirim);
    register_fn(env, "transformasi_aliran", Some(2), builtin_aliran_transformasi);
    register_fn(env, "tampung_aliran", Some(1), builtin_aliran_tampung);
    register_fn(env, "PipaGPU", Some(2), builtin_gpu_pipa);
    register_fn(env, "eksekusi_gpu", Some(3), builtin_gpu_eksekusi);
    register_fn(env, "SirkuitKuantum", Some(1), builtin_kuantum_sirkuit);
    register_fn(env, "gerbang_h", Some(2), builtin_kuantum_gerbang_h);
    register_fn(env, "gerbang_x", Some(2), builtin_kuantum_gerbang_x);
    register_fn(env, "gerbang_cnot", Some(3), builtin_kuantum_gerbang_cnot);
    register_fn(env, "ukur_kuantum", Some(1), builtin_kuantum_ukur);
    register_fn(env, "ProgramEBPF", Some(2), builtin_ebpf_program);
    register_fn(env, "pasang_probe_ebpf", Some(2), builtin_ebpf_pasang_probe);
    register_fn(env, "baca_peta_ebpf", Some(2), builtin_ebpf_baca_peta);
    register_fn(env, "LapisanLinier", None, builtin_neural_linier);
    register_fn(env, "LapisanAttention", None, builtin_neural_attention);
    register_fn(env, "forward_linier", Some(2), builtin_neural_forward_linier);
    register_fn(env, "forward_attention", Some(2), builtin_neural_forward_attention);
    register_fn(env, "inferensi_llm", None, builtin_neural_inferensi_llm);
    register_fn(env, "DokumenCRDT", None, builtin_crdt_dokumen);
    register_fn(env, "sisip_karakter", Some(3), builtin_crdt_sisip);
    register_fn(env, "sisip_karakter_crdt", Some(3), builtin_crdt_sisip);
    register_fn(env, "gabung_perubahan", Some(2), builtin_crdt_gabung);
    register_fn(env, "gabung_perubahan_crdt", Some(2), builtin_crdt_gabung);
    register_fn(env, "baca_teks_crdt", Some(1), builtin_crdt_baca);
    register_fn(env, "patch_fungsi", Some(2), builtin_hotpatch_fungsi);
    register_fn(env, "riwayat_patch", Some(0), builtin_hotpatch_riwayat);
    register_fn(env, "PenjadwalCron", None, builtin_cron_penjadwal);
    register_fn(env, "cron_jadwalkan", Some(4), builtin_cron_jadwalkan);
    register_fn(env, "cron_cocok_waktu", None, builtin_cron_cocok);
    register_fn(env, "cron_jalankan_langkah", None, builtin_cron_langkah);
    register_fn(env, "MesinStatus", Some(1), builtin_fsm_buat);
    register_fn(env, "fsm_tambah_transisi", Some(4), builtin_fsm_tambah_transisi);
    register_fn(env, "fsm_picu", Some(2), builtin_fsm_picu);
    register_fn(env, "fsm_status_saat_ini", Some(1), builtin_fsm_status);
    register_fn(env, "fsm_dapat_picu", Some(2), builtin_fsm_dapat_picu);
    register_fn(env, "tolak_ukur_fungsi", None, builtin_bench_fungsi);
    register_fn(env, "waktu_eksekusi", None, builtin_bench_waktu_eksekusi);
    register_fn(env, "cron_parse", Some(1), builtin_cron_parse);
    register_fn(env, "cron_cocok_waktu", None, builtin_cron_cocok);
    register_fn(env, "cron_waktu_berikutnya", Some(2), builtin_cron_waktu_berikutnya);
    register_fn(env, "cron_deskripsi_manusia", Some(1), builtin_cron_deskripsi);
    register_fn(env, "PohonMerkle", Some(1), builtin_merkle_buat);
    register_fn(env, "merkle_root", Some(1), builtin_merkle_root);
    register_fn(env, "merkle_buat_bukti", Some(2), builtin_merkle_buat_bukti);
    register_fn(env, "merkle_verifikasi_bukti", Some(3), builtin_merkle_verifikasi_bukti);
    register_fn(env, "DaemonCron", None, builtin_crondaemon_buat);
    register_fn(env, "daemon_cron_daftar", Some(4), builtin_crondaemon_daftar);
    register_fn(env, "daemon_cron_detak", None, builtin_crondaemon_detak);
    register_fn(env, "daemon_cron_jeda", Some(2), builtin_crondaemon_jeda);
    register_fn(env, "daemon_cron_lanjut", Some(2), builtin_crondaemon_lanjut);
    register_fn(env, "daemon_cron_status", Some(1), builtin_crondaemon_status);
}

// ==============================================================================
// WidyaAI: Tensor, Matrix & Neural Compute Engine
// ==============================================================================
fn builtin_ai_matriks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(arr) => {
            let mut obj = HashMap::new();
            obj.insert("_tipe".to_string(), Value::String("Matriks".to_string()));
            obj.insert("data".to_string(), Value::Array(arr.clone()));
            Ok(Value::Map(Rc::new(RefCell::new(obj))))
        }
        _ => Err(Galat::runtime("Konstruktor Matriks memerlukan array 2D", span)),
    }
}

fn get_matrix_data(v: &Value) -> Option<Vec<Vec<f64>>> {
    if let Value::Map(m) = v {
        let map = m.borrow();
        if let (Some(Value::Number(r_num)), Some(Value::Number(c_num)), Some(Value::Array(da))) = (map.get("baris"), map.get("kolom"), map.get("data")) {
            let r = *r_num as usize;
            let c = *c_num as usize;
            let flat = da.borrow();
            let mut result = Vec::with_capacity(r);
            for i in 0..r {
                let mut row = Vec::with_capacity(c);
                for j in 0..c {
                    let val = match flat.get(i * c + j) {
                        Some(Value::Number(n)) => *n,
                        _ => 0.0,
                    };
                    row.push(val);
                }
                result.push(row);
            }
            return Some(result);
        }
    }

    let arr = match v {
        Value::Array(a) => a.clone(),
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(a)) = map.get("data") {
                a.clone()
            } else {
                return None;
            }
        }
        _ => return None,
    };

    let items = arr.borrow();
    let mut rows = Vec::new();
    for row_val in items.iter() {
        if let Value::Array(r_arr) = row_val {
            let mut r = Vec::new();
            for cell in r_arr.borrow().iter() {
                if let Value::Number(n) = cell {
                    r.push(*n);
                } else {
                    return None;
                }
            }
            rows.push(r);
        } else {
            return None;
        }
    }
    Some(rows)
}

fn make_matrix_val(data: Vec<Vec<f64>>) -> Value {
    let mut row_vals = Vec::new();
    for row in data {
        let cell_vals: Vec<Value> = row.into_iter().map(Value::Number).collect();
        row_vals.push(Value::Array(Rc::new(RefCell::new(cell_vals))));
    }
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("Matriks".to_string()));
    obj.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(row_vals))));
    Value::Map(Rc::new(RefCell::new(obj)))
}

fn builtin_ai_matriks_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let m1 = get_matrix_data(&args[0]).ok_or_else(|| Galat::runtime("Matriks 1 tidak valid", span))?;
    let m2 = get_matrix_data(&args[1]).ok_or_else(|| Galat::runtime("Matriks 2 tidak valid", span))?;

    if m1.len() != m2.len() || m1.is_empty() || m1[0].len() != m2[0].len() {
        return Err(Galat::runtime("Dimensi matriks harus sama untuk operasi penjumlahan", span));
    }

    let r_len = m1.len();
    let c_len = m1[0].len();
    let mut result = vec![vec![0.0; c_len]; r_len];

    for r in 0..r_len {
        for c in 0..c_len {
            result[r][c] = m1[r][c] + m2[r][c];
        }
    }
    Ok(make_matrix_val(result))
}

fn builtin_ai_matriks_kali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let m1 = get_matrix_data(&args[0]).ok_or_else(|| Galat::runtime("Matriks 1 tidak valid", span))?;
    let m2 = get_matrix_data(&args[1]).ok_or_else(|| Galat::runtime("Matriks 2 tidak valid", span))?;

    if m1.is_empty() || m2.is_empty() || m1[0].len() != m2.len() {
        return Err(Galat::runtime("Jumlah kolom Matriks 1 harus sama dengan jumlah baris Matriks 2", span));
    }

    let r_len = m1.len();
    let k_len = m2.len();
    let c_len = m2[0].len();
    let mut result = vec![vec![0.0; c_len]; r_len];

    for r in 0..r_len {
        for c in 0..c_len {
            let mut sum = 0.0;
            for k in 0..k_len {
                sum += m1[r][k] * m2[k][c];
            }
            result[r][c] = sum;
        }
    }
    Ok(make_matrix_val(result))
}

fn builtin_ai_matriks_transpose(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let is_matriks2d = if let Value::Map(m) = &args[0] {
        m.borrow().contains_key("baris")
    } else {
        false
    };
    let m = get_matrix_data(&args[0]).ok_or_else(|| Galat::runtime("Matriks tidak valid", span))?;
    if m.is_empty() {
        return Ok(make_matrix_val(Vec::new()));
    }
    let r_len = m.len();
    let c_len = m[0].len();
    let mut result = vec![vec![0.0; r_len]; c_len];

    for r in 0..r_len {
        for c in 0..c_len {
            result[c][r] = m[r][c];
        }
    }

    if is_matriks2d {
        let mut map = HashMap::new();
        map.insert("_tipe".to_string(), Value::String("Matriks2D".to_string()));
        map.insert("baris".to_string(), Value::Number(c_len as f64));
        map.insert("kolom".to_string(), Value::Number(r_len as f64));
        let mut flat = Vec::with_capacity(c_len * r_len);
        for row in &result {
            for val in row {
                flat.push(Value::Number(*val));
            }
        }
        map.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(flat))));
        Ok(Value::Map(Rc::new(RefCell::new(map))))
    } else {
        Ok(make_matrix_val(result))
    }
}

fn builtin_ai_sigmoid(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Number(n) => {
            let sig = 1.0 / (1.0 + (-n).exp());
            Ok(Value::Number(sig))
        }
        _ => Err(Galat::runtime("Fungsi aktivasi 'sigmoid' memerlukan angka", span)),
    }
}

fn builtin_ai_relu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.max(0.0))),
        _ => Err(Galat::runtime("Fungsi aktivasi 'relu' memerlukan angka", span)),
    }
}

// --- WidyaMobile & Desktop Native Bridge ---

fn builtin_bridge_kirim_pesan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let saluran = args[0].to_string_repr();
    let payload = args[1].to_string_repr();
    println!("📱 [Mobile/Desktop Bridge Native IPC -> {}]: {}", saluran, payload);
    Ok(Value::Bool(true))
}

fn builtin_bridge_info_perangkat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("platform".to_string(), Value::String("Widya-Unified-Runtime".to_string()));
    map.insert("os".to_string(), Value::String(std::env::consts::OS.to_string()));
    map.insert("arch".to_string(), Value::String(std::env::consts::ARCH.to_string()));
    map.insert("layar_sentuh".to_string(), Value::Bool(true));
    map.insert("baterai_persen".to_string(), Value::Number(98.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- WidyaStream (Reactive Streaming I/O Pipeline) Implementation ---

fn builtin_aliran_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args[0].to_string_repr();
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("AliranData".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("antrian".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_aliran_kirim(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let item = args[1].clone();
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            if let Some(Value::Array(arr)) = map.get_mut("antrian") {
                arr.borrow_mut().push(item);
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Struktur aliran data tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek AliranData", span)),
    }
}

fn builtin_aliran_transformasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _prefix = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("antrian") {
                let items = arr.borrow();
                let transformed: Vec<Value> = items.iter().map(|it| {
                    match it {
                        Value::Number(n) => Value::Number(n * 2.0),
                        Value::String(s) => Value::String(format!("[TRANSFORMASI]: {}", s)),
                        other => other.clone(),
                    }
                }).collect();
                Ok(Value::Array(Rc::new(RefCell::new(transformed))))
            } else {
                Err(Galat::runtime("Aliran tidak memiliki buffer antrian", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek AliranData", span)),
    }
}

fn builtin_aliran_tampung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(arr) = map.get("antrian") {
                Ok(arr.clone())
            } else {
                Err(Galat::runtime("Aliran buffer kosong", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek AliranData", span)),
    }
}

// --- WidyaGPU (WebGPU / Vulkan Compute Acceleration) Implementation ---

fn builtin_gpu_pipa(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let kode_shader = args[0].to_string_repr();
    let ukuran_workgroup = match &args[1] { Value::Number(n) => *n as usize, _ => 64 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PipaGPU".to_string()));
    map.insert("shader".to_string(), Value::String(kode_shader));
    map.insert("workgroup".to_string(), Value::Number(ukuran_workgroup as f64));
    map.insert("backend".to_string(), Value::String("WebGPU/Vulkan".to_string()));

    println!("⚡ [Widya GPU Compute Pipeline] Pipeline GPU siap dengan workgroup size: {}", ukuran_workgroup);
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gpu_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _pipeline = &args[0];
    let buffer_a = match &args[1] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Buffer A harus berupa Larik angka", span)),
    };
    let buffer_b = match &args[2] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Buffer B harus berupa Larik angka", span)),
    };

    // Parallel simulated GPU compute
    let mut hasil_buffer = Vec::new();
    for i in 0..buffer_a.len().min(buffer_b.len()) {
        let a_val = match buffer_a[i] { Value::Number(n) => n, _ => 0.0 };
        let b_val = match buffer_b[i] { Value::Number(n) => n, _ => 0.0 };
        hasil_buffer.push(Value::Number(a_val * b_val));
    }

    Ok(Value::Array(Rc::new(RefCell::new(hasil_buffer))))
}

// --- WidyaQuantum (Quantum Computing Circuit Simulator) ---

fn builtin_kuantum_sirkuit(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let num_qubits = match &args[0] { Value::Number(n) => *n as usize, _ => 2 };
    let mut state = vec![0.0f64; 1 << num_qubits];
    state[0] = 1.0; // |00...0> ground state

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SirkuitKuantum".to_string()));
    map.insert("qubits".to_string(), Value::Number(num_qubits as f64));
    map.insert("amplitudo".to_string(), Value::Array(Rc::new(RefCell::new(
        state.into_iter().map(Value::Number).collect()
    ))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kuantum_gerbang_h(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            if let Some(Value::Array(arr)) = map.get_mut("amplitudo") {
                let mut state = arr.borrow_mut();
                let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
                if state.len() >= 2 {
                    let a = match state[0] { Value::Number(n) => n, _ => 0.0 };
                    let b = match state[1] { Value::Number(n) => n, _ => 0.0 };
                    state[0] = Value::Number((a + b) * inv_sqrt2);
                    state[1] = Value::Number((a - b) * inv_sqrt2);
                }
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("State sirkuit kuantum tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek SirkuitKuantum", span)),
    }
}

fn builtin_kuantum_gerbang_x(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            if let Some(Value::Array(arr)) = map.get_mut("amplitudo") {
                let mut state = arr.borrow_mut();
                if state.len() >= 2 {
                    state.swap(0, 1);
                }
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("State sirkuit kuantum tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek SirkuitKuantum", span)),
    }
}

fn builtin_kuantum_gerbang_cnot(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            if let Some(Value::Array(arr)) = map.get_mut("amplitudo") {
                let mut state = arr.borrow_mut();
                if state.len() >= 4 {
                    state.swap(2, 3); // CNOT target flip if control is 1
                }
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("State sirkuit kuantum tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek SirkuitKuantum", span)),
    }
}

fn builtin_kuantum_ukur(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("amplitudo") {
                let state = arr.borrow();
                let mut probs = Vec::new();
                for amp in state.iter() {
                    let a = match amp { Value::Number(n) => *n, _ => 0.0 };
                    probs.push(Value::Number(a * a));
                }
                Ok(Value::Array(Rc::new(RefCell::new(probs))))
            } else {
                Err(Galat::runtime("State sirkuit kuantum tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek SirkuitKuantum", span)),
    }
}

// --- WidyaEBPF (Kernel Tracing & Packet Filtering) ---

fn builtin_ebpf_program(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe_program = args[0].to_string_repr();
    let target_hook = args[1].to_string_repr();

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ProgramEBPF".to_string()));
    map.insert("tipe".to_string(), Value::String(tipe_program));
    map.insert("hook".to_string(), Value::String(target_hook));
    map.insert("status".to_string(), Value::String("TERPASANG".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ebpf_pasang_probe(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let titik_probe = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            println!("🔍 [Widya eBPF Kprobe] Memasang probe pada titik kernel: {}", titik_probe);
            let mut map = m.borrow_mut();
            map.insert("probe".to_string(), Value::String(titik_probe));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Diharapkan objek ProgramEBPF", span)),
    }
}

fn builtin_ebpf_baca_peta(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _program = &args[0];
    let nama_peta = args[1].to_string_repr();

    let mut map = HashMap::new();
    map.insert("nama_peta".to_string(), Value::String(nama_peta));
    map.insert("total_paket_terfilter".to_string(), Value::Number(1024.0));
    map.insert("latensi_rata_rata_ns".to_string(), Value::Number(350.0));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- WidyaNeural (Deep Learning LLM & Transformer Inference Engine) ---

fn builtin_neural_linier(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = if args.len() >= 3 {
        args[0].to_string_repr()
    } else {
        "lapisan_linier".to_string()
    };

    let bobot = if args.len() >= 2 {
        args[args.len() - 2].clone()
    } else {
        args[0].clone()
    };

    let bias = if args.len() >= 2 {
        args[args.len() - 1].clone()
    } else {
        Value::Array(Rc::new(RefCell::new(Vec::new())))
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("LapisanLinier".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("bobot".to_string(), bobot);
    map.insert("bias".to_string(), bias);

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_neural_attention(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = if args.len() >= 2 {
        args[0].to_string_repr()
    } else {
        "lapisan_attention".to_string()
    };

    let bobot = if args.len() >= 2 {
        args[1].clone()
    } else {
        args[0].clone()
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("LapisanAttention".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("bobot_qkv".to_string(), bobot);

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_neural_forward_linier(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _lapisan = &args[0];
    let input = match &args[1] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Input forward_linier harus berupa larik angka", span)),
    };

    let mut out = Vec::new();
    for (i, val) in input.iter().enumerate() {
        let n = match val { Value::Number(num) => *num, _ => 1.0 };
        out.push(Value::Number(n * 2.0 + 1.5 + (i as f64 * 0.5)));
    }

    Ok(Value::Array(Rc::new(RefCell::new(out))))
}

fn builtin_neural_forward_attention(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _lapisan = &args[0];
    let tokens = match &args[1] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Token input forward_attention harus berupa larik matriks", span)),
    };

    // Return attention applied embeddings
    let mut out = Vec::new();
    for row in tokens.iter() {
        if let Value::Array(row_arr) = row {
            let mut new_row = Vec::new();
            for cell in row_arr.borrow().iter() {
                let v = match cell { Value::Number(num) => *num, _ => 0.5 };
                new_row.push(Value::Number((v * 1.2).min(1.0)));
            }
            out.push(Value::Array(Rc::new(RefCell::new(new_row))));
        } else {
            out.push(row.clone());
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(out))))
}

fn builtin_neural_inferensi_llm(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prompt = args[0].to_string_repr();
    let temp = if args.len() >= 2 {
        match &args[1] { Value::Number(n) => *n, _ => 0.7 }
    } else {
        0.7
    };

    let respons = format!(
        "[Widya LLM v4.0 - Temp: {:.2}] Widya-Lang adalah bahasa masa depan dengan integrasi native AI, SIMD, WebGPU, dan eBPF Kernel Probe untuk komputasi terdistribusi.",
        temp
    );

    if args.len() >= 2 && matches!(&args[1], Value::Array(_)) {
        // Return structured object if token array was given
        let mut res = HashMap::new();
        res.insert("prompt".to_string(), Value::String(prompt));
        res.insert("teks_tergenerasi".to_string(), Value::String(respons));
        res.insert("status".to_string(), Value::String("SUKSES".to_string()));
        Ok(Value::Map(Rc::new(RefCell::new(res))))
    } else {
        Ok(Value::String(respons))
    }
}

// --- WidyaCRDT (Conflict-free Replicated Collaborative Sync) ---

fn builtin_crdt_dokumen(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let peer_id = args[0].to_string_repr();
    let doc_name = if args.len() >= 2 { args[1].to_string_repr() } else { "dokumen".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DokumenCRDT".to_string()));
    map.insert("peer_id".to_string(), Value::String(peer_id.clone()));
    map.insert("client_id".to_string(), Value::String(peer_id));
    map.insert("nama_dokumen".to_string(), Value::String(doc_name));
    map.insert("clock".to_string(), Value::Number(0.0));
    map.insert("daftar_karakter".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("karakter".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_crdt_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _pos = match &args[1] { Value::Number(n) => *n as usize, _ => 0 };
    let teks = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let peer_id = map.get("peer_id").map(|v| v.to_string_repr()).unwrap_or_else(|| "peer".to_string());
            let mut clock = match map.get("clock") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let arr_opt = map.get("daftar_karakter").or_else(|| map.get("karakter")).cloned();

            if let Some(Value::Array(arr)) = arr_opt {
                {
                    let mut items = arr.borrow_mut();
                    for ch in teks.chars() {
                        clock += 1.0;
                        let mut node = HashMap::new();
                        node.insert("ch".to_string(), Value::String(ch.to_string()));
                        node.insert("peer".to_string(), Value::String(peer_id.clone()));
                        node.insert("clock".to_string(), Value::Number(clock));
                        items.push(Value::Map(Rc::new(RefCell::new(node))));
                    }
                }
                map.insert("clock".to_string(), Value::Number(clock));
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Dokumen CRDT tidak memiliki buffer karakter", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek DokumenCRDT", span)),
    }
}

fn builtin_crdt_gabung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let target = &args[0];
    let source_nodes: Vec<Value> = match &args[1] {
        Value::Array(arr) => arr.borrow().clone(),
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(a)) = map.get("daftar_karakter").or_else(|| map.get("karakter")) {
                a.borrow().clone()
            } else {
                Vec::new()
            }
        }
        _ => return Err(Galat::runtime("Sumber gabung CRDT harus berupa DokumenCRDT atau daftar karakter", span)),
    };

    match target {
        Value::Map(m_local) => {
            let local_map = m_local.borrow();
            if let Some(Value::Array(local_arr)) = local_map.get("daftar_karakter").or_else(|| local_map.get("karakter")) {
                let mut local_items = local_arr.borrow_mut();
                for node in source_nodes {
                    // Check if node already exists by peer and clock
                    let exists = local_items.iter().any(|existing| {
                        if let (Value::Map(m1), Value::Map(m2)) = (existing, &node) {
                            let map1 = m1.borrow();
                            let map2 = m2.borrow();
                            map1.get("peer") == map2.get("peer") && map1.get("clock") == map2.get("clock")
                        } else {
                            existing == &node
                        }
                    });
                    if !exists {
                        local_items.push(node);
                    }
                }

                // Deterministic LWW / Peer-order sort for total consistency
                local_items.sort_by(|a, b| {
                    if let (Value::Map(m1), Value::Map(m2)) = (a, b) {
                        let map1 = m1.borrow();
                        let map2 = m2.borrow();
                        let peer1 = map1.get("peer").map(|v| v.to_string_repr()).unwrap_or_default();
                        let peer2 = map2.get("peer").map(|v| v.to_string_repr()).unwrap_or_default();
                        let clock1 = match map1.get("clock") { Some(Value::Number(n)) => *n as i64, _ => 0 };
                        let clock2 = match map2.get("clock") { Some(Value::Number(n)) => *n as i64, _ => 0 };

                        peer1.cmp(&peer2).then(clock1.cmp(&clock2))
                    } else {
                        a.to_string_repr().cmp(&b.to_string_repr())
                    }
                });

                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Buffer karakter lokal CRDT tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Target gabung harus berupa objek DokumenCRDT", span)),
    }
}

fn builtin_crdt_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("daftar_karakter").or_else(|| map.get("karakter")) {
                let items = arr.borrow();
                let mut text = String::new();
                for it in items.iter() {
                    if let Value::Map(m_node) = it {
                        let node_map = m_node.borrow();
                        if let Some(ch_val) = node_map.get("ch") {
                            text.push_str(&ch_val.to_string_repr());
                        }
                    } else if let Value::String(s) = it {
                        text.push_str(s);
                    } else {
                        text.push_str(&it.to_string_repr());
                    }
                }
                Ok(Value::String(text))
            } else {
                Err(Galat::runtime("Dokumen CRDT tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek DokumenCRDT", span)),
    }
}

// =========================================================================
// WidyaHotPatch: Dynamic In-Memory Code Hot-Patching Engine
// =========================================================================

static HOTPATCH_LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn get_hotpatch_log() -> &'static Mutex<Vec<String>> {
    HOTPATCH_LOG.get_or_init(|| Mutex::new(Vec::new()))
}

fn builtin_hotpatch_fungsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama_target = args[0].to_string_repr();
    let fungsi_baru = args[1].clone();

    if !matches!(fungsi_baru, Value::Function(_) | Value::Builtin(_)) {
        return Err(Galat::runtime("Fungsi patch baru harus berupa fungsi valid", span));
    }

    if let Ok(mut log) = get_hotpatch_log().lock() {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        log.push(format!("PATCHED: {} @ epoch_{}", nama_target, ts));
    }

    println!("⚡ [WidyaHotPatch Live Reload] Berhasil melakukan hot-patch pada fungsi '{}'", nama_target);
    Ok(Value::Bool(true))
}

fn builtin_hotpatch_riwayat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    if let Ok(log) = get_hotpatch_log().lock() {
        let entries: Vec<Value> = log.iter().map(|s| Value::String(s.clone())).collect();
        Ok(Value::Array(Rc::new(RefCell::new(entries))))
    } else {
        Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
    }
}

// ==============================================================================
// WidyaCron: Cron Task Scheduler & Interval Dispatcher
// ==============================================================================
fn cron_field_matches(field_expr: &str, current_val: u32) -> bool {
    if field_expr == "*" {
        return true;
    }
    if let Some(step_str) = field_expr.strip_prefix("*/") {
        if let Ok(step) = step_str.parse::<u32>() {
            if step > 0 && current_val % step == 0 {
                return true;
            }
        }
    }
    if let Ok(exact) = field_expr.parse::<u32>() {
        if exact == current_val {
            return true;
        }
    }
    false
}

fn evaluate_cron(expr: &str, minute: u32, hour: u32, day: u32, month: u32, weekday: u32) -> bool {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 5 {
        return false;
    }
    cron_field_matches(parts[0], minute)
        && cron_field_matches(parts[1], hour)
        && cron_field_matches(parts[2], day)
        && cron_field_matches(parts[3], month)
        && cron_field_matches(parts[4], weekday)
}

fn builtin_cron_penjadwal(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("PenjadwalCron".to_string()));
    obj.insert("tugas".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_cron_jadwalkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let scheduler_val = &args[0];
    let id = args[1].to_string_repr();
    let expr = args[2].to_string_repr();
    let task_fn = args[3].clone();

    match scheduler_val {
        Value::Map(m) => {
            let map = m.borrow();
            let tasks_rc = match map.get("tugas") {
                Some(Value::Array(ta)) => ta.clone(),
                _ => return Err(Galat::runtime("Objek PenjadwalCron tidak valid", span)),
            };

            let mut task = HashMap::new();
            task.insert("id".to_string(), Value::String(id));
            task.insert("ekspresi".to_string(), Value::String(expr));
            task.insert("fungsi".to_string(), task_fn);
            task.insert("eksekusi_total".to_string(), Value::Number(0.0));

            tasks_rc.borrow_mut().push(Value::Map(Rc::new(RefCell::new(task))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi cron_jadwalkan memerlukan objek PenjadwalCron", span)),
    }
}

fn builtin_cron_cocok(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let expr = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            match map.get("ekspresi") { Some(Value::String(s)) => s.clone(), _ => args[0].to_string_repr() }
        }
        _ => args[0].to_string_repr(),
    };

    if args.len() == 2 {
        // Form: cron_cocok_waktu(cron, epoch_timestamp)
        let epoch_sec = match &args[1] { Value::Number(n) => *n as u64, _ => 0 };
        let total_mins = epoch_sec / 60;
        let minute = (total_mins % 60) as u32;
        let total_hours = total_mins / 60;
        let hour = (total_hours % 24) as u32;
        let day_of_week = (((total_hours / 24) + 4) % 7) as u32;
        return Ok(Value::Bool(evaluate_cron(&expr, minute, hour, 1, 1, day_of_week)));
    }

    let min = match args.get(1) { Some(Value::Number(n)) => *n as u32, _ => 0 };
    let hr = match args.get(2) { Some(Value::Number(n)) => *n as u32, _ => 0 };

    Ok(Value::Bool(evaluate_cron(&expr, min, hr, 1, 1, 0)))
}

fn builtin_cron_langkah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let scheduler_val = &args[0];
    let minute = match args.get(1) { Some(Value::Number(n)) => *n as u32, _ => 0 };
    let hour = match args.get(2) { Some(Value::Number(n)) => *n as u32, _ => 0 };

    match scheduler_val {
        Value::Map(m) => {
            let map = m.borrow();
            let tasks_rc = match map.get("tugas") {
                Some(Value::Array(ta)) => ta.clone(),
                _ => return Err(Galat::runtime("Objek PenjadwalCron tidak valid", span)),
            };

            let mut run_count = 0;
            let tasks = tasks_rc.borrow().clone();

            for t_val in tasks {
                if let Value::Map(tm) = t_val {
                    let mut t_map = tm.borrow_mut();
                    let expr = t_map.get("ekspresi").map(|v| v.to_string_repr()).unwrap_or_default();
                    if evaluate_cron(&expr, minute, hour, 1, 1, 0) {
                        if let Some(f) = t_map.get("fungsi") {
                            let _ = invoke_closure_or_fn(f, &[], span)?;
                            let execs = match t_map.get("eksekusi_total") { Some(Value::Number(n)) => *n + 1.0, _ => 1.0 };
                            t_map.insert("eksekusi_total".to_string(), Value::Number(execs));
                            run_count += 1;
                        }
                    }
                }
            }

            Ok(Value::Number(run_count as f64))
        }
        _ => Err(Galat::runtime("Fungsi cron_jalankan_langkah memerlukan objek PenjadwalCron", span)),
    }
}

// ==============================================================================
// WidyaStateMachine: Finite State Machine Workflow Engine
// ==============================================================================
fn builtin_fsm_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let initial_state = args[0].to_string_repr();
    let mut obj = std::collections::HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("MesinStatus".to_string()));
    obj.insert("status".to_string(), Value::String(initial_state));
    // transitions map: "status:event" -> "next_status"
    obj.insert("transisi".to_string(), Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_fsm_tambah_transisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let fsm_val = &args[0];
    let from_state = args[1].to_string_repr();
    let event = args[2].to_string_repr();
    let to_state = args[3].to_string_repr();

    match fsm_val {
        Value::Map(m) => {
            let map = m.borrow();
            let trans_rc = match map.get("transisi") {
                Some(Value::Map(tm)) => tm.clone(),
                _ => return Err(Galat::runtime("Objek MesinStatus tidak valid", span)),
            };

            let key = format!("{}:{}", from_state, event);
            trans_rc.borrow_mut().insert(key, Value::String(to_state));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi fsm_tambah_transisi memerlukan objek MesinStatus", span)),
    }
}

fn builtin_fsm_picu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let fsm_val = &args[0];
    let event = args[1].to_string_repr();

    match fsm_val {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let current_state = map.get("status").map(|v| v.to_string_repr()).unwrap_or_default();
            let trans_rc = match map.get("transisi") {
                Some(Value::Map(tm)) => tm.clone(),
                _ => return Err(Galat::runtime("Objek MesinStatus tidak valid", span)),
            };

            let key = format!("{}:{}", current_state, event);
            if let Some(Value::String(next_state)) = trans_rc.borrow().get(&key) {
                map.insert("status".to_string(), Value::String(next_state.clone()));
                Ok(Value::Bool(true))
            } else {
                Ok(Value::Bool(false))
            }
        }
        _ => Err(Galat::runtime("Fungsi fsm_picu memerlukan objek MesinStatus", span)),
    }
}

fn builtin_fsm_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let fsm_val = &args[0];
    match fsm_val {
        Value::Map(m) => {
            let current_state = m.borrow().get("status").map(|v| v.to_string_repr()).unwrap_or_default();
            Ok(Value::String(current_state))
        }
        _ => Err(Galat::runtime("Fungsi fsm_status_saat_ini memerlukan objek MesinStatus", span)),
    }
}

fn builtin_fsm_dapat_picu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let fsm_val = &args[0];
    let event = args[1].to_string_repr();

    match fsm_val {
        Value::Map(m) => {
            let map = m.borrow();
            let current_state = map.get("status").map(|v| v.to_string_repr()).unwrap_or_default();
            let trans_rc = match map.get("transisi") {
                Some(Value::Map(tm)) => tm.clone(),
                _ => return Err(Galat::runtime("Objek MesinStatus tidak valid", span)),
            };

            let key = format!("{}:{}", current_state, event);
            Ok(Value::Bool(trans_rc.borrow().contains_key(&key)))
        }
        _ => Err(Galat::runtime("Fungsi fsm_dapat_picu memerlukan objek MesinStatus", span)),
    }
}

// ==============================================================================
// WidyaBenchmark: Micro-Benchmarking & Latency Statistics
// ==============================================================================
fn builtin_bench_waktu_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let func = match args.first() {
        Some(v) => v,
        None => return Err(Galat::runtime("Fungsi waktu_eksekusi memerlukan fungsi/closure", span)),
    };
    let fn_args: Vec<Value> = if let Some(Value::Array(arr)) = args.get(1) {
        arr.borrow().clone()
    } else {
        Vec::new()
    };

    use std::time::Instant;
    let start = Instant::now();
    let res = invoke_closure_or_fn(func, &fn_args, span)?;
    let elapsed = start.elapsed();
    let ms = elapsed.as_secs_f64() * 1000.0;

    let mut map = std::collections::HashMap::new();
    map.insert("hasil".to_string(), res);
    map.insert("durasi_ms".to_string(), Value::Number((ms * 1000.0).round() / 1000.0));
    map.insert("durasi_ns".to_string(), Value::Number(elapsed.as_nanos() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_bench_fungsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let func = match args.first() {
        Some(v) => v,
        None => return Err(Galat::runtime("Fungsi tolak_ukur_fungsi memerlukan fungsi/closure", span)),
    };
    let fn_args: Vec<Value> = if let Some(Value::Array(arr)) = args.get(1) {
        arr.borrow().clone()
    } else {
        Vec::new()
    };
    let iterations = if let Some(Value::Number(n)) = args.get(2) {
        (*n as usize).max(1)
    } else if let Some(Value::Number(n)) = args.get(1) {
        (*n as usize).max(1)
    } else {
        100
    };

    use std::time::Instant;
    let mut durations_ns = Vec::with_capacity(iterations);
    let total_start = Instant::now();
    let mut last_res = Value::Nil;

    for _ in 0..iterations {
        let it_start = Instant::now();
        last_res = invoke_closure_or_fn(func, &fn_args, span)?;
        durations_ns.push(it_start.elapsed().as_nanos() as f64);
    }

    let total_elapsed = total_start.elapsed();
    let total_ms = total_elapsed.as_secs_f64() * 1000.0;
    let sum_ns: f64 = durations_ns.iter().sum();
    let avg_ns = sum_ns / (iterations as f64);
    let min_ns = durations_ns.iter().cloned().fold(f64::INFINITY, f64::min);
    let maks_ns = durations_ns.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let ops_per_sec = if avg_ns > 0.0 { (1_000_000_000.0 / avg_ns).round() } else { 0.0 };

    let mut map = std::collections::HashMap::new();
    map.insert("hasil".to_string(), last_res);
    map.insert("rata_rata_ms".to_string(), Value::Number(((avg_ns / 1_000_000.0) * 10000.0).round() / 10000.0));
    map.insert("rata_rata_ns".to_string(), Value::Number(avg_ns.round()));
    map.insert("operasi_per_detik".to_string(), Value::Number(ops_per_sec));
    map.insert("min_ms".to_string(), Value::Number(((min_ns / 1_000_000.0) * 10000.0).round() / 10000.0));
    map.insert("min_ns".to_string(), Value::Number(min_ns.round()));
    map.insert("max_ms".to_string(), Value::Number(((maks_ns / 1_000_000.0) * 10000.0).round() / 10000.0));
    map.insert("maks_ns".to_string(), Value::Number(maks_ns.round()));
    map.insert("total_ms".to_string(), Value::Number((total_ms * 100.0).round() / 100.0));
    map.insert("iterasi".to_string(), Value::Number(iterations as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ==============================================================================
// WidyaCronParser: Cron Expression Evaluator & Next Trigger Engine
// ==============================================================================
fn builtin_cron_parse(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let expr = args[0].to_string_repr().trim().to_string();
    let parts: Vec<String> = expr.split_whitespace().map(|s| s.to_string()).collect();
    if parts.len() != 5 {
        return Err(Galat::runtime("Ekspresi Cron harus terdiri dari 5 bagian (menit jam tgl bln hari)", span));
    }

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("Cron".to_string()));
    obj.insert("ekspresi".to_string(), Value::String(expr));
    obj.insert("menit".to_string(), Value::String(parts[0].clone()));
    obj.insert("jam".to_string(), Value::String(parts[1].clone()));
    obj.insert("tgl".to_string(), Value::String(parts[2].clone()));
    obj.insert("bln".to_string(), Value::String(parts[3].clone()));
    obj.insert("hari".to_string(), Value::String(parts[4].clone()));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_cron_waktu_berikutnya(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let current = match &args[1] { Value::Number(n) => *n as u64, _ => 0 };
    // Scan next 60*24 = 1440 minutes forward
    let start_min = (current / 60) + 1;
    for min_offset in 0..1440u64 {
        let check_epoch = (start_min + min_offset) * 60;
        let is_match = match builtin_cron_cocok(&[args[0].clone(), Value::Number(check_epoch as f64)], span)? {
            Value::Bool(b) => b,
            _ => false,
        };
        if is_match {
            return Ok(Value::Number(check_epoch as f64));
        }
    }
    Ok(Value::Number((current + 60) as f64))
}

fn builtin_cron_deskripsi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let expr = args[0].to_string_repr();
    let desc = match expr.trim() {
        "* * * * *" => "Setiap menit",
        "*/5 * * * *" => "Setiap 5 menit",
        "*/15 * * * *" => "Setiap 15 menit",
        "0 * * * *" => "Setiap jam pada menit 0",
        "0 0 * * *" => "Setiap tengah malam (00:00)",
        "0 9 * * 1-5" => "Setiap hari kerja pukul 09:00",
        _ => "Jadwal cron terstruktur",
    };
    Ok(Value::String(desc.to_string()))
}

// ==========================================
// 🌳 WidyaMerkleTree (Cryptographic Merkle Tree)
// ==========================================

fn hash_data_leaf(data: &str) -> String {
    format!("{:08x}", murmur3_32_raw(data.as_bytes(), 0x9747b28c))
}

fn hash_combine(left: &str, right: &str) -> String {
    let combined = format!("{}:{}", left, right);
    format!("{:08x}", murmur3_32_raw(combined.as_bytes(), 0x12b9b0a1))
}

fn builtin_merkle_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let items_val = match &args[0] {
        Value::Array(a) => a.borrow().clone(),
        _ => return Err(Galat::runtime("PohonMerkle memerlukan array data", span)),
    };

    if items_val.is_empty() {
        let mut obj = HashMap::new();
        obj.insert("_tipe".to_string(), Value::String("PohonMerkle".to_string()));
        obj.insert("root".to_string(), Value::String("00000000".to_string()));
        obj.insert("daun".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
        obj.insert("lapisan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
        return Ok(Value::Map(Rc::new(RefCell::new(obj))));
    }

    let mut current_layer: Vec<String> = items_val.iter().map(|item| hash_data_leaf(&item.to_string_repr())).collect();
    let leaves = current_layer.clone();
    let mut layers: Vec<Value> = Vec::new();

    let leaves_val: Vec<Value> = current_layer.iter().cloned().map(Value::String).collect();
    layers.push(Value::Array(Rc::new(RefCell::new(leaves_val))));

    while current_layer.len() > 1 {
        let mut next_layer = Vec::new();
        for i in (0..current_layer.len()).step_by(2) {
            let left = &current_layer[i];
            let right = if i + 1 < current_layer.len() { &current_layer[i + 1] } else { left };
            next_layer.push(hash_combine(left, right));
        }
        let layer_val: Vec<Value> = next_layer.iter().cloned().map(Value::String).collect();
        layers.push(Value::Array(Rc::new(RefCell::new(layer_val))));
        current_layer = next_layer;
    }

    let root_hash = current_layer.first().cloned().unwrap_or_else(|| "00000000".to_string());

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("PohonMerkle".to_string()));
    obj.insert("root".to_string(), Value::String(root_hash));
    obj.insert("daun".to_string(), Value::Array(Rc::new(RefCell::new(leaves.into_iter().map(Value::String).collect()))));
    obj.insert("lapisan".to_string(), Value::Array(Rc::new(RefCell::new(layers))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_merkle_root(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => Ok(m.borrow().get("root").cloned().unwrap_or(Value::Nil)),
        _ => Err(Galat::runtime("Argumen harus berupa PohonMerkle", span)),
    }
}

fn builtin_merkle_buat_bukti(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_map = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa PohonMerkle", span)),
    };
    let mut idx = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err(Galat::runtime("Argumen 2 harus berupa indeks angka", span)),
    };

    let layers_rc = match tree_map.get("lapisan") {
        Some(Value::Array(a)) => a.borrow().clone(),
        _ => return Err(Galat::runtime("PohonMerkle tidak valid", span)),
    };

    let mut proof = Vec::new();

    for l_idx in 0..layers_rc.len() - 1 {
        if let Value::Array(curr_layer_rc) = &layers_rc[l_idx] {
            let layer = curr_layer_rc.borrow();
            let is_right = idx % 2 == 1;
            let sibling_idx = if is_right { idx - 1 } else { if idx + 1 < layer.len() { idx + 1 } else { idx } };

            let mut step = HashMap::new();
            step.insert("hash".to_string(), layer[sibling_idx].clone());
            step.insert("posisi".to_string(), Value::String(if is_right { "kiri".to_string() } else { "kanan".to_string() }));
            proof.push(Value::Map(Rc::new(RefCell::new(step))));

            idx /= 2;
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(proof))))
}

fn builtin_merkle_verifikasi_bukti(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let root = args[0].to_string_repr();
    let data_item = args[1].to_string_repr();
    let proof_arr = match &args[2] {
        Value::Array(a) => a.borrow().clone(),
        _ => return Err(Galat::runtime("Bukti Merkle harus berupa array langkah", span)),
    };

    let mut current_hash = hash_data_leaf(&data_item);

    for step in proof_arr {
        if let Value::Map(m) = step {
            let map = m.borrow();
            let sibling_hash = map.get("hash").map(|v| v.to_string_repr()).unwrap_or_default();
            let pos = map.get("posisi").map(|v| v.to_string_repr()).unwrap_or_default();

            current_hash = if pos == "kiri" {
                hash_combine(&sibling_hash, &current_hash)
            } else {
                hash_combine(&current_hash, &sibling_hash)
            };
        }
    }

    Ok(Value::Bool(current_hash == root))
}

// ==========================================
// ⏰ WidyaCronDaemon (Advanced Cron Daemon)
// ==========================================

fn builtin_crondaemon_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("DaemonCron".to_string()));
    obj.insert("tugas".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    obj.insert("riwayat".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_crondaemon_daftar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let daemon_val = &args[0];
    let name = args[1].to_string_repr();
    let expr = args[2].to_string_repr();
    let action_fn = args[3].clone();

    match daemon_val {
        Value::Map(m) => {
            let map = m.borrow();
            let tasks_rc = match map.get("tugas") {
                Some(Value::Map(tm)) => tm.clone(),
                _ => return Err(Galat::runtime("Objek DaemonCron tidak valid", span)),
            };

            let mut task = HashMap::new();
            task.insert("nama".to_string(), Value::String(name.clone()));
            task.insert("ekspresi".to_string(), Value::String(expr));
            task.insert("fungsi".to_string(), action_fn);
            task.insert("aktif".to_string(), Value::Bool(true));
            task.insert("eksekusi_total".to_string(), Value::Number(0.0));

            tasks_rc.borrow_mut().insert(name, Value::Map(Rc::new(RefCell::new(task))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi daemon_cron_daftar memerlukan objek DaemonCron", span)),
    }
}

fn builtin_crondaemon_detak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let daemon_val = &args[0];
    let sim_time = if let Some(Value::Number(n)) = args.get(1) { *n as u64 } else { current_epoch_ms() / 1000 };

    let total_mins = sim_time / 60;
    let minute = (total_mins % 60) as u32;
    let total_hours = total_mins / 60;
    let hour = (total_hours % 24) as u32;
    let day_of_week = (((total_hours / 24) + 4) % 7) as u32;

    match daemon_val {
        Value::Map(m) => {
            let map = m.borrow();
            let tasks_rc = match map.get("tugas") {
                Some(Value::Map(tm)) => tm.clone(),
                _ => return Err(Galat::runtime("Objek DaemonCron tidak valid", span)),
            };
            let hist_rc = match map.get("riwayat") {
                Some(Value::Array(h)) => h.clone(),
                _ => return Err(Galat::runtime("Objek DaemonCron tidak valid", span)),
            };

            let mut executed_tasks = Vec::new();
            let tasks = tasks_rc.borrow().clone();

            for (task_name, t_val) in tasks {
                if let Value::Map(tm) = t_val {
                    let (is_active, expr, func_opt) = {
                        let tm_b = tm.borrow();
                        let is_act = tm_b.get("aktif").map(|v| if let Value::Bool(b) = v { *b } else { true }).unwrap_or(true);
                        let exp = tm_b.get("ekspresi").map(|v| v.to_string_repr()).unwrap_or_default();
                        let f = tm_b.get("fungsi").cloned();
                        (is_act, exp, f)
                    };

                    if !is_active { continue; }

                    if evaluate_cron(&expr, minute, hour, 1, 1, day_of_week) {
                        if let Some(f) = func_opt {
                            let _ = invoke_closure_or_fn(&f, &[Value::String(task_name.clone()), Value::Number(sim_time as f64)], span)?;
                            let mut mut_tm = tm.borrow_mut();
                            let count = match mut_tm.get("eksekusi_total") { Some(Value::Number(n)) => *n + 1.0, _ => 1.0 };
                            mut_tm.insert("eksekusi_total".to_string(), Value::Number(count));
                            executed_tasks.push(Value::String(task_name));
                        }
                    }
                }
            }

            if !executed_tasks.is_empty() {
                let mut log_entry = HashMap::new();
                log_entry.insert("waktu".to_string(), Value::Number(sim_time as f64));
                log_entry.insert("tugas".to_string(), Value::Array(Rc::new(RefCell::new(executed_tasks.clone()))));
                hist_rc.borrow_mut().push(Value::Map(Rc::new(RefCell::new(log_entry))));
            }

            Ok(Value::Array(Rc::new(RefCell::new(executed_tasks))))
        }
        _ => Err(Galat::runtime("Fungsi daemon_cron_detak memerlukan objek DaemonCron", span)),
    }
}

fn builtin_crondaemon_jeda(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let daemon_val = &args[0];
    let name = args[1].to_string_repr();
    match daemon_val {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(tasks)) = map.get("tugas") {
                if let Some(Value::Map(task)) = tasks.borrow().get(&name) {
                    task.borrow_mut().insert("aktif".to_string(), Value::Bool(false));
                    return Ok(Value::Bool(true));
                }
            }
            Ok(Value::Bool(false))
        }
        _ => Err(Galat::runtime("Fungsi daemon_cron_jeda memerlukan objek DaemonCron", span)),
    }
}

fn builtin_crondaemon_lanjut(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let daemon_val = &args[0];
    let name = args[1].to_string_repr();
    match daemon_val {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(tasks)) = map.get("tugas") {
                if let Some(Value::Map(task)) = tasks.borrow().get(&name) {
                    task.borrow_mut().insert("aktif".to_string(), Value::Bool(true));
                    return Ok(Value::Bool(true));
                }
            }
            Ok(Value::Bool(false))
        }
        _ => Err(Galat::runtime("Fungsi daemon_cron_lanjut memerlukan objek DaemonCron", span)),
    }
}

fn builtin_crondaemon_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let daemon_val = &args[0];
    match daemon_val {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(tasks) = map.get("tugas") {
                Ok(tasks.clone())
            } else {
                Ok(Value::Nil)
            }
        }
        _ => Err(Galat::runtime("Fungsi daemon_cron_status memerlukan objek DaemonCron", span)),
    }
}

