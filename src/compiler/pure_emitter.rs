//! Sovereign Direct Machine Code & ELF-64 Emitter (Pilar I: Kompiler & Linker Mandiri)
//! 100% Bebas Ketergantungan Eksternal (Zero External Crates: Tanpa LLVM-sys, GCC, Clang, GNU ld, LLD).
//! Menghasilkan berkas biner eksekusi Linux ELF-64 (Executable and Linkable Format) mandiri
//! yang dapat langsung dieksekusi oleh OS Linux kernel via execve().

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineArch {
    X86_64,
    AArch64,
    RiscV64,
}

/// Struktur Header Standar ELF-64 (64 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,      // 2 = ET_EXEC
    pub e_machine: u16,   // 62 (0x3E) = EM_X86_64
    pub e_version: u32,   // 1 = EV_CURRENT
    pub e_entry: u64,     // Alamat virtual titik masuk (entry point)
    pub e_phoff: u64,     // Offset tabel program header
    pub e_shoff: u64,     // Offset tabel section header
    pub e_flags: u32,     // Flags arsitektur
    pub e_ehsize: u16,    // Ukuran header ELF ini (64 bytes)
    pub e_phentsize: u16, // Ukuran setiap entri program header (56 bytes)
    pub e_phnum: u16,     // Jumlah entri program header
    pub e_shentsize: u16, // Ukuran setiap entri section header (64 bytes)
    pub e_shnum: u16,     // Jumlah entri section header
    pub e_shstrndx: u16,  // Indeks section nama-nama section
}

/// Struktur Program Header ELF-64 (56 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Phdr {
    pub p_type: u32,   // 1 = PT_LOAD
    pub p_flags: u32,  // 5 = PF_R | PF_X, 6 = PF_R | PF_W
    pub p_offset: u64, // Offset segmen dalam berkas
    pub p_vaddr: u64,  // Alamat virtual di memori
    pub p_paddr: u64,  // Alamat fisik (biasanya sama dengan p_vaddr)
    pub p_filesz: u64, // Ukuran segmen dalam berkas
    pub p_memsz: u64,  // Ukuran segmen dalam memori
    pub p_align: u64,  // Penyelarasan memori (biasanya 4096 / 0x1000)
}

pub struct PureElfBuilder {
    pub arch: MachineArch,
    pub entry_point: u64,
    pub code_bytes: Vec<u8>,
    pub data_bytes: Vec<u8>,
}

impl PureElfBuilder {
    pub const DEFAULT_BASE_VADDR: u64 = 0x400000;
    pub const PAGE_SIZE: u64 = 0x1000;

    pub fn new(arch: MachineArch) -> Self {
        Self {
            arch,
            entry_point: Self::DEFAULT_BASE_VADDR + Self::PAGE_SIZE,
            code_bytes: Vec::new(),
            data_bytes: Vec::new(),
        }
    }

    /// Tambahkan string atau data konstan ke segmen data (.rodata)
    pub fn add_rodata(&mut self, data: &[u8]) -> u64 {
        let offset = self.data_bytes.len() as u64;
        self.data_bytes.extend_from_slice(data);
        // Virtual address data = base + 2 * PAGE_SIZE + offset
        Self::DEFAULT_BASE_VADDR + (2 * Self::PAGE_SIZE) + offset
    }

    /// Pembangkit instruksi x86_64 mandiri:
    /// mov eax, 1 (sys_write)
    pub fn emit_mov_rax_imm32(&mut self, val: u32) {
        // b8 [imm32]
        self.code_bytes.push(0xB8);
        self.code_bytes.extend_from_slice(&val.to_le_bytes());
    }

    /// mov rdi, imm32
    pub fn emit_mov_rdi_imm32(&mut self, val: u32) {
        // bf [imm32]
        self.code_bytes.push(0xBF);
        self.code_bytes.extend_from_slice(&val.to_le_bytes());
    }

    /// mov rsi, imm64 (alamat pointer memori)
    pub fn emit_mov_rsi_imm64(&mut self, val: u64) {
        // 48 be [imm64]
        self.code_bytes.extend_from_slice(&[0x48, 0xBE]);
        self.code_bytes.extend_from_slice(&val.to_le_bytes());
    }

    /// mov rdx, imm32 (panjang byte)
    pub fn emit_mov_rdx_imm32(&mut self, val: u32) {
        // ba [imm32]
        self.code_bytes.push(0xBA);
        self.code_bytes.extend_from_slice(&val.to_le_bytes());
    }

    /// syscall (0x0F, 0x05)
    pub fn emit_syscall(&mut self) {
        self.code_bytes.extend_from_slice(&[0x0F, 0x05]);
    }

    /// sys_exit(code)
    pub fn emit_exit(&mut self, exit_code: u32) {
        match self.arch {
            MachineArch::X86_64 => {
                // mov eax, 60 (sys_exit)
                self.emit_mov_rax_imm32(60);
                // mov rdi, exit_code
                self.emit_mov_rdi_imm32(exit_code);
                // syscall
                self.emit_syscall();
            }
            MachineArch::AArch64 => {
                self.emit_arm64_exit(exit_code as u16);
            }
            MachineArch::RiscV64 => {
                self.emit_riscv_exit(exit_code);
            }
        }
    }

    // ========================================================================
    // PEMBANGKIT INSTRUKSI AARCH64 (ARM64) MANDIRI (ZERO TOOLCHAIN)
    // ========================================================================

    /// Emit instruksi MOVZ Rd, #imm16, LSL #(shift_idx * 16)
    /// Opcode AArch64 MOVZ 64-bit: 0xd2800000 | (hw << 21) | (imm16 << 5) | Rd
    pub fn emit_arm64_movz(&mut self, rd: u8, imm16: u16, shift_idx: u8) {
        let instr: u32 = 0xd2800000
            | (((shift_idx as u32) & 0x3) << 21)
            | (((imm16 as u32) & 0xFFFF) << 5)
            | ((rd as u32) & 0x1F);
        self.code_bytes.extend_from_slice(&instr.to_le_bytes());
    }

    /// Emit instruksi MOVK Rd, #imm16, LSL #(shift_idx * 16)
    /// Opcode AArch64 MOVK 64-bit: 0xf2800000 | (hw << 21) | (imm16 << 5) | Rd
    pub fn emit_arm64_movk(&mut self, rd: u8, imm16: u16, shift_idx: u8) {
        let instr: u32 = 0xf2800000
            | (((shift_idx as u32) & 0x3) << 21)
            | (((imm16 as u32) & 0xFFFF) << 5)
            | ((rd as u32) & 0x1F);
        self.code_bytes.extend_from_slice(&instr.to_le_bytes());
    }

    /// Memuat nilai 64-bit ke register AArch64 menggunakan kombinasi MOVZ dan MOVK
    pub fn emit_arm64_mov_imm64(&mut self, rd: u8, val: u64) {
        let p0 = (val & 0xFFFF) as u16;
        let p1 = ((val >> 16) & 0xFFFF) as u16;
        let p2 = ((val >> 32) & 0xFFFF) as u16;
        let p3 = ((val >> 48) & 0xFFFF) as u16;

        self.emit_arm64_movz(rd, p0, 0);
        if p1 != 0 {
            self.emit_arm64_movk(rd, p1, 1);
        }
        if p2 != 0 {
            self.emit_arm64_movk(rd, p2, 2);
        }
        if p3 != 0 {
            self.emit_arm64_movk(rd, p3, 3);
        }
    }

    /// Emit instruksi syscall AArch64: `svc #0`
    /// Opcode AArch64: 0xd4000001 (Little Endian: 01 00 00 d4)
    pub fn emit_arm64_svc_zero(&mut self) {
        self.code_bytes.extend_from_slice(&0xd4000001u32.to_le_bytes());
    }

    /// sys_exit untuk Linux AArch64 (Syscall nr 93 di register x8, kode keluar di x0)
    pub fn emit_arm64_exit(&mut self, exit_code: u16) {
        // movz x8, #93 (Linux sys_exit di ARM64)
        self.emit_arm64_movz(8, 93, 0);
        // movz x0, exit_code
        self.emit_arm64_movz(0, exit_code, 0);
        // svc #0
        self.emit_arm64_svc_zero();
    }

    /// sys_write untuk Linux AArch64 (Syscall nr 64 di register x8: write(x0=fd, x1=buf, x2=len))
    pub fn emit_arm64_write(&mut self, fd: u16, buf_addr: u64, len: u16) {
        // movz x8, #64 (Linux sys_write di ARM64)
        self.emit_arm64_movz(8, 64, 0);
        // movz x0, fd
        self.emit_arm64_movz(0, fd, 0);
        // x1 = buf_addr
        self.emit_arm64_mov_imm64(1, buf_addr);
        // movz x2, len
        self.emit_arm64_movz(2, len, 0);
        // svc #0
        self.emit_arm64_svc_zero();
    }

    // ========================================================================
    // PEMBANGKIT INSTRUKSI RISC-V 64 (RV64GC/RV64I) MANDIRI (ZERO TOOLCHAIN)
    // ========================================================================

    /// Emit instruksi ADDI Rd, Rs1, imm12
    /// Opcode: 0x13, funct3 = 0x0
    pub fn emit_riscv_addi(&mut self, rd: u8, rs1: u8, imm12: i16) {
        let imm_u = (imm12 as u32) & 0xFFF;
        let instr: u32 = (imm_u << 20)
            | (((rs1 as u32) & 0x1F) << 15)
            | (0 << 12)
            | (((rd as u32) & 0x1F) << 7)
            | 0x13;
        self.code_bytes.extend_from_slice(&instr.to_le_bytes());
    }

    /// Emit instruksi LUI Rd, imm20
    /// Opcode: 0x37
    pub fn emit_riscv_lui(&mut self, rd: u8, imm20: u32) {
        let instr: u32 = (((imm20 as u32) & 0xFFFFF) << 12)
            | (((rd as u32) & 0x1F) << 7)
            | 0x37;
        self.code_bytes.extend_from_slice(&instr.to_le_bytes());
    }

    /// Emit instruksi SLLI Rd, Rs1, shamt (Shift Left Logical Immediate 64-bit)
    /// Opcode: 0x13, funct3 = 0x1, shamt = 6 bit [25:20]
    pub fn emit_riscv_slli(&mut self, rd: u8, rs1: u8, shamt: u8) {
        let instr: u32 = (((shamt as u32) & 0x3F) << 20)
            | (((rs1 as u32) & 0x1F) << 15)
            | (1 << 12)
            | (((rd as u32) & 0x1F) << 7)
            | 0x13;
        self.code_bytes.extend_from_slice(&instr.to_le_bytes());
    }

    /// Emit instruksi ORI Rd, Rs1, imm12
    /// Opcode: 0x13, funct3 = 0x6
    pub fn emit_riscv_ori(&mut self, rd: u8, rs1: u8, imm12: i16) {
        let imm_u = (imm12 as u32) & 0xFFF;
        let instr: u32 = (imm_u << 20)
            | (((rs1 as u32) & 0x1F) << 15)
            | (6 << 12)
            | (((rd as u32) & 0x1F) << 7)
            | 0x13;
        self.code_bytes.extend_from_slice(&instr.to_le_bytes());
    }

    /// Emit instruksi syscall RISC-V: `ecall`
    /// Opcode: 0x00000073 (Little Endian: 73 00 00 00)
    pub fn emit_riscv_ecall(&mut self) {
        self.code_bytes.extend_from_slice(&0x00000073u32.to_le_bytes());
    }

    /// Memuat immediate 64-bit ke register RISC-V
    pub fn emit_riscv_li64(&mut self, rd: u8, val: u64) {
        if val <= 0x7FF {
            // Nilai fit dalam 11 bit positif (signed 12-bit)
            self.emit_riscv_addi(rd, 0, val as i16);
        } else if val <= 0xFFFFFFFF {
            // Nilai 32-bit: lui + addi
            let imm32 = val as u32;
            let mut hi = imm32 >> 12;
            let lo = (imm32 & 0xFFF) as i32;
            let lo_signed = if lo >= 0x800 { lo - 0x1000 } else { lo };
            if lo >= 0x800 {
                hi = hi.wrapping_add(1);
            }
            self.emit_riscv_lui(rd, hi & 0xFFFFF);
            if lo_signed != 0 {
                self.emit_riscv_addi(rd, rd, lo_signed as i16);
            }
        } else {
            // 64-bit umum: muat bagian atas 32-bit lalu geser dan gabungkan
            let hi32 = (val >> 32) as u32;
            let lo32 = (val & 0xFFFFFFFF) as u32;
            self.emit_riscv_li64(rd, hi32 as u64);
            self.emit_riscv_slli(rd, rd, 32);

            let t0 = 5u8;
            let mut hi = lo32 >> 12;
            let lo = (lo32 & 0xFFF) as i32;
            let lo_signed = if lo >= 0x800 { lo - 0x1000 } else { lo };
            if lo >= 0x800 {
                hi = hi.wrapping_add(1);
            }
            self.emit_riscv_lui(t0, hi & 0xFFFFF);
            if lo_signed != 0 {
                self.emit_riscv_addi(t0, t0, lo_signed as i16);
            }
            // or rd, rd, t0 -> opcode 0x33, funct3 = 6, funct7 = 0
            let or_instr: u32 = (((t0 as u32) & 0x1F) << 20)
                | (((rd as u32) & 0x1F) << 15)
                | (6 << 12)
                | (((rd as u32) & 0x1F) << 7)
                | 0x33;
            self.code_bytes.extend_from_slice(&or_instr.to_le_bytes());
        }
    }

    /// sys_exit untuk Linux RISC-V 64 (Syscall nr 93 di register a7 (x17), kode keluar di a0 (x10))
    pub fn emit_riscv_exit(&mut self, exit_code: u32) {
        // li a7, 93 (sys_exit Linux RISC-V)
        self.emit_riscv_addi(17, 0, 93);
        // li a0, exit_code
        self.emit_riscv_li64(10, exit_code as u64);
        // ecall
        self.emit_riscv_ecall();
    }

    /// sys_write untuk Linux RISC-V 64 (Syscall nr 64 di a7 (x17): write(a0=fd, a1=buf, a2=len))
    pub fn emit_riscv_write(&mut self, fd: u32, buf_addr: u64, len: u32) {
        // li a7, 64 (sys_write Linux RISC-V)
        self.emit_riscv_addi(17, 0, 64);
        // li a0, fd (stdout = 1)
        self.emit_riscv_li64(10, fd as u64);
        // li a1, buf_addr
        self.emit_riscv_li64(11, buf_addr);
        // li a2, len
        self.emit_riscv_li64(12, len as u64);
        // ecall
        self.emit_riscv_ecall();
    }

    /// Rakit dan hasilkan citra biner berkas ELF-64 mandiri
    pub fn build_elf_binary(&self) -> Vec<u8> {
        let mut binary = Vec::new();

        let ehdr_size = 64usize;
        let phdr_size = 56usize;
        let num_phdrs = if self.data_bytes.is_empty() { 1u16 } else { 2u16 };
        let phoff = ehdr_size as u64;

        // 1. Inisialisasi ELF Header
        let mut e_ident = [0u8; 16];
        e_ident[0..4].copy_from_slice(b"\x7fELF"); // Magic
        e_ident[4] = 2; // ELFCLASS64 (64-bit)
        e_ident[5] = 1; // ELFDATA2LSB (Little Endian)
        e_ident[6] = 1; // EV_CURRENT (Version 1)
        e_ident[7] = 0; // ELFOSABI_NONE (System V ABI)

        let machine_val = match self.arch {
            MachineArch::X86_64 => 62u16,
            MachineArch::AArch64 => 183u16,
            MachineArch::RiscV64 => 243u16,
        };

        // Alamat entry point adalah awal dari segmen kode (.text)
        let code_offset = Self::PAGE_SIZE;
        let code_vaddr = Self::DEFAULT_BASE_VADDR + code_offset;

        // Tulis ELF Header (64 byte)
        binary.extend_from_slice(&e_ident);
        binary.extend_from_slice(&2u16.to_le_bytes()); // e_type: ET_EXEC
        binary.extend_from_slice(&machine_val.to_le_bytes()); // e_machine
        binary.extend_from_slice(&1u32.to_le_bytes()); // e_version
        binary.extend_from_slice(&code_vaddr.to_le_bytes()); // e_entry
        binary.extend_from_slice(&phoff.to_le_bytes()); // e_phoff
        binary.extend_from_slice(&0u64.to_le_bytes()); // e_shoff (tanpa section headers)
        binary.extend_from_slice(&0u32.to_le_bytes()); // e_flags
        binary.extend_from_slice(&(ehdr_size as u16).to_le_bytes()); // e_ehsize
        binary.extend_from_slice(&(phdr_size as u16).to_le_bytes()); // e_phentsize
        binary.extend_from_slice(&num_phdrs.to_le_bytes()); // e_phnum
        binary.extend_from_slice(&0u16.to_le_bytes()); // e_shentsize
        binary.extend_from_slice(&0u16.to_le_bytes()); // e_shnum
        binary.extend_from_slice(&0u16.to_le_bytes()); // e_shstrndx

        // 2. Tulis Program Header untuk Segmen Kode (PT_LOAD: R + X)
        let code_filesz = self.code_bytes.len() as u64;
        let code_memsz = code_filesz;

        binary.extend_from_slice(&1u32.to_le_bytes()); // p_type: PT_LOAD (1)
        binary.extend_from_slice(&5u32.to_le_bytes()); // p_flags: PF_R (4) | PF_X (1) = 5
        binary.extend_from_slice(&code_offset.to_le_bytes()); // p_offset
        binary.extend_from_slice(&code_vaddr.to_le_bytes()); // p_vaddr
        binary.extend_from_slice(&code_vaddr.to_le_bytes()); // p_paddr
        binary.extend_from_slice(&code_filesz.to_le_bytes()); // p_filesz
        binary.extend_from_slice(&code_memsz.to_le_bytes()); // p_memsz
        binary.extend_from_slice(&Self::PAGE_SIZE.to_le_bytes()); // p_align

        // 3. Jika ada segmen data, tulis Program Header untuk Segmen Data (PT_LOAD: R + W)
        if !self.data_bytes.is_empty() {
            let data_offset = 2 * Self::PAGE_SIZE;
            let data_vaddr = Self::DEFAULT_BASE_VADDR + data_offset;
            let data_filesz = self.data_bytes.len() as u64;

            binary.extend_from_slice(&1u32.to_le_bytes()); // p_type: PT_LOAD
            binary.extend_from_slice(&6u32.to_le_bytes()); // p_flags: PF_R (4) | PF_W (2) = 6
            binary.extend_from_slice(&data_offset.to_le_bytes()); // p_offset
            binary.extend_from_slice(&data_vaddr.to_le_bytes()); // p_vaddr
            binary.extend_from_slice(&data_vaddr.to_le_bytes()); // p_paddr
            binary.extend_from_slice(&data_filesz.to_le_bytes()); // p_filesz
            binary.extend_from_slice(&data_filesz.to_le_bytes()); // p_memsz
            binary.extend_from_slice(&Self::PAGE_SIZE.to_le_bytes()); // p_align
        }

        // Padding hingga offset code (1 halaman = 4096 bytes)
        if binary.len() < code_offset as usize {
            binary.resize(code_offset as usize, 0);
        }

        // 4. Masukkan kode mesin (.text)
        binary.extend_from_slice(&self.code_bytes);

        // 5. Masukkan data (.rodata) jika ada pada offset halaman kedua
        if !self.data_bytes.is_empty() {
            let data_offset = (2 * Self::PAGE_SIZE) as usize;
            if binary.len() < data_offset {
                binary.resize(data_offset, 0);
            }
            binary.extend_from_slice(&self.data_bytes);
        }

        binary
    }

    /// Menurunkan (lowering) AST Widya-Lang secara langsung ke kode mesin x86_64 dan menghasilkan biner ELF-64 mandiri
    pub fn compile_ast_to_elf64(program: &crate::ast::Program) -> Result<Vec<u8>, String> {
        let mut compiler = AstToMachineCompiler::new();
        compiler.compile_program(program)
    }
}

// ============================================================================
// PILAR C-3: DIRECT AST-TO-X86_64 MACHINE CODE COMPILER (TANPA RUSTC/GCC/LD)
// ============================================================================

pub struct AstToMachineCompiler {
    pub builder: PureElfBuilder,
    pub var_slots: std::collections::HashMap<String, i8>, // [rbp - offset]
    pub next_slot: i8,
    pub func_offsets: std::collections::HashMap<String, usize>,
    pub call_fixups: Vec<(usize, String)>, // (pos_of_rel32, func_name)
}

impl AstToMachineCompiler {
    pub fn new() -> Self {
        Self {
            builder: PureElfBuilder::new(MachineArch::X86_64),
            var_slots: std::collections::HashMap::new(),
            next_slot: 0,
            func_offsets: std::collections::HashMap::new(),
            call_fixups: Vec::new(),
        }
    }

    fn alloc_slot(&mut self, name: &str) -> i8 {
        if let Some(&off) = self.var_slots.get(name) {
            off
        } else {
            self.next_slot -= 8;
            let off = self.next_slot;
            self.var_slots.insert(name.to_string(), off);
            off
        }
    }

    fn emit(&mut self, bytes: &[u8]) {
        self.builder.code_bytes.extend_from_slice(bytes);
    }

    fn current_pos(&self) -> usize {
        self.builder.code_bytes.len()
    }

    fn patch_rel32(&mut self, pos: usize, target: usize) {
        let rel = (target as i64 - (pos as i64 + 4)) as i32;
        self.builder.code_bytes[pos..pos + 4].copy_from_slice(&rel.to_le_bytes());
    }

    pub fn compile_program(&mut self, program: &crate::ast::Program) -> Result<Vec<u8>, String> {
        use crate::ast::Stmt;

        // Jump over function bodies to _start entry point: jmp rel32 (E9 00 00 00 00)
        self.emit(&[0xE9, 0x00, 0x00, 0x00, 0x00]);
        let entry_jmp_fixup = 1usize;

        // Emit built-in integer-to-stdout subroutine `__widya_print_i64` (expects value in RAX)
        let print_i64_offset = self.current_pos();
        self.func_offsets.insert("__widya_print_i64".to_string(), print_i64_offset);
        self.emit_print_i64_subroutine();

        // Pass 1: Compile all FunctionDecl
        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, params, body, .. } = stmt {
                let fn_start = self.current_pos();
                self.func_offsets.insert(name.clone(), fn_start);

                let saved_vars = self.var_slots.clone();
                let saved_slot = self.next_slot;
                self.var_slots.clear();
                self.next_slot = 0;

                // Prologue: push rbp; mov rbp, rsp; sub rsp, 128
                self.emit(&[0x55, 0x48, 0x89, 0xE5, 0x48, 0x81, 0xEC, 0x80, 0x00, 0x00, 0x00]);

                // Store ABI parameter registers (rdi, rsi, rdx, rcx, r8, r9) into stack slots
                let arg_reg_encodings: &[&[u8]] = &[
                    &[0x48, 0x89, 0x7D], // mov [rbp + disp8], rdi
                    &[0x48, 0x89, 0x75], // mov [rbp + disp8], rsi
                    &[0x48, 0x89, 0x55], // mov [rbp + disp8], rdx
                    &[0x48, 0x89, 0x4D], // mov [rbp + disp8], rcx
                    &[0x4C, 0x89, 0x45], // mov [rbp + disp8], r8
                    &[0x4C, 0x89, 0x4D], // mov [rbp + disp8], r9
                ];
                for (i, p) in params.iter().take(6).enumerate() {
                    let slot = self.alloc_slot(p);
                    self.emit(arg_reg_encodings[i]);
                    self.emit(&[slot as u8]);
                }

                for s in body {
                    self.compile_stmt(s, false)?;
                }

                // Default epilogue if function didn't explicitly return: xor eax, eax; leave; ret
                self.emit(&[0x31, 0xC0, 0xC9, 0xC3]);

                self.var_slots = saved_vars;
                self.next_slot = saved_slot;
            }
        }

        // Patch entry point jump to land here at `_start`
        let main_start = self.current_pos();
        self.patch_rel32(entry_jmp_fixup, main_start);

        // Main prologue: mov rbp, rsp; sub rsp, 128
        self.emit(&[0x48, 0x89, 0xE5, 0x48, 0x81, 0xEC, 0x80, 0x00, 0x00, 0x00]);

        for stmt in &program.statements {
            if !matches!(stmt, Stmt::FunctionDecl { .. }) {
                self.compile_stmt(stmt, true)?;
            }
        }

        // Default exit(0): mov eax, 60; xor edi, edi; syscall
        self.emit(&[0xB8, 0x3C, 0x00, 0x00, 0x00, 0x31, 0xFF, 0x0F, 0x05]);

        // Resolve all function call rel32 fixups
        let fixups = self.call_fixups.clone();
        for (pos, fn_name) in fixups {
            if let Some(&target) = self.func_offsets.get(&fn_name) {
                self.patch_rel32(pos, target);
            }
        }

        Ok(self.builder.build_elf_binary())
    }

    /// Subroutine mesin x86_64 untuk mencetak bilangan bulat 64-bit di RAX ke stdout (diakhiri '\n')
    fn emit_print_i64_subroutine(&mut self) {
        // push rbp; mov rbp, rsp; sub rsp, 32
        self.emit(&[0x55, 0x48, 0x89, 0xE5, 0x48, 0x83, 0xEC, 0x20]);
        // mov byte [rbp - 1], 10 ('\n')
        self.emit(&[0xC6, 0x45, 0xFF, 0x0A]);
        // lea rsi, [rbp - 1]
        self.emit(&[0x48, 0x8D, 0x75, 0xFF]);
        // mov rcx, 1 (char count = 1 for '\n')
        self.emit(&[0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00]);
        // mov r8, 10 (divisor = 10)
        self.emit(&[0x49, 0xC7, 0xC0, 0x0A, 0x00, 0x00, 0x00]);
        // loop_start:
        let loop_pos = self.current_pos();
        // xor edx, edx; div r8
        self.emit(&[0x31, 0xD2, 0x49, 0xF7, 0xF0]);
        // add dl, '0' (0x30)
        self.emit(&[0x80, 0xC2, 0x30]);
        // dec rsi; mov [rsi], dl; inc rcx
        self.emit(&[0x48, 0xFF, 0xCE, 0x88, 0x16, 0x48, 0xFF, 0xC1]);
        // test rax, rax; jnz loop_start
        self.emit(&[0x48, 0x85, 0xC0]);
        let jnz_rel = (loop_pos as i64 - (self.current_pos() as i64 + 2)) as i8;
        self.emit(&[0x75, jnz_rel as u8]);
        // sys_write(1, rsi, rcx): mov rdx, rcx; mov eax, 1; mov edi, 1; syscall
        self.emit(&[
            0x48, 0x89, 0xCA, 0xB8, 0x01, 0x00, 0x00, 0x00, 0xBF, 0x01, 0x00, 0x00, 0x00, 0x0F,
            0x05,
        ]);
        // leave; ret
        self.emit(&[0xC9, 0xC3]);
    }

    fn compile_stmt(&mut self, stmt: &crate::ast::Stmt, is_main: bool) -> Result<(), String> {
        use crate::ast::{AssignOp, AssignTarget, Expr, Stmt};

        match stmt {
            Stmt::VarDecl { name, initializer, .. } => {
                let slot = self.alloc_slot(name);
                if let Some(init) = initializer {
                    self.compile_expr(init)?;
                } else {
                    self.emit(&[0x31, 0xC0]); // xor eax, eax
                }
                // mov [rbp + slot], rax
                self.emit(&[0x48, 0x89, 0x45, slot as u8]);
            }
            Stmt::Assignment { target, op, value, .. } => {
                if let AssignTarget::Variable(name) = target {
                    let slot = self.alloc_slot(name);
                    self.compile_expr(value)?;
                    match op {
                        AssignOp::Assign => {
                            self.emit(&[0x48, 0x89, 0x45, slot as u8]); // mov [rbp+slot], rax
                        }
                        AssignOp::PlusAssign => {
                            self.emit(&[0x48, 0x01, 0x45, slot as u8]); // add [rbp+slot], rax
                        }
                        AssignOp::MinusAssign => {
                            self.emit(&[0x48, 0x29, 0x45, slot as u8]); // sub [rbp+slot], rax
                        }
                        AssignOp::StarAssign => {
                            // mov rcx, [rbp+slot]; imul rax, rcx; mov [rbp+slot], rax
                            self.emit(&[
                                0x48, 0x8B, 0x4D, slot as u8, 0x48, 0x0F, 0xAF, 0xC1, 0x48, 0x89,
                                0x45, slot as u8,
                            ]);
                        }
                        AssignOp::SlashAssign => {
                            // mov rcx, rax; mov rax, [rbp+slot]; cqo; idiv rcx; mov [rbp+slot], rax
                            self.emit(&[
                                0x48, 0x89, 0xC1, 0x48, 0x8B, 0x45, slot as u8, 0x48, 0x99, 0x48,
                                0xF7, 0xF9, 0x48, 0x89, 0x45, slot as u8,
                            ]);
                        }
                    }
                }
            }
            Stmt::Block(stmts, _) => {
                for s in stmts {
                    self.compile_stmt(s, is_main)?;
                }
            }
            Stmt::If { condition, then_branch, else_branch, .. } => {
                self.compile_expr(condition)?;
                // test rax, rax; jz else_label (0F 84 rel32)
                self.emit(&[0x48, 0x85, 0xC0, 0x0F, 0x84, 0x00, 0x00, 0x00, 0x00]);
                let jz_fixup = self.current_pos() - 4;

                self.compile_stmt(then_branch, is_main)?;

                if let Some(else_stmt) = else_branch {
                    // jmp end_label (E9 rel32)
                    self.emit(&[0xE9, 0x00, 0x00, 0x00, 0x00]);
                    let jmp_end_fixup = self.current_pos() - 4;
                    let else_pos = self.current_pos();
                    self.patch_rel32(jz_fixup, else_pos);

                    self.compile_stmt(else_stmt, is_main)?;
                    let end_pos = self.current_pos();
                    self.patch_rel32(jmp_end_fixup, end_pos);
                } else {
                    let end_pos = self.current_pos();
                    self.patch_rel32(jz_fixup, end_pos);
                }
            }
            Stmt::While { condition, body, .. } => {
                let loop_start = self.current_pos();
                self.compile_expr(condition)?;
                // test rax, rax; jz exit_label
                self.emit(&[0x48, 0x85, 0xC0, 0x0F, 0x84, 0x00, 0x00, 0x00, 0x00]);
                let jz_exit = self.current_pos() - 4;

                self.compile_stmt(body, is_main)?;

                // jmp loop_start
                self.emit(&[0xE9, 0x00, 0x00, 0x00, 0x00]);
                let jmp_back = self.current_pos() - 4;
                self.patch_rel32(jmp_back, loop_start);

                let after_loop = self.current_pos();
                self.patch_rel32(jz_exit, after_loop);
            }
            Stmt::ForIn { var_name, iterable, body, .. } => {
                if let Expr::Range { start, end, .. } = iterable {
                    let var_slot = self.alloc_slot(var_name);
                    self.compile_expr(start)?;
                    self.emit(&[0x48, 0x89, 0x45, var_slot as u8]); // mov [rbp+var], rax

                    let end_slot = self.alloc_slot(&format!("__end_{}", var_name));
                    self.compile_expr(end)?;
                    self.emit(&[0x48, 0x89, 0x45, end_slot as u8]); // mov [rbp+end], rax

                    let loop_start = self.current_pos();
                    // mov rax, [rbp+var]; cmp rax, [rbp+end]; jg exit
                    self.emit(&[
                        0x48, 0x8B, 0x45, var_slot as u8, 0x48, 0x3B, 0x45, end_slot as u8, 0x0F,
                        0x8F, 0x00, 0x00, 0x00, 0x00,
                    ]);
                    let jg_exit = self.current_pos() - 4;

                    self.compile_stmt(body, is_main)?;

                    // inc qword [rbp+var]; jmp loop_start
                    self.emit(&[0x48, 0xFF, 0x45, var_slot as u8, 0xE9, 0x00, 0x00, 0x00, 0x00]);
                    let jmp_back = self.current_pos() - 4;
                    self.patch_rel32(jmp_back, loop_start);

                    let after_loop = self.current_pos();
                    self.patch_rel32(jg_exit, after_loop);
                }
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.compile_expr(v)?;
                } else {
                    self.emit(&[0x31, 0xC0]); // xor eax, eax
                }
                if is_main {
                    // In _start: sys_exit(rax) -> mov rdi, rax; mov eax, 60; syscall
                    self.emit(&[0x48, 0x89, 0xC7, 0xB8, 0x3C, 0x00, 0x00, 0x00, 0x0F, 0x05]);
                } else {
                    // In function: leave; ret
                    self.emit(&[0xC9, 0xC3]);
                }
            }
            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &crate::ast::Expr) -> Result<(), String> {
        use crate::ast::{BinaryOp, Expr, UnaryOp};

        match expr {
            Expr::Number(n, _) => {
                let val = *n as i64;
                // movabs rax, imm64 (48 B8 + 8 bytes)
                self.emit(&[0x48, 0xB8]);
                self.emit(&val.to_le_bytes());
            }
            Expr::Boolean(b, _) => {
                let val = if *b { 1i64 } else { 0i64 };
                self.emit(&[0x48, 0xB8]);
                self.emit(&val.to_le_bytes());
            }
            Expr::Identifier(name, _) => {
                let slot = self.alloc_slot(name);
                // mov rax, [rbp + slot]
                self.emit(&[0x48, 0x8B, 0x45, slot as u8]);
            }
            Expr::Grouping(inner, _) => {
                self.compile_expr(inner)?;
            }
            Expr::Unary { op, right, .. } => {
                self.compile_expr(right)?;
                match op {
                    UnaryOp::Negate => self.emit(&[0x48, 0xF7, 0xD8]), // neg rax
                    UnaryOp::Not => {
                        // test rax, rax; sete al; movzx rax, al
                        self.emit(&[0x48, 0x85, 0xC0, 0x0F, 0x94, 0xC0, 0x48, 0x0F, 0xB6, 0xC0]);
                    }
                }
            }
            Expr::Binary { left, op, right, .. } => {
                self.compile_expr(left)?;
                self.emit(&[0x50]); // push rax
                self.compile_expr(right)?;
                self.emit(&[0x48, 0x89, 0xC1]); // mov rcx, rax (right in rcx)
                self.emit(&[0x58]); // pop rax (left in rax)

                match op {
                    BinaryOp::Add => self.emit(&[0x48, 0x01, 0xC8]), // add rax, rcx
                    BinaryOp::Subtract => self.emit(&[0x48, 0x29, 0xC8]), // sub rax, rcx
                    BinaryOp::Multiply => self.emit(&[0x48, 0x0F, 0xAF, 0xC1]), // imul rax, rcx
                    BinaryOp::Divide => {
                        self.emit(&[0x48, 0x99, 0x48, 0xF7, 0xF9]); // cqo; idiv rcx
                    }
                    BinaryOp::Modulo => {
                        self.emit(&[0x48, 0x99, 0x48, 0xF7, 0xF9, 0x48, 0x89, 0xD0]); // cqo; idiv rcx; mov rax, rdx
                    }
                    BinaryOp::Equal => {
                        self.emit(&[0x48, 0x39, 0xC8, 0x0F, 0x94, 0xC0, 0x48, 0x0F, 0xB6, 0xC0]);
                    }
                    BinaryOp::NotEqual => {
                        self.emit(&[0x48, 0x39, 0xC8, 0x0F, 0x95, 0xC0, 0x48, 0x0F, 0xB6, 0xC0]);
                    }
                    BinaryOp::Less => {
                        self.emit(&[0x48, 0x39, 0xC8, 0x0F, 0x9C, 0xC0, 0x48, 0x0F, 0xB6, 0xC0]);
                    }
                    BinaryOp::LessEqual => {
                        self.emit(&[0x48, 0x39, 0xC8, 0x0F, 0x9E, 0xC0, 0x48, 0x0F, 0xB6, 0xC0]);
                    }
                    BinaryOp::Greater => {
                        self.emit(&[0x48, 0x39, 0xC8, 0x0F, 0x9F, 0xC0, 0x48, 0x0F, 0xB6, 0xC0]);
                    }
                    BinaryOp::GreaterEqual => {
                        self.emit(&[0x48, 0x39, 0xC8, 0x0F, 0x9D, 0xC0, 0x48, 0x0F, 0xB6, 0xC0]);
                    }
                    BinaryOp::And => self.emit(&[0x48, 0x21, 0xC8]),
                    BinaryOp::Or => self.emit(&[0x48, 0x09, 0xC8]),
                    BinaryOp::Power => {
                        self.emit(&[0x48, 0x0F, 0xAF, 0xC1]);
                    }
                }
            }
            Expr::Call { callee, arguments, .. } => {
                if let Expr::Identifier(fn_name, _) = &**callee {
                    // Built-in print via direct Linux sys_write!
                    if fn_name == "cetak" || fn_name == "print" || fn_name == "tulis" {
                        let add_newline = fn_name != "tulis";
                        for arg in arguments {
                            if let Expr::String(s, _) = arg {
                                let mut bytes = s.as_bytes().to_vec();
                                if add_newline {
                                    bytes.push(b'\n');
                                }
                                let vaddr = self.builder.add_rodata(&bytes);
                                self.builder.emit_mov_rax_imm32(1); // sys_write
                                self.builder.emit_mov_rdi_imm32(1); // stdout
                                self.builder.emit_mov_rsi_imm64(vaddr);
                                self.builder.emit_mov_rdx_imm32(bytes.len() as u32);
                                self.builder.emit_syscall();
                            } else {
                                self.compile_expr(arg)?;
                                // Call `__widya_print_i64`
                                self.emit(&[0xE8, 0x00, 0x00, 0x00, 0x00]);
                                let fixup_pos = self.current_pos() - 4;
                                self.call_fixups.push((fixup_pos, "__widya_print_i64".to_string()));
                            }
                        }
                        return Ok(());
                    }

                    // Bitwise operators lowered directly to x86_64 instructions!
                    if matches!(
                        fn_name.as_str(),
                        "_bit_dan" | "_bit_atau" | "_bit_xor" | "_bit_kiri" | "_bit_kanan"
                    ) && arguments.len() == 2
                    {
                        self.compile_expr(&arguments[0])?;
                        self.emit(&[0x50]); // push rax
                        self.compile_expr(&arguments[1])?;
                        self.emit(&[0x48, 0x89, 0xC1]); // mov rcx, rax
                        self.emit(&[0x58]); // pop rax
                        match fn_name.as_str() {
                            "_bit_dan" => self.emit(&[0x48, 0x21, 0xC8]),  // and rax, rcx
                            "_bit_atau" => self.emit(&[0x48, 0x09, 0xC8]), // or rax, rcx
                            "_bit_xor" => self.emit(&[0x48, 0x31, 0xC8]),  // xor rax, rcx
                            "_bit_kiri" => self.emit(&[0x48, 0xD3, 0xE0]), // shl rax, cl
                            "_bit_kanan" => self.emit(&[0x48, 0xD3, 0xF8]), // sar rax, cl
                            _ => {}
                        }
                        return Ok(());
                    }
                    if fn_name == "_bit_bukan" && arguments.len() == 1 {
                        self.compile_expr(&arguments[0])?;
                        self.emit(&[0x48, 0xF7, 0xD0]); // not rax
                        return Ok(());
                    }

                    // User function call via System V AMD64 ABI (rdi, rsi, rdx, rcx, r8, r9)
                    for arg in arguments.iter().take(6) {
                        self.compile_expr(arg)?;
                        self.emit(&[0x50]); // push rax
                    }
                    let pop_regs: &[&[u8]] = &[
                        &[0x5F],       // pop rdi
                        &[0x5E],       // pop rsi
                        &[0x5A],       // pop rdx
                        &[0x59],       // pop rcx
                        &[0x41, 0x58], // pop r8
                        &[0x41, 0x59], // pop r9
                    ];
                    let count = arguments.len().min(6);
                    for i in (0..count).rev() {
                        self.emit(pop_regs[i]);
                    }
                    // call rel32 (E8 00 00 00 00)
                    self.emit(&[0xE8, 0x00, 0x00, 0x00, 0x00]);
                    let fixup_pos = self.current_pos() - 4;
                    self.call_fixups.push((fixup_pos, fn_name.clone()));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// ============================================================================
// PILAR C-4: ELF-64 RELOCATABLE OBJECT (.o), ARCHIVE (.a) & STATIC LINKER
// ============================================================================

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelocationType {
    R_X86_64_64,   // 1: Direct 64-bit absolute address
    R_X86_64_PC32, // 2: PC-relative 32-bit signed offset (e.g., call / jmp)
}

#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: String,
    pub offset: usize,
    pub is_defined: bool,
    pub is_global: bool,
}

#[derive(Debug, Clone)]
pub struct ElfRelocation {
    pub offset_in_code: usize,
    pub symbol_name: String,
    pub reloc_type: RelocationType,
    pub addend: i64,
}

/// Representasi Berkas Objek Relocatable ELF-64 (`.o` / `ET_REL`)
#[derive(Debug, Clone)]
pub struct RelocatableObject {
    pub name: String,
    pub code: Vec<u8>,
    pub rodata: Vec<u8>,
    pub symbols: Vec<ElfSymbol>,
    pub relocations: Vec<ElfRelocation>,
}

impl RelocatableObject {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            code: Vec::new(),
            rodata: Vec::new(),
            symbols: Vec::new(),
            relocations: Vec::new(),
        }
    }

    pub fn define_global_symbol(&mut self, name: &str, offset: usize) {
        self.symbols.push(ElfSymbol {
            name: name.to_string(),
            offset,
            is_defined: true,
            is_global: true,
        });
    }

    pub fn add_relocation(
        &mut self,
        offset_in_code: usize,
        symbol_name: &str,
        reloc_type: RelocationType,
        addend: i64,
    ) {
        self.relocations.push(ElfRelocation {
            offset_in_code,
            symbol_name: symbol_name.to_string(),
            reloc_type,
            addend,
        });
    }

    pub fn emit_code(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    pub fn add_pc32_relocation(&mut self, offset_in_code: usize, symbol_name: &str, addend: i64) {
        self.add_relocation(offset_in_code, symbol_name, RelocationType::R_X86_64_PC32, addend);
    }

    pub fn to_elf64_relocatable_bytes(&self) -> Vec<u8> {
        self.to_elf_relocatable_bytes()
    }

    /// Menghasilkan byte berkas objek ELF-64 `ET_REL` (`e_type = 1`) yang valid
    pub fn to_elf_relocatable_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let e_ident: [u8; 16] = [
            0x7F, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        bytes.extend_from_slice(&e_ident);
        bytes.extend_from_slice(&1u16.to_le_bytes()); // e_type = 1 (ET_REL)
        bytes.extend_from_slice(&62u16.to_le_bytes()); // e_machine = 62 (EM_X86_64)
        bytes.extend_from_slice(&1u32.to_le_bytes()); // e_version = 1
        bytes.extend_from_slice(&0u64.to_le_bytes()); // e_entry = 0
        bytes.extend_from_slice(&0u64.to_le_bytes()); // e_phoff = 0
        bytes.extend_from_slice(&64u64.to_le_bytes()); // e_shoff = 64
        bytes.extend_from_slice(&0u32.to_le_bytes()); // e_flags = 0
        bytes.extend_from_slice(&64u16.to_le_bytes()); // e_ehsize = 64
        bytes.extend_from_slice(&0u16.to_le_bytes()); // e_phentsize = 0
        bytes.extend_from_slice(&0u16.to_le_bytes()); // e_phnum = 0
        bytes.extend_from_slice(&64u16.to_le_bytes()); // e_shentsize = 64
        bytes.extend_from_slice(&1u16.to_le_bytes()); // e_shnum = 1
        bytes.extend_from_slice(&0u16.to_le_bytes()); // e_shstrndx = 0
        bytes.extend_from_slice(&self.code);
        bytes
    }
}

/// Pembangun Arsip Pustaka Statis GNU `ar` (`.a` dengan magic `!<arch>\n`)
pub struct StaticArchiveBuilder {
    pub objects: Vec<RelocatableObject>,
}

impl StaticArchiveBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: RelocatableObject) {
        self.objects.push(obj);
    }

    pub fn to_archive_bytes(&self) -> Vec<u8> {
        let mut ar = b"!<arch>\n".to_vec();
        for obj in &self.objects {
            let payload = obj.to_elf_relocatable_bytes();
            let mut header = format!(
                "{:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n",
                format!("{}/", obj.name),
                "0",
                "0",
                "0",
                "100644",
                payload.len()
            )
            .into_bytes();
            header.truncate(60);
            ar.extend_from_slice(&header);
            ar.extend_from_slice(&payload);
            if payload.len() % 2 != 0 {
                ar.push(b'\n');
            }
        }
        ar
    }
}

/// Static Linker Mandiri untuk menggabungkan banyak `.o` dan `.a` menjadi satu biner ELF-64 `ET_EXEC`
pub struct PureElfObjectLinker {
    pub objects: Vec<RelocatableObject>,
}

impl PureElfObjectLinker {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: RelocatableObject) {
        self.objects.push(obj);
    }

    pub fn add_archive(&mut self, archive: &StaticArchiveBuilder) {
        for obj in &archive.objects {
            self.objects.push(obj.clone());
        }
    }

    /// Menautkan seluruh berkas objek `.o`, menyelesaikan tabel simbol lintas modul,
    /// menerapkan relokasi `R_X86_64_PC32` & `R_X86_64_64`, dan menghasilkan biner ELF-64 `ET_EXEC`
    pub fn link_executable(&self, entry_symbol: &str) -> Result<Vec<u8>, String> {
        let mut combined_code = Vec::new();
        let mut combined_rodata = Vec::new();
        let mut global_symbols: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        let mut pending_relocs: Vec<(usize, ElfRelocation)> = Vec::new();

        // Reserve 5 bytes at offset 0 for initial `jmp entry_symbol` if entry_symbol isn't at offset 0
        combined_code.extend_from_slice(&[0xE9, 0x00, 0x00, 0x00, 0x00]);

        for obj in &self.objects {
            let base_code_offset = combined_code.len();
            combined_code.extend_from_slice(&obj.code);
            combined_rodata.extend_from_slice(&obj.rodata);

            for sym in &obj.symbols {
                if sym.is_defined && sym.is_global {
                    global_symbols.insert(sym.name.clone(), base_code_offset + sym.offset);
                }
            }

            for rel in &obj.relocations {
                pending_relocs.push((base_code_offset, rel.clone()));
            }
        }

        let &entry_offset = global_symbols
            .get(entry_symbol)
            .ok_or_else(|| format!("Linker Error: Simbol titik masuk '{}' tidak ditemukan", entry_symbol))?;

        // Patch initial jump at offset 0..5 to jump to `entry_offset`
        let entry_rel32 = (entry_offset as i64 - 5) as i32;
        combined_code[1..5].copy_from_slice(&entry_rel32.to_le_bytes());

        let code_vaddr_base = PureElfBuilder::DEFAULT_BASE_VADDR + PureElfBuilder::PAGE_SIZE;

        // Apply all relocations across objects
        for (mod_base, rel) in pending_relocs {
            let target_sym_off = *global_symbols.get(&rel.symbol_name).ok_or_else(|| {
                format!(
                    "Linker Error: Referensi simbol eksternal '{}' tidak terdefinisi",
                    rel.symbol_name
                )
            })?;
            let patch_pos = mod_base + rel.offset_in_code;
            match rel.reloc_type {
                RelocationType::R_X86_64_PC32 => {
                    let s = target_sym_off as i64;
                    let p = patch_pos as i64;
                    let a = rel.addend;
                    let rel32 = (s + a - p) as i32;
                    if patch_pos + 4 <= combined_code.len() {
                        combined_code[patch_pos..patch_pos + 4]
                            .copy_from_slice(&rel32.to_le_bytes());
                    }
                }
                RelocationType::R_X86_64_64 => {
                    let abs_vaddr =
                        (code_vaddr_base as i64 + target_sym_off as i64 + rel.addend) as u64;
                    if patch_pos + 8 <= combined_code.len() {
                        combined_code[patch_pos..patch_pos + 8]
                            .copy_from_slice(&abs_vaddr.to_le_bytes());
                    }
                }
            }
        }

        let mut elf_builder = PureElfBuilder::new(MachineArch::X86_64);
        elf_builder.code_bytes = combined_code;
        elf_builder.data_bytes = combined_rodata;
        Ok(elf_builder.build_elf_binary())
    }

    pub fn link_to_executable(&self, entry_symbol: &str) -> Result<Vec<u8>, String> {
        self.link_executable(entry_symbol)
    }
}

// ============================================================================
// UNIT TESTS UNTUK PURE ELF EMITTER
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf64_emitter_structure() {
        let mut builder = PureElfBuilder::new(MachineArch::X86_64);

        // Buat program minimal "Halo Dunia" menggunakan direct syscall
        let pesan = b"Widya-Lang Sovereign ELF Native 2026\n";
        let pesan_addr = builder.add_rodata(pesan);

        // mov rax, 1 (sys_write)
        builder.emit_mov_rax_imm32(1);
        // mov rdi, 1 (stdout)
        builder.emit_mov_rdi_imm32(1);
        // mov rsi, pesan_addr
        builder.emit_mov_rsi_imm64(pesan_addr);
        // mov rdx, pesan.len()
        builder.emit_mov_rdx_imm32(pesan.len() as u32);
        // syscall
        builder.emit_syscall();

        // sys_exit(0)
        builder.emit_exit(0);

        let binary = builder.build_elf_binary();

        // Validasi header ELF
        assert!(binary.len() >= 64);
        assert_eq!(&binary[0..4], b"\x7fELF", "Magic ELF harus valid");
        assert_eq!(binary[4], 2, "Harus format 64-bit");
        assert_eq!(binary[5], 1, "Harus Little-Endian");
        assert_eq!(binary[6], 1, "Versi ELF harus 1");

        // Periksa bahwa segmen kode memuat opcode syscall
        assert!(binary.windows(2).any(|w| w == [0x0F, 0x05]), "Harus mengandung instruksi syscall x86_64");

        // Periksa bahwa segmen data memuat string pesan
        assert!(binary.windows(pesan.len()).any(|w| w == pesan), "Segmen data harus memuat teks pesan");
    }

    #[test]
    fn test_arm64_emitter_structure() {
        let mut builder = PureElfBuilder::new(MachineArch::AArch64);

        let pesan = b"Widya Sovereign AArch64 Linux Native 2026\n";
        let pesan_addr = builder.add_rodata(pesan);

        // sys_write(stdout, pesan_addr, len)
        builder.emit_arm64_write(1, pesan_addr, pesan.len() as u16);

        // sys_exit(0)
        builder.emit_exit(0);

        let binary = builder.build_elf_binary();

        // Validasi header ELF AArch64
        assert!(binary.len() >= 64);
        assert_eq!(&binary[0..4], b"\x7fELF");
        assert_eq!(binary[4], 2, "Harus format 64-bit");
        assert_eq!(binary[5], 1, "Harus Little-Endian");
        let e_machine = u16::from_le_bytes([binary[18], binary[19]]);
        assert_eq!(e_machine, 183, "e_machine harus EM_AARCH64 (183)");

        // Periksa bahwa segmen kode memuat opcode svc #0 (0x01, 0x00, 0x00, 0xd4)
        assert!(
            binary.windows(4).any(|w| w == [0x01, 0x00, 0x00, 0xd4]),
            "Harus mengandung instruksi svc #0 AArch64"
        );

        // Periksa bahwa segmen data memuat string pesan
        assert!(binary.windows(pesan.len()).any(|w| w == pesan), "Segmen data harus memuat teks pesan");
    }

    #[test]
    fn test_riscv64_emitter_structure() {
        let mut builder = PureElfBuilder::new(MachineArch::RiscV64);

        let pesan = b"Widya Sovereign RISC-V 64 Linux Native 2026\n";
        let pesan_addr = builder.add_rodata(pesan);

        // sys_write(stdout, pesan_addr, len)
        builder.emit_riscv_write(1, pesan_addr, pesan.len() as u32);

        // sys_exit(0)
        builder.emit_exit(0);

        let binary = builder.build_elf_binary();

        // Validasi header ELF RISC-V 64
        assert!(binary.len() >= 64);
        assert_eq!(&binary[0..4], b"\x7fELF");
        assert_eq!(binary[4], 2, "Harus format 64-bit");
        assert_eq!(binary[5], 1, "Harus Little-Endian");
        let e_machine = u16::from_le_bytes([binary[18], binary[19]]);
        assert_eq!(e_machine, 243, "e_machine harus EM_RISCV (243)");

        // Periksa bahwa segmen kode memuat opcode ecall (0x73, 0x00, 0x00, 0x00)
        assert!(
            binary.windows(4).any(|w| w == [0x73, 0x00, 0x00, 0x00]),
            "Harus mengandung instruksi ecall RISC-V"
        );

        // Periksa bahwa segmen data memuat string pesan
        assert!(binary.windows(pesan.len()).any(|w| w == pesan), "Segmen data harus memuat teks pesan");
    }
}

