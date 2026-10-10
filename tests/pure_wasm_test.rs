// ============================================================================
// Integration Test: Pure Sovereign WebAssembly (WASM) Emitter & Micro-VM
// ============================================================================
// Menguji serialisasi format biner WebAssembly v1 (\0asm) dan eksekusi
// stack machine di Micro-VM tanpa runtime eksternal (V8 / Wasmtime / Wasmer).
// ============================================================================

use widya::wasm::{
    PureWasmBuilder, WasmMicroVm, WasmVal, WasmValType, WASM_MAGIC, WASM_VERSION,
};

#[test]
fn test_wasm_binary_structure_and_headers() {
    let mut builder = PureWasmBuilder::baru();
    builder.tambah_fungsi(
        "konstanta",
        vec![],
        vec![WasmValType::I32],
        vec![],
        vec![0x41, 0x2A, 0x0B], // i32.const 42, end
    );

    let wasm_bytes = builder.bangun();

    // 1. Verifikasi Magic \0asm
    assert!(wasm_bytes.len() >= 8);
    assert_eq!(&wasm_bytes[0..4], &WASM_MAGIC);

    // 2. Verifikasi Version 1
    assert_eq!(&wasm_bytes[4..8], &WASM_VERSION);

    // 3. Muat ke Micro-VM
    let mut vm = WasmMicroVm::muat_modul(&wasm_bytes)
        .expect("Muat modul WASM harus berhasil");

    let hasil = vm.panggil("konstanta", &[])
        .expect("Panggilan fungsi harus sukses");

    assert_eq!(hasil, Some(WasmVal::I32(42)));
}

#[test]
fn test_wasm_addition_arithmetic() {
    let mut builder = PureWasmBuilder::baru();

    // Fungsi: tambah(a: i32, b: i32) -> i32
    // Bytecode: local.get 0, local.get 1, i32.add, end
    let bytecode = vec![
        0x20, 0x00, // local.get 0
        0x20, 0x01, // local.get 1
        0x6A,       // i32.add
        0x0B,       // end
    ];

    builder.tambah_fungsi(
        "tambah",
        vec![WasmValType::I32, WasmValType::I32],
        vec![WasmValType::I32],
        vec![],
        bytecode,
    );

    let wasm_bytes = builder.bangun();
    let mut vm = WasmMicroVm::muat_modul(&wasm_bytes)
        .expect("Muat modul tambah harus berhasil");

    let hasil = vm.panggil("tambah", &[WasmVal::I32(125), WasmVal::I32(375)])
        .expect("Eksekusi tambah harus sukses");

    assert_eq!(hasil, Some(WasmVal::I32(500)));
}

#[test]
fn test_wasm_multiplication_and_subtraction() {
    let mut builder = PureWasmBuilder::baru();

    // Fungsi: hitung(x: i32, y: i32) -> i32
    // Menghitung: (x * 10) - y
    let mut bytecode = Vec::new();
    bytecode.extend_from_slice(&[0x20, 0x00]); // local.get 0
    bytecode.push(0x41); // i32.const 10
    PureWasmBuilder::encode_i32(10, &mut bytecode);
    bytecode.push(0x6C); // i32.mul
    bytecode.extend_from_slice(&[0x20, 0x01]); // local.get 1
    bytecode.push(0x6B); // i32.sub
    bytecode.push(0x0B); // end

    builder.tambah_fungsi(
        "hitung",
        vec![WasmValType::I32, WasmValType::I32],
        vec![WasmValType::I32],
        vec![],
        bytecode,
    );

    let wasm_bytes = builder.bangun();
    let mut vm = WasmMicroVm::muat_modul(&wasm_bytes).unwrap();

    let res = vm.panggil("hitung", &[WasmVal::I32(7), WasmVal::I32(18)]).unwrap();
    // (7 * 10) - 18 = 70 - 18 = 52
    assert_eq!(res, Some(WasmVal::I32(52)));
}

#[test]
fn test_wasm_linear_memory_load_and_store() {
    let mut builder = PureWasmBuilder::baru();

    // 1 halaman memori (64KB)
    builder.tambah_memori(1, Some(2));

    // Data inisialisasi pada offset 64: nilai 77 (i32: 77, 0, 0, 0)
    builder.tambah_data(64, &77i32.to_le_bytes());

    // Fungsi 1: baca(addr: i32) -> i32
    // local.get 0, i32.load 2 0, end
    let read_code = vec![
        0x20, 0x00,       // local.get 0
        0x28, 0x02, 0x00, // i32.load align=2, offset=0
        0x0B,             // end
    ];
    builder.tambah_fungsi(
        "baca",
        vec![WasmValType::I32],
        vec![WasmValType::I32],
        vec![],
        read_code,
    );

    // Fungsi 2: tulis(addr: i32, val: i32)
    // local.get 0, local.get 1, i32.store 2 0, end
    let write_code = vec![
        0x20, 0x00,       // local.get 0 (addr)
        0x20, 0x01,       // local.get 1 (val)
        0x36, 0x02, 0x00, // i32.store align=2, offset=0
        0x0B,             // end
    ];
    builder.tambah_fungsi(
        "tulis",
        vec![WasmValType::I32, WasmValType::I32],
        vec![],
        vec![],
        write_code,
    );

    let wasm_bytes = builder.bangun();
    let mut vm = WasmMicroVm::muat_modul(&wasm_bytes)
        .expect("Muat modul memori harus berhasil");

    // 1. Baca data awal pada offset 64
    let val_awal = vm.panggil("baca", &[WasmVal::I32(64)]).unwrap();
    assert_eq!(val_awal, Some(WasmVal::I32(77)));

    // 2. Tulis nilai baru 9999 pada offset 128
    vm.panggil("tulis", &[WasmVal::I32(128), WasmVal::I32(9999)]).unwrap();

    // 3. Baca kembali dari offset 128
    let val_baru = vm.panggil("baca", &[WasmVal::I32(128)]).unwrap();
    assert_eq!(val_baru, Some(WasmVal::I32(9999)));
}
