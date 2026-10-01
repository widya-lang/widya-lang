use widya::compiler::decimal::Desimal;
use widya::compiler::ir::*;
use widya::compiler::vm::{VirtualMachine, BytecodeChunk, Opcode, VmValue};
use widya::compiler::llvm_backend::LlvmBackend;
use widya::compiler::fiber::WorkStealingScheduler;
use std::time::Duration;
use std::thread;

#[test]
fn test_desimal_fixed_point_fintech() {
    let d1 = Desimal::dari_teks("1500000.5000").unwrap();
    let d2 = Desimal::dari_teks("250000.2500").unwrap();
    let total = d1 + d2;
    assert_eq!(total.to_string(), "1750000.7500");
    assert_eq!(total.format_rupiah(), "Rp 1.750.000,75");

    let selisih = d1 - d2;
    assert_eq!(selisih.to_string(), "1250000.2500");

    let d_kali = Desimal::dari_int(10) * Desimal::dari_f64(2.5);
    assert_eq!(d_kali.ke_f64(), 25.0);
}

#[test]
fn test_widya_ir_ssa_and_optimizer() {
    let mut module = IrModule::new("test_fintech".into());
    let mut func = IrFunction::new("hitung_bunga".into(), vec![("pokok".into(), IrType::Decimal)], IrType::Decimal);

    let mut bb = BasicBlock::new("entry".into());
    bb.push(IrInstruction::BinOp {
        dst: IrValue::Reg("t0".into()),
        op: IrBinOp::Add,
        lhs: IrValue::ConstInt(100),
        rhs: IrValue::ConstInt(50),
        ty: IrType::I64,
    });
    bb.push(IrInstruction::Return {
        val: Some(IrValue::Reg("t0".into())),
    });

    func.blocks.push(bb);
    module.tambah_fungsi(func);

    // Jalankan Optimizer
    IrOptimizer::optimasi(&mut module);

    let ir_dump = module.dump_ir();
    assert!(ir_dump.contains("define"));
    assert!(ir_dump.contains("150")); // Constant folding result 100 + 50
}

#[test]
fn test_register_bytecode_vm() {
    let mut vm = VirtualMachine::new();
    let mut chunk = BytecodeChunk::new();

    let c_a = chunk.add_constant(VmValue::Decimal(Desimal::dari_int(1000)));
    let c_b = chunk.add_constant(VmValue::Decimal(Desimal::dari_int(250)));

    chunk.emit(Opcode::LoadConst(0, c_a), 1); // R0 = 1000
    chunk.emit(Opcode::LoadConst(1, c_b), 1); // R1 = 250
    chunk.emit(Opcode::AddDec(2, 0, 1), 1);   // R2 = R0 + R1 (1250)
    chunk.emit(Opcode::Return(2), 1);

    let chunk_idx = vm.load_chunk(chunk);
    let result = vm.execute(chunk_idx).unwrap();

    if let VmValue::Decimal(d) = result {
        assert_eq!(d.ke_f64(), 1250.0);
    } else {
        panic!("Hasil VM bukan Desimal!");
    }
}

#[test]
fn test_llvm_backend_codegen() {
    let mut module = IrModule::new("modul_sovereign".into());
    let func = IrFunction::new("main".into(), vec![], IrType::I64);
    module.tambah_fungsi(func);

    let backend = LlvmBackend::new();
    let llvm_ir = backend.emit_llvm_ir(&module).unwrap();

    assert!(llvm_ir.contains("target triple = \"x86_64-unknown-linux-gnu\""));
    assert!(llvm_ir.contains("define i64 @main()"));
}

#[test]
fn test_fiber_concurrency_work_stealing() {
    let scheduler = WorkStealingScheduler::new(4);
    scheduler.start();

    for i in 0..100 {
        scheduler.spawn(&format!("task_{}", i), move |_ctx| {
            let _calc = i * i;
            Ok(())
        });
    }

    thread::sleep(Duration::from_millis(50));
    assert!(scheduler.get_completed_count() > 0);
    scheduler.shutdown();
}
