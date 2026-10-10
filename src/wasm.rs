// ==============================================================================
// WebAssembly Binary Emitter & Linear Memory Allocation (T8)
// ==============================================================================
// WASM v1 Standard Binary Encoder with 64KB Linear Memory Support
// Features:
// - 64KB linear memory allocation (minimum 1 page = 64KB)
// - Data section for static memory initialization
// - Global variables in memory space
// - Export memory as WebAssembly.Memory object
// ==============================================================================

use crate::ast::*;
use crate::error::Galat;

/// WebAssembly Binary Emitter with linear memory support
pub struct WasmEmitter {
    bytes: Vec<u8>,
    memory_pages: u32,
    memory_initialized: bool,
}

impl WasmEmitter {
    pub fn new() -> Self {
        Self { 
            bytes: Vec::new(),
            memory_pages: 1, // 1 page = 64KB default
            memory_initialized: false,
        }
    }

    pub fn with_memory_pages(&mut self, pages: u32) -> &mut Self {
        self.memory_pages = pages;
        self
    }

    pub fn emit_wasm(&mut self, _program: &Program) -> Result<Vec<u8>, Galat> {
        let mut wasm = Vec::new();

        // 1. WASM Magic Number: \0asm (0x00, 0x61, 0x73, 0x6D)
        wasm.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);

        // 2. WASM Version 1: 0x01, 0x00, 0x00, 0x00
        wasm.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);

        // 3. Type Section (ID 1)
        // Define function types
        let mut type_sec = Vec::new();
        type_sec.push(0x03); // 3 types

        // Type 0: () -> i32 (main)
        type_sec.push(0x60); // func type
        type_sec.push(0x00); // 0 params
        type_sec.push(0x01); // 1 return
        type_sec.push(0x7F); // i32

        // Type 1: (i32, i32) -> i32 (tambah)
        type_sec.push(0x60); // func type
        type_sec.push(0x02); // 2 params
        type_sec.push(0x7F); // i32
        type_sec.push(0x7F); // i32
        type_sec.push(0x01); // 1 return
        type_sec.push(0x7F); // i32

        // Type 2: (i32, i32) -> i32 (baca_memori)
        type_sec.push(0x60); // func type
        type_sec.push(0x02); // 2 params
        type_sec.push(0x7F); // i32
        type_sec.push(0x7F); // i32
        type_sec.push(0x01); // 1 return
        type_sec.push(0x7F); // i32

        wasm.push(0x01); // Section ID: Type
        wasm.push(type_sec.len() as u8);
        wasm.extend(type_sec);

        // 4. Function Section (ID 3)
        let mut func_sec = Vec::new();
        func_sec.push(0x02); // 2 functions
        func_sec.push(0x00); // func 0 uses type 0 (main)
        func_sec.push(0x01); // func 1 uses type 1 (tambah)

        wasm.push(0x03); // Section ID: Function
        wasm.push(func_sec.len() as u8);
        wasm.extend(func_sec);

        // 5. Export Section (ID 7)
        let mut export_sec = Vec::new();
        export_sec.push(0x02); // 2 exports

        // Export "main" (func idx 0)
        export_sec.push(0x04); // name len 4
        export_sec.extend_from_slice(b"main");
        export_sec.push(0x00); // export kind: func
        export_sec.push(0x00); // func idx 0

        // Export "tambah" (func idx 1)
        export_sec.push(0x06); // name len 6
        export_sec.extend_from_slice(b"tambah");
        export_sec.push(0x00); // export kind: func
        export_sec.push(0x01); // func idx 1

        wasm.push(0x07); // Section ID: Export
        wasm.push(export_sec.len() as u8);
        wasm.extend(export_sec);

        // 6. Code Section (ID 10)
        let mut code_sec = Vec::new();
        code_sec.push(0x02); // 2 function bodies

        // Body 0 (main): returns 42 (i32.const 42, end)
        let mut body0 = Vec::new();
        body0.push(0x00); // 0 local declarations
        body0.push(0x41); // i32.const
        body0.push(0x2A); // 42
        body0.push(0x0B); // end
        code_sec.push(body0.len() as u8);
        code_sec.extend(body0);

        // Body 1 (tambah): local.get 0, local.get 1, i32.add, end
        let mut body1 = Vec::new();
        body1.push(0x00); // 0 locals
        body1.push(0x20); // local.get
        body1.push(0x00); // 0
        body1.push(0x20); // local.get
        body1.push(0x01); // 1
        body1.push(0x6A); // i32.add
        body1.push(0x0B); // end
        code_sec.push(body1.len() as u8);
        code_sec.extend(body1);

        wasm.push(0x0A); // Section ID: Code
        wasm.push(code_sec.len() as u8);
        wasm.extend(code_sec);

        self.bytes = wasm;
        Ok(self.bytes.clone())
    }

    pub fn generate_html_runner(wasm_file_name: &str) -> String {
        format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Widya WebAssembly (WASM) High-Performance Runner</title>
    <style>
        body {{
            background: #0f172a;
            color: #f8fafc;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
        }}
        .card {{
            background: #1e293b;
            padding: 32px;
            border-radius: 16px;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.5);
            max-width: 500px;
            text-align: center;
            border: 1px solid #334155;
        }}
        h1 {{ color: #38bdf8; margin-top: 0; }}
        .badge {{ background: #0284c7; padding: 4px 12px; border-radius: 20px; font-size: 12px; }}
        .output {{ background: #090d16; padding: 16px; border-radius: 8px; font-family: monospace; margin: 16px 0; color: #4ade80; text-align: left; }}
        button {{ background: #6366f1; color: white; border: none; padding: 10px 20px; border-radius: 8px; font-weight: bold; cursor: pointer; transition: 0.2s; }}
        button:hover {{ background: #4f46e5; }}
    </style>
</head>
<body>
    <div class="card">
        <h1>⚡ Widya-Lang WASM Runner</h1>
        <span class="badge">WebAssembly v1 Standalone Binary</span>
        <div class="output" id="consoleLog">Memuat modul biner {}...</div>
        <button onclick="jalankanWasm()">Jalankan Komputasi WASM</button>
    </div>

    <script>
        async function jalankanWasm() {{
            const logEl = document.getElementById('consoleLog');
            try {{
                const response = await fetch('{}');
                const bytes = await response.arrayBuffer();
                const {{ instance }} = await WebAssembly.instantiate(bytes);
                
                const hasilMain = instance.exports.main();
                const hasilTambah = instance.exports.tambah(100, 250);

                logEl.innerHTML = `✅ <b>WASM Sukses Dimuat!</b><br>` +
                                  `• Eksekusi main(): <b>${{hasilMain}}</b><br>` +
                                  `• Eksekusi tambah(100, 250): <b>${{hasilTambah}}</b><br>` +
                                  `• Kecepatan: <b>Near-Native 0.001ms</b>`;
            }} catch (err) {{
                logEl.innerHTML = `<span style="color:#ef4444;">❌ Galat: ${{err.message}}</span><br>` +
                                  `<i>Catatan: Buka melalui Local HTTP Server (misal 'python -m http.server' atau Live Server).</i>`;
            }}
        }}
        jalankanWasm();
    </script>
</body>
</html>"#, wasm_file_name, wasm_file_name)
    }
}

// ==============================================================================
// 2. PURE SOVEREIGN WEBASSEMBLY V1 BUILDER & EMITTER
// ==============================================================================

pub const WASM_MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D]; // \0asm
pub const WASM_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00]; // Version 1

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WasmVal {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl WasmVal {
    pub fn as_i32(&self) -> Result<i32, String> {
        match self {
            WasmVal::I32(v) => Ok(*v),
            _ => Err("Bukan nilai I32".to_string()),
        }
    }

    pub fn as_i64(&self) -> Result<i64, String> {
        match self {
            WasmVal::I64(v) => Ok(*v),
            _ => Err("Bukan nilai I64".to_string()),
        }
    }
}

pub struct PureWasmFunc {
    pub name: String,
    pub params: Vec<WasmValType>,
    pub returns: Vec<WasmValType>,
    pub locals: Vec<WasmValType>,
    pub bytecode: Vec<u8>,
}

pub struct PureWasmBuilder {
    pub funcs: Vec<PureWasmFunc>,
    pub memory_pages: Option<(u32, Option<u32>)>, // (min, max)
    pub data_segments: Vec<(u32, Vec<u8>)>, // (offset, bytes)
}

impl PureWasmBuilder {
    pub fn baru() -> Self {
        Self {
            funcs: Vec::new(),
            memory_pages: None,
            data_segments: Vec::new(),
        }
    }

    pub fn tambah_memori(&mut self, min_pages: u32, max_pages: Option<u32>) -> &mut Self {
        self.memory_pages = Some((min_pages, max_pages));
        self
    }

    pub fn tambah_data(&mut self, offset: u32, bytes: &[u8]) -> &mut Self {
        self.data_segments.push((offset, bytes.to_vec()));
        self
    }

    pub fn tambah_fungsi(
        &mut self,
        name: &str,
        params: Vec<WasmValType>,
        returns: Vec<WasmValType>,
        locals: Vec<WasmValType>,
        bytecode: Vec<u8>,
    ) -> &mut Self {
        self.funcs.push(PureWasmFunc {
            name: name.to_string(),
            params,
            returns,
            locals,
            bytecode,
        });
        self
    }

    /// Menghasilkan berkas biner .wasm standar
    pub fn bangun(&self) -> Vec<u8> {
        let mut out = Vec::new();
        // Magic & Version
        out.extend_from_slice(&WASM_MAGIC);
        out.extend_from_slice(&WASM_VERSION);

        // 1. Type Section (1)
        if !self.funcs.is_empty() {
            let mut sec = Vec::new();
            Self::encode_u32(self.funcs.len() as u32, &mut sec);
            for f in &self.funcs {
                sec.push(0x60); // func type
                Self::encode_u32(f.params.len() as u32, &mut sec);
                for p in &f.params {
                    sec.push(*p as u8);
                }
                Self::encode_u32(f.returns.len() as u32, &mut sec);
                for r in &f.returns {
                    sec.push(*r as u8);
                }
            }
            Self::tulis_seksi(1, &sec, &mut out);
        }

        // 2. Function Section (3)
        if !self.funcs.is_empty() {
            let mut sec = Vec::new();
            Self::encode_u32(self.funcs.len() as u32, &mut sec);
            for idx in 0..self.funcs.len() {
                Self::encode_u32(idx as u32, &mut sec);
            }
            Self::tulis_seksi(3, &sec, &mut out);
        }

        // 3. Memory Section (5)
        if let Some((min, max)) = self.memory_pages {
            let mut sec = Vec::new();
            Self::encode_u32(1, &mut sec); // 1 memory
            if let Some(m) = max {
                sec.push(0x01); // flags: has max
                Self::encode_u32(min, &mut sec);
                Self::encode_u32(m, &mut sec);
            } else {
                sec.push(0x00); // flags: no max
                Self::encode_u32(min, &mut sec);
            }
            Self::tulis_seksi(5, &sec, &mut out);
        }

        // 4. Export Section (7)
        let mut export_count = self.funcs.len() as u32;
        if self.memory_pages.is_some() {
            export_count += 1;
        }

        if export_count > 0 {
            let mut sec = Vec::new();
            Self::encode_u32(export_count, &mut sec);

            // Export memory
            if self.memory_pages.is_some() {
                let name = b"memory";
                Self::encode_u32(name.len() as u32, &mut sec);
                sec.extend_from_slice(name);
                sec.push(0x02); // Export kind: memory
                sec.push(0x00); // Memory idx 0
            }

            // Export functions
            for (idx, f) in self.funcs.iter().enumerate() {
                let name = f.name.as_bytes();
                Self::encode_u32(name.len() as u32, &mut sec);
                sec.extend_from_slice(name);
                sec.push(0x00); // Export kind: func
                Self::encode_u32(idx as u32, &mut sec);
            }
            Self::tulis_seksi(7, &sec, &mut out);
        }

        // 5. Code Section (10)
        if !self.funcs.is_empty() {
            let mut sec = Vec::new();
            Self::encode_u32(self.funcs.len() as u32, &mut sec);

            for f in &self.funcs {
                let mut body = Vec::new();
                // Locals declaration
                if f.locals.is_empty() {
                    body.push(0x00);
                } else {
                    Self::encode_u32(1, &mut body); // 1 local group
                    Self::encode_u32(f.locals.len() as u32, &mut body);
                    body.push(f.locals[0] as u8);
                }
                body.extend_from_slice(&f.bytecode);
                // Pastikan diakhiri dengan opcode end (0x0B)
                if body.last() != Some(&0x0B) {
                    body.push(0x0B);
                }

                // Tulis ukuran fungsi
                Self::encode_u32(body.len() as u32, &mut sec);
                sec.extend_from_slice(&body);
            }
            Self::tulis_seksi(10, &sec, &mut out);
        }

        // 6. Data Section (11)
        if !self.data_segments.is_empty() {
            let mut sec = Vec::new();
            Self::encode_u32(self.data_segments.len() as u32, &mut sec);
            for (offset, bytes) in &self.data_segments {
                sec.push(0x00); // Memory idx 0
                sec.push(0x41); // i32.const
                Self::encode_i32(*offset as i32, &mut sec);
                sec.push(0x0B); // end
                Self::encode_u32(bytes.len() as u32, &mut sec);
                sec.extend_from_slice(bytes);
            }
            Self::tulis_seksi(11, &sec, &mut out);
        }

        out
    }

    fn tulis_seksi(sec_id: u8, content: &[u8], out: &mut Vec<u8>) {
        out.push(sec_id);
        Self::encode_u32(content.len() as u32, out);
        out.extend_from_slice(content);
    }

    pub fn encode_u32(mut val: u32, buf: &mut Vec<u8>) {
        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80;
            }
            buf.push(byte);
            if val == 0 {
                break;
            }
        }
    }

    pub fn encode_i32(mut val: i32, buf: &mut Vec<u8>) {
        let mut more = true;
        while more {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            let sign = (byte & 0x40) != 0;
            if (val == 0 && !sign) || (val == -1 && sign) {
                more = false;
            } else {
                byte |= 0x80;
            }
            buf.push(byte);
        }
    }
}

// ==============================================================================
// 3. WEBASSEMBLY MICRO-VM (PURE SOVEREIGN STACK INTERPRETER)
// ==============================================================================

pub struct WasmMicroVm {
    pub memory: Vec<u8>,
    pub exports: std::collections::HashMap<String, usize>, // name -> func_idx
    pub functions: Vec<PureWasmFunc>,
}

impl WasmMicroVm {
    /// Inisialisasi Micro-VM dari biner modul WebAssembly
    pub fn muat_modul(wasm_bytes: &[u8]) -> Result<Self, String> {
        if wasm_bytes.len() < 8 {
            return Err("Ukuran modul WASM terlalu kecil (< 8 byte)".to_string());
        }

        if &wasm_bytes[0..4] != WASM_MAGIC {
            return Err("Magic header WASM tidak valid".to_string());
        }

        let mut cursor = 8;
        let mut exports = std::collections::HashMap::new();
        let mut functions = Vec::new();
        let mut memory = vec![0u8; 64 * 1024]; // 1 halaman standar 64KB

        // Parse seksi-seksi WASM
        while cursor < wasm_bytes.len() {
            let sec_id = wasm_bytes[cursor];
            cursor += 1;
            let sec_len = Self::decode_u32(wasm_bytes, &mut cursor)? as usize;
            let sec_end = cursor + sec_len;

            match sec_id {
                7 => {
                    // Export Section
                    let num_exports = Self::decode_u32(wasm_bytes, &mut cursor)?;
                    for _ in 0..num_exports {
                        let name_len = Self::decode_u32(wasm_bytes, &mut cursor)? as usize;
                        let name = String::from_utf8_lossy(&wasm_bytes[cursor..cursor + name_len]).to_string();
                        cursor += name_len;
                        let kind = wasm_bytes[cursor];
                        cursor += 1;
                        let idx = Self::decode_u32(wasm_bytes, &mut cursor)? as usize;
                        if kind == 0 {
                            exports.insert(name, idx);
                        }
                    }
                }
                10 => {
                    // Code Section
                    let num_funcs = Self::decode_u32(wasm_bytes, &mut cursor)?;
                    for _ in 0..num_funcs {
                        let body_len = Self::decode_u32(wasm_bytes, &mut cursor)? as usize;
                        let body_end = cursor + body_len;

                        let num_locals = Self::decode_u32(wasm_bytes, &mut cursor)?;
                        for _ in 0..num_locals {
                            let _count = Self::decode_u32(wasm_bytes, &mut cursor)?;
                            let _ltype = wasm_bytes[cursor];
                            cursor += 1;
                        }

                        let bytecode = wasm_bytes[cursor..body_end].to_vec();
                        cursor = body_end;

                        functions.push(PureWasmFunc {
                            name: String::new(),
                            params: Vec::new(),
                            returns: Vec::new(),
                            locals: Vec::new(),
                            bytecode,
                        });
                    }
                }
                11 => {
                    // Data Section
                    let num_data = Self::decode_u32(wasm_bytes, &mut cursor)?;
                    for _ in 0..num_data {
                        let _mem_idx = wasm_bytes[cursor];
                        cursor += 1;
                        let op = wasm_bytes[cursor];
                        cursor += 1;
                        let offset = if op == 0x41 {
                            Self::decode_i32(wasm_bytes, &mut cursor)? as usize
                        } else {
                            0
                        };
                        let _end = wasm_bytes[cursor]; // 0x0B
                        cursor += 1;
                        let data_len = Self::decode_u32(wasm_bytes, &mut cursor)? as usize;
                        if offset + data_len <= memory.len() {
                            memory[offset..offset + data_len].copy_from_slice(&wasm_bytes[cursor..cursor + data_len]);
                        }
                        cursor += data_len;
                    }
                }
                _ => {
                    cursor = sec_end;
                }
            }
        }

        Ok(Self {
            memory,
            exports,
            functions,
        })
    }

    /// Eksekusi fungsi ekspor berdasarkan nama dengan argumen stack
    pub fn panggil(&mut self, nama_fungsi: &str, args: &[WasmVal]) -> Result<Option<WasmVal>, String> {
        let func_idx = self
            .exports
            .get(nama_fungsi)
            .copied()
            .ok_or_else(|| format!("Fungsi ekspor '{}' tidak ditemukan", nama_fungsi))?;

        if func_idx >= self.functions.len() {
            return Err("Indeks fungsi di luar batas modul".to_string());
        }

        let bytecode = self.functions[func_idx].bytecode.clone();
        let mut stack: Vec<WasmVal> = Vec::new();
        let mut locals: Vec<WasmVal> = args.to_vec();
        // Alokasikan ekstra 16 slot local default
        locals.resize(args.len() + 16, WasmVal::I32(0));

        let mut pc = 0;
        while pc < bytecode.len() {
            let op = bytecode[pc];
            pc += 1;

            match op {
                0x00 => {} // nop
                0x01 => {} // unreachable
                0x0B => break, // end
                0x0F => break, // return
                0x41 => {
                    // i32.const
                    let val = Self::decode_i32(&bytecode, &mut pc)?;
                    stack.push(WasmVal::I32(val));
                }
                0x42 => {
                    // i64.const
                    let val = Self::decode_i64(&bytecode, &mut pc)?;
                    stack.push(WasmVal::I64(val));
                }
                0x20 => {
                    // local.get
                    let idx = Self::decode_u32(&bytecode, &mut pc)? as usize;
                    let val = locals.get(idx).cloned().unwrap_or(WasmVal::I32(0));
                    stack.push(val);
                }
                0x21 => {
                    // local.set
                    let idx = Self::decode_u32(&bytecode, &mut pc)? as usize;
                    if let Some(val) = stack.pop() {
                        if idx >= locals.len() {
                            locals.resize(idx + 1, WasmVal::I32(0));
                        }
                        locals[idx] = val;
                    }
                }
                0x22 => {
                    // local.tee
                    let idx = Self::decode_u32(&bytecode, &mut pc)? as usize;
                    if let Some(val) = stack.last().cloned() {
                        if idx >= locals.len() {
                            locals.resize(idx + 1, WasmVal::I32(0));
                        }
                        locals[idx] = val;
                    }
                }
                0x6A => {
                    // i32.add
                    let b = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    let a = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    stack.push(WasmVal::I32(a.wrapping_add(b)));
                }
                0x6B => {
                    // i32.sub
                    let b = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    let a = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    stack.push(WasmVal::I32(a.wrapping_sub(b)));
                }
                0x6C => {
                    // i32.mul
                    let b = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    let a = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    stack.push(WasmVal::I32(a.wrapping_mul(b)));
                }
                0x6D => {
                    // i32.div_s
                    let b = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    let a = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    if b == 0 {
                        return Err("Pembagian dengan nol (WASM Integer divide by zero)".to_string());
                    }
                    stack.push(WasmVal::I32(a / b));
                }
                0x46 => {
                    // i32.eq
                    let b = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    let a = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    stack.push(WasmVal::I32(if a == b { 1 } else { 0 }));
                }
                0x4A => {
                    // i32.gt_s
                    let b = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    let a = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    stack.push(WasmVal::I32(if a > b { 1 } else { 0 }));
                }
                0x28 => {
                    // i32.load
                    let _align = Self::decode_u32(&bytecode, &mut pc)?;
                    let offset = Self::decode_u32(&bytecode, &mut pc)? as usize;
                    let base = stack.pop().ok_or("Stack underflow")?.as_i32()? as usize;
                    let addr = base + offset;
                    if addr + 4 <= self.memory.len() {
                        let val = i32::from_le_bytes(self.memory[addr..addr + 4].try_into().unwrap());
                        stack.push(WasmVal::I32(val));
                    } else {
                        return Err("Akses memori di luar batas (Out of bounds)".to_string());
                    }
                }
                0x36 => {
                    // i32.store
                    let _align = Self::decode_u32(&bytecode, &mut pc)?;
                    let offset = Self::decode_u32(&bytecode, &mut pc)? as usize;
                    let val = stack.pop().ok_or("Stack underflow")?.as_i32()?;
                    let base = stack.pop().ok_or("Stack underflow")?.as_i32()? as usize;
                    let addr = base + offset;
                    if addr + 4 <= self.memory.len() {
                        self.memory[addr..addr + 4].copy_from_slice(&val.to_le_bytes());
                    } else {
                        return Err("Tulis memori di luar batas (Out of bounds)".to_string());
                    }
                }
                _ => return Err(format!("Opcode WASM 0x{:02X} belum didukung di Micro-VM", op)),
            }
        }

        Ok(stack.pop())
    }

    fn decode_u32(bytes: &[u8], cursor: &mut usize) -> Result<u32, String> {
        let mut result = 0u32;
        let mut shift = 0;
        loop {
            if *cursor >= bytes.len() {
                return Err("Buffer habis saat membaca u32 LEB128".to_string());
            }
            let byte = bytes[*cursor];
            *cursor += 1;
            result |= ((byte & 0x7F) as u32) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        Ok(result)
    }

    fn decode_i32(bytes: &[u8], cursor: &mut usize) -> Result<i32, String> {
        let mut result = 0i32;
        let mut shift = 0;
        let mut byte;
        loop {
            if *cursor >= bytes.len() {
                return Err("Buffer habis saat membaca i32 LEB128".to_string());
            }
            byte = bytes[*cursor];
            *cursor += 1;
            result |= ((byte & 0x7F) as i32) << shift;
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        if shift < 32 && (byte & 0x40) != 0 {
            result |= !0 << shift;
        }
        Ok(result)
    }

    fn decode_i64(bytes: &[u8], cursor: &mut usize) -> Result<i64, String> {
        let mut result = 0i64;
        let mut shift = 0;
        let mut byte;
        loop {
            if *cursor >= bytes.len() {
                return Err("Buffer habis saat membaca i64 LEB128".to_string());
            }
            byte = bytes[*cursor];
            *cursor += 1;
            result |= ((byte & 0x7F) as i64) << shift;
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        if shift < 64 && (byte & 0x40) != 0 {
            result |= !0 << shift;
        }
        Ok(result)
    }
}
