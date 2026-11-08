//! Miri-Grade Undefined Behavior (UB) & Memory Safety Inspector for Widya-Lang
//! Mendeteksi pelanggaran memori tingkat rendah pada blok `tidak_aman` (unsafe), bare-metal,
//! dan representasi MIR selevel Miri di Rust (Null pointer, Misaligned access, Use-after-free,
//! Out-of-bounds, dan pelanggaran Stacked Borrows Aliasing).

use std::collections::{HashMap, HashSet};
use std::fmt;

/// Jenis-Jenis Undefined Behavior (UB) Standar Industri
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UbViolationKind {
    /// Dereferensi pointer bernilai 0 (Null Pointer Dereference)
    NullPointerDereference,
    /// Akses memori dengan alignment yang salah (misal i64 pada alamat ganjil)
    MisalignedAccess {
        address: usize,
        required_alignment: usize,
    },
    /// Akses ke blok memori yang telah dibebaskan / dealokasi (Use-After-Free)
    UseAfterFree {
        address: usize,
        size: usize,
    },
    /// Akses memori di luar batas blok yang dialokasikan (Out-Of-Bounds Buffer Overflow)
    OutOfBounds {
        address: usize,
        access_size: usize,
        block_start: usize,
        block_size: usize,
    },
    /// Pelanggaran Stacked Borrows / Aliasing (menulis saat ada referensi imutabel aktif)
    AliasingViolation {
        address: usize,
        reason: String,
    },
    /// Pembacaan byte memori yang belum diinisialisasi
    UninitializedMemoryRead {
        address: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UbViolation {
    pub code: &'static str,
    pub kind: UbViolationKind,
    pub description: String,
    pub instruction_trace: String,
}

impl fmt::Display for UbViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[UB-DETECTOR {}]: {} (Instruksi: {})",
            self.code, self.description, self.instruction_trace
        )
    }
}

/// Laporan Diagnostik Pemeriksaan Undefined Behavior
#[derive(Debug, Clone)]
pub struct UbInspectionReport {
    pub total_memory_accesses: usize,
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub violations: Vec<UbViolation>,
}

impl UbInspectionReport {
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Status Blok Memori Virtual dalam Simulator Miri
#[derive(Debug, Clone)]
pub struct VirtualMemoryBlock {
    pub start_address: usize,
    pub size: usize,
    pub alignment: usize,
    pub is_alive: bool,
    pub initialized_bytes: Vec<bool>,
    pub active_shared_borrows: usize,
    pub has_active_mutable_borrow: bool,
}

/// Simulator Mesin Eksekusi Virtual Memori Miri
pub struct MiriUbInspector {
    memory_blocks: HashMap<usize, VirtualMemoryBlock>,
    next_virtual_address: usize,
    violations: Vec<UbViolation>,
    access_counter: usize,
    alloc_counter: usize,
    dealloc_counter: usize,
}

impl MiriUbInspector {
    pub fn new() -> Self {
        Self {
            memory_blocks: HashMap::new(),
            next_virtual_address: 0x1000, // Mulai dari alamat 4KB untuk mencegah alamat 0
            violations: Vec::new(),
            access_counter: 0,
            alloc_counter: 0,
            dealloc_counter: 0,
        }
    }

    /// Alokasikan blok memori virtual dengan ukuran dan alignment tertentu
    pub fn virtual_alloc(&mut self, size: usize, alignment: usize) -> usize {
        let align = alignment.max(1);
        let rem = self.next_virtual_address % align;
        let start = if rem == 0 {
            self.next_virtual_address
        } else {
            self.next_virtual_address + (align - rem)
        };

        let block = VirtualMemoryBlock {
            start_address: start,
            size,
            alignment: align,
            is_alive: true,
            initialized_bytes: vec![false; size],
            active_shared_borrows: 0,
            has_active_mutable_borrow: false,
        };

        self.memory_blocks.insert(start, block);
        self.next_virtual_address = start + size + 16; // Beri celah batas (guard page)
        self.alloc_counter += 1;
        start
    }

    /// Dealokasikan blok memori virtual
    pub fn virtual_free(&mut self, address: usize, trace: &str) {
        self.dealloc_counter += 1;
        if let Some(block) = self.memory_blocks.get_mut(&address) {
            if !block.is_alive {
                self.violations.push(UbViolation {
                    code: "UB0006",
                    kind: UbViolationKind::UseAfterFree { address, size: block.size },
                    description: format!("Double Free terdeteksi: alamat 0x{:X} sudah pernah dibebaskan", address),
                    instruction_trace: trace.to_string(),
                });
            } else {
                block.is_alive = false;
            }
        } else {
            self.violations.push(UbViolation {
                code: "UB0007",
                kind: UbViolationKind::OutOfBounds { address, access_size: 0, block_start: 0, block_size: 0 },
                description: format!("Invalid Free: mencoba membebaskan pointer liar yang tidak dialokasikan di 0x{:X}", address),
                instruction_trace: trace.to_string(),
            });
        }
    }

    /// Periksa pembacaan memori (Read Access)
    pub fn check_read(&mut self, address: usize, size: usize, alignment: usize, trace: &str) {
        self.access_counter += 1;

        // 1. Cek Null Pointer Dereference
        if address == 0 {
            self.violations.push(UbViolation {
                code: "UB0001",
                kind: UbViolationKind::NullPointerDereference,
                description: "Dereferensi Null Pointer: membaca memori dari alamat 0x0".to_string(),
                instruction_trace: trace.to_string(),
            });
            return;
        }

        // 2. Cek Misaligned Memory Access
        if alignment > 0 && (address % alignment) != 0 {
            self.violations.push(UbViolation {
                code: "UB0002",
                kind: UbViolationKind::MisalignedAccess { address, required_alignment: alignment },
                description: format!(
                    "Akses memori tidak selaras (misaligned): alamat 0x{:X} bukan kelipatan dari {} byte",
                    address, alignment
                ),
                instruction_trace: trace.to_string(),
            });
        }

        // 3. Cari blok alokasi yang menaungi alamat ini
        let mut found = false;
        for (start, block) in &self.memory_blocks {
            if address >= *start && address < *start + block.size {
                found = true;
                if !block.is_alive {
                    self.violations.push(UbViolation {
                        code: "UB0003",
                        kind: UbViolationKind::UseAfterFree { address, size },
                        description: format!(
                            "Use-After-Free terdeteksi: membaca 0x{:X} yang berada pada blok yang telah dibebaskan",
                            address
                        ),
                        instruction_trace: trace.to_string(),
                    });
                } else if address + size > *start + block.size {
                    self.violations.push(UbViolation {
                        code: "UB0004",
                        kind: UbViolationKind::OutOfBounds {
                            address,
                            access_size: size,
                            block_start: *start,
                            block_size: block.size,
                        },
                        description: format!(
                            "Buffer Overflow: pembacaan {} byte pada 0x{:X} melampaui batas blok alokasi (kapasitas {} byte)",
                            size, address, block.size
                        ),
                        instruction_trace: trace.to_string(),
                    });
                }
                break;
            }
        }

        if !found {
            self.violations.push(UbViolation {
                code: "UB0005",
                kind: UbViolationKind::OutOfBounds {
                    address,
                    access_size: size,
                    block_start: 0,
                    block_size: 0,
                },
                description: format!("Akses memori liar: membaca alamat asing yang tidak terdaftar di 0x{:X}", address),
                instruction_trace: trace.to_string(),
            });
        }
    }

    /// Periksa penulisan memori (Write Access)
    pub fn check_write(&mut self, address: usize, size: usize, alignment: usize, trace: &str) {
        self.access_counter += 1;

        // 1. Cek Null Pointer
        if address == 0 {
            self.violations.push(UbViolation {
                code: "UB0001",
                kind: UbViolationKind::NullPointerDereference,
                description: "Dereferensi Null Pointer: menulis memori ke alamat 0x0".to_string(),
                instruction_trace: trace.to_string(),
            });
            return;
        }

        // 2. Cek Misalignment
        if alignment > 0 && (address % alignment) != 0 {
            self.violations.push(UbViolation {
                code: "UB0002",
                kind: UbViolationKind::MisalignedAccess { address, required_alignment: alignment },
                description: format!(
                    "Penulisan memori tidak selaras: alamat 0x{:X} bukan kelipatan {} byte",
                    address, alignment
                ),
                instruction_trace: trace.to_string(),
            });
        }

        // 3. Cek blok alokasi dan aliasing (Stacked Borrows)
        let mut found = false;
        let mut violation_to_push = None;

        for (start, block) in self.memory_blocks.iter_mut() {
            if address >= *start && address < *start + block.size {
                found = true;
                if !block.is_alive {
                    violation_to_push = Some(UbViolation {
                        code: "UB0003",
                        kind: UbViolationKind::UseAfterFree { address, size },
                        description: format!("Use-After-Free: penulisan ke 0x{:X} yang sudah dibebaskan", address),
                        instruction_trace: trace.to_string(),
                    });
                } else if address + size > *start + block.size {
                    violation_to_push = Some(UbViolation {
                        code: "UB0004",
                        kind: UbViolationKind::OutOfBounds {
                            address,
                            access_size: size,
                            block_start: *start,
                            block_size: block.size,
                        },
                        description: format!(
                            "Buffer Overflow: penulisan {} byte melampaui batas blok di 0x{:X}",
                            size, address
                        ),
                        instruction_trace: trace.to_string(),
                    });
                } else if block.active_shared_borrows > 0 {
                    violation_to_push = Some(UbViolation {
                        code: "UB0008",
                        kind: UbViolationKind::AliasingViolation {
                            address,
                            reason: format!(
                                "Stacked Borrows Violation: penulisan ke 0x{:X} saat terdapat {} pinjaman imutabel aktif",
                                address, block.active_shared_borrows
                            ),
                        },
                        description: "Pelanggaran Aliasing XOR Mutability pada level memori Miri".to_string(),
                        instruction_trace: trace.to_string(),
                    });
                } else {
                    // Tandai byte sebagai terinisialisasi
                    let rel_start = address - *start;
                    for b in 0..size {
                        if rel_start + b < block.initialized_bytes.len() {
                            block.initialized_bytes[rel_start + b] = true;
                        }
                    }
                }
                break;
            }
        }

        if let Some(v) = violation_to_push {
            self.violations.push(v);
        } else if !found {
            self.violations.push(UbViolation {
                code: "UB0005",
                kind: UbViolationKind::OutOfBounds {
                    address,
                    access_size: size,
                    block_start: 0,
                    block_size: 0,
                },
                description: format!("Akses memori liar: penulisan ke alamat tidak terdaftar di 0x{:X}", address),
                instruction_trace: trace.to_string(),
            });
        }
    }

    /// Catat peminjaman imutabel pada blok memori (untuk Stacked Borrows Aliasing Verification)
    pub fn acquire_shared_borrow(&mut self, address: usize) {
        for (start, block) in self.memory_blocks.iter_mut() {
            if address >= *start && address < *start + block.size {
                block.active_shared_borrows += 1;
                break;
            }
        }
    }

    pub fn release_shared_borrow(&mut self, address: usize) {
        for (start, block) in self.memory_blocks.iter_mut() {
            if address >= *start && address < *start + block.size {
                block.active_shared_borrows = block.active_shared_borrows.saturating_sub(1);
                break;
            }
        }
    }

    /// Hasilkan laporan komprehensif audit Undefined Behavior
    pub fn finish_inspection(self) -> UbInspectionReport {
        UbInspectionReport {
            total_memory_accesses: self.access_counter,
            total_allocations: self.alloc_counter,
            total_deallocations: self.dealloc_counter,
            violations: self.violations,
        }
    }
}
