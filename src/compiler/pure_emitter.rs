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
                // Future RISC-V implementation
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
}
