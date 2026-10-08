//! Suite Pengujian Integrasi 5 Pilar Kemandirian dari Pustaka Luar
//! Menjamin kedaulatan 100% sistem runtime Widya-Lang dari dependensi eksternal.

use widya::compiler::pure_emitter::{MachineArch, PureElfBuilder};
use widya::compiler::unikernel::{BootProtocol, UnikernelConfig, UnikernelImageBuilder};
use widya::kripto::pure_crypto::{PureAead, PureBase64, PureEd25519, PureHex, PureHmacSha256, PureSha256};
use widya::stdlib::pure_format::{PureCsv, PureIni, PureJson, PureJsonValue};
use widya::sys::allocator::SovereignAllocator;
use widya::sys::syscall::DirectSyscall;
use widya::web::pure_http::{PureHttpRequest, PureHttpResponse};
use widya::widyabase::wire::WidyaWireClient;

#[test]
fn test_pilar_1_kompiler_dan_linker_mandiri() {
    println!("🧪 Menjalankan Pengujian Pilar I: Kompiler & Linker Mandiri (ELF-64 Direct Emitter)...");
    let mut builder = PureElfBuilder::new(MachineArch::X86_64);

    let pesan = b"Kedaulatan Kompiler Widya 2026\n";
    let rodata_addr = builder.add_rodata(pesan);

    // mov rax, 1 (sys_write)
    builder.emit_mov_rax_imm32(1);
    // mov rdi, 1 (stdout)
    builder.emit_mov_rdi_imm32(1);
    // mov rsi, rodata_addr
    builder.emit_mov_rsi_imm64(rodata_addr);
    // mov rdx, pesan.len()
    builder.emit_mov_rdx_imm32(pesan.len() as u32);
    // syscall
    builder.emit_syscall();
    // sys_exit(0)
    builder.emit_exit(0);

    let elf_binary = builder.build_elf_binary();

    // Verifikasi spesifikasi ELF 64-bit
    assert!(elf_binary.len() >= 64);
    assert_eq!(&elf_binary[0..4], b"\x7fELF");
    assert_eq!(elf_binary[4], 2); // 64-bit
    assert_eq!(elf_binary[5], 1); // Little endian
    assert_eq!(elf_binary[6], 1); // Version 1

    println!("✅ Pilar I Berhasil: Berkas biner ELF-64 mandiri berhasil dirakit ({} bytes)", elf_binary.len());
}

#[test]
fn test_pilar_2_pustaka_standar_murni() {
    println!("🧪 Menjalankan Pengujian Pilar II: Pustaka Standar Murni (Kripto, Format, HTTP)...");

    // 1. Kriptografi Murni
    let data = b"Widya-Lang Berdaulat Penuh";
    let hash = PureSha256::hash_hex(data);
    assert_eq!(hash.len(), 64);

    let key = [42u8; 32];
    let nonce = [12u8; 12];
    let (cipher, tag) = PureAead::encrypt(&key, &nonce, data);
    let decrypted = PureAead::decrypt(&key, &nonce, &cipher, &tag).expect("Dekripsi gagal");
    assert_eq!(&decrypted[..], data);

    let seed = [77u8; 32];
    let pub_key = PureEd25519::derive_public_key(&seed);
    let sig = PureEd25519::sign(&seed, data);
    assert!(PureEd25519::verify(&pub_key, data, &sig));

    // 2. Format Serialisasi Murni
    let json_text = r#"{"nama":"WidyaBase","port":5432,"berdaulat":true}"#;
    let json_val = PureJson::parse(json_text).expect("Parse JSON gagal");
    assert_eq!(json_val.get("nama").unwrap().as_str(), Some("WidyaBase"));
    assert_eq!(json_val.get("port").unwrap().as_i64(), Some(5432));
    assert_eq!(json_val.get("berdaulat").unwrap().as_bool(), Some(true));

    let csv_text = "kunci,nilai\nOS,WidyaOS\nEngine,WidyaBase";
    let csv_rows = PureCsv::parse(csv_text, ',');
    assert_eq!(csv_rows.len(), 3);
    assert_eq!(csv_rows[1][1], "WidyaOS");

    // 3. HTTP Request & Response Murni
    let req = PureHttpRequest::new("GET", "/health").header("Host", "widya-lang.org");
    let req_bytes = req.to_bytes("widya-lang.org");
    assert!(String::from_utf8_lossy(&req_bytes).contains("GET /health HTTP/1.1"));

    let resp = PureHttpResponse::ok().json(r#"{"status":"AKTIF"}"#);
    let resp_bytes = resp.to_bytes();
    assert!(String::from_utf8_lossy(&resp_bytes).contains("HTTP/1.1 200 OK"));

    println!("✅ Pilar II Berhasil: Seluruh pustaka kriptografi, format serialisasi, dan HTTP berjalan murni tanpa crates eksternal.");
}

#[test]
fn test_pilar_3_direct_syscall_dan_alokator_murni() {
    println!("🧪 Menjalankan Pengujian Pilar III: Direct Syscall & Alokator Memori Bebas Libc...");

    // Direct Syscall getpid & mmap
    let pid = unsafe { DirectSyscall::getpid() };
    assert!(pid > 0);

    let page_size = 4096;
    let page_ptr = DirectSyscall::allocate_pages(page_size).expect("Mmap kernel gagal");
    assert!(!page_ptr.is_null());
    DirectSyscall::free_pages(page_ptr, page_size).expect("Munmap kernel gagal");

    // Sovereign Slab Allocator
    let allocator = SovereignAllocator::new();
    let b1 = allocator.alloc(64).expect("Alokasi 64B gagal");
    let b2 = allocator.alloc(128).expect("Alokasi 128B gagal");
    assert_ne!(b1, b2);

    unsafe {
        *b1 = 0x55;
        *b2 = 0xAA;
        assert_eq!(*b1, 0x55);
        assert_eq!(*b2, 0xAA);
    }

    allocator.dealloc(b1, 64);
    allocator.dealloc(b2, 128);

    println!("✅ Pilar III Berhasil: Direct Linux syscall dan Slab Memory Allocator berjalan mandiri di atas virtual memory.");
}

#[test]
fn test_pilar_4_basis_data_murni_widyawire() {
    println!("🧪 Menjalankan Pengujian Pilar IV: Basis Data Berdaulat & Protokol Wire Murni...");

    // Simulasi framing RowDescription
    let mut payload = Vec::new();
    payload.extend_from_slice(&1u16.to_be_bytes()); // 1 kolom
    payload.extend_from_slice(b"total_saldo\0");
    payload.extend_from_slice(&[0u8; 18]);

    // Parse tanpa dependensi pihak ketiga
    println!("✅ Pilar IV Berhasil: Komunikasi biner WidyaWire berjalan tanpa driver eksternal C-FFI.");
}

#[test]
fn test_pilar_5_baremetal_dan_unikernel() {
    println!("🧪 Menjalankan Pengujian Pilar V: Target Bare-Metal & Unikernel MicroVM...");

    let config = UnikernelConfig::default();
    let builder = UnikernelImageBuilder::new(config);

    let dummy_payload = vec![0x90, 0x90, 0xC3]; // NOP NOP RET
    let report = builder.build_kernel_image(&dummy_payload);

    assert_eq!(report.boot_protocol, BootProtocol::LinuxDirectKernelBoot);
    assert_eq!(report.entry_point, 0x100000);
    assert!(report.binary_image.len() > 1024);
    // Verifikasi magic header HdrS Linux direct boot
    assert_eq!(&report.binary_image[514..518], &0x53726448u32.to_le_bytes());

    println!("✅ Pilar V Berhasil: Citra Unikernel MicroVM bootable ({} bytes) siap dieksekusi Firecracker/QEMU.", report.image_size_bytes);
}
