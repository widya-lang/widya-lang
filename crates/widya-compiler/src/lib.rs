#![recursion_limit = "512"]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod ir;
pub mod mir;
pub mod mir_borrowck;
pub mod llvm_backend;
pub mod arena;
pub mod unikernel;
pub mod incremental;
pub mod hmr;
pub mod generic_codegen;
pub mod jit;
pub mod selfhost;
pub mod miri_ub_inspector;
pub mod clippy_linter;
pub mod formal_memory_model;
pub mod safety_certification;

pub use ir::{IrModule, IrFunction, BasicBlock, IrInstruction, IrValue, IrType, IrOptimizer};
pub use mir::{
    BorrowKind, Local, MirBasicBlock, MirBody, MirStatement, MirTerminator, Operand, Place,
    ProjectionElem, Rvalue,
};
pub use mir_borrowck::{
    Loan, MirBorrowChecker, MirBorrowckError, MirBorrowckErrorKind, MirBorrowckReport, MirLocation,
};
pub use llvm_backend::LlvmBackend;
pub use arena::{ScopedArena, EscapeAnalysis, EscapeAnalysisReport, ArenaStats};
pub use unikernel::{UnikernelImageBuilder, UnikernelConfig, UnikernelArch, BootProtocol, UnikernelImageReport};
pub use incremental::{IncrementalCache, ModuleMetadata};
pub use hmr::{HotModuleReloader, StateSnapshot};
pub use generic_codegen::GenericCodeGenerator;
pub use jit::{JitEngine, JitExecutionReport};
pub use selfhost::{BootstrapStage, SelfHostCompilerEngine, StageReport};
pub use miri_ub_inspector::{MiriUbInspector, UbInspectionReport, UbViolation, UbViolationKind};
pub use clippy_linter::{ClippyLintCategory, ClippyLintFinding, LintSeverity, WidyaClippyLinter};
pub use formal_memory_model::{
    FormalMemoryModelVerifier, FormalProofStatus, HoareTripleProof, MemoryActionKind,
    MemoryOrdering,
};
pub use safety_certification::{
    SafetyAuditEngine, SafetyComplianceMetrics, SafetyQualificationReport, SafetyStandard,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widya_ir_ssa_module_and_optimizer() {
        let mut module = IrModule::new("test_calc".into());
        let mut func = IrFunction::new("tambah".into(), vec![("x".into(), IrType::I64)], IrType::I64);

        let mut bb = BasicBlock::new("entry".into());
        bb.push(IrInstruction::BinOp {
            dst: IrValue::Reg("t0".into()),
            op: ir::IrBinOp::Add,
            lhs: IrValue::ConstInt(100),
            rhs: IrValue::ConstInt(50),
            ty: IrType::I64,
        });
        bb.push(IrInstruction::Return {
            val: Some(IrValue::Reg("t0".into())),
        });

        func.blocks.push(bb);
        module.tambah_fungsi(func);

        IrOptimizer::optimasi(&mut module);

        let ir_dump = module.dump_ir();
        assert!(ir_dump.contains("define"));
        assert!(ir_dump.contains("150"));
    }

    #[test]
    fn test_scoped_arena_fast_alloc() {
        let mut arena = ScopedArena::with_chunk_size(4096);
        let p1 = arena.alloc(12345i64);
        assert_eq!(*p1, 12345);

        arena.enter_scope();
        let p2 = arena.alloc(99.99f64);
        assert_eq!(*p2, 99.99);
        assert_eq!(arena.stats().current_scope_depth, 1);

        arena.exit_scope();
        assert_eq!(arena.stats().current_scope_depth, 0);
    }

    #[test]
    fn test_unikernel_microvm_builder() {
        let cfg = UnikernelConfig {
            target_arch: UnikernelArch::X86_64,
            boot_protocol: BootProtocol::LinuxDirectKernelBoot,
            memory_limit_mb: 64,
            kernel_cmdline: "console=ttyS0 panic=1".into(),
            enable_serial_uart: true,
            enable_virtio_net: true,
            enable_virtio_block: false,
            enable_acpi_shutdown: true,
        };
        let builder = UnikernelImageBuilder::new(cfg);
        let payload = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x01, 0x02];
        let rep = builder.build_kernel_image(&payload);

        assert!(rep.image_size_bytes > 1024);
        assert!(rep.footprint_mb < 4.0);
        assert!(rep.estimated_boot_time_ms < 5.0);
    }

    #[test]
    fn test_mir_cfg_and_terminators() {
        let mut body = MirBody::new("hitung_kuadrat".into(), 1, IrType::I64);
        let x = body.add_local("x", IrType::I64, true);
        let hasil = body.add_local("hasil", IrType::I64, false);

        // bb0: entry
        let bb0 = body.add_block();
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(hasil.clone()),
            Rvalue::BinaryOp(
                ir::IrBinOp::Mul,
                Operand::Copy(Place::from_local(x.clone())),
                Operand::Copy(Place::from_local(x.clone())),
            ),
        ));
        body.basic_blocks[bb0].set_terminator(MirTerminator::Goto { target: 1 });

        // bb1: exit
        let bb1 = body.add_block();
        body.basic_blocks[bb1].set_terminator(MirTerminator::Return);

        let preds = body.predecessors();
        assert_eq!(preds[1], vec![0]);
        assert_eq!(body.basic_blocks[0].successors(), vec![1]);

        let dump = body.dump_mir();
        assert!(dump.contains("hitung_kuadrat"));
        assert!(dump.contains("bb0:"));
        assert!(dump.contains("bb1:"));
    }

    #[test]
    fn test_mir_borrowck_safe_multiple_shared_borrows() {
        // fn test() { let x = 42; let p1 = &x; let p2 = &x; } -> AMAN (Aliasing XOR Mutability)
        let mut body = MirBody::new("test_shared".into(), 0, IrType::Void);
        let x = body.add_local("x", IrType::I64, true);
        let p1 = body.add_local("p1", IrType::Pointer(Box::new(IrType::I64)), false);
        let p2 = body.add_local("p2", IrType::Pointer(Box::new(IrType::I64)), false);

        let bb0 = body.add_block();
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(x.clone()),
            Rvalue::Use(Operand::Constant(IrValue::ConstInt(42))),
        ));
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(p1.clone()),
            Rvalue::Ref(BorrowKind::Shared, Place::from_local(x.clone())),
        ));
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(p2.clone()),
            Rvalue::Ref(BorrowKind::Shared, Place::from_local(x.clone())),
        ));
        body.basic_blocks[bb0].set_terminator(MirTerminator::Return);

        let report = MirBorrowChecker::new(&body).check();
        assert!(report.is_safe(), "Multiple shared borrows harus valid!");
        assert_eq!(report.errors.len(), 0);
    }

    #[test]
    fn test_mir_borrowck_detects_double_mut_borrow() {
        // fn test() { let mut x = 42; let p1 = &mut x; let p2 = &mut x; } -> GALAT E0501
        let mut body = MirBody::new("test_double_mut".into(), 0, IrType::Void);
        let x = body.add_local("x", IrType::I64, true);
        let p1 = body.add_local("p1", IrType::Pointer(Box::new(IrType::I64)), false);
        let p2 = body.add_local("p2", IrType::Pointer(Box::new(IrType::I64)), false);

        let bb0 = body.add_block();
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(x.clone()),
            Rvalue::Use(Operand::Constant(IrValue::ConstInt(42))),
        ));
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(p1.clone()),
            Rvalue::Ref(BorrowKind::Mut, Place::from_local(x.clone())),
        ));
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(p2.clone()),
            Rvalue::Ref(BorrowKind::Mut, Place::from_local(x.clone())),
        ));
        body.basic_blocks[bb0].set_terminator(MirTerminator::Return);

        let report = MirBorrowChecker::new(&body).check();
        assert!(!report.is_safe(), "Double mutable borrow harus memicu galat E0501!");
        assert_eq!(report.errors[0].code, "E0501");
    }

    #[test]
    fn test_mir_borrowck_detects_mut_while_shared() {
        // fn test() { let mut x = 42; let p1 = &x; let p2 = &mut x; } -> GALAT E0502
        let mut body = MirBody::new("test_mut_shared_conflict".into(), 0, IrType::Void);
        let x = body.add_local("x", IrType::I64, true);
        let p1 = body.add_local("p1", IrType::Pointer(Box::new(IrType::I64)), false);
        let p2 = body.add_local("p2", IrType::Pointer(Box::new(IrType::I64)), false);

        let bb0 = body.add_block();
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(x.clone()),
            Rvalue::Use(Operand::Constant(IrValue::ConstInt(42))),
        ));
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(p1.clone()),
            Rvalue::Ref(BorrowKind::Shared, Place::from_local(x.clone())),
        ));
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(p2.clone()),
            Rvalue::Ref(BorrowKind::Mut, Place::from_local(x.clone())),
        ));
        body.basic_blocks[bb0].set_terminator(MirTerminator::Return);

        let report = MirBorrowChecker::new(&body).check();
        assert!(!report.is_safe());
        assert_eq!(report.errors[0].code, "E0502");
    }

    #[test]
    fn test_mir_borrowck_detects_use_of_moved_value() {
        // fn test() { let x = "teks"; let y = move x; let z = move x; } -> GALAT E0382
        let mut body = MirBody::new("test_move_semantics".into(), 0, IrType::Void);
        let x = body.add_local("x", IrType::String, true);
        let y = body.add_local("y", IrType::String, true);
        let z = body.add_local("z", IrType::String, true);

        let bb0 = body.add_block();
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(x.clone()),
            Rvalue::Use(Operand::Constant(IrValue::ConstString("widya".into()))),
        ));
        // y = move x
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(y.clone()),
            Rvalue::Use(Operand::Move(Place::from_local(x.clone()))),
        ));
        // z = move x (Pelanggaran! x sudah dipindahkan ke y)
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(z.clone()),
            Rvalue::Use(Operand::Move(Place::from_local(x.clone()))),
        ));
        body.basic_blocks[bb0].set_terminator(MirTerminator::Return);

        let report = MirBorrowChecker::new(&body).check();
        assert!(!report.is_safe());
        assert_eq!(report.errors[0].code, "E0382");
        assert!(report.errors[0].message.contains("dipindahkan"));
    }

    #[test]
    fn test_mir_borrowck_detects_mutate_while_borrowed() {
        // fn test() { let mut x = 10; let p = &x; x = 20; } -> GALAT E0503
        let mut body = MirBody::new("test_mutate_while_borrowed".into(), 0, IrType::Void);
        let x = body.add_local("x", IrType::I64, true);
        let p = body.add_local("p", IrType::Pointer(Box::new(IrType::I64)), false);

        let bb0 = body.add_block();
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(x.clone()),
            Rvalue::Use(Operand::Constant(IrValue::ConstInt(10))),
        ));
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(p.clone()),
            Rvalue::Ref(BorrowKind::Shared, Place::from_local(x.clone())),
        ));
        // Penulisan ke x saat masih dipinjam oleh p
        body.basic_blocks[bb0].push_stmt(MirStatement::Assign(
            Place::from_local(x.clone()),
            Rvalue::Use(Operand::Constant(IrValue::ConstInt(20))),
        ));
        // p masih digunakan di terminator (memperpanjang lifetime p)
        body.basic_blocks[bb0].set_terminator(MirTerminator::Drop {
            place: Place::from_local(p.clone()),
            target: 0,
        });

        let report = MirBorrowChecker::new(&body).check();
        assert!(!report.is_safe());
        assert_eq!(report.errors[0].code, "E0503");
    }

    #[test]
    fn test_self_hosting_stage0_to_stage2_pipeline() {
        let mut engine = SelfHostCompilerEngine::new();
        engine.register_module(
            "token.widya",
            "struktur Token { jenis: String, nilai: String }\nfungsi ambil_jenis() -> String { kembalikan \"TOKEN\"; }",
        );
        engine.register_module(
            "parser.widya",
            "fungsi parsing() -> Int { kembalikan 1; }",
        );
        engine.register_module(
            "main.widya",
            "fungsi utama() -> Int { kembalikan 0; }",
        );

        let reports = engine.run_bootstrap_pipeline().expect("Siklus bootstrap harus berhasil");
        assert_eq!(reports.len(), 3);

        // Stage 0
        assert_eq!(reports[0].stage, BootstrapStage::Stage0Host);
        assert!(reports[0].artifact_hash.starts_with("sha256:"));

        // Stage 1
        assert_eq!(reports[1].stage, BootstrapStage::Stage1SelfNative);
        assert!(reports[1].artifact_hash.starts_with("sha256:"));
        assert!(reports[1].generated_lines > 10);

        // Stage 2
        assert_eq!(reports[2].stage, BootstrapStage::Stage2Reproducible);
        assert!(reports[2].is_deterministic_match);
        assert!(reports[2].artifact_hash.contains("_deterministic_convergence"));

        let stage1_prefix = &reports[1].artifact_hash[7..30];
        assert!(reports[2].artifact_hash.contains(stage1_prefix));
    }

    #[test]
    fn test_miri_detects_null_pointer_and_use_after_free() {
        let mut miri = MiriUbInspector::new();
        let addr = miri.virtual_alloc(64, 8);

        // 1. Valid read/write
        miri.check_write(addr, 8, 8, "store i64 100, ptr");
        miri.check_read(addr, 8, 8, "load i64, ptr");

        // 2. Null Pointer Dereference (UB0001)
        miri.check_read(0, 4, 4, "load i32, null_ptr");

        // 3. Misaligned access (UB0002) - alamat ganjil addr + 1 untuk i64
        miri.check_read(addr + 1, 8, 8, "load misaligned i64");

        // 4. Free lalu Use-After-Free (UB0003)
        miri.virtual_free(addr, "free(ptr)");
        miri.check_read(addr, 8, 8, "load i64, freed_ptr");

        // 5. Out-of-bounds (UB0004)
        let addr2 = miri.virtual_alloc(16, 4);
        miri.check_write(addr2 + 12, 8, 4, "buffer_overflow_write");

        let rep = miri.finish_inspection();
        assert!(!rep.is_clean());
        let codes: Vec<&str> = rep.violations.iter().map(|v| v.code).collect();
        assert!(codes.contains(&"UB0001"), "Harus mendeteksi Null Pointer Dereference");
        assert!(codes.contains(&"UB0002"), "Harus mendeteksi Misaligned Access");
        assert!(codes.contains(&"UB0003"), "Harus mendeteksi Use-After-Free");
        assert!(codes.contains(&"UB0004"), "Harus mendeteksi Buffer Overflow");
    }

    #[test]
    fn test_miri_detects_stacked_borrows_aliasing_violation() {
        let mut miri = MiriUbInspector::new();
        let addr = miri.virtual_alloc(32, 4);

        // Pinjam secara imutabel
        miri.acquire_shared_borrow(addr);

        // Pelanggaran: Menulis ke memori saat ada pinjaman imutabel aktif (UB0008)
        miri.check_write(addr, 4, 4, "write_while_shared_borrow_active");

        let rep = miri.finish_inspection();
        assert!(!rep.is_clean());
        assert!(rep.violations.iter().any(|v| v.code == "UB0008"));
    }

    #[test]
    fn test_clippy_linter_detects_idiomatic_warnings() {
        let source = r#"
            fungsi contoh() {
                misal mut x = 10;
                misal _y = 20;
                jika true {
                    jika false {
                        cetak("nested");
                    }
                }
                kembalikan x;
                misal z = 30; // dead code
            }
        "#;

        let findings = WidyaClippyLinter::lint_source(source);
        let rule_ids: Vec<&str> = findings.iter().map(|f| f.rule_id).collect();

        assert!(rule_ids.contains(&"WIDYA_CLNT002_UNNECESSARY_MUT"), "Harus deteksi unnecessary mut x");
        assert!(rule_ids.contains(&"WIDYA_CLNT003_COLLAPSE_IF"), "Harus deteksi nested if collapse");
        assert!(rule_ids.contains(&"WIDYA_CLNT005_DEAD_CODE"), "Harus deteksi dead code setelah kembalikan");
    }

    #[test]
    fn test_formal_memory_model_proves_drf_and_detects_data_races() {
        // Kasus 1: Dua utas mengakses memori berbeda -> Terbukti Aman (DRF-SC)
        let mut verifier = FormalMemoryModelVerifier::new();
        verifier.record_event(1, 0x1000, 8, MemoryActionKind::Write, MemoryOrdering::SeqCst);
        verifier.record_event(2, 0x2000, 8, MemoryActionKind::Read, MemoryOrdering::SeqCst);
        verifier.register_hoare_proof(HoareTripleProof {
            function_name: "proses_independen".to_string(),
            precondition: "x > 0".to_string(),
            postcondition: "hasil > 0".to_string(),
            invariant_holds: true,
        });

        let proof = verifier.verify_drf_theorem();
        assert!(matches!(proof, FormalProofStatus::ProvenSafe { drf_sc_guaranteed: true, .. }));

        // Kasus 2: Dua utas mengakses alamat yang sama, salah satunya Write, TANPA happens-before -> Terdeteksi Data Race!
        let mut race_verifier = FormalMemoryModelVerifier::new();
        let e1 = race_verifier.record_event(1, 0x3000, 4, MemoryActionKind::Write, MemoryOrdering::Relaxed);
        let e2 = race_verifier.record_event(2, 0x3000, 4, MemoryActionKind::Read, MemoryOrdering::Relaxed);

        let race_proof = race_verifier.verify_drf_theorem();
        assert!(matches!(race_proof, FormalProofStatus::ViolationFound { rule_violation: "DATA_RACE_DRF_VIOLATION", .. }));

        // Kasus 3: Jika ditambahkan sinkronisasi happens-before (e1 -> hb e2), menjadi aman
        race_verifier.add_happens_before(e1, e2);
        let sync_proof = race_verifier.verify_drf_theorem();
        assert!(matches!(sync_proof, FormalProofStatus::ProvenSafe { drf_sc_guaranteed: true, .. }));
    }

    #[test]
    fn test_industrial_safety_qualification_iso26262_and_do178c() {
        // Program Kritis Otomotif yang Memenuhi Syarat (ISO 26262 ASIL-D)
        let compliant_metrics = SafetyComplianceMetrics {
            has_unbounded_recursion: false,
            has_dynamic_heap_after_init: false,
            has_unverified_division: false,
            has_lossy_type_cast: false,
            has_unreachable_dead_code: false,
            worst_case_stack_bytes: 4096,
            max_loop_iterations_bound: Some(100),
        };

        let asil_d_report = SafetyAuditEngine::audit_compliance(SafetyStandard::Iso26262AsilD, &compliant_metrics);
        assert!(asil_d_report.is_certified, "Harus lolos sertifikasi ISO 26262 ASIL-D");
        assert_eq!(asil_d_report.compliance_score, 100.0);
        assert!(asil_d_report.certified_seal_hash.starts_with("widya-safety-seal:sha256:"));

        // Program dengan Pelanggaran Keselamatan (Rekursi tak terbatas & Kode mati)
        let non_compliant_metrics = SafetyComplianceMetrics {
            has_unbounded_recursion: true,
            has_dynamic_heap_after_init: true,
            has_unverified_division: true,
            has_lossy_type_cast: false,
            has_unreachable_dead_code: true,
            worst_case_stack_bytes: 65536,
            max_loop_iterations_bound: None,
        };

        let do178c_report = SafetyAuditEngine::audit_compliance(SafetyStandard::Do178cDalA, &non_compliant_metrics);
        assert!(!do178c_report.is_certified, "Harus ditolak pada sertifikasi avionik DO-178C DAL-A");
        assert!(do178c_report.violations.len() >= 4);
    }
}
