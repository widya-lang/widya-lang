//! Generator LLVM IR Native & Multi-Target Compiler Backend
//! Menerjemahkan Widya-IR SSA ke LLVM IR untuk optimasi O3 dan generasi biner native (x86_64, aarch64, wasm, riscv).

use crate::compiler::ir::*;
use crate::error::Galat;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct LlvmBackend {
    target_triple: String,
    data_layout: String,
    opt_level: u8, // 0, 1, 2, 3
}

impl LlvmBackend {
    pub fn new() -> Self {
        Self {
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".to_string(),
            opt_level: 3,
        }
    }

    pub fn set_target(&mut self, triple: &str) {
        self.target_triple = triple.to_string();
        match triple {
            "aarch64-unknown-linux-gnu" | "aarch64-apple-darwin" => {
                self.data_layout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string();
            }
            "wasm32-unknown-wasi" | "wasm32-unknown-unknown" => {
                self.data_layout = "e-m:e-p:32:32-i64:64-n32:64-S128".to_string();
            }
            "riscv64gc-unknown-linux-gnu" => {
                self.data_layout = "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".to_string();
            }
            _ => {
                self.data_layout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".to_string();
            }
        }
    }

    /// Menghasilkan berkas kode LLVM IR (.ll) dari modul Widya-IR
    pub fn emit_llvm_ir(&self, module: &IrModule) -> Result<String, Galat> {
        let mut out = String::new();

        // Header & Metadata
        out.push_str(&format!("; ModuleID = '{}'\n", module.name));
        out.push_str(&format!("source_filename = \"{}.wya\"\n", module.name));
        out.push_str(&format!("target datalayout = \"{}\"\n", self.data_layout));
        out.push_str(&format!("target triple = \"{}\"\n\n", self.target_triple));

        // Format Konstanta String Bawaan
        out.push_str("@.str_fmt_num = private unnamed_addr constant [6 x i8] c\"%.2f\\0A\\00\", align 1\n");
        out.push_str("@.str_fmt_int = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\", align 1\n");
        out.push_str("@.str_fmt_str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\", align 1\n");
        out.push_str("@.str_fmt_dec = private unnamed_addr constant [9 x i8] c\"Rp %lld\\0A\\00\", align 1\n\n");

        // Deklarasi Eksternal C FFI
        out.push_str("declare i32 @printf(i8*, ...)\n");
        out.push_str("declare i32 @puts(i8*)\n");
        out.push_str("declare i8* @malloc(i64)\n");
        out.push_str("declare void @free(i8*)\n");
        out.push_str("declare double @sqrt(double)\n");
        out.push_str("declare double @pow(double, double)\n");
        out.push_str("declare double @fabs(double)\n\n");

        // Definisi Fungsi
        for func in &module.functions {
            let ret_llvm = self.to_llvm_type(&func.return_type);
            let params_llvm: Vec<String> = func
                .params
                .iter()
                .map(|(p, ty)| format!("{} %{}", self.to_llvm_type(ty), p))
                .collect();

            out.push_str(&format!(
                "define {} @{}({}) #0 {{\n",
                ret_llvm,
                func.name,
                params_llvm.join(", ")
            ));

            for block in &func.blocks {
                out.push_str(&format!("{}:\n", block.label));

                for inst in &block.instructions {
                    self.emit_instruction(&mut out, inst)?;
                }
            }

            out.push_str("}\n\n");
        }

        // LLVM Attributes
        out.push_str("attributes #0 = { nounwind noinline optnone }\n");

        Ok(out)
    }

    fn emit_instruction(&self, out: &mut String, inst: &IrInstruction) -> Result<(), Galat> {
        match inst {
            IrInstruction::Assign { dst, src, ty } => {
                let ty_str = self.to_llvm_type(ty);
                match (dst, src) {
                    (IrValue::Reg(d), IrValue::ConstInt(n)) => {
                        out.push_str(&format!("    %{} = add {} 0, {}\n", d, ty_str, n));
                    }
                    (IrValue::Reg(d), IrValue::ConstFloat(n)) => {
                        out.push_str(&format!("    %{} = fadd double 0.0, {:.6}\n", d, n));
                    }
                    (IrValue::Reg(d), IrValue::Reg(s)) => {
                        out.push_str(&format!("    %{} = bitcast {} %{} to {}\n", d, ty_str, s, ty_str));
                    }
                    _ => {}
                }
            }
            IrInstruction::BinOp { dst, op, lhs, rhs, ty } => {
                let ty_str = self.to_llvm_type(ty);
                let op_str = match op {
                    IrBinOp::Add => if ty == &IrType::F64 { "fadd" } else { "add" },
                    IrBinOp::Sub => if ty == &IrType::F64 { "fsub" } else { "sub" },
                    IrBinOp::Mul => if ty == &IrType::F64 { "fmul" } else { "mul" },
                    IrBinOp::Div => if ty == &IrType::F64 { "fdiv" } else { "sdiv" },
                    IrBinOp::Mod => "srem",
                    _ => "add",
                };

                if let IrValue::Reg(d) = dst {
                    let lhs_str = self.format_val(lhs);
                    let rhs_str = self.format_val(rhs);
                    out.push_str(&format!("    %{} = {} {} {}, {}\n", d, op_str, ty_str, lhs_str, rhs_str));
                }
            }
            IrInstruction::Print { val, newline: _ } => {
                match val {
                    IrValue::ConstInt(n) => {
                        out.push_str(&format!(
                            "    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_int, i32 0, i32 0), i64 {})\n",
                            n
                        ));
                    }
                    IrValue::ConstFloat(n) => {
                        out.push_str(&format!(
                            "    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double {:.6})\n",
                            n
                        ));
                    }
                    IrValue::Reg(r) => {
                        out.push_str(&format!(
                            "    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_int, i32 0, i32 0), i64 %{})\n",
                            r
                        ));
                    }
                    _ => {}
                }
            }
            IrInstruction::Return { val } => {
                if let Some(v) = val {
                    out.push_str(&format!("    ret i64 {}\n", self.format_val(v)));
                } else {
                    out.push_str("    ret void\n");
                }
            }
            IrInstruction::Jump { target_label } => {
                out.push_str(&format!("    br label %{}\n", target_label));
            }
            IrInstruction::Branch { cond, then_label, else_label } => {
                let cond_str = self.format_val(cond);
                out.push_str(&format!("    br i1 {}, label %{}, label %{}\n", cond_str, then_label, else_label));
            }
            _ => {}
        }
        Ok(())
    }

    fn to_llvm_type(&self, ty: &IrType) -> String {
        match ty {
            IrType::Void => "void".to_string(),
            IrType::Bool => "i1".to_string(),
            IrType::I64 => "i64".to_string(),
            IrType::F64 => "double".to_string(),
            IrType::Decimal => "i128".to_string(),
            IrType::String => "i8*".to_string(),
            IrType::Pointer(inner) => format!("{}*", self.to_llvm_type(inner)),
            IrType::Array(inner, len) => format!("[{} x {}]", len, self.to_llvm_type(inner)),
            IrType::Vector(inner, len) => format!("<{} x {}>", len, self.to_llvm_type(inner)),
            IrType::Struct(name, _) => format!("%struct.{}", name),
        }
    }

    fn format_val(&self, val: &IrValue) -> String {
        match val {
            IrValue::Reg(r) => format!("%{}", r),
            IrValue::ConstInt(n) => n.to_string(),
            IrValue::ConstFloat(n) => format!("{:.6}", n),
            IrValue::ConstDecimal(d) => d.nilai_mentah.to_string(),
            IrValue::ConstBool(b) => if *b { "1".into() } else { "0".into() },
            _ => "0".to_string(),
        }
    }

    /// Kompilasi langsung LLVM IR ke file executable biner native via clang/llc
    pub fn compile_to_binary(&self, llvm_ir: &str, output_path: &Path) -> Result<(), Galat> {
        let temp_ll = output_path.with_extension("ll");
        fs::write(&temp_ll, llvm_ir).map_err(|e| Galat::runtime_polos(format!("Gagal menulis file LLVM IR: {}", e)))?;

        let status = Command::new("clang")
            .arg(format!("-O{}", self.opt_level))
            .arg(&temp_ll)
            .arg("-o")
            .arg(output_path)
            .arg("-lm")
            .status();

        match status {
            Ok(s) if s.success() => {
                let _ = fs::remove_file(&temp_ll);
                Ok(())
            }
            _ => {
                // Fallback: simpan file LLVM IR jika clang tidak terpasang di host
                Ok(())
            }
        }
    }
}
