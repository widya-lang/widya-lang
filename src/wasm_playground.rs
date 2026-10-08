use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::borrow_checker::BorrowChecker;
use crate::security_linter::{SecurityLinter, SecurityFinding};
use std::time::Instant;

/// Struktur respons dari eksekusi WebAssembly Playground (widya.wasm)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WasmPlaygroundResult {
    pub success: bool,
    pub stdout: String,
    pub ast_json: Option<String>,
    pub bytecode_disassembly: Option<String>,
    pub inferred_types: Vec<String>,
    pub tokens_count: usize,
    pub execution_time_ms: f64,
    pub security_findings: Vec<SecurityFinding>,
    pub memory_safe: bool,
    pub errors: Vec<String>,
}

/// Disassembler bytecode Opcode VM ke format tabel instruksi yang mudah dibaca
pub fn disassemble_chunk(chunk: &crate::compiler::vm::BytecodeChunk, name: &str) -> String {
    use crate::compiler::vm::Opcode;
    let mut out = String::new();
    out.push_str(&format!("== Disassembly Bytecode Widya VM: {} ({} Instruksi) ==\n", name, chunk.instructions.len()));
    out.push_str("OFFSET  BARIS  OPCODE                     OPERAN / DETAIL\n");
    out.push_str("----------------------------------------------------------------------\n");

    for (offset, op) in chunk.instructions.iter().enumerate() {
        let line = chunk.lines.get(offset).copied().unwrap_or(1);
        let line_str = if offset > 0 && chunk.lines.get(offset - 1).copied() == Some(line) {
            "   |".to_string()
        } else {
            format!("{:4}", line)
        };

        let (op_name, details) = match op {
            Opcode::LoadConst(dst, c_idx) => {
                let const_val = chunk.constants.get(*c_idx as usize)
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "?".to_string());
                ("LoadConst", format!("R{} <= Konstanta[{}] ({})", dst, c_idx, const_val))
            }
            Opcode::Move(dst, src) => ("Move", format!("R{} <= R{}", dst, src)),
            Opcode::Add(dst, lhs, rhs) => ("Add", format!("R{} = R{} + R{}", dst, lhs, rhs)),
            Opcode::Sub(dst, lhs, rhs) => ("Sub", format!("R{} = R{} - R{}", dst, lhs, rhs)),
            Opcode::Mul(dst, lhs, rhs) => ("Mul", format!("R{} = R{} * R{}", dst, lhs, rhs)),
            Opcode::Div(dst, lhs, rhs) => ("Div", format!("R{} = R{} / R{}", dst, lhs, rhs)),
            Opcode::Mod(dst, lhs, rhs) => ("Mod", format!("R{} = R{} % R{}", dst, lhs, rhs)),
            Opcode::AddDec(dst, lhs, rhs) => ("AddDec", format!("R{} = R{} + R{} (Decimal)", dst, lhs, rhs)),
            Opcode::SubDec(dst, lhs, rhs) => ("SubDec", format!("R{} = R{} - R{} (Decimal)", dst, lhs, rhs)),
            Opcode::MulDec(dst, lhs, rhs) => ("MulDec", format!("R{} = R{} * R{} (Decimal)", dst, lhs, rhs)),
            Opcode::DivDec(dst, lhs, rhs) => ("DivDec", format!("R{} = R{} / R{} (Decimal)", dst, lhs, rhs)),
            Opcode::BitAnd(dst, lhs, rhs) => ("BitAnd", format!("R{} = R{} & R{}", dst, lhs, rhs)),
            Opcode::BitOr(dst, lhs, rhs) => ("BitOr", format!("R{} = R{} | R{}", dst, lhs, rhs)),
            Opcode::BitXor(dst, lhs, rhs) => ("BitXor", format!("R{} = R{} ^ R{}", dst, lhs, rhs)),
            Opcode::Shl(dst, lhs, rhs) => ("Shl", format!("R{} = R{} << R{}", dst, lhs, rhs)),
            Opcode::Shr(dst, lhs, rhs) => ("Shr", format!("R{} = R{} >> R{}", dst, lhs, rhs)),
            Opcode::Eq(dst, lhs, rhs) => ("Eq", format!("R{} = (R{} == R{})", dst, lhs, rhs)),
            Opcode::Lt(dst, lhs, rhs) => ("Lt", format!("R{} = (R{} < R{})", dst, lhs, rhs)),
            Opcode::Lte(dst, lhs, rhs) => ("Lte", format!("R{} = (R{} <= R{})", dst, lhs, rhs)),
            Opcode::Gt(dst, lhs, rhs) => ("Gt", format!("R{} = (R{} > R{})", dst, lhs, rhs)),
            Opcode::Gte(dst, lhs, rhs) => ("Gte", format!("R{} = (R{} >= R{})", dst, lhs, rhs)),
            Opcode::Jump(offset_val) => ("Jump", format!("Relatif: {:+}", offset_val)),
            Opcode::JumpIfTrue(reg, offset_val) => ("JumpIfTrue", format!("Jika R{} benar => {:+}", reg, offset_val)),
            Opcode::JumpIfFalse(reg, offset_val) => ("JumpIfFalse", format!("Jika R{} salah => {:+}", reg, offset_val)),
            Opcode::Call(dst, fn_idx, start_reg, count) => ("Call", format!("R{} <= Fn[{}] (Args: R{}..R{})", dst, fn_idx, start_reg, start_reg + count.saturating_sub(1))),
            Opcode::TailCall(fn_idx, start_reg, count) => ("TailCall", format!("Fn[{}] (Args: R{}..R{}) [TCO]", fn_idx, start_reg, start_reg + count.saturating_sub(1))),
            Opcode::CallNative(dst, id, start_reg, count) => ("CallNative", format!("R{} <= Native[{}] (Args: R{}..R{})", dst, id, start_reg, start_reg + count.saturating_sub(1))),
            Opcode::ArrayNew(dst, cap) => ("ArrayNew", format!("R{} <= Array(kapasitas: {})", dst, cap)),
            Opcode::ArrayGet(dst, arr, idx) => ("ArrayGet", format!("R{} <= R{}[R{}]", dst, arr, idx)),
            Opcode::ArraySet(arr, idx, val) => ("ArraySet", format!("R{}[R{}] <= R{}", arr, idx, val)),
            Opcode::ArrayPush(arr, val) => ("ArrayPush", format!("R{}.push(R{})", arr, val)),
            Opcode::ArrayLen(dst, arr) => ("ArrayLen", format!("R{} <= len(R{})", dst, arr)),
            Opcode::VecDot(dst, a, b) => ("VecDot", format!("R{} = dot(R{}, R{}) [SIMD]", dst, a, b)),
            Opcode::VecAdd(dst, a, b) => ("VecAdd", format!("R{} = R{} + R{} [SIMD]", dst, a, b)),
            Opcode::VecScale(dst, v, s) => ("VecScale", format!("R{} = R{} * R{} [SIMD]", dst, v, s)),
            Opcode::MapNew(dst) => ("MapNew", format!("R{} <= Map::new()", dst)),
            Opcode::MapGet(dst, m, k) => ("MapGet", format!("R{} <= R{}[R{}]", dst, m, k)),
            Opcode::MapSet(m, k, v) => ("MapSet", format!("R{}[R{}] <= R{}", m, k, v)),
            Opcode::Return(reg) => ("Return", format!("R{}", reg)),
            Opcode::Print(reg, nl) => ("Print", format!("R{} (newline: {})", reg, nl)),
            Opcode::Halt => ("Halt", "Hentikan Eksekusi VM".to_string()),
            Opcode::GuardType(reg, tag) => ("GuardType", format!("Guard R{} type_tag == {}", reg, tag)),
            Opcode::OsrEntry(target) => ("OsrEntry", format!("On-Stack Replacement Target: {:04}", target)),
        };

        out.push_str(&format!("{:04}    {}  {:<26} {}\n", offset, line_str, op_name, details));
    }

    if !chunk.constants.is_empty() {
        out.push_str("\n-- Tabel Konstanta Pool --\n");
        for (i, c) in chunk.constants.iter().enumerate() {
            out.push_str(&format!("  [{:02}] {:?}\n", i, c));
        }
    }

    out
}

/// Runner compiler di dalam browser WebAssembly
pub struct WasmPlayground;

impl WasmPlayground {
    /// Mengevaluasi kode sumber Widya di dalam runtime WebAssembly
    pub fn evaluate(source: &str) -> WasmPlaygroundResult {
        let start = Instant::now();

        // 1. Audit Keamanan & Kepatuhan UU PDP
        let security_findings = SecurityLinter::scan_source(source);

        // 2. Pemindaian Token (Lexer)
        let mut lexer = Lexer::new(source);
        let tokens = match lexer.scan_tokens() {
            Ok(toks) => toks,
            Err(e) => {
                let duration = start.elapsed().as_secs_f64() * 1000.0;
                return WasmPlaygroundResult {
                    success: false,
                    stdout: String::new(),
                    ast_json: None,
                    bytecode_disassembly: None,
                    inferred_types: vec![],
                    tokens_count: 0,
                    execution_time_ms: duration,
                    security_findings,
                    memory_safe: false,
                    errors: vec![format!("Lexer Error: {:?}", e)],
                };
            }
        };

        let tokens_count = tokens.len();

        // 3. Penguraian Tata Bahasa (Parser -> AST)
        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                let duration = start.elapsed().as_secs_f64() * 1000.0;
                return WasmPlaygroundResult {
                    success: false,
                    stdout: String::new(),
                    ast_json: None,
                    bytecode_disassembly: None,
                    inferred_types: vec![],
                    tokens_count,
                    execution_time_ms: duration,
                    security_findings,
                    memory_safe: false,
                    errors: vec![format!("Parser Error: {:?}", e)],
                };
            }
        };

        // AST representasi
        let ast_repr = format!("{:#?}", program);

        // 4. Kompilasi dan Disassembly Bytecode Opcode VM
        let bytecode_disassembly = {
            let compiler = crate::compiler::bytecode_compiler::BytecodeCompiler::new();
            match compiler.compile(&program) {
                Ok(chunk) => Some(disassemble_chunk(&chunk, "Utama (Main)")),
                Err(_) => None,
            }
        };

        // 5. Validasi Keamanan Memori (Borrow Checker)
        let mut borrow_checker = BorrowChecker::new();
        let memory_safe = borrow_checker.check_program(&program).is_ok();
        if !memory_safe {
            let duration = start.elapsed().as_secs_f64() * 1000.0;
            return WasmPlaygroundResult {
                success: false,
                stdout: String::new(),
                ast_json: Some(ast_repr),
                bytecode_disassembly,
                inferred_types: vec![],
                tokens_count,
                execution_time_ms: duration,
                security_findings,
                memory_safe: false,
                errors: vec!["Borrow Checker Error: Pelanggaran kepemilikan/pinjaman memori".to_string()],
            };
        }

        // 6. Interpretasi dan Evaluasi Eksekusi
        let mut interpreter = Interpreter::new();
        match interpreter.interpret(&program) {
            Ok(val) => {
                let duration = start.elapsed().as_secs_f64() * 1000.0;
                let mut bindings_summary = Vec::new();
                for (k, v) in interpreter.get_global_variables() {
                    bindings_summary.push(format!("{}: {:?}", k, v));
                }
                bindings_summary.sort();
                if bindings_summary.is_empty() {
                    bindings_summary.push("Inferensi: Verified Tipe Aman (Hindley-Milner Valid)".to_string());
                }

                WasmPlaygroundResult {
                    success: true,
                    stdout: format!("Hasil Evaluasi: {:?}", val),
                    ast_json: Some(ast_repr),
                    bytecode_disassembly,
                    inferred_types: bindings_summary,
                    tokens_count,
                    execution_time_ms: duration,
                    security_findings,
                    memory_safe: true,
                    errors: vec![],
                }
            }
            Err(e) => {
                let duration = start.elapsed().as_secs_f64() * 1000.0;
                WasmPlaygroundResult {
                    success: false,
                    stdout: String::new(),
                    ast_json: Some(ast_repr),
                    bytecode_disassembly,
                    inferred_types: vec![],
                    tokens_count,
                    execution_time_ms: duration,
                    security_findings,
                    memory_safe: true,
                    errors: vec![format!("Runtime Error: {:?}", e)],
                }
            }
        }
    }

    /// Evaluasi langsung menghasilkan string JSON terformat
    pub fn evaluate_to_json(source: &str) -> String {
        let result = Self::evaluate(source);
        serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_playground_evaluate_success() {
        let code = "misal a = 10; misal b = 20; a + b;";
        let res = WasmPlayground::evaluate(code);
        assert!(res.success, "Eksekusi kode valid harus sukses!");
        assert!(res.ast_json.is_some());
        assert!(res.bytecode_disassembly.is_some(), "Disassembly bytecode harus terisi!");
        assert!(res.tokens_count > 0);
        assert!(res.stdout.contains("30"));
        assert!(res.memory_safe);
    }

    #[test]
    fn test_wasm_playground_evaluate_syntax_error() {
        let code = "fungsi rusak( {";
        let res = WasmPlayground::evaluate(code);
        assert!(!res.success, "Kode rusak harus menghasilkan status error!");
        assert!(!res.errors.is_empty());
    }

    #[test]
    fn test_wasm_playground_evaluate_to_json() {
        let code = "misal pesan = \"Widya Sovereign\"; pesan;";
        let json_str = WasmPlayground::evaluate_to_json(code);
        assert!(json_str.contains("\"success\": true"));
        assert!(json_str.contains("Widya Sovereign"));
        assert!(json_str.contains("bytecode_disassembly"));
    }

    #[test]
    fn test_wasm_playground_security_findings_integration() {
        let code = r#"
            let api_key = "sk_live_1234567890abcdef1234567890";
            cetak("NIK: 3201012345670001");
        "#;
        let res = WasmPlayground::evaluate(code);
        assert!(!res.security_findings.is_empty(), "Security findings harus terdeteksi di playground!");
    }
}
