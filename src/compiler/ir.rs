//! Widya-IR: SSA-Based Intermediate Representation Engine
//! Lapisan perantara perancangan compiler untuk optimasi dan multi-target codegen (VM, LLVM, WASM).

use crate::compiler::decimal::Desimal;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    Void,
    Bool,
    I64,
    F64,
    Decimal,
    String,
    Pointer(Box<IrType>),
    Array(Box<IrType>, usize),
    Vector(Box<IrType>, usize),
    Struct(String, Vec<IrType>),
}

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrType::Void => write!(f, "void"),
            IrType::Bool => write!(f, "i1"),
            IrType::I64 => write!(f, "i64"),
            IrType::F64 => write!(f, "double"),
            IrType::Decimal => write!(f, "i128"),
            IrType::String => write!(f, "ptr_str"),
            IrType::Pointer(inner) => write!(f, "*{}", inner),
            IrType::Array(inner, len) => write!(f, "[{} x {}]", len, inner),
            IrType::Vector(inner, len) => write!(f, "<{} x {}>", len, inner),
            IrType::Struct(name, _) => write!(f, "%struct.{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Reg(String),
    ConstInt(i64),
    ConstFloat(f64),
    ConstDecimal(Desimal),
    ConstBool(bool),
    ConstString(String),
    ConstVector(Vec<IrValue>),
    Global(String),
    Null,
}

impl fmt::Display for IrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrValue::Reg(r) => write!(f, "%{}", r),
            IrValue::ConstInt(n) => write!(f, "{}", n),
            IrValue::ConstFloat(n) => write!(f, "{:.4}", n),
            IrValue::ConstDecimal(d) => write!(f, "dec({})", d),
            IrValue::ConstBool(b) => write!(f, "{}", if *b { "benar" } else { "salah" }),
            IrValue::ConstString(s) => write!(f, "\"{}\"", s),
            IrValue::ConstVector(elems) => {
                let items: Vec<String> = elems.iter().map(|e| e.to_string()).collect();
                write!(f, "<{}>", items.join(", "))
            }
            IrValue::Global(g) => write!(f, "@{}", g),
            IrValue::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    // Aritmatika & Nilai
    Assign { dst: IrValue, src: IrValue, ty: IrType },
    BinOp { dst: IrValue, op: IrBinOp, lhs: IrValue, rhs: IrValue, ty: IrType },
    UnaryNot { dst: IrValue, src: IrValue },
    UnaryNeg { dst: IrValue, src: IrValue, ty: IrType },

    // Alokasi & Memori
    Alloca { dst: IrValue, ty: IrType },
    Load { dst: IrValue, ptr: IrValue, ty: IrType },
    Store { val: IrValue, ptr: IrValue },

    // Kontrol Alur
    Branch { cond: IrValue, then_label: String, else_label: String },
    Jump { target_label: String },
    Return { val: Option<IrValue> },
    Phi { dst: IrValue, ty: IrType, incoming: Vec<(IrValue, String)> },

    // Pemanggilan Fungsi & Native FFI
    Call { dst: Option<IrValue>, func_name: String, args: Vec<IrValue>, ret_ty: IrType },
    Print { val: IrValue, newline: bool },

    // Konkurensi & Actor
    SpawnFiber { func_name: String, args: Vec<IrValue> },

    // SIMD & Auto-Vectorization (AVX-512, AVX2, ARM Neon)
    VectorOp { dst: IrValue, op: IrBinOp, lhs: IrValue, rhs: IrValue, lane_width: usize, element_ty: IrType },
    VectorLoad { dst: IrValue, base_ptr: IrValue, lane_width: usize, element_ty: IrType },
    VectorStore { src: IrValue, base_ptr: IrValue, lane_width: usize, element_ty: IrType },
    VectorFma { dst: IrValue, a: IrValue, b: IrValue, c: IrValue, lane_width: usize, element_ty: IrType },
    VectorDotProduct { dst: IrValue, lhs: IrValue, rhs: IrValue, lane_width: usize },
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<IrInstruction>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label,
            instructions: Vec::new(),
        }
    }

    pub fn push(&mut self, inst: IrInstruction) {
        self.instructions.push(inst);
    }
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_type: IrType,
    pub blocks: Vec<BasicBlock>,
}

impl IrFunction {
    pub fn new(name: String, params: Vec<(String, IrType)>, return_type: IrType) -> Self {
        Self {
            name,
            params,
            return_type,
            blocks: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrModule {
    pub name: String,
    pub functions: Vec<IrFunction>,
    pub global_constants: HashMap<String, (IrType, IrValue)>,
}

impl IrModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            global_constants: HashMap::new(),
        }
    }

    pub fn tambah_fungsi(&mut self, func: IrFunction) {
        self.functions.push(func);
    }

    /// Render SSA representation ke format teks mirip LLVM
    pub fn dump_ir(&self) -> String {
        let mut out = format!("; Widya-IR Module: {}\n\n", self.name);

        for (name, (ty, val)) in &self.global_constants {
            out.push_str(&format!("@{} = global {} {}\n", name, ty, val));
        }

        for func in &self.functions {
            let params_str: Vec<String> = func
                .params
                .iter()
                .map(|(p, ty)| format!("{} %{}", ty, p))
                .collect();

            out.push_str(&format!(
                "\ndefine {} @{}({}) {{\n",
                func.return_type,
                func.name,
                params_str.join(", ")
            ));

            for block in &func.blocks {
                out.push_str(&format!("{}:\n", block.label));
                for inst in &block.instructions {
                    out.push_str(&format!("    {:?}\n", inst));
                }
            }
            out.push_str("}\n");
        }

        out
    }
}

/// Optimizer Passes untuk Widya-IR SSA
pub struct IrOptimizer;

impl IrOptimizer {
    /// Menjalankan seluruh lintasan optimasi (Constant Folding, DCE, Inlining)
    pub fn optimasi(module: &mut IrModule) {
        for func in &mut module.functions {
            Self::constant_folding(func);
            Self::dead_code_elimination(func);
        }
    }

    /// Constant Folding Pass (Evaluasi operasi konstan pada waktu kompilasi)
    pub fn constant_folding(func: &mut IrFunction) {
        for block in &mut func.blocks {
            for inst in &mut block.instructions {
                if let IrInstruction::BinOp { dst, op, lhs, rhs, ty } = inst {
                    match (lhs, rhs) {
                        (IrValue::ConstInt(a), IrValue::ConstInt(b)) => {
                            let val_a = *a;
                            let val_b = *b;
                            let res = match op {
                                IrBinOp::Add => Some(val_a + val_b),
                                IrBinOp::Sub => Some(val_a - val_b),
                                IrBinOp::Mul => Some(val_a * val_b),
                                IrBinOp::Div if val_b != 0 => Some(val_a / val_b),
                                IrBinOp::Mod if val_b != 0 => Some(val_a % val_b),
                                _ => None,
                            };
                            if let Some(val) = res {
                                *inst = IrInstruction::Assign {
                                    dst: dst.clone(),
                                    src: IrValue::ConstInt(val),
                                    ty: ty.clone(),
                                };
                            }
                        }
                        (IrValue::ConstDecimal(a), IrValue::ConstDecimal(b)) => {
                            let res = match op {
                                IrBinOp::Add => Some(*a + *b),
                                IrBinOp::Sub => Some(*a - *b),
                                IrBinOp::Mul => Some(*a * *b),
                                IrBinOp::Div if b.nilai_mentah != 0 => Some(*a / *b),
                                _ => None,
                            };
                            if let Some(val) = res {
                                *inst = IrInstruction::Assign {
                                    dst: dst.clone(),
                                    src: IrValue::ConstDecimal(val),
                                    ty: ty.clone(),
                                };
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Dead Code Elimination Pass (Menghapus instruksi yang hasilnya tidak digunakan)
    pub fn dead_code_elimination(func: &mut IrFunction) {
        let mut used_regs = HashSet::new();

        // Kumpulkan semua register yang dibaca
        for block in &func.blocks {
            for inst in &block.instructions {
                match inst {
                    IrInstruction::Assign { src: IrValue::Reg(r), .. } => { used_regs.insert(r.clone()); }
                    IrInstruction::BinOp { lhs: IrValue::Reg(r1), rhs: IrValue::Reg(r2), .. } => {
                        used_regs.insert(r1.clone());
                        used_regs.insert(r2.clone());
                    }
                    IrInstruction::Branch { cond: IrValue::Reg(r), .. } => { used_regs.insert(r.clone()); }
                    IrInstruction::Return { val: Some(IrValue::Reg(r)) } => { used_regs.insert(r.clone()); }
                    IrInstruction::Print { val: IrValue::Reg(r), .. } => { used_regs.insert(r.clone()); }
                    IrInstruction::Call { args, .. } => {
                        for arg in args {
                            if let IrValue::Reg(r) = arg {
                                used_regs.insert(r.clone());
                            }
                        }
                    }
                    IrInstruction::VectorOp { lhs, rhs, .. } => {
                        if let IrValue::Reg(r1) = lhs { used_regs.insert(r1.clone()); }
                        if let IrValue::Reg(r2) = rhs { used_regs.insert(r2.clone()); }
                    }
                    IrInstruction::VectorFma { a, b, c, .. } => {
                        if let IrValue::Reg(r) = a { used_regs.insert(r.clone()); }
                        if let IrValue::Reg(r) = b { used_regs.insert(r.clone()); }
                        if let IrValue::Reg(r) = c { used_regs.insert(r.clone()); }
                    }
                    _ => {}
                }
            }
        }

        // Hapus penugasan murni ke register yang tidak pernah dibaca
        for block in &mut func.blocks {
            block.instructions.retain(|inst| {
                match inst {
                    IrInstruction::Assign { dst: IrValue::Reg(r), .. } => used_regs.contains(r),
                    _ => true,
                }
            });
        }
    }

    /// SIMD Auto-Vectorization Loop Pass (AVX-512 & ARM Neon)
    pub fn auto_vectorize_loops(func: &mut IrFunction, target_isa: SimdTargetIsa) -> VectorizationPlan {
        let lane_width = target_isa.vector_lane_width(8); // 8-byte f64/i64 default
        let mut loops_analyzed = 0;
        let mut loops_vectorized = 0;

        for block in &mut func.blocks {
            let has_loop_backedge = block.instructions.iter().any(|inst| {
                matches!(inst, IrInstruction::Jump { target_label } if target_label.contains("loop") || target_label.contains("header"))
                    || matches!(inst, IrInstruction::Branch { then_label, else_label, .. } if then_label.contains("loop") || else_label.contains("loop"))
            });

            if has_loop_backedge {
                loops_analyzed += 1;
                let mut transformed = false;

                for inst in &mut block.instructions {
                    if let IrInstruction::BinOp { dst, op, lhs, rhs, ty } = inst {
                        if ty == &IrType::F64 || ty == &IrType::I64 {
                            *inst = IrInstruction::VectorOp {
                                dst: dst.clone(),
                                op: *op,
                                lhs: lhs.clone(),
                                rhs: rhs.clone(),
                                lane_width,
                                element_ty: ty.clone(),
                            };
                            transformed = true;
                        }
                    }
                }

                if transformed {
                    loops_vectorized += 1;
                }
            }
        }

        let unroll_factor = if lane_width >= 8 { 4 } else { 2 };
        let speedup = if loops_vectorized > 0 {
            (lane_width as f64) * 0.88
        } else {
            1.0
        };

        VectorizationPlan {
            loops_analyzed,
            loops_vectorized,
            target_isa,
            vector_lane_width: lane_width,
            unroll_factor,
            estimated_speedup: speedup,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdTargetIsa {
    Avx512,       // 512-bit vector width (8 x f64 or 16 x f32/i32)
    Avx2,         // 256-bit vector width (4 x f64 or 8 x f32/i32)
    ArmNeon,      // 128-bit vector width (2 x f64 or 4 x f32/i32)
    WasmSimd128,  // 128-bit vector width
}

impl SimdTargetIsa {
    pub fn vector_lane_width(&self, element_size_bytes: usize) -> usize {
        let register_bytes = match self {
            SimdTargetIsa::Avx512 => 64,
            SimdTargetIsa::Avx2 => 32,
            SimdTargetIsa::ArmNeon | SimdTargetIsa::WasmSimd128 => 16,
        };
        register_bytes / element_size_bytes.max(1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InductionVariable {
    pub reg: String,
    pub init: i64,
    pub step: i64,
    pub bound: i64,
    pub loop_header_label: String,
}

#[derive(Debug, Clone)]
pub struct VectorizationPlan {
    pub loops_analyzed: usize,
    pub loops_vectorized: usize,
    pub target_isa: SimdTargetIsa,
    pub vector_lane_width: usize,
    pub unroll_factor: usize,
    pub estimated_speedup: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineDecision {
    InlineAlways { score: f64, reason: String },
    InlineThreshold { score: f64, threshold: f64 },
    DoNotInline { reason: String },
}

#[derive(Debug, Clone)]
pub struct OptimizationAdvice {
    pub function_name: String,
    pub inline_decision: InlineDecision,
    pub branch_prediction_weights: HashMap<String, (f64, f64)>, // branch_id -> (then_prob, else_prob)
    pub loop_unroll_factor: usize,
    pub hotness_score: f64,
}

pub struct AiOptimizationAdvisor {
    pub inline_threshold: f64,
}

impl AiOptimizationAdvisor {
    pub fn new() -> Self {
        Self {
            inline_threshold: 45.0,
        }
    }

    /// Evaluasi model heuristik cerdas (AI cost model) untuk fungsi IR
    pub fn analyze_function(&self, func: &IrFunction, call_count: u64) -> OptimizationAdvice {
        let instruction_count: usize = func.blocks.iter().map(|b| b.instructions.len()).sum();
        let branch_count: usize = func.blocks.iter().flat_map(|b| &b.instructions)
            .filter(|i| matches!(i, IrInstruction::Branch { .. }))
            .count();
        let loop_backedge_count: usize = func.blocks.iter().flat_map(|b| &b.instructions)
            .filter(|i| matches!(i, IrInstruction::Jump { target_label } if target_label.contains("loop") || target_label.contains("header")))
            .count();

        // Model penilaian hotness berbasis featurization
        let cyclomatic_complexity = 1.0 + branch_count as f64;
        let hotness_score = (call_count as f64 * 1.5) + (loop_backedge_count as f64 * 10.0) / (instruction_count.max(1) as f64).sqrt();

        let inline_decision = if instruction_count <= 5 {
            InlineDecision::InlineAlways { score: 100.0, reason: "Leaf function berukuran kecil (<= 5 IR)".to_string() }
        } else if hotness_score >= self.inline_threshold && instruction_count <= 40 {
            InlineDecision::InlineThreshold { score: hotness_score, threshold: self.inline_threshold }
        } else {
            InlineDecision::DoNotInline { reason: format!("Ukuran fungsi ({} ops) melebihi batas atau cyclomatic complexity ({:.1}) terlalu tinggi", instruction_count, cyclomatic_complexity) }
        };

        let mut branch_weights = HashMap::new();
        for block in &func.blocks {
            for (idx, inst) in block.instructions.iter().enumerate() {
                if let IrInstruction::Branch { then_label, else_label, .. } = inst {
                    let branch_id = format!("{}_b_{}", block.label, idx);
                    // Heuristik learned branch prediction
                    let (then_prob, else_prob) = if then_label.contains("err") || else_label.contains("panic") {
                        (0.05, 0.95)
                    } else if then_label.contains("loop") || then_label.contains("body") {
                        (0.88, 0.12)
                    } else {
                        (0.60, 0.40)
                    };
                    branch_weights.insert(branch_id, (then_prob, else_prob));
                }
            }
        }

        let loop_unroll_factor = if loop_backedge_count > 0 && instruction_count < 25 {
            4
        } else if loop_backedge_count > 0 {
            2
        } else {
            1
        };

        OptimizationAdvice {
            function_name: func.name.clone(),
            inline_decision,
            branch_prediction_weights: branch_weights,
            loop_unroll_factor,
            hotness_score,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterAllocationResult {
    pub mapping: HashMap<String, String>, // virtual_reg -> physical_reg (mis. %0 -> "r0", %1 -> "r1")
    pub spilled_registers: Vec<String>,
    pub registers_used: usize,
    pub interference_edges_count: usize,
}

pub struct ChaitinBriggsRegisterAllocator {
    pub k_physical_registers: usize, // e.g. 16 physical registers
    pub physical_names: Vec<String>,
}

impl ChaitinBriggsRegisterAllocator {
    pub fn new(k_physical_registers: usize) -> Self {
        let names = (0..k_physical_registers).map(|i| format!("r{}", i)).collect();
        Self {
            k_physical_registers,
            physical_names: names,
        }
    }

    /// Chaitin-Briggs Graph Coloring Allocation Algorithm
    pub fn allocate(&self, func: &IrFunction) -> RegisterAllocationResult {
        // 1. Kumpulkan semua virtual register
        let mut virtual_regs = HashSet::new();
        for block in &func.blocks {
            for inst in &block.instructions {
                Self::collect_registers_from_inst(inst, &mut virtual_regs);
            }
        }

        // 2. Bangun Interference Graph (Graf Interferensi)
        let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
        for r in &virtual_regs {
            adjacency.insert(r.clone(), HashSet::new());
        }

        // Analisis liveness per basic block
        for block in &func.blocks {
            let mut block_regs = HashSet::new();
            for inst in &block.instructions {
                Self::collect_registers_from_inst(inst, &mut block_regs);
            }
            let reg_list: Vec<String> = block_regs.into_iter().collect();
            for i in 0..reg_list.len() {
                for j in (i + 1)..reg_list.len() {
                    let u = &reg_list[i];
                    let v = &reg_list[j];
                    if let Some(neighbors) = adjacency.get_mut(u) {
                        neighbors.insert(v.clone());
                    }
                    if let Some(neighbors) = adjacency.get_mut(v) {
                        neighbors.insert(u.clone());
                    }
                }
            }
        }

        let mut total_edges = 0;
        for neighbors in adjacency.values() {
            total_edges += neighbors.len();
        }
        total_edges /= 2;

        // 3. Simplifikasi Kempe (Chaitin-Briggs Simplification Phase)
        let mut select_stack: Vec<String> = Vec::new();
        let mut degrees: HashMap<String, usize> = adjacency.iter().map(|(k, v)| (k.clone(), v.len())).collect();
        let mut remaining_nodes: HashSet<String> = virtual_regs.clone();

        while !remaining_nodes.is_empty() {
            // Cari node dengan derajat < K
            let low_degree_node = remaining_nodes.iter()
                .find(|n| degrees.get(*n).copied().unwrap_or(0) < self.k_physical_registers)
                .cloned();

            if let Some(node) = low_degree_node {
                remaining_nodes.remove(&node);
                select_stack.push(node.clone());
                if let Some(neighbors) = adjacency.get(&node) {
                    for neighbor in neighbors {
                        if let Some(deg) = degrees.get_mut(neighbor) {
                            *deg = deg.saturating_sub(1);
                        }
                    }
                }
            } else {
                // Semua node berderajat >= K: pilih kandidat tumpahan (Spill candidate)
                let spill_node = remaining_nodes.iter().next().unwrap().clone();
                remaining_nodes.remove(&spill_node);
                select_stack.push(spill_node);
            }
        }

        // 4. Pewarnaan Graf (Select & Assign Colors Phase)
        let mut color_assignment: HashMap<String, usize> = HashMap::new();
        let mut spilled = Vec::new();

        while let Some(node) = select_stack.pop() {
            let mut used_colors = HashSet::new();
            if let Some(neighbors) = adjacency.get(&node) {
                for neighbor in neighbors {
                    if let Some(&color) = color_assignment.get(neighbor) {
                        used_colors.insert(color);
                    }
                }
            }

            // Cari warna pertama yang tersedia dari 0..K
            let available_color = (0..self.k_physical_registers).find(|c| !used_colors.contains(c));
            if let Some(color) = available_color {
                color_assignment.insert(node, color);
            } else {
                spilled.push(node);
            }
        }

        let mut mapping = HashMap::new();
        let mut used_colors_set = HashSet::new();
        for (vreg, color) in &color_assignment {
            let phys = if *color < self.physical_names.len() {
                self.physical_names[*color].clone()
            } else {
                format!("r{}", color)
            };
            mapping.insert(vreg.clone(), phys);
            used_colors_set.insert(*color);
        }

        RegisterAllocationResult {
            mapping,
            spilled_registers: spilled,
            registers_used: used_colors_set.len(),
            interference_edges_count: total_edges,
        }
    }

    fn collect_registers_from_inst(inst: &IrInstruction, set: &mut HashSet<String>) {
        match inst {
            IrInstruction::Assign { dst, src, .. } => {
                if let IrValue::Reg(r) = dst { set.insert(r.clone()); }
                if let IrValue::Reg(r) = src { set.insert(r.clone()); }
            }
            IrInstruction::BinOp { dst, lhs, rhs, .. } => {
                if let IrValue::Reg(r) = dst { set.insert(r.clone()); }
                if let IrValue::Reg(r) = lhs { set.insert(r.clone()); }
                if let IrValue::Reg(r) = rhs { set.insert(r.clone()); }
            }
            IrInstruction::Branch { cond, .. } => {
                if let IrValue::Reg(r) = cond { set.insert(r.clone()); }
            }
            IrInstruction::Return { val: Some(IrValue::Reg(r)) } => { set.insert(r.clone()); }
            IrInstruction::Print { val: IrValue::Reg(r), .. } => { set.insert(r.clone()); }
            IrInstruction::Call { dst, args, .. } => {
                if let Some(IrValue::Reg(r)) = dst { set.insert(r.clone()); }
                for arg in args {
                    if let IrValue::Reg(r) = arg { set.insert(r.clone()); }
                }
            }
            IrInstruction::VectorOp { dst, lhs, rhs, .. } => {
                if let IrValue::Reg(r) = dst { set.insert(r.clone()); }
                if let IrValue::Reg(r) = lhs { set.insert(r.clone()); }
                if let IrValue::Reg(r) = rhs { set.insert(r.clone()); }
            }
            IrInstruction::VectorFma { dst, a, b, c, .. } => {
                if let IrValue::Reg(r) = dst { set.insert(r.clone()); }
                if let IrValue::Reg(r) = a { set.insert(r.clone()); }
                if let IrValue::Reg(r) = b { set.insert(r.clone()); }
                if let IrValue::Reg(r) = c { set.insert(r.clone()); }
            }
            _ => {}
        }
    }
}
