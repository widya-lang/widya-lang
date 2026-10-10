//! Test Integrasi 100% Berdaulat: WidyaDesktop Tauri 2.0 & Qt 6 Parity
//! Zero External Dependencies (Tanpa Electron, Chromium, Qt, SDL, atau GTK).

use widya::desktop::pure_window::{JendelaMurni, PengelolaJendelaMurni, StatusJendela, EfekLatarJendela};
use widya::desktop::pure_ipc::JembatanIPCMurni;
use widya::desktop::pure_hardware::{EscPosGenerator, ModbusRTUEngine, ParserTimbanganPOS};
use widya::desktop::pure_display::{PengontrolMultiMonitor, InfoMonitor};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn test_pure_window_lifecycle_geometry_and_dirty_regions() {
    let mut window = JendelaMurni::baru("win-pos-main", "Widya POS Enterprise", 1024, 768);
    window.efek_latar = EfekLatarJendela::MicaDark;

    // Test resize constraint
    window.batasan.min_lebar = 800;
    window.batasan.min_tinggi = 600;
    window.ubah_ukuran(500, 400); // Harus di-clamp ke 800x600
    assert_eq!(window.lebar, 800);
    assert_eq!(window.tinggi, 600);

    // Test Dirty Region Tracking
    let dirty = window.bersihkan_dirty_regions();
    assert_eq!(dirty.len(), 1);
    assert_eq!(dirty[0], (0, 0, 800, 600));

    // Test Kiosk Mode
    window.masuk_mode_kiosk();
    assert_eq!(window.status, StatusJendela::Kiosk);
    assert!(window.tanpa_bingkai);
    assert!(window.selalu_di_atas);

    // Test Window Manager
    let wm = PengelolaJendelaMurni::baru();
    wm.daftarkan_jendela(window);
    assert_eq!(wm.total_jendela(), 1);

    wm.mutasi_jendela("win-pos-main", |w| {
        w.tutup();
    });

    let status = wm.ambil_jendela("win-pos-main", |w| w.status);
    assert_eq!(status, Some(StatusJendela::Ditutup));
}

#[test]
fn test_zero_copy_ipc_and_token_security() {
    let ipc = JembatanIPCMurni::baru();

    // Security token
    ipc.daftarkan_token_jendela("win-kasir-1", "token_rahasia_ipc_9921");
    assert!(ipc.validasi_token("win-kasir-1", "token_rahasia_ipc_9921"));
    assert!(!ipc.validasi_token("win-kasir-1", "token_palsu"));

    // Register RPC handler
    ipc.daftarkan_metode("ambil_total_penjualan", |_payload| {
        let total = 1500000u64;
        Ok(total.to_le_bytes().to_vec())
    });

    let res = ipc.panggil("ambil_total_penjualan", &[]);
    assert!(res.is_ok());
    let bytes = res.unwrap();
    let total = u64::from_le_bytes(bytes.try_into().unwrap());
    assert_eq!(total, 1500000);

    // Pub-Sub Event Channel
    let event_received = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&event_received);

    ipc.langganan_topik("transaksi_sukses", move |data| {
        if data == b"TRX-20261009-001" {
            flag_clone.store(true, Ordering::SeqCst);
        }
    });

    let count = ipc.siarkan("transaksi_sukses", b"TRX-20261009-001");
    assert_eq!(count, 1);
    assert!(event_received.load(Ordering::SeqCst));
}

#[test]
fn test_escpos_thermal_receipt_generator() {
    let mut escpos = EscPosGenerator::baru();
    escpos
        .rata_tengah()
        .tebal(true)
        .tulis_baris("TOKO BERDAULAT NUSANTARA")
        .tebal(false)
        .tulis_baris("Jl. Kedaulatan Digital No. 1")
        .garis_pembatas('-', 32)
        .rata_kiri()
        .tulis_baris("1x Laptop Widya Murni    Rp 15.000.000")
        .tulis_baris("1x Mouse Nirkabel        Rp    250.000")
        .garis_pembatas('=', 32)
        .rata_kanan()
        .tulis_baris("TOTAL: Rp 15.250.000")
        .rata_tengah()
        .cetak_barcode_code128("TRX-8839210")
        .potong_kertas()
        .buka_laci_kasir();

    let bytes = escpos.ambil_bytes();
    assert!(!bytes.is_empty());
    // ESC @ (Inisialisasi)
    assert_eq!(&bytes[0..2], &[0x1B, 0x40]);
    // Harus mengandung string nama toko
    let raw_str = String::from_utf8_lossy(bytes);
    assert!(raw_str.contains("TOKO BERDAULAT NUSANTARA"));
    assert!(raw_str.contains("TOTAL: Rp 15.250.000"));
}

#[test]
fn test_modbus_rtu_and_scale_parser() {
    // 1. Modbus RTU Frame Creation
    // Slave 1, Read Holding Register starting address 0x006B, count 3
    let frame = ModbusRTUEngine::buat_frame_baca_holding_registers(1, 107, 3);
    assert_eq!(frame.len(), 8);
    assert_eq!(frame[0], 1); // Slave ID
    assert_eq!(frame[1], 3); // Function Code 0x03

    // Verify CRC-16
    assert!(ModbusRTUEngine::verifikasi_frame_respon(&frame));

    // 2. POS Scale ASCII Parsing
    let raw_scale_cas = "ST,GS,+  12.450kg\r\n";
    let berat = ParserTimbanganPOS::parse_berat_kg(raw_scale_cas);
    assert!(berat.is_ok());
    assert!((berat.unwrap() - 12.450).abs() < 0.001);
}

#[test]
fn test_multi_monitor_and_customer_facing_display() {
    let controller = PengontrolMultiMonitor::baru();

    // Tambah monitor sekunder (Customer Display)
    let secondary = InfoMonitor {
        id: 1,
        nama: "Customer Display 10-inch".to_string(),
        x: 1920,
        y: 0,
        lebar: 1280,
        tinggi: 800,
        scale_factor: 1.0,
        is_primary: false,
        refresh_rate_hz: 60,
    };
    controller.daftarkan_monitor(secondary);

    let monitors = controller.daftar_monitor();
    assert_eq!(monitors.len(), 2);

    let primary = controller.ambil_monitor_primer();
    assert!(primary.is_some());
    assert!(primary.unwrap().is_primary);

    let sec = controller.ambil_monitor_sekunder();
    assert!(sec.is_some());
    assert!(!sec.unwrap().is_primary);

    // Update customer checkout screen
    controller.perbarui_layar_pelanggan(
        15250000,
        "Laptop Widya Murni",
        Some("00020101021226580014ID.LINKAJA.WWW..."),
    );

    let snap = controller.snapshot_layar_pelanggan();
    assert_eq!(snap.total_belanja_rp, 15250000);
    assert_eq!(snap.item_terakhir, "Laptop Widya Murni");
    assert!(snap.qris_payload.is_some());
}
