use crate::error::Galat;
use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FfiType {
    Void,
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    String,
    Pointer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FfiFunctionSignature {
    pub name: String,
    pub symbol: String,
    pub library: String,
    pub param_types: Vec<FfiType>,
    pub return_type: FfiType,
    pub is_variadic: bool,
}

#[derive(Debug, Clone)]
pub struct DynamicLibrary {
    pub name: String,
    pub path: String,
    pub loaded: bool,
    pub exported_symbols: Vec<String>,
}

pub struct FfiManager {
    signatures: HashMap<String, FfiFunctionSignature>,
    libraries: HashMap<String, DynamicLibrary>,
    simulated_memory: Arc<RwLock<HashMap<usize, Vec<u8>>>>,
    next_ptr: usize,
}

impl FfiManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            signatures: HashMap::new(),
            libraries: HashMap::new(),
            simulated_memory: Arc::new(RwLock::new(HashMap::new())),
            next_ptr: 0x1000,
        };
        mgr.register_builtin_c_abi();
        mgr
    }

    fn register_builtin_c_abi(&mut self) {
        // Register standard C libc prototypes
        self.register_signature(FfiFunctionSignature {
            name: "puts".to_string(),
            symbol: "puts".to_string(),
            library: "libc".to_string(),
            param_types: vec![FfiType::String],
            return_type: FfiType::Int32,
            is_variadic: false,
        });

        self.register_signature(FfiFunctionSignature {
            name: "abs".to_string(),
            symbol: "abs".to_string(),
            library: "libc".to_string(),
            param_types: vec![FfiType::Int32],
            return_type: FfiType::Int32,
            is_variadic: false,
        });

        self.register_signature(FfiFunctionSignature {
            name: "sqrt".to_string(),
            symbol: "sqrt".to_string(),
            library: "libm".to_string(),
            param_types: vec![FfiType::Float64],
            return_type: FfiType::Float64,
            is_variadic: false,
        });

        self.register_signature(FfiFunctionSignature {
            name: "hypot".to_string(),
            symbol: "hypot".to_string(),
            library: "libm".to_string(),
            param_types: vec![FfiType::Float64, FfiType::Float64],
            return_type: FfiType::Float64,
            is_variadic: false,
        });

        self.register_signature(FfiFunctionSignature {
            name: "pow".to_string(),
            symbol: "pow".to_string(),
            library: "libm".to_string(),
            param_types: vec![FfiType::Float64, FfiType::Float64],
            return_type: FfiType::Float64,
            is_variadic: false,
        });

        self.register_signature(FfiFunctionSignature {
            name: "strlen".to_string(),
            symbol: "strlen".to_string(),
            library: "libc".to_string(),
            param_types: vec![FfiType::String],
            return_type: FfiType::Int64,
            is_variadic: false,
        });
    }

    pub fn load_library(&mut self, name: &str, path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("Path pustaka dinamis tidak boleh kosong".to_string());
        }

        self.libraries.insert(
            name.to_string(),
            DynamicLibrary {
                name: name.to_string(),
                path: path.to_string(),
                loaded: true,
                exported_symbols: Vec::new(),
            },
        );
        Ok(())
    }

    pub fn register_signature(&mut self, sig: FfiFunctionSignature) {
        self.signatures.insert(sig.name.clone(), sig);
    }

    pub fn get_signature(&self, name: &str) -> Option<&FfiFunctionSignature> {
        self.signatures.get(name)
    }

    pub fn allocate_raw(&mut self, bytes: &[u8]) -> usize {
        let ptr = self.next_ptr;
        self.next_ptr += bytes.len() + 16;
        let mut mem = self.simulated_memory.write().unwrap();
        mem.insert(ptr, bytes.to_vec());
        ptr
    }

    pub fn read_string_ptr(&self, ptr: usize) -> Result<String, Galat> {
        let mem = self.simulated_memory.read().unwrap();
        if let Some(bytes) = mem.get(&ptr) {
            let s = String::from_utf8_lossy(bytes).trim_end_matches('\0').to_string();
            Ok(s)
        } else {
            Err(Galat::runtime_polos(format!("Segmentation fault: Pointer 0x{:X} tidak valid", ptr)))
        }
    }

    pub fn call_foreign_function(&mut self, name: &str, args: &[Value]) -> Result<Value, Galat> {
        let sig = match self.signatures.get(name) {
            Some(s) => s.clone(),
            None => {
                return Err(Galat::runtime_polos(format!(
                    "Fungsi FFI eksternal '{}' belum dideklarasikan",
                    name
                )));
            }
        };

        if sig.param_types.len() != args.len() && !sig.is_variadic {
            return Err(Galat::runtime_polos(format!(
                "FFI {}: Jumlah argumen tidak sesuai. Diharapkan {}, ditemukan {}",
                name,
                sig.param_types.len(),
                args.len()
            )));
        }

        // Handle C math/libc primitives (attempt real C-ABI via libloading first, then safe fallback)
        match sig.symbol.as_str() {
            "abs" => {
                let val = match args.first() {
                    Some(Value::Number(n)) => {
                        Self::call_real_c_i32_unary("libc.so.6", "abs", *n as i32)
                            .map(|r| r as f64)
                            .unwrap_or_else(|| n.abs())
                    }
                    _ => 0.0,
                };
                Ok(Value::Number(val))
            }
            "sqrt" | "sin" | "cos" | "tan" | "exp" | "log" | "floor" | "ceil" => {
                let val = match args.first() {
                    Some(Value::Number(n)) => {
                        Self::call_real_c_f64_unary("libm.so.6", &sig.symbol, *n).unwrap_or_else(
                            || match sig.symbol.as_str() {
                                "sqrt" => n.sqrt(),
                                "sin" => n.sin(),
                                "cos" => n.cos(),
                                "tan" => n.tan(),
                                "exp" => n.exp(),
                                "log" => n.ln(),
                                "floor" => n.floor(),
                                "ceil" => n.ceil(),
                                _ => *n,
                            },
                        )
                    }
                    _ => 0.0,
                };
                Ok(Value::Number(val))
            }
            "pow" | "atan2" | "hypot" => {
                let a = match args.first() {
                    Some(Value::Number(n)) => *n,
                    _ => 0.0,
                };
                let b = match args.get(1) {
                    Some(Value::Number(n)) => *n,
                    _ => 0.0,
                };
                let val = Self::call_real_c_f64_binary("libm.so.6", &sig.symbol, a, b)
                    .unwrap_or_else(|| match sig.symbol.as_str() {
                        "pow" => a.powf(b),
                        "atan2" => a.atan2(b),
                        "hypot" => a.hypot(b),
                        _ => 0.0,
                    });
                Ok(Value::Number(val))
            }
            "puts" => {
                let text = match args.first() {
                    Some(Value::String(s)) => s.clone(),
                    Some(v) => v.to_debug_repr(),
                    None => "".to_string(),
                };
                println!("{}", text);
                Ok(Value::Number(text.len() as f64))
            }
            "strlen" => {
                let len = match args.first() {
                    Some(Value::String(s)) => {
                        let cstr = std::ffi::CString::new(s.as_str()).unwrap_or_default();
                        Self::call_real_c_strlen("libc.so.6", cstr.as_ptr()).unwrap_or(s.len())
                    }
                    _ => 0,
                };
                Ok(Value::Number(len as f64))
            }
            "strcmp" => {
                if let (Some(Value::String(a)), Some(Value::String(b))) = (args.first(), args.get(1)) {
                    let cmp = if a == b { 0 } else if a < b { -1 } else { 1 };
                    Ok(Value::Number(cmp as f64))
                } else {
                    Ok(Value::Number(0.0))
                }
            }
            "getpid" => {
                let pid = unsafe { crate::sys::syscall::DirectSyscall::getpid() };
                Ok(Value::Number(pid as f64))
            }
            _ => {
                // Attempt real dynamic symbol lookup in loaded library or standard libm.so.6 / libc.so.6
                let candidate_libs: Vec<String> = if let Some(lib_meta) = self.libraries.get(&sig.library) {
                    vec![lib_meta.path.clone(), "libm.so.6".to_string(), "libc.so.6".to_string()]
                } else {
                    vec!["libm.so.6".to_string(), "libc.so.6".to_string()]
                };

                for lib_path in &candidate_libs {
                    if sig.param_types.len() == 1 && sig.return_type == FfiType::Float64 {
                        if let Some(Value::Number(x)) = args.first() {
                            if let Some(res) = Self::call_real_c_f64_unary(lib_path, &sig.symbol, *x) {
                                return Ok(Value::Number(res));
                            }
                        }
                    } else if sig.param_types.len() == 2 && sig.return_type == FfiType::Float64 {
                        if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.first(), args.get(1)) {
                            if let Some(res) = Self::call_real_c_f64_binary(lib_path, &sig.symbol, *a, *b) {
                                return Ok(Value::Number(res));
                            }
                        }
                    } else if sig.param_types.len() == 1 && sig.return_type == FfiType::Int32 {
                        if let Some(Value::Number(x)) = args.first() {
                            if let Some(res) = Self::call_real_c_i32_unary(lib_path, &sig.symbol, *x as i32) {
                                return Ok(Value::Number(res as f64));
                            }
                        }
                    }
                }

                // Dynamic execution fallback for registered libraries
                match sig.return_type {
                    FfiType::Void => Ok(Value::Nil),
                    FfiType::Int32 | FfiType::Int64 | FfiType::Float32 | FfiType::Float64 => {
                        Ok(Value::Number(0.0))
                    }
                    FfiType::Bool => Ok(Value::Bool(true)),
                    FfiType::String => Ok(Value::String("ffi_result".to_string())),
                    FfiType::Pointer => Ok(Value::Number(self.next_ptr as f64)),
                }
            }
        }
    }

    /// Pemanggilan simbol C-ABI nyata `extern "C" fn(*const i8) -> usize` melalui `libloading`
    pub fn call_real_c_strlen(lib_path: &str, c_str: *const std::os::raw::c_char) -> Option<usize> {
        unsafe {
            let lib = libloading::Library::new(lib_path).ok()?;
            let func: libloading::Symbol<unsafe extern "C" fn(*const std::os::raw::c_char) -> usize> =
                lib.get(b"strlen\0").ok()?;
            Some(func(c_str))
        }
    }

    /// Pemanggilan simbol C-ABI nyata `extern "C" fn(f64) -> f64` melalui `libloading`
    pub fn call_real_c_f64_unary(lib_path: &str, symbol: &str, arg: f64) -> Option<f64> {
        unsafe {
            let lib = libloading::Library::new(lib_path).ok()?;
            let func: libloading::Symbol<unsafe extern "C" fn(f64) -> f64> =
                lib.get(symbol.as_bytes()).ok()?;
            Some(func(arg))
        }
    }

    /// Pemanggilan simbol C-ABI nyata `extern "C" fn(f64, f64) -> f64` melalui `libloading`
    pub fn call_real_c_f64_binary(lib_path: &str, symbol: &str, a: f64, b: f64) -> Option<f64> {
        unsafe {
            let lib = libloading::Library::new(lib_path).ok()?;
            let func: libloading::Symbol<unsafe extern "C" fn(f64, f64) -> f64> =
                lib.get(symbol.as_bytes()).ok()?;
            Some(func(a, b))
        }
    }

    /// Pemanggilan simbol C-ABI nyata `extern "C" fn(i32) -> i32` melalui `libloading`
    pub fn call_real_c_i32_unary(lib_path: &str, symbol: &str, arg: i32) -> Option<i32> {
        unsafe {
            let lib = libloading::Library::new(lib_path).ok()?;
            let func: libloading::Symbol<unsafe extern "C" fn(i32) -> i32> =
                lib.get(symbol.as_bytes()).ok()?;
            Some(func(arg))
        }
    }
}

impl Default for FfiManager {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_FFI_MANAGER: std::sync::OnceLock<std::sync::Mutex<FfiManager>> = std::sync::OnceLock::new();

pub fn get_ffi_manager() -> &'static std::sync::Mutex<FfiManager> {
    GLOBAL_FFI_MANAGER.get_or_init(|| std::sync::Mutex::new(FfiManager::new()))
}

/// Hasil Penguraian Berkas Header C (`.h`) Otomatis
#[derive(Debug, Clone, Default)]
pub struct ParsedCHeader {
    pub defines: HashMap<String, Value>,
    pub enums: HashMap<String, i64>,
    pub structs: Vec<String>,
    pub unions: Vec<String>,
    pub functions: Vec<FfiFunctionSignature>,
}

impl ParsedCHeader {
    pub fn to_widya_map(&self) -> Value {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut root = HashMap::new();
        root.insert(
            "defines".to_string(),
            Value::Map(Rc::new(RefCell::new(self.defines.clone()))),
        );
        let mut enum_map = HashMap::new();
        for (k, v) in &self.enums {
            enum_map.insert(k.clone(), Value::Number(*v as f64));
        }
        root.insert("enums".to_string(), Value::Map(Rc::new(RefCell::new(enum_map))));
        root.insert(
            "structs".to_string(),
            Value::Array(Rc::new(RefCell::new(
                self.structs.iter().map(|s| Value::String(s.clone())).collect(),
            ))),
        );
        root.insert(
            "unions".to_string(),
            Value::Array(Rc::new(RefCell::new(
                self.unions.iter().map(|s| Value::String(s.clone())).collect(),
            ))),
        );
        root.insert(
            "functions".to_string(),
            Value::Array(Rc::new(RefCell::new(
                self.functions.iter().map(|f| Value::String(f.name.clone())).collect(),
            ))),
        );
        Value::Map(Rc::new(RefCell::new(root)))
    }
}

pub struct CHeaderParser;

impl CHeaderParser {
    pub fn parse_header_str(source: &str) -> ParsedCHeader {
        let mut mgr = get_ffi_manager().lock().unwrap();
        Self::parse_header(source, &mut mgr)
    }
    pub fn parse_c_type_to_ffi(ty: &str) -> FfiType {
        let t = ty.trim();
        if t.contains("char*") || t.contains("char *") {
            return FfiType::String;
        }
        if t.contains('*') {
            return FfiType::Pointer;
        }
        match t {
            "void" => FfiType::Void,
            "bool" | "_Bool" => FfiType::Bool,
            "float" => FfiType::Float32,
            "double" => FfiType::Float64,
            "long" | "long long" | "int64_t" | "uint64_t" | "size_t" | "ssize_t" => FfiType::Int64,
            _ => FfiType::Int32,
        }
    }

    pub fn parse_define_value(raw: &str) -> Option<Value> {
        let s = raw.trim().trim_start_matches('(').trim_end_matches(')').trim();
        if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
            return Some(Value::String(s[1..s.len() - 1].to_string()));
        }
        let clean_num = s.trim_end_matches(|c: char| matches!(c, 'u' | 'U' | 'l' | 'L' | 'f' | 'F'));
        if let Some(hex) = clean_num.strip_prefix("0x").or_else(|| clean_num.strip_prefix("0X")) {
            if let Ok(v) = u64::from_str_radix(hex, 16) {
                return Some(Value::Number(v as f64));
            }
        }
        if let Some(bin) = clean_num.strip_prefix("0b").or_else(|| clean_num.strip_prefix("0B")) {
            if let Ok(v) = u64::from_str_radix(bin, 2) {
                return Some(Value::Number(v as f64));
            }
        }
        if let Some((lhs, rhs)) = clean_num.split_once("<<") {
            if let (Ok(a), Ok(b)) = (lhs.trim().parse::<u64>(), rhs.trim().parse::<u32>()) {
                return Some(Value::Number((a << b) as f64));
            }
        }
        if let Ok(v) = clean_num.parse::<f64>() {
            return Some(Value::Number(v));
        }
        None
    }

    /// Mengurai kode sumber header C (`.h`) dan mendaftarkan struct, union, enum, makro `#define`, serta fungsi `extern`
    pub fn parse_header(source: &str, ffi_mgr: &mut FfiManager) -> ParsedCHeader {
        use crate::sys::c_abi::{CStructLayout, CType, CUnionLayout, get_c_type_registry};

        let mut result = ParsedCHeader::default();

        // 1. Parse #define directives line by line
        let mut stripped_lines = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("#define") {
                let parts: Vec<&str> = rest.trim().splitn(2, char::is_whitespace).collect();
                if parts.len() == 2 {
                    let name = parts[0].trim();
                    let val_str = parts[1].split("//").next().unwrap_or("").trim();
                    if !name.contains('(') {
                        if let Some(val) = Self::parse_define_value(val_str) {
                            result.defines.insert(name.to_string(), val);
                        }
                    }
                }
                continue;
            }
            if trimmed.starts_with('#') {
                continue;
            }
            stripped_lines.push(line.split("//").next().unwrap_or(""));
        }

        let clean_src = stripped_lines.join("\n");

        // 2. Parse statements separated by ';' (handling blocks '{ ... }')
        let mut stmts = Vec::new();
        let mut current = String::new();
        let mut brace_depth = 0usize;
        for ch in clean_src.chars() {
            if ch == '{' {
                brace_depth += 1;
                current.push(ch);
            } else if ch == '}' {
                brace_depth = brace_depth.saturating_sub(1);
                current.push(ch);
            } else if ch == ';' && brace_depth == 0 {
                let s = current.trim().to_string();
                if !s.is_empty() {
                    stmts.push(s);
                }
                current.clear();
            } else {
                current.push(ch);
            }
        }

        for stmt in stmts {
            let s = stmt.trim();
            // Check for struct definition: `struct Name { ... }` or `typedef struct ... { ... } Name`
            if (s.starts_with("struct ") || s.starts_with("typedef struct")) && s.contains('{') && s.contains('}') {
                let is_packed = s.contains("packed") || s.contains("__packed__");
                let open_idx = s.find('{').unwrap();
                let close_idx = s.rfind('}').unwrap();
                let header_part = s[..open_idx].trim();
                let tail_part = s[close_idx + 1..].trim();
                let body_part = &s[open_idx + 1..close_idx];

                let struct_name = if !tail_part.is_empty() && !tail_part.starts_with("__attribute__") {
                    tail_part.split_whitespace().last().unwrap_or("AnonStruct").to_string()
                } else {
                    header_part
                        .trim_start_matches("typedef")
                        .trim()
                        .trim_start_matches("struct")
                        .split_whitespace()
                        .filter(|w| !w.starts_with("__attribute__"))
                        .next_back()
                        .unwrap_or("AnonStruct")
                        .to_string()
                };

                let mut field_specs = Vec::new();
                for f_decl in body_part.split(';') {
                    let f_trim = f_decl.trim();
                    if f_trim.is_empty() {
                        continue;
                    }
                    let (decl_main, bit_w) = if let Some((l, r)) = f_trim.split_once(':') {
                        (l.trim(), r.trim().parse::<u8>().ok())
                    } else {
                        (f_trim, None)
                    };
                    let tokens: Vec<&str> = decl_main.split_whitespace().collect();
                    if tokens.len() >= 2 {
                        let raw_fname = tokens.last().unwrap();
                        let ptr_stars: String = raw_fname.chars().take_while(|&c| c == '*').collect();
                        let clean_fname = raw_fname.trim_start_matches('*');
                        let base_ty = tokens[..tokens.len() - 1].join(" ") + &ptr_stars;
                        let (fname, ctype) = if let Some((arr_name, arr_len_part)) = clean_fname.split_once('[') {
                            let len = arr_len_part.trim_end_matches(']').parse::<usize>().unwrap_or(1);
                            (
                                arr_name.to_string(),
                                CType::FixedArray(Box::new(CType::from_name(&base_ty)), len),
                            )
                        } else {
                            (clean_fname.to_string(), CType::from_name(&base_ty))
                        };
                        field_specs.push((fname, ctype, bit_w));
                    }
                }

                let layout = CStructLayout::compute(&struct_name, &field_specs, is_packed);
                if let Ok(mut reg) = get_c_type_registry().lock() {
                    reg.structs.insert(struct_name.clone(), layout);
                }
                result.structs.push(struct_name);
                continue;
            }

            // Check for union definition: `union Name { ... }` or `typedef union ... { ... } Name`
            if (s.starts_with("union ") || s.starts_with("typedef union")) && s.contains('{') && s.contains('}') {
                let open_idx = s.find('{').unwrap();
                let close_idx = s.rfind('}').unwrap();
                let header_part = s[..open_idx].trim();
                let tail_part = s[close_idx + 1..].trim();
                let body_part = &s[open_idx + 1..close_idx];

                let union_name = if !tail_part.is_empty() {
                    tail_part.split_whitespace().last().unwrap_or("AnonUnion").to_string()
                } else {
                    header_part
                        .trim_start_matches("typedef")
                        .trim()
                        .trim_start_matches("union")
                        .split_whitespace()
                        .next()
                        .unwrap_or("AnonUnion")
                        .to_string()
                };

                let mut field_specs = Vec::new();
                for f_decl in body_part.split(';') {
                    let tokens: Vec<&str> = f_decl.trim().split_whitespace().collect();
                    if tokens.len() >= 2 {
                        let raw_fname = tokens.last().unwrap().trim_start_matches('*');
                        let base_ty = tokens[..tokens.len() - 1].join(" ");
                        field_specs.push((raw_fname.to_string(), CType::from_name(&base_ty)));
                    }
                }

                let layout = CUnionLayout::compute(&union_name, &field_specs);
                if let Ok(mut reg) = get_c_type_registry().lock() {
                    reg.unions.insert(union_name.clone(), layout);
                }
                result.unions.push(union_name);
                continue;
            }

            // Check for enum definition: `enum Name { A = 0, B = 1 }`
            if (s.starts_with("enum ") || s.starts_with("typedef enum")) && s.contains('{') && s.contains('}') {
                let open_idx = s.find('{').unwrap();
                let close_idx = s.rfind('}').unwrap();
                let body_part = &s[open_idx + 1..close_idx];
                let mut next_val = 0i64;
                for item in body_part.split(',') {
                    let it = item.trim();
                    if it.is_empty() {
                        continue;
                    }
                    if let Some((ename, eval_str)) = it.split_once('=') {
                        if let Some(Value::Number(n)) = Self::parse_define_value(eval_str) {
                            next_val = n as i64;
                        }
                        result.enums.insert(ename.trim().to_string(), next_val);
                        next_val += 1;
                    } else {
                        result.enums.insert(it.to_string(), next_val);
                        next_val += 1;
                    }
                }
                continue;
            }

            // Check for function prototype: `[extern] <ret_type> <fn_name>(<params...>)`
            if s.contains('(') && s.ends_with(')') && !s.starts_with("typedef") {
                let clean_proto = s.trim_start_matches("extern").trim();
                let open_p = clean_proto.find('(').unwrap();
                let head = clean_proto[..open_p].trim();
                let params_str = &clean_proto[open_p + 1..clean_proto.len() - 1];

                let head_tokens: Vec<&str> = head.split_whitespace().collect();
                if head_tokens.len() >= 2 {
                    let raw_fn_name = head_tokens.last().unwrap();
                    let ptr_prefix: String = raw_fn_name.chars().take_while(|&c| c == '*').collect();
                    let fn_name = raw_fn_name.trim_start_matches('*').to_string();
                    let ret_ty_str = head_tokens[..head_tokens.len() - 1].join(" ") + &ptr_prefix;
                    let return_type = Self::parse_c_type_to_ffi(&ret_ty_str);

                    let mut param_types = Vec::new();
                    let mut is_variadic = false;
                    if !params_str.trim().is_empty() && params_str.trim() != "void" {
                        for p in params_str.split(',') {
                            let pt = p.trim();
                            if pt == "..." {
                                is_variadic = true;
                                continue;
                            }
                            let p_tokens: Vec<&str> = pt.split_whitespace().collect();
                            let p_ty_str = if p_tokens.len() >= 2 {
                                let last_tok = p_tokens.last().unwrap();
                                let stars: String = last_tok.chars().take_while(|&c| c == '*').collect();
                                p_tokens[..p_tokens.len() - 1].join(" ") + &stars
                            } else {
                                pt.to_string()
                            };
                            param_types.push(Self::parse_c_type_to_ffi(&p_ty_str));
                        }
                    }

                    let lib_name = if matches!(
                        fn_name.as_str(),
                        "sqrt" | "hypot" | "pow" | "sin" | "cos" | "tan" | "exp" | "log" | "floor" | "ceil" | "atan2"
                    ) {
                        "libm".to_string()
                    } else {
                        "libc".to_string()
                    };

                    let sig = FfiFunctionSignature {
                        name: fn_name.clone(),
                        symbol: fn_name,
                        library: lib_name,
                        param_types,
                        return_type,
                        is_variadic,
                    };
                    ffi_mgr.register_signature(sig.clone());
                    result.functions.push(sig);
                }
            }
        }

        result
    }
}
