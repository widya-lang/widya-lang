use widya::desktop::{
    baca_timbangan_digital_raw, bangun_aplikasi_desktop, bentuk_modbus_read_holding_registers,
    bentuk_modbus_write_single_register, buat_delta_patch, buat_tanda_tangan_ed25519,
    buka_layar_pelanggan_dual_screen, daftar_hotkeys_aktif, daftar_serial_ports,
    daftarkan_global_hotkey, dekripsi_aes256_gcm, deteksi_daftar_monitor, enkripsi_aes256_gcm,
    hasilkan_manifest_keamanan_lock, hasilkan_raw_esc_pos, hitung_modbus_crc16,
    hitung_sha256_bytes, inisialisasi_proyek_desktop, papan_klip_baca, papan_klip_tulis,
    periksa_aplikasi_desktop, periksa_pembaruan, terapkan_delta_patch,
    verifikasi_tanda_tangan_ed25519, ChildWindowConfig, DesktopCompiler, DesktopConfig,
    DesktopPlatform, StrukThermalData, StrukThermalItem,
};

#[test]
fn test_desktop_config_defaults() {
    let config = DesktopConfig::default();
    assert_eq!(config.name, "AplikasiDesktopWidya");
    assert_eq!(config.app_id, "id.widya.aplikasi.desktop");
    assert_eq!(config.version, "1.0.0");
    assert_eq!(config.window.width, 1100);
    assert_eq!(config.window.height, 720);
    assert!(config.window.resizable);
    assert!(config.window.centered);
    assert!(!config.window.fullscreen);
    assert!(config.window.dark_mode);
    assert!(config.menu.enabled);
    assert_eq!(config.menu.items.len(), 4);
    assert!(config.tray.enabled);
    assert_eq!(config.bridge.port, 9095);
    assert!(config.targets.contains(&"windows".to_string()));

    // Enterprise Security Defaults
    assert!(config.security.require_ipc_token);
    assert!(config.security.csp_enabled);
    assert!(config.security.asset_integrity_check);
    assert!(config.security.strict_origin);
    assert!(config.security.encrypted_storage);
    assert_eq!(config.industrial.security_grade, "Enterprise Grade (A+)");
}

#[test]
fn test_desktop_platform_mapping() {
    let platforms = vec![
        DesktopPlatform::Windows,
        DesktopPlatform::Linux,
        DesktopPlatform::MacOS,
        DesktopPlatform::Universal,
    ];
    assert_eq!(platforms.len(), 4);
    assert_eq!(DesktopPlatform::Windows.as_str(), "windows");
    assert_eq!(DesktopPlatform::Windows.executable_extension(), "exe");
    assert_eq!(DesktopPlatform::Linux.as_str(), "linux");
    assert_eq!(DesktopPlatform::Linux.executable_extension(), "bin");
    assert_eq!(DesktopPlatform::MacOS.as_str(), "macos");
    assert_eq!(DesktopPlatform::MacOS.executable_extension(), "app");
    assert_eq!(DesktopPlatform::Universal.as_str(), "universal");
}

#[test]
fn test_desktop_project_scaffolding() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_desktop_test_scaffold_{}", pid));
    let project_name = "StudioWidyaApp";

    let res = inisialisasi_proyek_desktop(project_name, Some(&temp_dir), "standard");
    assert!(res.is_ok(), "Gagal scaffold proyek desktop: {:?}", res.err());

    let proj_path = res.unwrap();
    assert!(proj_path.exists());
    assert!(proj_path.join("widya_desktop.json").exists());
    assert!(proj_path.join("widya_security.lock").exists());
    assert!(proj_path.join("src").join("main.wya").exists());
    assert!(proj_path.join("src").join("database.wya").exists());
    assert!(proj_path.join("ui").join("index.html").exists());
    assert!(proj_path.join("ui").join("css").join("style.css").exists());
    assert!(proj_path.join("ui").join("js").join("widya-bridge.js").exists());
    assert!(proj_path.join("ui").join("js").join("app.js").exists());
    assert!(proj_path.join("assets").join("icon.svg").exists());
    assert!(proj_path.join("assets").join("icon.png").exists());
    assert!(proj_path.join("assets").join("tray_icon.png").exists());
    assert!(proj_path.join("README.md").exists());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_desktop_project_validation_and_audit() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_desktop_test_validate_{}", pid));
    let project_name = "ValidasiDesktopApp";

    let proj_path = inisialisasi_proyek_desktop(project_name, Some(&temp_dir), "standard").unwrap();
    let report_res = periksa_aplikasi_desktop(&proj_path);
    assert!(report_res.is_ok());

    let report = report_res.unwrap();
    assert!(report.valid);
    assert_eq!(report.app_name, project_name);
    assert!(report.manifest_exists);
    assert!(report.backend_exists);
    assert!(report.frontend_exists);
    assert!(report.security_lock_exists);
    assert!(report.security_score >= 90);
    assert_eq!(report.security_grade, "Enterprise Grade (A+)");
    assert!(report.issues.is_empty());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_desktop_build_windows_bundle() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_desktop_test_build_win_{}", pid));
    let out_dir = std::env::temp_dir().join(format!("widya_desktop_out_win_{}", pid));
    let project_name = "KompilasiWindowsApp";

    let proj_path = inisialisasi_proyek_desktop(project_name, Some(&temp_dir), "standard").unwrap();
    let build_res = bangun_aplikasi_desktop(&proj_path, Some(&out_dir), "windows");
    assert!(build_res.is_ok(), "Gagal build desktop windows bundle: {:?}", build_res.err());

    let bundle_dir = build_res.unwrap();
    assert!(bundle_dir.exists());
    assert!(bundle_dir.join(format!("{}.bat", project_name)).exists());
    assert!(bundle_dir.join("run.ps1").exists());
    assert!(bundle_dir.join("app").join("widya_desktop.json").exists());
    assert!(bundle_dir.join("app").join("widya_security.lock").exists());
    assert!(bundle_dir.join("app").join("ui").join("index.html").exists());
    assert!(bundle_dir.join("app").join("src").join("main.wya").exists());

    let _ = std::fs::remove_dir_all(&temp_dir);
    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn test_desktop_build_linux_bundle() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_desktop_test_build_lin_{}", pid));
    let out_dir = std::env::temp_dir().join(format!("widya_desktop_out_lin_{}", pid));
    let project_name = "KompilasiLinuxApp";

    let proj_path = inisialisasi_proyek_desktop(project_name, Some(&temp_dir), "standard").unwrap();
    let build_res = bangun_aplikasi_desktop(&proj_path, Some(&out_dir), "linux");
    assert!(build_res.is_ok(), "Gagal build desktop linux bundle: {:?}", build_res.err());

    let bundle_dir = build_res.unwrap();
    assert!(bundle_dir.exists());
    assert!(bundle_dir.join(format!("{}.sh", project_name)).exists());
    assert!(bundle_dir.join("app").join("widya_desktop.json").exists());
    assert!(bundle_dir.join("app").join("widya_security.lock").exists());

    let _ = std::fs::remove_dir_all(&temp_dir);
    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn test_desktop_crypto_vault_aes256gcm() {
    let secret_data = b"DataRahasiaSangatPentingWidya2026!";
    let master_key = "KunciMasterSuperAman32ByteWidya!";

    // 1. Enkripsi
    let enc_res = enkripsi_aes256_gcm(secret_data, master_key);
    assert!(enc_res.is_ok());
    let (ciphertext, nonce) = enc_res.unwrap();
    assert!(!ciphertext.is_empty());
    assert_eq!(nonce.len(), 24); // 12 bytes = 24 hex chars

    // 2. Dekripsi dengan kunci yang benar
    let dec_res = dekripsi_aes256_gcm(&ciphertext, &nonce, master_key);
    assert!(dec_res.is_ok());
    let decrypted = dec_res.unwrap();
    assert_eq!(decrypted, secret_data);

    // 3. Dekripsi dengan kunci yang salah harus gagal (otentikasi GCM)
    let wrong_key_res = dekripsi_aes256_gcm(&ciphertext, &nonce, "KunciSalahTotal!!");
    assert!(wrong_key_res.is_err());

    // 4. Dekripsi dengan ciphertext yang dimodifikasi (tampered) harus gagal
    let mut tampered_cipher = ciphertext.clone();
    let first_char = tampered_cipher.chars().next().unwrap_or('0');
    let replacement = if first_char == 'a' { "b" } else { "a" };
    tampered_cipher.replace_range(0..1, replacement);
    let tampered_res = dekripsi_aes256_gcm(&tampered_cipher, &nonce, master_key);
    assert!(tampered_res.is_err());
}

#[test]
fn test_desktop_sha256_lockfile_integrity() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_desktop_test_lock_{}", pid));
    let project_name = "IntegrityTestApp";

    let proj_path = inisialisasi_proyek_desktop(project_name, Some(&temp_dir), "standard").unwrap();
    let lock_file = hasilkan_manifest_keamanan_lock(&proj_path);
    assert!(lock_file.is_ok());

    let lock_content = std::fs::read_to_string(lock_file.unwrap()).unwrap();
    assert!(lock_content.contains("SHA-256"));
    assert!(lock_content.contains("ui/index.html"));
    assert!(lock_content.contains("src/main.wya"));

    // Hash of specific string verification
    let h = hitung_sha256_bytes(b"Widya-Lang");
    assert!(!h.is_empty());
    assert_eq!(h.len(), 64); // SHA-256 produces 64 hex characters

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_desktop_modul_wya_execution() {
    let code = r#"
        impor "./modul/desktop.wya";

        var app = BuatAplikasiDesktop("Uji Otomatis Widya", 1024, 768);
        app.set_judul("Uji Widya Desktop Modul Enterprise");
        app.pusatkan();
        app.set_layar_penuh(benar);
        app.set_layar_penuh(salah);

        app.tambah_menu("Uji Menu", "Ctrl+U");
        app.kirim_notifikasi("Pengujian", "Test Notifikasi Baki Berhasil!");

        // Daftarkan RPC Handler
        app.daftarkan_rpc("hitung_luas", fungsi(payload) {
            kembalikan payload.p * payload.l;
        });

        var hasil_rpc = app.jalankan_rpc("hitung_luas", { "p": 10, "l": 5 });

        // Verifikasi dialog
        var file_buka = BukaDialogPilihBerkas("*.wya");
        var file_simpan = BukaDialogSimpanBerkas("hasil_uji.txt");

        // Verifikasi Audit Keamanan Modul
        var audit = AuditKeamananAplikasi();
        pastikan(audit.skor == 100);
        pastikan(audit.token_aktif == benar);

        app.tutup();
        hasil_rpc;
    "#;

    let hasil = widya::jalankan(code);
    assert!(hasil.is_ok(), "Gagal mengeksekusi modul/desktop.wya: {:?}", hasil.err());
    let val = hasil.unwrap();
    match val {
        widya::value::Value::Number(n) => assert_eq!(n, 50.0),
        _ => panic!("Expected Value::Number(50.0), got {:?}", val),
    }
}

#[test]
fn test_desktop_ed25519_signatures() {
    let payload = b"Widya Desktop Enterprise Update v1.1.0-sha256:abcd1234efgh5678";
    let private_key_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    // 1. Generate digital signature & public key
    let sign_res = buat_tanda_tangan_ed25519(payload, private_key_hex);
    assert!(sign_res.is_ok(), "Signing failed: {:?}", sign_res.err());
    let (sig_hex, pub_hex) = sign_res.unwrap();
    assert_eq!(sig_hex.len(), 128); // 64 bytes = 128 hex chars
    assert_eq!(pub_hex.len(), 64);  // 32 bytes = 64 hex chars

    // 2. Verify valid signature
    let verify_res = verifikasi_tanda_tangan_ed25519(payload, &sig_hex, &pub_hex);
    assert!(verify_res.is_ok());
    assert!(verify_res.unwrap());

    // 3. Tampered payload must fail verification
    let tampered_payload = b"Widya Desktop Enterprise Update v1.1.0-TAMPERED-INJECTION";
    let tampered_res = verifikasi_tanda_tangan_ed25519(tampered_payload, &sig_hex, &pub_hex);
    assert!(tampered_res.is_err());

    // 4. Invalid signature format must fail gracefully
    let invalid_sig = verifikasi_tanda_tangan_ed25519(payload, "invalid_sig_hex", &pub_hex);
    assert!(invalid_sig.is_err());

    // 5. Invalid key length must return error
    let bad_key_res = buat_tanda_tangan_ed25519(payload, "short_key");
    assert!(bad_key_res.is_err());
}

#[test]
fn test_desktop_esc_pos_receipt_generation() {
    let struk = StrukThermalData {
        nama_toko: "WIDYA ENTERPRISE POS".to_string(),
        alamat: "Sudirman Central Business District".to_string(),
        nomor_struk: "WIDYA-TRX-2026-999".to_string(),
        kasir: "Kasir Widya 01".to_string(),
        waktu: "2026-09-25 12:30:00".to_string(),
        items: vec![
            StrukThermalItem {
                nama: "Widya Studio Pro".to_string(),
                jumlah: 1,
                harga: 1500000.0,
                total: 1500000.0,
            },
            StrukThermalItem {
                nama: "Dongle Hardware".to_string(),
                jumlah: 2,
                harga: 250000.0,
                total: 500000.0,
            },
        ],
        subtotal: 2000000.0,
        pajak: 220000.0,
        total: 2220000.0,
        metode_bayar: "QRIS Dinamis".to_string(),
        catatan_kaki: "Terima Kasih atas Kepercayaan Anda!".to_string(),
    };

    let raw_esc_pos = hasilkan_raw_esc_pos(&struk);
    assert!(!raw_esc_pos.is_empty());

    // Verify ESC @ (Printer Init: 0x1B, 0x40)
    assert_eq!(&raw_esc_pos[0..2], &[0x1B, 0x40]);

    // Verify GS V 66 0 (Paper Cut: 0x1D, 0x56, 0x42, 0x00)
    let cut_opcode = [0x1D, 0x56, 0x42, 0x00];
    let has_cut = raw_esc_pos.windows(4).any(|w| w == cut_opcode);
    assert!(has_cut, "ESC/POS buffer must contain paper cut opcode");

    // Verify content embedded in binary stream
    let raw_text = String::from_utf8_lossy(&raw_esc_pos);
    assert!(raw_text.contains("WIDYA ENTERPRISE POS"));
    assert!(raw_text.contains("WIDYA-TRX-2026-999"));
    assert!(raw_text.contains("Widya Studio Pro"));
    assert!(raw_text.contains("Dongle Hardware"));
    assert!(raw_text.contains("QRIS Dinamis"));
}

#[test]
fn test_desktop_clipboard_operations() {
    let sample = "Widya Desktop Clipboard Test Content 2026";
    let ok = papan_klip_tulis(sample);
    assert!(ok, "Writing to clipboard should succeed");

    let read_back = papan_klip_baca();
    assert_eq!(read_back, sample, "Reading back from clipboard should match written text");
}

#[test]
fn test_desktop_child_window_config() {
    let child = ChildWindowConfig {
        id: "modal_editor".to_string(),
        title: "Sub Editor Widya".to_string(),
        width: 800,
        height: 600,
        url: "editor.html".to_string(),
        modal: true,
        always_on_top: true,
        resizable: false,
    };

    let json_serialized = serde_json::to_string(&child).unwrap();
    assert!(json_serialized.contains("\"modal\":true"));
    assert!(json_serialized.contains("\"always_on_top\":true"));

    let deserialized: ChildWindowConfig = serde_json::from_str(&json_serialized).unwrap();
    assert_eq!(deserialized.id, "modal_editor");
    assert_eq!(deserialized.title, "Sub Editor Widya");
    assert!(deserialized.modal);
    assert!(deserialized.always_on_top);
    assert!(!deserialized.resizable);
}

#[test]
fn test_desktop_splash_screen_and_window_config() {
    let config = DesktopConfig::default();
    assert!(config.window.splash_screen, "Splash screen should be enabled by default");
    assert_eq!(config.window.width, 1100);
    assert_eq!(config.window.height, 720);
    assert!(config.window.dark_mode);
}

#[test]
fn test_desktop_auto_updater_and_delta_patch() {
    let base_binary = b"WidyaDesktopEnterpriseApplicationBinary_v1.0.0_Payload_Data";
    let new_binary = b"WidyaDesktopEnterpriseApplicationBinary_v1.1.0_Payload_Data_With_POS_Driver";

    // Create delta patch
    let delta = buat_delta_patch(base_binary, new_binary);
    assert!(delta.starts_with(b"WIDYADELTA10"), "Patch must start with magic header");
    assert!(delta.len() > 28, "Patch must contain metadata");

    // Apply delta patch
    let reconstructed = terapkan_delta_patch(base_binary, &delta).unwrap();
    assert_eq!(reconstructed, new_binary, "Reconstructed binary must match target byte-for-byte");

    // Manifest verification
    let manifest_json = r#"{
        "app_name": "WidyaPOS",
        "version": "1.1.0",
        "url": "https://releases.widya-lang.org/app.bin",
        "sha256": "abcdef1234567890",
        "signature_ed25519": "sig",
        "mandatory": true,
        "release_notes": "Driver Timbangan & Modbus"
    }"#;
    let check = periksa_pembaruan(manifest_json, "1.0.0").unwrap();
    assert!(check.update_available, "Update should be available");
    assert_eq!(check.latest_version, "1.1.0");
    assert!(check.mandatory);
}

#[test]
fn test_desktop_industrial_serial_and_modbus_protocol() {
    // 1. Serial port detection
    let ports = daftar_serial_ports();
    assert!(!ports.is_empty(), "Harus mendeteksi daftar port serial default");

    // 2. Modbus CRC16 Calculation
    let test_frame = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x0A];
    let crc = hitung_modbus_crc16(&test_frame);
    assert_ne!(crc, 0, "CRC16 tidak boleh 0");

    // 3. Modbus Read Frame
    let read_frame = bentuk_modbus_read_holding_registers(1, 100, 5);
    assert_eq!(read_frame.len(), 8);
    assert_eq!(read_frame[0], 1);
    assert_eq!(read_frame[1], 0x03);

    // 4. Modbus Write Frame
    let write_frame = bentuk_modbus_write_single_register(1, 200, 1250);
    assert_eq!(write_frame.len(), 8);
    assert_eq!(write_frame[0], 1);
    assert_eq!(write_frame[1], 0x06);

    // 5. Digital Weighing Scale ASCII Stream Parser
    let raw_stream = "ST,GS,+0024.75kg\r\n";
    let reading = baca_timbangan_digital_raw(raw_stream).unwrap();
    assert_eq!(reading.weight_kg, 24.75);
    assert_eq!(reading.unit, "kg");
    assert!(reading.is_stable);
}

#[test]
fn test_desktop_multi_monitor_and_customer_display() {
    let monitors = deteksi_daftar_monitor();
    assert!(monitors.len() >= 2, "Harus mendukung minimal 2 layar monitor");
    assert!(monitors[0].is_primary, "Layar 1 harus menjadi primary");
    assert!(!monitors[1].is_primary, "Layar 2 adalah customer display");

    let dual_screen = buka_layar_pelanggan_dual_screen(2, "Layar Kasir Pelanggan", "customer.html");
    assert_eq!(dual_screen.title, "Layar Kasir Pelanggan");
    assert_eq!(dual_screen.width, 1024);
    assert_eq!(dual_screen.height, 768);
    assert_eq!(dual_screen.url, "customer.html");
}

#[test]
fn test_desktop_global_hotkeys_engine() {
    let reg = daftarkan_global_hotkey("Ctrl+Shift+K", "buka_kasir_cepat").unwrap();
    assert_eq!(reg.key_combination, "Ctrl+Shift+K");
    assert_eq!(reg.action_name, "buka_kasir_cepat");
    assert!(reg.registered);

    let active = daftar_hotkeys_aktif();
    assert!(active.iter().any(|h| h.key_combination == "Ctrl+Shift+K"));
}


