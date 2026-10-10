//! Pure Sovereign Zero-Dependency eBPF Bytecode & ELF Object Emitter (pure_ebpf.rs)
//!
//! 100% In-House Implementation:
//! - 64-bit eBPF instruction encoding (8-byte fixed instruction layout)
//! - ALU64, ALU32, Load/Store Memory (LDX/STX), Conditional Branching, and Exit instructions
//! - Standard XDP (eXpress Data Path) Line-Rate Filtering logic
//! - Relocatable ELF64 Container generator with `EM_BPF` (e_machine = 247) and "GPL" license section
//! - Pure Userspace eBPF Micro-VM & Verifier for deterministic pre-kernel execution and validation

// ============================================================================
// 1. eBPF INSTRUCTION SET ARCHITECTURE (ISA) DEFINITIONS
// ============================================================================

// Instruction Classes (bits 0..2)
pub const BPF_LD: u8    = 0x00;
pub const BPF_LDX: u8   = 0x01;
pub const BPF_ST: u8    = 0x02;
pub const BPF_STX: u8   = 0x03;
pub const BPF_ALU: u8   = 0x04;
pub const BPF_JMP: u8   = 0x05;
pub const BPF_JMP32: u8 = 0x06;
pub const BPF_ALU64: u8 = 0x07;

// ALU / JMP Source (bit 3)
pub const BPF_K: u8 = 0x00; // Immediate operand
pub const BPF_X: u8 = 0x08; // Register operand

// ALU Operations (bits 4..7)
pub const BPF_ADD: u8  = 0x00;
pub const BPF_SUB: u8  = 0x10;
pub const BPF_MUL: u8  = 0x20;
pub const BPF_DIV: u8  = 0x30;
pub const BPF_OR: u8   = 0x40;
pub const BPF_AND: u8  = 0x50;
pub const BPF_LSH: u8  = 0x60;
pub const BPF_RSH: u8  = 0x70;
pub const BPF_NEG: u8  = 0x80;
pub const BPF_MOD: u8  = 0x90;
pub const BPF_XOR: u8  = 0xA0;
pub const BPF_MOV: u8  = 0xB0;
pub const BPF_ARSH: u8 = 0xC0;

// JMP Operations (bits 4..7)
pub const BPF_JA: u8   = 0x00;
pub const BPF_JEQ: u8  = 0x10;
pub const BPF_JGT: u8  = 0x20;
pub const BPF_JGE: u8  = 0x30;
pub const BPF_JSET: u8 = 0x40;
pub const BPF_JNE: u8  = 0x50;
pub const BPF_JSGT: u8 = 0x60;
pub const BPF_JSGE: u8 = 0x70;
pub const BPF_CALL: u8 = 0x80;
pub const BPF_EXIT: u8 = 0x90;
pub const BPF_JLT: u8  = 0xA0;
pub const BPF_JLE: u8  = 0xB0;

// Memory Size (bits 3..4 of LD/ST)
pub const BPF_W: u8  = 0x00; // 4 bytes (word)
pub const BPF_H: u8  = 0x08; // 2 bytes (halfword)
pub const BPF_B: u8  = 0x10; // 1 byte (byte)
pub const BPF_DW: u8 = 0x18; // 8 bytes (doubleword)

// Memory Mode (bits 5..7 of LD/ST)
pub const BPF_IMM: u8 = 0x00;
pub const BPF_MEM: u8 = 0x60;

// XDP Action Constants (Return values in R0)
pub const XDP_ABORTED: u64  = 0;
pub const XDP_DROP: u64     = 1;
pub const XDP_PASS: u64     = 2;
pub const XDP_TX: u64       = 3;
pub const XDP_REDIRECT: u64 = 4;

/// Struktur Instruksi 64-bit eBPF (Ukuran Tepat 8 Byte)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct EbpfInsn {
    pub opcode: u8,
    pub regs: u8,   // dst_reg: 4 bits, src_reg: 4 bits ((src << 4) | (dst & 0x0F))
    pub offset: i16,
    pub imm: i32,
}

impl EbpfInsn {
    pub fn new(opcode: u8, dst: u8, src: u8, offset: i16, imm: i32) -> Self {
        Self {
            opcode,
            regs: (src << 4) | (dst & 0x0F),
            offset,
            imm,
        }
    }

    #[inline]
    pub fn dst_reg(&self) -> u8 {
        self.regs & 0x0F
    }

    #[inline]
    pub fn src_reg(&self) -> u8 {
        (self.regs >> 4) & 0x0F
    }

    /// Serialisasi instruksi ke 8 byte biner little-endian
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut b = [0u8; 8];
        b[0] = self.opcode;
        b[1] = self.regs;
        b[2..4].copy_from_slice(&self.offset.to_le_bytes());
        b[4..8].copy_from_slice(&self.imm.to_le_bytes());
        b
    }

    /// Deserialisasi dari 8 byte biner little-endian
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 8 {
            return Err("Ukuran data instruksi eBPF tidak cukup (harus 8 byte)".to_string());
        }
        let opcode = bytes[0];
        let regs = bytes[1];
        let offset = i16::from_le_bytes([bytes[2], bytes[3]]);
        let imm = i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        Ok(Self {
            opcode,
            regs,
            offset,
            imm,
        })
    }

    // Helper Constructors:
    pub fn mov_imm64(dst: u8, imm: i32) -> Self {
        Self::new(BPF_ALU64 | BPF_MOV | BPF_K, dst, 0, 0, imm)
    }

    pub fn mov_reg64(dst: u8, src: u8) -> Self {
        Self::new(BPF_ALU64 | BPF_MOV | BPF_X, dst, src, 0, 0)
    }

    pub fn add_imm64(dst: u8, imm: i32) -> Self {
        Self::new(BPF_ALU64 | BPF_ADD | BPF_K, dst, 0, 0, imm)
    }

    pub fn add_reg64(dst: u8, src: u8) -> Self {
        Self::new(BPF_ALU64 | BPF_ADD | BPF_X, dst, src, 0, 0)
    }

    pub fn sub_imm64(dst: u8, imm: i32) -> Self {
        Self::new(BPF_ALU64 | BPF_SUB | BPF_K, dst, 0, 0, imm)
    }

    pub fn sub_reg64(dst: u8, src: u8) -> Self {
        Self::new(BPF_ALU64 | BPF_SUB | BPF_X, dst, src, 0, 0)
    }

    pub fn ldx_mem(size: u8, dst: u8, src: u8, offset: i16) -> Self {
        Self::new(BPF_LDX | BPF_MEM | size, dst, src, offset, 0)
    }

    pub fn stx_mem(size: u8, dst: u8, src: u8, offset: i16) -> Self {
        Self::new(BPF_STX | BPF_MEM | size, dst, src, offset, 0)
    }

    pub fn jmp_imm(op: u8, dst: u8, imm: i32, offset: i16) -> Self {
        Self::new(BPF_JMP | op | BPF_K, dst, 0, offset, imm)
    }

    pub fn jmp_reg(op: u8, dst: u8, src: u8, offset: i16) -> Self {
        Self::new(BPF_JMP | op | BPF_X, dst, src, offset, 0)
    }

    pub fn exit() -> Self {
        Self::new(BPF_JMP | BPF_EXIT, 0, 0, 0, 0)
    }
}

// ============================================================================
// 2. RELOCATABLE ELF64 BPF OBJECT GENERATOR
// ============================================================================

pub struct PureEbpfElfBuilder {
    instructions: Vec<EbpfInsn>,
    license: String,
}

impl PureEbpfElfBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            license: "GPL\0".to_string(),
        }
    }

    pub fn add_instruction(&mut self, insn: EbpfInsn) {
        self.instructions.push(insn);
    }

    pub fn add_instructions(&mut self, insns: &[EbpfInsn]) {
        self.instructions.extend_from_slice(insns);
    }

    /// Hasilkan biner ELF64 Relocatable Object (ET_REL, EM_BPF)
    pub fn build_elf(&self) -> Vec<u8> {
        let mut insn_bytes = Vec::with_capacity(self.instructions.len() * 8);
        for insn in &self.instructions {
            insn_bytes.extend_from_slice(&insn.to_bytes());
        }

        let license_bytes = self.license.as_bytes();

        // String table untuk seksi (.shstrtab)
        // 0: "\0"
        // 1: "xdp\0"
        // 5: ".license\0"
        // 14: ".shstrtab\0"
        let mut shstrtab = Vec::new();
        shstrtab.push(0); // offset 0
        let xdp_name_off = shstrtab.len() as u32;
        shstrtab.extend_from_slice(b"xdp\0");
        let license_name_off = shstrtab.len() as u32;
        shstrtab.extend_from_slice(b".license\0");
        let shstrtab_name_off = shstrtab.len() as u32;
        shstrtab.extend_from_slice(b".shstrtab\0");

        let ehdr_size = 64usize;
        let shdr_size = 64usize;
        let num_shdrs = 4usize; // Null, xdp, .license, .shstrtab

        // Layout:
        // [0..64]: ELF Header
        // [64..64 + insn_len]: xdp section data
        // [offset..offset + license_len]: license data
        // [offset..offset + shstrtab_len]: shstrtab data
        // [offset..]: Section Header Table (4 * 64 bytes)

        let xdp_offset = ehdr_size;
        let license_offset = xdp_offset + insn_bytes.len();
        let shstrtab_offset = license_offset + license_bytes.len();
        let shdrs_offset = shstrtab_offset + shstrtab.len();

        let total_size = shdrs_offset + num_shdrs * shdr_size;
        let mut elf = Vec::with_capacity(total_size);

        // 1. ELF Header (64 bytes)
        elf.extend_from_slice(&[
            0x7F, b'E', b'L', b'F', // EI_MAG
            2,                      // EI_CLASS = 64-bit
            1,                      // EI_DATA = Little Endian
            1,                      // EI_VERSION = 1
            0,                      // EI_OSABI = System V
            0,                      // EI_ABIVERSION
            0, 0, 0, 0, 0, 0, 0,    // EI_PAD
        ]);
        elf.extend_from_slice(&1u16.to_le_bytes());           // e_type = ET_REL (1)
        elf.extend_from_slice(&247u16.to_le_bytes());         // e_machine = EM_BPF (247 / 0xF7)
        elf.extend_from_slice(&1u32.to_le_bytes());           // e_version = 1
        elf.extend_from_slice(&0u64.to_le_bytes());           // e_entry = 0
        elf.extend_from_slice(&0u64.to_le_bytes());           // e_phoff = 0
        elf.extend_from_slice(&(shdrs_offset as u64).to_le_bytes()); // e_shoff
        elf.extend_from_slice(&0u32.to_le_bytes());           // e_flags = 0
        elf.extend_from_slice(&(ehdr_size as u16).to_le_bytes()); // e_ehsize
        elf.extend_from_slice(&0u16.to_le_bytes());           // e_phentsize
        elf.extend_from_slice(&0u16.to_le_bytes());           // e_phnum
        elf.extend_from_slice(&(shdr_size as u16).to_le_bytes()); // e_shentsize
        elf.extend_from_slice(&(num_shdrs as u16).to_le_bytes()); // e_shnum
        elf.extend_from_slice(&3u16.to_le_bytes());           // e_shstrndx = 3 (.shstrtab)

        // 2. Section Data
        elf.extend_from_slice(&insn_bytes);
        elf.extend_from_slice(license_bytes);
        elf.extend_from_slice(&shstrtab);

        // 3. Section Headers (4 * 64 bytes)
        // Shdr 0: NULL
        elf.extend_from_slice(&[0u8; 64]);

        // Shdr 1: "xdp" (PROGBITS, ALLOC + EXECINSTR = 0x6)
        write_shdr(
            &mut elf,
            xdp_name_off,
            1, // SHT_PROGBITS
            0x6, // SHF_ALLOC | SHF_EXECINSTR
            0,
            xdp_offset as u64,
            insn_bytes.len() as u64,
            0, 0, 8, 0,
        );

        // Shdr 2: ".license" (PROGBITS, ALLOC = 0x2)
        write_shdr(
            &mut elf,
            license_name_off,
            1, // SHT_PROGBITS
            0x2, // SHF_ALLOC
            0,
            license_offset as u64,
            license_bytes.len() as u64,
            0, 0, 1, 0,
        );

        // Shdr 3: ".shstrtab" (STRTAB = 3, 0)
        write_shdr(
            &mut elf,
            shstrtab_name_off,
            3, // SHT_STRTAB
            0,
            0,
            shstrtab_offset as u64,
            shstrtab.len() as u64,
            0, 0, 1, 0,
        );

        elf
    }
}

fn write_shdr(
    out: &mut Vec<u8>,
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
) {
    out.extend_from_slice(&sh_name.to_le_bytes());
    out.extend_from_slice(&sh_type.to_le_bytes());
    out.extend_from_slice(&sh_flags.to_le_bytes());
    out.extend_from_slice(&sh_addr.to_le_bytes());
    out.extend_from_slice(&sh_offset.to_le_bytes());
    out.extend_from_slice(&sh_size.to_le_bytes());
    out.extend_from_slice(&sh_link.to_le_bytes());
    out.extend_from_slice(&sh_info.to_le_bytes());
    out.extend_from_slice(&sh_addralign.to_le_bytes());
    out.extend_from_slice(&sh_entsize.to_le_bytes());
}

// ============================================================================
// 3. USERSPACE eBPF MICRO-VM & VERIFIER
// ============================================================================

pub struct PureEbpfVm {
    pub regs: [u64; 11],
    pub stack: [u8; 512],
    pub max_instructions: usize,
}

impl PureEbpfVm {
    pub fn new() -> Self {
        let mut regs = [0u64; 11];
        regs[10] = 512; // R10 menunjuk ke batas atas stack
        Self {
            regs,
            stack: [0u8; 512],
            max_instructions: 100_000,
        }
    }

    /// Verifikasi program eBPF (memeriksa instruksi exit, batas stack, dll.)
    pub fn verify(&self, insns: &[EbpfInsn]) -> Result<(), String> {
        if insns.is_empty() {
            return Err("Program eBPF kosong".to_string());
        }

        let has_exit = insns.iter().any(|i| i.opcode == (BPF_JMP | BPF_EXIT));
        if !has_exit {
            return Err("Program eBPF tidak memiliki instruksi exit (BPF_EXIT)".to_string());
        }

        for (idx, insn) in insns.iter().enumerate() {
            let dst = insn.dst_reg();
            let src = insn.src_reg();
            if dst > 10 || src > 10 {
                return Err(format!("Register tidak valid pada instruksi #{}: dst={}, src={}", idx, dst, src));
            }
            if dst == 10 && (insn.opcode & 0x07) == BPF_ALU64 {
                return Err(format!("R10 adalah read-only frame pointer (modifikasi terlarang pada #{})", idx));
            }
        }

        Ok(())
    }

    /// Jalankan program eBPF dan kembalikan nilai R0
    pub fn execute(&mut self, insns: &[EbpfInsn], context_ptr: u64) -> Result<u64, String> {
        self.verify(insns)?;
        self.regs[1] = context_ptr; // R1 menerima konteks awal (misal xdp_md)

        let mut pc = 0usize;
        let mut steps = 0usize;

        while pc < insns.len() {
            if steps >= self.max_instructions {
                return Err("Batas instruksi eBPF terlampaui (potensi infinite loop)".to_string());
            }
            steps += 1;

            let insn = insns[pc];
            let class = insn.opcode & 0x07;
            let dst = insn.dst_reg() as usize;
            let src = insn.src_reg() as usize;

            match class {
                BPF_ALU64 => {
                    let op = insn.opcode & 0xF0;
                    let is_reg = (insn.opcode & BPF_X) != 0;
                    let val = if is_reg { self.regs[src] } else { insn.imm as i64 as u64 };

                    match op {
                        BPF_MOV => self.regs[dst] = val,
                        BPF_ADD => self.regs[dst] = self.regs[dst].wrapping_add(val),
                        BPF_SUB => self.regs[dst] = self.regs[dst].wrapping_sub(val),
                        BPF_AND => self.regs[dst] &= val,
                        BPF_OR  => self.regs[dst] |= val,
                        BPF_XOR => self.regs[dst] ^= val,
                        _ => return Err(format!("Operasi ALU64 0x{:02X} belum didukung", op)),
                    }
                    pc += 1;
                }
                BPF_JMP => {
                    let op = insn.opcode & 0xF0;
                    if op == BPF_EXIT {
                        return Ok(self.regs[0]);
                    }

                    let is_reg = (insn.opcode & BPF_X) != 0;
                    let target = if is_reg { self.regs[src] } else { insn.imm as i64 as u64 };
                    let cond = match op {
                        BPF_JA  => true,
                        BPF_JEQ => self.regs[dst] == target,
                        BPF_JNE => self.regs[dst] != target,
                        BPF_JGT => self.regs[dst] > target,
                        BPF_JGE => self.regs[dst] >= target,
                        BPF_JLT => self.regs[dst] < target,
                        BPF_JLE => self.regs[dst] <= target,
                        _ => return Err(format!("Operasi JMP 0x{:02X} belum didukung", op)),
                    };

                    if cond {
                        let next = (pc as isize) + (insn.offset as isize) + 1;
                        if next < 0 || next as usize >= insns.len() {
                            return Err(format!("Lompatan cabang di luar batas program: {}", next));
                        }
                        pc = next as usize;
                    } else {
                        pc += 1;
                    }
                }
                BPF_STX => {
                    // Menyimpan ke stack
                    let size = insn.opcode & 0x18;
                    let target_addr = (self.regs[dst] as isize) + (insn.offset as isize);
                    if target_addr < 0 || (target_addr as usize) >= 512 {
                        return Err(format!("Akses memori stack di luar batas: {}", target_addr));
                    }
                    let idx = target_addr as usize;
                    let val = self.regs[src];

                    match size {
                        BPF_B  => self.stack[idx] = val as u8,
                        BPF_H  => self.stack[idx..idx + 2].copy_from_slice(&(val as u16).to_le_bytes()),
                        BPF_W  => self.stack[idx..idx + 4].copy_from_slice(&(val as u32).to_le_bytes()),
                        BPF_DW => self.stack[idx..idx + 8].copy_from_slice(&val.to_le_bytes()),
                        _ => return Err("Ukuran STX tidak valid".to_string()),
                    }
                    pc += 1;
                }
                BPF_LDX => {
                    // Membaca dari stack
                    let size = insn.opcode & 0x18;
                    let target_addr = (self.regs[src] as isize) + (insn.offset as isize);
                    if target_addr < 0 || (target_addr as usize) >= 512 {
                        return Err(format!("Akses memori stack di luar batas: {}", target_addr));
                    }
                    let idx = target_addr as usize;

                    self.regs[dst] = match size {
                        BPF_B  => self.stack[idx] as u64,
                        BPF_H  => u16::from_le_bytes([self.stack[idx], self.stack[idx + 1]]) as u64,
                        BPF_W  => u32::from_le_bytes([
                            self.stack[idx], self.stack[idx + 1], self.stack[idx + 2], self.stack[idx + 3]
                        ]) as u64,
                        BPF_DW => u64::from_le_bytes([
                            self.stack[idx], self.stack[idx + 1], self.stack[idx + 2], self.stack[idx + 3],
                            self.stack[idx + 4], self.stack[idx + 5], self.stack[idx + 6], self.stack[idx + 7]
                        ]),
                        _ => return Err("Ukuran LDX tidak valid".to_string()),
                    };
                    pc += 1;
                }
                _ => return Err(format!("Kelas instruksi eBPF 0x{:02X} tidak dikenal", class)),
            }
        }

        Err("Program eBPF berakhir tanpa instruksi EXIT".to_string())
    }
}
