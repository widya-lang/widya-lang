//! Mid-Level Intermediate Representation (MIR) Engine for Widya-Lang
//! Model representasi perantara berbasis Control Flow Graph (CFG) eksplisit,
//! SSA places, projections, explicit terminators, dan basic blocks selevel rustc MIR.

use crate::ir::{IrBinOp, IrType, IrValue};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Identifikasi lokal / variabel pada MIR (argumen, lokal pengguna, dan temporer kompilator)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Local {
    pub index: usize,
    pub name: String,
    pub ty: IrType,
    pub is_user_variable: bool,
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_{}({})", self.index, self.name)
    }
}

/// Proyeksi akses memori (Deref, Field, Index) untuk Place
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectionElem {
    Deref,
    Field(String),
    Index(usize),
}

/// Lokasi penyimpanan memori (Place) dalam MIR, setara rustc `Place`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Place {
    pub local: Local,
    pub projection: Vec<ProjectionElem>,
}

impl Place {
    pub fn from_local(local: Local) -> Self {
        Self {
            local,
            projection: Vec::new(),
        }
    }

    pub fn deref(mut self) -> Self {
        self.projection.push(ProjectionElem::Deref);
        self
    }

    pub fn field(mut self, name: &str) -> Self {
        self.projection.push(ProjectionElem::Field(name.to_string()));
        self
    }

    pub fn base_name(&self) -> &str {
        &self.local.name
    }

    /// Cek apakah suatu place menaungi atau beririsan dengan place lain (aliasing prefix)
    pub fn overlaps_with(&self, other: &Place) -> bool {
        if self.local != other.local {
            return false;
        }
        let min_len = self.projection.len().min(other.projection.len());
        self.projection[..min_len] == other.projection[..min_len]
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.local)?;
        for p in &self.projection {
            match p {
                ProjectionElem::Deref => write!(f, ".*")?,
                ProjectionElem::Field(name) => write!(f, ".{}", name)?,
                ProjectionElem::Index(i) => write!(f, "[{}]", i)?,
            }
        }
        Ok(())
    }
}

/// Jenis peminjaman memori pada MIR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorrowKind {
    /// Peminjaman Imutabel (Shared: &T)
    Shared,
    /// Peminjaman Mutabel (Unique/Exclusive: &mut T)
    Mut,
}

impl fmt::Display for BorrowKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorrowKind::Shared => write!(f, "&"),
            BorrowKind::Mut => write!(f, "&mut "),
        }
    }
}

/// Operand (Rvalue source): Salin, Pindah (Move), atau Konstanta
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Copy(Place),
    Move(Place),
    Constant(IrValue),
}

impl Operand {
    pub fn place(&self) -> Option<&Place> {
        match self {
            Operand::Copy(p) | Operand::Move(p) => Some(p),
            Operand::Constant(_) => None,
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Copy(p) => write!(f, "salin {}", p),
            Operand::Move(p) => write!(f, "pindah {}", p),
            Operand::Constant(c) => write!(f, "{}", c),
        }
    }
}

/// Evaluasi nilai sisi kanan (Rvalue)
#[derive(Debug, Clone, PartialEq)]
pub enum Rvalue {
    Use(Operand),
    Ref(BorrowKind, Place),
    BinaryOp(IrBinOp, Operand, Operand),
    UnaryNot(Operand),
    UnaryNeg(Operand),
    Discriminant(Place),
    Aggregate(IrType, Vec<Operand>),
}

/// Instruksi sekuensial (Statement) di dalam Basic Block
#[derive(Debug, Clone, PartialEq)]
pub enum MirStatement {
    Assign(Place, Rvalue),
    StorageLive(Local),
    StorageDead(Local),
    Nop,
}

/// Terminator: Instruksi penutup Basic Block yang menentukan transfer kendali CFG
#[derive(Debug, Clone, PartialEq)]
pub enum MirTerminator {
    Goto {
        target: usize,
    },
    SwitchInt {
        discr: Operand,
        targets: Vec<(i64, usize)>,
        otherwise: usize,
    },
    Return,
    Call {
        func_name: String,
        args: Vec<Operand>,
        destination: Place,
        target: Option<usize>,
    },
    Drop {
        place: Place,
        target: usize,
    },
    Unreachable,
}

/// Basic Block dalam graf alur kendali MIR
#[derive(Debug, Clone, PartialEq)]
pub struct MirBasicBlock {
    pub id: usize,
    pub statements: Vec<MirStatement>,
    pub terminator: Option<MirTerminator>,
}

impl MirBasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            statements: Vec::new(),
            terminator: None,
        }
    }

    pub fn push_stmt(&mut self, stmt: MirStatement) {
        self.statements.push(stmt);
    }

    pub fn set_terminator(&mut self, term: MirTerminator) {
        self.terminator = Some(term);
    }

    pub fn successors(&self) -> Vec<usize> {
        match &self.terminator {
            Some(MirTerminator::Goto { target }) => vec![*target],
            Some(MirTerminator::SwitchInt { targets, otherwise, .. }) => {
                let mut succ: Vec<usize> = targets.iter().map(|(_, t)| *t).collect();
                succ.push(*otherwise);
                succ
            }
            Some(MirTerminator::Call { target: Some(t), .. }) => vec![*t],
            Some(MirTerminator::Drop { target, .. }) => vec![*target],
            _ => Vec::new(),
        }
    }
}

/// Tubuh Fungsi MIR Lengkap
#[derive(Debug, Clone)]
pub struct MirBody {
    pub name: String,
    pub arg_count: usize,
    pub return_type: IrType,
    pub locals: Vec<Local>,
    pub basic_blocks: Vec<MirBasicBlock>,
}

impl MirBody {
    pub fn new(name: String, arg_count: usize, return_type: IrType) -> Self {
        Self {
            name,
            arg_count,
            return_type,
            locals: Vec::new(),
            basic_blocks: Vec::new(),
        }
    }

    pub fn add_local(&mut self, name: &str, ty: IrType, is_user: bool) -> Local {
        let local = Local {
            index: self.locals.len(),
            name: name.to_string(),
            ty,
            is_user_variable: is_user,
        };
        self.locals.push(local.clone());
        local
    }

    pub fn add_block(&mut self) -> usize {
        let id = self.basic_blocks.len();
        self.basic_blocks.push(MirBasicBlock::new(id));
        id
    }

    /// Menghitung relasi pendahulu (predecessors) untuk setiap basic block
    pub fn predecessors(&self) -> Vec<Vec<usize>> {
        let mut preds = vec![Vec::new(); self.basic_blocks.len()];
        for bb in &self.basic_blocks {
            for succ in bb.successors() {
                if succ < preds.len() {
                    preds[succ].push(bb.id);
                }
            }
        }
        preds
    }

    /// Render representasi teks MIR yang human-readable
    pub fn dump_mir(&self) -> String {
        let mut out = format!("// MIR Body: fn {} ({} args) -> {}\n", self.name, self.arg_count, self.return_type);
        out.push_str("locals {\n");
        for local in &self.locals {
            out.push_str(&format!("    _{}: {} ({})\n", local.index, local.ty, local.name));
        }
        out.push_str("}\n\n");

        for bb in &self.basic_blocks {
            out.push_str(&format!("bb{}: {{\n", bb.id));
            for stmt in &bb.statements {
                match stmt {
                    MirStatement::Assign(place, rvalue) => {
                        out.push_str(&format!("    {} = {:?};\n", place, rvalue));
                    }
                    MirStatement::StorageLive(l) => {
                        out.push_str(&format!("    StorageLive({});\n", l));
                    }
                    MirStatement::StorageDead(l) => {
                        out.push_str(&format!("    StorageDead({});\n", l));
                    }
                    MirStatement::Nop => {
                        out.push_str("    nop;\n");
                    }
                }
            }
            if let Some(term) = &bb.terminator {
                out.push_str(&format!("    terminator: {:?};\n", term));
            } else {
                out.push_str("    // <missing terminator>\n");
            }
            out.push_str("}\n\n");
        }
        out
    }
}
