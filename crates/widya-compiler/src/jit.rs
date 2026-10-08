use widya_core::ast::*;
use widya_vm::bytecode_compiler::BytecodeCompiler;
use widya_vm::vm::{VirtualMachine, JitTier};
use widya_core::error::Galat;
use widya_core::lexer::Lexer;
use widya_core::parser::Parser;
use std::time::Instant;
use colored::*;

#[derive(Debug, Clone)]
pub struct JitExecutionReport {
    pub duration_ms: f64,
    pub mode: String,
    pub instructions_executed: u64,
    pub mips: f64,
    pub jit_tier: JitTier,
    pub osr_promotions: u32,
    pub inline_cache_hits: u64,
    pub deopt_bailouts: usize,
}

pub struct JitEngine {
    pub optimizations_enabled: bool,
}

impl JitEngine {
    pub fn new() -> Self {
        Self {
            optimizations_enabled: true,
        }
    }

    pub fn execute_jit(&self, source: &str) -> Result<f64, Galat> {
        let rep = self.execute_jit_advanced(source)?;
        Ok(rep.duration_ms)
    }

    pub fn execute_jit_advanced(&self, source: &str) -> Result<JitExecutionReport, Galat> {
        let start = Instant::now();

        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let mut program = parser.parse()?;

        // Perform AST optimizations before execution
        if self.optimizations_enabled {
            self.optimize_program(&mut program);
        }

        // Jalur 1: Coba kompilasi langsung ke Register Bytecode VM
        let compiler = BytecodeCompiler::new();
        if let Ok(compilation_unit) = compiler.compile_program(&program) {
            let mut vm = VirtualMachine::new();
            for chunk in &compilation_unit.chunks {
                vm.load_chunk(chunk.clone());
            }

            if let Ok(_val) = vm.execute_tiered(compilation_unit.main_chunk_idx) {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                let elapsed_sec = elapsed / 1000.0;
                let mips = if elapsed_sec > 0.0 && vm.instruction_count > 0 {
                    (vm.instruction_count as f64 / elapsed_sec) / 1_000_000.0
                } else {
                    0.0
                };
                let ic_hits: u64 = vm.ic_table.values().map(|ic| ic.entries.iter().map(|e| e.hits).sum::<u64>()).sum();

                return Ok(JitExecutionReport {
                    duration_ms: elapsed,
                    mode: format!("Register-VM Multi-Tier JIT ({:?}, NaN-Boxing, Tail-Call Opt)", vm.current_tier),
                    instructions_executed: vm.instruction_count,
                    mips,
                    jit_tier: vm.current_tier,
                    osr_promotions: vm.osr_promotions,
                    inline_cache_hits: ic_hits,
                    deopt_bailouts: vm.deopt_log.len(),
                });
            }
        }

        let compiler = BytecodeCompiler::new();
        let compilation_unit = compiler.compile_program(&program)?;
        let mut vm = VirtualMachine::new();
        for chunk in &compilation_unit.chunks {
            vm.load_chunk(chunk.clone());
        }
        vm.execute_tiered(compilation_unit.main_chunk_idx)
            .map_err(|e| Galat::runtime_polos(format!("VM JIT execution failed: {:?}", e)))?;
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        let elapsed_sec = elapsed / 1000.0;
        let mips = if elapsed_sec > 0.0 && vm.instruction_count > 0 {
            (vm.instruction_count as f64 / elapsed_sec) / 1_000_000.0
        } else {
            0.0
        };
        let ic_hits: u64 = vm.ic_table.values().map(|ic| ic.entries.iter().map(|e| e.hits).sum::<u64>()).sum();

        Ok(JitExecutionReport {
            duration_ms: elapsed,
            mode: format!("Register-VM Multi-Tier JIT ({:?}, NaN-Boxing, Tail-Call Opt)", vm.current_tier),
            instructions_executed: vm.instruction_count,
            mips,
            jit_tier: vm.current_tier,
            osr_promotions: vm.osr_promotions,
            inline_cache_hits: ic_hits,
            deopt_bailouts: vm.deopt_log.len(),
        })
    }

    fn optimize_program(&self, program: &mut Program) {
        for stmt in &mut program.statements {
            self.optimize_stmt(stmt);
        }
    }

    fn optimize_stmt(&self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Expression(expr) => self.optimize_expr(expr),
            Stmt::VarDecl { initializer: Some(expr), .. } => self.optimize_expr(expr),
            Stmt::FunctionDecl { body, .. } => {
                for s in body {
                    self.optimize_stmt(s);
                }
            }
            Stmt::Block(stmts, _) => {
                for s in stmts {
                    self.optimize_stmt(s);
                }
            }
            _ => {}
        }
    }

    fn optimize_expr(&self, expr: &mut Expr) {
        if let Expr::Binary { left, op, right, span } = expr {
            self.optimize_expr(left);
            self.optimize_expr(right);

            // 1. Constant folding untuk nilai angka konstan
            if let (Expr::Number(a, _), Expr::Number(b, _)) = (&**left, &**right) {
                let folded_val = match op {
                    BinaryOp::Add => Some(a + b),
                    BinaryOp::Subtract => Some(a - b),
                    BinaryOp::Multiply => Some(a * b),
                    BinaryOp::Divide if *b != 0.0 => Some(a / b),
                    _ => None,
                };

                if let Some(val) = folded_val {
                    *expr = Expr::Number(val, span.clone());
                    return;
                }
            }

            // 2. Optimasi identitas aljabar: x + 0 => x, x * 1 => x, x * 0 => 0
            match op {
                BinaryOp::Add => {
                    if let Expr::Number(0.0, _) = &**right {
                        *expr = (**left).clone();
                        return;
                    }
                    if let Expr::Number(0.0, _) = &**left {
                        *expr = (**right).clone();
                        return;
                    }
                }
                BinaryOp::Multiply => {
                    if let Expr::Number(1.0, _) = &**right {
                        *expr = (**left).clone();
                        return;
                    }
                    if let Expr::Number(1.0, _) = &**left {
                        *expr = (**right).clone();
                        return;
                    }
                    if let Expr::Number(0.0, _) = &**right {
                        *expr = Expr::Number(0.0, span.clone());
                        return;
                    }
                    if let Expr::Number(0.0, _) = &**left {
                        *expr = Expr::Number(0.0, span.clone());
                        return;
                    }
                }
                BinaryOp::Subtract => {
                    if let Expr::Number(0.0, _) = &**right {
                        *expr = (**left).clone();
                        return;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn jalankan_jit_berkas(path: &std::path::PathBuf) {
    println!("{}", "============================================================".bright_blue());
    println!("{}", "  ⚡ WIDYA JIT OPTIMIZING COMPILER & REGISTER-VM RUNTIME".bright_cyan().bold());
    println!("{}", "============================================================".bright_blue());
    println!("📂 Berkas Target : {}", path.display().to_string().bright_yellow());

    let sumber = match std::fs::read_to_string(path) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("❌ Gagal membaca berkas: {}", e);
            return;
        }
    };

    let jit = JitEngine::new();
    match jit.execute_jit_advanced(&sumber) {
        Ok(rep) => {
            println!("{}", "------------------------------------------------------------".bright_blue());
            println!("⚙️  Mode Eksekusi : {}", rep.mode.bright_green().bold());
            println!("🚀 Waktu Eksekusi : {:.3} ms", rep.duration_ms);
            println!("⚡ JIT Tier       : {:?}", rep.jit_tier);
            if rep.osr_promotions > 0 {
                println!("🔥 OSR Promosi    : {} loop", rep.osr_promotions);
            }
            if rep.inline_cache_hits > 0 {
                println!("🎯 IC Cache Hits  : {}", rep.inline_cache_hits);
            }
            if rep.instructions_executed > 0 {
                println!("📊 Total Instruksi: {} op", rep.instructions_executed.to_string().bright_yellow());
                println!("⚡ Throughput      : {:.2} MIPS", rep.mips);
            }
            println!("{}", "------------------------------------------------------------".bright_blue());
            println!("{}", "✅ Eksekusi JIT Berhasil Sempurna!".bright_green().bold());
        }
        Err(e) => {
            eprintln!("❌ Galat JIT Runtime: {}", e);
        }
    }
}
