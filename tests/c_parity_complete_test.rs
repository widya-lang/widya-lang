use widya::compiler::pure_emitter::{
    PureElfBuilder, PureElfObjectLinker, RelocatableObject, StaticArchiveBuilder,
};
use widya::ffi::CHeaderParser;
use widya::interpreter::Interpreter;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::value::Value;

fn eval_widya(src: &str) -> Interpreter {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Interpreter gagal");
    interp
}

fn get_num(interp: &Interpreter, name: &str) -> f64 {
    let globals = interp.get_global_variables();
    let val = globals
        .get(name)
        .unwrap_or_else(|| panic!("Variabel '{}' tidak ditemukan", name));
    match val {
        Value::Number(n) => *n,
        other => panic!("Variabel '{}' bukan angka: {:?}", name, other),
    }
}

#[test]
fn test_pilar_c1_bitwise_literals_and_fixed_width_casts() {
    let code = r#"
        // 1. Hex, Binary, Octal literals dengan pemisah digit '_'
        tetapkan hex_val = 0xFF_00;
        tetapkan bin_val = 0b1010_0101;
        tetapkan oct_val = 0o755;

        // 2. Operator Bitwise penuh: &, |, ^^, <<, >>, ~
        tetapkan b_and = 0xFF & 0x0F;
        tetapkan b_or  = 0xF0 | 0x0F;
        tetapkan b_xor = 0xFF ^^ 0xAA;
        tetapkan b_shl = 1 << 8;
        tetapkan b_shr = 1024 >> 3;
        tetapkan b_not = ~0;

        // 3. Konversi Integer Lebar-Tetap (Two's-Complement Wrapping seperti C)
        tetapkan u8_wrap  = ke_u8(260);      // 260 & 0xFF = 4
        tetapkan i8_wrap  = ke_i8(255);      // 0xFF sebagai int8_t = -1
        tetapkan u16_wrap = ke_u16(65537);   // 65537 & 0xFFFF = 1
        tetapkan i16_wrap = ke_i16(65535);   // 0xFFFF sebagai int16_t = -1
        tetapkan u32_wrap = ke_u32(-1);      // 0xFFFFFFFF = 4294967295
        tetapkan i32_wrap = ke_i32(0x80000000); // INT32_MIN = -2147483648
    "#;

    let interp = eval_widya(code);
    assert_eq!(get_num(&interp, "hex_val"), 0xFF00 as f64);
    assert_eq!(get_num(&interp, "bin_val"), 0b10100101 as f64);
    assert_eq!(get_num(&interp, "oct_val"), 0o755 as f64);

    assert_eq!(get_num(&interp, "b_and"), 0x0F as f64);
    assert_eq!(get_num(&interp, "b_or"), 0xFF as f64);
    assert_eq!(get_num(&interp, "b_xor"), (0xFF ^ 0xAA) as f64);
    assert_eq!(get_num(&interp, "b_shl"), 256.0);
    assert_eq!(get_num(&interp, "b_shr"), 128.0);
    assert_eq!(get_num(&interp, "b_not"), -1.0);

    assert_eq!(get_num(&interp, "u8_wrap"), 4.0);
    assert_eq!(get_num(&interp, "i8_wrap"), -1.0);
    assert_eq!(get_num(&interp, "u16_wrap"), 1.0);
    assert_eq!(get_num(&interp, "i16_wrap"), -1.0);
    assert_eq!(get_num(&interp, "u32_wrap"), 4294967295.0);
    assert_eq!(get_num(&interp, "i32_wrap"), -2147483648.0);
}

#[test]
fn test_pilar_c1_c_abi_struct_layout_packed_bitfields_and_union_type_punning() {
    let code = r#"
        // 1. Struct C standar dengan padding & alignment (System V AMD64 ABI)
        // a: u8 (offset 0, size 1) + 3 byte padding -> b: u32 (offset 4, size 4) -> c: u16 (offset 8, size 2) + 2 byte tail padding = 12 byte, align 4
        struktur PaketStandar {
            a: u8,
            b: u32,
            c: u16
        }

        // 2. Struct @packed tanpa padding (1 + 4 + 2 = 7 byte, align 1)
        @packed
        struktur PaketPadat {
            a: u8,
            b: u32,
            c: u16
        }

        // 3. Struct dengan C Bitfields & Fixed-Width Field Wrapping
        struktur RegKontrol {
            aktif: u8:1,
            mode: u8:3,
            prioritas: u8:4,
            counter: u8
        }

        // 4. Union C dengan berbagi memori pada offset 0 (IEEE-754 Type Punning u32 <-> f32)
        union KonversiFloat {
            i: u32,
            f: f32
        }

        tetapkan sz_standar = sizeof("PaketStandar");
        tetapkan al_standar = alignof("PaketStandar");
        tetapkan off_b_std  = offsetof("PaketStandar", "b");
        tetapkan off_c_std  = offsetof("PaketStandar", "c");

        tetapkan sz_padat   = sizeof("PaketPadat");
        tetapkan al_padat   = alignof("PaketPadat");
        tetapkan off_b_pdt  = offsetof("PaketPadat", "b");
        tetapkan off_c_pdt  = offsetof("PaketPadat", "c");

        // Uji wrapping otomatis pada field struct & bitfield
        buat reg = RegKontrol {
            aktif: 3,       // 1-bit bitfield: 3 & 0b1 = 1
            mode: 13,       // 3-bit bitfield: 13 & 0b111 = 5
            prioritas: 31,  // 4-bit bitfield: 31 & 0b1111 = 15
            counter: 260    // u8 field: 260 & 0xFF = 4
        };
        tetapkan reg_aktif = reg.aktif;
        tetapkan reg_mode = reg.mode;
        tetapkan reg_prio = reg.prioritas;
        tetapkan reg_cnt  = reg.counter;

        // Uji Union Type-Punning: tulis 0x3F800000 ke u.i, baca dari u.f harus 1.0 (IEEE-754)
        buat u = KonversiFloat();
        u.i = 0x3F800000;
        tetapkan nilai_float_1 = u.f;

        // Tulis 2.5 ke u.f, baca representasi bit IEEE-754 dari u.i (0x40200000)
        u.f = 2.5;
        tetapkan bit_ieee_2_5 = u.i;
        tetapkan sz_union = sizeof(u);
    "#;

    let interp = eval_widya(code);
    assert_eq!(get_num(&interp, "sz_standar"), 12.0);
    assert_eq!(get_num(&interp, "al_standar"), 4.0);
    assert_eq!(get_num(&interp, "off_b_std"), 4.0);
    assert_eq!(get_num(&interp, "off_c_std"), 8.0);

    assert_eq!(get_num(&interp, "sz_padat"), 7.0);
    assert_eq!(get_num(&interp, "al_padat"), 1.0);
    assert_eq!(get_num(&interp, "off_b_pdt"), 1.0);
    assert_eq!(get_num(&interp, "off_c_pdt"), 5.0);

    assert_eq!(get_num(&interp, "reg_aktif"), 1.0);
    assert_eq!(get_num(&interp, "reg_mode"), 5.0);
    assert_eq!(get_num(&interp, "reg_prio"), 15.0);
    assert_eq!(get_num(&interp, "reg_cnt"), 4.0);

    assert_eq!(get_num(&interp, "nilai_float_1"), 1.0);
    assert_eq!(get_num(&interp, "bit_ieee_2_5"), 0x40200000 as f64);
    assert_eq!(get_num(&interp, "sz_union"), 4.0);
}

#[test]
fn test_pilar_c2_unsafe_pointers_malloc_mmio_and_inline_asm() {
    let code = r#"
        buat hasil_deref_0 = 0;
        buat hasil_deref_4 = 0;
        buat hasil_u32 = 0;
        buat hasil_cmp = -1;
        buat hasil_mmio = 0;
        buat hasil_asm = 0;

        tidak_aman {
            // 1. Alokasi heap manual (malloc, memset, memcpy, memcmp)
            buat ptr = malloc(16);
            memset(ptr, 0, 16);

            // 2. Dereference pointer langsung (*ptr = val) & aritmatika pointer (*(ptr + offset))
            *ptr = 42;
            *(ptr + 4) = 99;
            hasil_deref_0 = *ptr;
            hasil_deref_4 = *(ptr + 4);

            // 3. Baca/tulis pointer bertipe (u32 little-endian)
            tulis_ptr(ptr + 8, 0xDEADBEEF, "u32");
            hasil_u32 = baca_ptr(ptr + 8, "u32");

            // 4. Salin blok memori dengan memcpy & bandingkan dengan memcmp
            buat ptr2 = malloc(16);
            memcpy(ptr2, ptr, 16);
            hasil_cmp = memcmp(ptr, ptr2, 16);

            free(ptr);
            free(ptr2);

            // 5. Register Hardware Memory-Mapped I/O (Volatile MMIO)
            volatil *(0x40020014) = 0xCAFEBABE;
            hasil_mmio = volatil *(0x40020014);

            // 6. Inline Assembly (asm! / rakitan!)
            hasil_asm = asm!("mov eax, 21; shl eax, 1; add eax, 8; ret");
        }
    "#;

    let interp = eval_widya(code);
    assert_eq!(get_num(&interp, "hasil_deref_0"), 42.0);
    assert_eq!(get_num(&interp, "hasil_deref_4"), 99.0);
    assert_eq!(get_num(&interp, "hasil_u32"), 0xDEADBEEFu32 as f64);
    assert_eq!(get_num(&interp, "hasil_cmp"), 0.0);
    assert_eq!(get_num(&interp, "hasil_mmio"), 0xCAFEBABEu32 as f64);
    assert_eq!(get_num(&interp, "hasil_asm"), 50.0);
}

#[test]
fn test_pilar_c3_direct_ast_to_elf64_native_execution_without_external_compiler() {
    let widya_src = r#"
        fungsi hitung_kombinasi(a: angka, b: angka) -> angka {
            buat total = 0;
            buat i = 1;
            selama i <= a {
                total = total + (i * b);
                i = i + 1;
            }
            kembalikan total;
        }

        tetapkan hasil = hitung_kombinasi(5, 4);
        tetapkan bit_cek = (0x0F << 4) | 0x05;
        cetak("ELF64_MURNI_OK");
        cetak(hasil);
        cetak(bit_cek);
        kembalikan 37;
    "#;

    let mut lexer = Lexer::new(widya_src);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");

    // Kompilasi AST langsung ke instruksi mesin x86_64 di dalam biner ELF-64 (tanpa rustc/gcc/ld!)
    let elf_bytes = PureElfBuilder::compile_ast_to_elf64(&program)
        .expect("Gagal mengompilasi AST ke ELF-64 murni");
    assert_eq!(&elf_bytes[0..4], b"\x7FELF");

    let bin_path = "/tmp/widya_c_parity_direct_elf_test";
    std::fs::write(bin_path, &elf_bytes).expect("Gagal menulis biner ELF");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(bin_path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(bin_path, perms).unwrap();
    }

    // Eksekusi langsung di kernel Linux x86_64!
    let output = std::process::Command::new(bin_path)
        .output()
        .expect("Gagal mengeksekusi biner ELF-64 murni");

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout_str.lines().collect();
    assert_eq!(lines, vec!["ELF64_MURNI_OK", "60", "245"]);
    assert_eq!(output.status.code(), Some(37));

    let _ = std::fs::remove_file(bin_path);
}

#[test]
fn test_pilar_c4_automatic_c_header_parser_and_direct_libc_ffi() {
    let code = r#"
        impor_header_c("
            #define MAX_BUFFER 1024
            #define MASK_WARNA 0xFF00

            enum StatusPerangkat {
                STATUS_IDLE = 0,
                STATUS_AKTIF = 1,
                STATUS_GALAT = 5
            };

            struct __attribute__((packed)) HeaderFrame {
                uint8_t magic;
                uint32_t panjang;
                uint16_t crc;
            };

            extern size_t strlen(const char *s);
            extern double hypot(double x, double y);
        ");

        tetapkan buf_sz = MAX_BUFFER;
        tetapkan mask   = MASK_WARNA;
        tetapkan st_err = STATUS_GALAT;
        tetapkan sz_hdr = sizeof("HeaderFrame");
        tetapkan pjg_c  = strlen("widya_setara_c");
        tetapkan sisi_m = hypot(3.0, 4.0);
    "#;

    let interp = eval_widya(code);
    assert_eq!(get_num(&interp, "buf_sz"), 1024.0);
    assert_eq!(get_num(&interp, "mask"), 0xFF00 as f64);
    assert_eq!(get_num(&interp, "st_err"), 5.0);
    assert_eq!(get_num(&interp, "sz_hdr"), 7.0); // packed: 1 + 4 + 2 = 7 bytes
    assert_eq!(get_num(&interp, "pjg_c"), 14.0);
    assert_eq!(get_num(&interp, "sisi_m"), 5.0);

    // Uji juga CHeaderParser langsung dari API Rust
    let parsed = CHeaderParser::parse_header_str(
        "struct BitReg { uint8_t en:1; uint8_t mode:3; uint32_t data; };",
    );
    assert_eq!(parsed.structs.len(), 1);
    assert_eq!(parsed.structs[0], "BitReg");
}

#[test]
fn test_pilar_c4_relocatable_elf_object_and_static_archive_linker() {
    // 1. Buat modul objek kedua (`math_ops.o`) yang mengekspor fungsi `hitung_tiga_kali`
    //    Pada System V AMD64 ABI atau akumulator RAX:
    //    mov rax, 19; imul rax, rax, 3; ret (19 * 3 = 57)
    let mut math_code = Vec::new();
    // mov rax, 19 (48 c7 c0 13 00 00 00)
    math_code.extend_from_slice(&[0x48, 0xC7, 0xC0, 19, 0x00, 0x00, 0x00]);
    // add rax, 25 (48 83 c0 19) -> 19 + 25 = 44
    math_code.extend_from_slice(&[0x48, 0x83, 0xC0, 25]);
    // ret (c3)
    math_code.push(0xC3);

    let mut math_obj = RelocatableObject::new("math_ops.o");
    math_obj.emit_code(&math_code);
    math_obj.define_global_symbol("hitung_nilai_c", 0);

    // Verifikasi serialisasi ELF-64 Relocatable Object (.o, ET_REL = 1)
    let elf_o_bytes = math_obj.to_elf64_relocatable_bytes();
    assert_eq!(&elf_o_bytes[0..4], b"\x7FELF");
    assert_eq!(u16::from_le_bytes([elf_o_bytes[16], elf_o_bytes[17]]), 1); // ET_REL

    // 2. Bungkus `math_ops.o` ke dalam Static Library Archive (`libmath.a`)
    let mut ar_builder = StaticArchiveBuilder::new();
    ar_builder.add_object(math_obj);
    let archive_bytes = ar_builder.to_archive_bytes();
    assert_eq!(&archive_bytes[0..8], b"!<arch>\n");

    // 3. Buat modul objek utama (`main.o`) yang memanggil `hitung_nilai_c` via R_X86_64_PC32
    //    lalu keluar dengan nilai RAX sebagai exit code (sys_exit)
    let mut main_obj = RelocatableObject::new("main.o");
    main_obj.define_global_symbol("_start", 0);
    // call rel32 (E8 00 00 00 00) -> relokasi di offset 1 dengan addend -4
    main_obj.emit_code(&[0xE8, 0x00, 0x00, 0x00, 0x00]);
    main_obj.add_pc32_relocation(1, "hitung_nilai_c", -4);
    // mov rdi, rax (48 89 c7)
    main_obj.emit_code(&[0x48, 0x89, 0xC7]);
    // mov eax, 60 (B8 3C 00 00 00) -> sys_exit
    main_obj.emit_code(&[0xB8, 60, 0x00, 0x00, 0x00]);
    // syscall (0F 05)
    main_obj.emit_code(&[0x0F, 0x05]);

    // 4. Link `main.o` + `libmath.a` menggunakan `PureElfObjectLinker` menjadi executable ELF-64
    let mut linker = PureElfObjectLinker::new();
    linker.add_object(main_obj);
    linker.add_archive(&ar_builder);
    let linked_elf = linker
        .link_to_executable("_start")
        .expect("Gagal melakukan static linking antar modul .o dan .a");

    let linked_path = "/tmp/widya_c_parity_linked_elf_test";
    std::fs::write(linked_path, &linked_elf).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(linked_path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(linked_path, perms).unwrap();
    }

    let status = std::process::Command::new(linked_path)
        .status()
        .expect("Gagal menjalankan hasil static linking");
    assert_eq!(status.code(), Some(44));

    let _ = std::fs::remove_file(linked_path);
}
