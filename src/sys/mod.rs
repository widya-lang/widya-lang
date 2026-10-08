//! Sovereign System & Low-Level Runtime Module (src/sys/mod.rs)
//! Pilar III: Direct Syscalls & Sovereign Memory Allocator (Bebas Libc / Zero Libc Malloc)

pub mod syscall;
pub mod allocator;

pub use syscall::*;
pub use allocator::*;
