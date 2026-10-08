//! Suite Pengujian Komprehensif: Next-Gen Compiler & Runtime Frontiers
//! Menguji 6 pilar inovasi mendalam:
//! 1. Multi-Tier JIT & On-Stack Replacement (OSR) + Polymorphic Inline Cache (PIC) + Deopt Bailout
//! 2. SIMD Auto-Vectorization (AVX-512 & ARM Neon) SSA IR
//! 3. Scoped Arena Linear Allocator & Escape Analysis
//! 4. Bare-Metal Unikernel MicroVM Boot Target (Multiboot2 & Linux Direct Boot)
//! 5. Distributed Fiber Runtime & Erlang/OTP Supervision Trees
//! 6. AI-Guided Optimization Advisor & Chaitin-Briggs Graph Coloring Register Allocator

use std::sync::Arc;
use std::time::Duration;
use widya::compiler::{
    // Pillar 1: JIT & OSR & PIC
    VirtualMachine, BytecodeChunk, Opcode, VmValue, JitTier, PolymorphicInlineCache, DeoptReason,
    // Pillar 2: SIMD SSA IR
    IrModule, IrFunction, IrInstruction, IrType, IrValue, IrBinOp, BasicBlock, IrOptimizer,
    SimdTargetIsa,
    // Pillar 3: Scoped Arena & Escape Analysis
    ScopedArena, EscapeAnalysis, EscapeStatus,
    // Pillar 4: Bare-Metal Unikernel
    UnikernelConfig, UnikernelArch, BootProtocol, UnikernelImageBuilder, Multiboot2Header, LinuxBootHeader,
    // Pillar 5: Distributed Fiber & Supervision Trees
    WorkStealingScheduler, SupervisionStrategy, RestartPolicy, ChildSpec, Supervisor,
    DistributedFiberNetwork, DistributedMessage, FiberPid, FiberPriority,
    // Pillar 6: AI Advisor & Register Allocation
    AiOptimizationAdvisor, InlineDecision, ChaitinBriggsRegisterAllocator,
};

// =========================================================================
// PILAR 1: MULTI-TIER JIT, OSR, PIC, & DEOPTIMIZATION BAILOUTS
// =========================================================================

#[test]
fn test_pillar1_tiered_jit_and_osr_loop_trigger() {
    let mut vm = VirtualMachine::new();
    assert_eq!(vm.current_tier, JitTier::Tier0Interpreter);

    // Buat bytecode dengan loop yang memiliki backedge negatif (Jump offset < 0)
    let mut chunk = BytecodeChunk::new();
    // R0 = 0 (counter)
    let c0 = chunk.add_constant(VmValue::Int(0));
    chunk.emit(Opcode::LoadConst(0, c0), 1);
    // R1 = 1 (step)
    let c1 = chunk.add_constant(VmValue::Int(1));
    chunk.emit(Opcode::LoadConst(1, c1), 1);
    // R2 = 30 (limit)
    let c30 = chunk.add_constant(VmValue::Int(30));
    chunk.emit(Opcode::LoadConst(2, c30), 1);

    // Loop Header (index 3)
    // R3 = (R0 < R2)
    chunk.emit(Opcode::Lt(3, 0, 2), 2);
    // JumpIfFalse R3 ke exit (+2 inst: dari ip 5 ke ip 7)
    chunk.emit(Opcode::JumpIfFalse(3, 2), 2);
    // R0 = R0 + R1
    chunk.emit(Opcode::Add(0, 0, 1), 3);
    // Jump kembali ke loop header (-4 offset: dari ip 7 ke ip 3)
    chunk.emit(Opcode::Jump(-4), 3);

    // Exit (index 7)
    chunk.emit(Opcode::Return(0), 4);

    let chunk_idx = vm.load_chunk(chunk);
    let result = vm.execute_tiered(chunk_idx).expect("Eksekusi bytecode bertingkat harus sukses");

    assert_eq!(result, VmValue::Int(30));
    // Loop berputar 30 kali, dengan threshold OSR default 25, OSR harus terpicu dan mempromosikan ke Tier2
    assert!(vm.osr_promotions >= 1, "OSR harus terpicu pada loop panas");
    assert_eq!(vm.current_tier, JitTier::Tier2OptimizingOsr, "VM harus terpromosikan ke Tier 2 Optimizing OSR");
}

#[test]
fn test_pillar1_polymorphic_inline_caching() {
    let mut pic = PolymorphicInlineCache::new(101);
    assert!(!pic.is_monomorphic());
    assert!(!pic.is_polymorphic());

    // 1. Rekam pemanggilan receiver Int (tag 2) ke fungsi 0x10
    pic.record(2, 0x10);
    assert!(pic.is_monomorphic(), "Satu receiver harus monomorphic");
    assert_eq!(pic.lookup(2), Some(0x10));

    // 2. Rekam pemanggilan receiver Float (tag 3) ke fungsi 0x20
    pic.record(3, 0x20);
    assert!(pic.is_polymorphic(), "Dua receiver berbeda harus polymorphic");
    assert_eq!(pic.lookup(3), Some(0x20));

    // 3. Tambahkan receiver String (tag 5) dan Array (tag 7)
    pic.record(5, 0x30);
    pic.record(7, 0x40);
    assert!(pic.is_polymorphic());

    // 4. Melebihi max_monomorphic_entries (4) beralih ke megamorphic
    pic.record(8, 0x50); // Map tag 8
    assert!(pic.megamorphic, "Lebih dari 4 jenis receiver harus beralih ke megamorphic");
    assert_eq!(pic.lookup(2), None, "Megamorphic cache melewati fast-path lookup");
}

#[test]
fn test_pillar1_deoptimization_bailout() {
    let mut vm = VirtualMachine::new();
    vm.current_tier = JitTier::Tier2OptimizingOsr;

    // Masukkan data register
    vm.registers[0] = VmValue::String("teks".into());
    vm.registers[1] = VmValue::Int(42);

    let bailout = vm.bailout_to_interpreter(0, 5, DeoptReason::TypeGuardFailed {
        expected: 2, // Int
        actual: 5,   // String
    });

    assert_eq!(vm.current_tier, JitTier::Tier0Interpreter, "Bailout harus mengembalikan VM ke Tier 0 Interpreter");
    assert_eq!(vm.deopt_log.len(), 1);
    assert_eq!(bailout.chunk_idx, 0);
    assert_eq!(bailout.ip, 5);
    assert!(!bailout.restored_registers.is_empty(), "Register harus tersimpan saat bailout");
}

// =========================================================================
// PILAR 2: SIMD AUTO-VECTORIZATION (AVX-512 & ARM NEON) IN SSA IR
// =========================================================================

#[test]
fn test_pillar2_simd_auto_vectorization_avx512_and_neon() {
    let _module = IrModule::new("simd_vector_test".into());

    let mut func = IrFunction::new("vector_add_kernel".into(), vec![
        ("a".into(), IrType::Pointer(Box::new(IrType::F64))),
        ("b".into(), IrType::Pointer(Box::new(IrType::F64))),
        ("n".into(), IrType::I64),
    ], IrType::Void);

    let mut header = BasicBlock::new("loop_header".into());
    // i = 0 .. n loop
    header.push(IrInstruction::BinOp {
        dst: IrValue::Reg("t0".into()),
        op: IrBinOp::Add,
        lhs: IrValue::Reg("a_elem".into()),
        rhs: IrValue::Reg("b_elem".into()),
        ty: IrType::F64,
    });
    header.push(IrInstruction::Jump { target_label: "loop_header".into() });
    func.blocks.push(header);

    // 1. Uji target AVX-512 (64-byte vector, 8 x 64-bit float)
    let plan_avx = IrOptimizer::auto_vectorize_loops(&mut func, SimdTargetIsa::Avx512);
    assert_eq!(plan_avx.target_isa, SimdTargetIsa::Avx512);
    assert_eq!(plan_avx.vector_lane_width, 8, "AVX-512 harus memiliki lane width 8 untuk f64");
    assert_eq!(plan_avx.loops_vectorized, 1);
    assert!(plan_avx.estimated_speedup > 6.0, "Estimasi speedup AVX-512 harus > 6x");

    // Pastikan instruksi ditransformasikan menjadi VectorOp
    let first_inst = &func.blocks[0].instructions[0];
    assert!(matches!(first_inst, IrInstruction::VectorOp { lane_width: 8, .. }));

    // 2. Uji target ARM Neon (16-byte vector, 2 x 64-bit float atau 4 x 32-bit)
    let lane_neon_f64 = SimdTargetIsa::ArmNeon.vector_lane_width(8);
    let lane_neon_f32 = SimdTargetIsa::ArmNeon.vector_lane_width(4);
    assert_eq!(lane_neon_f64, 2, "ARM Neon memproses 2 x f64");
    assert_eq!(lane_neon_f32, 4, "ARM Neon memproses 4 x f32");
}

// =========================================================================
// PILAR 3: ESCAPE ANALYSIS & SCOPED ARENA MEMORY ALLOCATOR
// =========================================================================

#[test]
fn test_pillar3_scoped_arena_linear_allocation_and_unwind() {
    let mut arena = ScopedArena::with_chunk_size(4096);

    // Alokasikan beberapa angka di level root
    let p1 = arena.alloc(12345i64);
    assert_eq!(*p1, 12345);

    let initial_allocated = arena.stats().total_allocated_bytes;
    assert!(initial_allocated > 0);

    // Masuk ke skop bersarang (Level 1)
    arena.enter_scope();
    let p_scoped = arena.alloc(99.99f64);
    assert_eq!(*p_scoped, 99.99);

    let slice_data = [10i32, 20, 30, 40, 50];
    let p_slice = arena.alloc_slice(&slice_data);
    assert_eq!(p_slice, &[10, 20, 30, 40, 50]);

    assert_eq!(arena.stats().current_scope_depth, 1);

    // Keluar dari skop (Rollback O(1))
    arena.exit_scope();
    assert_eq!(arena.stats().current_scope_depth, 0);

    // Reset total
    arena.reset();
    assert_eq!(arena.stats().total_allocated_bytes, 0);
    assert_eq!(arena.stats().allocation_count, 0);
}

#[test]
fn test_pillar3_escape_analysis_detection() {
    let mut func = IrFunction::new("process_tensor".into(), vec![], IrType::I64);
    let mut bb = BasicBlock::new("entry".into());

    // v_local: tidak lolos
    bb.push(IrInstruction::Assign {
        dst: IrValue::Reg("v_local".into()),
        src: IrValue::ConstInt(100),
        ty: IrType::I64,
    });
    // v_arg: dilewatkan ke pemanggilan fungsi lain
    bb.push(IrInstruction::Call {
        dst: None,
        func_name: "helper_print".into(),
        args: vec![IrValue::Reg("v_arg".into())],
        ret_ty: IrType::Void,
    });
    // v_escaped: direturn ke luar fungsi
    bb.push(IrInstruction::Return {
        val: Some(IrValue::Reg("v_escaped".into())),
    });
    func.blocks.push(bb);

    let report = EscapeAnalysis::analyze_function(&func);
    assert_eq!(report.variables_analyzed, 3);
    assert_eq!(report.stack_promoted, 1, "v_local harus dipromosikan ke stack (NoEscape)");
    assert_eq!(report.arena_scoped, 1, "v_arg harus dialokasikan di arena (ArgEscape)");
    assert_eq!(report.heap_escaped, 1, "v_escaped harus lolos ke heap (GlobalEscape)");

    assert_eq!(report.variable_statuses.get("v_local"), Some(&EscapeStatus::NoEscape));
    assert_eq!(report.variable_statuses.get("v_arg"), Some(&EscapeStatus::ArgEscape));
    assert_eq!(report.variable_statuses.get("v_escaped"), Some(&EscapeStatus::GlobalEscape));
    assert!(report.heap_reduction_percentage >= 60.0);
}

// =========================================================================
// PILAR 4: BARE-METAL UNIKERNEL & MICROVM BOOT TARGET
// =========================================================================

#[test]
fn test_pillar4_unikernel_multiboot2_and_linux_direct_boot() {
    // 1. Verifikasi Multiboot2 Header
    let mb2 = Multiboot2Header::emit();
    assert!(mb2.len() >= 24);
    let magic = u32::from_le_bytes(mb2[0..4].try_into().unwrap());
    assert_eq!(magic, 0xE85250D6, "Magic multiboot2 harus 0xE85250D6");

    let arch = u32::from_le_bytes(mb2[4..8].try_into().unwrap());
    let len = u32::from_le_bytes(mb2[8..12].try_into().unwrap());
    let checksum = u32::from_le_bytes(mb2[12..16].try_into().unwrap());
    let sum = magic.wrapping_add(arch).wrapping_add(len).wrapping_add(checksum);
    assert_eq!(sum, 0, "Checksum Multiboot2 (magic + arch + len + checksum) harus 0");

    // 2. Verifikasi Linux 64-bit Direct Boot Header
    let linux_hdr = LinuxBootHeader::emit("console=ttyS0 quiet");
    assert_eq!(linux_hdr[510], 0x55);
    assert_eq!(linux_hdr[511], 0xAA, "MBR Boot flag 0xAA55 harus berada di offset 510-511");
    let hdrs = &linux_hdr[514..518];
    assert_eq!(hdrs, b"HdrS", "Linux kernel header magic 'HdrS' harus valid");

    // 3. Bangun citra unikernel utuh untuk Firecracker MicroVM
    let cfg = UnikernelConfig {
        target_arch: UnikernelArch::X86_64,
        boot_protocol: BootProtocol::LinuxDirectKernelBoot,
        memory_limit_mb: 32,
        kernel_cmdline: "console=ttyS0 panic=1 reboot=k".to_string(),
        enable_serial_uart: true,
        enable_virtio_net: true,
        enable_virtio_block: true,
        enable_acpi_shutdown: true,
    };
    let builder = UnikernelImageBuilder::new(cfg);
    let payload = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x01, 0x02]; // Bytecode payload
    let rep = builder.build_kernel_image(&payload);

    assert!(rep.image_size_bytes > 1024);
    assert!(rep.footprint_mb < 4.0, "Footprint unikernel harus di bawah 4 MB");
    assert!(rep.estimated_boot_time_ms < 5.0, "Waktu boot unikernel harus di bawah 5 ms");
}

// =========================================================================
// PILAR 5: DISTRIBUTED FIBER RUNTIME & SUPERVISION TREES
// =========================================================================

#[test]
fn test_pillar5_fiber_supervision_tree_one_for_one() {
    let scheduler = Arc::new(WorkStealingScheduler::new(2));
    scheduler.start();

    let mut supervisor = Supervisor::new("root_supervisor", SupervisionStrategy::OneForOne, Arc::clone(&scheduler));

    // Tambahkan 2 worker fiber
    supervisor.add_child(ChildSpec::new("worker_a", RestartPolicy::Transient, FiberPriority::Normal, |_ctx| {
        Ok(())
    }));
    supervisor.add_child(ChildSpec::new("worker_b", RestartPolicy::Transient, FiberPriority::High, |_ctx| {
        Ok(())
    }));

    supervisor.start_children();

    // Simulasikan kegagalan worker_a
    assert_eq!(supervisor.get_restart_count("worker_a"), 0);
    let restarted = supervisor.handle_child_failure("worker_a", "Panic: Division by zero simulated");
    assert!(restarted, "Supervisor harus berhasil me-restart child");
    assert_eq!(supervisor.get_restart_count("worker_a"), 1, "Restart counter worker_a harus naik");
    assert_eq!(supervisor.get_restart_count("worker_b"), 0, "worker_b tidak boleh terpengaruh dalam strategi OneForOne");

    scheduler.shutdown();
}

#[test]
fn test_pillar5_distributed_fiber_network_message_routing() {
    let net = DistributedFiberNetwork::new("node_alpha");

    let local_msg = DistributedMessage {
        from: FiberPid { node_id: "node_alpha".into(), local_fiber_id: 1 },
        to: FiberPid { node_id: "node_alpha".into(), local_fiber_id: 2 },
        payload: "PING_LOCAL".into(),
    };
    net.route_message(local_msg);

    let remote_msg = DistributedMessage {
        from: FiberPid { node_id: "node_alpha".into(), local_fiber_id: 1 },
        to: FiberPid { node_id: "node_beta".into(), local_fiber_id: 42 },
        payload: "PING_REMOTE".into(),
    };
    net.route_message(remote_msg);

    // Periksa pesan lokal terkirim
    let received = net.receive_local();
    assert!(received.is_some());
    assert_eq!(received.unwrap().payload, "PING_LOCAL");

    // Periksa pesan remote masuk antrean transmisi node_beta
    assert_eq!(net.get_pending_remote_count("node_beta"), 1);
}

// =========================================================================
// PILAR 6: AI-GUIDED OPTIMIZATION ADVISOR & REGISTER ALLOCATOR
// =========================================================================

#[test]
fn test_pillar6_ai_optimization_advisor_heuristics() {
    let advisor = AiOptimizationAdvisor::new();

    // Fungsi kecil (3 instruksi) -> rekomendasi InlineAlways
    let mut small_fn = IrFunction::new("leaf_add".into(), vec![], IrType::I64);
    let mut b0 = BasicBlock::new("entry".into());
    b0.push(IrInstruction::Assign { dst: IrValue::Reg("a".into()), src: IrValue::ConstInt(1), ty: IrType::I64 });
    b0.push(IrInstruction::Assign { dst: IrValue::Reg("b".into()), src: IrValue::ConstInt(2), ty: IrType::I64 });
    b0.push(IrInstruction::Return { val: Some(IrValue::Reg("b".into())) });
    small_fn.blocks.push(b0);

    let advice_small = advisor.analyze_function(&small_fn, 50);
    assert!(matches!(advice_small.inline_decision, InlineDecision::InlineAlways { .. }), "Fungsi kecil harus direkomendasikan inline always");

    // Fungsi dengan loop backedge -> branch weights & loop unrolling factor
    let mut loop_fn = IrFunction::new("hot_loop".into(), vec![], IrType::Void);
    let mut b_loop = BasicBlock::new("loop_header".into());
    b_loop.push(IrInstruction::Branch {
        cond: IrValue::Reg("c".into()),
        then_label: "loop_body".into(),
        else_label: "loop_exit".into(),
    });
    b_loop.push(IrInstruction::Jump { target_label: "loop_header".into() });
    loop_fn.blocks.push(b_loop);

    let advice_loop = advisor.analyze_function(&loop_fn, 1000);
    assert!(advice_loop.loop_unroll_factor >= 2, "Hot loop harus direkomendasikan unroll minimal 2x atau 4x");
    assert!(!advice_loop.branch_prediction_weights.is_empty(), "AI advisor harus menghasilkan bobot prediksi branch");
}

#[test]
fn test_pillar6_chaitin_briggs_graph_coloring_register_allocator() {
    let allocator = ChaitinBriggsRegisterAllocator::new(4); // 4 register fisik (r0, r1, r2, r3)

    let mut func = IrFunction::new("matrix_cell_calc".into(), vec![], IrType::I64);
    let mut bb = BasicBlock::new("entry".into());
    bb.push(IrInstruction::Assign { dst: IrValue::Reg("v0".into()), src: IrValue::ConstInt(10), ty: IrType::I64 });
    bb.push(IrInstruction::Assign { dst: IrValue::Reg("v1".into()), src: IrValue::ConstInt(20), ty: IrType::I64 });
    bb.push(IrInstruction::BinOp {
        dst: IrValue::Reg("v2".into()),
        op: IrBinOp::Add,
        lhs: IrValue::Reg("v0".into()),
        rhs: IrValue::Reg("v1".into()),
        ty: IrType::I64,
    });
    bb.push(IrInstruction::Return { val: Some(IrValue::Reg("v2".into())) });
    func.blocks.push(bb);

    let alloc_res = allocator.allocate(&func);
    assert!(alloc_res.registers_used > 0, "Harus mengalokasikan register fisik");
    assert!(alloc_res.spilled_registers.is_empty(), "Dengan 4 register untuk 3 vreg, tidak boleh ada spill");

    // Pastikan v0, v1, v2 dipetakan ke register fisik unik yang sah
    assert!(alloc_res.mapping.contains_key("v0"));
    assert!(alloc_res.mapping.contains_key("v1"));
    assert!(alloc_res.mapping.contains_key("v2"));
}
