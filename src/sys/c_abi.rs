//! Sovereign C-ABI Memory Layout, Fixed-Width Types, Union, Bitfield & Raw Pointer Engine
//! Menyediakan kompatibilitas 100% setara Bahasa C (ISO C11/C23 System V AMD64 ABI):
//! - Tipe integer ukuran pasti (u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64)
//! - Tata letak struct C (`#[repr(C)]` dan `__attribute__((packed))`) beserta `sizeof`, `alignof`, `offsetof`
//! - Tata letak `union` C (memori tumpang-tindih pada offset 0 untuk type-punning) dan bitfield
//! - Manajemen memori heap mentah (`malloc`, `calloc`, `realloc`, `free`, `memcpy`, `memmove`, `memset`, `memcmp`)
//! - Dereferensi pointer mentah (`*ptr`), pembacaan/penulisan bertipe, dan operasi `volatile` perangkat keras.

use crate::error::{Galat, Span};
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, PartialEq)]
pub enum CType {
    Void,
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    Usize,
    Isize,
    F32,
    F64,
    Pointer(Box<CType>),
    FixedArray(Box<CType>, usize),
    Struct(String),
    Union(String),
}

impl CType {
    pub fn from_type_str(name: &str) -> Self {
        Self::from_name(name)
    }

    pub fn from_name(name: &str) -> Self {
        let trimmed = name.trim();
        if trimmed.starts_with('*') || trimmed.ends_with('*') {
            let inner = trimmed.trim_start_matches('*').trim_end_matches('*').trim();
            let inner_ty = if inner.is_empty() || inner == "void" || inner == "const char" || inner == "char" {
                CType::U8
            } else {
                Self::from_name(inner)
            };
            return CType::Pointer(Box::new(inner_ty));
        }
        // Check for array syntax: [u8; 16] or u8[16]
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let inner = &trimmed[1..trimmed.len() - 1];
            if let Some((ty_part, len_part)) = inner.split_once(';') {
                let len = len_part.trim().parse::<usize>().unwrap_or(1);
                return CType::FixedArray(Box::new(Self::from_name(ty_part)), len);
            }
        }
        if let Some((base, rest)) = trimmed.split_once('[') {
            if let Some(len_str) = rest.strip_suffix(']') {
                if let Ok(len) = len_str.trim().parse::<usize>() {
                    return CType::FixedArray(Box::new(Self::from_name(base)), len);
                }
            }
        }

        match trimmed {
            "void" | "kosong" => CType::Void,
            "bool" | "boolean" | "_Bool" => CType::Bool,
            "u8" | "uint8_t" | "unsigned char" | "byte" => CType::U8,
            "i8" | "int8_t" | "signed char" | "char" => CType::I8,
            "u16" | "uint16_t" | "unsigned short" | "ushort" => CType::U16,
            "i16" | "int16_t" | "short" => CType::I16,
            "u32" | "uint32_t" | "unsigned int" | "uint" => CType::U32,
            "i32" | "int32_t" | "int" => CType::I32,
            "u64" | "uint64_t" | "unsigned long" | "unsigned long long" | "ulong" => CType::U64,
            "i64" | "int64_t" | "long" | "long long" | "angka_bulat" => CType::I64,
            "usize" | "size_t" | "uintptr_t" => CType::Usize,
            "isize" | "ssize_t" | "ptrdiff_t" | "intptr_t" => CType::Isize,
            "f32" | "float" => CType::F32,
            "f64" | "double" | "desimal" | "angka" | "Angka" => CType::F64,
            "ptr" | "pointer" | "void*" => CType::Pointer(Box::new(CType::Void)),
            other => {
                // Check registered structs/unions
                let reg = get_c_type_registry().lock().unwrap();
                if reg.structs.contains_key(other) {
                    CType::Struct(other.to_string())
                } else if reg.unions.contains_key(other) {
                    CType::Union(other.to_string())
                } else {
                    CType::I32
                }
            }
        }
    }

    pub fn size_bytes(&self) -> usize {
        match self {
            CType::Void => 0,
            CType::Bool | CType::U8 | CType::I8 => 1,
            CType::U16 | CType::I16 => 2,
            CType::U32 | CType::I32 | CType::F32 => 4,
            CType::U64 | CType::I64 | CType::F64 | CType::Usize | CType::Isize | CType::Pointer(_) => 8,
            CType::FixedArray(inner, len) => inner.size_bytes() * (*len),
            CType::Struct(name) => {
                let reg = get_c_type_registry().lock().unwrap();
                reg.structs.get(name).map(|s| s.total_size).unwrap_or(8)
            }
            CType::Union(name) => {
                let reg = get_c_type_registry().lock().unwrap();
                reg.unions.get(name).map(|u| u.total_size).unwrap_or(8)
            }
        }
    }

    pub fn align_bytes(&self) -> usize {
        match self {
            CType::Void => 1,
            CType::Bool | CType::U8 | CType::I8 => 1,
            CType::U16 | CType::I16 => 2,
            CType::U32 | CType::I32 | CType::F32 => 4,
            CType::U64 | CType::I64 | CType::F64 | CType::Usize | CType::Isize | CType::Pointer(_) => 8,
            CType::FixedArray(inner, _) => inner.align_bytes(),
            CType::Struct(name) => {
                let reg = get_c_type_registry().lock().unwrap();
                reg.structs.get(name).map(|s| s.alignment).unwrap_or(8)
            }
            CType::Union(name) => {
                let reg = get_c_type_registry().lock().unwrap();
                reg.unions.get(name).map(|u| u.alignment).unwrap_or(8)
            }
        }
    }

    /// Membulatkan nilai numerik sesuai representasi bit two's complement C
    pub fn cast_value(&self, val: f64) -> f64 {
        match self {
            CType::Void => 0.0,
            CType::Bool => if val != 0.0 { 1.0 } else { 0.0 },
            CType::U8 => ((val as i64) as u8) as f64,
            CType::I8 => ((val as i64) as i8) as f64,
            CType::U16 => ((val as i64) as u16) as f64,
            CType::I16 => ((val as i64) as i16) as f64,
            CType::U32 => ((val as i64) as u32) as f64,
            CType::I32 => ((val as i64) as i32) as f64,
            CType::U64 | CType::Usize | CType::Pointer(_) => ((val as i64) as u64) as f64,
            CType::I64 | CType::Isize => (val as i64) as f64,
            CType::F32 => (val as f32) as f64,
            CType::F64 | CType::FixedArray(_, _) | CType::Struct(_) | CType::Union(_) => val,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CStructFieldLayout {
    pub name: String,
    pub ctype: CType,
    pub offset: usize,
    pub size: usize,
    pub bit_width: Option<u8>,
    pub bit_offset: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct CStructLayout {
    pub name: String,
    pub fields: Vec<CStructFieldLayout>,
    pub field_map: HashMap<String, CStructFieldLayout>,
    pub total_size: usize,
    pub alignment: usize,
    pub is_packed: bool,
}

impl CStructLayout {
    pub fn compute(name: &str, field_specs: &[(String, CType, Option<u8>)], is_packed: bool) -> Self {
        let mut current_offset = 0usize;
        let mut max_align = 1usize;
        let mut fields = Vec::new();
        let mut field_map = HashMap::new();

        let mut active_bitfield_offset: Option<usize> = None;
        let mut active_bitfield_used_bits: u8 = 0;
        let mut active_bitfield_unit_bytes: usize = 0;

        for (fname, ftype, bit_width) in field_specs {
            let sz = ftype.size_bytes();
            let align = if is_packed { 1 } else { ftype.align_bytes().max(1) };
            if align > max_align {
                max_align = align;
            }

            if let Some(bits) = bit_width {
                let unit_bits = (sz * 8) as u8;
                if let Some(bf_off) = active_bitfield_offset {
                    if active_bitfield_unit_bytes == sz && active_bitfield_used_bits + *bits <= unit_bits {
                        let layout = CStructFieldLayout {
                            name: fname.clone(),
                            ctype: ftype.clone(),
                            offset: bf_off,
                            size: sz,
                            bit_width: Some(*bits),
                            bit_offset: Some(active_bitfield_used_bits),
                        };
                        active_bitfield_used_bits += *bits;
                        field_map.insert(fname.clone(), layout.clone());
                        fields.push(layout);
                        continue;
                    }
                }
                // Start new bitfield storage unit
                if !is_packed && current_offset % align != 0 {
                    current_offset += align - (current_offset % align);
                }
                let bf_off = current_offset;
                current_offset += sz;
                active_bitfield_offset = Some(bf_off);
                active_bitfield_used_bits = *bits;
                active_bitfield_unit_bytes = sz;

                let layout = CStructFieldLayout {
                    name: fname.clone(),
                    ctype: ftype.clone(),
                    offset: bf_off,
                    size: sz,
                    bit_width: Some(*bits),
                    bit_offset: Some(0),
                };
                field_map.insert(fname.clone(), layout.clone());
                fields.push(layout);
            } else {
                active_bitfield_offset = None;
                active_bitfield_used_bits = 0;

                if !is_packed && current_offset % align != 0 {
                    current_offset += align - (current_offset % align);
                }
                let layout = CStructFieldLayout {
                    name: fname.clone(),
                    ctype: ftype.clone(),
                    offset: current_offset,
                    size: sz,
                    bit_width: None,
                    bit_offset: None,
                };
                current_offset += sz;
                field_map.insert(fname.clone(), layout.clone());
                fields.push(layout);
            }
        }

        if !is_packed && current_offset % max_align != 0 {
            current_offset += max_align - (current_offset % max_align);
        }

        Self {
            name: name.to_string(),
            fields,
            field_map,
            total_size: current_offset.max(if field_specs.is_empty() { 0 } else { 1 }),
            alignment: if is_packed { 1 } else { max_align },
            is_packed,
        }
    }

    pub fn offset_of(&self, field_name: &str) -> Option<usize> {
        self.field_map.get(field_name).map(|f| f.offset)
    }
}

#[derive(Debug, Clone)]
pub struct CUnionLayout {
    pub name: String,
    pub fields: HashMap<String, CType>,
    pub total_size: usize,
    pub alignment: usize,
}

impl CUnionLayout {
    pub fn compute(name: &str, field_specs: &[(String, CType)]) -> Self {
        let mut max_size = 1usize;
        let mut max_align = 1usize;
        let mut fields = HashMap::new();
        for (fname, ftype) in field_specs {
            let sz = ftype.size_bytes();
            let align = ftype.align_bytes();
            if sz > max_size {
                max_size = sz;
            }
            if align > max_align {
                max_align = align;
            }
            fields.insert(fname.clone(), ftype.clone());
        }
        if max_size % max_align != 0 {
            max_size += max_align - (max_size % max_align);
        }
        Self {
            name: name.to_string(),
            fields,
            total_size: max_size,
            alignment: max_align,
        }
    }
}

#[derive(Default)]
pub struct CTypeRegistry {
    pub structs: HashMap<String, CStructLayout>,
    pub unions: HashMap<String, CUnionLayout>,
    pub typedefs: HashMap<String, CType>,
}

impl CTypeRegistry {
    pub fn get_struct(&self, name: &str) -> Option<&CStructLayout> {
        self.structs.get(name)
    }

    pub fn get_union(&self, name: &str) -> Option<&CUnionLayout> {
        self.unions.get(name)
    }

    pub fn register_struct(&mut self, layout: CStructLayout) {
        self.structs.insert(layout.name.clone(), layout);
    }

    pub fn register_union(&mut self, layout: CUnionLayout) {
        self.unions.insert(layout.name.clone(), layout);
    }
}

pub fn get_c_type_registry() -> &'static Mutex<CTypeRegistry> {
    static REGISTRY: OnceLock<Mutex<CTypeRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(CTypeRegistry::default()))
}

/// Manajer Memori Mentah C (Mendukung Pointer Asli di Ruang Alamat Proses + Pemetaan MMIO)
pub struct RawMemoryManager {
    pub allocations: HashMap<usize, Box<[u8]>>,
    pub mmio_registers: HashMap<usize, u64>,
}

impl Default for RawMemoryManager {
    fn default() -> Self {
        Self {
            allocations: HashMap::new(),
            mmio_registers: HashMap::new(),
        }
    }
}

pub fn get_raw_memory_manager() -> &'static Mutex<RawMemoryManager> {
    static MEM_MGR: OnceLock<Mutex<RawMemoryManager>> = OnceLock::new();
    MEM_MGR.get_or_init(|| Mutex::new(RawMemoryManager::default()))
}

impl RawMemoryManager {
    pub fn malloc(&mut self, size: usize) -> usize {
        let actual_size = size.max(1);
        let mut buf = vec![0u8; actual_size].into_boxed_slice();
        let ptr = buf.as_mut_ptr() as usize;
        self.allocations.insert(ptr, buf);
        ptr
    }

    pub fn calloc(&mut self, nmemb: usize, size: usize) -> usize {
        self.malloc(nmemb.saturating_mul(size))
    }

    pub fn realloc(&mut self, old_ptr: usize, new_size: usize) -> usize {
        let actual_new = new_size.max(1);
        let mut new_buf = vec![0u8; actual_new].into_boxed_slice();
        if let Some(old_buf) = self.allocations.remove(&old_ptr) {
            let copy_len = old_buf.len().min(actual_new);
            new_buf[..copy_len].copy_from_slice(&old_buf[..copy_len]);
        }
        let new_ptr = new_buf.as_mut_ptr() as usize;
        self.allocations.insert(new_ptr, new_buf);
        new_ptr
    }

    pub fn free(&mut self, ptr: usize) -> bool {
        self.allocations.remove(&ptr).is_some()
    }

    /// Periksa apakah alamat `addr..addr+len` berada di dalam salah satu blok yang dialokasikan
    pub fn contains_range(&self, addr: usize, len: usize) -> bool {
        for (&base, buf) in &self.allocations {
            if addr >= base && addr.saturating_add(len) <= base.saturating_add(buf.len()) {
                return true;
            }
        }
        false
    }

    pub fn read_typed(&self, addr: usize, ctype: &CType) -> Result<f64, String> {
        let sz = ctype.size_bytes();
        if self.contains_range(addr, sz) {
            unsafe {
                let p = addr as *const u8;
                let val = match ctype {
                    CType::Bool | CType::U8 => *p as f64,
                    CType::I8 => *(p as *const i8) as f64,
                    CType::U16 => std::ptr::read_unaligned(p as *const u16) as f64,
                    CType::I16 => std::ptr::read_unaligned(p as *const i16) as f64,
                    CType::U32 => std::ptr::read_unaligned(p as *const u32) as f64,
                    CType::I32 => std::ptr::read_unaligned(p as *const i32) as f64,
                    CType::U64 | CType::Usize | CType::Pointer(_) => {
                        std::ptr::read_unaligned(p as *const u64) as f64
                    }
                    CType::I64 | CType::Isize => std::ptr::read_unaligned(p as *const i64) as f64,
                    CType::F32 => std::ptr::read_unaligned(p as *const f32) as f64,
                    CType::F64 => std::ptr::read_unaligned(p as *const f64),
                    _ => *p as f64,
                };
                Ok(val)
            }
        } else if let Some(&reg_val) = self.mmio_registers.get(&addr) {
            Ok(ctype.cast_value(reg_val as f64))
        } else {
            Err(format!("Segmentation fault: Alamat pointer 0x{:X} tidak terpetakan", addr))
        }
    }

    pub fn write_typed(&mut self, addr: usize, val: f64, ctype: &CType) -> Result<(), String> {
        let sz = ctype.size_bytes();
        if self.contains_range(addr, sz) {
            unsafe {
                let p = addr as *mut u8;
                match ctype {
                    CType::Bool => *p = if val != 0.0 { 1 } else { 0 },
                    CType::U8 => *p = (val as i64) as u8,
                    CType::I8 => *(p as *mut i8) = (val as i64) as i8,
                    CType::U16 => std::ptr::write_unaligned(p as *mut u16, (val as i64) as u16),
                    CType::I16 => std::ptr::write_unaligned(p as *mut i16, (val as i64) as i16),
                    CType::U32 => std::ptr::write_unaligned(p as *mut u32, (val as i64) as u32),
                    CType::I32 => std::ptr::write_unaligned(p as *mut i32, (val as i64) as i32),
                    CType::U64 | CType::Usize | CType::Pointer(_) => {
                        std::ptr::write_unaligned(p as *mut u64, (val as i64) as u64)
                    }
                    CType::I64 | CType::Isize => {
                        std::ptr::write_unaligned(p as *mut i64, val as i64)
                    }
                    CType::F32 => std::ptr::write_unaligned(p as *mut f32, val as f32),
                    CType::F64 => std::ptr::write_unaligned(p as *mut f64, val),
                    _ => *p = (val as i64) as u8,
                }
            }
            Ok(())
        } else {
            // Simpan di tabel register MMIO (misal 0x40020014)
            self.mmio_registers.insert(addr, (val as i64) as u64);
            Ok(())
        }
    }

    pub fn write_u8(&mut self, addr: usize, val: u8) -> Result<(), String> {
        self.write_typed(addr, val as f64, &CType::U8)
    }

    pub fn read_u8(&self, addr: usize) -> Result<u8, String> {
        self.read_typed(addr, &CType::U8).map(|v| v as u8)
    }

    pub fn memcpy(&mut self, dst: usize, src: usize, n: usize) -> Result<(), String> {
        if n == 0 {
            return Ok(());
        }
        if self.contains_range(dst, n) && self.contains_range(src, n) {
            unsafe {
                std::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n);
            }
            Ok(())
        } else {
            Err("Segmentation fault pada memcpy".to_string())
        }
    }

    pub fn memmove(&mut self, dst: usize, src: usize, n: usize) -> Result<(), String> {
        if n == 0 {
            return Ok(());
        }
        if self.contains_range(dst, n) && self.contains_range(src, n) {
            unsafe {
                std::ptr::copy(src as *const u8, dst as *mut u8, n);
            }
            Ok(())
        } else {
            Err("Segmentation fault pada memmove".to_string())
        }
    }

    pub fn memset(&mut self, dst: usize, val: u8, n: usize) -> Result<(), String> {
        if n == 0 {
            return Ok(());
        }
        if self.contains_range(dst, n) {
            unsafe {
                std::ptr::write_bytes(dst as *mut u8, val, n);
            }
            Ok(())
        } else {
            Err("Segmentation fault pada memset".to_string())
        }
    }

    pub fn memcmp(&self, a: usize, b: usize, n: usize) -> Result<i32, String> {
        if n == 0 {
            return Ok(0);
        }
        if self.contains_range(a, n) && self.contains_range(b, n) {
            let sa = unsafe { std::slice::from_raw_parts(a as *const u8, n) };
            let sb = unsafe { std::slice::from_raw_parts(b as *const u8, n) };
            for i in 0..n {
                if sa[i] != sb[i] {
                    return Ok((sa[i] as i32) - (sb[i] as i32));
                }
            }
            Ok(0)
        } else {
            Err("Segmentation fault pada memcmp".to_string())
        }
    }
}

/// Membaca nilai dari byte buffer C Union sesuai tipe field yang diminta
pub fn read_bytes_as_ctype(bytes: &[u8], offset: usize, ctype: &CType) -> Value {
    if offset >= bytes.len() {
        return Value::Number(0.0);
    }
    let slice = &bytes[offset..];
    match ctype {
        CType::Bool => Value::Bool(slice[0] != 0),
        CType::U8 => Value::Number(slice[0] as f64),
        CType::I8 => Value::Number((slice[0] as i8) as f64),
        CType::U16 if slice.len() >= 2 => {
            Value::Number(u16::from_le_bytes([slice[0], slice[1]]) as f64)
        }
        CType::I16 if slice.len() >= 2 => {
            Value::Number(i16::from_le_bytes([slice[0], slice[1]]) as f64)
        }
        CType::U32 if slice.len() >= 4 => {
            Value::Number(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]) as f64)
        }
        CType::I32 if slice.len() >= 4 => {
            Value::Number(i32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]) as f64)
        }
        CType::F32 if slice.len() >= 4 => {
            Value::Number(f32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]) as f64)
        }
        CType::U64 | CType::Usize | CType::Pointer(_) if slice.len() >= 8 => {
            let mut b = [0u8; 8];
            b.copy_from_slice(&slice[..8]);
            Value::Number(u64::from_le_bytes(b) as f64)
        }
        CType::I64 | CType::Isize if slice.len() >= 8 => {
            let mut b = [0u8; 8];
            b.copy_from_slice(&slice[..8]);
            Value::Number(i64::from_le_bytes(b) as f64)
        }
        CType::F64 if slice.len() >= 8 => {
            let mut b = [0u8; 8];
            b.copy_from_slice(&slice[..8]);
            Value::Number(f64::from_le_bytes(b))
        }
        CType::FixedArray(inner, len) => {
            let elem_sz = inner.size_bytes().max(1);
            let mut arr = Vec::with_capacity(*len);
            for i in 0..*len {
                arr.push(read_bytes_as_ctype(slice, i * elem_sz, inner));
            }
            Value::Array(Rc::new(RefCell::new(arr)))
        }
        _ => Value::Number(slice[0] as f64),
    }
}

/// Menulis nilai ke byte buffer C Union / Struct sesuai tipe field
pub fn write_ctype_to_bytes(bytes: &mut [u8], offset: usize, ctype: &CType, val: &Value) {
    if offset >= bytes.len() {
        return;
    }
    let slice = &mut bytes[offset..];
    let num = match val {
        Value::Number(n) => *n,
        Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        _ => 0.0,
    };
    match ctype {
        CType::Bool => slice[0] = if num != 0.0 { 1 } else { 0 },
        CType::U8 => slice[0] = (num as i64) as u8,
        CType::I8 => slice[0] = ((num as i64) as i8) as u8,
        CType::U16 if slice.len() >= 2 => {
            slice[..2].copy_from_slice(&((num as i64) as u16).to_le_bytes());
        }
        CType::I16 if slice.len() >= 2 => {
            slice[..2].copy_from_slice(&((num as i64) as i16).to_le_bytes());
        }
        CType::U32 if slice.len() >= 4 => {
            slice[..4].copy_from_slice(&((num as i64) as u32).to_le_bytes());
        }
        CType::I32 if slice.len() >= 4 => {
            slice[..4].copy_from_slice(&((num as i64) as i32).to_le_bytes());
        }
        CType::F32 if slice.len() >= 4 => {
            slice[..4].copy_from_slice(&(num as f32).to_le_bytes());
        }
        CType::U64 | CType::Usize | CType::Pointer(_) if slice.len() >= 8 => {
            slice[..8].copy_from_slice(&((num as i64) as u64).to_le_bytes());
        }
        CType::I64 | CType::Isize if slice.len() >= 8 => {
            slice[..8].copy_from_slice(&(num as i64).to_le_bytes());
        }
        CType::F64 if slice.len() >= 8 => {
            slice[..8].copy_from_slice(&num.to_le_bytes());
        }
        CType::FixedArray(inner, len) => {
            if let Value::Array(arr) = val {
                let elem_sz = inner.size_bytes().max(1);
                for (i, item) in arr.borrow().iter().take(*len).enumerate() {
                    write_ctype_to_bytes(slice, i * elem_sz, inner, item);
                }
            }
        }
        _ => {}
    }
}

/// Membuat instance C Union interaktif berbasis buffer byte bersama (shared byte buffer)
pub fn create_c_union_instance(union_name: &str) -> Result<Value, Galat> {
    let layout = {
        let reg = get_c_type_registry().lock().unwrap();
        reg.unions.get(union_name).cloned()
    };
    let layout = layout.ok_or_else(|| {
        Galat::runtime_polos(format!("Union C '{}' belum didefinisikan", union_name))
    })?;

    let mut map = HashMap::new();
    map.insert("_tipe_c_union".to_string(), Value::String(layout.name.clone()));
    map.insert("_ukuran_c".to_string(), Value::Number(layout.total_size as f64));
    map.insert("_perataan_c".to_string(), Value::Number(layout.alignment as f64));
    let raw_bytes: Vec<Value> = vec![Value::Number(0.0); layout.total_size];
    map.insert("_bytes".to_string(), Value::Array(Rc::new(RefCell::new(raw_bytes))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}
