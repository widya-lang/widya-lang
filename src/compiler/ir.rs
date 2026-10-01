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
}
