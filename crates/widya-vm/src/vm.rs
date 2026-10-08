//! Mesin Virtual Bytecode Berbasis Register (Register-Based Bytecode VM)
//! Dirancang untuk eksekusi skrip sub-milidetik, live hot-reloading, NaN-boxing, dan konsumsi memori minimal.

use crate::decimal::Desimal;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum VmValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Decimal(Desimal),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<VmValue>),
    Map(HashMap<String, VmValue>),
}

impl fmt::Display for VmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmValue::Nil => write!(f, "nil"),
            VmValue::Bool(b) => write!(f, "{}", if *b { "benar" } else { "salah" }),
            VmValue::Int(n) => write!(f, "{}", n),
            VmValue::Float(n) => write!(f, "{}", n),
            VmValue::Decimal(d) => write!(f, "{}", d),
            VmValue::String(s) => write!(f, "{}", s),
            VmValue::Bytes(b) => write!(f, "<bytes: {} B>", b.len()),
            VmValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            VmValue::Map(m) => {
                let entries: Vec<String> = m.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", entries.join(", "))
            }
        }
    }
}

impl VmValue {
    pub fn type_tag(&self) -> u32 {
        match self {
            VmValue::Nil => 0,
            VmValue::Bool(_) => 1,
            VmValue::Int(_) => 2,
            VmValue::Float(_) => 3,
            VmValue::Decimal(_) => 4,
            VmValue::String(_) => 5,
            VmValue::Bytes(_) => 6,
            VmValue::Array(_) => 7,
            VmValue::Map(_) => 8,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            VmValue::Nil => "nil",
            VmValue::Bool(_) => "bool",
            VmValue::Int(_) => "int",
            VmValue::Float(_) => "float",
            VmValue::Decimal(_) => "decimal",
            VmValue::String(_) => "string",
            VmValue::Bytes(_) => "bytes",
            VmValue::Array(_) => "array",
            VmValue::Map(_) => "map",
        }
    }
}

/// Representasi NaN-Boxing 64-bit untuk nilai instan tanpa alokasi heap
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NanBoxedValue(pub u64);

const QNAN: u64 = 0x7FFC000000000000;
const TAG_NIL: u64 = 0x0001000000000000;
const TAG_FALSE: u64 = 0x0002000000000000;
const TAG_TRUE: u64 = 0x0003000000000000;
const TAG_INT: u64 = 0x0004000000000000;

impl NanBoxedValue {
    pub fn from_f64(val: f64) -> Self {
        Self(val.to_bits())
    }

    pub fn from_i32(val: i32) -> Self {
        Self(QNAN | TAG_INT | ((val as u32 as u64) & 0xFFFFFFFF))
    }

    pub fn from_bool(val: bool) -> Self {
        Self(QNAN | (if val { TAG_TRUE } else { TAG_FALSE }))
    }

    pub fn nil() -> Self {
        Self(QNAN | TAG_NIL)
    }

    pub fn is_f64(&self) -> bool {
        (self.0 & QNAN) != QNAN
    }

    pub fn as_f64(&self) -> Option<f64> {
        if self.is_f64() {
            Some(f64::from_bits(self.0))
        } else {
            None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        if (self.0 & (QNAN | TAG_INT)) == (QNAN | TAG_INT) {
            Some((self.0 & 0xFFFFFFFF) as i32)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if self.0 == (QNAN | TAG_TRUE) {
            Some(true)
        } else if self.0 == (QNAN | TAG_FALSE) {
            Some(false)
        } else {
            None
        }
    }

    pub fn is_nil(&self) -> bool {
        self.0 == (QNAN | TAG_NIL)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    LoadConst(u8, u16),      // dst_reg, const_idx
    Move(u8, u8),            // dst_reg, src_reg
    Add(u8, u8, u8),         // dst, lhs, rhs
    Sub(u8, u8, u8),         // dst, lhs, rhs
    Mul(u8, u8, u8),         // dst, lhs, rhs
    Div(u8, u8, u8),         // dst, lhs, rhs
    Mod(u8, u8, u8),         // dst, lhs, rhs
    AddDec(u8, u8, u8),      // dst, lhs, rhs (Desimal Fixed-Point)
    SubDec(u8, u8, u8),      // dst, lhs, rhs
    MulDec(u8, u8, u8),      // dst, lhs, rhs
    DivDec(u8, u8, u8),      // dst, lhs, rhs
    BitAnd(u8, u8, u8),      // dst, lhs, rhs
    BitOr(u8, u8, u8),       // dst, lhs, rhs
    BitXor(u8, u8, u8),      // dst, lhs, rhs
    Shl(u8, u8, u8),         // dst, lhs, rhs
    Shr(u8, u8, u8),         // dst, lhs, rhs
    Eq(u8, u8, u8),          // dst = (lhs == rhs)
    Lt(u8, u8, u8),          // dst = (lhs < rhs)
    Lte(u8, u8, u8),         // dst = (lhs <= rhs)
    Gt(u8, u8, u8),          // dst = (lhs > rhs)
    Gte(u8, u8, u8),         // dst = (lhs >= rhs)
    Jump(i32),               // relative instruction offset
    JumpIfTrue(u8, i32),     // reg, offset
    JumpIfFalse(u8, i32),    // reg, offset
    Call(u8, u16, u8, u8),   // dst_reg, fn_idx, arg_start_reg, arg_count
    TailCall(u16, u8, u8),   // fn_idx, arg_start_reg, arg_count (Tail Call Optimization)
    CallNative(u8, u16, u8, u8), // dst_reg, native_fn_id, arg_start_reg, arg_count
    ArrayNew(u8, usize),     // dst_reg, initial_capacity
    ArrayGet(u8, u8, u8),    // dst_reg, arr_reg, idx_reg
    ArraySet(u8, u8, u8),    // arr_reg, idx_reg, val_reg
    ArrayPush(u8, u8),       // arr_reg, val_reg
    ArrayLen(u8, u8),        // dst_reg, arr_reg
    VecDot(u8, u8, u8),      // dst_reg, vec_a_reg, vec_b_reg (SIMD/Vector Dot Product)
    VecAdd(u8, u8, u8),      // dst_reg, vec_a_reg, vec_b_reg (Elementwise Vector Add)
    VecScale(u8, u8, u8),    // dst_reg, vec_reg, scalar_reg (Vector Scalar Scaling)
    MapNew(u8),              // dst_reg
    MapGet(u8, u8, u8),      // dst_reg, map_reg, key_reg
    MapSet(u8, u8, u8),      // map_reg, key_reg, val_reg
    Return(u8),              // reg
    Print(u8, bool),         // reg, newline
    Halt,
    GuardType(u8, u32),      // reg, expected_type_tag (Deoptimasi ke Tier0 jika gagal)
    OsrEntry(usize),         // On-Stack Replacement hot loop target ip
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitTier {
    Tier0Interpreter,
    Tier1Baseline,
    Tier2OptimizingOsr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineCacheEntry {
    pub receiver_tag: u32,
    pub target_fn_idx: u16,
    pub hits: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolymorphicInlineCache {
    pub callsite_id: u32,
    pub entries: Vec<InlineCacheEntry>,
    pub megamorphic: bool,
    pub max_monomorphic_entries: usize,
}

impl PolymorphicInlineCache {
    pub fn new(callsite_id: u32) -> Self {
        Self {
            callsite_id,
            entries: Vec::new(),
            megamorphic: false,
            max_monomorphic_entries: 4,
        }
    }

    pub fn lookup(&mut self, receiver_tag: u32) -> Option<u16> {
        if self.megamorphic {
            return None;
        }
        for entry in &mut self.entries {
            if entry.receiver_tag == receiver_tag {
                entry.hits += 1;
                return Some(entry.target_fn_idx);
            }
        }
        None
    }

    pub fn record(&mut self, receiver_tag: u32, target_fn_idx: u16) {
        if self.megamorphic {
            return;
        }
        for entry in &mut self.entries {
            if entry.receiver_tag == receiver_tag {
                entry.target_fn_idx = target_fn_idx;
                entry.hits += 1;
                return;
            }
        }
        if self.entries.len() < self.max_monomorphic_entries {
            self.entries.push(InlineCacheEntry {
                receiver_tag,
                target_fn_idx,
                hits: 1,
            });
        } else {
            self.megamorphic = true;
        }
    }

    pub fn is_monomorphic(&self) -> bool {
        !self.megamorphic && self.entries.len() == 1
    }

    pub fn is_polymorphic(&self) -> bool {
        !self.megamorphic && self.entries.len() > 1
    }
}

#[derive(Debug, Clone)]
pub struct OsrLoopTracker {
    pub backedge_counts: HashMap<(usize, usize), u32>,
    pub osr_threshold: u32,
    pub osr_compiled_entries: HashMap<(usize, usize), JitTier>,
}

impl Default for OsrLoopTracker {
    fn default() -> Self {
        Self {
            backedge_counts: HashMap::new(),
            osr_threshold: 25,
            osr_compiled_entries: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeoptReason {
    TypeGuardFailed { expected: u32, actual: u32 },
    IntegerOverflow { op: &'static str },
    ArrayOutOfBounds { idx: i64, len: usize },
    DivisionByZero,
}

#[derive(Debug, Clone)]
pub struct DeoptimizationBailout {
    pub reason: DeoptReason,
    pub chunk_idx: usize,
    pub ip: usize,
    pub restored_registers: Vec<(u8, VmValue)>,
}

#[derive(Debug, Clone)]
pub struct BytecodeChunk {
    pub instructions: Vec<Opcode>,
    pub constants: Vec<VmValue>,
    pub lines: Vec<usize>,
}

impl BytecodeChunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn emit(&mut self, op: Opcode, line: usize) -> usize {
        let idx = self.instructions.len();
        self.instructions.push(op);
        self.lines.push(line);
        idx
    }

    pub fn add_constant(&mut self, val: VmValue) -> u16 {
        let idx = self.constants.len();
        self.constants.push(val);
        idx as u16
    }
}

pub struct CallFrame {
    pub chunk_idx: usize,
    pub ip: usize,
    pub register_offset: usize,
}

pub type NativeFn = Arc<dyn Fn(&[VmValue]) -> Result<VmValue, String> + Send + Sync>;

pub struct VirtualMachine {
    pub chunks: Vec<BytecodeChunk>,
    pub registers: [VmValue; 256], // 256 virtual registers
    pub frames: Vec<CallFrame>,
    pub instruction_count: u64,
    pub native_registry: HashMap<u16, NativeFn>,
    pub current_tier: JitTier,
    pub ic_table: HashMap<u32, PolymorphicInlineCache>,
    pub osr_tracker: OsrLoopTracker,
    pub deopt_log: Vec<DeoptimizationBailout>,
    pub osr_promotions: u32,
    pub method_calls_count: u64,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            registers: std::array::from_fn(|_| VmValue::Nil),
            frames: Vec::new(),
            instruction_count: 0,
            native_registry: HashMap::new(),
            current_tier: JitTier::Tier0Interpreter,
            ic_table: HashMap::new(),
            osr_tracker: OsrLoopTracker::default(),
            deopt_log: Vec::new(),
            osr_promotions: 0,
            method_calls_count: 0,
        }
    }

    pub fn trigger_osr_if_hot(&mut self, chunk_idx: usize, ip: usize) -> bool {
        let count = self.osr_tracker.backedge_counts.entry((chunk_idx, ip)).or_insert(0);
        *count += 1;
        if *count >= self.osr_tracker.osr_threshold && !self.osr_tracker.osr_compiled_entries.contains_key(&(chunk_idx, ip)) {
            self.osr_tracker.osr_compiled_entries.insert((chunk_idx, ip), JitTier::Tier2OptimizingOsr);
            self.current_tier = JitTier::Tier2OptimizingOsr;
            self.osr_promotions += 1;
            true
        } else {
            false
        }
    }

    pub fn bailout_to_interpreter(&mut self, chunk_idx: usize, ip: usize, reason: DeoptReason) -> DeoptimizationBailout {
        self.current_tier = JitTier::Tier0Interpreter;
        let mut snapshot = Vec::new();
        for i in 0..64 {
            if self.registers[i] != VmValue::Nil {
                snapshot.push((i as u8, self.registers[i].clone()));
            }
        }
        let bailout = DeoptimizationBailout {
            reason,
            chunk_idx,
            ip,
            restored_registers: snapshot,
        };
        self.deopt_log.push(bailout.clone());
        bailout
    }

    pub fn register_native<F>(&mut self, id: u16, f: F)
    where
        F: Fn(&[VmValue]) -> Result<VmValue, String> + Send + Sync + 'static,
    {
        self.native_registry.insert(id, Arc::new(f));
    }

    pub fn load_chunk(&mut self, chunk: BytecodeChunk) -> usize {
        let idx = self.chunks.len();
        self.chunks.push(chunk);
        idx
    }

    pub fn execute(&mut self, main_chunk_idx: usize) -> Result<VmValue, String> {
        let mut ip = 0;
        let mut current_chunk_idx = main_chunk_idx;
        self.frames.clear();

        while current_chunk_idx < self.chunks.len() {
            if ip >= self.chunks[current_chunk_idx].instructions.len() {
                if let Some(prev_frame) = self.frames.pop() {
                    self.registers[prev_frame.register_offset] = VmValue::Nil;
                    current_chunk_idx = prev_frame.chunk_idx;
                    ip = prev_frame.ip;
                    continue;
                } else {
                    break;
                }
            }

            self.instruction_count += 1;
            let op = self.chunks[current_chunk_idx].instructions[ip];
            ip += 1;

            match op {
                Opcode::LoadConst(reg, const_idx) => {
                    let val = self.chunks[current_chunk_idx].constants[const_idx as usize].clone();
                    self.registers[reg as usize] = val;
                }
                Opcode::Move(dst, src) => {
                    self.registers[dst as usize] = self.registers[src as usize].clone();
                }
                Opcode::Add(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => VmValue::Int(x + y),
                        (VmValue::Float(x), VmValue::Float(y)) => VmValue::Float(x + y),
                        (VmValue::Int(x), VmValue::Float(y)) => VmValue::Float(*x as f64 + y),
                        (VmValue::Float(x), VmValue::Int(y)) => VmValue::Float(x + *y as f64),
                        (VmValue::String(x), VmValue::String(y)) => VmValue::String(format!("{}{}", x, y)),
                        _ => return Err(format!("Operasi Add tidak kompatibel: {:?} + {:?}", a, b)),
                    };
                    self.registers[dst as usize] = res;
                }
                Opcode::Sub(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => VmValue::Int(x - y),
                        (VmValue::Float(x), VmValue::Float(y)) => VmValue::Float(x - y),
                        _ => return Err(format!("Operasi Sub tidak kompatibel: {:?} - {:?}", a, b)),
                    };
                    self.registers[dst as usize] = res;
                }
                Opcode::Mul(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => VmValue::Int(x * y),
                        (VmValue::Float(x), VmValue::Float(y)) => VmValue::Float(x * y),
                        _ => return Err("Operasi Mul tidak kompatibel".into()),
                    };
                    self.registers[dst as usize] = res;
                }
                Opcode::Div(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => {
                            if *y == 0 { return Err("Pembagian dengan nol".into()); }
                            VmValue::Int(x / y)
                        }
                        (VmValue::Float(x), VmValue::Float(y)) => {
                            if *y == 0.0 { return Err("Pembagian dengan nol".into()); }
                            VmValue::Float(x / y)
                        }
                        _ => return Err("Operasi Div tidak kompatibel".into()),
                    };
                    self.registers[dst as usize] = res;
                }
                Opcode::Mod(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => {
                            if *y == 0 { return Err("Modulo dengan nol".into()); }
                            VmValue::Int(x % y)
                        }
                        _ => return Err("Operasi Modulo hanya mendukung bilangan bulat (Int)".into()),
                    };
                    self.registers[dst as usize] = res;
                }
                Opcode::BitAnd(dst, lhs, rhs) => {
                    if let (VmValue::Int(x), VmValue::Int(y)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Int(x & y);
                    } else {
                        return Err("Operasi BitAnd memerlukan tipe Int".into());
                    }
                }
                Opcode::BitOr(dst, lhs, rhs) => {
                    if let (VmValue::Int(x), VmValue::Int(y)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Int(x | y);
                    } else {
                        return Err("Operasi BitOr memerlukan tipe Int".into());
                    }
                }
                Opcode::BitXor(dst, lhs, rhs) => {
                    if let (VmValue::Int(x), VmValue::Int(y)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Int(x ^ y);
                    } else {
                        return Err("Operasi BitXor memerlukan tipe Int".into());
                    }
                }
                Opcode::Shl(dst, lhs, rhs) => {
                    if let (VmValue::Int(x), VmValue::Int(y)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Int(x << y);
                    } else {
                        return Err("Operasi Shl memerlukan tipe Int".into());
                    }
                }
                Opcode::Shr(dst, lhs, rhs) => {
                    if let (VmValue::Int(x), VmValue::Int(y)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Int(x >> y);
                    } else {
                        return Err("Operasi Shr memerlukan tipe Int".into());
                    }
                }
                Opcode::AddDec(dst, lhs, rhs) => {
                    if let (VmValue::Decimal(a), VmValue::Decimal(b)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Decimal(*a + *b);
                    } else {
                        return Err("Operasi AddDec memerlukan operand bertipe Desimal".into());
                    }
                }
                Opcode::SubDec(dst, lhs, rhs) => {
                    if let (VmValue::Decimal(a), VmValue::Decimal(b)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Decimal(*a - *b);
                    } else {
                        return Err("Operasi SubDec memerlukan operand bertipe Desimal".into());
                    }
                }
                Opcode::MulDec(dst, lhs, rhs) => {
                    if let (VmValue::Decimal(a), VmValue::Decimal(b)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Decimal(*a * *b);
                    } else {
                        return Err("Operasi MulDec memerlukan operand bertipe Desimal".into());
                    }
                }
                Opcode::DivDec(dst, lhs, rhs) => {
                    if let (VmValue::Decimal(a), VmValue::Decimal(b)) = (&self.registers[lhs as usize], &self.registers[rhs as usize]) {
                        self.registers[dst as usize] = VmValue::Decimal(*a / *b);
                    } else {
                        return Err("Operasi DivDec memerlukan operand bertipe Desimal".into());
                    }
                }
                Opcode::Eq(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    self.registers[dst as usize] = VmValue::Bool(a == b);
                }
                Opcode::Lt(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => x < y,
                        (VmValue::Float(x), VmValue::Float(y)) => x < y,
                        (VmValue::Decimal(x), VmValue::Decimal(y)) => x < y,
                        _ => false,
                    };
                    self.registers[dst as usize] = VmValue::Bool(res);
                }
                Opcode::Lte(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => x <= y,
                        (VmValue::Float(x), VmValue::Float(y)) => x <= y,
                        (VmValue::Decimal(x), VmValue::Decimal(y)) => x <= y,
                        _ => false,
                    };
                    self.registers[dst as usize] = VmValue::Bool(res);
                }
                Opcode::Gt(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => x > y,
                        (VmValue::Float(x), VmValue::Float(y)) => x > y,
                        (VmValue::Decimal(x), VmValue::Decimal(y)) => x > y,
                        _ => false,
                    };
                    self.registers[dst as usize] = VmValue::Bool(res);
                }
                Opcode::Gte(dst, lhs, rhs) => {
                    let a = &self.registers[lhs as usize];
                    let b = &self.registers[rhs as usize];
                    let res = match (a, b) {
                        (VmValue::Int(x), VmValue::Int(y)) => x >= y,
                        (VmValue::Float(x), VmValue::Float(y)) => x >= y,
                        (VmValue::Decimal(x), VmValue::Decimal(y)) => x >= y,
                        _ => false,
                    };
                    self.registers[dst as usize] = VmValue::Bool(res);
                }
                Opcode::ArrayNew(dst, cap) => {
                    self.registers[dst as usize] = VmValue::Array(Vec::with_capacity(cap));
                }
                Opcode::ArrayGet(dst, arr_reg, idx_reg) => {
                    let arr = &self.registers[arr_reg as usize];
                    let idx_val = &self.registers[idx_reg as usize];
                    if let (VmValue::Array(list), VmValue::Int(idx)) = (arr, idx_val) {
                        if *idx >= 0 && (*idx as usize) < list.len() {
                            self.registers[dst as usize] = list[*idx as usize].clone();
                        } else {
                            return Err(format!("Index out of bounds: {} (len: {})", idx, list.len()));
                        }
                    } else {
                        return Err("ArrayGet memerlukan register Array dan index Int".into());
                    }
                }
                Opcode::ArraySet(arr_reg, idx_reg, val_reg) => {
                    let val = self.registers[val_reg as usize].clone();
                    let idx_val = &self.registers[idx_reg as usize];
                    if let VmValue::Int(idx) = idx_val {
                        let i = *idx as usize;
                        if let VmValue::Array(list) = &mut self.registers[arr_reg as usize] {
                            if i < list.len() {
                                list[i] = val;
                            } else {
                                return Err(format!("Index out of bounds saat ArraySet: {}", i));
                            }
                        } else {
                            return Err("ArraySet target bukan Array".into());
                        }
                    } else {
                        return Err("ArraySet index harus Int".into());
                    }
                }
                Opcode::ArrayPush(arr_reg, val_reg) => {
                    let val = self.registers[val_reg as usize].clone();
                    if let VmValue::Array(list) = &mut self.registers[arr_reg as usize] {
                        list.push(val);
                    } else {
                        return Err("ArrayPush target bukan Array".into());
                    }
                }
                Opcode::ArrayLen(dst, arr_reg) => {
                    if let VmValue::Array(list) = &self.registers[arr_reg as usize] {
                        self.registers[dst as usize] = VmValue::Int(list.len() as i64);
                    } else {
                        return Err("ArrayLen target bukan Array".into());
                    }
                }
                Opcode::VecDot(dst, vec_a, vec_b) => {
                    let a = &self.registers[vec_a as usize];
                    let b = &self.registers[vec_b as usize];
                    if let (VmValue::Array(list_a), VmValue::Array(list_b)) = (a, b) {
                        if list_a.len() != list_b.len() {
                            return Err("VecDot: Dimensi vektor tidak sama".into());
                        }
                        let mut sum = 0.0;
                        for (va, vb) in list_a.iter().zip(list_b.iter()) {
                            let num_a = match va {
                                VmValue::Int(n) => *n as f64,
                                VmValue::Float(f) => *f,
                                _ => return Err("VecDot: Elemen vektor harus numerik".into()),
                            };
                            let num_b = match vb {
                                VmValue::Int(n) => *n as f64,
                                VmValue::Float(f) => *f,
                                _ => return Err("VecDot: Elemen vektor harus numerik".into()),
                            };
                            sum += num_a * num_b;
                        }
                        self.registers[dst as usize] = VmValue::Float(sum);
                    } else {
                        return Err("VecDot: Operand harus bertipe Array".into());
                    }
                }
                Opcode::VecAdd(dst, vec_a, vec_b) => {
                    let a = &self.registers[vec_a as usize];
                    let b = &self.registers[vec_b as usize];
                    if let (VmValue::Array(list_a), VmValue::Array(list_b)) = (a, b) {
                        if list_a.len() != list_b.len() {
                            return Err("VecAdd: Dimensi vektor tidak sama".into());
                        }
                        let mut result = Vec::with_capacity(list_a.len());
                        for (va, vb) in list_a.iter().zip(list_b.iter()) {
                            let res = match (va, vb) {
                                (VmValue::Int(x), VmValue::Int(y)) => VmValue::Int(x + y),
                                (VmValue::Float(x), VmValue::Float(y)) => VmValue::Float(x + y),
                                (VmValue::Int(x), VmValue::Float(y)) => VmValue::Float(*x as f64 + y),
                                (VmValue::Float(x), VmValue::Int(y)) => VmValue::Float(x + *y as f64),
                                _ => return Err("VecAdd: Elemen vektor harus numerik".into()),
                            };
                            result.push(res);
                        }
                        self.registers[dst as usize] = VmValue::Array(result);
                    } else {
                        return Err("VecAdd: Operand harus bertipe Array".into());
                    }
                }
                Opcode::VecScale(dst, vec_a, scalar_reg) => {
                    let a = &self.registers[vec_a as usize];
                    let s = &self.registers[scalar_reg as usize];
                    let factor = match s {
                        VmValue::Int(n) => *n as f64,
                        VmValue::Float(f) => *f,
                        _ => return Err("VecScale: Skalar harus numerik".into()),
                    };
                    if let VmValue::Array(list) = a {
                        let mut result = Vec::with_capacity(list.len());
                        for item in list {
                            let res = match item {
                                VmValue::Int(x) => VmValue::Float(*x as f64 * factor),
                                VmValue::Float(x) => VmValue::Float(*x * factor),
                                _ => return Err("VecScale: Elemen vektor harus numerik".into()),
                            };
                            result.push(res);
                        }
                        self.registers[dst as usize] = VmValue::Array(result);
                    } else {
                        return Err("VecScale: Operand harus bertipe Array".into());
                    }
                }
                Opcode::MapNew(dst) => {
                    self.registers[dst as usize] = VmValue::Map(HashMap::new());
                }
                Opcode::MapGet(dst, map_reg, key_reg) => {
                    let map = &self.registers[map_reg as usize];
                    let key = &self.registers[key_reg as usize];
                    if let (VmValue::Map(m), VmValue::String(k)) = (map, key) {
                        self.registers[dst as usize] = m.get(k).cloned().unwrap_or(VmValue::Nil);
                    } else {
                        return Err("MapGet memerlukan Map dan String key".into());
                    }
                }
                Opcode::MapSet(map_reg, key_reg, val_reg) => {
                    let key = self.registers[key_reg as usize].clone();
                    let val = self.registers[val_reg as usize].clone();
                    if let VmValue::String(k) = key {
                        if let VmValue::Map(m) = &mut self.registers[map_reg as usize] {
                            m.insert(k, val);
                        } else {
                            return Err("MapSet target bukan Map".into());
                        }
                    } else {
                        return Err("MapSet key harus String".into());
                    }
                }
                Opcode::CallNative(dst, native_id, arg_start, arg_count) => {
                    if let Some(f) = self.native_registry.get(&native_id).cloned() {
                        let mut args = Vec::with_capacity(arg_count as usize);
                        for i in 0..arg_count {
                            args.push(self.registers[(arg_start + i) as usize].clone());
                        }
                        let res = f(&args)?;
                        self.registers[dst as usize] = res;
                    } else {
                        return Err(format!("Native function id {} tidak terdaftar", native_id));
                    }
                }
                Opcode::Call(dst, fn_idx, arg_start, arg_count) => {
                    let target_idx = fn_idx as usize;
                    if target_idx >= self.chunks.len() {
                        return Err(format!("Call: chunk index {} tidak valid", target_idx));
                    }
                    self.method_calls_count += 1;
                    if self.method_calls_count >= 10 && self.current_tier == JitTier::Tier0Interpreter {
                        self.current_tier = JitTier::Tier1Baseline;
                    }
                    let callsite_id = ((current_chunk_idx as u32) << 16) | (ip as u32);
                    let rec_tag = if arg_count > 0 {
                        self.registers[arg_start as usize].type_tag()
                    } else {
                        0
                    };
                    let ic = self.ic_table.entry(callsite_id).or_insert_with(|| PolymorphicInlineCache::new(callsite_id));
                    ic.record(rec_tag, fn_idx);

                    self.frames.push(CallFrame {
                        chunk_idx: current_chunk_idx,
                        ip,
                        register_offset: dst as usize,
                    });
                    let mut args = Vec::with_capacity(arg_count as usize);
                    for i in 0..arg_count {
                        args.push(self.registers[(arg_start + i) as usize].clone());
                    }
                    for (i, val) in args.into_iter().enumerate() {
                        self.registers[i] = val;
                    }
                    current_chunk_idx = target_idx;
                    ip = 0;
                }
                Opcode::TailCall(fn_idx, arg_start, arg_count) => {
                    let target_idx = fn_idx as usize;
                    if target_idx >= self.chunks.len() {
                        return Err(format!("TailCall: chunk index {} tidak valid", target_idx));
                    }
                    // TCO: Menggunakan frame aktif saat ini tanpa alokasi frame baru
                    let mut args = Vec::with_capacity(arg_count as usize);
                    for i in 0..arg_count {
                        args.push(self.registers[(arg_start + i) as usize].clone());
                    }
                    for (i, val) in args.into_iter().enumerate() {
                        self.registers[i] = val;
                    }
                    current_chunk_idx = target_idx;
                    ip = 0;
                }
                Opcode::Jump(offset) => {
                    if offset < 0 {
                        let backedge_ip = ((ip as i32) + offset) as usize;
                        self.trigger_osr_if_hot(current_chunk_idx, backedge_ip);
                    }
                    ip = ((ip as i32) + offset) as usize;
                }
                Opcode::JumpIfTrue(reg, offset) => {
                    if let VmValue::Bool(true) = self.registers[reg as usize] {
                        if offset < 0 {
                            let backedge_ip = ((ip as i32) + offset) as usize;
                            self.trigger_osr_if_hot(current_chunk_idx, backedge_ip);
                        }
                        ip = ((ip as i32) + offset) as usize;
                    }
                }
                Opcode::JumpIfFalse(reg, offset) => {
                    if let VmValue::Bool(false) = self.registers[reg as usize] {
                        if offset < 0 {
                            let backedge_ip = ((ip as i32) + offset) as usize;
                            self.trigger_osr_if_hot(current_chunk_idx, backedge_ip);
                        }
                        ip = ((ip as i32) + offset) as usize;
                    }
                }
                Opcode::GuardType(reg, expected_tag) => {
                    let actual_tag = self.registers[reg as usize].type_tag();
                    if actual_tag != expected_tag {
                        self.bailout_to_interpreter(
                            current_chunk_idx,
                            ip,
                            DeoptReason::TypeGuardFailed {
                                expected: expected_tag,
                                actual: actual_tag,
                            },
                        );
                    }
                }
                Opcode::OsrEntry(target) => {
                    self.current_tier = JitTier::Tier2OptimizingOsr;
                    self.osr_promotions += 1;
                    ip = target;
                }
                Opcode::Print(reg, newline) => {
                    let val = &self.registers[reg as usize];
                    if newline {
                        println!("{}", val);
                    } else {
                        print!("{}", val);
                    }
                }
                Opcode::Return(reg) => {
                    let val = self.registers[reg as usize].clone();
                    if let Some(prev_frame) = self.frames.pop() {
                        self.registers[prev_frame.register_offset] = val;
                        current_chunk_idx = prev_frame.chunk_idx;
                        ip = prev_frame.ip;
                    } else {
                        return Ok(val);
                    }
                }
                Opcode::Halt => {
                    break;
                }
            }
        }

        Ok(self.registers[0].clone())
    }

    pub fn execute_tiered(&mut self, main_chunk_idx: usize) -> Result<VmValue, String> {
        self.current_tier = JitTier::Tier1Baseline;
        self.execute(main_chunk_idx)
    }
}
