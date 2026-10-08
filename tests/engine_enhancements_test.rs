use widya::compiler::{
    BytecodeChunk, BytecodeCompiler, FiberChannel, FiberPriority, HotModuleReloader, IncrementalCache, NanBoxedValue,
    Opcode, VirtualMachine, VmValue, WorkStealingScheduler,
};
use widya::ast::*;
use widya::borrow_checker::BorrowChecker;
use widya::formal_verifier::{FormalVerifier, FunctionContract, LoopInvariant, NumericInterval, SmtPredicate};
use widya::error::Span;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_vm_nan_boxing_and_new_opcodes() {
    // 1. NaN-Boxing Value Representation Test
    let f_val = NanBoxedValue::from_f64(3.14159);
    assert!(f_val.is_f64());
    assert!((f_val.as_f64().unwrap() - 3.14159).abs() < 1e-5);

    let i_val = NanBoxedValue::from_i32(42);
    assert_eq!(i_val.as_i32(), Some(42));

    let b_val = NanBoxedValue::from_bool(true);
    assert_eq!(b_val.as_bool(), Some(true));

    let nil_val = NanBoxedValue::nil();
    assert!(nil_val.is_nil());

    // 2. Register Bytecode VM Bitwise, Array, Map & Native Function Execution
    let mut vm = VirtualMachine::new();
    let mut chunk = BytecodeChunk::new();

    // Register constants
    let c_arr_len = chunk.add_constant(VmValue::Int(0)); // placeholder
    let c_val1 = chunk.add_constant(VmValue::Int(12));
    let c_val2 = chunk.add_constant(VmValue::Int(5));
    let c_key = chunk.add_constant(VmValue::String("versi".to_string()));
    let c_str = chunk.add_constant(VmValue::String("v2.0-enhanced".to_string()));

    // Instructions:
    // r0 = 12, r1 = 5
    chunk.emit(Opcode::LoadConst(0, c_val1), 1);
    chunk.emit(Opcode::LoadConst(1, c_val2), 2);
    // r2 = r0 & r1 (12 & 5 = 4)
    chunk.emit(Opcode::BitAnd(2, 0, 1), 3);
    // r3 = r0 | r1 (12 | 5 = 13)
    chunk.emit(Opcode::BitOr(3, 0, 1), 4);
    // r4 = r0 ^ r1 (12 ^ 5 = 9)
    chunk.emit(Opcode::BitXor(4, 0, 1), 5);
    // r5 = r0 % r1 (12 % 5 = 2)
    chunk.emit(Opcode::Mod(5, 0, 1), 6);

    // Array operations: r6 = new Array(4), push r2, push r3
    chunk.emit(Opcode::ArrayNew(6, 4), 7);
    chunk.emit(Opcode::ArrayPush(6, 2), 8);
    chunk.emit(Opcode::ArrayPush(6, 3), 9);
    // r7 = len(r6) (should be 2)
    chunk.emit(Opcode::ArrayLen(7, 6), 10);

    // Map operations: r8 = new Map(), map_set(r8, "versi", "v2.0-enhanced")
    chunk.emit(Opcode::LoadConst(9, c_key), 11);
    chunk.emit(Opcode::LoadConst(10, c_str), 12);
    chunk.emit(Opcode::MapNew(8), 13);
    chunk.emit(Opcode::MapSet(8, 9, 10), 14);
    // r11 = map_get(r8, "versi")
    chunk.emit(Opcode::MapGet(11, 8, 9), 15);

    // Native Function Call: r12 = native_fn(r0, r1) -> 12 * 5 + 100 = 160
    vm.register_native(1, |args| {
        if let (VmValue::Int(a), VmValue::Int(b)) = (&args[0], &args[1]) {
            Ok(VmValue::Int(a * b + 100))
        } else {
            Err("Invalid args".into())
        }
    });
    chunk.emit(Opcode::CallNative(12, 1, 0, 2), 16);

    chunk.emit(Opcode::Return(12), 17);

    let idx = vm.load_chunk(chunk);
    let result = vm.execute(idx).expect("VM execution succeeded");

    assert_eq!(vm.registers[2], VmValue::Int(4));  // BitAnd
    assert_eq!(vm.registers[3], VmValue::Int(13)); // BitOr
    assert_eq!(vm.registers[4], VmValue::Int(9));  // BitXor
    assert_eq!(vm.registers[5], VmValue::Int(2));  // Modulo
    assert_eq!(vm.registers[7], VmValue::Int(2));  // ArrayLen
    assert_eq!(vm.registers[11], VmValue::String("v2.0-enhanced".to_string())); // MapGet
    assert_eq!(result, VmValue::Int(160)); // Native Call Return
}

#[test]
fn test_fiber_scheduler_priorities_and_channel_rendezvous() {
    let scheduler = WorkStealingScheduler::new(2);
    scheduler.start();

    let channel: FiberChannel<String> = FiberChannel::new(10);
    let ch_clone = channel.clone();

    // High priority worker produces messages
    scheduler.spawn_with_priority("producer_fiber", FiberPriority::High, move |_ctx| {
        for i in 1..=3 {
            ch_clone.send(format!("Pesan-{}", i)).unwrap();
        }
        Ok(())
    });

    // Normal priority worker consumes messages
    let (tx_done, rx_done) = std::sync::mpsc::channel();
    scheduler.spawn_with_priority("consumer_fiber", FiberPriority::Normal, move |_ctx| {
        let mut received = Vec::new();
        for _ in 1..=3 {
            if let Some(msg) = channel.recv_timeout(Duration::from_millis(500)) {
                received.push(msg);
            }
        }
        tx_done.send(received).unwrap();
        Ok(())
    });

    let msgs = rx_done.recv_timeout(Duration::from_secs(2)).expect("Consumer received all messages");
    assert_eq!(msgs.len(), 3);
    assert_eq!(msgs[0], "Pesan-1");
    assert_eq!(msgs[1], "Pesan-2");
    assert_eq!(msgs[2], "Pesan-3");

    scheduler.shutdown();
}

#[test]
fn test_borrow_checker_nll_liveness() {
    let mut checker = BorrowChecker::new();
    let span = Span::new(1, 1);

    // Register immutable borrow that expires at CFG point 5
    checker.borrow_immutable_nll("data_buffer", span.clone(), 5).unwrap();

    // At CFG point 3, mutating data_buffer should fail because loan is active until 5
    checker.update_nll_regions(3);

    // At CFG point 6, NLL automatically expires the borrow
    checker.update_nll_regions(6);

    // Now mutating / borrowing mutably should succeed
    let mut_res = checker.borrow_mutable("data_buffer", span);
    assert!(mut_res.is_ok(), "Mutable borrow should succeed after NLL loan expiry");
}

#[test]
fn test_formal_verifier_smt_and_loop_invariants() {
    // 1. SMT Constraints Verification
    let predicates = vec![
        SmtPredicate::GreaterThan("x".to_string(), 0),
        SmtPredicate::Range("y".to_string(), 10, 100),
        SmtPredicate::NonZero("z".to_string()),
    ];
    let concrete_safe = vec![
        ("x".to_string(), 15),
        ("y".to_string(), 50),
        ("z".to_string(), 7),
    ];
    assert!(FormalVerifier::verify_smt_constraints(&predicates, &concrete_safe).is_ok());

    let concrete_unsafe = vec![
        ("x".to_string(), -5), // Violated x > 0
        ("y".to_string(), 50),
        ("z".to_string(), 7),
    ];
    assert!(FormalVerifier::verify_smt_constraints(&predicates, &concrete_unsafe).is_err());

    // 2. Loop Invariant & Termination Verification
    let loop_inv = LoopInvariant {
        loop_name: "hitung_faktorial".to_string(),
        initial_counter: 0,
        step: 1,
        target_bound: 1000,
        invariant_condition: "i < 1000 && acc >= 1".to_string(),
    };
    assert!(FormalVerifier::verify_loop_termination(&loop_inv).is_ok());

    let infinite_loop = LoopInvariant {
        loop_name: "loop_macet".to_string(),
        initial_counter: 0,
        step: 0, // Step 0 causing infinite loop
        target_bound: 1000,
        invariant_condition: "true".to_string(),
    };
    assert!(FormalVerifier::verify_loop_termination(&infinite_loop).is_err());
}

#[test]
fn test_incremental_cache_and_hmr_engine() {
    // 1. Incremental Compilation Cache DAG
    let mut cache = IncrementalCache::new();
    let src_mod_a = "fungsi hitung() { kembalikan 42; }";
    let src_mod_b = "pakai mod_a; fungsi proses() { kembalikan hitung() * 2; }";

    cache.update_module("mod_a", src_mod_a, vec![], 128, 1001);
    cache.update_module("mod_b", src_mod_b, vec!["mod_a".to_string()], 256, 1002);

    assert_eq!(cache.cached_module_count(), 2);
    assert!(!cache.is_dirty("mod_a", src_mod_a));

    // When mod_a source changes, mod_a and downstream mod_b are dirty
    let src_mod_a_v2 = "fungsi hitung() { kembalikan 100; }";
    assert!(cache.is_dirty("mod_a", src_mod_a_v2));

    let affected = cache.get_affected_modules("mod_a");
    assert!(affected.contains("mod_a"));
    assert!(affected.contains("mod_b"));

    // 2. Hot-Module Replacement (HMR) State Preservation
    let mut hmr = HotModuleReloader::new();
    let mut live_state = HashMap::new();
    live_state.insert("user_session_id".to_string(), VmValue::Int(998877));
    live_state.insert("theme".to_string(), VmValue::String("dark".to_string()));

    hmr.save_state("web_dashboard", live_state.clone(), 2001);
    assert_eq!(hmr.get_current_version("web_dashboard"), 1);

    let restored = hmr.apply_patch("web_dashboard", "Optimasi render panel").unwrap();
    assert!(restored.is_some());
    let snapshot = restored.unwrap();
    assert_eq!(snapshot.global_bindings.get("user_session_id"), Some(&VmValue::Int(998877)));
    assert_eq!(hmr.get_current_version("web_dashboard"), 2);
    assert_eq!(hmr.total_patches_applied(), 1);
}

#[test]
fn test_vm_simd_vector_opcodes() {
    let mut vm = VirtualMachine::new();
    let mut chunk = BytecodeChunk::new();

    // Vektor A = [1.0, 2.0, 3.0], B = [4.0, 5.0, 6.0], Skalar = 2.0
    let c_vec_a = chunk.add_constant(VmValue::Array(vec![VmValue::Float(1.0), VmValue::Float(2.0), VmValue::Float(3.0)]));
    let c_vec_b = chunk.add_constant(VmValue::Array(vec![VmValue::Float(4.0), VmValue::Float(5.0), VmValue::Float(6.0)]));
    let c_scalar = chunk.add_constant(VmValue::Float(2.0));

    chunk.emit(Opcode::LoadConst(0, c_vec_a), 1);
    chunk.emit(Opcode::LoadConst(1, c_vec_b), 2);
    chunk.emit(Opcode::LoadConst(2, c_scalar), 3);

    // r3 = dot(A, B) = 1*4 + 2*5 + 3*6 = 32.0
    chunk.emit(Opcode::VecDot(3, 0, 1), 4);
    // r4 = vec_add(A, B) = [5.0, 7.0, 9.0]
    chunk.emit(Opcode::VecAdd(4, 0, 1), 5);
    // r5 = vec_scale(A, 2.0) = [2.0, 4.0, 6.0]
    chunk.emit(Opcode::VecScale(5, 0, 2), 6);

    chunk.emit(Opcode::Return(3), 7);

    let idx = vm.load_chunk(chunk);
    let ret = vm.execute(idx).expect("VM execute successful");

    assert_eq!(ret, VmValue::Float(32.0));
    assert_eq!(vm.registers[3], VmValue::Float(32.0));

    if let VmValue::Array(add_res) = &vm.registers[4] {
        assert_eq!(add_res.len(), 3);
        assert_eq!(add_res[0], VmValue::Float(5.0));
        assert_eq!(add_res[1], VmValue::Float(7.0));
        assert_eq!(add_res[2], VmValue::Float(9.0));
    } else {
        panic!("Expected array for VecAdd result");
    }

    if let VmValue::Array(scale_res) = &vm.registers[5] {
        assert_eq!(scale_res.len(), 3);
        assert_eq!(scale_res[0], VmValue::Float(2.0));
        assert_eq!(scale_res[1], VmValue::Float(4.0));
        assert_eq!(scale_res[2], VmValue::Float(6.0));
    } else {
        panic!("Expected array for VecScale result");
    }
}

#[test]
fn test_ast_to_bytecode_compiler_pipeline() {
    let span = Span::new(1, 1);
    let prog = Program {
        statements: vec![
            Stmt::VarDecl {
                name: "a".to_string(),
                is_const: false,
                initializer: Some(Expr::Number(15.0, span)),
                span,
            },
            Stmt::VarDecl {
                name: "b".to_string(),
                is_const: false,
                initializer: Some(Expr::Number(25.0, span)),
                span,
            },
            Stmt::VarDecl {
                name: "c".to_string(),
                is_const: false,
                initializer: Some(Expr::Binary {
                    left: Box::new(Expr::Identifier("a".to_string(), span)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Identifier("b".to_string(), span)),
                    span,
                }),
                span,
            },
            Stmt::Return {
                value: Some(Expr::Identifier("c".to_string(), span)),
                span,
            },
        ],
    };

    let compiler = BytecodeCompiler::new();
    let chunk = compiler.compile(&prog).expect("Compilation to bytecode succeeded");

    let mut vm = VirtualMachine::new();
    let idx = vm.load_chunk(chunk);
    let result = vm.execute(idx).expect("VM executed bytecode from AST");

    assert_eq!(result, VmValue::Int(40));
}

#[test]
fn test_vm_user_function_call_and_tco() {
    let mut vm = VirtualMachine::new();

    // Chunk 1: Fungsi tambah(a, b) -> a + b
    let mut fn_chunk = BytecodeChunk::new();
    fn_chunk.emit(Opcode::Add(2, 0, 1), 1);
    fn_chunk.emit(Opcode::Return(2), 2);
    let fn_idx = vm.load_chunk(fn_chunk) as u16;

    // Chunk 0: Main
    // r0 = 10, r1 = 20, r2 = Call(fn_idx, arg_start=0, count=2)
    // Return r2
    let mut main_chunk = BytecodeChunk::new();
    let c10 = main_chunk.add_constant(VmValue::Int(10));
    let c20 = main_chunk.add_constant(VmValue::Int(20));
    main_chunk.emit(Opcode::LoadConst(0, c10), 1);
    main_chunk.emit(Opcode::LoadConst(1, c20), 2);
    main_chunk.emit(Opcode::Call(2, fn_idx, 0, 2), 3);
    main_chunk.emit(Opcode::Return(2), 4);
    let main_idx = vm.load_chunk(main_chunk);

    let res = vm.execute(main_idx).expect("Call function succeeded");
    assert_eq!(res, VmValue::Int(30));

    // Uji Tail Call Optimization (TCO)
    let mut tco_vm = VirtualMachine::new();
    let mut target_chunk = BytecodeChunk::new();
    let c_bonus = target_chunk.add_constant(VmValue::Int(100));
    target_chunk.emit(Opcode::LoadConst(1, c_bonus), 1);
    target_chunk.emit(Opcode::Add(2, 0, 1), 2);
    target_chunk.emit(Opcode::Return(2), 3);
    let target_idx = tco_vm.load_chunk(target_chunk) as u16;

    let mut caller_chunk = BytecodeChunk::new();
    let c50 = caller_chunk.add_constant(VmValue::Int(50));
    caller_chunk.emit(Opcode::LoadConst(0, c50), 1);
    caller_chunk.emit(Opcode::TailCall(target_idx, 0, 1), 2);
    let caller_idx = tco_vm.load_chunk(caller_chunk);

    let tco_res = tco_vm.execute(caller_idx).expect("Tail call succeeded");
    assert_eq!(tco_res, VmValue::Int(150));
    assert_eq!(tco_vm.frames.len(), 0, "TCO tidak menambah kedalaman call stack");
}

#[test]
fn test_field_sensitive_borrow_checker() {
    let mut bc = BorrowChecker::new();
    let span = Span::new(1, 1);

    // 1. Peminjaman dua field berbeda dari struct yang sama secara simultan
    // user.nama dipinjam imutabel (&user.nama)
    assert!(bc.borrow_field_immutable("user", "nama", span).is_ok());
    assert!(bc.is_field_borrowed("user", "nama"));

    // user.saldo dipinjam mutabel (&mut user.saldo) -> HARUS SUKSES karena disjoint field!
    assert!(bc.borrow_field_mutable("user", "saldo", span).is_ok());
    assert!(bc.is_field_borrowed("user", "saldo"));

    // 2. Konflik mutasi pada field yang sedang dipinjam imutabel
    // Mencoba &mut user.nama -> HARUS GAGAL karena user.nama sedang dipinjam imutabel
    assert!(bc.borrow_field_mutable("user", "nama", span).is_err());

    // 3. Melepas peminjaman user.nama
    bc.release_field_borrow("user", "nama");
    assert!(!bc.is_field_borrowed("user", "nama"));

    // Sekarang user.nama dapat dipinjam mutabel
    assert!(bc.borrow_field_mutable("user", "nama", span).is_ok());
}
