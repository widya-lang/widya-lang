//! Mesin Virtual Bytecode Berbasis Register (Register-Based Bytecode VM)
//! Dirancang untuk eksekusi skrip sub-milidetik, live hot-reloading, dan konsumsi memori minimal.

use crate::compiler::decimal::Desimal;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum VmValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Decimal(Desimal),
    String(String),
    Array(Vec<VmValue>),
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
            VmValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
        }
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
    AddDec(u8, u8, u8),      // dst, lhs, rhs (Desimal Fixed-Point)
    SubDec(u8, u8, u8),      // dst, lhs, rhs
    MulDec(u8, u8, u8),      // dst, lhs, rhs
    DivDec(u8, u8, u8),      // dst, lhs, rhs
    Eq(u8, u8, u8),          // dst = (lhs == rhs)
    Lt(u8, u8, u8),          // dst = (lhs < rhs)
    Lte(u8, u8, u8),         // dst = (lhs <= rhs)
    Jump(i32),               // relative instruction offset
    JumpIfTrue(u8, i32),     // reg, offset
    JumpIfFalse(u8, i32),    // reg, offset
    Call(u8, u16, u8, u8),   // dst_reg, fn_idx, arg_start_reg, arg_count
    Return(u8),              // reg
    Print(u8, bool),         // reg, newline
    Halt,
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

pub struct VirtualMachine {
    pub chunks: Vec<BytecodeChunk>,
    pub registers: [VmValue; 256], // 256 virtual registers
    pub frames: Vec<CallFrame>,
    pub instruction_count: u64,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            registers: std::array::from_fn(|_| VmValue::Nil),
            frames: Vec::new(),
            instruction_count: 0,
        }
    }

    pub fn load_chunk(&mut self, chunk: BytecodeChunk) -> usize {
        let idx = self.chunks.len();
        self.chunks.push(chunk);
        idx
    }

    pub fn execute(&mut self, main_chunk_idx: usize) -> Result<VmValue, String> {
        let mut ip = 0;
        let chunk = &self.chunks[main_chunk_idx];

        while ip < chunk.instructions.len() {
            self.instruction_count += 1;
            let op = chunk.instructions[ip];
            ip += 1;

            match op {
                Opcode::LoadConst(reg, const_idx) => {
                    let val = chunk.constants[const_idx as usize].clone();
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
                        _ => return Err(format!("Operasi Mul tidak kompatibel")),
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
                        _ => return Err(format!("Operasi Div tidak kompatibel")),
                    };
                    self.registers[dst as usize] = res;
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
                Opcode::Jump(offset) => {
                    ip = ((ip as i32) + offset) as usize;
                }
                Opcode::JumpIfTrue(reg, offset) => {
                    if let VmValue::Bool(true) = self.registers[reg as usize] {
                        ip = ((ip as i32) + offset) as usize;
                    }
                }
                Opcode::JumpIfFalse(reg, offset) => {
                    if let VmValue::Bool(false) = self.registers[reg as usize] {
                        ip = ((ip as i32) + offset) as usize;
                    }
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
                    return Ok(self.registers[reg as usize].clone());
                }
                Opcode::Halt => {
                    break;
                }
                _ => {}
            }
        }

        Ok(self.registers[0].clone())
    }
}
