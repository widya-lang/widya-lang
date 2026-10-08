//! Widya Scoped Arena & Escape Analysis Engine
//! Menyediakan alokasi memori berlatensi nol (zero-allocation bump pointer) dengan batas skop O(1),
//! serta analisis pelarian memori (Escape Analysis) untuk mempromosikan alokasi heap ke stack/arena.

use std::alloc::{alloc, Layout};
use std::collections::{HashMap, HashSet};
use crate::compiler::ir::{IrFunction, IrInstruction, IrValue};

/// Status pelarian memori suatu variabel / objek
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeStatus {
    /// Variabel murni lokal, tidak pernah keluar dari fungsi (bisa dialokasikan di stack/arena lokal)
    NoEscape,
    /// Variabel dilewatkan sebagai argumen ke pemanggilan fungsi lain, masa hidupnya dibatasi oleh pemanggil
    ArgEscape,
    /// Variabel lolos ke luar fungsi (direturn atau disimpan ke pointer/global), wajib alokasi heap umum
    GlobalEscape,
}

/// Laporan hasil analisis pelarian memori (Escape Analysis Report)
#[derive(Debug, Clone)]
pub struct EscapeAnalysisReport {
    pub function_name: String,
    pub variables_analyzed: usize,
    pub stack_promoted: usize,      // NoEscape (Stack/Arena)
    pub arena_scoped: usize,        // ArgEscape (Scoped Arena)
    pub heap_escaped: usize,        // GlobalEscape (Heap)
    pub variable_statuses: HashMap<String, EscapeStatus>,
    pub heap_reduction_percentage: f64,
}

/// Engine Analisis Pelarian Memori (Escape Analysis)
pub struct EscapeAnalysis;

impl EscapeAnalysis {
    /// Menganalisis fungsi IR untuk mendeteksi alokasi mana yang lolos ke heap vs aman di stack/arena
    pub fn analyze_function(func: &IrFunction) -> EscapeAnalysisReport {
        let mut variable_statuses: HashMap<String, EscapeStatus> = HashMap::new();
        let mut returned_regs: HashSet<String> = HashSet::new();
        let mut passed_as_args: HashSet<String> = HashSet::new();
        let mut stored_to_ptrs: HashSet<String> = HashSet::new();
        let mut all_declared_vars: HashSet<String> = HashSet::new();

        // 1. Kumpulkan semua instruksi Return, Call, dan Store
        for block in &func.blocks {
            for inst in &block.instructions {
                match inst {
                    IrInstruction::Alloca { dst: IrValue::Reg(r), .. } => {
                        all_declared_vars.insert(r.clone());
                    }
                    IrInstruction::Assign { dst, src, .. } => {
                        if let IrValue::Reg(r) = dst { all_declared_vars.insert(r.clone()); }
                        if let IrValue::Reg(r) = src { all_declared_vars.insert(r.clone()); }
                    }
                    IrInstruction::Return { val: Some(IrValue::Reg(r)) } => {
                        all_declared_vars.insert(r.clone());
                        returned_regs.insert(r.clone());
                    }
                    IrInstruction::Call { dst, args, .. } => {
                        if let Some(IrValue::Reg(r)) = dst { all_declared_vars.insert(r.clone()); }
                        for arg in args {
                            if let IrValue::Reg(r) = arg {
                                all_declared_vars.insert(r.clone());
                                passed_as_args.insert(r.clone());
                            }
                        }
                    }
                    IrInstruction::Store { val, ptr } => {
                        if let IrValue::Reg(r) = val {
                            all_declared_vars.insert(r.clone());
                            stored_to_ptrs.insert(r.clone());
                        }
                        if let IrValue::Reg(r) = ptr {
                            all_declared_vars.insert(r.clone());
                        }
                    }
                    _ => {}
                }
            }
        }

        // 2. Tentukan status escape untuk setiap variabel
        let mut stack_promoted = 0;
        let mut arena_scoped = 0;
        let mut heap_escaped = 0;

        for var in &all_declared_vars {
            let status = if returned_regs.contains(var) || stored_to_ptrs.contains(var) {
                heap_escaped += 1;
                EscapeStatus::GlobalEscape
            } else if passed_as_args.contains(var) {
                arena_scoped += 1;
                EscapeStatus::ArgEscape
            } else {
                stack_promoted += 1;
                EscapeStatus::NoEscape
            };
            variable_statuses.insert(var.clone(), status);
        }

        let total = all_declared_vars.len();
        let non_heap = stack_promoted + arena_scoped;
        let heap_reduction_percentage = if total > 0 {
            (non_heap as f64 / total as f64) * 100.0
        } else {
            100.0
        };

        EscapeAnalysisReport {
            function_name: func.name.clone(),
            variables_analyzed: total,
            stack_promoted,
            arena_scoped,
            heap_escaped,
            variable_statuses,
            heap_reduction_percentage,
        }
    }
}

/// Statistik penggunaan Scoped Arena
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArenaStats {
    pub total_allocated_bytes: usize,
    pub peak_memory_bytes: usize,
    pub active_chunks: usize,
    pub allocation_count: usize,
    pub current_scope_depth: usize,
}

/// Alokator Memori Cepat Linear Bump Pointer dengan Dukungan Skop Bertingkat (Scoped Arena)
pub struct ScopedArena {
    chunk_size: usize,
    chunks: Vec<Vec<u8>>,
    current_chunk_idx: usize,
    current_offset: usize,
    scope_markers: Vec<(usize, usize)>, // (chunk_idx, offset)
    total_allocated_bytes: usize,
    peak_memory_bytes: usize,
    allocation_count: usize,
}

impl ScopedArena {
    pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024; // 64 KB per chunk

    pub fn new() -> Self {
        Self::with_chunk_size(Self::DEFAULT_CHUNK_SIZE)
    }

    pub fn with_chunk_size(chunk_size: usize) -> Self {
        let initial_chunk = Vec::with_capacity(chunk_size);
        Self {
            chunk_size,
            chunks: vec![initial_chunk],
            current_chunk_idx: 0,
            current_offset: 0,
            scope_markers: Vec::new(),
            total_allocated_bytes: 0,
            peak_memory_bytes: chunk_size,
            allocation_count: 0,
        }
    }

    /// Masuk ke skop baru (menyimpan titik balik offset alokasi)
    pub fn enter_scope(&mut self) -> usize {
        self.scope_markers.push((self.current_chunk_idx, self.current_offset));
        self.scope_markers.len()
    }

    /// Keluar dari skop (mengembalikan offset alokator dalam O(1) tanpa memicu free per elemen)
    pub fn exit_scope(&mut self) {
        if let Some((saved_chunk, saved_offset)) = self.scope_markers.pop() {
            self.current_chunk_idx = saved_chunk;
            self.current_offset = saved_offset;
            // Potong chunk yang dialokasikan di dalam skop
            if self.chunks.len() > saved_chunk + 1 {
                self.chunks.truncate(saved_chunk + 1);
            }
            if let Some(chunk) = self.chunks.get_mut(saved_chunk) {
                chunk.truncate(saved_offset);
            }
        }
    }

    /// Alokasi byte mentah dengan penyelarasan memori (memory alignment)
    pub fn alloc_raw(&mut self, size: usize, align: usize) -> *mut u8 {
        let align_mask = align.saturating_sub(1);
        let aligned_offset = (self.current_offset + align_mask) & !align_mask;

        if aligned_offset + size > self.chunk_size {
            // Butuh chunk baru
            let new_size = self.chunk_size.max(size);
            let mut new_chunk = Vec::with_capacity(new_size);
            let ptr = new_chunk.as_mut_ptr();
            unsafe {
                new_chunk.set_len(size);
            }
            self.chunks.push(new_chunk);
            self.current_chunk_idx = self.chunks.len() - 1;
            self.current_offset = size;
            self.total_allocated_bytes += size;
            self.peak_memory_bytes += new_size;
            self.allocation_count += 1;
            ptr
        } else {
            let chunk = &mut self.chunks[self.current_chunk_idx];
            let additional_needed = (aligned_offset + size).saturating_sub(chunk.len());
            if additional_needed > 0 {
                chunk.reserve(additional_needed);
                unsafe {
                    chunk.set_len(aligned_offset + size);
                }
            }
            let ptr = unsafe { chunk.as_mut_ptr().add(aligned_offset) };
            self.current_offset = aligned_offset + size;
            self.total_allocated_bytes += size;
            self.allocation_count += 1;
            ptr
        }
    }

    /// Alokasikan nilai tunggal bertipe Copy ke dalam arena
    pub fn alloc<T: Copy>(&mut self, val: T) -> &'static mut T {
        let layout = Layout::new::<T>();
        let ptr = self.alloc_raw(layout.size(), layout.align()) as *mut T;
        unsafe {
            ptr.write(val);
            &mut *ptr
        }
    }

    /// Alokasikan irisan (slice) bernilai Copy ke dalam arena
    pub fn alloc_slice<T: Copy>(&mut self, slice: &[T]) -> &'static mut [T] {
        let layout = Layout::array::<T>(slice.len()).unwrap_or(Layout::new::<T>());
        let ptr = self.alloc_raw(layout.size(), layout.align()) as *mut T;
        unsafe {
            for (i, item) in slice.iter().enumerate() {
                ptr.add(i).write(*item);
            }
            std::slice::from_raw_parts_mut(ptr, slice.len())
        }
    }

    /// Reset seluruh alokator dalam O(1) tanpa membuang memori yang sudah dialokasikan ke OS
    pub fn reset(&mut self) {
        for chunk in &mut self.chunks {
            chunk.clear();
        }
        self.current_chunk_idx = 0;
        self.current_offset = 0;
        self.scope_markers.clear();
        self.total_allocated_bytes = 0;
        self.allocation_count = 0;
    }

    /// Mendapatkan statistik terkini alokator
    pub fn stats(&self) -> ArenaStats {
        ArenaStats {
            total_allocated_bytes: self.total_allocated_bytes,
            peak_memory_bytes: self.peak_memory_bytes,
            active_chunks: self.chunks.len(),
            allocation_count: self.allocation_count,
            current_scope_depth: self.scope_markers.len(),
        }
    }
}

impl Default for ScopedArena {
    fn default() -> Self {
        Self::new()
    }
}
