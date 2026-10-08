use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use widya::ast::*;
use widya::error::Span;
use widya::compiler::{
    BytecodeCompiler, CompilationUnit, FiberPriority, VirtualMachine, VmValue, WorkStealingScheduler,
};
use widya::dap::{DapEngine, DebuggerState};
use widya::formal_verifier::{FormalVerifier, NumericInterval};
use widya::lsp_enhanced::{SymbolKind, WidyaLsp};
use widya::widyabase::WidyaBase;

fn dummy_span() -> Span {
    Span::new(1, 1)
}

#[test]
fn test_compilation_unit_functions_and_tco() {
    let span = dummy_span();

    // Definisikan fungsi:
    // fungsi tambah(a, b) { kembalikan a + b; }
    let fn_tambah = Stmt::FunctionDecl {
        name: "tambah".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Identifier("a".to_string(), span)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Identifier("b".to_string(), span)),
                    span,
                }),
                span,
            },
        ],
        attributes: Vec::new(),
        span,
    };

    // Program utama:
    // var res = tambah(15, 27);
    // kembalikan res;
    let main_call = Stmt::VarDecl {
        name: "res".to_string(),
        initializer: Some(Expr::Call {
            callee: Box::new(Expr::Identifier("tambah".to_string(), span)),
            arguments: vec![Expr::Number(15.0, span), Expr::Number(27.0, span)],
            span,
        }),
        is_const: false,
        span,
    };

    let main_ret = Stmt::Return {
        value: Some(Expr::Identifier("res".to_string(), span)),
        span,
    };

    let prog = Program {
        statements: vec![fn_tambah, main_call, main_ret],
    };

    let compiler = BytecodeCompiler::new();
    let unit: CompilationUnit = compiler.compile_program(&prog).expect("compile_program succeeded");

    assert_eq!(unit.chunks.len(), 2, "Harus ada 2 chunks: main dan tambah");
    assert!(unit.function_map.contains_key("tambah"));

    let res = unit.run().expect("Bytecode execution succeeded");
    assert_eq!(res, VmValue::Int(42));
}

#[test]
fn test_compilation_unit_tail_call_optimization() {
    let span = dummy_span();

    // fungsi hitung_mundur(n, acc) {
    //     jika (n <= 0) { kembalikan acc; }
    //     kembalikan hitung_mundur(n - 1, acc + n);
    // }
    let fn_rec = Stmt::FunctionDecl {
        name: "hitung_mundur".to_string(),
        params: vec!["n".to_string(), "acc".to_string()],
        body: vec![
            Stmt::If {
                condition: Expr::Binary {
                    left: Box::new(Expr::Identifier("n".to_string(), span)),
                    op: BinaryOp::LessEqual,
                    right: Box::new(Expr::Number(0.0, span)),
                    span,
                },
                then_branch: Box::new(Stmt::Return {
                    value: Some(Expr::Identifier("acc".to_string(), span)),
                    span,
                }),
                elif_branches: Vec::new(),
                else_branch: None,
                span,
            },
            Stmt::Return {
                value: Some(Expr::Call {
                    callee: Box::new(Expr::Identifier("hitung_mundur".to_string(), span)),
                    arguments: vec![
                        Expr::Binary {
                            left: Box::new(Expr::Identifier("n".to_string(), span)),
                            op: BinaryOp::Subtract,
                            right: Box::new(Expr::Number(1.0, span)),
                            span,
                        },
                        Expr::Binary {
                            left: Box::new(Expr::Identifier("acc".to_string(), span)),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Identifier("n".to_string(), span)),
                            span,
                        },
                    ],
                    span,
                }),
                span,
            },
        ],
        attributes: Vec::new(),
        span,
    };

    let main_call = Stmt::Return {
        value: Some(Expr::Call {
            callee: Box::new(Expr::Identifier("hitung_mundur".to_string(), span)),
            arguments: vec![Expr::Number(100.0, span), Expr::Number(0.0, span)],
            span,
        }),
        span,
    };

    let prog = Program {
        statements: vec![fn_rec, main_call],
    };

    let compiler = BytecodeCompiler::new();
    let unit = compiler.compile_program(&prog).expect("compile_program with TCO succeeded");

    let res = unit.run().expect("TCO recursion executed without stack overflow");
    assert_eq!(res, VmValue::Int(5050));
}

#[test]
fn test_fiber_scheduler_spawn_bytecode() {
    let span = dummy_span();

    let prog = Program {
        statements: vec![
            Stmt::VarDecl {
                name: "x".to_string(),
                initializer: Some(Expr::Number(100.0, span)),
                is_const: false,
                span,
            },
            Stmt::VarDecl {
                name: "y".to_string(),
                initializer: Some(Expr::Number(250.0, span)),
                is_const: false,
                span,
            },
            Stmt::Return {
                value: Some(Expr::Binary {
                    left: Box::new(Expr::Identifier("x".to_string(), span)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Identifier("y".to_string(), span)),
                    span,
                }),
                span,
            },
        ],
    };

    let compiler = BytecodeCompiler::new();
    let unit = compiler.compile_program(&prog).expect("compile unit");

    let scheduler = WorkStealingScheduler::new(2);
    scheduler.start();

    let (_fiber_id, result_box) = scheduler.spawn_bytecode("fiber_calc", unit, FiberPriority::High);

    // Tunggu hasil eksekusi fiber
    let mut final_result = None;
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(20));
        let lock = result_box.lock().unwrap();
        if let Some(ref r) = *lock {
            final_result = Some(r.clone());
            break;
        }
    }

    scheduler.shutdown();
    let computed = final_result.expect("Fiber calculated result").expect("VmValue result");
    assert_eq!(computed, VmValue::Int(350));
}

#[test]
fn test_formal_verifier_ast_loop_and_division() {
    let span = dummy_span();

    // 1. Loop termination proof untuk ForIn range
    let loop_stmt = Stmt::ForIn {
        var_name: "i".to_string(),
        iterable: Expr::Range {
            start: Box::new(Expr::Number(0.0, span)),
            end: Box::new(Expr::Number(50.0, span)),
            span,
        },
        body: Box::new(Stmt::Block(Vec::new(), span)),
        span,
    };

    let inv = FormalVerifier::verify_ast_loop(&loop_stmt).expect("Loop termination proof passed");
    assert_eq!(inv.loop_name, "for_i");
    assert_eq!(inv.initial_counter, 0);
    assert_eq!(inv.target_bound, 50);

    // 2. Infinite loop detection
    let infinite_while = Stmt::While {
        condition: Expr::Boolean(true, span),
        body: Box::new(Stmt::Block(Vec::new(), span)),
        span,
    };
    assert!(FormalVerifier::verify_ast_loop(&infinite_while).is_err());

    // 3. Division safety proof
    let mut env = HashMap::new();
    env.insert("divisor_aman".to_string(), NumericInterval::new(5, 20));
    env.insert("divisor_bahaya".to_string(), NumericInterval::new(-3, 10));

    let aman_expr = Expr::Binary {
        left: Box::new(Expr::Number(100.0, span)),
        op: BinaryOp::Divide,
        right: Box::new(Expr::Identifier("divisor_aman".to_string(), span)),
        span,
    };
    assert!(FormalVerifier::verify_ast_division_in_expr(&aman_expr, &env).is_ok());

    let bahaya_expr = Expr::Binary {
        left: Box::new(Expr::Number(100.0, span)),
        op: BinaryOp::Divide,
        right: Box::new(Expr::Identifier("divisor_bahaya".to_string(), span)),
        span,
    };
    assert!(FormalVerifier::verify_ast_division_in_expr(&bahaya_expr, &env).is_err());
}

#[test]
fn test_dap_conditional_breakpoint_and_watchpoints() {
    let mut dap = DapEngine::new();
    dap.set_variable("counter", "10", "int");
    dap.set_variable("status", "selesai", "string");

    // Set conditional breakpoint pada baris 42: "counter == 10"
    dap.set_conditional_breakpoint("app.wya", 42, "counter == 10");
    assert!(dap.check_breakpoint("app.wya", 42), "Breakpoint harus dipicu saat counter == 10");
    assert_eq!(dap.state, DebuggerState::Paused);

    // Coba saat kondisi tidak terpenuhi: "counter == 99"
    dap.set_conditional_breakpoint("app.wya", 50, "counter == 99");
    assert!(!dap.check_breakpoint("app.wya", 50), "Breakpoint harus diabaikan saat counter != 99");

    // Watchpoint
    dap.add_watchpoint("status", Some("status == selesai".to_string()));
    let wp_triggered = dap.check_watchpoints();
    assert!(wp_triggered.is_some());
}

#[test]
fn test_lsp_document_symbols_and_formatting() {
    let lsp = WidyaLsp::new();

    let code = r#"
struktur Pengguna {
    id: Int,
    nama: Teks
}

tetap MAKS_KONEKSI = 100

fungsi proses() {
misal data = 50;
jika (data > 10) {
cetak(data);
}
}
"#;

    let symbols = lsp.get_document_symbols(code);
    let names: Vec<String> = symbols.iter().map(|s| s.name.clone()).collect();
    assert!(names.contains(&"Pengguna".to_string()));
    assert!(names.contains(&"MAKS_KONEKSI".to_string()));
    assert!(names.contains(&"proses".to_string()));
    assert!(names.contains(&"data".to_string()));

    let unformatted = "fungsi tes() {\nmisal x = 1;\njika (x == 1) {\ncetak(x);\n}\n}";
    let edits = lsp.format_document("file:///app.wya", unformatted);
    assert!(!edits.is_empty(), "Harus ada text edit untuk auto formatting");
    assert!(edits[0].new_text.contains("    misal x = 1;"));
    assert!(edits[0].new_text.contains("        cetak(x);"));
}

#[test]
fn test_widyabase_vector_and_hybrid_facade() {
    let tmp_dir = std::env::temp_dir().join(format!("wdb_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    let db = WidyaBase::open(&tmp_dir).expect("Open WidyaBase");

    let mut vec_idx = db.vector_index(4);
    vec_idx.insert("doc1", &[1.0, 0.0, 0.0, 0.0]);
    vec_idx.insert("doc2", &[0.0, 1.0, 0.0, 0.0]);

    let res = vec_idx.search_knn(&[0.9, 0.1, 0.0, 0.0], 1, 50);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].0, "doc1");

    let hybrid = db.hybrid_search();
    assert_eq!(hybrid.documents.len(), 0);
}
