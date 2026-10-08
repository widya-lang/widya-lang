#![recursion_limit = "512"]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod decimal;
pub mod vm;
pub mod fiber;
pub mod bytecode_compiler;

pub use decimal::Desimal;
pub use vm::{VirtualMachine, VmValue, Opcode, BytecodeChunk};
pub use fiber::{WorkStealingScheduler, Supervisor, ChildSpec, FiberPriority, FiberState, FiberContext, FiberMessage, FiberChannel, DistributedFiberNetwork};
pub use bytecode_compiler::{BytecodeCompiler, CompilationUnit};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desimal_fixed_point_math() {
        let d1 = Desimal::dari_teks("1500000.5000").unwrap();
        let d2 = Desimal::dari_teks("250000.2500").unwrap();
        let total = d1 + d2;
        assert_eq!(total.to_string(), "1750000.7500");
        assert_eq!(total.format_rupiah(), "Rp 1.750.000,75");

        let selisih = d1 - d2;
        assert_eq!(selisih.to_string(), "1250000.2500");
    }

    #[test]
    fn test_register_bytecode_vm_execution() {
        let mut vm = VirtualMachine::new();
        let mut chunk = BytecodeChunk::new();

        let c_a = chunk.add_constant(VmValue::Decimal(Desimal::dari_int(1000)));
        let c_b = chunk.add_constant(VmValue::Decimal(Desimal::dari_int(250)));

        chunk.emit(Opcode::LoadConst(0, c_a), 1);
        chunk.emit(Opcode::LoadConst(1, c_b), 1);
        chunk.emit(Opcode::AddDec(2, 0, 1), 1);
        chunk.emit(Opcode::Return(2), 1);

        let chunk_idx = vm.load_chunk(chunk);
        let result = vm.execute(chunk_idx).unwrap();

        assert_eq!(result, VmValue::Decimal(Desimal::dari_int(1250)));
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

        std::thread::sleep(std::time::Duration::from_millis(50));
        assert!(scheduler.get_completed_count() > 0);
        scheduler.shutdown();
    }
}
