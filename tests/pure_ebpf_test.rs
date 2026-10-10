use widya::compiler::pure_ebpf::{
    EbpfInsn, PureEbpfElfBuilder, PureEbpfVm, BPF_B, BPF_JEQ, BPF_JLT, XDP_DROP, XDP_PASS,
};

#[test]
fn test_ebpf_instruction_serialization_roundtrip() {
    let insn = EbpfInsn::mov_imm64(0, 1337);
    assert_eq!(insn.dst_reg(), 0);
    assert_eq!(insn.src_reg(), 0);
    assert_eq!(insn.imm, 1337);

    let bytes = insn.to_bytes();
    assert_eq!(bytes.len(), 8);

    let decoded = EbpfInsn::from_bytes(&bytes).expect("Failed to decode instruction");
    assert_eq!(decoded.opcode, insn.opcode);
    assert_eq!(decoded.regs, insn.regs);
    assert_eq!(decoded.imm, 1337);
}

#[test]
fn test_pure_ebpf_elf64_object_container() {
    let mut builder = PureEbpfElfBuilder::new();
    // Program: mov r0, XDP_PASS (2); exit
    builder.add_instruction(EbpfInsn::mov_imm64(0, XDP_PASS as i32));
    builder.add_instruction(EbpfInsn::exit());

    let elf_bytes = builder.build_elf();

    // 1. Check ELF Magic
    assert!(elf_bytes.len() >= 64);
    assert_eq!(&elf_bytes[0..4], &[0x7F, b'E', b'L', b'F']);
    assert_eq!(elf_bytes[4], 2); // 64-bit
    assert_eq!(elf_bytes[5], 1); // Little Endian

    // 2. Check e_type (ET_REL = 1) and e_machine (EM_BPF = 247)
    let e_type = u16::from_le_bytes([elf_bytes[16], elf_bytes[17]]);
    let e_machine = u16::from_le_bytes([elf_bytes[18], elf_bytes[19]]);
    assert_eq!(e_type, 1);
    assert_eq!(e_machine, 247); // EM_BPF

    // 3. Check section contents embedded
    let content_str = String::from_utf8_lossy(&elf_bytes);
    assert!(content_str.contains("GPL"));
    assert!(content_str.contains("xdp"));
    assert!(content_str.contains(".shstrtab"));
}

#[test]
fn test_userspace_ebpf_vm_arithmetic_and_branching() {
    let mut vm = PureEbpfVm::new();

    // Program:
    // r1 = 10
    // r2 = 25
    // r1 = r1 + r2 (35)
    // if r1 < 50 goto label (+1 insn)
    // r0 = 0
    // exit
    // label:
    // r0 = r1 (35)
    // exit
    let program = vec![
        EbpfInsn::mov_imm64(1, 10),
        EbpfInsn::mov_imm64(2, 25),
        EbpfInsn::add_reg64(1, 2),
        EbpfInsn::jmp_imm(BPF_JLT, 1, 50, 2), // skip 2 instructions to label
        EbpfInsn::mov_imm64(0, 0),
        EbpfInsn::exit(),
        // label:
        EbpfInsn::mov_reg64(0, 1),
        EbpfInsn::exit(),
    ];

    let result = vm.execute(&program, 0).expect("VM execution failed");
    assert_eq!(result, 35);
}

#[test]
fn test_userspace_ebpf_xdp_packet_inspection_simulation() {
    let mut vm = PureEbpfVm::new();

    // Simulate writing a packet IP protocol to stack (e.g., protocol = 6 for TCP)
    // R10 is frame pointer (512). We store byte 6 at R10 - 8
    let offset: i16 = -8;

    // eBPF Program:
    // r1 = 6 (TCP protocol)
    // stx.b [r10 - 8], r1
    // r2 = ldx.b [r10 - 8]
    // if r2 == 6 goto drop_packet (+1 insn)
    // r0 = XDP_PASS
    // exit
    // drop_packet:
    // r0 = XDP_DROP
    // exit
    let program = vec![
        EbpfInsn::mov_imm64(1, 6),
        EbpfInsn::stx_mem(BPF_B, 10, 1, offset),
        EbpfInsn::ldx_mem(BPF_B, 2, 10, offset),
        EbpfInsn::jmp_imm(BPF_JEQ, 2, 6, 2), // jump to drop_packet
        EbpfInsn::mov_imm64(0, XDP_PASS as i32),
        EbpfInsn::exit(),
        // drop_packet:
        EbpfInsn::mov_imm64(0, XDP_DROP as i32),
        EbpfInsn::exit(),
    ];

    let action = vm.execute(&program, 0).expect("XDP filter failed");
    assert_eq!(action, XDP_DROP);
}

#[test]
fn test_userspace_ebpf_verifier_rejection() {
    let vm = PureEbpfVm::new();

    // Program without BPF_EXIT should be rejected by the verifier
    let bad_program = vec![
        EbpfInsn::mov_imm64(0, 42),
    ];
    assert!(vm.verify(&bad_program).is_err());

    // Program trying to modify read-only R10 should be rejected
    let bad_r10_program = vec![
        EbpfInsn::mov_imm64(10, 42),
        EbpfInsn::exit(),
    ];
    assert!(vm.verify(&bad_r10_program).is_err());
}
