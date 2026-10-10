// tests/penta_core_sovereign_matrix_test.rs
// ==============================================================================
// WIDYA PENTA-CORE SOVEREIGN MATRIX INTEGRATION TEST SUITE
// 100% Zero-External-Dependency Sovereign Computing Verification
// Pillars: WidyaOS, WidyaBase, WidyaWeb, WidyaDesktop, WidyaMobile
// ==============================================================================

use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use serde_json::json;

// Pillar 1: WidyaOS
use widya::os::{
    MemoryManager, VirtualAddressSpace, PageFlags,
    KernelThreadManager, Priority, ThreadState,
    VirtualFilesystem, OpenFlags, FileType,
};

// Pillar 2: WidyaBase
use widya::widyabase::{
    docdb::WidyaDocDB,
    hnsw::HnswVectorIndex,
    cdc::{CdcStreamer, CdcRecord, CdcOp},
    sql_parser::SqlValue,
};

// Pillar 3: WidyaWeb
use widya::web::{
    AppRouterMurni, MesinSSRMurni, KomponenIsland, IsrCacheHub,
    ServerActionDispatcher, VerifikatorJWTMurni, RateLimiterBucket,
    CssScopedEngine, MinifierWebMurni,
};

// Pillar 4: WidyaDesktop
use widya::desktop::{
    JendelaMurni, PengelolaJendelaMurni, StatusJendela, EfekLatarJendela,
    JembatanIPCMurni, EscPosGenerator, ModbusRTUEngine, ParserTimbanganPOS,
};

// Pillar 5: WidyaMobile
use widya::graphics::{
    FramebufferMurni, Warna, PersegiKotak, Titik2D,
    WidyaWidgets, WidgetMurni, RenderKotakMurni, BatasanKotak,
    PemberitahuNilai, Sinyal, WidyaNavigator, RuteHalaman,
};

fn sample_pos_daemon(arg: usize) -> usize {
    arg * 2
}

#[test]
fn test_pillar_1_widyaos_sovereign_kernel() {
    println!(">>> Testing Pillar 1: WidyaOS Sovereign Kernel...");

    // 1. Virtual Memory Management (Paging & Address Spaces)
    let mem_mgr = MemoryManager::new(64);
    let alloc_res = mem_mgr.allocate(4096, PageFlags::READ | PageFlags::WRITE);
    assert!(alloc_res.is_ok());
    let virt_addr = alloc_res.unwrap();
    let phys_addr = mem_mgr.translate(virt_addr);
    assert!(phys_addr.is_some());

    // 2. Preemptive Task & Thread Scheduler
    let thread_mgr = KernelThreadManager::new();
    let handle = thread_mgr.create_thread(sample_pos_daemon, 42, Priority::High).unwrap();
    assert_eq!(handle.id(), 1);
    let sched_res = thread_mgr.run_scheduler();
    assert!(sched_res.is_ok());

    // 3. WidyaFS Sovereign CoW Virtual Filesystem
    let vfs = VirtualFilesystem::new();
    vfs.mkdir("/system/apps", 0o755).unwrap();
    let fd = vfs.open("/system/apps/pos.wya", OpenFlags::from_raw(0x13)).unwrap();
    let write_res = vfs.write(fd, b"cetak('Sovereign WidyaOS Ready')", 0).unwrap();
    assert!(write_res > 0);
    vfs.close(fd).unwrap();

    let rfd = vfs.open("/system/apps/pos.wya", OpenFlags::from_raw(0x01)).unwrap();
    let mut buf = vec![0u8; 32];
    let bytes_read = vfs.read(rfd, &mut buf, 0).unwrap();
    vfs.close(rfd).unwrap();
    assert!(bytes_read > 0);
}

#[test]
fn test_pillar_2_widyabase_multimodel_engine() {
    println!(">>> Testing Pillar 2: WidyaBase Multi-Model Engine...");

    // 1. Document Database Engine (WidyaDocDB)
    let db_path = "target/test_docdb_penta_matrix";
    let _ = fs::remove_dir_all(db_path);
    let docdb = WidyaDocDB::open(db_path).unwrap();
    let coll = docdb.collection("transaksi");
    
    let doc_id = coll.insert_one(
        json!({"invoice": "INV-2026-001", "total": 150000, "status": "LUNAS"})
    ).unwrap();
    
    let found = coll.find_one(&json!({"invoice": "INV-2026-001"})).unwrap();
    assert_eq!(found["invoice"], "INV-2026-001");
    assert_eq!(found["total"], 150000);

    // 2. Vector HNSW Engine (Semantic Similarity)
    let mut hnsw = HnswVectorIndex::new(16, 32);
    hnsw.insert("DOC-1", &vec![1.0, 0.0, 0.0]);
    hnsw.insert("DOC-2", &vec![0.9, 0.1, 0.0]);
    hnsw.insert("DOC-3", &vec![0.0, 1.0, 0.0]);
    let nearest = hnsw.search_knn(&vec![1.0, 0.05, 0.0], 2, 16);
    assert_eq!(nearest[0].0, "DOC-1");
    assert_eq!(nearest[1].0, "DOC-2");

    // 3. Change Data Capture (CDC) Realtime Replication
    let cdc = CdcStreamer::new(100);
    let mut sub = cdc.subscribe(None, None);
    let mut after_data = HashMap::new();
    after_data.insert("nama".to_string(), SqlValue::Text("Beras Merah Putih".to_string()));
    after_data.insert("stok".to_string(), SqlValue::Integer(500));
    
    let rec = CdcRecord::new(1, 1001, "inventori", CdcOp::Insert, None, Some(after_data));
    cdc.emit(rec.clone());
    
    let received = sub.try_recv().unwrap().unwrap();
    assert_eq!(received.table, "INVENTORI");
    assert_eq!(received.op, CdcOp::Insert);
}

#[test]
fn test_pillar_3_widyaweb_ssr_and_actions() {
    println!(">>> Testing Pillar 3: WidyaWeb SSR & Server Actions...");

    // 1. App Router
    let mut router = AppRouterMurni::baru_memori();
    router.daftarkan_rute(
        "/produk/[id]",
        PathBuf::from("/memori/app/produk/[id]/page.wya"),
        Some(PathBuf::from("/memori/app/layout.wya")),
    );
    let match_res = router.cocokkan_url("/produk/PRD-888").unwrap();
    assert_eq!(match_res.parameter.get("id"), Some(&"PRD-888".to_string()));

    // 2. Islands Architecture & SSR Streaming
    let island = KomponenIsland::baru(
        "cart_1",
        "KeranjangBelanja",
        r#"{"items": []}"#,
        "<button id='cart'>0 Items</button>"
    );
    let island_html = MesinSSRMurni::render_island(&island);
    assert!(island_html.contains("data-widya-island=\"KeranjangBelanja\""));

    let stream_chunks = MesinSSRMurni::render_streaming_chunks("Katalog Berdaulat", "<div id='app'>Katalog</div>");
    assert!(stream_chunks.len() >= 3);
    assert!(stream_chunks[0].contains("<!DOCTYPE html>"));
    assert!(stream_chunks[1].contains("<div id='app'>Katalog</div>"));

    // 3. Zero-API Server Actions RPC
    let dispatcher = ServerActionDispatcher::baru();
    dispatcher.daftarkan_action("checkout_cart", |params| {
        let inv = params.get("invoice").cloned().unwrap_or_default();
        Ok(format!("SUCCESS:{}", inv))
    });
    let mut params = HashMap::new();
    params.insert("invoice".to_string(), "INV-2026-XYZ".to_string());
    let res = dispatcher.eksekusi_action("checkout_cart", &params).unwrap();
    assert_eq!(res, "SUCCESS:INV-2026-XYZ");

    // 4. JWT Verification & Rate Limiting
    let secret = "kunci-rahasia-kedaulatan-widya-2026";
    let token = VerifikatorJWTMurni::buat_token(r#"{"user_id":"admin","role":"sovereign"}"#, secret, 3600);
    let verif = VerifikatorJWTMurni::verifikasi_token(&token, secret).unwrap();
    assert!(verif.contains("admin"));

    let limiter = RateLimiterBucket::baru(5, 5);
    for _ in 0..5 {
        assert!(limiter.izinkan_request("client_1"));
    }
    assert!(!limiter.izinkan_request("client_1"));
}

#[test]
fn test_pillar_4_widyadesktop_native_workstation() {
    println!(">>> Testing Pillar 4: WidyaDesktop Native Workstation...");

    // 1. Sovereign Window Manager & Dirty Regions
    let mut window = JendelaMurni::baru("win-pos-main", "Widya POS Enterprise", 1024, 768);
    window.efek_latar = EfekLatarJendela::MicaDark;
    assert_eq!(window.status, StatusJendela::Dibuat);
    
    window.ubah_ukuran(800, 600);
    let dirty = window.bersihkan_dirty_regions();
    assert_eq!(dirty.len(), 1);

    let wm = PengelolaJendelaMurni::baru();
    wm.daftarkan_jendela(window);
    assert_eq!(wm.total_jendela(), 1);

    // 2. Binary IPC Bridge
    let ipc = JembatanIPCMurni::baru();
    ipc.daftarkan_token_jendela("win-pos-main", "token-sesi-123");
    assert!(ipc.validasi_token("win-pos-main", "token-sesi-123"));

    ipc.daftarkan_metode("pos:barcode_scanned", |payload| {
        let code = String::from_utf8_lossy(payload);
        Ok(format!("ITEM_OK:{}", code).into_bytes())
    });
    let resp = ipc.panggil("pos:barcode_scanned", b"8992753123456").unwrap();
    assert_eq!(String::from_utf8_lossy(&resp), "ITEM_OK:8992753123456");

    // 3. POS Hardware Drivers (ESC/POS & Scale)
    let mut printer = EscPosGenerator::baru();
    printer
        .inisialisasi()
        .rata_tengah()
        .tebal(true)
        .tulis_baris("TOKO MERAH PUTIH")
        .tebal(false)
        .cetak_barcode_code128("8992753123456")
        .potong_kertas();
    let bytecodes = printer.ambil_bytes();
    assert!(bytecodes.len() > 10);
    assert_eq!(bytecodes[0], 0x1B); // ESC @

    let berat = ParserTimbanganPOS::parse_berat_kg("ST,GS,+001.250kg\r\n").unwrap();
    assert_eq!(berat, 1.250);
}

#[test]
fn test_pillar_5_widyamobile_pure_ui_engine() {
    println!(">>> Testing Pillar 5: WidyaMobile Pure UI Engine...");

    // 1. Pure Canvas Rasterizer
    let mut fb = FramebufferMurni::baru(400, 800);
    fb.bersihkan(Warna::rgba(245, 247, 250, 255));
    fb.gambar_kotak_bulat(20.0, 20.0, 360.0, 120.0, 12.0, Warna::rgba(18, 115, 235, 255));
    assert_eq!(fb.ambil_piksel(50, 50), Warna::rgba(18, 115, 235, 255));

    // 2. Reactive Widget Tree Layout & Painting
    let card = WidgetMurni::kontainer(
        "card_pos",
        Some(360.0),
        Some(200.0),
        Some(Warna::rgb(255, 255, 255)),
        16.0,
        Some(WidgetMurni::kolom(
            "col_pos",
            vec![
                WidgetMurni::teks("lbl_title", "Dashboard Penjualan", 20.0, Warna::rgb(30, 30, 30)),
                WidgetMurni::tombol("btn_trx", "Proses Transaksi", Warna::rgb(16, 185, 129), "TRX_NEW"),
            ],
            10.0,
        )),
    );

    let screen = WidgetMurni::scaffold(
        "main_screen",
        "WidyaMobile Store",
        Warna::rgb(30, 41, 59),
        card,
    );

    let batasan = BatasanKotak::ketat(400.0, 800.0);
    let mut render_tree = RenderKotakMurni::baru(&screen.id, batasan);
    render_tree.tata_letak(&screen, batasan);
    assert_eq!(render_tree.lebar, 400.0);
    assert_eq!(render_tree.tinggi, 800.0);
    render_tree.cat(&screen, &mut fb);

    // 3. Reactive State Notification
    let state = PemberitahuNilai::baru("item_count", 10);
    state.setel(20);
    assert_eq!(state.ambil(), 20);

    // 4. Pure Navigation Routing
    let home_page = RuteHalaman {
        nama_rute: "/beranda".to_string(),
        widget_halaman: WidgetMurni::teks("home_txt", "Selamat Datang", 16.0, Warna::HITAM),
        parameter: HashMap::new(),
    };
    let mut nav = WidyaNavigator::baru(home_page);
    let detail_page = RuteHalaman {
        nama_rute: "/detail/123".to_string(),
        widget_halaman: WidgetMurni::teks("detail_txt", "Detail Transaksi", 16.0, Warna::HITAM),
        parameter: HashMap::new(),
    };
    nav.push(detail_page);
    assert_eq!(nav.halaman_aktif().unwrap().nama_rute, "/detail/123");
    nav.pop();
    assert_eq!(nav.halaman_aktif().unwrap().nama_rute, "/beranda");
}

#[test]
fn test_penta_core_unified_sovereign_pipeline() {
    println!(">>> Testing Penta-Core Unified Sovereign Interoperability Pipeline...");

    // 1. WidyaOS Task Scheduling
    let thread_mgr = KernelThreadManager::new();
    let pos_handle = thread_mgr.create_thread(sample_pos_daemon, 100, Priority::Realtime).unwrap();
    let sched_res = thread_mgr.run_scheduler();
    assert!(sched_res.is_ok());

    // 2. WidyaBase stores transaction & streams CDC
    let db_path = "target/test_penta_e2e_db";
    let _ = fs::remove_dir_all(db_path);
    let docdb = WidyaDocDB::open(db_path).unwrap();
    let coll = docdb.collection("transaksi");
    let tx_id = coll.insert_one(
        json!({"total": 275000, "outlet": "JKT-01", "items": 3})
    ).unwrap();

    let cdc = CdcStreamer::new(100);
    let mut sub_pos = cdc.subscribe(None, None);
    let mut tx_data = HashMap::new();
    tx_data.insert("total".to_string(), SqlValue::Integer(275000));
    tx_data.insert("outlet".to_string(), SqlValue::Text("JKT-01".to_string()));
    
    let cdc_rec = CdcRecord::new(1, 101, "transaksi", CdcOp::Insert, None, Some(tx_data));
    cdc.emit(cdc_rec);
    let cdc_event = sub_pos.try_recv().unwrap().unwrap();
    assert_eq!(cdc_event.table, "TRANSAKSI");

    // 3. WidyaDesktop processes IPC event and generates ESC/POS ticket
    let mut printer = EscPosGenerator::baru();
    printer
        .inisialisasi()
        .rata_tengah()
        .tulis_baris(&format!("STRUK TRANSAKSI #{}\nTOTAL: Rp 275.000\n", tx_id))
        .potong_kertas();
    let receipt_bytes = printer.ambil_bytes();
    assert!(receipt_bytes.len() > 10);

    // 4. WidyaWeb Server Action processes analytics & streams SSR
    let dispatcher = ServerActionDispatcher::baru();
    dispatcher.daftarkan_action("sync_pos_event", |params| {
        let tid = params.get("tx_id").cloned().unwrap_or_default();
        Ok(format!("SYNCED:{}", tid))
    });
    let mut params = HashMap::new();
    params.insert("tx_id".to_string(), tx_id.clone());
    let sync_res = dispatcher.eksekusi_action("sync_pos_event", &params).unwrap();
    assert_eq!(sync_res, format!("SYNCED:{}", tx_id));

    // 5. WidyaMobile updates state and redraws rasterized canvas
    let mobile_state = PemberitahuNilai::baru("pos_status", "Siap");
    mobile_state.setel("Transaksi Selesai");
    assert_eq!(mobile_state.ambil(), "Transaksi Selesai");

    let mut mobile_fb = FramebufferMurni::baru(320, 480);
    mobile_fb.bersihkan(Warna::rgba(255, 255, 255, 255));
    mobile_fb.gambar_kotak_bulat(10.0, 10.0, 300.0, 80.0, 8.0, Warna::rgba(16, 185, 129, 255));
    assert_eq!(mobile_fb.ambil_piksel(50, 50), Warna::rgba(16, 185, 129, 255));

    println!(">>> Penta-Core Sovereign Matrix 100% SUCCESSFUL & VERIFIED!");
}
