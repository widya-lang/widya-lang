#![recursion_limit = "512"]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod ir;
pub mod llvm_backend;
pub mod arena;
pub mod unikernel;
pub mod incremental;
pub mod hmr;
pub mod generic_codegen;
pub mod jit;

pub use ir::{IrModule, IrFunction, BasicBlock, IrInstruction, IrValue, IrType, IrOptimizer};
pub use llvm_backend::LlvmBackend;
pub use arena::{ScopedArena, EscapeAnalysis, EscapeAnalysisReport, ArenaStats};
pub use unikernel::{UnikernelImageBuilder, UnikernelConfig, UnikernelArch, BootProtocol, UnikernelImageReport};
pub use incremental::{IncrementalCache, ModuleMetadata};
pub use hmr::{HotModuleReloader, StateSnapshot};
pub use generic_codegen::GenericCodeGenerator;
pub use jit::{JitEngine, JitExecutionReport};

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
}
