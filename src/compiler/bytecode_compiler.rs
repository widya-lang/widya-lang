//! AST-to-Bytecode Compiler (bytecode_compiler.rs)
//! Mengompilasi AST Widya-Lang (`Program`, `Stmt`, `Expr`) langsung menjadi `BytecodeChunk` & `CompilationUnit`
//! untuk dieksekusi oleh Register-Based Virtual Machine tanpa ketergantungan pada backend eksternal.

use crate::ast::*;
use crate::compiler::vm::{BytecodeChunk, Opcode, VirtualMachine, VmValue};
use crate::error::Galat;
use std::collections::HashMap;

/// Unit Kompilasi Bytecode Lengkap (Koleksi Chunk Program Utama dan Fungsi)
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub chunks: Vec<BytecodeChunk>,
    pub function_map: HashMap<String, u16>,
    pub native_function_names: Vec<String>,
    pub main_chunk_idx: usize,
}

impl CompilationUnit {
    /// Mendaftarkan seluruh fungsi stdlib yang digunakan unit ini ke VirtualMachine
    pub fn bind_natives(&self, vm: &mut VirtualMachine) {
        let mut std_env = crate::environment::Environment::new();
        crate::stdlib::register_stdlib(&mut std_env);
        let all_std = std_env.get_all_local();

        for (idx, name) in self.native_function_names.iter().enumerate() {
            if let Some(crate::value::Value::Builtin(b)) = all_std.get(name).cloned() {
                let func_ptr = b.func;
                let fn_name = b.name.clone();
                let arity = b.arity;
                vm.register_native(idx as u16, move |vm_args: &[VmValue]| {
                    if let Some(expected) = arity {
                        if vm_args.len() != expected {
                            return Err(format!(
                                "Fungsi bawaan '{}' mengharapkan {} argumen, menerima {}",
                                fn_name,
                                expected,
                                vm_args.len()
                            ));
                        }
                    }
                    let rt_args: Vec<crate::value::Value> =
                        vm_args.iter().map(|v| v.to_runtime_value()).collect();
                    let span = crate::error::Span::new(1, 1);
                    match (func_ptr)(&rt_args, &span) {
                        Ok(val) => Ok(VmValue::from_runtime_value(&val)),
                        Err(e) => Err(format!("{}", e)),
                    }
                });
            }
        }
    }

    /// Menjalankan program bytecode menggunakan Virtual Machine
    pub fn run(&self) -> Result<VmValue, String> {
        let mut vm = VirtualMachine::new();
        self.bind_natives(&mut vm);
        for chunk in &self.chunks {
            vm.load_chunk(chunk.clone());
        }
        vm.execute(self.main_chunk_idx)
    }
}

pub struct BytecodeCompiler {
    chunks: Vec<BytecodeChunk>,
    current_chunk_idx: usize,
    function_map: HashMap<String, u16>,
    native_map: HashMap<String, u16>,
    native_names: Vec<String>,
    symbol_table: HashMap<String, u8>,
    next_register: u8,
    current_function_name: Option<String>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        Self {
            chunks: vec![BytecodeChunk::new()],
            current_chunk_idx: 0,
            function_map: HashMap::new(),
            native_map: HashMap::new(),
            native_names: Vec::new(),
            symbol_table: HashMap::new(),
            next_register: 0,
            current_function_name: None,
        }
    }

    fn get_or_register_native(&mut self, name: &str) -> u16 {
        if let Some(&id) = self.native_map.get(name) {
            return id;
        }
        let id = self.native_names.len() as u16;
        self.native_names.push(name.to_string());
        self.native_map.insert(name.to_string(), id);
        id
    }

    fn current_chunk(&mut self) -> &mut BytecodeChunk {
        &mut self.chunks[self.current_chunk_idx]
    }

    fn alloc_reg(&mut self) -> u8 {
        let r = self.next_register;
        self.next_register = self.next_register.saturating_add(1);
        r
    }

    /// Mengompilasi program AST menjadi satu biner BytecodeChunk utama (Backward Compatible)
    pub fn compile(self, program: &Program) -> Result<BytecodeChunk, Galat> {
        let unit = self.compile_program(program)?;
        Ok(unit.chunks[unit.main_chunk_idx].clone())
    }

    /// Mengompilasi program AST lengkap menjadi CompilationUnit multi-chunk (Fungsi & TCO)
    pub fn compile_program(mut self, program: &Program) -> Result<CompilationUnit, Galat> {
        // Pass 1: Daftarkan seluruh deklarasi fungsi
        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, .. } = stmt {
                let fn_idx = self.chunks.len() as u16;
                self.function_map.insert(name.clone(), fn_idx);
                self.chunks.push(BytecodeChunk::new());
            }
        }

        // Pass 2: Kompilasi setiap fungsi ke chunk tersendiri
        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, params, body, span, .. } = stmt {
                let fn_idx = self.function_map[name] as usize;
                self.current_chunk_idx = fn_idx;
                self.current_function_name = Some(name.clone());
                self.symbol_table.clear();

                for (i, p) in params.iter().enumerate() {
                    let clean_p = p.trim_start_matches('*').to_string();
                    self.symbol_table.insert(clean_p, i as u8);
                }
                self.next_register = params.len() as u8;

                for s in body {
                    self.compile_stmt(s)?;
                }

                // Jika fungsi tidak diakhiri Return, emit return nil otomatis
                let is_ret = self.current_chunk().instructions.last().map(|op| {
                    matches!(op, Opcode::Return(_) | Opcode::TailCall(_, _, _))
                }).unwrap_or(false);

                if !is_ret {
                    let nil_const = self.current_chunk().add_constant(VmValue::Nil);
                    let nil_reg = self.alloc_reg();
                    self.current_chunk().emit(Opcode::LoadConst(nil_reg, nil_const), span.line);
                    self.current_chunk().emit(Opcode::Return(nil_reg), span.line);
                }

                self.current_function_name = None;
            }
        }

        // Pass 3: Kompilasi program utama pada chunk 0
        self.current_chunk_idx = 0;
        self.current_function_name = None;
        self.symbol_table.clear();
        self.next_register = 0;

        for stmt in &program.statements {
            if !matches!(stmt, Stmt::FunctionDecl { .. }) {
                self.compile_stmt(stmt)?;
            }
        }

        // Emit Halt pada akhir chunk utama jika belum berhenti
        let is_terminated = self.current_chunk().instructions.last().map(|op| {
            matches!(op, Opcode::Halt | Opcode::Return(_))
        }).unwrap_or(false);

        if !is_terminated {
            self.current_chunk().emit(Opcode::Halt, 0);
        }

        Ok(CompilationUnit {
            chunks: self.chunks,
            function_map: self.function_map,
            native_function_names: self.native_names,
            main_chunk_idx: 0,
        })
    }

    pub fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), Galat> {
        match stmt {
            Stmt::VarDecl { name, initializer, span, .. } => {
                let reg = if let Some(&existing) = self.symbol_table.get(name) {
                    existing
                } else {
                    let r = self.alloc_reg();
                    self.symbol_table.insert(name.clone(), r);
                    r
                };

                if let Some(init) = initializer {
                    let val_reg = self.compile_expr(init)?;
                    self.current_chunk().emit(Opcode::Move(reg, val_reg), span.line);
                } else {
                    let const_idx = self.current_chunk().add_constant(VmValue::Nil);
                    self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                }
            }
            Stmt::Assignment { target, op, value, span } => {
                let val_reg = self.compile_expr(value)?;
                match target {
                    AssignTarget::Variable(name) => {
                        let reg = if let Some(&r) = self.symbol_table.get(name) {
                            r
                        } else {
                            let r = self.alloc_reg();
                            self.symbol_table.insert(name.clone(), r);
                            r
                        };
                        match op {
                            AssignOp::Assign => {
                                self.current_chunk().emit(Opcode::Move(reg, val_reg), span.line);
                            }
                            AssignOp::PlusAssign => {
                                self.current_chunk().emit(Opcode::Add(reg, reg, val_reg), span.line);
                            }
                            AssignOp::MinusAssign => {
                                self.current_chunk().emit(Opcode::Sub(reg, reg, val_reg), span.line);
                            }
                            AssignOp::StarAssign => {
                                self.current_chunk().emit(Opcode::Mul(reg, reg, val_reg), span.line);
                            }
                            AssignOp::SlashAssign => {
                                self.current_chunk().emit(Opcode::Div(reg, reg, val_reg), span.line);
                            }
                        }
                    }
                    AssignTarget::Index { target: arr_expr, index: idx_expr } => {
                        let arr_reg = if let Expr::Identifier(ref var_name, _) = **arr_expr {
                            if let Some(&sym_reg) = self.symbol_table.get(var_name) {
                                sym_reg
                            } else {
                                self.compile_expr(arr_expr)?
                            }
                        } else {
                            self.compile_expr(arr_expr)?
                        };
                        let idx_reg = self.compile_expr(idx_expr)?;
                        self.current_chunk().emit(Opcode::ArraySet(arr_reg, idx_reg, val_reg), span.line);
                    }
                }
            }
            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
            }
            Stmt::Block(stmts, _) => {
                for s in stmts {
                    self.compile_stmt(s)?;
                }
            }
            Stmt::If { condition, then_branch, elif_branches, else_branch, span } => {
                if !elif_branches.is_empty() {
                    let mut folded_else = else_branch.clone();
                    for (elif_cond, elif_body) in elif_branches.iter().rev() {
                        folded_else = Some(Box::new(Stmt::If {
                            condition: elif_cond.clone(),
                            then_branch: Box::new(elif_body.clone()),
                            elif_branches: Vec::new(),
                            else_branch: folded_else,
                            span: span.clone(),
                        }));
                    }
                    let desugared = Stmt::If {
                        condition: condition.clone(),
                        then_branch: then_branch.clone(),
                        elif_branches: Vec::new(),
                        else_branch: folded_else,
                        span: span.clone(),
                    };
                    return self.compile_stmt(&desugared);
                }

                let cond_reg = self.compile_expr(condition)?;

                // JumpIfFalse placeholder
                let jump_if_false_idx = self.current_chunk().instructions.len();
                self.current_chunk().emit(Opcode::JumpIfFalse(cond_reg, 0), span.line);

                self.compile_stmt(then_branch)?;

                if let Some(else_b) = else_branch {
                    // Jump placeholder past else
                    let jump_past_else_idx = self.current_chunk().instructions.len();
                    self.current_chunk().emit(Opcode::Jump(0), span.line);

                    // Patch JumpIfFalse target
                    let after_then = self.current_chunk().instructions.len();
                    let offset_false = (after_then as i32) - (jump_if_false_idx as i32) - 1;
                    self.current_chunk().instructions[jump_if_false_idx] = Opcode::JumpIfFalse(cond_reg, offset_false);

                    self.compile_stmt(else_b)?;

                    // Patch Jump target
                    let after_else = self.current_chunk().instructions.len();
                    let offset_else = (after_else as i32) - (jump_past_else_idx as i32) - 1;
                    self.current_chunk().instructions[jump_past_else_idx] = Opcode::Jump(offset_else);
                } else {
                    let after_then = self.current_chunk().instructions.len();
                    let offset_false = (after_then as i32) - (jump_if_false_idx as i32) - 1;
                    self.current_chunk().instructions[jump_if_false_idx] = Opcode::JumpIfFalse(cond_reg, offset_false);
                }
            }
            Stmt::While { condition, body, span } => {
                let loop_start = self.current_chunk().instructions.len();
                let cond_reg = self.compile_expr(condition)?;

                let jump_out_idx = self.current_chunk().instructions.len();
                self.current_chunk().emit(Opcode::JumpIfFalse(cond_reg, 0), span.line);

                self.compile_stmt(body)?;

                // Jump back to loop start
                let current_pos = self.current_chunk().instructions.len();
                let back_offset = (loop_start as i32) - (current_pos as i32) - 1;
                self.current_chunk().emit(Opcode::Jump(back_offset), span.line);

                // Patch jump_out
                let loop_end = self.current_chunk().instructions.len();
                let out_offset = (loop_end as i32) - (jump_out_idx as i32) - 1;
                self.current_chunk().instructions[jump_out_idx] = Opcode::JumpIfFalse(cond_reg, out_offset);
            }
            Stmt::Loop { body, span } => {
                let loop_start = self.current_chunk().instructions.len();
                self.compile_stmt(body)?;

                let current_pos = self.current_chunk().instructions.len();
                let back_offset = (loop_start as i32) - (current_pos as i32) - 1;
                self.current_chunk().emit(Opcode::Jump(back_offset), span.line);
            }
            Stmt::ForIn { var_name, iterable, body, span } => {
                if let Expr::Range { start, end, .. } = iterable {
                    let var_reg = self.alloc_reg();
                    self.symbol_table.insert(var_name.clone(), var_reg);

                    let start_reg = self.compile_expr(start)?;
                    self.current_chunk().emit(Opcode::Move(var_reg, start_reg), span.line);
                    let end_reg = self.compile_expr(end)?;

                    let loop_start = self.current_chunk().instructions.len();
                    let cond_reg = self.alloc_reg();
                    self.current_chunk().emit(Opcode::Lt(cond_reg, var_reg, end_reg), span.line);

                    let jump_exit_idx = self.current_chunk().instructions.len();
                    self.current_chunk().emit(Opcode::JumpIfFalse(cond_reg, 0), span.line);

                    self.compile_stmt(body)?;

                    // Increment loop counter
                    let one_const = self.current_chunk().add_constant(VmValue::Int(1));
                    let one_reg = self.alloc_reg();
                    self.current_chunk().emit(Opcode::LoadConst(one_reg, one_const), span.line);
                    self.current_chunk().emit(Opcode::Add(var_reg, var_reg, one_reg), span.line);

                    let current_pos = self.current_chunk().instructions.len();
                    let back_offset = (loop_start as i32) - (current_pos as i32) - 1;
                    self.current_chunk().emit(Opcode::Jump(back_offset), span.line);

                    let after_loop = self.current_chunk().instructions.len();
                    let exit_offset = (after_loop as i32) - (jump_exit_idx as i32) - 1;
                    self.current_chunk().instructions[jump_exit_idx] = Opcode::JumpIfFalse(cond_reg, exit_offset);
                } else {
                    // Iterasi atas Array / String menggunakan indeks internal
                    let arr_reg = self.compile_expr(iterable)?;
                    let len_reg = self.alloc_reg();
                    self.current_chunk().emit(Opcode::ArrayLen(len_reg, arr_reg), span.line);

                    let idx_reg = self.alloc_reg();
                    let zero_const = self.current_chunk().add_constant(VmValue::Int(0));
                    self.current_chunk().emit(Opcode::LoadConst(idx_reg, zero_const), span.line);

                    let var_reg = self.alloc_reg();
                    self.symbol_table.insert(var_name.clone(), var_reg);

                    let loop_start = self.current_chunk().instructions.len();
                    let cond_reg = self.alloc_reg();
                    self.current_chunk().emit(Opcode::Lt(cond_reg, idx_reg, len_reg), span.line);

                    let jump_exit_idx = self.current_chunk().instructions.len();
                    self.current_chunk().emit(Opcode::JumpIfFalse(cond_reg, 0), span.line);

                    self.current_chunk().emit(Opcode::ArrayGet(var_reg, arr_reg, idx_reg), span.line);

                    self.compile_stmt(body)?;

                    let one_const = self.current_chunk().add_constant(VmValue::Int(1));
                    let one_reg = self.alloc_reg();
                    self.current_chunk().emit(Opcode::LoadConst(one_reg, one_const), span.line);
                    self.current_chunk().emit(Opcode::Add(idx_reg, idx_reg, one_reg), span.line);

                    let current_pos = self.current_chunk().instructions.len();
                    let back_offset = (loop_start as i32) - (current_pos as i32) - 1;
                    self.current_chunk().emit(Opcode::Jump(back_offset), span.line);

                    let after_loop = self.current_chunk().instructions.len();
                    let exit_offset = (after_loop as i32) - (jump_exit_idx as i32) - 1;
                    self.current_chunk().instructions[jump_exit_idx] = Opcode::JumpIfFalse(cond_reg, exit_offset);
                }
            }
            Stmt::TryCatch { try_block, .. } => {
                self.compile_stmt(try_block)?;
            }
            Stmt::Return { value, span } => {
                // Deteksi Otomatis Tail-Call Optimization (TCO)
                if let Some(Expr::Call { callee, arguments, span: call_span }) = value {
                    if let Expr::Identifier(ref fn_name, _) = **callee {
                        if let Some(&target_idx) = self.function_map.get(fn_name) {
                            let mut arg_regs = Vec::new();
                            for arg in arguments {
                                arg_regs.push(self.compile_expr(arg)?);
                            }
                            let contiguous_start = self.alloc_reg();
                            for (i, &arg_r) in arg_regs.iter().enumerate() {
                                if i > 0 { self.alloc_reg(); }
                                let target_reg = contiguous_start + (i as u8);
                                self.current_chunk().emit(Opcode::Move(target_reg, arg_r), call_span.line);
                            }
                            self.current_chunk().emit(
                                Opcode::TailCall(target_idx, contiguous_start, arguments.len() as u8),
                                span.line,
                            );
                            return Ok(());
                        }
                    }
                }

                let ret_reg = if let Some(v) = value {
                    self.compile_expr(v)?
                } else {
                    let r = self.alloc_reg();
                    let c = self.current_chunk().add_constant(VmValue::Nil);
                    self.current_chunk().emit(Opcode::LoadConst(r, c), span.line);
                    r
                };
                self.current_chunk().emit(Opcode::Return(ret_reg), span.line);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn compile_expr(&mut self, expr: &Expr) -> Result<u8, Galat> {
        let span = expr.span();
        match expr {
            Expr::Number(n, _) => {
                let reg = self.alloc_reg();
                let const_idx = if n.fract() == 0.0 && *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
                    self.current_chunk().add_constant(VmValue::Int(*n as i64))
                } else {
                    self.current_chunk().add_constant(VmValue::Float(*n))
                };
                self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                Ok(reg)
            }
            Expr::String(s, _) => {
                let reg = self.alloc_reg();
                let const_idx = self.current_chunk().add_constant(VmValue::String(s.clone()));
                self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                Ok(reg)
            }
            Expr::Boolean(b, _) => {
                let reg = self.alloc_reg();
                let const_idx = self.current_chunk().add_constant(VmValue::Bool(*b));
                self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                Ok(reg)
            }
            Expr::Nil(_) => {
                let reg = self.alloc_reg();
                let const_idx = self.current_chunk().add_constant(VmValue::Nil);
                self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                Ok(reg)
            }
            Expr::Identifier(name, _) => {
                if let Some(&reg) = self.symbol_table.get(name) {
                    let dst = self.alloc_reg();
                    self.current_chunk().emit(Opcode::Move(dst, reg), span.line);
                    Ok(dst)
                } else {
                    let reg = self.alloc_reg();
                    let const_idx = self.current_chunk().add_constant(VmValue::Nil);
                    self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                    Ok(reg)
                }
            }
            Expr::Grouping(inner, _) => {
                self.compile_expr(inner)
            }
            Expr::Unary { op, right, span } => {
                let r = self.compile_expr(right)?;
                let dst = self.alloc_reg();
                match op {
                    UnaryOp::Negate => {
                        let zero_const = self.current_chunk().add_constant(VmValue::Int(0));
                        let zero_reg = self.alloc_reg();
                        self.current_chunk().emit(Opcode::LoadConst(zero_reg, zero_const), span.line);
                        self.current_chunk().emit(Opcode::Sub(dst, zero_reg, r), span.line);
                    }
                    UnaryOp::Not => {
                        let false_const = self.current_chunk().add_constant(VmValue::Bool(false));
                        let false_reg = self.alloc_reg();
                        self.current_chunk().emit(Opcode::LoadConst(false_reg, false_const), span.line);
                        self.current_chunk().emit(Opcode::Eq(dst, r, false_reg), span.line);
                    }
                }
                Ok(dst)
            }
            Expr::Binary { left, op, right, span } => {
                let lhs = self.compile_expr(left)?;
                let rhs = self.compile_expr(right)?;
                let dst = self.alloc_reg();

                match op {
                    BinaryOp::Add => self.current_chunk().emit(Opcode::Add(dst, lhs, rhs), span.line),
                    BinaryOp::Subtract => self.current_chunk().emit(Opcode::Sub(dst, lhs, rhs), span.line),
                    BinaryOp::Multiply => self.current_chunk().emit(Opcode::Mul(dst, lhs, rhs), span.line),
                    BinaryOp::Divide => self.current_chunk().emit(Opcode::Div(dst, lhs, rhs), span.line),
                    BinaryOp::Modulo => self.current_chunk().emit(Opcode::Mod(dst, lhs, rhs), span.line),
                    BinaryOp::Equal => self.current_chunk().emit(Opcode::Eq(dst, lhs, rhs), span.line),
                    BinaryOp::NotEqual => {
                        let eq_reg = self.alloc_reg();
                        self.current_chunk().emit(Opcode::Eq(eq_reg, lhs, rhs), span.line);
                        let false_const = self.current_chunk().add_constant(VmValue::Bool(false));
                        let false_reg = self.alloc_reg();
                        self.current_chunk().emit(Opcode::LoadConst(false_reg, false_const), span.line);
                        self.current_chunk().emit(Opcode::Eq(dst, eq_reg, false_reg), span.line)
                    }
                    BinaryOp::Less => self.current_chunk().emit(Opcode::Lt(dst, lhs, rhs), span.line),
                    BinaryOp::LessEqual => self.current_chunk().emit(Opcode::Lte(dst, lhs, rhs), span.line),
                    BinaryOp::Greater => self.current_chunk().emit(Opcode::Gt(dst, lhs, rhs), span.line),
                    BinaryOp::GreaterEqual => self.current_chunk().emit(Opcode::Gte(dst, lhs, rhs), span.line),
                    BinaryOp::And => self.current_chunk().emit(Opcode::BitAnd(dst, lhs, rhs), span.line),
                    BinaryOp::Or => self.current_chunk().emit(Opcode::BitOr(dst, lhs, rhs), span.line),
                    _ => self.current_chunk().emit(Opcode::Add(dst, lhs, rhs), span.line),
                };

                Ok(dst)
            }
            Expr::Call { callee, arguments, span } => {
                if let Expr::Identifier(ref name, _) = **callee {
                    if name == "cetak" || name == "print" || name == "tulis" {
                        let newline = name != "tulis";
                        let total_args = arguments.len();
                        for (i, arg) in arguments.iter().enumerate() {
                            let r = self.compile_expr(arg)?;
                            let is_last = i + 1 == total_args;
                            self.current_chunk().emit(Opcode::Print(r, newline && is_last), span.line);
                        }
                        let ret_reg = self.alloc_reg();
                        let nil_c = self.current_chunk().add_constant(VmValue::Nil);
                        self.current_chunk().emit(Opcode::LoadConst(ret_reg, nil_c), span.line);
                        return Ok(ret_reg);
                    }

                    if let Some(&fn_idx) = self.function_map.get(name) {
                        let mut arg_regs = Vec::new();
                        for arg in arguments {
                            arg_regs.push(self.compile_expr(arg)?);
                        }
                        let contiguous_start = self.alloc_reg();
                        for (i, &arg_r) in arg_regs.iter().enumerate() {
                            if i > 0 { self.alloc_reg(); }
                            let target_reg = contiguous_start + (i as u8);
                            self.current_chunk().emit(Opcode::Move(target_reg, arg_r), span.line);
                        }
                        let dst = self.alloc_reg();
                        self.current_chunk().emit(
                            Opcode::Call(dst, fn_idx, contiguous_start, arguments.len() as u8),
                            span.line,
                        );
                        return Ok(dst);
                    }

                    if (name == "panjang" || name == "len") && arguments.len() == 1 {
                        let arr_r = self.compile_expr(&arguments[0])?;
                        let dst = self.alloc_reg();
                        self.current_chunk().emit(Opcode::ArrayLen(dst, arr_r), span.line);
                        return Ok(dst);
                    }

                    if (name == "tambah" || name == "push") && arguments.len() == 2 {
                        let arr_r = if let Expr::Identifier(ref var_name, _) = arguments[0] {
                            if let Some(&sym_reg) = self.symbol_table.get(var_name) {
                                sym_reg
                            } else {
                                self.compile_expr(&arguments[0])?
                            }
                        } else {
                            self.compile_expr(&arguments[0])?
                        };
                        let val_r = self.compile_expr(&arguments[1])?;
                        self.current_chunk().emit(Opcode::ArrayPush(arr_r, val_r), span.line);
                        return Ok(arr_r);
                    }

                    // Fallback ke CallNative untuk fungsi stdlib apa pun!
                    let native_id = self.get_or_register_native(name);
                    let mut arg_regs = Vec::new();
                    for arg in arguments {
                        arg_regs.push(self.compile_expr(arg)?);
                    }
                    let contiguous_start = self.alloc_reg();
                    for (i, &arg_r) in arg_regs.iter().enumerate() {
                        if i > 0 {
                            self.alloc_reg();
                        }
                        let target_reg = contiguous_start + (i as u8);
                        self.current_chunk()
                            .emit(Opcode::Move(target_reg, arg_r), span.line);
                    }
                    let dst = self.alloc_reg();
                    self.current_chunk().emit(
                        Opcode::CallNative(dst, native_id, contiguous_start, arguments.len() as u8),
                        span.line,
                    );
                    return Ok(dst);
                }

                // Inline IIFE (Immediately Invoked Function Expression) untuk List/Dict Comprehension & Match
                if let Expr::FunctionExpr { params, body, .. } = &**callee {
                    for (p, arg) in params.iter().zip(arguments.iter()) {
                        let arg_r = self.compile_expr(arg)?;
                        self.symbol_table.insert(p.clone(), arg_r);
                    }
                    let mut result_reg = None;
                    for stmt in body {
                        if let Stmt::Return { value: Some(ret_expr), .. } = stmt {
                            result_reg = Some(self.compile_expr(ret_expr)?);
                            break;
                        } else {
                            self.compile_stmt(stmt)?;
                        }
                    }
                    if let Some(r) = result_reg {
                        return Ok(r);
                    }
                }

                let reg = self.alloc_reg();
                let const_idx = self.current_chunk().add_constant(VmValue::Nil);
                self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                Ok(reg)
            }
            Expr::Array(items, span) => {
                let dst = self.alloc_reg();
                self.current_chunk().emit(Opcode::ArrayNew(dst, items.len()), span.line);
                for item in items {
                    let item_reg = self.compile_expr(item)?;
                    self.current_chunk().emit(Opcode::ArrayPush(dst, item_reg), span.line);
                }
                Ok(dst)
            }
            Expr::Index { target, index, span } => {
                let target_reg = self.compile_expr(target)?;
                let index_reg = self.compile_expr(index)?;
                let dst = self.alloc_reg();
                self.current_chunk().emit(Opcode::ArrayGet(dst, target_reg, index_reg), span.line);
                Ok(dst)
            }
            Expr::Map(entries, span) => {
                let dst = self.alloc_reg();
                self.current_chunk().emit(Opcode::MapNew(dst), span.line);
                for (k, v) in entries {
                    let k_reg = self.compile_expr(k)?;
                    let v_reg = self.compile_expr(v)?;
                    self.current_chunk().emit(Opcode::MapSet(dst, k_reg, v_reg), span.line);
                }
                Ok(dst)
            }
            _ => {
                let reg = self.alloc_reg();
                let const_idx = self.current_chunk().add_constant(VmValue::Nil);
                self.current_chunk().emit(Opcode::LoadConst(reg, const_idx), span.line);
                Ok(reg)
            }
        }
    }

    pub fn get_var_register(&self, name: &str) -> Option<u8> {
        self.symbol_table.get(name).copied()
    }
}
