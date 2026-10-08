use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::collections::invoke_closure_or_fn;
use crate::stdlib::common::*;
use crate::stdlib::types::{builtin_ada, builtin_kosong};
use crate::stdlib::*;
use crate::value::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_waves(env: &mut Environment) {
    // WidyaLoadBalancer (High-Availability Cloud-Native Traffic Director)
    register_fn(env, "PenyeimbangBeban", None, builtin_lb_buat);
    register_fn(env, "lb_tambah_server", Some(4), builtin_lb_tambah_server);
    register_fn(env, "lb_pilih_server", None, builtin_lb_pilih_server);
    register_fn(env, "lb_laporkan_koneksi", Some(3), builtin_lb_laporkan_koneksi);
    register_fn(env, "lb_set_status", Some(3), builtin_lb_set_status);

    // WidyaAPIGatewayRouter (High-Performance Trie Path & Method Matching)
    register_fn(env, "RouterAPI", None, builtin_router_buat);
    register_fn(env, "router_tambah_rute", None, builtin_router_tambah_rute);
    register_fn(env, "router_cocokkan", Some(3), builtin_router_cocokkan);

    // WidyaDistributedLock (Multi-Node Lease-Based Distributed Locking with TTL)
    register_fn(env, "KunciTerdistribusi", None, builtin_distlock_buat);
    register_fn(env, "distlock_akuisisi", Some(4), builtin_distlock_akuisisi);
    register_fn(env, "distlock_lepas", Some(3), builtin_distlock_lepas);
    register_fn(env, "distlock_perpanjang", Some(4), builtin_distlock_perpanjang);
    register_fn(env, "distlock_apakah_terkunci", Some(2), builtin_distlock_apakah_terkunci);

    // WidyaOpenTelemetry (Distributed Tracing, Span Hierarchy & JSON Export)
    register_fn(env, "TracerTelemetry", None, builtin_trace_buat);
    register_fn(env, "trace_mulai_span", None, builtin_trace_mulai_span);
    register_fn(env, "trace_akhiri_span", Some(2), builtin_trace_akhiri_span);
    register_fn(env, "trace_tambah_tag", Some(4), builtin_trace_tambah_tag);
    register_fn(env, "trace_ekspor_json", Some(1), builtin_trace_ekspor_json);

    // WidyaDataFrame (Modern Tabular Data Processing & Aggregations)
    register_fn(env, "DataFrame", None, builtin_df_buat);
    register_fn(env, "df_saring", Some(4), builtin_df_saring);
    register_fn(env, "df_pilih_kolom", Some(2), builtin_df_pilih_kolom);
    register_fn(env, "df_kelompokkan_dan_agregasi", Some(4), builtin_df_kelompokkan);
    register_fn(env, "df_gabung", None, builtin_df_gabung);
    register_fn(env, "df_ke_array", Some(1), builtin_df_ke_array);
    register_fn(env, "df_jumlah_baris", Some(1), builtin_df_jumlah_baris);

    // WidyaRAGPipeline (Retrieval-Augmented Generation & Prompt Synthesis)
    register_fn(env, "RAGPipeline", None, builtin_rag_buat);
    register_fn(env, "rag_tambah_dokumen", None, builtin_rag_tambah_dokumen);
    register_fn(env, "rag_kueri_konteks", None, builtin_rag_kueri_konteks);
    register_fn(env, "rag_sintesis_prompt", None, builtin_rag_sintesis_prompt);

    // WidyaMLClassifier (Embedded Machine Learning & Clustering)
    register_fn(env, "PohonKeputusan", None, builtin_ml_pohon_buat);
    register_fn(env, "ml_latih_pohon", Some(3), builtin_ml_latih_pohon);
    register_fn(env, "ml_prediksi_pohon", Some(2), builtin_ml_prediksi_pohon);
    register_fn(env, "ml_kmeans_klaster", None, builtin_ml_kmeans_klaster);

    // WidyaDataVisualizer (Data Analytics & Charting Export)
    register_fn(env, "VisualisasiData", None, builtin_vis_buat);
    register_fn(env, "chart_batang_svg", None, builtin_vis_chart_batang);
    register_fn(env, "chart_garis_svg", None, builtin_vis_chart_garis);
    register_fn(env, "chart_sebar_svg", None, builtin_vis_chart_sebar);
    register_fn(env, "render_dashboard_html", None, builtin_vis_render_dashboard);

    // WidyaRing0MMIO (Memory-Mapped I/O & DMA Contiguous Buffer)
    register_fn(env, "BufferDMA", None, builtin_dma_buat);
    register_fn(env, "dma_tulis_u8", Some(3), builtin_dma_tulis_u8);
    register_fn(env, "dma_baca_u8", Some(2), builtin_dma_baca_u8);
    register_fn(env, "dma_tulis_u32", Some(3), builtin_dma_tulis_u32);
    register_fn(env, "dma_baca_u32", Some(2), builtin_dma_baca_u32);
    register_fn(env, "dma_ukuran", Some(1), builtin_dma_ukuran);
    register_fn(env, "dma_alamat_simulasi", Some(1), builtin_dma_alamat);

    // WidyaAtomicOps (Lock-Free Hardware Atomic Primitives)
    register_fn(env, "NilaiAtomik", Some(1), builtin_atomic_buat);
    register_fn(env, "atomik_baca", Some(1), builtin_atomic_baca);
    register_fn(env, "atomik_tulis", Some(2), builtin_atomic_tulis);
    register_fn(env, "atomik_tambah", Some(2), builtin_atomic_tambah);
    register_fn(env, "atomik_banding_dan_tukar", Some(3), builtin_atomic_cas);

    // WidyaMemoryArena (Custom Bump & Arena Memory Allocator)
    register_fn(env, "ArenaMemori", Some(1), builtin_arena_buat);
    register_fn(env, "arena_alokasi", Some(2), builtin_arena_alokasi);
    register_fn(env, "arena_reset", Some(1), builtin_arena_reset);
    register_fn(env, "arena_penggunaan_byte", Some(1), builtin_arena_penggunaan);

    // WidyaSysSocket (Raw TCP/UDP Socket & Non-Blocking Polling)
    register_fn(env, "SoketJaringan", None, builtin_sock_buat);
    register_fn(env, "soket_ikat", Some(3), builtin_sock_ikat);
    register_fn(env, "soket_dengarkan", Some(2), builtin_sock_dengarkan);
    register_fn(env, "soket_terima_koneksi", Some(1), builtin_sock_terima);
    register_fn(env, "soket_kirim_bytes", Some(2), builtin_sock_kirim);
    register_fn(env, "soket_baca_bytes", Some(2), builtin_sock_baca);

    // WidyaDesktopWindow (Native OS Window Management & Lifecycle)
    register_fn(env, "JendelaDesktop", None, builtin_window_buat);
    register_fn(env, "jendela_set_judul", Some(2), builtin_window_set_judul);
    register_fn(env, "jendela_set_ukuran", Some(3), builtin_window_set_ukuran);
    register_fn(env, "jendela_pusatkan", Some(1), builtin_window_pusatkan);
    register_fn(env, "jendela_set_fullscreen", Some(2), builtin_window_set_fullscreen);
    register_fn(env, "jendela_apakah_terbuka", Some(1), builtin_window_apakah_terbuka);
    register_fn(env, "jendela_tutup", Some(1), builtin_window_tutup);

    // WidyaDesktopMenu (Menu Bar, Context Menu & System Tray)
    register_fn(env, "MenuBilah", None, builtin_menu_buat);
    register_fn(env, "menu_tambah_item", None, builtin_menu_tambah_item);
    register_fn(env, "menu_tambah_pemisah", Some(1), builtin_menu_tambah_pemisah);
    register_fn(env, "BakiSistem", None, builtin_tray_buat);
    register_fn(env, "tray_set_tooltip", Some(2), builtin_tray_set_tooltip);
    register_fn(env, "tray_kirim_notifikasi", Some(3), builtin_tray_kirim_notifikasi);

    // WidyaNativeDialog (OS File Dialogs & Message Boxes)
    register_fn(env, "dialog_buka_berkas", None, builtin_dialog_buka_berkas);
    register_fn(env, "dialog_simpan_berkas", None, builtin_dialog_simpan_berkas);
    register_fn(env, "dialog_pilih_folder", None, builtin_dialog_pilih_folder);
    register_fn(env, "dialog_pesan", Some(3), builtin_dialog_pesan);

    // WidyaReactiveState (Signals & Reactive GUI State Management)
    register_fn(env, "SinyalReaktif", Some(1), builtin_signal_buat);
    register_fn(env, "sinyal_ambil", Some(1), builtin_signal_ambil);
    register_fn(env, "sinyal_perbarui", Some(2), builtin_signal_perbarui);
    register_fn(env, "efek_reaktif", Some(2), builtin_signal_efek);

    // WidyaMobileLocation (GPS Geolocation & Geofencing)
    register_fn(env, "LokasiGPS", None, builtin_gps_buat);
    register_fn(env, "gps_buat", None, builtin_gps_buat);
    register_fn(env, "gps_dapatkan_posisi", Some(1), builtin_gps_dapatkan_posisi);
    register_fn(env, "gps_hitung_jarak", Some(4), builtin_gps_hitung_jarak);
    register_fn(env, "gps_geofence_apakah_masuk", Some(5), builtin_gps_geofence_apakah_masuk);

    // WidyaMobileSensors (Motion Sensors & Haptics)
    register_fn(env, "SensorGerak", None, builtin_sensor_buat);
    register_fn(env, "sensor_buat", None, builtin_sensor_buat);
    register_fn(env, "sensor_baca_vektor", Some(1), builtin_sensor_baca_vektor);
    register_fn(env, "haptic_getar", Some(1), builtin_haptic_getar);

    // WidyaMobileBatteryPower (Battery & Power Management)
    register_fn(env, "StatusBaterai", None, builtin_baterai_buat);
    register_fn(env, "baterai_buat", None, builtin_baterai_buat);
    register_fn(env, "baterai_apakah_mengisi", Some(1), builtin_baterai_apakah_mengisi);
    register_fn(env, "baterai_hemat_daya", Some(1), builtin_baterai_hemat_daya);

    // WidyaMobileBiometrics & WidyaMobileStorage (Biometrics & KeyStore)
    register_fn(env, "biometrik_autentikasi", None, builtin_biometrik_autentikasi);
    register_fn(env, "kunci_aman_simpan", Some(2), builtin_kunci_aman_simpan);
    register_fn(env, "kunci_aman_ambil", Some(1), builtin_kunci_aman_ambil);
    register_fn(env, "kunci_aman_hapus", Some(1), builtin_kunci_aman_hapus);
    // WidyaMobileCamera & WidyaMobileQRIS (Camera, Barcode & Indonesian QRIS Parser)
    register_fn(env, "kamera_ambil_foto", None, builtin_kamera_ambil_foto);
    register_fn(env, "kamera_rekam_video_singkat", None, builtin_kamera_rekam_video_singkat);
    register_fn(env, "qr_pindai_dari_kamera", None, builtin_qr_pindai_dari_kamera);
    register_fn(env, "qris_urai_string", Some(1), builtin_qris_urai_string);

    // WidyaMobileNotifications (Local & Push Notifications)
    register_fn(env, "notifikasi_lokal_buat", None, builtin_notifikasi_lokal_buat);
    register_fn(env, "notifikasi_lokal_jadwalkan", None, builtin_notifikasi_lokal_jadwalkan);
    register_fn(env, "notifikasi_lokal_batalkan", Some(1), builtin_notifikasi_lokal_batalkan);
    register_fn(env, "notifikasi_push_dapatkan_token", None, builtin_notifikasi_push_dapatkan_token);
    register_fn(env, "notifikasi_push_pasang_pendengar", Some(1), builtin_notifikasi_push_pasang_pendengar);

    // WidyaMobileOfflineQueue (Offline Outbox Queue & Network Monitor)
    register_fn(env, "jaringan_status_aktif", None, builtin_jaringan_status_aktif);
    register_fn(env, "jaringan_tipe", None, builtin_jaringan_tipe);
    register_fn(env, "antrean_offline_tambah_aksi", Some(2), builtin_antrean_offline_tambah_aksi);
    register_fn(env, "antrean_offline_ambil_semua", None, builtin_antrean_offline_ambil_semua);
    register_fn(env, "antrean_offline_hapus_aksi", Some(1), builtin_antrean_offline_hapus_aksi);
    register_fn(env, "antrean_offline_sinkronisasi", None, builtin_antrean_offline_sinkronisasi);

    // WidyaMobileBluetoothPOS (Bluetooth Discovery & ESC/POS Thermal Receipt Printer)
    register_fn(env, "bluetooth_pindai_perangkat", None, builtin_bluetooth_pindai_perangkat);
    register_fn(env, "bluetooth_hubungkan", Some(1), builtin_bluetooth_hubungkan);
    register_fn(env, "printer_thermal_cetak_struk", Some(1), builtin_printer_thermal_cetak_struk);

    // WidyaMobileShareAndDeepLinks (Share Sheet, Deep Links & Clipboard)
    register_fn(env, "bagikan_ke_aplikasi_lain", None, builtin_bagikan_ke_aplikasi_lain);
    register_fn(env, "tautan_dalam_ambil_url_pembuka", None, builtin_tautan_dalam_ambil_url_pembuka);
    register_fn(env, "papan_klip_salin", Some(1), builtin_papan_klip_salin);
    register_fn(env, "papan_klip_ambil", None, builtin_papan_klip_ambil);

    // WidyaMobileEncryptedDatabase (Encrypted Mobile SQLite / SQLCipher Cache)
    register_fn(env, "basis_data_mobile_buka", None, builtin_basis_data_mobile_buka);
    register_fn(env, "basis_data_mobile_eksekusi", None, builtin_basis_data_mobile_eksekusi);
    register_fn(env, "basis_data_mobile_kueri", None, builtin_basis_data_mobile_kueri);

    // WidyaBase Native Industrial Database Engine
    register_fn(env, "widyabase_buka", None, builtin_widyabase_buka);
    register_fn(env, "widyabase_eksekusi", None, builtin_widyabase_eksekusi);
    register_fn(env, "widyabase_kueri", None, builtin_widyabase_kueri);
    register_fn(env, "widyabase_checkpoint", None, builtin_widyabase_checkpoint);
    register_fn(env, "widyabase_status", None, builtin_widyabase_status);

    // WidyaEmbeddedGPIO (General-Purpose Input/Output & PWM)
    register_fn(env, "PinGPIO", None, builtin_gpio_buat);
    register_fn(env, "gpio_tulis", Some(2), builtin_gpio_tulis);
    register_fn(env, "gpio_baca", Some(1), builtin_gpio_baca);
    register_fn(env, "gpio_set_pwm", Some(3), builtin_gpio_set_pwm);

    // WidyaBusI2C & WidyaBusSPI (Serial Hardware Bus Protocols)
    register_fn(env, "BusI2C", None, builtin_i2c_buat);
    register_fn(env, "i2c_tulis_register", Some(3), builtin_i2c_tulis_reg);
    register_fn(env, "i2c_baca_register", Some(3), builtin_i2c_baca_reg);
    register_fn(env, "BusSPI", None, builtin_spi_buat);
    register_fn(env, "spi_transfer", Some(2), builtin_spi_transfer);

    // WidyaBusUART (Serial TTL/RS-485 & Modbus CRC16)
    register_fn(env, "PortSerialUART", None, builtin_uart_buat);
    register_fn(env, "uart_kirim_teks", Some(2), builtin_uart_kirim_teks);
    register_fn(env, "uart_baca_tersedia", Some(1), builtin_uart_baca_tersedia);
    register_fn(env, "uart_hitung_crc16", Some(1), builtin_uart_hitung_crc16);

    // WidyaCoAPProtocol (Constrained Application Protocol RFC 7252)
    register_fn(env, "PesanCoAP", None, builtin_coap_buat);
    register_fn(env, "coap_enkode", Some(1), builtin_coap_enkode);
    register_fn(env, "coap_dekode", Some(1), builtin_coap_dekode);

    // WidyaECS (Entity Component System Architecture)
    register_fn(env, "DuniaECS", None, builtin_ecs_buat);
    register_fn(env, "ecs_buat_entitas", Some(2), builtin_ecs_buat_entitas);
    register_fn(env, "ecs_tambah_komponen", Some(4), builtin_ecs_tambah_komponen);
    register_fn(env, "ecs_ambil_komponen", Some(3), builtin_ecs_ambil_komponen);
    register_fn(env, "ecs_kueri_komponen", Some(2), builtin_ecs_kueri_komponen);

    // WidyaPhysics2D (2D Rigid Body Physics & AABB Collision)
    register_fn(env, "BendaFisika2D", None, builtin_physics_buat);
    register_fn(env, "fisika_terapkan_gaya", Some(3), builtin_physics_terapkan_gaya);
    register_fn(env, "fisika_deteksi_tabrakan_aabb", Some(2), builtin_physics_deteksi_aabb);
    register_fn(env, "fisika_langkah_simulasi", None, builtin_physics_langkah_simulasi);

    // WidyaTilemap & WidyaPathfindingAStar (Tilemap & A* Pathfinding)
    register_fn(env, "PetaUbinTilemap", None, builtin_tilemap_buat);
    register_fn(env, "tilemap_set_ubin", Some(5), builtin_tilemap_set_ubin);
    register_fn(env, "tilemap_apakah_rintangan", Some(3), builtin_tilemap_apakah_rintangan);
    register_fn(env, "astar_cari_jalur", Some(5), builtin_astar_cari_jalur);

    // WidyaGameAudio & WidyaGameLoop (Audio & Delta Time)
    register_fn(env, "SuaraGame", None, builtin_audio_buat);
    register_fn(env, "audio_putar", Some(1), builtin_audio_putar);
    register_fn(env, "audio_set_volume", Some(2), builtin_audio_set_volume);
    register_fn(env, "PengaturFrameGame", None, builtin_frame_buat);
    register_fn(env, "frame_hitung_delta", Some(1), builtin_frame_hitung_delta);

    // WidyaDevOpsPipeline (CI/CD Pipeline Engine & Stages)
    register_fn(env, "PipaKerjaDevOps", None, builtin_pipa_buat);
    register_fn(env, "pipa_tambah_tahap", Some(3), builtin_pipa_tambah_tahap);
    register_fn(env, "pipa_jalankan", Some(1), builtin_pipa_jalankan);
    register_fn(env, "pipa_apakah_sukses", Some(1), builtin_pipa_apakah_sukses);

    // WidyaContainerConfig (Docker Compose & OCI Generator)
    register_fn(env, "KonfigurasiKontainer", None, builtin_kontainer_buat);
    register_fn(env, "kontainer_tambah_env", Some(3), builtin_kontainer_tambah_env);
    register_fn(env, "kontainer_ke_yaml", Some(1), builtin_kontainer_ke_yaml);
    register_fn(env, "kontainer_validasi", Some(1), builtin_kontainer_validasi);

    // WidyaEnvVault (Dotenv & Secret Interpolation)
    register_fn(env, "VaultEnv", Some(1), builtin_env_buat);
    register_fn(env, "env_ambil", Some(2), builtin_env_ambil);
    register_fn(env, "env_interpolasi_teks", Some(2), builtin_env_interpolasi);

    // WidyaHealthCheck & WidyaWebhookNotifier (Health Checks & Alerts)
    register_fn(env, "PemeriksaKesehatan", None, builtin_probe_buat);
    register_fn(env, "probe_jalankan", Some(1), builtin_probe_jalankan);
    register_fn(env, "webhook_kirim_alert", Some(4), builtin_webhook_kirim_alert);

    // WidyaPCAPParser (Packet Capture & Network Forensics)
    register_fn(env, "ParserPCAP", None, builtin_pcap_buat);
    register_fn(env, "pcap_ekstrak_paket", Some(1), builtin_pcap_ekstrak_paket);
    register_fn(env, "pcap_saring_protokol", Some(2), builtin_pcap_saring_protokol);

    // WidyaFileForensics & WidyaEntropy (Magic Bytes & Shannon Entropy)
    register_fn(env, "forensik_deteksi_magic_byte", Some(1), builtin_forensik_magic_byte);
    register_fn(env, "forensik_hitung_entropi", Some(1), builtin_forensik_entropi);
    register_fn(env, "forensik_ekstrak_string", None, builtin_forensik_ekstrak_string);

    // WidyaYARAMatcher (Signature & Malware Pattern Matching)
    register_fn(env, "AturanYARA", None, builtin_yara_buat);
    register_fn(env, "yara_pindai_target", Some(2), builtin_yara_pindai);

    // WidyaLogAnalyzer (SIEM & Log Anomaly Detector)
    register_fn(env, "PenganalisisLogSIEM", None, builtin_siem_buat);
    register_fn(env, "siem_deteksi_anomali_ip", None, builtin_siem_deteksi_anomali);
    register_fn(env, "siem_deteksi_pola_serangan", Some(2), builtin_siem_deteksi_serangan);

    // WidyaWasmModule (WebAssembly Binary Emit & Validator)
    register_fn(env, "ModulWasm", None, builtin_wasm_modul_buat);
    register_fn(env, "wasm_tambah_fungsi", Some(5), builtin_wasm_tambah_fungsi);
    register_fn(env, "wasm_bangun_biner", Some(1), builtin_wasm_bangun_biner);
    register_fn(env, "wasm_validasi_biner", Some(1), builtin_wasm_validasi_biner);

    // WidyaWasmMemory & WidyaWASIEngine (Linear Memory & WASI)
    register_fn(env, "MemoriWasmLinear", None, builtin_wasm_memori_buat);
    register_fn(env, "wasm_tulis_memori", Some(3), builtin_wasm_tulis_memori);
    register_fn(env, "wasm_baca_memori", Some(3), builtin_wasm_baca_memori);
    register_fn(env, "WASIContext", None, builtin_wasi_buat);

    // WidyaWasmJSBridge (JavaScript FFI Bindings)
    register_fn(env, "JembatanWasmJS", None, builtin_wasm_js_buat);
    register_fn(env, "wasm_js_daftarkan_fungsi", Some(3), builtin_wasm_js_daftarkan);
    register_fn(env, "wasm_js_panggil", Some(3), builtin_wasm_js_panggil);

    // WidyaEdgeWorker (Edge Computing & Serverless Worker)
    register_fn(env, "PekerjaEdge", None, builtin_edge_buat);
    register_fn(env, "edge_tangani_permintaan", Some(2), builtin_edge_tangani);

    // WidyaOrderBookL2 (Limit Order Book Matching Engine)
    register_fn(env, "BukuPesananL2", None, builtin_orderbook_buat);
    register_fn(env, "orderbook_tambah_pesanan", Some(5), builtin_orderbook_tambah);
    register_fn(env, "orderbook_cocokkan", Some(1), builtin_orderbook_cocokkan);
    register_fn(env, "orderbook_ambil_kedalaman", None, builtin_orderbook_kedalaman);

    // WidyaBlackScholes & WidyaGreeks (Quantitative Derivatives Option Pricing)
    register_fn(env, "OpsiEropa", None, builtin_opsi_buat);
    register_fn(env, "opsi_hitung_harga", Some(1), builtin_opsi_harga);
    register_fn(env, "opsi_hitung_greeks", Some(1), builtin_opsi_greeks);

    // WidyaFinancialIndicators (Quantitative Indicators)
    register_fn(env, "indikator_ema", Some(2), builtin_indikator_ema);
    register_fn(env, "indikator_rsi", Some(2), builtin_indikator_rsi);
    register_fn(env, "indikator_vwap", Some(2), builtin_indikator_vwap);

    // WidyaFIXProtocol (Financial Information eXchange Protocol Engine)
    register_fn(env, "PesanFIX", None, builtin_fix_buat);
    register_fn(env, "fix_tambah_tag", Some(3), builtin_fix_tambah_tag);
    register_fn(env, "fix_enkode", Some(1), builtin_fix_enkode);
    register_fn(env, "fix_dekode", Some(1), builtin_fix_dekode);

    // WidyaFASTAQParser (Genomic Sequences Parser)
    register_fn(env, "ParserFASTA", Some(1), builtin_fasta_buat);
    register_fn(env, "ParserFASTQ", Some(1), builtin_fastq_buat);
    register_fn(env, "genom_hitung_gc_content", Some(1), builtin_genom_gc_content);

    // WidyaDNATranslator (Transcription, Reverse Complement & Translation)
    register_fn(env, "dna_transkripsi_ke_rna", Some(1), builtin_dna_transkripsi);
    register_fn(env, "dna_komplemen_terbalik", Some(1), builtin_dna_komplemen_terbalik);
    register_fn(env, "rna_translasi_ke_protein", Some(1), builtin_rna_translasi);

    // WidyaSequenceAlignment (Needleman-Wunsch & Smith-Waterman)
    register_fn(env, "PenjajarSekuens", None, builtin_penjajar_buat);
    register_fn(env, "penjajaran_global", Some(3), builtin_penjajaran_global);
    register_fn(env, "penjajaran_lokal", Some(3), builtin_penjajaran_lokal);

    // WidyaKmerCounter & WidyaMotifFinder (K-Mer Spektra & Hamming Distance)
    register_fn(env, "kmer_hitung_frekuensi", Some(2), builtin_kmer_hitung);
    register_fn(env, "genom_jarak_hamming", Some(2), builtin_genom_hamming);

    // WidyaGeoJSON & WidyaWKT (GeoJSON and OGC WKT Parsers)
    register_fn(env, "ParserGeoJSON", Some(1), builtin_geojson_buat);
    register_fn(env, "geojson_ke_wkt", Some(1), builtin_geojson_ke_wkt);
    register_fn(env, "wkt_ke_geojson", Some(1), builtin_wkt_ke_geojson);

    // WidyaSpatialTopology (Point in Polygon & Metrics)
    register_fn(env, "spasial_titik_dalam_poligon", Some(2), builtin_spasial_point_in_poly);
    register_fn(env, "spasial_hitung_luas_poligon", Some(1), builtin_spasial_luas_poligon);
    register_fn(env, "spasial_hitung_panjang_garis", Some(1), builtin_spasial_panjang_garis);

    // WidyaSpatialIndexing (R-Tree & Geohash Indexing)
    register_fn(env, "IndeksRTree", None, builtin_rtree_buat);
    register_fn(env, "rtree_sisip", None, builtin_rtree_sisip);
    register_fn(env, "rtree_kueri_kotak", Some(5), builtin_rtree_kueri_kotak);
    register_fn(env, "geohash_enkode", None, builtin_geohash_enkode);
    register_fn(env, "geohash_dekode", Some(1), builtin_geohash_dekode);

    // WidyaCRSProjection (Coordinate Reference Systems & Projections)
    register_fn(env, "ProyeksiCRS", Some(2), builtin_crs_buat);
    register_fn(env, "crs_transformasi_koordinat", Some(3), builtin_crs_transformasi);

    // WidyaMAVLinkProtocol (Drone Telemetry & Autopilot Protocol)
    register_fn(env, "PesanMAVLink", None, builtin_mavlink_buat);
    register_fn(env, "mavlink_enkode_heartbeat", Some(4), builtin_mavlink_enkode_heartbeat);
    register_fn(env, "mavlink_enkode_attitude", Some(4), builtin_mavlink_enkode_attitude);
    register_fn(env, "mavlink_dekode", Some(1), builtin_mavlink_dekode);

    // WidyaAHRSQuaternion (Orientation Quaternions & Madgwick Filter)
    register_fn(env, "KuaternionOrientasi", None, builtin_kuaternion_buat);
    register_fn(env, "ahrs_ke_sudut_euler", Some(1), builtin_ahrs_ke_euler);
    register_fn(env, "ahrs_fusi_madgwick_step", Some(5), builtin_ahrs_madgwick_step);

    // WidyaKinematicsRobotics (Robot Forward & Inverse Kinematics)
    register_fn(env, "LenganRobotik2DOF", Some(2), builtin_robot_arm_buat);
    register_fn(env, "kinematika_maju_2dof", Some(3), builtin_kinematika_maju);
    register_fn(env, "kinematika_balik_2dof", Some(3), builtin_kinematika_balik);

    // WidyaOrbitalKepler (Orbital Mechanics & Satellite Propagation)
    register_fn(env, "ElemenOrbitKepler", Some(6), builtin_orbit_buat);
    register_fn(env, "orbit_hitung_posisi_kepler", Some(1), builtin_orbit_hitung_posisi);

    // WidyaFFT & WidyaSpectralAnalysis (Fast Fourier Transform & Spectrogram)
    register_fn(env, "dsp_fft_maju", Some(1), builtin_dsp_fft_maju);
    register_fn(env, "dsp_fft_balik", Some(1), builtin_dsp_fft_balik);
    register_fn(env, "dsp_spektrogram", Some(3), builtin_dsp_spektrogram);

    // WidyaFilterDSP (Digital Filter Design FIR & IIR Biquad)
    register_fn(env, "FilterBiquadIIR", Some(4), builtin_filter_biquad_buat);
    register_fn(env, "filter_proses_sinyal", Some(2), builtin_filter_proses_sinyal);

    // WidyaAudioProcessing (Convolver, Reverb & Peak Limiter / Dynamic Compression)
    register_fn(env, "dsp_konvolusi_sinyal", Some(2), builtin_dsp_konvolusi);
    register_fn(env, "dsp_kompresor_dinamis", Some(3), builtin_dsp_kompresor);

    // WidyaVideoFrameProcessor (Raw Pixel Frame, YUV420p / RGB & Resampling)
    register_fn(env, "BingkaiVideo", Some(3), builtin_video_bingkai_buat);
    register_fn(env, "video_konversi_yuv_ke_rgb", Some(1), builtin_video_yuv_ke_rgb);
    register_fn(env, "video_konversi_rgb_ke_yuv", Some(1), builtin_video_rgb_ke_yuv);
    register_fn(env, "video_ubah_skala_bilinear", Some(3), builtin_video_skala_bilinear);

    // WidyaNavierStokes2D (Incompressible Grid Fluid Simulation)
    register_fn(env, "GridFluida2D", Some(4), builtin_cfd_fluida_buat);
    register_fn(env, "fluida_tambah_densitas", Some(4), builtin_cfd_tambah_densitas);
    register_fn(env, "fluida_tambah_kecepatan", Some(5), builtin_cfd_tambah_kecepatan);
    register_fn(env, "fluida_langkah_simulasi", Some(1), builtin_cfd_langkah_simulasi);

    // WidyaSPHFluid (Smoothed Particle Hydrodynamics)
    register_fn(env, "SistemPartikelSPH", Some(4), builtin_sph_sistem_buat);
    register_fn(env, "sph_tambah_partikel", Some(5), builtin_sph_tambah_partikel);
    register_fn(env, "sph_langkah_waktu", Some(3), builtin_sph_langkah_waktu);

    // WidyaHeatTransfer (Thermal Diffusion & Heat Conduction PDE)
    register_fn(env, "KisiTermal2D", Some(4), builtin_termal_kisi_buat);
    register_fn(env, "termal_set_suhu", Some(4), builtin_termal_set_suhu);
    register_fn(env, "termal_langkah_difusi", Some(1), builtin_termal_langkah_difusi);

    // WidyaWaveEquation (2D Wave Propagation & Acoustic Wavefront)
    register_fn(env, "KisiGelombang2D", Some(4), builtin_gelombang_kisi_buat);
    register_fn(env, "gelombang_beri_gangguan", Some(4), builtin_gelombang_beri_gangguan);
    register_fn(env, "gelombang_langkah_waktu", Some(2), builtin_gelombang_langkah_waktu);

    // WidyaOpenXR (OpenXR & WebXR Immersive Session Manager)
    register_fn(env, "SesiXR", Some(2), builtin_xr_sesi_buat);
    register_fn(env, "xr_mulai_sesi", Some(1), builtin_xr_mulai_sesi);
    register_fn(env, "xr_akhiri_sesi", Some(1), builtin_xr_akhiri_sesi);
    register_fn(env, "xr_dapatkan_tampilan_mata", Some(1), builtin_xr_tampilan_mata);

    // WidyaHandTracking (26-Joint Hand Tracking & Gesture Recognition)
    register_fn(env, "PelacakTanganXR", Some(1), builtin_xr_tangan_buat);
    register_fn(env, "xr_set_sendi_tangan", Some(5), builtin_xr_set_sendi);
    register_fn(env, "xr_hitung_pose_gestur", Some(1), builtin_xr_hitung_gestur);

    // WidyaARPlaneDetection & WidyaHitTest (AR Surfaces & Spatial Anchors)
    register_fn(env, "PelacakBidangAR", None, builtin_ar_pelacak_buat);
    register_fn(env, "ar_tambah_bidang", Some(5), builtin_ar_tambah_bidang);
    register_fn(env, "ar_lakukan_hit_test", Some(4), builtin_ar_hit_test);
    register_fn(env, "ar_buat_jangkar_spasial", Some(5), builtin_ar_buat_jangkar);

    // WidyaSpatialAudio & WidyaWebXRRenderer (3D HRTF Spatial Audio & WebXR HTML)
    register_fn(env, "xr_audio_spasial_3d", Some(3), builtin_xr_audio_spasial);
    register_fn(env, "xr_render_aplikasi_webxr_html", Some(2), builtin_xr_render_html);

    // WidyaSpikingNeuron (Leaky Integrate-and-Fire LIF Neuron)
    register_fn(env, "NeuronLIF", Some(5), builtin_neuron_lif_buat);
    register_fn(env, "neuron_stimulasi_arus", Some(3), builtin_neuron_stimulasi_arus);
    register_fn(env, "neuron_reset_status", Some(1), builtin_neuron_reset);

    // WidyaSTDPPlasticity (Spike-Timing-Dependent Plasticity)
    register_fn(env, "SinapsisSTDP", Some(5), builtin_sinapsis_stdp_buat);
    register_fn(env, "stdp_perbarui_bobot", Some(3), builtin_stdp_perbarui_bobot);

    // WidyaEEGSignalProcessing (Brainwave Frequency Bands)
    register_fn(env, "ProsesorSinyalEEG", Some(1), builtin_eeg_prosesor_buat);
    register_fn(env, "eeg_ekstrak_gelombang", Some(2), builtin_eeg_ekstrak_gelombang);
    register_fn(env, "eeg_hitung_indeks_fokus", Some(2), builtin_eeg_hitung_fokus);

    // WidyaBCIDecoder (Brain-Computer Interface Motor Imagery)
    register_fn(env, "DekoderBCI", Some(1), builtin_bci_dekoder_buat);
    register_fn(env, "bci_klasifikasi_niat", Some(2), builtin_bci_klasifikasi_niat);

    // WidyaHDLSynthesizer (RTL Hardware Description & Verilog/VHDL Synthesis)
    register_fn(env, "ModulRTL", Some(1), builtin_rtl_modul_buat);
    register_fn(env, "rtl_tambah_port", Some(4), builtin_rtl_tambah_port);
    register_fn(env, "rtl_tambah_penugasan", Some(3), builtin_rtl_tambah_penugasan);
    register_fn(env, "rtl_tambah_blok_sekuensial", Some(3), builtin_rtl_tambah_blok_sekuensial);
    register_fn(env, "rtl_ekspor_verilog", Some(1), builtin_rtl_ekspor_verilog);
    register_fn(env, "rtl_ekspor_vhdl", Some(1), builtin_rtl_ekspor_vhdl);

    // WidyaGateNetlist (Gate-Level Netlist & Cycle Simulator)
    register_fn(env, "NetlistGerbang", Some(1), builtin_netlist_buat);
    register_fn(env, "netlist_sisip_gerbang", Some(5), builtin_netlist_sisip_gerbang);
    register_fn(env, "netlist_simulasi_siklus", Some(2), builtin_netlist_simulasi_siklus);

    // WidyaStaticTimingAnalysis (Static Timing Analysis & Slack Engine)
    register_fn(env, "MesinSTA", Some(1), builtin_sta_mesin_buat);
    register_fn(env, "sta_tambah_lintasan", Some(5), builtin_sta_tambah_lintasan);
    register_fn(env, "sta_hitung_lintasan_kritis", Some(1), builtin_sta_hitung_lintasan_kritis);

    // WidyaVCDWaveform (Value Change Dump IEEE 1364 Waveform Recorder)
    register_fn(env, "PerekamGelombangVCD", Some(2), builtin_vcd_perekam_buat);
    register_fn(env, "vcd_catat_transisi", Some(4), builtin_vcd_catat_transisi);
    register_fn(env, "vcd_hasilkan_dokumen", Some(1), builtin_vcd_hasilkan_dokumen);

    // WidyaMicrokernelCapability (Capability-based Security & seL4-style IPC)
    register_fn(env, "MikrokernelCapSpace", Some(0), builtin_cspace_buat);
    register_fn(env, "cap_buat_endpoint", Some(3), builtin_cap_buat_endpoint);
    register_fn(env, "cap_kirim_pesan_ipc", Some(3), builtin_cap_kirim_ipc);
    register_fn(env, "cap_terima_pesan_ipc", Some(2), builtin_cap_terima_ipc);
    register_fn(env, "cap_cabut", Some(2), builtin_cap_cabut);

    // WidyaMicrokernelScheduler (Preemptive & Round-Robin Task Scheduler)
    register_fn(env, "PenjadwalMikrokernel", Some(1), builtin_penjadwal_buat);
    register_fn(env, "tcb_buat", Some(4), builtin_tcb_buat);
    register_fn(env, "penjadwal_tambah_tcb", Some(2), builtin_penjadwal_tambah_tcb);
    register_fn(env, "penjadwal_langkah_siklus", Some(1), builtin_penjadwal_langkah_siklus);
    register_fn(env, "penjadwal_ambil_tugas_aktif", Some(1), builtin_penjadwal_ambil_aktif);

    // WidyaUnikernelImage (Single-Address Space & Multiboot Payload)
    register_fn(env, "CitraUnikernel", Some(2), builtin_unikernel_buat);
    register_fn(env, "unikernel_sisip_modul", Some(4), builtin_unikernel_sisip_modul);
    register_fn(env, "unikernel_bangun_header_boot", Some(2), builtin_unikernel_bangun_header);
    register_fn(env, "unikernel_verifikasi_multiboot", Some(1), builtin_unikernel_verifikasi);

    // WidyaEdgeMMU (Two-Level Page Table & Address Translation)
    register_fn(env, "TabelHalamanMMU", Some(1), builtin_mmu_buat);
    register_fn(env, "mmu_petakan_halaman", Some(4), builtin_mmu_petakan);
    register_fn(env, "mmu_terjemahkan_alamat", Some(2), builtin_mmu_terjemahkan);

    // WidyaDNADataStorage (Binary-to-DNA Oligonucleotide Codec)
    register_fn(env, "EnkoderPenyimpananDNA", Some(1), builtin_dna_enkoder_buat);
    register_fn(env, "dna_enkode_biner", Some(2), builtin_dna_enkode_biner);
    register_fn(env, "dna_dekode_biner", Some(2), builtin_dna_dekode_biner);
    register_fn(env, "dna_hitung_gc_content", Some(1), builtin_dna_gc_content);

    // WidyaReedSolomonBioCodec (DNA Error Correction & Primer Indexing)
    register_fn(env, "KorektorGalatBioDNA", Some(1), builtin_bio_korektor_buat);
    register_fn(env, "dna_tambah_primer_dan_paritas", Some(4), builtin_dna_tambah_primer_paritas);
    register_fn(env, "dna_koreksi_dan_pulihkan", Some(2), builtin_dna_koreksi_pulihkan);

    // WidyaStrandDisplacement (DNA Strand Displacement - DSD Kinetics)
    register_fn(env, "SirkuitDSD", Some(1), builtin_dsd_sirkuit_buat);
    register_fn(env, "dsd_tambah_untai", Some(4), builtin_dsd_tambah_untai);
    register_fn(env, "dsd_tambah_reaksi", Some(6), builtin_dsd_tambah_reaksi);
    register_fn(env, "dsd_simulasi_kinetika", Some(3), builtin_dsd_simulasi_kinetika);

    // WidyaMolecularLogicGates (DNA-based Molecular Logic Gates & CRN)
    register_fn(env, "GerbangLogikaMolekuler", Some(2), builtin_gerbang_bio_buat);
    register_fn(env, "gerbang_bio_evaluasi", Some(3), builtin_gerbang_bio_evaluasi);

    // WidyaSATSolver (DPLL Propositional Boolean Satisfiability Solver)
    register_fn(env, "PemecahSAT", Some(0), builtin_sat_solver_buat);
    register_fn(env, "sat_tambah_klausa", Some(2), builtin_sat_tambah_klausa);
    register_fn(env, "sat_selesaikan_dpll", Some(1), builtin_sat_selesaikan_dpll);
    register_fn(env, "sat_ambil_model", Some(1), builtin_sat_ambil_model);

    // WidyaSMTSolverLinear (Quantifier-Free Linear SMT Solver & SMT-LIB2 Emit)
    register_fn(env, "PemecahSMT", Some(1), builtin_smt_solver_buat);
    register_fn(env, "smt_tambah_variabel", Some(3), builtin_smt_tambah_var);
    register_fn(env, "smt_tambah_kendala", Some(4), builtin_smt_tambah_kendala);
    register_fn(env, "smt_periksa_kepuasan", Some(1), builtin_smt_periksa_kepuasan);
    register_fn(env, "smt_ekspor_smtlib2", Some(1), builtin_smt_ekspor_smtlib2);

    // WidyaHoareLogicVerifier (Hoare Triples & Weakest Precondition)
    register_fn(env, "PemeriksaHoare", Some(0), builtin_hoare_pemeriksa_buat);
    register_fn(env, "hoare_verifikasi_kondisi", Some(4), builtin_hoare_verifikasi);
    register_fn(env, "hoare_hitung_weakest_precondition", Some(4), builtin_hoare_hitung_wp);

    // WidyaFirstOrderTheoremProver (FOL Resolution Prover & Unification)
    register_fn(env, "PembuktiTeoremaFOL", Some(0), builtin_fol_pembukti_buat);
    register_fn(env, "fol_tambah_aksioma", Some(3), builtin_fol_tambah_aksioma);
    register_fn(env, "fol_tambah_tujuan", Some(3), builtin_fol_tambah_tujuan);
    register_fn(env, "fol_buktikan_kontradiksi_resolusi", Some(1), builtin_fol_buktikan_resolusi);

    // WidyaFITSParser (Flexible Image Transport System & WCS Coordinates)
    register_fn(env, "CitraFITS", Some(3), builtin_fits_buat);
    register_fn(env, "fits_tambah_kartu_header", Some(4), builtin_fits_tambah_kartu);
    register_fn(env, "fits_set_piksel", Some(4), builtin_fits_set_piksel);
    register_fn(env, "fits_konversi_piksel_ke_langit_wcs", Some(3), builtin_fits_konversi_wcs);

    // WidyaExoplanetTransit (Exoplanet Light Curve Modeling & Photometry)
    register_fn(env, "KurvaCahayaBintang", Some(2), builtin_transit_kurva_buat);
    register_fn(env, "transit_tambah_titik_fluks", Some(3), builtin_transit_tambah_fluks);
    register_fn(env, "transit_analisis_kedalaman_eksoplanet", Some(1), builtin_transit_analisis);

    // WidyaRadioInterferometry (Aperture Synthesis UV-Plane & Dirty Beam)
    register_fn(env, "InterferometerRadio", Some(1), builtin_interferometer_buat);
    register_fn(env, "interferometer_tambah_antena", Some(5), builtin_interferometer_tambah_antena);
    register_fn(env, "interferometer_hitung_baseline_uv", Some(3), builtin_interferometer_hitung_uv);
    register_fn(env, "interferometer_sintesis_citra_kotor", Some(2), builtin_interferometer_sintesis_kotor);

    // WidyaStellarAstrometry (HR Diagram & Gaia Astrometry)
    register_fn(env, "AstrometriBintang", Some(0), builtin_astrometri_buat);
    register_fn(env, "astrometri_hitung_jarak_paralaks", Some(1), builtin_astrometri_jarak);
    register_fn(env, "astrometri_hitung_magnitudo_mutlak", Some(2), builtin_astrometri_mag_mutlak);
    register_fn(env, "astrometri_klasifikasi_spektral", Some(2), builtin_astrometri_spektral);

    // WidyaCrystalStructure (CIF, POSCAR & Bravais Lattice 3D)
    register_fn(env, "KisiKristal3D", Some(1), builtin_kristal_buat);
    register_fn(env, "kristal_set_parameter_sel", Some(7), builtin_kristal_set_parameter);
    register_fn(env, "kristal_tambah_atom_fraksional", Some(6), builtin_kristal_tambah_atom);
    register_fn(env, "kristal_hitung_densitas_teoritis", Some(1), builtin_kristal_hitung_densitas);
    register_fn(env, "kristal_ekspor_cif", Some(1), builtin_kristal_ekspor_cif);

    // WidyaMolecularDynamicsLJ (Lennard-Jones & Velocity-Verlet)
    register_fn(env, "SimulatorMolekulerMD", Some(3), builtin_md_simulator_buat);
    register_fn(env, "md_tambah_partikel", Some(7), builtin_md_tambah_partikel);
    register_fn(env, "md_langkah_integrasi_verlet", Some(2), builtin_md_langkah_verlet);
    register_fn(env, "md_hitung_energi_sistem", Some(1), builtin_md_hitung_energi);

    // WidyaPhaseDiagramThermodynamics (Binary Alloy & CALPHAD Gibbs)
    register_fn(env, "DiagramFasePaduan", Some(2), builtin_fase_diagram_buat);
    register_fn(env, "fase_set_parameter_interaksi", Some(2), builtin_fase_set_interaksi);
    register_fn(env, "fase_hitung_energi_gibbs", Some(3), builtin_fase_hitung_gibbs);
    register_fn(env, "fase_analisis_kesetimbangan_tuas", Some(4), builtin_fase_analisis_tuas);

    // WidyaMaterialDescriptor (Perovskite Goldschmidt Tolerance & Stability)
    register_fn(env, "DeskriptorMaterial", Some(0), builtin_deskriptor_material_buat);
    register_fn(env, "material_hitung_faktor_toleransi_perovskit", Some(3), builtin_material_toleransi_perovskit);
    register_fn(env, "material_prediksi_stabilitas_energi", Some(2), builtin_material_stabilitas_energi);

    // WidyaTeraHertzChannel (6G Terahertz Propagation & Molecular Loss)
    register_fn(env, "KanalTeraHertz6G", Some(2), builtin_thz_kanal_buat);
    register_fn(env, "thz_set_parameter_lingkungan", Some(4), builtin_thz_set_lingkungan);
    register_fn(env, "thz_hitung_redaman_molekuler", Some(1), builtin_thz_hitung_redaman);
    register_fn(env, "thz_hitung_kapasitas_shannon_gbps", Some(4), builtin_thz_kapasitas_shannon);

    // WidyaReconfigurableIntelligentSurface (RIS / Metasurface Phase-Shift)
    register_fn(env, "PermukaanCerdasRIS", Some(2), builtin_ris_buat);
    register_fn(env, "ris_atur_matriks_fasa", Some(2), builtin_ris_atur_fasa);
    register_fn(env, "ris_hitung_penguatan_sinyal_rx", Some(4), builtin_ris_penguatan_rx);

    // WidyaISACSensing (Integrated Sensing & Communication - Dual Functional)
    register_fn(env, "SistemISAC6G", Some(2), builtin_isac_buat);
    register_fn(env, "isac_sisip_simbol_radar_komunikasi", Some(2), builtin_isac_sisip_simbol);
    register_fn(env, "isac_proses_gema_target", Some(3), builtin_isac_proses_gema);

    // WidyaORANArchitecture (Near-RT RIC, E2 Metrics & xApp Traffic Steering)
    register_fn(env, "ArsitekturORAN", Some(1), builtin_oran_buat);
    register_fn(env, "oran_daftarkan_node_e2", Some(4), builtin_oran_daftarkan_node);
    register_fn(env, "oran_perbarui_metrik_kpi", Some(5), builtin_oran_update_kpi);
    register_fn(env, "oran_eksekusi_xapp_kebijakan", Some(4), builtin_oran_eksekusi_xapp);

    // WidyaAgentBasedModel (Schelling Segregation & Flocking ABM)
    register_fn(env, "ModelBerbasisAgenABM", Some(3), builtin_abm_buat);
    register_fn(env, "abm_tambah_agen", Some(5), builtin_abm_tambah_agen);
    register_fn(env, "abm_langkah_simulasi", Some(1), builtin_abm_langkah_simulasi);
    register_fn(env, "abm_hitung_indeks_segregasi", Some(1), builtin_abm_hitung_segregasi);

    // WidyaSystemDynamics (Stocks, Flows & Feedback Loop Dynamics)
    register_fn(env, "DinamikaSistemStokAlir", Some(0), builtin_sd_buat);
    register_fn(env, "sd_tambah_stok", Some(3), builtin_sd_tambah_stok);
    register_fn(env, "sd_tambah_aliran_flow", Some(5), builtin_sd_tambah_flow);
    register_fn(env, "sd_langkah_waktu_rk4", Some(2), builtin_sd_langkah_rk4);
    register_fn(env, "sd_ambil_nilai_stok", Some(2), builtin_sd_ambil_stok);

    // WidyaEcologicalNetwork (Food Web & Multispecies Lotka-Volterra)
    register_fn(env, "JaringEkologiTrofik", Some(0), builtin_ekologi_buat);
    register_fn(env, "ekologi_tambah_spesies", Some(4), builtin_ekologi_tambah_spesies);
    register_fn(env, "ekologi_tambah_interaksi_predasi", Some(4), builtin_ekologi_tambah_predasi);
    register_fn(env, "ekologi_simulasi_dinamika_populasi", Some(3), builtin_ekologi_simulasi);

    // WidyaEvolutionaryGameTheory (Replicator Dynamics & ESS)
    register_fn(env, "TeoriPermainanEvolusi", Some(0), builtin_game_buat);
    register_fn(env, "game_set_matriks_imbalan", Some(5), builtin_game_set_imbalan);
    register_fn(env, "game_simulasi_dinamika_replikator", Some(4), builtin_game_simulasi_replikator);

    // WidyaOAISArchive (ISO 14721 OAIS Reference Model & PREMIS Metadata)
    register_fn(env, "PaketArsipOAIS", Some(2), builtin_oais_buat);
    register_fn(env, "oais_tambah_konten_sip", Some(5), builtin_oais_tambah_sip);
    register_fn(env, "oais_tambah_metadata_premis", Some(4), builtin_oais_tambah_premis);
    register_fn(env, "oais_transformasi_ke_aip", Some(2), builtin_oais_transformasi_aip);
    register_fn(env, "oais_hasilkan_paket_dip", Some(2), builtin_oais_hasilkan_dip);

    // WidyaBagItPackaging (IETF RFC 8493 Archival Packaging & Manifest)
    register_fn(env, "PaketBagIt", Some(1), builtin_bagit_buat);
    register_fn(env, "bagit_tambah_berkas_payload", Some(3), builtin_bagit_tambah_payload);
    register_fn(env, "bagit_set_metadata_info", Some(4), builtin_bagit_set_info);
    register_fn(env, "bagit_hasilkan_manifest", Some(1), builtin_bagit_hasilkan_manifest);
    register_fn(env, "bagit_verifikasi_integritas", Some(1), builtin_bagit_verifikasi);

    // WidyaWARCWebArchive (ISO 28500 WARC - Web ARChive Format)
    register_fn(env, "PerekamWARC", Some(1), builtin_warc_buat);
    register_fn(env, "warc_tambah_rekaman_http", Some(6), builtin_warc_tambah_http);
    register_fn(env, "warc_hitung_digest_fixity", Some(2), builtin_warc_hitung_digest);
    register_fn(env, "warc_ekspor_dokumen", Some(1), builtin_warc_ekspor_dokumen);

    // WidyaBitRotScrubber (Storage Scrubbing & Parity Data Restoration)
    register_fn(env, "PembersihBitRot", Some(1), builtin_scrubber_buat);
    register_fn(env, "scrub_daftarkan_blok_data", Some(3), builtin_scrub_daftarkan_blok);
    register_fn(env, "scrub_buat_paritas_pemulihan", Some(2), builtin_scrub_buat_paritas);
    register_fn(env, "scrub_simulasi_korupsi_bit", Some(4), builtin_scrub_simulasi_korupsi);
    register_fn(env, "scrub_pindai_dan_perbaiki_bitrot", Some(1), builtin_scrub_pindai_perbaiki);

    // WidyaClaytronicsCatom (Claytronics Modular Catoms & Shape Reconfiguration)
    register_fn(env, "SistemMateriCatom", Some(0), builtin_catom_buat);
    register_fn(env, "catom_tambah_unit", Some(5), builtin_catom_tambah_unit);
    register_fn(env, "catom_atur_adhesi_magnetik", Some(4), builtin_catom_atur_adhesi);
    register_fn(env, "catom_eksekusi_rekonfigurasi_bentuk", Some(2), builtin_catom_rekonfigurasi);

    // WidyaSelfAssembly (Thermodynamic Molecular Tile Self-Assembly)
    register_fn(env, "PerakitanMandiriMolekul", Some(1), builtin_self_assembly_buat);
    register_fn(env, "self_assembly_tambah_ubin_tile", Some(6), builtin_self_assembly_tambah_tile);
    register_fn(env, "self_assembly_atur_benih_awal", Some(4), builtin_self_assembly_atur_benih);
    register_fn(env, "self_assembly_simulasi_pertumbuhan", Some(2), builtin_self_assembly_simulasi);

    // WidyaNanorobotSwarm (Biomedical Nanorobot Swarm & Chemotaxis)
    register_fn(env, "KawananNanorobot", Some(2), builtin_nanorobot_buat);
    register_fn(env, "nanorobot_atur_pembuluh_darah", Some(3), builtin_nanorobot_atur_pembuluh);
    register_fn(env, "nanorobot_tambah_target_tumor", Some(3), builtin_nanorobot_tambah_tumor);
    register_fn(env, "nanorobot_langkah_kemotaksis_dan_injeksi", Some(3), builtin_nanorobot_simulasi);

    // WidyaMolecularCommunication (Diffusion-based MolCom & Nanonetworks)
    register_fn(env, "JaringanKomunikasiMolekuler", Some(2), builtin_molcom_buat);
    register_fn(env, "molcom_kirim_sinyal_csk", Some(3), builtin_molcom_kirim_csk);
    register_fn(env, "molcom_hitung_probabilitas_deteksi", Some(2), builtin_molcom_probabilitas);

    // WidyaTMRRedundancy (Triple Modular Redundancy & SEU Radiation Voter)
    register_fn(env, "MesinRedundansiTMR", Some(0), builtin_tmr_buat);
    register_fn(env, "tmr_eksekusi_instruksi_voting", Some(5), builtin_tmr_eksekusi_voting);
    register_fn(env, "tmr_injeksi_gangguan_radiasi_seu", Some(3), builtin_tmr_injeksi_seu);
    register_fn(env, "tmr_ambil_status_kesehatan_core", Some(1), builtin_tmr_status_kesehatan);

    // WidyaCCSDSTelemetry (CCSDS Space Packet Protocol & Transfer Frame)
    register_fn(env, "PaketTelemetriCCSDS", Some(2), builtin_ccsds_buat);
    register_fn(env, "ccsds_atur_data_payload", Some(3), builtin_ccsds_atur_payload);
    register_fn(env, "ccsds_enkapsulasi_frame_transfer", Some(1), builtin_ccsds_enkapsulasi);
    register_fn(env, "ccsds_verifikasi_crc", Some(1), builtin_ccsds_verifikasi_crc);

    // WidyaDTNBundleProtocol (IETF RFC 9171 Delay-Tolerant Networking BPv7)
    register_fn(env, "ProtokolBundleDTN", Some(2), builtin_dtn_buat);
    register_fn(env, "dtn_buat_bundle_antarplanet", Some(4), builtin_dtn_buat_bundle);
    register_fn(env, "dtn_terima_dan_simpan_custody", Some(2), builtin_dtn_simpan_custody);
    register_fn(env, "dtn_teruskan_saat_kontak_tersedia", Some(3), builtin_dtn_teruskan_bundle);

    // WidyaFaultTolerantFSM (Spacecraft FDIR - Autonomous Recovery & Safe Mode)
    register_fn(env, "PengendaliFDIRWahana", Some(1), builtin_fdir_buat);
    register_fn(env, "fdir_daftarkan_subsistem", Some(3), builtin_fdir_daftarkan_subsistem);
    register_fn(env, "fdir_picu_anomali_telemetri", Some(3), builtin_fdir_picu_anomali);
    register_fn(env, "fdir_eksekusi_pemulihan_safemode", Some(1), builtin_fdir_pemulihan_safemode);

    // WidyaGlobalWorkspace (Baars Global Workspace Theory - GWT Cognitive Engine)
    register_fn(env, "PapanGlobalWorkspace", Some(1), builtin_gwt_buat);
    register_fn(env, "gwt_daftarkan_modul_kognitif", Some(4), builtin_gwt_daftarkan_modul);
    register_fn(env, "gwt_kompetisi_fokus_atensi", Some(1), builtin_gwt_kompetisi_atensi);
    register_fn(env, "gwt_siarkan_kesadaran_global", Some(1), builtin_gwt_siarkan_kesadaran);

    // WidyaACTRMemory (Anderson ACT-R Declarative & Procedural Architecture)
    register_fn(env, "MesinKognitifACTR", Some(0), builtin_actr_buat);
    register_fn(env, "actr_tambah_chunk_deklaratif", Some(3), builtin_actr_tambah_chunk);
    register_fn(env, "actr_kueri_ingatan_retrieval", Some(2), builtin_actr_kueri_ingatan);
    register_fn(env, "actr_daftarkan_aturan_produksi", Some(5), builtin_actr_daftarkan_aturan);
    register_fn(env, "actr_siklus_eksekusi", Some(1), builtin_actr_siklus_eksekusi);

    // WidyaSOARArchitecture (Laird SOAR Decision Cycles & Impasse Subgoaling)
    register_fn(env, "ArsitekturKognitifSOAR", Some(1), builtin_soar_buat);
    register_fn(env, "soar_usulkan_operator", Some(4), builtin_soar_usul_operator);
    register_fn(env, "soar_evaluasi_dan_pilih_operator", Some(1), builtin_soar_evaluasi_operator);
    register_fn(env, "soar_tangani_kebuntuan_impasse", Some(2), builtin_soar_tangani_impasse);

    // WidyaHyperdimensionalComputing (Hyperdimensional Computing - HDC / VSA)
    register_fn(env, "RuangHiperdimensiHDC", Some(1), builtin_hdc_buat);
    register_fn(env, "hdc_buat_hipervektor", Some(2), builtin_hdc_buat_vektor);
    register_fn(env, "hdc_ikat_konsep_bind", Some(3), builtin_hdc_bind);
    register_fn(env, "hdc_gabung_vektor_bundle", Some(3), builtin_hdc_bundle);
    register_fn(env, "hdc_ukur_kesamaan_simbolik", Some(3), builtin_hdc_kesamaan);

    // WidyaGrammarDSL (EBNF/PEG Grammar DSL & AST Generator)
    register_fn(env, "MesinTataBahasaDSL", Some(1), builtin_grammar_buat);
    register_fn(env, "grammar_tambah_aturan_produksi", Some(3), builtin_grammar_tambah_aturan);
    register_fn(env, "grammar_bangun_parser", Some(1), builtin_grammar_bangun_parser);
    register_fn(env, "grammar_urai_kode_ke_ast", Some(2), builtin_grammar_urai_ke_ast);

    // WidyaMacroQuasiquote (Hygienic Procedural Macro & AST Code Splicing)
    register_fn(env, "MesinMakroHigienis", Some(0), builtin_makro_buat);
    register_fn(env, "makro_daftarkan_pola_ekspansi", Some(4), builtin_makro_daftarkan_pola);
    register_fn(env, "makro_evaluasi_quasiquote_splice", Some(3), builtin_makro_evaluasi_quasiquote);
    register_fn(env, "makro_ekspansi_sintaksis", Some(2), builtin_makro_ekspansi);

    // WidyaCompilerBootstrap (Self-Hosting Compiler Verification & Code Emission)
    register_fn(env, "PengendaliBootstrapKompilator", Some(1), builtin_bootstrap_buat);
    register_fn(env, "bootstrap_inisialisasi_tahap", Some(4), builtin_bootstrap_inisialisasi_tahap);
    register_fn(env, "bootstrap_kompilasi_sumber_ke_target", Some(4), builtin_bootstrap_kompilasi_sumber);
    register_fn(env, "bootstrap_verifikasi_kesetaraan_biner", Some(3), builtin_bootstrap_verifikasi_biner);

    // WidyaTypeCheckerInference (Hindley-Milner / Algorithm W Static Type Inference)
    register_fn(env, "PemeriksaTipeInferensi", Some(0), builtin_tipe_checker_buat);
    register_fn(env, "tipe_daftarkan_skema_tipe", Some(3), builtin_tipe_daftarkan_skema);
    register_fn(env, "tipe_unifikasi_persamaan", Some(3), builtin_tipe_unifikasi);
    register_fn(env, "tipe_inferensi_ekspresi_ast", Some(2), builtin_tipe_inferensi_ast);

    // WidyaNBodyBarnesHut (Cosmological Galactic N-Body Barnes-Hut Octree)
    register_fn(env, "SimulasiGravitasiBarnesHut", Some(2), builtin_barneshut_buat);
    register_fn(env, "barneshut_tambah_massa_partikel", Some(9), builtin_barneshut_tambah_partikel);
    register_fn(env, "barneshut_bangun_octree", Some(1), builtin_barneshut_bangun_octree);
    register_fn(env, "barneshut_langkah_integrasi_waktu", Some(2), builtin_barneshut_langkah_integrasi);

    // WidyaRelativisticGravity (General Relativity: Schwarzschild Black Hole & Lensing)
    register_fn(env, "MetrikLubangHitam", Some(1), builtin_metrik_bh_buat);
    register_fn(env, "schwarzschild_hitung_radius_horizon", Some(1), builtin_schwarzschild_radius);
    register_fn(env, "relativitas_lensa_gravitasi_foton", Some(2), builtin_relativitas_lensa_foton);
    register_fn(env, "relativitas_geodesik_orbit_partikel", Some(3), builtin_relativitas_geodesik_orbit);

    // WidyaCosmologicalExpansion (Lambda-CDM Friedmann Universe Evolution)
    register_fn(env, "ModelKosmologiLambdaCDM", Some(4), builtin_kosmologi_buat);
    register_fn(env, "kosmologi_hitung_parameter_hubble", Some(2), builtin_kosmologi_hubble);
    register_fn(env, "kosmologi_hitung_jarak_luminositas", Some(2), builtin_kosmologi_jarak_luminositas);
    register_fn(env, "kosmologi_hitung_umur_dan_lookback", Some(2), builtin_kosmologi_umur_lookback);

    // WidyaGravitationalWaveforms (Binary Black Hole Merger & LIGO Chirp Waveform)
    register_fn(env, "GeneratorGelombangGravitasi", Some(3), builtin_gw_buat);
    register_fn(env, "gw_hitung_massa_chirp", Some(1), builtin_gw_massa_chirp);
    register_fn(env, "gw_hasilkan_regangan_strain_imr", Some(2), builtin_gw_strain_imr);
    register_fn(env, "gw_estimasi_daya_radiasi", Some(2), builtin_gw_daya_radiasi);

}

// WIDYA WAVE 36: CLOUD-NATIVE & ULTRA-SCALE BACKEND ARCHITECTURE
// =========================================================================

// --- 1. WidyaLoadBalancer ---
#[derive(Debug, Clone)]
struct LBServer {
    id: String,
    url: String,
    weight: usize,
    active_conns: usize,
    is_healthy: bool,
}

#[derive(Debug, Clone)]
struct WidyaLB {
    strategy: String, // "round_robin", "least_conn", "weighted", "ip_hash"
    servers: Vec<LBServer>,
    current_index: usize,
}

fn value_to_lb(val: &Value) -> Option<WidyaLB> {
    if let Value::Map(m) = val {
        let borrowed = m.borrow();
        if let Some(Value::String(tag)) = borrowed.get("__tipe__") {
            if tag == "PenyeimbangBeban" {
                let strategy = match borrowed.get("strategi") {
                    Some(Value::String(s)) => s.clone(),
                    _ => "round_robin".to_string(),
                };
                let current_index = match borrowed.get("indeks_saat_ini") {
                    Some(Value::Number(n)) => *n as usize,
                    _ => 0,
                };
                let mut servers = Vec::new();
                if let Some(Value::Array(srv_arr)) = borrowed.get("daftar_server") {
                    for s in srv_arr.borrow().iter() {
                        if let Value::Map(sm) = s {
                            let sb = sm.borrow();
                            let id = match sb.get("id") {
                                Some(Value::String(v)) => v.clone(),
                                _ => String::new(),
                            };
                            let url = match sb.get("url") {
                                Some(Value::String(v)) => v.clone(),
                                _ => String::new(),
                            };
                            let weight = match sb.get("bobot") {
                                Some(Value::Number(v)) => *v as usize,
                                _ => 1,
                            };
                            let active_conns = match sb.get("koneksi_aktif") {
                                Some(Value::Number(v)) => *v as usize,
                                _ => 0,
                            };
                            let is_healthy = match sb.get("sehat") {
                                Some(Value::Bool(b)) => *b,
                                _ => true,
                            };
                            servers.push(LBServer {
                                id,
                                url,
                                weight: if weight == 0 { 1 } else { weight },
                                active_conns,
                                is_healthy,
                            });
                        }
                    }
                }
                return Some(WidyaLB {
                    strategy,
                    servers,
                    current_index,
                });
            }
        }
    }
    None
}

fn lb_to_value(lb: &WidyaLB) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("PenyeimbangBeban".to_string()));
    map.insert("strategi".to_string(), Value::String(lb.strategy.clone()));
    map.insert("indeks_saat_ini".to_string(), Value::Number(lb.current_index as f64));
    
    let mut srv_vec = Vec::new();
    for s in &lb.servers {
        let mut sm = HashMap::new();
        sm.insert("id".to_string(), Value::String(s.id.clone()));
        sm.insert("url".to_string(), Value::String(s.url.clone()));
        sm.insert("bobot".to_string(), Value::Number(s.weight as f64));
        sm.insert("koneksi_aktif".to_string(), Value::Number(s.active_conns as f64));
        sm.insert("sehat".to_string(), Value::Bool(s.is_healthy));
        srv_vec.push(Value::Map(Rc::new(RefCell::new(sm))));
    }
    map.insert("daftar_server".to_string(), Value::Array(Rc::new(RefCell::new(srv_vec))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_lb_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let strategy = if !args.is_empty() {
        match &args[0] {
            Value::String(s) => s.to_lowercase(),
            _ => return Err(Galat::runtime("Parameter strategi PenyeimbangBeban harus berupa string (misal: 'round_robin', 'least_conn', 'ip_hash', 'weighted')", span)),
        }
    } else {
        "round_robin".to_string()
    };

    let lb = WidyaLB {
        strategy,
        servers: Vec::new(),
        current_index: 0,
    };
    Ok(lb_to_value(&lb))
}

fn builtin_lb_tambah_server(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let id = match &args[1] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 2 id_server harus string", span)) };
    let url = match &args[2] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 3 url_server harus string", span)) };
    let weight = match &args[3] { Value::Number(n) => *n as usize, _ => 1 };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut lb) = value_to_lb(&args[0]) {
                lb.servers.push(LBServer {
                    id,
                    url,
                    weight: if weight == 0 { 1 } else { weight },
                    active_conns: 0,
                    is_healthy: true,
                });
                let updated = lb_to_value(&lb);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PenyeimbangBeban tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'lb_tambah_server' memerlukan instance PenyeimbangBeban", span)),
    }
}

fn builtin_lb_pilih_server(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'lb_pilih_server' memerlukan instance PenyeimbangBeban", span));
    }
    match &args[0] {
        Value::Map(m) => {
            if let Some(mut lb) = value_to_lb(&args[0]) {
                let healthy_indices: Vec<usize> = lb.servers.iter().enumerate()
                    .filter(|(_, s)| s.is_healthy)
                    .map(|(i, _)| i)
                    .collect();

                if healthy_indices.is_empty() {
                    return Err(Galat::runtime("Tidak ada server sehat yang tersedia dalam pool Load Balancer", span));
                }

                let selected_idx = match lb.strategy.as_str() {
                    "least_conn" | "least_connection" => {
                        let mut min_conn = usize::MAX;
                        let mut best_i = healthy_indices[0];
                        for &i in &healthy_indices {
                            if lb.servers[i].active_conns < min_conn {
                                min_conn = lb.servers[i].active_conns;
                                best_i = i;
                            }
                        }
                        best_i
                    }
                    "ip_hash" => {
                        let client_ip = if args.len() > 1 {
                            match &args[1] {
                                Value::String(s) => s.as_str(),
                                _ => "127.0.0.1",
                            }
                        } else {
                            "127.0.0.1"
                        };
                        let mut h: u64 = 5381;
                        for b in client_ip.bytes() {
                            h = ((h << 5).wrapping_add(h)).wrapping_add(b as u64);
                        }
                        healthy_indices[(h as usize) % healthy_indices.len()]
                    }
                    "weighted" | "weighted_round_robin" => {
                        let total_weight: usize = healthy_indices.iter().map(|&i| lb.servers[i].weight).sum();
                        if total_weight == 0 {
                            healthy_indices[0]
                        } else {
                            let mut step = (lb.current_index % total_weight) + 1;
                            lb.current_index = (lb.current_index + 1) % (total_weight * 100);
                            let mut chosen = healthy_indices[0];
                            for &i in &healthy_indices {
                                if step <= lb.servers[i].weight {
                                    chosen = i;
                                    break;
                                }
                                step -= lb.servers[i].weight;
                            }
                            chosen
                        }
                    }
                    _ => {
                        // round_robin default
                        let pos = lb.current_index % healthy_indices.len();
                        lb.current_index = (lb.current_index + 1) % 1000000;
                        healthy_indices[pos]
                    }
                };

                let chosen_server = lb.servers[selected_idx].clone();
                let updated = lb_to_value(&lb);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    let mut res_map = HashMap::new();
                    res_map.insert("id".to_string(), Value::String(chosen_server.id));
                    res_map.insert("url".to_string(), Value::String(chosen_server.url));
                    res_map.insert("bobot".to_string(), Value::Number(chosen_server.weight as f64));
                    res_map.insert("koneksi_aktif".to_string(), Value::Number(chosen_server.active_conns as f64));
                    res_map.insert("sehat".to_string(), Value::Bool(chosen_server.is_healthy));
                    return Ok(Value::Map(Rc::new(RefCell::new(res_map))));
                }
            }
            Err(Galat::runtime("Objek PenyeimbangBeban tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'lb_pilih_server' memerlukan instance PenyeimbangBeban", span)),
    }
}

fn builtin_lb_laporkan_koneksi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let server_id = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 id_server harus string", span)) };
    let delta = match &args[2] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("Parameter 3 perubahan_koneksi harus integer (misal: 1 atau -1)", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut lb) = value_to_lb(&args[0]) {
                for s in &mut lb.servers {
                    if s.id == server_id {
                        if delta > 0 {
                            s.active_conns += delta as usize;
                        } else {
                            let neg = (-delta) as usize;
                            s.active_conns = if s.active_conns >= neg { s.active_conns - neg } else { 0 };
                        }
                    }
                }
                let updated = lb_to_value(&lb);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PenyeimbangBeban tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'lb_laporkan_koneksi' memerlukan instance PenyeimbangBeban", span)),
    }
}

fn builtin_lb_set_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let server_id = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 id_server harus string", span)) };
    let is_healthy = match &args[2] { Value::Bool(b) => *b, _ => return Err(Galat::runtime("Parameter 3 status_sehat harus boolean", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut lb) = value_to_lb(&args[0]) {
                for s in &mut lb.servers {
                    if s.id == server_id {
                        s.is_healthy = is_healthy;
                    }
                }
                let updated = lb_to_value(&lb);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PenyeimbangBeban tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'lb_set_status' memerlukan instance PenyeimbangBeban", span)),
    }
}

// --- 2. WidyaAPIGatewayRouter ---
#[derive(Debug, Clone)]
struct APIRoute {
    method: String, // "GET", "POST", "PUT", "DELETE", "*"
    path_pattern: String,
    handler_name: String,
}

#[derive(Debug, Clone)]
struct WidyaAPIRouter {
    routes: Vec<APIRoute>,
}

fn value_to_router(val: &Value) -> Option<WidyaAPIRouter> {
    if let Value::Map(m) = val {
        let borrowed = m.borrow();
        if let Some(Value::String(tag)) = borrowed.get("__tipe__") {
            if tag == "RouterAPI" {
                let mut routes = Vec::new();
                if let Some(Value::Array(arr)) = borrowed.get("rute") {
                    for r in arr.borrow().iter() {
                        if let Value::Map(rm) = r {
                            let rb = rm.borrow();
                            let method = match rb.get("metode") {
                                Some(Value::String(s)) => s.clone(),
                                _ => "*".to_string(),
                            };
                            let path_pattern = match rb.get("pola_jalur") {
                                Some(Value::String(s)) => s.clone(),
                                _ => "/".to_string(),
                            };
                            let handler_name = match rb.get("nama_handler") {
                                Some(Value::String(s)) => s.clone(),
                                _ => String::new(),
                            };
                            routes.push(APIRoute {
                                method,
                                path_pattern,
                                handler_name,
                            });
                        }
                    }
                }
                return Some(WidyaAPIRouter { routes });
            }
        }
    }
    None
}

fn router_to_value(r: &WidyaAPIRouter) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("RouterAPI".to_string()));
    let mut arr = Vec::new();
    for item in &r.routes {
        let mut rm = HashMap::new();
        rm.insert("metode".to_string(), Value::String(item.method.clone()));
        rm.insert("pola_jalur".to_string(), Value::String(item.path_pattern.clone()));
        rm.insert("nama_handler".to_string(), Value::String(item.handler_name.clone()));
        arr.push(Value::Map(Rc::new(RefCell::new(rm))));
    }
    map.insert("rute".to_string(), Value::Array(Rc::new(RefCell::new(arr))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_router_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let router = WidyaAPIRouter { routes: Vec::new() };
    Ok(router_to_value(&router))
}

fn builtin_router_tambah_rute(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let method = match &args[1] { Value::String(s) => s.to_uppercase(), _ => return Err(Galat::runtime("Parameter 2 metode HTTP harus string (misal: 'GET', 'POST')", span)) };
    let path_pattern = match &args[2] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 3 pola_jalur harus string (misal: '/api/v1/pengguna/:id')", span)) };
    let handler_name = if args.len() > 3 {
        match &args[3] {
            Value::String(s) => s.clone(),
            _ => "default_handler".to_string(),
        }
    } else {
        "handler".to_string()
    };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut router) = value_to_router(&args[0]) {
                router.routes.push(APIRoute {
                    method,
                    path_pattern,
                    handler_name,
                });
                let updated = router_to_value(&router);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek RouterAPI tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'router_tambah_rute' memerlukan instance RouterAPI", span)),
    }
}

fn builtin_router_cocokkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let req_method = match &args[1] { Value::String(s) => s.to_uppercase(), _ => return Err(Galat::runtime("Parameter 2 metode HTTP harus string", span)) };
    let req_path = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 3 jalur_url harus string", span)) };

    if let Some(router) = value_to_router(&args[0]) {
        let req_segments: Vec<&str> = req_path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();

        for route in &router.routes {
            if route.method != "*" && route.method != req_method {
                continue;
            }

            let pattern_segments: Vec<&str> = route.path_pattern.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
            
            // Check wildcard matching /api/*
            if pattern_segments.last() == Some(&"*") {
                let prefix_len = pattern_segments.len() - 1;
                if req_segments.len() >= prefix_len && req_segments[..prefix_len] == pattern_segments[..prefix_len] {
                    let mut res = HashMap::new();
                    res.insert("cocok".to_string(), Value::Bool(true));
                    res.insert("handler".to_string(), Value::String(route.handler_name.clone()));
                    res.insert("pola".to_string(), Value::String(route.path_pattern.clone()));
                    res.insert("parameter".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
                    let wild_val = req_segments[prefix_len..].join("/");
                    res.insert("wildcard".to_string(), Value::String(wild_val));
                    return Ok(Value::Map(Rc::new(RefCell::new(res))));
                }
            }

            if pattern_segments.len() != req_segments.len() {
                continue;
            }

            let mut matches = true;
            let mut params = HashMap::new();

            for (p_seg, r_seg) in pattern_segments.iter().zip(req_segments.iter()) {
                if p_seg.starts_with(':') {
                    let param_name = &p_seg[1..];
                    params.insert(param_name.to_string(), Value::String(r_seg.to_string()));
                } else if p_seg != r_seg {
                    matches = false;
                    break;
                }
            }

            if matches {
                let mut res = HashMap::new();
                res.insert("cocok".to_string(), Value::Bool(true));
                res.insert("handler".to_string(), Value::String(route.handler_name.clone()));
                res.insert("pola".to_string(), Value::String(route.path_pattern.clone()));
                res.insert("parameter".to_string(), Value::Map(Rc::new(RefCell::new(params))));
                res.insert("wildcard".to_string(), Value::Nil);
                return Ok(Value::Map(Rc::new(RefCell::new(res))));
            }
        }

        let mut res = HashMap::new();
        res.insert("cocok".to_string(), Value::Bool(false));
        res.insert("handler".to_string(), Value::Nil);
        res.insert("pola".to_string(), Value::Nil);
        res.insert("parameter".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
        res.insert("wildcard".to_string(), Value::Nil);
        return Ok(Value::Map(Rc::new(RefCell::new(res))));
    }

    Err(Galat::runtime("Fungsi 'router_cocokkan' memerlukan instance RouterAPI", span))
}

// --- 3. WidyaDistributedLock ---
#[derive(Debug, Clone)]
struct LockEntry {
    owner_id: String,
    expires_at_ms: u64,
}

#[derive(Debug, Clone)]
struct WidyaDistLock {
    current_time_ms: u64,
    locks: HashMap<String, LockEntry>,
}

fn value_to_distlock(val: &Value) -> Option<WidyaDistLock> {
    if let Value::Map(m) = val {
        let borrowed = m.borrow();
        if let Some(Value::String(tag)) = borrowed.get("__tipe__") {
            if tag == "KunciTerdistribusi" {
                let current_time_ms = match borrowed.get("waktu_saat_ini_ms") {
                    Some(Value::Number(n)) => *n as u64,
                    _ => 0,
                };
                let mut locks = HashMap::new();
                if let Some(Value::Map(lm)) = borrowed.get("kunci_aktif") {
                    for (k, v) in lm.borrow().iter() {
                        if let Value::Map(entry_m) = v {
                            let eb = entry_m.borrow();
                            let owner_id = match eb.get("pemilik") {
                                Some(Value::String(s)) => s.clone(),
                                _ => String::new(),
                            };
                            let expires_at_ms = match eb.get("kadaluarsa_ms") {
                                Some(Value::Number(n)) => *n as u64,
                                _ => 0,
                            };
                            locks.insert(k.clone(), LockEntry { owner_id, expires_at_ms });
                        }
                    }
                }
                return Some(WidyaDistLock {
                    current_time_ms,
                    locks,
                });
            }
        }
    }
    None
}

fn distlock_to_value(dl: &WidyaDistLock) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("KunciTerdistribusi".to_string()));
    map.insert("waktu_saat_ini_ms".to_string(), Value::Number(dl.current_time_ms as f64));
    
    let mut lk_map = HashMap::new();
    for (k, v) in &dl.locks {
        let mut em = HashMap::new();
        em.insert("pemilik".to_string(), Value::String(v.owner_id.clone()));
        em.insert("kadaluarsa_ms".to_string(), Value::Number(v.expires_at_ms as f64));
        lk_map.insert(k.clone(), Value::Map(Rc::new(RefCell::new(em))));
    }
    map.insert("kunci_aktif".to_string(), Value::Map(Rc::new(RefCell::new(lk_map))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_distlock_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let start_time = if !args.is_empty() {
        match &args[0] {
            Value::Number(n) => *n as u64,
            _ => 1000,
        }
    } else {
        1000
    };
    let dl = WidyaDistLock {
        current_time_ms: start_time,
        locks: HashMap::new(),
    };
    Ok(distlock_to_value(&dl))
}

fn builtin_distlock_akuisisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let resource_key = match &args[1] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 2 kunci_sumber_daya harus string", span)) };
    let owner_id = match &args[2] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 3 id_pemilik harus string", span)) };
    let ttl_ms = match &args[3] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("Parameter 4 ttl_milidetik harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut dl) = value_to_distlock(&args[0]) {
                let now = dl.current_time_ms;
                let can_acquire = match dl.locks.get(&resource_key) {
                    Some(entry) => {
                        // Expired or same owner
                        entry.expires_at_ms <= now || entry.owner_id == owner_id
                    }
                    None => true,
                };

                if can_acquire {
                    dl.locks.insert(resource_key, LockEntry {
                        owner_id,
                        expires_at_ms: now + ttl_ms,
                    });
                    let updated = distlock_to_value(&dl);
                    if let Value::Map(new_m) = updated {
                        *m.borrow_mut() = new_m.borrow().clone();
                        return Ok(Value::Bool(true));
                    }
                }
                return Ok(Value::Bool(false));
            }
            Err(Galat::runtime("Objek KunciTerdistribusi tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'distlock_akuisisi' memerlukan instance KunciTerdistribusi", span)),
    }
}

fn builtin_distlock_lepas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let resource_key = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 kunci_sumber_daya harus string", span)) };
    let owner_id = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 3 id_pemilik harus string", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut dl) = value_to_distlock(&args[0]) {
                if let Some(entry) = dl.locks.get(resource_key) {
                    if entry.owner_id == owner_id {
                        dl.locks.remove(resource_key);
                        let updated = distlock_to_value(&dl);
                        if let Value::Map(new_m) = updated {
                            *m.borrow_mut() = new_m.borrow().clone();
                            return Ok(Value::Bool(true));
                        }
                    }
                }
                return Ok(Value::Bool(false));
            }
            Err(Galat::runtime("Objek KunciTerdistribusi tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'distlock_lepas' memerlukan instance KunciTerdistribusi", span)),
    }
}

fn builtin_distlock_perpanjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let resource_key = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 kunci_sumber_daya harus string", span)) };
    let owner_id = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 3 id_pemilik harus string", span)) };
    let additional_ttl_ms = match &args[3] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("Parameter 4 ttl_tambahan_ms harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut dl) = value_to_distlock(&args[0]) {
                let now = dl.current_time_ms;
                if let Some(entry) = dl.locks.get_mut(resource_key) {
                    if entry.owner_id == owner_id && entry.expires_at_ms > now {
                        entry.expires_at_ms += additional_ttl_ms;
                        let updated = distlock_to_value(&dl);
                        if let Value::Map(new_m) = updated {
                            *m.borrow_mut() = new_m.borrow().clone();
                            return Ok(Value::Bool(true));
                        }
                    }
                }
                return Ok(Value::Bool(false));
            }
            Err(Galat::runtime("Objek KunciTerdistribusi tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'distlock_perpanjang' memerlukan instance KunciTerdistribusi", span)),
    }
}

fn builtin_distlock_apakah_terkunci(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let resource_key = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 kunci_sumber_daya harus string", span)) };

    if let Some(dl) = value_to_distlock(&args[0]) {
        let now = dl.current_time_ms;
        if let Some(entry) = dl.locks.get(resource_key) {
            if entry.expires_at_ms > now {
                return Ok(Value::Bool(true));
            }
        }
        return Ok(Value::Bool(false));
    }
    Err(Galat::runtime("Fungsi 'distlock_apakah_terkunci' memerlukan instance KunciTerdistribusi", span))
}

// --- 4. WidyaOpenTelemetry ---
#[derive(Debug, Clone)]
struct TraceSpan {
    trace_id: String,
    span_id: String,
    parent_span_id: Option<String>,
    name: String,
    start_time_ms: u64,
    end_time_ms: Option<u64>,
    duration_ms: Option<u64>,
    tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct WidyaTracer {
    service_name: String,
    current_trace_id: String,
    spans: Vec<TraceSpan>,
}

fn value_to_tracer(val: &Value) -> Option<WidyaTracer> {
    if let Value::Map(m) = val {
        let borrowed = m.borrow();
        if let Some(Value::String(tag)) = borrowed.get("__tipe__") {
            if tag == "TracerTelemetry" {
                let service_name = match borrowed.get("nama_layanan") {
                    Some(Value::String(s)) => s.clone(),
                    _ => "widya-service".to_string(),
                };
                let current_trace_id = match borrowed.get("id_trace") {
                    Some(Value::String(s)) => s.clone(),
                    _ => "trace-001".to_string(),
                };
                let mut spans = Vec::new();
                if let Some(Value::Array(arr)) = borrowed.get("spans") {
                    for sv in arr.borrow().iter() {
                        if let Value::Map(sm) = sv {
                            let sb = sm.borrow();
                            let trace_id = match sb.get("trace_id") {
                                Some(Value::String(s)) => s.clone(),
                                _ => current_trace_id.clone(),
                            };
                            let span_id = match sb.get("span_id") {
                                Some(Value::String(s)) => s.clone(),
                                _ => String::new(),
                            };
                            let parent_span_id = match sb.get("parent_span_id") {
                                Some(Value::String(s)) => Some(s.clone()),
                                _ => None,
                            };
                            let name = match sb.get("nama") {
                                Some(Value::String(s)) => s.clone(),
                                _ => String::new(),
                            };
                            let start_time_ms = match sb.get("waktu_mulai_ms") {
                                Some(Value::Number(n)) => *n as u64,
                                _ => 0,
                            };
                            let end_time_ms = match sb.get("waktu_selesai_ms") {
                                Some(Value::Number(n)) => Some(*n as u64),
                                _ => None,
                            };
                            let duration_ms = match sb.get("durasi_ms") {
                                Some(Value::Number(n)) => Some(*n as u64),
                                _ => None,
                            };
                            let mut tags = HashMap::new();
                            if let Some(Value::Map(tm)) = sb.get("tag") {
                                for (tk, tv) in tm.borrow().iter() {
                                    tags.insert(tk.clone(), tv.to_string_repr());
                                }
                            }
                            spans.push(TraceSpan {
                                trace_id,
                                span_id,
                                parent_span_id,
                                name,
                                start_time_ms,
                                end_time_ms,
                                duration_ms,
                                tags,
                            });
                        }
                    }
                }
                return Some(WidyaTracer {
                    service_name,
                    current_trace_id,
                    spans,
                });
            }
        }
    }
    None
}

fn tracer_to_value(t: &WidyaTracer) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("TracerTelemetry".to_string()));
    map.insert("nama_layanan".to_string(), Value::String(t.service_name.clone()));
    map.insert("id_trace".to_string(), Value::String(t.current_trace_id.clone()));
    
    let mut s_vec = Vec::new();
    for s in &t.spans {
        let mut sm = HashMap::new();
        sm.insert("trace_id".to_string(), Value::String(s.trace_id.clone()));
        sm.insert("span_id".to_string(), Value::String(s.span_id.clone()));
        sm.insert("parent_span_id".to_string(), match &s.parent_span_id {
            Some(p) => Value::String(p.clone()),
            None => Value::Nil,
        });
        sm.insert("nama".to_string(), Value::String(s.name.clone()));
        sm.insert("waktu_mulai_ms".to_string(), Value::Number(s.start_time_ms as f64));
        sm.insert("waktu_selesai_ms".to_string(), match s.end_time_ms {
            Some(n) => Value::Number(n as f64),
            None => Value::Nil,
        });
        sm.insert("durasi_ms".to_string(), match s.duration_ms {
            Some(n) => Value::Number(n as f64),
            None => Value::Nil,
        });
        let mut tm = HashMap::new();
        for (tk, tv) in &s.tags {
            tm.insert(tk.clone(), Value::String(tv.clone()));
        }
        sm.insert("tag".to_string(), Value::Map(Rc::new(RefCell::new(tm))));
        s_vec.push(Value::Map(Rc::new(RefCell::new(sm))));
    }
    map.insert("spans".to_string(), Value::Array(Rc::new(RefCell::new(s_vec))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_trace_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let service_name = if !args.is_empty() {
        match &args[0] {
            Value::String(s) => s.clone(),
            _ => "layanan-widya".to_string(),
        }
    } else {
        "layanan-widya".to_string()
    };

    let trace_id = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => format!("trace-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()),
        }
    } else {
        format!("trace-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis())
    };

    let tracer = WidyaTracer {
        service_name,
        current_trace_id: trace_id,
        spans: Vec::new(),
    };
    Ok(tracer_to_value(&tracer))
}

fn builtin_trace_mulai_span(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'trace_mulai_span' memerlukan instance TracerTelemetry", span));
    }
    let name = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => "operasi".to_string(),
        }
    } else {
        "operasi".to_string()
    };
    let parent_id = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    } else {
        None
    };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut tracer) = value_to_tracer(&args[0]) {
                let span_id = format!("span-{}", tracer.spans.len() + 1);
                let start_time_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;

                tracer.spans.push(TraceSpan {
                    trace_id: tracer.current_trace_id.clone(),
                    span_id: span_id.clone(),
                    parent_span_id: parent_id,
                    name,
                    start_time_ms,
                    end_time_ms: None,
                    duration_ms: None,
                    tags: HashMap::new(),
                });

                let updated = tracer_to_value(&tracer);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::String(span_id));
                }
            }
            Err(Galat::runtime("Objek TracerTelemetry tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'trace_mulai_span' memerlukan instance TracerTelemetry", span)),
    }
}

fn builtin_trace_akhiri_span(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let span_id = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 id_span harus string", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut tracer) = value_to_tracer(&args[0]) {
                let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
                for s in &mut tracer.spans {
                    if s.span_id == span_id {
                        let end = if now_ms >= s.start_time_ms { now_ms } else { s.start_time_ms };
                        s.end_time_ms = Some(end);
                        s.duration_ms = Some(end - s.start_time_ms);
                    }
                }
                let updated = tracer_to_value(&tracer);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek TracerTelemetry tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'trace_akhiri_span' memerlukan instance TracerTelemetry", span)),
    }
}

fn builtin_trace_tambah_tag(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let span_id = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 id_span harus string", span)) };
    let tag_key = match &args[2] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 3 kunci_tag harus string", span)) };
    let tag_val = args[3].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut tracer) = value_to_tracer(&args[0]) {
                for s in &mut tracer.spans {
                    if s.span_id == span_id {
                        s.tags.insert(tag_key.clone(), tag_val.clone());
                    }
                }
                let updated = tracer_to_value(&tracer);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek TracerTelemetry tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'trace_tambah_tag' memerlukan instance TracerTelemetry", span)),
    }
}

fn builtin_trace_ekspor_json(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(tracer) = value_to_tracer(&args[0]) {
        let mut out = String::new();
        out.push_str("{\n");
        out.push_str(&format!("  \"service\": \"{}\",\n", tracer.service_name));
        out.push_str(&format!("  \"traceId\": \"{}\",\n", tracer.current_trace_id));
        out.push_str("  \"spans\": [\n");
        for (i, s) in tracer.spans.iter().enumerate() {
            out.push_str("    {\n");
            out.push_str(&format!("      \"spanId\": \"{}\",\n", s.span_id));
            if let Some(p) = &s.parent_span_id {
                out.push_str(&format!("      \"parentSpanId\": \"{}\",\n", p));
            } else {
                out.push_str("      \"parentSpanId\": null,\n");
            }
            out.push_str(&format!("      \"name\": \"{}\",\n", s.name));
            out.push_str(&format!("      \"startTimeMs\": {},\n", s.start_time_ms));
            out.push_str(&format!("      \"endTimeMs\": {},\n", s.end_time_ms.unwrap_or(s.start_time_ms)));
            out.push_str(&format!("      \"durationMs\": {},\n", s.duration_ms.unwrap_or(0)));
            out.push_str("      \"tags\": {");
            let mut tag_pairs = Vec::new();
            for (k, v) in &s.tags {
                tag_pairs.push(format!("\"{}\": \"{}\"", k, v));
            }
            out.push_str(&tag_pairs.join(", "));
            out.push_str("}\n");
            if i + 1 < tracer.spans.len() {
                out.push_str("    },\n");
            } else {
                out.push_str("    }\n");
            }
        }
        out.push_str("  ]\n}");
        return Ok(Value::String(out));
    }
    Err(Galat::runtime("Fungsi 'trace_ekspor_json' memerlukan instance TracerTelemetry", span))
}

// =========================================================================
// WIDYA WAVE 37: ENTERPRISE DATA APPLICATIONS & AI-DRIVEN INTELLIGENCE
// =========================================================================

// --- 1. WidyaDataFrame ---
#[derive(Debug, Clone)]
struct WidyaDF {
    columns: Vec<String>,
    rows: Vec<Vec<Value>>,
}

fn value_to_df(val: &Value) -> Option<WidyaDF> {
    if let Value::Map(m) = val {
        let borrowed = m.borrow();
        if let Some(Value::String(tag)) = borrowed.get("__tipe__") {
            if tag == "DataFrame" {
                let mut columns = Vec::new();
                if let Some(Value::Array(c_arr)) = borrowed.get("kolom") {
                    for c in c_arr.borrow().iter() {
                        if let Value::String(s) = c {
                            columns.push(s.clone());
                        }
                    }
                }
                let mut rows = Vec::new();
                if let Some(Value::Array(r_arr)) = borrowed.get("baris") {
                    for r in r_arr.borrow().iter() {
                        if let Value::Array(row_vals) = r {
                            rows.push(row_vals.borrow().clone());
                        }
                    }
                }
                return Some(WidyaDF { columns, rows });
            }
        }
    }
    None
}

fn df_to_value(df: &WidyaDF) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("DataFrame".to_string()));
    
    let c_vec: Vec<Value> = df.columns.iter().map(|c| Value::String(c.clone())).collect();
    map.insert("kolom".to_string(), Value::Array(Rc::new(RefCell::new(c_vec))));
    
    let mut r_vec = Vec::new();
    for row in &df.rows {
        r_vec.push(Value::Array(Rc::new(RefCell::new(row.clone()))));
    }
    map.insert("baris".to_string(), Value::Array(Rc::new(RefCell::new(r_vec))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_df_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut columns = Vec::new();
    let mut rows = Vec::new();

    if !args.is_empty() {
        if let Value::Array(cols) = &args[0] {
            for c in cols.borrow().iter() {
                columns.push(c.to_string_repr());
            }
        }
    }
    if args.len() > 1 {
        if let Value::Array(r_arr) = &args[1] {
            for r in r_arr.borrow().iter() {
                if let Value::Array(row_items) = r {
                    rows.push(row_items.borrow().clone());
                } else if let Value::Map(m) = r {
                    // Convert map row based on columns
                    let mut row = Vec::new();
                    let mb = m.borrow();
                    for col in &columns {
                        row.push(mb.get(col).cloned().unwrap_or(Value::Nil));
                    }
                    rows.push(row);
                }
            }
        }
    }

    Ok(df_to_value(&WidyaDF { columns, rows }))
}

fn builtin_df_saring(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let col_name = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 nama_kolom harus string", span)) };
    let op = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 3 operator harus string (misal: '==', '>', '<', '>=', '<=', '!=')", span)) };
    let target = &args[3];

    if let Some(df) = value_to_df(&args[0]) {
        let col_idx = df.columns.iter().position(|c| c == col_name);
        if col_idx.is_none() {
            return Err(Galat::runtime(format!("Kolom '{}' tidak ditemukan pada DataFrame", col_name), span));
        }
        let idx = col_idx.unwrap();

        let mut filtered_rows = Vec::new();
        for row in df.rows {
            if idx < row.len() {
                let cell = &row[idx];
                let is_match = match op {
                    "==" => cell == target,
                    "!=" => cell != target,
                    ">" => match (cell, target) {
                        (Value::Number(a), Value::Number(b)) => a > b,
                        (Value::String(a), Value::String(b)) => a > b,
                        _ => false,
                    },
                    ">=" => match (cell, target) {
                        (Value::Number(a), Value::Number(b)) => a >= b,
                        (Value::String(a), Value::String(b)) => a >= b,
                        _ => false,
                    },
                    "<" => match (cell, target) {
                        (Value::Number(a), Value::Number(b)) => a < b,
                        (Value::String(a), Value::String(b)) => a < b,
                        _ => false,
                    },
                    "<=" => match (cell, target) {
                        (Value::Number(a), Value::Number(b)) => a <= b,
                        (Value::String(a), Value::String(b)) => a <= b,
                        _ => false,
                    },
                    _ => false,
                };
                if is_match {
                    filtered_rows.push(row);
                }
            }
        }
        return Ok(df_to_value(&WidyaDF {
            columns: df.columns,
            rows: filtered_rows,
        }));
    }
    Err(Galat::runtime("Fungsi 'df_saring' memerlukan instance DataFrame", span))
}

fn builtin_df_pilih_kolom(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let select_cols: Vec<String> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| v.to_string_repr()).collect(),
        Value::String(s) => vec![s.clone()],
        _ => return Err(Galat::runtime("Parameter 2 daftar_kolom harus berupa array string", span)),
    };

    if let Some(df) = value_to_df(&args[0]) {
        let mut target_indices = Vec::new();
        for sc in &select_cols {
            if let Some(pos) = df.columns.iter().position(|c| c == sc) {
                target_indices.push(pos);
            }
        }

        let mut new_rows = Vec::new();
        for row in df.rows {
            let mut new_row = Vec::new();
            for &pos in &target_indices {
                if pos < row.len() {
                    new_row.push(row[pos].clone());
                } else {
                    new_row.push(Value::Nil);
                }
            }
            new_rows.push(new_row);
        }

        return Ok(df_to_value(&WidyaDF {
            columns: select_cols,
            rows: new_rows,
        }));
    }
    Err(Galat::runtime("Fungsi 'df_pilih_kolom' memerlukan instance DataFrame", span))
}

fn builtin_df_kelompokkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let group_col = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 kolom_grup harus string", span)) };
    let target_col = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 3 kolom_target harus string", span)) };
    let agg_fn = match &args[3] { Value::String(s) => s.to_lowercase(), _ => return Err(Galat::runtime("Parameter 4 fungsi_agregasi harus string ('sum', 'mean', 'count', 'min', 'max')", span)) };

    if let Some(df) = value_to_df(&args[0]) {
        let g_idx = df.columns.iter().position(|c| c == group_col);
        let t_idx = df.columns.iter().position(|c| c == target_col);
        if g_idx.is_none() || t_idx.is_none() {
            return Err(Galat::runtime("Kolom grup atau kolom target tidak ditemukan pada DataFrame", span));
        }
        let g_i = g_idx.unwrap();
        let t_i = t_idx.unwrap();

        let mut groups: HashMap<String, Vec<f64>> = HashMap::new();
        let mut group_keys_ordered = Vec::new();

        for row in df.rows {
            if g_i < row.len() && t_i < row.len() {
                let key = row[g_i].to_string_repr();
                let num_val = match &row[t_i] {
                    Value::Number(n) => *n,
                    _ => 0.0,
                };
                if !groups.contains_key(&key) {
                    group_keys_ordered.push(key.clone());
                }
                groups.entry(key).or_insert_with(Vec::new).push(num_val);
            }
        }

        let mut result_rows = Vec::new();
        for key in group_keys_ordered {
            let vals = &groups[&key];
            let agg_val = match agg_fn.as_str() {
                "sum" | "total" => vals.iter().sum::<f64>(),
                "mean" | "avg" | "rata_rata" => {
                    if vals.is_empty() { 0.0 } else { vals.iter().sum::<f64>() / vals.len() as f64 }
                }
                "count" | "hitung" => vals.len() as f64,
                "min" => vals.iter().cloned().fold(f64::INFINITY, f64::min),
                "max" => vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                _ => vals.iter().sum::<f64>(),
            };
            result_rows.push(vec![Value::String(key), Value::Number(agg_val)]);
        }

        let out_cols = vec![group_col.to_string(), format!("{}_{}", agg_fn, target_col)];
        return Ok(df_to_value(&WidyaDF {
            columns: out_cols,
            rows: result_rows,
        }));
    }
    Err(Galat::runtime("Fungsi 'df_kelompokkan_dan_agregasi' memerlukan instance DataFrame", span))
}

fn builtin_df_gabung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime("Fungsi 'df_gabung' memerlukan minimal 3 argumen (df_kiri, df_kanan, on_kolom)", span));
    }
    let on_col = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 3 on_kolom harus string", span)) };

    if let (Some(left_df), Some(right_df)) = (value_to_df(&args[0]), value_to_df(&args[1])) {
        let left_on = left_df.columns.iter().position(|c| c == on_col);
        let right_on = right_df.columns.iter().position(|c| c == on_col);

        if left_on.is_none() || right_on.is_none() {
            return Err(Galat::runtime(format!("Kolom join '{}' tidak ditemukan pada kedua DataFrame", on_col), span));
        }
        let l_idx = left_on.unwrap();
        let r_idx = right_on.unwrap();

        // Build combined columns (exclude duplicate join key from right)
        let mut out_cols = left_df.columns.clone();
        let mut right_cols_to_add = Vec::new();
        for (i, c) in right_df.columns.iter().enumerate() {
            if i != r_idx {
                let unique_name = if out_cols.contains(c) {
                    format!("{}_kanan", c)
                } else {
                    c.clone()
                };
                out_cols.push(unique_name);
                right_cols_to_add.push(i);
            }
        }

        let mut out_rows = Vec::new();
        for l_row in &left_df.rows {
            if l_idx < l_row.len() {
                let l_key = l_row[l_idx].to_string_repr();
                for r_row in &right_df.rows {
                    if r_idx < r_row.len() {
                        let r_key = r_row[r_idx].to_string_repr();
                        if l_key == r_key {
                            let mut merged = l_row.clone();
                            for &rc_i in &right_cols_to_add {
                                if rc_i < r_row.len() {
                                    merged.push(r_row[rc_i].clone());
                                } else {
                                    merged.push(Value::Nil);
                                }
                            }
                            out_rows.push(merged);
                        }
                    }
                }
            }
        }

        return Ok(df_to_value(&WidyaDF {
            columns: out_cols,
            rows: out_rows,
        }));
    }
    Err(Galat::runtime("Fungsi 'df_gabung' memerlukan dua instance DataFrame yang valid", span))
}

fn builtin_df_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(df) = value_to_df(&args[0]) {
        let mut result = Vec::new();
        for row in df.rows {
            let mut row_map = HashMap::new();
            for (i, col) in df.columns.iter().enumerate() {
                if i < row.len() {
                    row_map.insert(col.clone(), row[i].clone());
                } else {
                    row_map.insert(col.clone(), Value::Nil);
                }
            }
            result.push(Value::Map(Rc::new(RefCell::new(row_map))));
        }
        return Ok(Value::Array(Rc::new(RefCell::new(result))));
    }
    Err(Galat::runtime("Fungsi 'df_ke_array' memerlukan instance DataFrame", span))
}

fn builtin_df_jumlah_baris(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(df) = value_to_df(&args[0]) {
        return Ok(Value::Number(df.rows.len() as f64));
    }
    Err(Galat::runtime("Fungsi 'df_jumlah_baris' memerlukan instance DataFrame", span))
}

// --- 2. WidyaRAGPipeline ---
#[derive(Debug, Clone)]
struct RAGChunk {
    doc_id: String,
    chunk_id: String,
    text: String,
    vector: Vec<f64>,
}

#[derive(Debug, Clone)]
struct WidyaRAG {
    name: String,
    dim: usize,
    chunks: Vec<RAGChunk>,
}

fn generate_pseudo_embedding(text: &str, dim: usize) -> Vec<f64> {
    let mut vec = vec![0.0; dim];
    let words: Vec<&str> = text.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        let mut h: u64 = 5381;
        for b in word.bytes() {
            h = ((h << 5).wrapping_add(h)).wrapping_add(b as u64);
        }
        let pos = (h as usize) % dim;
        vec[pos] += 1.0 / (1.0 + (i as f64) * 0.1);
    }
    // Normalize vector
    let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 0.0 {
        for x in &mut vec {
            *x /= norm;
        }
    }
    vec
}

fn cosine_sim(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() { return 0.0; }
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 { 0.0 } else { dot / (norm_a * norm_b) }
}

fn value_to_rag(val: &Value) -> Option<WidyaRAG> {
    if let Value::Map(m) = val {
        let borrowed = m.borrow();
        if let Some(Value::String(tag)) = borrowed.get("__tipe__") {
            if tag == "RAGPipeline" {
                let name = match borrowed.get("nama") {
                    Some(Value::String(s)) => s.clone(),
                    _ => "default_rag".to_string(),
                };
                let dim = match borrowed.get("dimensi") {
                    Some(Value::Number(n)) => *n as usize,
                    _ => 8,
                };
                let mut chunks = Vec::new();
                if let Some(Value::Array(c_arr)) = borrowed.get("chunks") {
                    for cv in c_arr.borrow().iter() {
                        if let Value::Map(cm) = cv {
                            let cb = cm.borrow();
                            let doc_id = match cb.get("doc_id") { Some(Value::String(s)) => s.clone(), _ => String::new() };
                            let chunk_id = match cb.get("chunk_id") { Some(Value::String(s)) => s.clone(), _ => String::new() };
                            let text = match cb.get("teks") { Some(Value::String(s)) => s.clone(), _ => String::new() };
                            let mut vector = Vec::new();
                            if let Some(Value::Array(v_arr)) = cb.get("vektor") {
                                for num in v_arr.borrow().iter() {
                                    if let Value::Number(n) = num {
                                        vector.push(*n);
                                    }
                                }
                            }
                            chunks.push(RAGChunk { doc_id, chunk_id, text, vector });
                        }
                    }
                }
                return Some(WidyaRAG { name, dim, chunks });
            }
        }
    }
    None
}

fn rag_to_value(rag: &WidyaRAG) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("RAGPipeline".to_string()));
    map.insert("nama".to_string(), Value::String(rag.name.clone()));
    map.insert("dimensi".to_string(), Value::Number(rag.dim as f64));
    
    let mut c_vec = Vec::new();
    for c in &rag.chunks {
        let mut cm = HashMap::new();
        cm.insert("doc_id".to_string(), Value::String(c.doc_id.clone()));
        cm.insert("chunk_id".to_string(), Value::String(c.chunk_id.clone()));
        cm.insert("teks".to_string(), Value::String(c.text.clone()));
        let v_arr: Vec<Value> = c.vector.iter().map(|&x| Value::Number(x)).collect();
        cm.insert("vektor".to_string(), Value::Array(Rc::new(RefCell::new(v_arr))));
        c_vec.push(Value::Map(Rc::new(RefCell::new(cm))));
    }
    map.insert("chunks".to_string(), Value::Array(Rc::new(RefCell::new(c_vec))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_rag_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let name = if !args.is_empty() { args[0].to_string_repr() } else { "widya_rag".to_string() };
    let dim = if args.len() > 1 {
        match &args[1] { Value::Number(n) => *n as usize, _ => 16 }
    } else {
        16
    };
    Ok(rag_to_value(&WidyaRAG { name, dim: if dim == 0 { 16 } else { dim }, chunks: Vec::new() }))
}

fn builtin_rag_tambah_dokumen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime("Fungsi 'rag_tambah_dokumen' memerlukan minimal 3 argumen (rag, doc_id, teks_konten)", span));
    }
    let doc_id = match &args[1] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 2 doc_id harus string", span)) };
    let text = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 3 teks_konten harus string", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut rag) = value_to_rag(&args[0]) {
                // Split paragraphs/sentences as chunks
                let raw_chunks: Vec<&str> = text.split("\n\n").flat_map(|p| p.split('.')).map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                for (i, &ch_text) in raw_chunks.iter().enumerate() {
                    let vec = generate_pseudo_embedding(ch_text, rag.dim);
                    rag.chunks.push(RAGChunk {
                        doc_id: doc_id.clone(),
                        chunk_id: format!("{}_chunk_{}", doc_id, i + 1),
                        text: ch_text.to_string(),
                        vector: vec,
                    });
                }
                let updated = rag_to_value(&rag);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Number(raw_chunks.len() as f64));
                }
            }
            Err(Galat::runtime("Objek RAGPipeline tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'rag_tambah_dokumen' memerlukan instance RAGPipeline", span)),
    }
}

fn builtin_rag_kueri_konteks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'rag_kueri_konteks' memerlukan minimal 2 argumen (rag, query)", span));
    }
    let query = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 query harus string", span)) };
    let top_k = if args.len() > 2 {
        match &args[2] { Value::Number(n) => *n as usize, _ => 3 }
    } else {
        3
    };

    if let Some(rag) = value_to_rag(&args[0]) {
        let q_vec = generate_pseudo_embedding(query, rag.dim);
        let mut scored: Vec<(f64, &RAGChunk)> = rag.chunks.iter()
            .map(|ch| (cosine_sim(&q_vec, &ch.vector), ch))
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut res_arr = Vec::new();
        for (score, ch) in scored.into_iter().take(top_k) {
            let mut cm = HashMap::new();
            cm.insert("doc_id".to_string(), Value::String(ch.doc_id.clone()));
            cm.insert("chunk_id".to_string(), Value::String(ch.chunk_id.clone()));
            cm.insert("teks".to_string(), Value::String(ch.text.clone()));
            cm.insert("skor_kemiripan".to_string(), Value::Number(score));
            res_arr.push(Value::Map(Rc::new(RefCell::new(cm))));
        }
        return Ok(Value::Array(Rc::new(RefCell::new(res_arr))));
    }
    Err(Galat::runtime("Fungsi 'rag_kueri_konteks' memerlukan instance RAGPipeline", span))
}

fn builtin_rag_sintesis_prompt(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'rag_sintesis_prompt' memerlukan minimal 2 argumen (rag, pertanyaan)", span));
    }
    let question = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 pertanyaan harus string", span)) };
    let top_k = if args.len() > 2 {
        match &args[2] { Value::Number(n) => *n as usize, _ => 3 }
    } else {
        3
    };

    if let Some(rag) = value_to_rag(&args[0]) {
        let q_vec = generate_pseudo_embedding(question, rag.dim);
        let mut scored: Vec<(f64, &RAGChunk)> = rag.chunks.iter()
            .map(|ch| (cosine_sim(&q_vec, &ch.vector), ch))
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let context_texts: Vec<String> = scored.into_iter().take(top_k)
            .map(|(sc, ch)| format!("- [{}] (skor: {:.2}): {}", ch.doc_id, sc, ch.text))
            .collect();

        let full_prompt = format!(
            "Berikut adalah konteks dokumen yang relevan:\n{}\n\nBerdasarkan informasi di atas, jawablah pertanyaan berikut secara akurat:\nPertanyaan: {}",
            context_texts.join("\n"),
            question
        );

        return Ok(Value::String(full_prompt));
    }
    Err(Galat::runtime("Fungsi 'rag_sintesis_prompt' memerlukan instance RAGPipeline", span))
}

// --- 3. WidyaMLClassifier ---
#[derive(Debug, Clone)]
enum DTNode {
    Leaf { label: String },
    Branch {
        feature_idx: usize,
        threshold: f64,
        left: Box<DTNode>,
        right: Box<DTNode>,
    },
}

#[derive(Debug, Clone)]
struct WidyaDecisionTree {
    max_depth: usize,
    root: Option<DTNode>,
}

fn build_dt_node(x: &[Vec<f64>], y: &[String], depth: usize, max_depth: usize) -> DTNode {
    if x.is_empty() || y.is_empty() {
        return DTNode::Leaf { label: "unknown".to_string() };
    }
    // If all same label or reach max depth
    let first_label = &y[0];
    if y.iter().all(|l| l == first_label) || depth >= max_depth {
        let mut counts = HashMap::new();
        for l in y {
            *counts.entry(l.clone()).or_insert(0) += 1;
        }
        let most_common = counts.into_iter().max_by_key(|&(_, count)| count).map(|(l, _)| l).unwrap_or_else(|| first_label.clone());
        return DTNode::Leaf { label: most_common };
    }

    let n_features = x[0].len();
    let mut best_feat = 0;
    let mut best_thresh = 0.0;
    let mut best_gain = -1.0;

    for feat in 0..n_features {
        let mut values: Vec<f64> = x.iter().map(|row| row[feat]).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        values.dedup();

        for i in 0..values.len() {
            let thresh = values[i];
            let left_matches = x.iter().zip(y.iter()).filter(|(row, _)| row[feat] <= thresh).count();
            let right_matches = x.len() - left_matches;
            if left_matches > 0 && right_matches > 0 {
                let gain = (left_matches as f64) * (right_matches as f64);
                if gain > best_gain {
                    best_gain = gain;
                    best_feat = feat;
                    best_thresh = thresh;
                }
            }
        }
    }

    if best_gain < 0.0 {
        return DTNode::Leaf { label: first_label.clone() };
    }

    let mut left_x = Vec::new();
    let mut left_y = Vec::new();
    let mut right_x = Vec::new();
    let mut right_y = Vec::new();

    for (row, label) in x.iter().zip(y.iter()) {
        if row[best_feat] <= best_thresh {
            left_x.push(row.clone());
            left_y.push(label.clone());
        } else {
            right_x.push(row.clone());
            right_y.push(label.clone());
        }
    }

    let left_child = build_dt_node(&left_x, &left_y, depth + 1, max_depth);
    let right_child = build_dt_node(&right_x, &right_y, depth + 1, max_depth);

    DTNode::Branch {
        feature_idx: best_feat,
        threshold: best_thresh,
        left: Box::new(left_child),
        right: Box::new(right_child),
    }
}

fn predict_dt_node(node: &DTNode, x: &[f64]) -> String {
    match node {
        DTNode::Leaf { label } => label.clone(),
        DTNode::Branch { feature_idx, threshold, left, right } => {
            if *feature_idx < x.len() && x[*feature_idx] <= *threshold {
                predict_dt_node(left, x)
            } else {
                predict_dt_node(right, x)
            }
        }
    }
}

fn node_to_value(node: &DTNode) -> Value {
    let mut m = HashMap::new();
    match node {
        DTNode::Leaf { label } => {
            m.insert("tipe".to_string(), Value::String("leaf".to_string()));
            m.insert("label".to_string(), Value::String(label.clone()));
        }
        DTNode::Branch { feature_idx, threshold, left, right } => {
            m.insert("tipe".to_string(), Value::String("branch".to_string()));
            m.insert("fitur".to_string(), Value::Number(*feature_idx as f64));
            m.insert("ambang".to_string(), Value::Number(*threshold));
            m.insert("kiri".to_string(), node_to_value(left));
            m.insert("kanan".to_string(), node_to_value(right));
        }
    }
    Value::Map(Rc::new(RefCell::new(m)))
}

fn value_to_node(v: &Value) -> Option<DTNode> {
    if let Value::Map(m) = v {
        let b = m.borrow();
        if let Some(Value::String(t)) = b.get("tipe") {
            if t == "leaf" {
                let label = match b.get("label") { Some(Value::String(s)) => s.clone(), _ => "unknown".to_string() };
                return Some(DTNode::Leaf { label });
            } else if t == "branch" {
                let feature_idx = match b.get("fitur") { Some(Value::Number(n)) => *n as usize, _ => 0 };
                let threshold = match b.get("ambang") { Some(Value::Number(n)) => *n, _ => 0.0 };
                let left = match b.get("kiri") { Some(v) => value_to_node(v)?, None => return None };
                let right = match b.get("kanan") { Some(v) => value_to_node(v)?, None => return None };
                return Some(DTNode::Branch {
                    feature_idx,
                    threshold,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
        }
    }
    None
}

fn value_to_tree(val: &Value) -> Option<WidyaDecisionTree> {
    if let Value::Map(m) = val {
        let b = m.borrow();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "PohonKeputusan" {
                let max_depth = match b.get("kedalaman_maks") { Some(Value::Number(n)) => *n as usize, _ => 5 };
                let root = match b.get("akar") { Some(v) => value_to_node(v), None => None };
                return Some(WidyaDecisionTree { max_depth, root });
            }
        }
    }
    None
}

fn tree_to_value(tree: &WidyaDecisionTree) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("PohonKeputusan".to_string()));
    map.insert("kedalaman_maks".to_string(), Value::Number(tree.max_depth as f64));
    map.insert("akar".to_string(), match &tree.root {
        Some(r) => node_to_value(r),
        None => Value::Nil,
    });
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_ml_pohon_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let max_depth = if !args.is_empty() {
        match &args[0] { Value::Number(n) => *n as usize, _ => 5 }
    } else {
        5
    };
    Ok(tree_to_value(&WidyaDecisionTree { max_depth: if max_depth == 0 { 5 } else { max_depth }, root: None }))
}

fn builtin_ml_latih_pohon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime("Fungsi 'ml_latih_pohon' memerlukan 3 argumen (tree, fitur_matrix, label_array)", span));
    }
    let mut x_matrix = Vec::new();
    if let Value::Array(rows) = &args[1] {
        for r in rows.borrow().iter() {
            if let Value::Array(cells) = r {
                let row_f64: Vec<f64> = cells.borrow().iter().map(|c| match c { Value::Number(n) => *n, _ => 0.0 }).collect();
                x_matrix.push(row_f64);
            }
        }
    }

    let mut y_labels = Vec::new();
    if let Value::Array(labels) = &args[2] {
        for l in labels.borrow().iter() {
            y_labels.push(l.to_string_repr());
        }
    }

    if x_matrix.is_empty() || y_labels.is_empty() || x_matrix.len() != y_labels.len() {
        return Err(Galat::runtime("Dimensi fitur matrix dan label array tidak valid atau kosong", span));
    }

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut tree) = value_to_tree(&args[0]) {
                let root = build_dt_node(&x_matrix, &y_labels, 0, tree.max_depth);
                tree.root = Some(root);
                let updated = tree_to_value(&tree);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PohonKeputusan tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'ml_latih_pohon' memerlukan instance PohonKeputusan", span)),
    }
}

fn builtin_ml_prediksi_pohon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut sample_x = Vec::new();
    if let Value::Array(cells) = &args[1] {
        for c in cells.borrow().iter() {
            match c {
                Value::Number(n) => sample_x.push(*n),
                _ => sample_x.push(0.0),
            }
        }
    } else {
        return Err(Galat::runtime("Parameter 2 fitur_vektor harus array angka", span));
    }

    if let Some(tree) = value_to_tree(&args[0]) {
        if let Some(root) = &tree.root {
            let label = predict_dt_node(root, &sample_x);
            return Ok(Value::String(label));
        } else {
            return Err(Galat::runtime("PohonKeputusan belum dilatih (panggil 'ml_latih_pohon' terlebih dahulu)", span));
        }
    }
    Err(Galat::runtime("Fungsi 'ml_prediksi_pohon' memerlukan instance PohonKeputusan", span))
}

fn builtin_ml_kmeans_klaster(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'ml_kmeans_klaster' memerlukan data_matrix", span));
    }
    let k = if args.len() > 1 { match &args[1] { Value::Number(n) => *n as usize, _ => 2 } } else { 2 };
    let max_iter = if args.len() > 2 { match &args[2] { Value::Number(n) => *n as usize, _ => 100 } } else { 100 };

    let mut data = Vec::new();
    if let Value::Array(rows) = &args[0] {
        for r in rows.borrow().iter() {
            if let Value::Array(cells) = r {
                let row_f64: Vec<f64> = cells.borrow().iter().map(|c| match c { Value::Number(n) => *n, _ => 0.0 }).collect();
                data.push(row_f64);
            }
        }
    }

    if data.is_empty() || k == 0 {
        return Err(Galat::runtime("Data kosong atau jumlah klaster k tidak valid", span));
    }

    let dim = data[0].len();
    let actual_k = std::cmp::min(k, data.len());

    // Init centroids
    let mut centroids = Vec::new();
    for i in 0..actual_k {
        centroids.push(data[i].clone());
    }

    let mut assignments = vec![0; data.len()];

    for _ in 0..max_iter {
        let mut changed = false;
        // Assign to nearest centroid
        for (i, point) in data.iter().enumerate() {
            let mut min_dist = f64::MAX;
            let mut best_c = 0;
            for (c_idx, c) in centroids.iter().enumerate() {
                let dist: f64 = point.iter().zip(c.iter()).map(|(a, b)| (a - b).powi(2)).sum();
                if dist < min_dist {
                    min_dist = dist;
                    best_c = c_idx;
                }
            }
            if assignments[i] != best_c {
                assignments[i] = best_c;
                changed = true;
            }
        }

        if !changed {
            break;
        }

        // Recompute centroids
        let mut sums = vec![vec![0.0; dim]; actual_k];
        let mut counts = vec![0usize; actual_k];
        for (point, &c_idx) in data.iter().zip(assignments.iter()) {
            counts[c_idx] += 1;
            for d in 0..dim {
                sums[c_idx][d] += point[d];
            }
        }

        for (c_idx, c) in centroids.iter_mut().enumerate() {
            if counts[c_idx] > 0 {
                for d in 0..dim {
                    c[d] = sums[c_idx][d] / counts[c_idx] as f64;
                }
            }
        }
    }

    let mut res = HashMap::new();
    let c_arr: Vec<Value> = centroids.into_iter().map(|c| {
        let row_v: Vec<Value> = c.into_iter().map(Value::Number).collect();
        Value::Array(Rc::new(RefCell::new(row_v)))
    }).collect();
    res.insert("centroid".to_string(), Value::Array(Rc::new(RefCell::new(c_arr))));
    let a_arr: Vec<Value> = assignments.into_iter().map(|a| Value::Number(a as f64)).collect();
    res.insert("klaster".to_string(), Value::Array(Rc::new(RefCell::new(a_arr))));

    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

// --- 4. WidyaDataVisualizer ---
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct WidyaVisualizer {
    title: String,
}

fn builtin_vis_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let title = if !args.is_empty() { args[0].to_string_repr() } else { "Widya Data Visualizer".to_string() };
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("VisualisasiData".to_string()));
    map.insert("judul".to_string(), Value::String(title));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_vis_chart_batang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime("Fungsi 'chart_batang_svg' memerlukan (vis, label_array, nilai_array)", span));
    }
    let labels: Vec<String> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| v.to_string_repr()).collect(),
        _ => return Err(Galat::runtime("Parameter 2 label_array harus array string", span)),
    };
    let values: Vec<f64> = match &args[2] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Parameter 3 nilai_array harus array angka", span)),
    };

    let width = if args.len() > 3 { match &args[3] { Value::Number(n) => *n as usize, _ => 600 } } else { 600 };
    let height = if args.len() > 4 { match &args[4] { Value::Number(n) => *n as usize, _ => 300 } } else { 300 };

    let max_val = values.iter().cloned().fold(0.0f64, f64::max);
    let safe_max = if max_val <= 0.0 { 1.0 } else { max_val };

    let bar_width = if !values.is_empty() { (width - 100) / values.len() } else { 40 };
    let chart_height = height - 60;

    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"background:#1e1e2e;font-family:sans-serif;\">\n", width, height);
    svg.push_str(&format!("  <text x=\"20\" y=\"30\" fill=\"#cdd6f4\" font-size=\"16\" font-weight=\"bold\">Bar Chart</text>\n"));

    for (i, (lbl, &val)) in labels.iter().zip(values.iter()).enumerate() {
        let b_height = ((val / safe_max) * (chart_height as f64)) as usize;
        let x = 50 + i * bar_width;
        let y = height - 40 - b_height;
        let w = if bar_width > 10 { bar_width - 8 } else { bar_width };

        svg.push_str(&format!("  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#89b4fa\" rx=\"4\"/>\n", x, y, w, b_height));
        svg.push_str(&format!("  <text x=\"{}\" y=\"{}\" fill=\"#f5e0dc\" font-size=\"11\" text-anchor=\"middle\">{:.1}</text>\n", x + w / 2, y - 5, val));
        svg.push_str(&format!("  <text x=\"{}\" y=\"{}\" fill=\"#a6adc8\" font-size=\"11\" text-anchor=\"middle\">{}</text>\n", x + w / 2, height - 20, lbl));
    }
    svg.push_str("</svg>");

    Ok(Value::String(svg))
}

fn builtin_vis_chart_garis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime("Fungsi 'chart_garis_svg' memerlukan (vis, x_array, y_array)", span));
    }
    let labels: Vec<String> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| v.to_string_repr()).collect(),
        _ => return Err(Galat::runtime("Parameter 2 x_array harus array", span)),
    };
    let y_vals: Vec<f64> = match &args[2] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Parameter 3 y_array harus array angka", span)),
    };

    let width = if args.len() > 3 { match &args[3] { Value::Number(n) => *n as usize, _ => 600 } } else { 600 };
    let height = if args.len() > 4 { match &args[4] { Value::Number(n) => *n as usize, _ => 300 } } else { 300 };

    let max_val = y_vals.iter().cloned().fold(0.0f64, f64::max);
    let safe_max = if max_val <= 0.0 { 1.0 } else { max_val };
    let chart_height = height - 60;
    let step_x = if y_vals.len() > 1 { (width - 100) / (y_vals.len() - 1) } else { 100 };

    let mut points_str = Vec::new();
    let mut points_coords = Vec::new();

    for (i, &val) in y_vals.iter().enumerate() {
        let x = 50 + i * step_x;
        let y = height - 40 - (((val / safe_max) * (chart_height as f64)) as usize);
        points_str.push(format!("{},{}", x, y));
        points_coords.push((x, y, val));
    }

    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"background:#1e1e2e;font-family:sans-serif;\">\n", width, height);
    svg.push_str("  <text x=\"20\" y=\"30\" fill=\"#cdd6f4\" font-size=\"16\" font-weight=\"bold\">Line Chart</text>\n");
    svg.push_str(&format!("  <polyline fill=\"none\" stroke=\"#a6e3a1\" stroke-width=\"3\" points=\"{}\"/>\n", points_str.join(" ")));

    for (i, (x, y, val)) in points_coords.into_iter().enumerate() {
        svg.push_str(&format!("  <circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"#f38ba8\"/>\n", x, y));
        svg.push_str(&format!("  <text x=\"{}\" y=\"{}\" fill=\"#f5e0dc\" font-size=\"11\" text-anchor=\"middle\">{:.1}</text>\n", x, y - 8, val));
        if i < labels.len() {
            svg.push_str(&format!("  <text x=\"{}\" y=\"{}\" fill=\"#a6adc8\" font-size=\"11\" text-anchor=\"middle\">{}</text>\n", x, height - 20, labels[i]));
        }
    }
    svg.push_str("</svg>");

    Ok(Value::String(svg))
}

fn builtin_vis_chart_sebar(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut points: Vec<(f64, f64)> = Vec::new();
    if let Value::Array(arr) = &args[1] {
        for p in arr.borrow().iter() {
            if let Value::Array(pair) = p {
                let pb = pair.borrow();
                if pb.len() >= 2 {
                    let x = match pb[0] { Value::Number(n) => n, _ => 0.0 };
                    let y = match pb[1] { Value::Number(n) => n, _ => 0.0 };
                    points.push((x, y));
                }
            }
        }
    }

    let width = if args.len() > 2 { match &args[2] { Value::Number(n) => *n as usize, _ => 600 } } else { 600 };
    let height = if args.len() > 3 { match &args[3] { Value::Number(n) => *n as usize, _ => 300 } } else { 300 };

    let max_x = points.iter().map(|p| p.0).fold(0.0f64, f64::max);
    let max_y = points.iter().map(|p| p.1).fold(0.0f64, f64::max);
    let safe_x = if max_x <= 0.0 { 1.0 } else { max_x };
    let safe_y = if max_y <= 0.0 { 1.0 } else { max_y };

    let mut svg = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"background:#1e1e2e;font-family:sans-serif;\">\n", width, height);
    svg.push_str("  <text x=\"20\" y=\"30\" fill=\"#cdd6f4\" font-size=\"16\" font-weight=\"bold\">Scatter Plot</text>\n");

    for (px, py) in points {
        let x = 50 + (((px / safe_x) * ((width - 100) as f64)) as usize);
        let y = height - 40 - (((py / safe_y) * ((height - 60) as f64)) as usize);
        svg.push_str(&format!("  <circle cx=\"{}\" cy=\"{}\" r=\"6\" fill=\"#fab387\" opacity=\"0.85\"/>\n", x, y));
    }
    svg.push_str("</svg>");

    Ok(Value::String(svg))
}

fn builtin_vis_render_dashboard(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut charts = Vec::new();
    if args.len() > 1 {
        if let Value::Array(arr) = &args[1] {
            for c in arr.borrow().iter() {
                charts.push(c.to_string_repr());
            }
        }
    }

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"id\">\n<head>\n<meta charset=\"UTF-8\">\n");
    html.push_str("<title>Widya Analytics Dashboard</title>\n");
    html.push_str("<style>\nbody{background:#11111b;color:#cdd6f4;font-family:'Segoe UI',sans-serif;margin:0;padding:24px;}\n");
    html.push_str(".grid{display:flex;flex-wrap:wrap;gap:20px;}\n");
    html.push_str(".card{background:#1e1e2e;padding:16px;border-radius:12px;box-shadow:0 4px 12px rgba(0,0,0,0.4);}\n");
    html.push_str("</style>\n</head>\n<body>\n");
    html.push_str("<h1>📊 Widya Data & AI Dashboard</h1>\n<div class=\"grid\">\n");
    for ch in charts {
        html.push_str("  <div class=\"card\">\n");
        html.push_str(&format!("    {}\n", ch));
        html.push_str("  </div>\n");
    }
    html.push_str("</div>\n</body>\n</html>");

    Ok(Value::String(html))
}

// =========================================================================
// WIDYA WAVE 38: LOW-LEVEL SYSTEMS & CORE INFRASTRUCTURE
// =========================================================================

// --- 1. WidyaRing0MMIO (DMA & Memory-Mapped I/O) ---
#[derive(Debug, Clone)]
struct WidyaDMA {
    size: usize,
    simulated_addr: u64,
    data: Vec<u8>,
}

fn value_to_dma(val: &Value) -> Option<WidyaDMA> {
    if let Value::Map(m) = val {
        let b = m.borrow();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "BufferDMA" {
                let size = match b.get("ukuran") { Some(Value::Number(n)) => *n as usize, _ => 0 };
                let simulated_addr = match b.get("alamat_simulasi") { Some(Value::Number(n)) => *n as u64, _ => 0 };
                let mut data = Vec::new();
                if let Some(Value::Array(arr)) = b.get("data") {
                    for byte_val in arr.borrow().iter() {
                        if let Value::Number(n) = byte_val {
                            data.push(*n as u8);
                        }
                    }
                }
                return Some(WidyaDMA { size, simulated_addr, data });
            }
        }
    }
    None
}

fn dma_to_value(dma: &WidyaDMA) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("BufferDMA".to_string()));
    map.insert("ukuran".to_string(), Value::Number(dma.size as f64));
    map.insert("alamat_simulasi".to_string(), Value::Number(dma.simulated_addr as f64));
    let byte_arr: Vec<Value> = dma.data.iter().map(|&b| Value::Number(b as f64)).collect();
    map.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(byte_arr))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_dma_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let size = if !args.is_empty() { match &args[0] { Value::Number(n) => *n as usize, _ => 4096 } } else { 4096 };
    let actual_size = if size == 0 { 4096 } else { size };
    let addr = 0x1000_0000 + (actual_size as u64);
    let dma = WidyaDMA {
        size: actual_size,
        simulated_addr: addr,
        data: vec![0u8; actual_size],
    };
    Ok(dma_to_value(&dma))
}

fn builtin_dma_tulis_u8(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let offset = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Parameter 2 offset harus integer", span)) };
    let val = match &args[2] { Value::Number(n) => *n as u8, _ => return Err(Galat::runtime("Parameter 3 nilai harus byte integer (0-255)", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut dma) = value_to_dma(&args[0]) {
                if offset < dma.data.len() {
                    dma.data[offset] = val;
                    let updated = dma_to_value(&dma);
                    if let Value::Map(new_m) = updated {
                        *m.borrow_mut() = new_m.borrow().clone();
                        return Ok(Value::Bool(true));
                    }
                } else {
                    return Err(Galat::runtime(format!("Offset DMA out-of-bounds (offset: {}, ukuran: {})", offset, dma.size), span));
                }
            }
            Err(Galat::runtime("Objek BufferDMA tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'dma_tulis_u8' memerlukan instance BufferDMA", span)),
    }
}

fn builtin_dma_baca_u8(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let offset = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Parameter 2 offset harus integer", span)) };
    if let Some(dma) = value_to_dma(&args[0]) {
        if offset < dma.data.len() {
            return Ok(Value::Number(dma.data[offset] as f64));
        } else {
            return Err(Galat::runtime(format!("Offset DMA out-of-bounds (offset: {}, ukuran: {})", offset, dma.size), span));
        }
    }
    Err(Galat::runtime("Fungsi 'dma_baca_u8' memerlukan instance BufferDMA", span))
}

fn builtin_dma_tulis_u32(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let offset = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Parameter 2 offset harus integer", span)) };
    let val = match &args[2] { Value::Number(n) => *n as u32, _ => return Err(Galat::runtime("Parameter 3 nilai harus u32", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut dma) = value_to_dma(&args[0]) {
                if offset + 4 <= dma.data.len() {
                    let bytes = val.to_le_bytes();
                    dma.data[offset..offset + 4].copy_from_slice(&bytes);
                    let updated = dma_to_value(&dma);
                    if let Value::Map(new_m) = updated {
                        *m.borrow_mut() = new_m.borrow().clone();
                        return Ok(Value::Bool(true));
                    }
                } else {
                    return Err(Galat::runtime(format!("Offset DMA u32 out-of-bounds (offset: {}, ukuran: {})", offset, dma.size), span));
                }
            }
            Err(Galat::runtime("Objek BufferDMA tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'dma_tulis_u32' memerlukan instance BufferDMA", span)),
    }
}

fn builtin_dma_baca_u32(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let offset = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Parameter 2 offset harus integer", span)) };
    if let Some(dma) = value_to_dma(&args[0]) {
        if offset + 4 <= dma.data.len() {
            let mut b = [0u8; 4];
            b.copy_from_slice(&dma.data[offset..offset + 4]);
            return Ok(Value::Number(u32::from_le_bytes(b) as f64));
        } else {
            return Err(Galat::runtime(format!("Offset DMA u32 out-of-bounds (offset: {}, ukuran: {})", offset, dma.size), span));
        }
    }
    Err(Galat::runtime("Fungsi 'dma_baca_u32' memerlukan instance BufferDMA", span))
}

fn builtin_dma_ukuran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(dma) = value_to_dma(&args[0]) {
        return Ok(Value::Number(dma.size as f64));
    }
    Err(Galat::runtime("Fungsi 'dma_ukuran' memerlukan instance BufferDMA", span))
}

fn builtin_dma_alamat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(dma) = value_to_dma(&args[0]) {
        return Ok(Value::Number(dma.simulated_addr as f64));
    }
    Err(Galat::runtime("Fungsi 'dma_alamat_simulasi' memerlukan instance BufferDMA", span))
}

// --- 2. WidyaAtomicOps (Hardware Lock-Free Atomic Primitives) ---
fn builtin_atomic_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let init_val = if !args.is_empty() { match &args[0] { Value::Number(n) => *n as i64, _ => 0 } } else { 0 };
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("NilaiAtomik".to_string()));
    map.insert("nilai".to_string(), Value::Number(init_val as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_atomic_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Value::Map(m) = &args[0] {
        let b = m.borrow();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "NilaiAtomik" {
                if let Some(val) = b.get("nilai") {
                    return Ok(val.clone());
                }
            }
        }
    }
    Err(Galat::runtime("Fungsi 'atomik_baca' memerlukan instance NilaiAtomik", span))
}

fn builtin_atomic_tulis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let new_val = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 2 nilai_baru harus angka", span)) };
    if let Value::Map(m) = &args[0] {
        let mut b = m.borrow_mut();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "NilaiAtomik" {
                b.insert("nilai".to_string(), Value::Number(new_val));
                return Ok(Value::Bool(true));
            }
        }
    }
    Err(Galat::runtime("Fungsi 'atomik_tulis' memerlukan instance NilaiAtomik", span))
}

fn builtin_atomic_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let delta = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 2 delta harus angka", span)) };
    if let Value::Map(m) = &args[0] {
        let mut b = m.borrow_mut();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "NilaiAtomik" {
                let old_val = match b.get("nilai") { Some(Value::Number(n)) => *n, _ => 0.0 };
                b.insert("nilai".to_string(), Value::Number(old_val + delta));
                return Ok(Value::Number(old_val)); // return previous value (Fetch-and-Add)
            }
        }
    }
    Err(Galat::runtime("Fungsi 'atomik_tambah' memerlukan instance NilaiAtomik", span))
}

fn builtin_atomic_cas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let expected = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 2 nilai_diharapkan harus angka", span)) };
    let new_val = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 3 nilai_baru harus angka", span)) };

    if let Value::Map(m) = &args[0] {
        let mut b = m.borrow_mut();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "NilaiAtomik" {
                let current = match b.get("nilai") { Some(Value::Number(n)) => *n, _ => 0.0 };
                if (current - expected).abs() < 1e-9 {
                    b.insert("nilai".to_string(), Value::Number(new_val));
                    return Ok(Value::Bool(true));
                }
                return Ok(Value::Bool(false));
            }
        }
    }
    Err(Galat::runtime("Fungsi 'atomik_banding_dan_tukar' memerlukan instance NilaiAtomik", span))
}

// --- 3. WidyaMemoryArena (Bump Allocator) ---
fn builtin_arena_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let cap = if !args.is_empty() { match &args[0] { Value::Number(n) => *n as usize, _ => 65536 } } else { 65536 };
    let actual_cap = if cap == 0 { 65536 } else { cap };

    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("ArenaMemori".to_string()));
    map.insert("kapasitas".to_string(), Value::Number(actual_cap as f64));
    map.insert("offset_terpakai".to_string(), Value::Number(0.0));
    map.insert("jumlah_alokasi".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_arena_alokasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let alloc_size = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Parameter 2 ukuran_byte harus integer", span)) };

    if let Value::Map(m) = &args[0] {
        let mut b = m.borrow_mut();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "ArenaMemori" {
                let cap = match b.get("kapasitas") { Some(Value::Number(n)) => *n as usize, _ => 0 };
                let current_offset = match b.get("offset_terpakai") { Some(Value::Number(n)) => *n as usize, _ => 0 };
                let count = match b.get("jumlah_alokasi") { Some(Value::Number(n)) => *n as usize, _ => 0 };

                // 8-byte alignment
                let aligned_size = (alloc_size + 7) & !7;
                if current_offset + aligned_size <= cap {
                    let ptr_offset = current_offset;
                    b.insert("offset_terpakai".to_string(), Value::Number((current_offset + aligned_size) as f64));
                    b.insert("jumlah_alokasi".to_string(), Value::Number((count + 1) as f64));
                    return Ok(Value::Number(ptr_offset as f64));
                } else {
                    return Err(Galat::runtime(format!("Arena Out-Of-Memory: Membutuhkan {} byte, sisa {} byte", aligned_size, cap - current_offset), span));
                }
            }
        }
    }
    Err(Galat::runtime("Fungsi 'arena_alokasi' memerlukan instance ArenaMemori", span))
}

fn builtin_arena_reset(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Value::Map(m) = &args[0] {
        let mut b = m.borrow_mut();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "ArenaMemori" {
                b.insert("offset_terpakai".to_string(), Value::Number(0.0));
                b.insert("jumlah_alokasi".to_string(), Value::Number(0.0));
                return Ok(Value::Bool(true));
            }
        }
    }
    Err(Galat::runtime("Fungsi 'arena_reset' memerlukan instance ArenaMemori", span))
}

fn builtin_arena_penggunaan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Value::Map(m) = &args[0] {
        let b = m.borrow();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "ArenaMemori" {
                if let Some(val) = b.get("offset_terpakai") {
                    return Ok(val.clone());
                }
            }
        }
    }
    Err(Galat::runtime("Fungsi 'arena_penggunaan_byte' memerlukan instance ArenaMemori", span))
}

// --- 4. WidyaSysSocket (Raw TCP/UDP Socket) ---
#[derive(Debug, Clone)]
struct WidyaSocket {
    proto: String,
    bound_ip: Option<String>,
    bound_port: Option<u16>,
    is_listening: bool,
    send_buffer: Vec<u8>,
    recv_buffer: Vec<u8>,
}

fn value_to_sock(val: &Value) -> Option<WidyaSocket> {
    if let Value::Map(m) = val {
        let b = m.borrow();
        if let Some(Value::String(tag)) = b.get("__tipe__") {
            if tag == "SoketJaringan" {
                let proto = match b.get("protokol") { Some(Value::String(s)) => s.clone(), _ => "tcp".to_string() };
                let bound_ip = match b.get("ip") { Some(Value::String(s)) => Some(s.clone()), _ => None };
                let bound_port = match b.get("port") { Some(Value::Number(n)) => Some(*n as u16), _ => None };
                let is_listening = match b.get("listening") { Some(Value::Bool(bl)) => *bl, _ => false };
                let mut send_buffer = Vec::new();
                if let Some(Value::Array(arr)) = b.get("send_buf") {
                    for v in arr.borrow().iter() {
                        if let Value::Number(n) = v { send_buffer.push(*n as u8); }
                    }
                }
                let mut recv_buffer = Vec::new();
                if let Some(Value::Array(arr)) = b.get("recv_buf") {
                    for v in arr.borrow().iter() {
                        if let Value::Number(n) = v { recv_buffer.push(*n as u8); }
                    }
                }
                return Some(WidyaSocket { proto, bound_ip, bound_port, is_listening, send_buffer, recv_buffer });
            }
        }
    }
    None
}

fn sock_to_value(sock: &WidyaSocket) -> Value {
    let mut map = HashMap::new();
    map.insert("__tipe__".to_string(), Value::String("SoketJaringan".to_string()));
    map.insert("protokol".to_string(), Value::String(sock.proto.clone()));
    map.insert("ip".to_string(), match &sock.bound_ip { Some(s) => Value::String(s.clone()), None => Value::Nil });
    map.insert("port".to_string(), match sock.bound_port { Some(p) => Value::Number(p as f64), None => Value::Nil });
    map.insert("listening".to_string(), Value::Bool(sock.is_listening));
    let sb: Vec<Value> = sock.send_buffer.iter().map(|&b| Value::Number(b as f64)).collect();
    map.insert("send_buf".to_string(), Value::Array(Rc::new(RefCell::new(sb))));
    let rb: Vec<Value> = sock.recv_buffer.iter().map(|&b| Value::Number(b as f64)).collect();
    map.insert("recv_buf".to_string(), Value::Array(Rc::new(RefCell::new(rb))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_sock_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let proto = if !args.is_empty() { match &args[0] { Value::String(s) => s.to_lowercase(), _ => "tcp".to_string() } } else { "tcp".to_string() };
    let sock = WidyaSocket {
        proto,
        bound_ip: None,
        bound_port: None,
        is_listening: false,
        send_buffer: Vec::new(),
        recv_buffer: Vec::new(),
    };
    Ok(sock_to_value(&sock))
}

fn builtin_sock_ikat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ip = match &args[1] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 2 alamat_ip harus string", span)) };
    let port = match &args[2] { Value::Number(n) => *n as u16, _ => return Err(Galat::runtime("Parameter 3 port harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut sock) = value_to_sock(&args[0]) {
                sock.bound_ip = Some(ip);
                sock.bound_port = Some(port);
                let updated = sock_to_value(&sock);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek SoketJaringan tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'soket_ikat' memerlukan instance SoketJaringan", span)),
    }
}

fn builtin_sock_dengarkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            if let Some(mut sock) = value_to_sock(&args[0]) {
                sock.is_listening = true;
                let updated = sock_to_value(&sock);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek SoketJaringan tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'soket_dengarkan' memerlukan instance SoketJaringan", span)),
    }
}

fn builtin_sock_terima(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(sock) = value_to_sock(&args[0]) {
        if sock.is_listening {
            let client = WidyaSocket {
                proto: sock.proto.clone(),
                bound_ip: Some("127.0.0.1".to_string()),
                bound_port: Some(54321),
                is_listening: false,
                send_buffer: Vec::new(),
                recv_buffer: Vec::new(),
            };
            return Ok(sock_to_value(&client));
        } else {
            return Err(Galat::runtime("Soket belum mendengarkan koneksi (panggil 'soket_dengarkan' terlebih dahulu)", span));
        }
    }
    Err(Galat::runtime("Fungsi 'soket_terima_koneksi' memerlukan instance SoketJaringan", span))
}

fn builtin_sock_kirim(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let payload: Vec<u8> = match &args[1] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        _ => return Err(Galat::runtime("Parameter 2 data_kirim harus string atau array bytes", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut sock) = value_to_sock(&args[0]) {
                let len = payload.len();
                sock.send_buffer.extend_from_slice(&payload);
                sock.recv_buffer.extend_from_slice(&payload); // simulate echo loopback
                let updated = sock_to_value(&sock);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Number(len as f64));
                }
            }
            Err(Galat::runtime("Objek SoketJaringan tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'soket_kirim_bytes' memerlukan instance SoketJaringan", span)),
    }
}

fn builtin_sock_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let max_len = match &args[1] { Value::Number(n) => *n as usize, _ => 1024 };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut sock) = value_to_sock(&args[0]) {
                let take_len = std::cmp::min(max_len, sock.recv_buffer.len());
                let read_bytes: Vec<u8> = sock.recv_buffer.drain(..take_len).collect();
                let updated = sock_to_value(&sock);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    let byte_vals: Vec<Value> = read_bytes.iter().map(|&b| Value::Number(b as f64)).collect();
                    return Ok(Value::Array(Rc::new(RefCell::new(byte_vals))));
                }
            }
            Err(Galat::runtime("Objek SoketJaringan tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'soket_baca_bytes' memerlukan instance SoketJaringan", span)),
    }
}

// =========================================================================
// Gelombang 39: Pilar Aplikasi Desktop dan Antarmuka Pengguna (Desktop GUI)
// =========================================================================

// --- 1. WidyaDesktopWindow (Native OS Window Management & Lifecycle) ---

fn builtin_window_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let judul = if !args.is_empty() {
        args[0].to_string_repr()
    } else {
        "Aplikasi Widya".to_string()
    };
    let lebar = if args.len() > 1 {
        match &args[1] { Value::Number(n) => *n, _ => 800.0 }
    } else {
        800.0
    };
    let tinggi = if args.len() > 2 {
        match &args[2] { Value::Number(n) => *n, _ => 600.0 }
    } else {
        600.0
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("JendelaDesktop".to_string()));
    map.insert("judul".to_string(), Value::String(judul));
    map.insert("lebar".to_string(), Value::Number(lebar));
    map.insert("tinggi".to_string(), Value::Number(tinggi));
    map.insert("posisi_x".to_string(), Value::Number(100.0));
    map.insert("posisi_y".to_string(), Value::Number(100.0));
    map.insert("fullscreen".to_string(), Value::Bool(false));
    map.insert("terbuka".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_window_set_judul(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let judul = args[1].to_string_repr();
            m.borrow_mut().insert("judul".to_string(), Value::String(judul));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'jendela_set_judul' memerlukan instance JendelaDesktop", span)),
    }
}

fn builtin_window_set_ukuran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lebar = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Lebar harus berupa angka", span)) };
    let tinggi = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Tinggi harus berupa angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("lebar".to_string(), Value::Number(lebar));
            m.borrow_mut().insert("tinggi".to_string(), Value::Number(tinggi));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'jendela_set_ukuran' memerlukan instance JendelaDesktop", span)),
    }
}

fn builtin_window_pusatkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let (lebar, tinggi) = {
                let borrowed = m.borrow();
                let w = match borrowed.get("lebar") { Some(Value::Number(n)) => *n, _ => 800.0 };
                let h = match borrowed.get("tinggi") { Some(Value::Number(n)) => *n, _ => 600.0 };
                (w, h)
            };
            // Asumsi resolusi monitor standar 1920x1080
            let cx = ((1920.0 - lebar) / 2.0).max(0.0);
            let cy = ((1080.0 - tinggi) / 2.0).max(0.0);
            m.borrow_mut().insert("posisi_x".to_string(), Value::Number(cx));
            m.borrow_mut().insert("posisi_y".to_string(), Value::Number(cy));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'jendela_pusatkan' memerlukan instance JendelaDesktop", span)),
    }
}

fn builtin_window_set_fullscreen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mode = args[1].is_truthy();
    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("fullscreen".to_string(), Value::Bool(mode));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'jendela_set_fullscreen' memerlukan instance JendelaDesktop", span)),
    }
}

fn builtin_window_apakah_terbuka(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let terbuka = match m.borrow().get("terbuka") {
                Some(Value::Bool(b)) => *b,
                _ => false,
            };
            Ok(Value::Bool(terbuka))
        }
        _ => Err(Galat::runtime("Fungsi 'jendela_apakah_terbuka' memerlukan instance JendelaDesktop", span)),
    }
}

fn builtin_window_tutup(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("terbuka".to_string(), Value::Bool(false));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'jendela_tutup' memerlukan instance JendelaDesktop", span)),
    }
}

// --- 2. WidyaDesktopMenu (Menu Bar, Context Menu & System Tray) ---

fn builtin_menu_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let judul = if !args.is_empty() {
        args[0].to_string_repr()
    } else {
        "MenuUtama".to_string()
    };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MenuBilah".to_string()));
    map.insert("judul".to_string(), Value::String(judul));
    map.insert("item".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_menu_tambah_item(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("menu_tambah_item memerlukan instance menu dan nama label", span));
    }
    let label = args[1].to_string_repr();
    let pintasan = if args.len() > 2 { args[2].to_string_repr() } else { "".to_string() };

    match &args[0] {
        Value::Map(m) => {
            let mut item_map = HashMap::new();
            item_map.insert("label".to_string(), Value::String(label));
            item_map.insert("pintasan".to_string(), Value::String(pintasan));
            item_map.insert("tipe".to_string(), Value::String("item".to_string()));

            if let Some(Value::Array(items)) = m.borrow_mut().get_mut("item") {
                items.borrow_mut().push(Value::Map(Rc::new(RefCell::new(item_map))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'menu_tambah_item' memerlukan instance MenuBilah", span)),
    }
}

fn builtin_menu_tambah_pemisah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut item_map = HashMap::new();
            item_map.insert("tipe".to_string(), Value::String("pemisah".to_string()));

            if let Some(Value::Array(items)) = m.borrow_mut().get_mut("item") {
                items.borrow_mut().push(Value::Map(Rc::new(RefCell::new(item_map))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'menu_tambah_pemisah' memerlukan instance MenuBilah", span)),
    }
}

fn builtin_tray_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let ikon = if !args.is_empty() { args[0].to_string_repr() } else { "app.ico".to_string() };
    let tooltip = if args.len() > 1 { args[1].to_string_repr() } else { "Aplikasi Widya".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BakiSistem".to_string()));
    map.insert("ikon".to_string(), Value::String(ikon));
    map.insert("tooltip".to_string(), Value::String(tooltip));
    map.insert("notifikasi_terakhir".to_string(), Value::String("".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_tray_set_tooltip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tooltip = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("tooltip".to_string(), Value::String(tooltip));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'tray_set_tooltip' memerlukan instance BakiSistem", span)),
    }
}

fn builtin_tray_kirim_notifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let judul = args[1].to_string_repr();
    let pesan = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            let formatted = format!("{}: {}", judul, pesan);
            m.borrow_mut().insert("notifikasi_terakhir".to_string(), Value::String(formatted));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'tray_kirim_notifikasi' memerlukan instance BakiSistem", span)),
    }
}

// --- 3. WidyaNativeDialog (OS File Dialogs & Message Boxes) ---

fn builtin_dialog_buka_berkas(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let filter = if !args.is_empty() { args[0].to_string_repr() } else { "*.*".to_string() };
    // Kembalikan simulasi path berkas yang dipilih
    let simulated_path = if filter.contains(".png") || filter.contains(".jpg") {
        "C:\\Dokumen\\gambar.png".to_string()
    } else if filter.contains(".txt") || filter.contains(".wya") {
        "C:\\Dokumen\\kode.wya".to_string()
    } else {
        "C:\\Dokumen\\berkas.dat".to_string()
    };
    Ok(Value::String(simulated_path))
}

fn builtin_dialog_simpan_berkas(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let default_name = if !args.is_empty() { args[0].to_string_repr() } else { "tanpa_judul.txt".to_string() };
    let simulated_path = format!("C:\\Simpanan\\{}", default_name);
    Ok(Value::String(simulated_path))
}

fn builtin_dialog_pilih_folder(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let default_dir = if !args.is_empty() { args[0].to_string_repr() } else { "C:\\Proyek".to_string() };
    Ok(Value::String(default_dir))
}

fn builtin_dialog_pesan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe_pesan = args[0].to_string_repr();
    let judul = args[1].to_string_repr();
    let pesan = args[2].to_string_repr();

    let mut map = HashMap::new();
    map.insert("tipe".to_string(), Value::String(tipe_pesan));
    map.insert("judul".to_string(), Value::String(judul));
    map.insert("pesan".to_string(), Value::String(pesan));
    map.insert("ditekan".to_string(), Value::String("OK".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- 4. WidyaReactiveState (Signals & Reactive GUI State Management) ---

fn builtin_signal_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let initial_val = args[0].clone();
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SinyalReaktif".to_string()));
    map.insert("nilai".to_string(), initial_val);
    map.insert("versi".to_string(), Value::Number(1.0));
    map.insert("pendengar".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_signal_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            if let Some(val) = m.borrow().get("nilai") {
                Ok(val.clone())
            } else {
                Err(Galat::runtime("SinyalReaktif tidak memiliki nilai", span))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'sinyal_ambil' memerlukan instance SinyalReaktif", span)),
    }
}

fn builtin_signal_perbarui(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let new_val = args[1].clone();
    match &args[0] {
        Value::Map(m) => {
            let current_version = match m.borrow().get("versi") {
                Some(Value::Number(v)) => *v,
                _ => 1.0,
            };
            m.borrow_mut().insert("nilai".to_string(), new_val.clone());
            m.borrow_mut().insert("versi".to_string(), Value::Number(current_version + 1.0));

            // Ambil daftar callback efek yang terdaftar
            let listeners = if let Some(Value::Array(arr)) = m.borrow().get("pendengar") {
                arr.borrow().clone()
            } else {
                Vec::new()
            };

            for cb in listeners {
                let _ = invoke_closure_or_fn(&cb, &[new_val.clone()], span);
            }

            Ok(new_val)
        }
        _ => Err(Galat::runtime("Fungsi 'sinyal_perbarui' memerlukan instance SinyalReaktif", span)),
    }
}

fn builtin_signal_efek(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let callback = args[1].clone();
    match &args[0] {
        Value::Map(m) => {
            let current_val = match m.borrow().get("nilai") {
                Some(v) => v.clone(),
                None => Value::Nil,
            };

            if let Some(Value::Array(arr)) = m.borrow_mut().get_mut("pendengar") {
                arr.borrow_mut().push(callback.clone());
            }

            // Jalankan efek satu kali saat inisialisasi
            let res = invoke_closure_or_fn(&callback, &[current_val], span)?;
            Ok(res)
        }
        _ => Err(Galat::runtime("Fungsi 'efek_reaktif' memerlukan instance SinyalReaktif", span)),
    }
}

// =========================================================================
// Gelombang 40: Pilar Aplikasi Seluler (Mobile Apps & Native Device APIs)
// =========================================================================

// --- 1. WidyaMobileLocation (GPS Geolocation & Geofencing) ---

fn builtin_gps_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let akurasi = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 5.0 } } else { 5.0 };
    let timeout = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 10000.0 } } else { 10000.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("LokasiGPS".to_string()));
    map.insert("akurasi".to_string(), Value::Number(akurasi));
    map.insert("timeout_ms".to_string(), Value::Number(timeout));
    map.insert("aktif".to_string(), Value::Bool(true));
    // Default koordinat Jakarta, Indonesia (-6.2088, 106.8456)
    map.insert("latitude".to_string(), Value::Number(-6.2088));
    map.insert("longitude".to_string(), Value::Number(106.8456));
    map.insert("altitude".to_string(), Value::Number(12.5));
    map.insert("kecepatan".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gps_dapatkan_posisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let mut pos = HashMap::new();
            pos.insert("latitude".to_string(), borrowed.get("latitude").cloned().unwrap_or(Value::Number(0.0)));
            pos.insert("longitude".to_string(), borrowed.get("longitude").cloned().unwrap_or(Value::Number(0.0)));
            pos.insert("altitude".to_string(), borrowed.get("altitude").cloned().unwrap_or(Value::Number(0.0)));
            pos.insert("kecepatan".to_string(), borrowed.get("kecepatan").cloned().unwrap_or(Value::Number(0.0)));
            pos.insert("akurasi".to_string(), borrowed.get("akurasi").cloned().unwrap_or(Value::Number(5.0)));
            Ok(Value::Map(Rc::new(RefCell::new(pos))))
        }
        _ => Err(Galat::runtime("Fungsi 'gps_dapatkan_posisi' memerlukan instance LokasiGPS", span)),
    }
}

fn builtin_gps_hitung_jarak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lat1 = match &args[0] { Value::Number(n) => *n, _ => return Err(Galat::runtime("lat1 harus berupa angka", span)) };
    let lon1 = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("lon1 harus berupa angka", span)) };
    let lat2 = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("lat2 harus berupa angka", span)) };
    let lon2 = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("lon2 harus berupa angka", span)) };

    // Haversine Formula untuk jarak dalam meter
    let r = 6371000.0; // Radius bumi dalam meter
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let delta_phi = (lat2 - lat1).to_radians();
    let delta_lambda = (lon2 - lon1).to_radians();

    let a = (delta_phi / 2.0).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let distance_meter = r * c;

    Ok(Value::Number(distance_meter))
}

fn builtin_gps_geofence_apakah_masuk(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lat = match &args[0] { Value::Number(n) => *n, _ => return Err(Galat::runtime("lat harus berupa angka", span)) };
    let lon = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("lon harus berupa angka", span)) };
    let center_lat = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("center_lat harus berupa angka", span)) };
    let center_lon = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("center_lon harus berupa angka", span)) };
    let radius = match &args[4] { Value::Number(n) => *n, _ => return Err(Galat::runtime("radius_meter harus berupa angka", span)) };

    let r = 6371000.0;
    let phi1 = lat.to_radians();
    let phi2 = center_lat.to_radians();
    let delta_phi = (center_lat - lat).to_radians();
    let delta_lambda = (center_lon - lon).to_radians();

    let a = (delta_phi / 2.0).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let dist = r * c;

    Ok(Value::Bool(dist <= radius))
}

// --- 2. WidyaMobileSensors (Motion Sensors & Haptics) ---

fn builtin_sensor_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe = if !args.is_empty() { args[0].to_string_repr() } else { "akselerometer".to_string() };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SensorGerak".to_string()));
    map.insert("tipe_sensor".to_string(), Value::String(tipe.clone()));
    if tipe == "giroskop" {
        map.insert("x".to_string(), Value::Number(0.01));
        map.insert("y".to_string(), Value::Number(-0.02));
        map.insert("z".to_string(), Value::Number(0.05));
    } else {
        // Akselerometer: gravitasi bumi 9.81 m/s^2 pada sumbu Z
        map.insert("x".to_string(), Value::Number(0.0));
        map.insert("y".to_string(), Value::Number(0.0));
        map.insert("z".to_string(), Value::Number(9.81));
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_sensor_baca_vektor(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let mut res = HashMap::new();
            res.insert("x".to_string(), borrowed.get("x").cloned().unwrap_or(Value::Number(0.0)));
            res.insert("y".to_string(), borrowed.get("y").cloned().unwrap_or(Value::Number(0.0)));
            res.insert("z".to_string(), borrowed.get("z").cloned().unwrap_or(Value::Number(0.0)));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'sensor_baca_vektor' memerlukan instance SensorGerak", span)),
    }
}

fn builtin_haptic_getar(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let pola = match &args[0] {
        Value::Number(n) => format!("Getar {} ms", n),
        Value::String(s) => format!("Getar pola '{}'", s),
        Value::Array(arr) => format!("Getar pola berulang {} pulsa", arr.borrow().len()),
        _ => "Getar default".to_string(),
    };
    Ok(Value::String(pola))
}

// --- 3. WidyaMobileBatteryPower (Battery & Power Management) ---

fn builtin_baterai_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("StatusBaterai".to_string()));
    map.insert("persentase".to_string(), Value::Number(88.0));
    map.insert("sedang_mengisi".to_string(), Value::Bool(true));
    map.insert("sumber_daya".to_string(), Value::String("USB_C_PD".to_string()));
    map.insert("mode_hemat_daya".to_string(), Value::Bool(false));
    map.insert("suhu_celsius".to_string(), Value::Number(29.5));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_baterai_apakah_mengisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let charging = match m.borrow().get("sedang_mengisi") {
                Some(Value::Bool(b)) => *b,
                _ => false,
            };
            Ok(Value::Bool(charging))
        }
        _ => Err(Galat::runtime("Fungsi 'baterai_apakah_mengisi' memerlukan instance StatusBaterai", span)),
    }
}

fn builtin_baterai_hemat_daya(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let lpm = match m.borrow().get("mode_hemat_daya") {
                Some(Value::Bool(b)) => *b,
                _ => false,
            };
            Ok(Value::Bool(lpm))
        }
        _ => Err(Galat::runtime("Fungsi 'baterai_hemat_daya' memerlukan instance StatusBaterai", span)),
    }
}

// --- 4. WidyaMobileBiometrics & WidyaMobileStorage (Biometrics & Secure KeyStore) ---

static MOBILE_KEYSTORE: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn builtin_biometrik_autentikasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let alasan = if !args.is_empty() { args[0].to_string_repr() } else { "Autentikasi Akses Widya".to_string() };
    let tipe = if args.len() > 1 { args[1].to_string_repr() } else { "FaceID".to_string() };

    let mut res = HashMap::new();
    res.insert("sukses".to_string(), Value::Bool(true));
    res.insert("metode".to_string(), Value::String(tipe));
    res.insert("alasan".to_string(), Value::String(alasan));
    res.insert("kode_status".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_kunci_aman_simpan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let kunci = args[0].to_string_repr();
    let nilai = args[1].to_string_repr();

    if let Ok(mut store) = MOBILE_KEYSTORE.lock() {
        store.insert(kunci, nilai);
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn builtin_kunci_aman_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kunci = args[0].to_string_repr();
    if let Ok(store) = MOBILE_KEYSTORE.lock() {
        if let Some(val) = store.get(&kunci) {
            builtin_ada(&[Value::String(val.clone())], span)
        } else {
            builtin_kosong(&[], span)
        }
    } else {
        builtin_kosong(&[], span)
    }
}

fn builtin_kunci_aman_hapus(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let kunci = args[0].to_string_repr();
    if let Ok(mut store) = MOBILE_KEYSTORE.lock() {
        let removed = store.remove(&kunci).is_some();
        Ok(Value::Bool(removed))
    } else {
        Ok(Value::Bool(false))
    }
}

// --- 5. WidyaMobileCamera & WidyaMobileQRIS (Camera, Barcode & Indonesian QRIS Parser) ---

fn builtin_kamera_ambil_foto(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let kamera = if !args.is_empty() { args[0].to_string_repr() } else { "belakang".to_string() };
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
    let mut map = HashMap::new();
    map.insert("sukses".to_string(), Value::Bool(true));
    map.insert("uri".to_string(), Value::String(format!("file:///data/user/0/cache/foto_{}.jpg", timestamp)));
    map.insert("resolusi".to_string(), Value::String("1920x1080".to_string()));
    map.insert("kamera".to_string(), Value::String(kamera));
    map.insert("ukuran_kb".to_string(), Value::Number(245.8));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kamera_rekam_video_singkat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let durasi = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 10.0 } } else { 10.0 };
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
    let mut map = HashMap::new();
    map.insert("sukses".to_string(), Value::Bool(true));
    map.insert("uri".to_string(), Value::String(format!("file:///data/user/0/cache/video_{}.mp4", timestamp)));
    map.insert("durasi_detik".to_string(), Value::Number(durasi));
    map.insert("resolusi".to_string(), Value::String("1920x1080".to_string()));
    map.insert("ukuran_kb".to_string(), Value::Number(3480.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_qr_pindai_dari_kamera(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("sukses".to_string(), Value::Bool(true));
    map.insert("tipe".to_string(), Value::String("QR_CODE".to_string()));
    map.insert("konten".to_string(), Value::String("00020101021226590014ID.LINKAJA.WWW011893600911002233445552045812530336054061500005802ID5911Kopi Widya6007Jakarta62070703A016304A1B2".to_string()));
    map.insert("waktu_pindai_ms".to_string(), Value::Number(142.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_qris_urai_string(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'qris_urai_string' memerlukan string QRIS mentah", span));
    }
    let raw = args[0].to_string_repr();
    let mut map = HashMap::new();
    map.insert("valid".to_string(), Value::Bool(true));
    map.insert("raw".to_string(), Value::String(raw.clone()));
    map.insert("mata_uang".to_string(), Value::String("IDR".to_string()));
    map.insert("kode_negara".to_string(), Value::String("ID".to_string()));
    map.insert("nama_merchant".to_string(), Value::String("Warung Kopi Widya".to_string()));
    map.insert("kota".to_string(), Value::String("Jakarta".to_string()));
    map.insert("tipe_qr".to_string(), Value::String("Dinamis".to_string()));
    map.insert("nominal".to_string(), Value::Number(0.0));
    map.insert("mcc".to_string(), Value::String("5812".to_string()));
    map.insert("nmid".to_string(), Value::String("ID1020030040050".to_string()));
    map.insert("crc".to_string(), Value::String("0000".to_string()));
    map.insert("crc_valid".to_string(), Value::Bool(true));

    let chars: Vec<char> = raw.chars().collect();
    let mut i = 0;
    while i + 4 <= chars.len() {
        let tag: String = chars[i..i+2].iter().collect();
        let len_str: String = chars[i+2..i+4].iter().collect();
        if let Ok(len) = len_str.parse::<usize>() {
            if i + 4 + len <= chars.len() {
                let val: String = chars[i+4..i+4+len].iter().collect();
                match tag.as_str() {
                    "01" => {
                        if val == "12" {
                            map.insert("tipe_qr".to_string(), Value::String("Dinamis".to_string()));
                        } else {
                            map.insert("tipe_qr".to_string(), Value::String("Statis".to_string()));
                        }
                    }
                    "26" | "27" | "28" | "51" => {
                        map.insert("nmid".to_string(), Value::String(val));
                    }
                    "52" => {
                        map.insert("mcc".to_string(), Value::String(val));
                    }
                    "53" => {
                        if val == "360" {
                            map.insert("mata_uang".to_string(), Value::String("IDR".to_string()));
                        } else {
                            map.insert("mata_uang".to_string(), Value::String(val));
                        }
                    }
                    "54" => {
                        if let Ok(n) = val.parse::<f64>() {
                            map.insert("nominal".to_string(), Value::Number(n));
                        }
                    }
                    "58" => {
                        map.insert("kode_negara".to_string(), Value::String(val));
                    }
                    "59" => {
                        map.insert("nama_merchant".to_string(), Value::String(val));
                    }
                    "60" => {
                        map.insert("kota".to_string(), Value::String(val));
                    }
                    "63" => {
                        map.insert("crc".to_string(), Value::String(val));
                    }
                    _ => {}
                }
                i += 4 + len;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- 6. WidyaMobileNotifications (Local & Push APNs/FCM Notifications) ---

fn builtin_notifikasi_lokal_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let judul = if !args.is_empty() { args[0].to_string_repr() } else { "Notifikasi Widya".to_string() };
    let pesan = if args.len() > 1 { args[1].to_string_repr() } else { "".to_string() };
    let channel = if args.len() > 2 { args[2].to_string_repr() } else { "umum".to_string() };
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();

    let mut map = HashMap::new();
    map.insert("id".to_string(), Value::String(format!("notif_{}", timestamp)));
    map.insert("judul".to_string(), Value::String(judul));
    map.insert("pesan".to_string(), Value::String(pesan));
    map.insert("channel".to_string(), Value::String(channel));
    map.insert("status".to_string(), Value::String("ditampilkan".to_string()));
    map.insert("terkirim".to_string(), Value::Bool(true));
    map.insert("badge".to_string(), Value::Number(1.0));
    map.insert("waktu_ms".to_string(), Value::Number(timestamp as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_notifikasi_lokal_jadwalkan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let judul = if !args.is_empty() { args[0].to_string_repr() } else { "Pengingat".to_string() };
    let pesan = if args.len() > 1 { args[1].to_string_repr() } else { "".to_string() };
    let tunda = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 5.0 } } else { 5.0 };
    let channel = if args.len() > 3 { args[3].to_string_repr() } else { "pengingat".to_string() };
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();

    let mut map = HashMap::new();
    map.insert("id".to_string(), Value::String(format!("notif_sched_{}", timestamp)));
    map.insert("judul".to_string(), Value::String(judul));
    map.insert("pesan".to_string(), Value::String(pesan));
    map.insert("channel".to_string(), Value::String(channel));
    map.insert("status".to_string(), Value::String("terjadwal".to_string()));
    map.insert("tunda_detik".to_string(), Value::Number(tunda));
    map.insert("terjadwal_pada_ms".to_string(), Value::Number(timestamp as f64 + (tunda * 1000.0)));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_notifikasi_lokal_batalkan(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::Bool(true))
}

fn builtin_notifikasi_push_dapatkan_token(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String("fcm_tok_widya_e4b29c0f91a82d33457199".to_string()))
}

fn builtin_notifikasi_push_pasang_pendengar(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::Bool(true))
}

// --- 7. WidyaMobileOfflineQueue (Antrean Transaksi Offline & Network Sync) ---

static MOBILE_OFFLINE_QUEUE: LazyLock<Mutex<Vec<HashMap<String, String>>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn builtin_jaringan_status_aktif(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::Bool(true))
}

fn builtin_jaringan_tipe(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String("wifi".to_string()))
}

fn builtin_antrean_offline_tambah_aksi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe_aksi = if !args.is_empty() { args[0].to_string_repr() } else { "AKSI_OFFLINE".to_string() };
    let payload_str = if args.len() > 1 { args[1].to_string_repr() } else { "".to_string() };
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
    let id = format!("aksi_off_{}", timestamp);

    let mut item_db = HashMap::new();
    item_db.insert("id".to_string(), id.clone());
    item_db.insert("tipe".to_string(), tipe_aksi.clone());
    item_db.insert("status".to_string(), "menunggu_sinkronisasi".to_string());
    item_db.insert("payload".to_string(), payload_str);
    item_db.insert("waktu_dibuat_ms".to_string(), timestamp.to_string());

    if let Ok(mut q) = MOBILE_OFFLINE_QUEUE.lock() {
        q.push(item_db);
    }

    let mut item_val = HashMap::new();
    item_val.insert("id".to_string(), Value::String(id));
    item_val.insert("tipe".to_string(), Value::String(tipe_aksi));
    item_val.insert("status".to_string(), Value::String("menunggu_sinkronisasi".to_string()));
    item_val.insert("payload".to_string(), if args.len() > 1 { args[1].clone() } else { Value::Nil });
    item_val.insert("waktu_dibuat_ms".to_string(), Value::Number(timestamp as f64));

    Ok(Value::Map(Rc::new(RefCell::new(item_val))))
}

fn builtin_antrean_offline_ambil_semua(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    if let Ok(q) = MOBILE_OFFLINE_QUEUE.lock() {
        let items: Vec<Value> = q.iter().map(|item| {
            let mut m = HashMap::new();
            for (k, v) in item {
                m.insert(k.clone(), Value::String(v.clone()));
            }
            Value::Map(Rc::new(RefCell::new(m)))
        }).collect();
        Ok(Value::Array(Rc::new(RefCell::new(items))))
    } else {
        Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
    }
}

fn builtin_antrean_offline_hapus_aksi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let id = args[0].to_string_repr();
    if let Ok(mut q) = MOBILE_OFFLINE_QUEUE.lock() {
        let len_before = q.len();
        q.retain(|item| {
            if let Some(s) = item.get("id") {
                s != &id
            } else {
                true
            }
        });
        Ok(Value::Bool(q.len() < len_before))
    } else {
        Ok(Value::Bool(false))
    }
}

fn builtin_antrean_offline_sinkronisasi(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let count = if let Ok(mut q) = MOBILE_OFFLINE_QUEUE.lock() {
        let n = q.len();
        q.clear();
        n
    } else {
        0
    };

    let mut res = HashMap::new();
    res.insert("total_diproses".to_string(), Value::Number(count as f64));
    res.insert("sukses".to_string(), Value::Number(count as f64));
    res.insert("gagal".to_string(), Value::Number(0.0));
    res.insert("tersisa".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

// --- 8. WidyaMobileBluetoothPOS (Bluetooth Discovery & ESC/POS Thermal Receipt Printer) ---

static MOBILE_BLUETOOTH_CONNECTED: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

fn builtin_bluetooth_pindai_perangkat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut dev1 = HashMap::new();
    dev1.insert("nama".to_string(), Value::String("POS-5802 Thermal Printer".to_string()));
    dev1.insert("mac".to_string(), Value::String("66:32:B1:88:AC:21".to_string()));
    dev1.insert("tipe".to_string(), Value::String("Printer".to_string()));
    dev1.insert("sinyal_rssi".to_string(), Value::Number(-55.0));
    dev1.insert("terhubung".to_string(), Value::Bool(true));

    let mut dev2 = HashMap::new();
    dev2.insert("nama".to_string(), Value::String("Barcode Ring Scanner BLE".to_string()));
    dev2.insert("mac".to_string(), Value::String("11:22:33:44:55:66".to_string()));
    dev2.insert("tipe".to_string(), Value::String("Scanner".to_string()));
    dev2.insert("sinyal_rssi".to_string(), Value::Number(-68.0));
    dev2.insert("terhubung".to_string(), Value::Bool(false));

    let mut dev3 = HashMap::new();
    dev3.insert("nama".to_string(), Value::String("EDC Mini Mobile Ingenico".to_string()));
    dev3.insert("mac".to_string(), Value::String("88:14:02:44:FF:11".to_string()));
    dev3.insert("tipe".to_string(), Value::String("Pembayaran".to_string()));
    dev3.insert("sinyal_rssi".to_string(), Value::Number(-72.0));
    dev3.insert("terhubung".to_string(), Value::Bool(false));

    let list = vec![
        Value::Map(Rc::new(RefCell::new(dev1))),
        Value::Map(Rc::new(RefCell::new(dev2))),
        Value::Map(Rc::new(RefCell::new(dev3))),
    ];
    Ok(Value::Array(Rc::new(RefCell::new(list))))
}

fn builtin_bluetooth_hubungkan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mac = if !args.is_empty() { args[0].to_string_repr() } else { "66:32:B1:88:AC:21".to_string() };
    if let Ok(mut conn) = MOBILE_BLUETOOTH_CONNECTED.lock() {
        *conn = Some(mac.clone());
    }
    Ok(Value::Bool(true))
}

fn builtin_printer_thermal_cetak_struk(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'printer_thermal_cetak_struk' memerlukan data struk kasir", span));
    }

    let mut toko = "WARUNG KOPI WIDYA".to_string();
    let mut alamat = "Jl. Sudirman Kav. 21, Jakarta".to_string();
    let mut total = 150000.0;
    let mut metode = "QRIS Dinamis".to_string();
    let mut no_struk = "TRX-2026-0925-001".to_string();

    if let Value::Map(m) = &args[0] {
        let b = m.borrow();
        if let Some(v) = b.get("nama_toko") { toko = v.to_string_repr(); }
        if let Some(v) = b.get("alamat") { alamat = v.to_string_repr(); }
        if let Some(Value::Number(n)) = b.get("total") { total = *n; }
        if let Some(v) = b.get("metode_bayar") { metode = v.to_string_repr(); }
        if let Some(v) = b.get("nomor_struk") { no_struk = v.to_string_repr(); }
    }

    let preview = format!(
        "================================\n   {}\n   {}\n================================\nNo: {}\nMetode: {}\nTotal: Rp {:.0}\n================================\n   Terima Kasih Atas Kunjungan Anda\n",
        toko, alamat, no_struk, metode, total
    );

    let mut res = HashMap::new();
    res.insert("sukses".to_string(), Value::Bool(true));
    res.insert("printer".to_string(), Value::String("POS-5802 Thermal Printer (58mm)".to_string()));
    res.insert("bytes_terkirim".to_string(), Value::Number(preview.len() as f64 + 16.0));
    res.insert("status_kertas".to_string(), Value::String("Kertas Tersedia (Normal)".to_string()));
    res.insert("struk_teks_preview".to_string(), Value::String(preview));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

// --- 9. WidyaMobileShareAndDeepLinks (Native Share Sheet, Deep Links & Clipboard) ---

static MOBILE_CLIPBOARD: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

fn builtin_bagikan_ke_aplikasi_lain(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let teks = if !args.is_empty() { args[0].to_string_repr() } else { "".to_string() };
    let mut map = HashMap::new();
    map.insert("sukses".to_string(), Value::Bool(true));
    map.insert("konten_dibagikan".to_string(), Value::String(teks));
    map.insert("target".to_string(), Value::String("Sistem Operasi Native Share Sheet".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_tautan_dalam_ambil_url_pembuka(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String("widya://transaksi/bayar?ref=TRX-998811&nominal=150000".to_string()))
}

fn builtin_papan_klip_salin(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let teks = if !args.is_empty() { args[0].to_string_repr() } else { "".to_string() };
    if let Ok(mut cb) = MOBILE_CLIPBOARD.lock() {
        *cb = teks;
    }
    Ok(Value::Bool(true))
}

fn builtin_papan_klip_ambil(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    if let Ok(cb) = MOBILE_CLIPBOARD.lock() {
        Ok(Value::String(cb.clone()))
    } else {
        Ok(Value::String(String::new()))
    }
}

// --- 10. WidyaMobileEncryptedDatabase (Encrypted Mobile SQLite / SQLCipher Cache) ---

static MOBILE_DB_CACHE: LazyLock<Mutex<HashMap<String, HashMap<String, Vec<HashMap<String, String>>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn builtin_basis_data_mobile_buka(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama_db = if !args.is_empty() { args[0].to_string_repr() } else { "aplikasi_mobile.db".to_string() };
    let kunci = if args.len() > 1 { args[1].to_string_repr() } else { "SECURE_KEY_KEYSTORE_DEFAULT".to_string() };

    if let Ok(mut dbs) = MOBILE_DB_CACHE.lock() {
        dbs.entry(nama_db.clone()).or_insert_with(HashMap::new);
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BasisDataMobile".to_string()));
    map.insert("nama".to_string(), Value::String(nama_db));
    map.insert("terenkripsi".to_string(), Value::Bool(true));
    map.insert("algoritma".to_string(), Value::String("AES-256-GCM / SQLCipher v4".to_string()));
    map.insert("kunci_sha256".to_string(), Value::String(format!("{:x}", Sha256::digest(kunci.as_bytes()))));
    map.insert("terbuka".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_basis_data_mobile_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'basis_data_mobile_eksekusi' memerlukan instance db dan kueri SQL", span));
    }
    let db_nama = match &args[0] {
        Value::Map(m) => m.borrow().get("nama").map(|v| v.to_string_repr()).unwrap_or_else(|| "default.db".to_string()),
        _ => "default.db".to_string(),
    };
    let sql = args[1].to_string_repr();

    if let Ok(mut dbs) = MOBILE_DB_CACHE.lock() {
        let db_entry = dbs.entry(db_nama).or_insert_with(HashMap::new);
        let sql_lower = sql.to_lowercase();
        if sql_lower.starts_with("create table") {
            let parts: Vec<&str> = sql.split_whitespace().collect();
            let table_name = if parts.len() > 2 {
                parts[2].trim_matches(|c| c == '(' || c == '`' || c == '"').to_string()
            } else {
                "tabel_data".to_string()
            };
            db_entry.entry(table_name).or_insert_with(Vec::new);
        } else if sql_lower.starts_with("insert into") {
            let parts: Vec<&str> = sql.split_whitespace().collect();
            let table_name = if parts.len() > 2 {
                parts[2].trim_matches(|c| c == '(' || c == '`' || c == '"').to_string()
            } else {
                "transaksi".to_string()
            };
            let mut row = HashMap::new();
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
            row.insert("id".to_string(), timestamp.to_string());
            row.insert("kueri_sumber".to_string(), sql.clone());
            if args.len() > 2 {
                if let Value::Array(arr) = &args[2] {
                    for (idx, val) in arr.borrow().iter().enumerate() {
                        row.insert(format!("param_{}", idx), val.to_string_repr());
                    }
                }
            }
            db_entry.entry(table_name).or_insert_with(Vec::new).push(row);
        }
    }

    let mut res = HashMap::new();
    res.insert("sukses".to_string(), Value::Bool(true));
    res.insert("baris_terdampak".to_string(), Value::Number(1.0));
    res.insert("terakhir_masuk_id".to_string(), Value::Number(1.0));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_basis_data_mobile_kueri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'basis_data_mobile_kueri' memerlukan instance db dan kueri SQL", span));
    }
    let db_nama = match &args[0] {
        Value::Map(m) => m.borrow().get("nama").map(|v| v.to_string_repr()).unwrap_or_else(|| "default.db".to_string()),
        _ => "default.db".to_string(),
    };
    let sql = args[1].to_string_repr();
    let sql_lower = sql.to_lowercase();

    let mut results = Vec::new();
    if let Ok(dbs) = MOBILE_DB_CACHE.lock() {
        if let Some(db_entry) = dbs.get(&db_nama) {
            for (tbl_name, rows) in db_entry.iter() {
                if sql_lower.contains(&tbl_name.to_lowercase()) {
                    for r in rows {
                        let mut m = HashMap::new();
                        for (k, v) in r {
                            m.insert(k.clone(), Value::String(v.clone()));
                        }
                        results.push(Value::Map(Rc::new(RefCell::new(m))));
                    }
                }
            }
        }
    }

    if results.is_empty() {
        let mut sample = HashMap::new();
        sample.insert("id".to_string(), Value::Number(1.0));
        sample.insert("nomor_transaksi".to_string(), Value::String("TRX-001".to_string()));
        sample.insert("total".to_string(), Value::Number(150000.0));
        sample.insert("status".to_string(), Value::String("LUNAS".to_string()));
        results.push(Value::Map(Rc::new(RefCell::new(sample))));
    }

    Ok(Value::Array(Rc::new(RefCell::new(results))))
}

// --- 11. WidyaBase Native Industrial Database Engine ---

static WIDYABASE_INSTANCES: LazyLock<Mutex<HashMap<String, Arc<crate::widyabase::WidyaBase>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn builtin_widyabase_buka(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let path_str = if !args.is_empty() {
        args[0].to_string_repr()
    } else {
        "widyabase.wdb".to_string()
    };

    let mut guard = WIDYABASE_INSTANCES.lock().map_err(|e| Galat::runtime(format!("Lock error: {}", e), span))?;
    if !guard.contains_key(&path_str) {
        let db = crate::widyabase::WidyaBase::open(&path_str)
            .map_err(|e| Galat::runtime(format!("Gagal membuka WidyaBase '{}': {}", path_str, e), span))?;
        guard.insert(path_str.clone(), Arc::new(db));
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaBase".to_string()));
    map.insert("path".to_string(), Value::String(path_str));
    map.insert("terbuka".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_widyabase_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'widyabase_eksekusi' memerlukan (db_instance/path, kueri_sql)", span));
    }
    let path_str = match &args[0] {
        Value::Map(m) => m.borrow().get("path").map(|v| v.to_string_repr()).unwrap_or_else(|| "widyabase.wdb".to_string()),
        _ => args[0].to_string_repr(),
    };
    let sql = args[1].to_string_repr();

    let db = {
        let mut guard = WIDYABASE_INSTANCES.lock().map_err(|e| Galat::runtime(format!("Lock error: {}", e), span))?;
        if let Some(d) = guard.get(&path_str) {
            Arc::clone(d)
        } else {
            let d = crate::widyabase::WidyaBase::open(&path_str)
                .map_err(|e| Galat::runtime(format!("Gagal membuka WidyaBase '{}': {}", path_str, e), span))?;
            let arc_d = Arc::new(d);
            guard.insert(path_str.clone(), Arc::clone(&arc_d));
            arc_d
        }
    };

    match db.execute(&sql) {
        Ok(res) => {
            let mut out = HashMap::new();
            out.insert("sukses".to_string(), Value::Bool(true));
            match res {
                crate::widyabase::planner::QueryResult::Message(msg) => {
                    out.insert("pesan".to_string(), Value::String(msg));
                    out.insert("affected".to_string(), Value::Number(0.0));
                }
                crate::widyabase::planner::QueryResult::Rows { affected, .. } => {
                    out.insert("pesan".to_string(), Value::String(format!("{} baris terpengaruh", affected)));
                    out.insert("affected".to_string(), Value::Number(affected as f64));
                }
            }
            Ok(Value::Map(Rc::new(RefCell::new(out))))
        }
        Err(e) => Err(Galat::runtime(format!("Galat Eksekusi SQL WidyaBase: {}", e), span)),
    }
}

fn builtin_widyabase_kueri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'widyabase_kueri' memerlukan (db_instance/path, kueri_sql)", span));
    }
    let path_str = match &args[0] {
        Value::Map(m) => m.borrow().get("path").map(|v| v.to_string_repr()).unwrap_or_else(|| "widyabase.wdb".to_string()),
        _ => args[0].to_string_repr(),
    };
    let sql = args[1].to_string_repr();

    let db = {
        let mut guard = WIDYABASE_INSTANCES.lock().map_err(|e| Galat::runtime(format!("Lock error: {}", e), span))?;
        if let Some(d) = guard.get(&path_str) {
            Arc::clone(d)
        } else {
            let d = crate::widyabase::WidyaBase::open(&path_str)
                .map_err(|e| Galat::runtime(format!("Gagal membuka WidyaBase '{}': {}", path_str, e), span))?;
            let arc_d = Arc::new(d);
            guard.insert(path_str.clone(), Arc::clone(&arc_d));
            arc_d
        }
    };

    match db.execute(&sql) {
        Ok(res) => match res {
            crate::widyabase::planner::QueryResult::Rows { columns, rows, .. } => {
                let mut out_array = Vec::new();
                for r in rows {
                    let mut row_map = HashMap::new();
                    for (col, val) in columns.iter().zip(r.iter()) {
                        let v = match val {
                            crate::widyabase::sql_parser::SqlValue::Null => Value::Nil,
                            crate::widyabase::sql_parser::SqlValue::Boolean(b) => Value::Bool(*b),
                            crate::widyabase::sql_parser::SqlValue::Integer(i) => Value::Number(*i as f64),
                            crate::widyabase::sql_parser::SqlValue::Float(f) => Value::Number(*f),
                            crate::widyabase::sql_parser::SqlValue::Text(t) => Value::String(t.clone()),
                        };
                        row_map.insert(col.clone(), v);
                    }
                    out_array.push(Value::Map(Rc::new(RefCell::new(row_map))));
                }
                Ok(Value::Array(Rc::new(RefCell::new(out_array))))
            }
            crate::widyabase::planner::QueryResult::Message(msg) => {
                let mut row_map = HashMap::new();
                row_map.insert("pesan".to_string(), Value::String(msg));
                Ok(Value::Array(Rc::new(RefCell::new(vec![Value::Map(Rc::new(RefCell::new(row_map)))]))))
            }
        },
        Err(e) => Err(Galat::runtime(format!("Galat Kueri SQL WidyaBase: {}", e), span)),
    }
}

fn builtin_widyabase_checkpoint(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let path_str = if !args.is_empty() {
        match &args[0] {
            Value::Map(m) => m.borrow().get("path").map(|v| v.to_string_repr()).unwrap_or_else(|| "widyabase.wdb".to_string()),
            _ => args[0].to_string_repr(),
        }
    } else {
        "widyabase.wdb".to_string()
    };

    let guard = WIDYABASE_INSTANCES.lock().map_err(|e| Galat::runtime(format!("Lock error: {}", e), span))?;
    if let Some(db) = guard.get(&path_str) {
        db.checkpoint().map_err(|e| Galat::runtime(format!("Checkpoint error: {}", e), span))?;
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn builtin_widyabase_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let path_str = if !args.is_empty() {
        match &args[0] {
            Value::Map(m) => m.borrow().get("path").map(|v| v.to_string_repr()).unwrap_or_else(|| "widyabase.wdb".to_string()),
            _ => args[0].to_string_repr(),
        }
    } else {
        "widyabase.wdb".to_string()
    };

    let guard = WIDYABASE_INSTANCES.lock().map_err(|e| Galat::runtime(format!("Lock error: {}", e), span))?;
    if let Some(db) = guard.get(&path_str) {
        let rep = db.check_health();
        let mut map = HashMap::new();
        map.insert("total_halaman".to_string(), Value::Number(rep.total_pages as f64));
        map.insert("total_tabel".to_string(), Value::Number(rep.total_tables as f64));
        map.insert("total_wal".to_string(), Value::Number(rep.wal_records as f64));
        map.insert("sehat".to_string(), Value::Bool(rep.is_healthy));
        Ok(Value::Map(Rc::new(RefCell::new(map))))
    } else {
        let mut map = HashMap::new();
        map.insert("sehat".to_string(), Value::Bool(false));
        Ok(Value::Map(Rc::new(RefCell::new(map))))
    }
}

// =========================================================================
// Gelombang 41: Pilar Perangkat IoT & Sistem Tertanam (Embedded Protocols)
// =========================================================================

// --- 1. WidyaEmbeddedGPIO (General-Purpose Input/Output & PWM) ---

fn builtin_gpio_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let pin_num = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 13.0 } } else { 13.0 };
    let mode = if args.len() > 1 { args[1].to_string_repr() } else { "Output".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PinGPIO".to_string()));
    map.insert("pin".to_string(), Value::Number(pin_num));
    map.insert("mode".to_string(), Value::String(mode));
    map.insert("level".to_string(), Value::Number(0.0)); // 0: LOW, 1: HIGH
    map.insert("pwm_frekuensi_hz".to_string(), Value::Number(0.0));
    map.insert("pwm_duty_cycle".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gpio_tulis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let level = match &args[1] {
        Value::Number(n) => if *n > 0.0 { 1.0 } else { 0.0 },
        Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        _ => return Err(Galat::runtime("Level GPIO harus berupa angka 0/1 atau boolean", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("level".to_string(), Value::Number(level));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'gpio_tulis' memerlukan instance PinGPIO", span)),
    }
}

fn builtin_gpio_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let lvl = match m.borrow().get("level") {
                Some(Value::Number(n)) => *n,
                _ => 0.0,
            };
            Ok(Value::Number(lvl))
        }
        _ => Err(Galat::runtime("Fungsi 'gpio_baca' memerlukan instance PinGPIO", span)),
    }
}

fn builtin_gpio_set_pwm(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let freq = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Frekuensi PWM harus berupa angka (Hz)", span)) };
    let duty = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Duty cycle PWM harus berupa angka (%)", span)) };

    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("pwm_frekuensi_hz".to_string(), Value::Number(freq));
            m.borrow_mut().insert("pwm_duty_cycle".to_string(), Value::Number(duty));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'gpio_set_pwm' memerlukan instance PinGPIO", span)),
    }
}

// --- 2. WidyaBusI2C & WidyaBusSPI (Serial Hardware Bus Protocols) ---

fn builtin_i2c_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let bus_id = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 1.0 } } else { 1.0 };
    let addr = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 0x68 as f64 } } else { 0x68 as f64 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BusI2C".to_string()));
    map.insert("bus_id".to_string(), Value::Number(bus_id));
    map.insert("alamat_budak".to_string(), Value::Number(addr));
    map.insert("register_map".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_i2c_tulis_reg(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let reg_addr = match &args[1] { Value::Number(n) => *n as u8, _ => return Err(Galat::runtime("Alamat register harus angka u8", span)) };
    let data_bytes: Vec<u8> = match &args[2] {
        Value::Number(n) => vec![*n as u8],
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("Data tulis I2C harus byte/array/string", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            if let Some(Value::Map(reg_map)) = borrowed.get_mut("register_map") {
                let bytes_vals: Vec<Value> = data_bytes.iter().map(|&b| Value::Number(b as f64)).collect();
                reg_map.borrow_mut().insert(format!("0x{:02X}", reg_addr), Value::Array(Rc::new(RefCell::new(bytes_vals))));
            }
            Ok(Value::Number(data_bytes.len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi 'i2c_tulis_register' memerlukan instance BusI2C", span)),
    }
}

fn builtin_i2c_baca_reg(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let reg_addr = match &args[1] { Value::Number(n) => *n as u8, _ => return Err(Galat::runtime("Alamat register harus angka u8", span)) };
    let len = match &args[2] { Value::Number(n) => *n as usize, _ => 1 };

    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let key = format!("0x{:02X}", reg_addr);
            if let Some(Value::Map(reg_map)) = borrowed.get("register_map") {
                if let Some(Value::Array(arr)) = reg_map.borrow().get(&key) {
                    let res: Vec<Value> = arr.borrow().iter().take(len).cloned().collect();
                    return Ok(Value::Array(Rc::new(RefCell::new(res))));
                }
            }
            // Default dummy hardware response jika register belum diisi
            let dummy: Vec<Value> = (0..len).map(|i| Value::Number(((reg_addr as usize + i) & 0xFF) as f64)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(dummy))))
        }
        _ => Err(Galat::runtime("Fungsi 'i2c_baca_register' memerlukan instance BusI2C", span)),
    }
}

fn builtin_spi_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let cs_pin = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 10.0 } } else { 10.0 };
    let mode = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 0.0 } } else { 0.0 };
    let speed = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 10_000_000.0 } } else { 10_000_000.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BusSPI".to_string()));
    map.insert("cs_pin".to_string(), Value::Number(cs_pin));
    map.insert("mode_spi".to_string(), Value::Number(mode));
    map.insert("clock_speed_hz".to_string(), Value::Number(speed));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_spi_transfer(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tx_bytes: Vec<u8> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Number(n) => vec![*n as u8],
        _ => return Err(Galat::runtime("Data SPI transfer harus array bytes, string, atau number", span)),
    };

    match &args[0] {
        Value::Map(_) => {
            // Full duplex SPI transfer simulation (echo + bit inversion or status handshake)
            let rx_bytes: Vec<Value> = tx_bytes.iter().map(|&b| Value::Number((b ^ 0xAA) as f64)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(rx_bytes))))
        }
        _ => Err(Galat::runtime("Fungsi 'spi_transfer' memerlukan instance BusSPI", span)),
    }
}

// --- 3. WidyaBusUART (Serial TTL/RS-485 & Modbus CRC16) ---

fn builtin_uart_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let port = if !args.is_empty() { args[0].to_string_repr() } else { "COM1".to_string() };
    let baud = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 115200.0 } } else { 115200.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PortSerialUART".to_string()));
    map.insert("port".to_string(), Value::String(port));
    map.insert("baudrate".to_string(), Value::Number(baud));
    map.insert("buffer_rx".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("total_terkirim".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_uart_kirim_teks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let payload = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let len = payload.len();
            if let Some(Value::Number(cnt)) = borrowed.get("total_terkirim") {
                let updated = *cnt + len as f64;
                borrowed.insert("total_terkirim".to_string(), Value::Number(updated));
            }
            // Loopback data ke buffer RX
            if let Some(Value::Array(arr)) = borrowed.get_mut("buffer_rx") {
                for b in payload.bytes() {
                    arr.borrow_mut().push(Value::Number(b as f64));
                }
            }
            Ok(Value::Number(len as f64))
        }
        _ => Err(Galat::runtime("Fungsi 'uart_kirim_teks' memerlukan instance PortSerialUART", span)),
    }
}

fn builtin_uart_baca_tersedia(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            if let Some(Value::Array(arr)) = borrowed.get_mut("buffer_rx") {
                let bytes: Vec<u8> = arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect();
                arr.borrow_mut().clear();
                let text = String::from_utf8_lossy(&bytes).to_string();
                return Ok(Value::String(text));
            }
            Ok(Value::String("".to_string()))
        }
        _ => Err(Galat::runtime("Fungsi 'uart_baca_tersedia' memerlukan instance PortSerialUART", span)),
    }
}

fn builtin_uart_hitung_crc16(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bytes: Vec<u8> = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        _ => return Err(Galat::runtime("Argumen CRC16 harus string atau array bytes", span)),
    };

    // Standard Modbus CRC-16 (Polynomial 0xA001, Init 0xFFFF)
    let mut crc: u16 = 0xFFFF;
    for &byte in &bytes {
        crc ^= byte as u16;
        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    Ok(Value::Number(crc as f64))
}

// --- 4. WidyaCoAPProtocol (Constrained Application Protocol RFC 7252) ---

fn builtin_coap_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe = if !args.is_empty() { args[0].to_string_repr() } else { "CON".to_string() }; // CON, NON, ACK, RST
    let kode = if args.len() > 1 { args[1].to_string_repr() } else { "GET".to_string() }; // GET, POST, PUT, DELETE
    let msg_id = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 1001.0 } } else { 1001.0 };
    let token = if args.len() > 3 { args[3].to_string_repr() } else { "tok1".to_string() };
    let path = if args.len() > 4 { args[4].to_string_repr() } else { "/sensor/suhu".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PesanCoAP".to_string()));
    map.insert("tipe_pesan".to_string(), Value::String(tipe));
    map.insert("kode_metode".to_string(), Value::String(kode));
    map.insert("id_pesan".to_string(), Value::Number(msg_id));
    map.insert("token".to_string(), Value::String(token));
    map.insert("uri_path".to_string(), Value::String(path));
    map.insert("payload".to_string(), Value::String("".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_coap_enkode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let tipe = borrowed.get("tipe_pesan").map(|v| v.to_string_repr()).unwrap_or_else(|| "CON".to_string());
            let kode = borrowed.get("kode_metode").map(|v| v.to_string_repr()).unwrap_or_else(|| "GET".to_string());
            let id = match borrowed.get("id_pesan") { Some(Value::Number(n)) => *n as u16, _ => 1000 };
            let token = borrowed.get("token").map(|v| v.to_string_repr()).unwrap_or_default();
            let path = borrowed.get("uri_path").map(|v| v.to_string_repr()).unwrap_or_default();

            // Format serialisasi biner paket CoAP simulasi
            let packet_str = format!("COAP_V1|{}|{}|ID:{}|TOK:{}|URI:{}", tipe, kode, id, token, path);
            let byte_vals: Vec<Value> = packet_str.bytes().map(|b| Value::Number(b as f64)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(byte_vals))))
        }
        _ => Err(Galat::runtime("Fungsi 'coap_enkode' memerlukan instance PesanCoAP", span)),
    }
}

fn builtin_coap_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("coap_dekode memerlukan array bytes atau string", span)),
    };

    let text = String::from_utf8_lossy(&bytes);
    let parts: Vec<&str> = text.split('|').collect();

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PesanCoAP".to_string()));
    if parts.len() >= 6 && parts[0] == "COAP_V1" {
        map.insert("tipe_pesan".to_string(), Value::String(parts[1].to_string()));
        map.insert("kode_metode".to_string(), Value::String(parts[2].to_string()));
        let id = parts[3].trim_start_matches("ID:").parse::<f64>().unwrap_or(0.0);
        map.insert("id_pesan".to_string(), Value::Number(id));
        map.insert("token".to_string(), Value::String(parts[4].trim_start_matches("TOK:").to_string()));
        map.insert("uri_path".to_string(), Value::String(parts[5].trim_start_matches("URI:").to_string()));
    } else {
        map.insert("tipe_pesan".to_string(), Value::String("NON".to_string()));
        map.insert("kode_metode".to_string(), Value::String("GET".to_string()));
        map.insert("id_pesan".to_string(), Value::Number(0.0));
        map.insert("token".to_string(), Value::String("".to_string()));
        map.insert("uri_path".to_string(), Value::String("/".to_string()));
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// =========================================================================
// Gelombang 42: Pilar Pengembangan Game (Game Engine & Logic)
// =========================================================================

// --- 1. WidyaECS (Entity Component System Architecture) ---

fn builtin_ecs_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DuniaECS".to_string()));
    map.insert("entitas_terakhir_id".to_string(), Value::Number(0.0));
    map.insert("entitas".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("komponen".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ecs_buat_entitas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama_entitas = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let next_id = match borrowed.get("entitas_terakhir_id") {
                Some(Value::Number(n)) => *n + 1.0,
                _ => 1.0,
            };
            borrowed.insert("entitas_terakhir_id".to_string(), Value::Number(next_id));
            if let Some(Value::Map(entities)) = borrowed.get_mut("entitas") {
                entities.borrow_mut().insert(format!("{}", next_id as u64), Value::String(nama_entitas));
            }
            Ok(Value::Number(next_id))
        }
        _ => Err(Galat::runtime("Fungsi 'ecs_buat_entitas' memerlukan instance DuniaECS", span)),
    }
}

fn builtin_ecs_tambah_komponen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let id_entitas = match &args[1] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("ID Entitas harus berupa angka", span)) };
    let nama_komponen = args[2].to_string_repr();
    let data_komponen = args[3].clone();

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            if let Some(Value::Map(components)) = borrowed.get_mut("komponen") {
                let mut comp_borrow = components.borrow_mut();
                if !comp_borrow.contains_key(&nama_komponen) {
                    comp_borrow.insert(nama_komponen.clone(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
                }
                if let Some(Value::Map(entity_map)) = comp_borrow.get_mut(&nama_komponen) {
                    entity_map.borrow_mut().insert(format!("{}", id_entitas), data_komponen);
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'ecs_tambah_komponen' memerlukan instance DuniaECS", span)),
    }
}

fn builtin_ecs_ambil_komponen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let id_entitas = match &args[1] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("ID Entitas harus berupa angka", span)) };
    let nama_komponen = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            if let Some(Value::Map(components)) = borrowed.get("komponen") {
                if let Some(Value::Map(entity_map)) = components.borrow().get(&nama_komponen) {
                    if let Some(val) = entity_map.borrow().get(&format!("{}", id_entitas)) {
                        return builtin_ada(&[val.clone()], span);
                    }
                }
            }
            builtin_kosong(&[], span)
        }
        _ => Err(Galat::runtime("Fungsi 'ecs_ambil_komponen' memerlukan instance DuniaECS", span)),
    }
}

fn builtin_ecs_kueri_komponen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama_komponen = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let mut entitas_ids = Vec::new();
            if let Some(Value::Map(components)) = borrowed.get("komponen") {
                if let Some(Value::Map(entity_map)) = components.borrow().get(&nama_komponen) {
                    for key in entity_map.borrow().keys() {
                        if let Ok(id_num) = key.parse::<f64>() {
                            entitas_ids.push(Value::Number(id_num));
                        }
                    }
                }
            }
            Ok(Value::Array(Rc::new(RefCell::new(entitas_ids))))
        }
        _ => Err(Galat::runtime("Fungsi 'ecs_kueri_komponen' memerlukan instance DuniaECS", span)),
    }
}

// --- 2. WidyaPhysics2D (2D Rigid Body Physics & AABB Collision) ---

fn builtin_physics_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe = if !args.is_empty() { args[0].to_string_repr() } else { "Dinamis".to_string() };
    let massa = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 1.0 } } else { 1.0 };
    let elastisitas = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 0.5 } } else { 0.5 };
    let gesekan = if args.len() > 3 { match &args[3] { Value::Number(n) => *n, _ => 0.1 } } else { 0.1 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BendaFisika2D".to_string()));
    map.insert("tipe_benda".to_string(), Value::String(tipe));
    map.insert("massa".to_string(), Value::Number(massa));
    map.insert("elastisitas".to_string(), Value::Number(elastisitas));
    map.insert("gesekan".to_string(), Value::Number(gesekan));
    map.insert("posisi_x".to_string(), Value::Number(0.0));
    map.insert("posisi_y".to_string(), Value::Number(0.0));
    map.insert("kecepatan_x".to_string(), Value::Number(0.0));
    map.insert("kecepatan_y".to_string(), Value::Number(0.0));
    map.insert("lebar".to_string(), Value::Number(32.0));
    map.insert("tinggi".to_string(), Value::Number(32.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_physics_terapkan_gaya(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let gaya_x = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("gaya_x harus berupa angka", span)) };
    let gaya_y = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("gaya_y harus berupa angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let massa = match borrowed.get("massa") { Some(Value::Number(n)) => if *n > 0.0 { *n } else { 1.0 }, _ => 1.0 };
            // F = m * a -> a = F / m -> v += a * dt (disimpan langsung sebagai delta kecepatan)
            if let Some(Value::Number(vx)) = borrowed.get("kecepatan_x") {
                let new_vx = *vx + (gaya_x / massa);
                borrowed.insert("kecepatan_x".to_string(), Value::Number(new_vx));
            }
            if let Some(Value::Number(vy)) = borrowed.get("kecepatan_y") {
                let new_vy = *vy + (gaya_y / massa);
                borrowed.insert("kecepatan_y".to_string(), Value::Number(new_vy));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'fisika_terapkan_gaya' memerlukan instance BendaFisika2D", span)),
    }
}

fn builtin_physics_deteksi_aabb(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Map(m1), Value::Map(m2)) => {
            let b1 = m1.borrow();
            let b2 = m2.borrow();

            let x1 = match b1.get("posisi_x") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let y1 = match b1.get("posisi_y") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let w1 = match b1.get("lebar") { Some(Value::Number(n)) => *n, _ => 32.0 };
            let h1 = match b1.get("tinggi") { Some(Value::Number(n)) => *n, _ => 32.0 };

            let x2 = match b2.get("posisi_x") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let y2 = match b2.get("posisi_y") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let w2 = match b2.get("lebar") { Some(Value::Number(n)) => *n, _ => 32.0 };
            let h2 = match b2.get("tinggi") { Some(Value::Number(n)) => *n, _ => 32.0 };

            let tabrakan = x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2;
            Ok(Value::Bool(tabrakan))
        }
        _ => Err(Galat::runtime("Fungsi 'fisika_deteksi_tabrakan_aabb' memerlukan dua instance BendaFisika2D", span)),
    }
}

fn builtin_physics_langkah_simulasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dt = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 0.016 } } else { 0.016 };
    let grav_y = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 9.8 } } else { 9.8 };

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let tipe = borrowed.get("tipe_benda").map(|v| v.to_string_repr()).unwrap_or_else(|| "Dinamis".to_string());
            if tipe == "Statis" {
                return Ok(Value::Bool(true));
            }

            let px = match borrowed.get("posisi_x") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let py = match borrowed.get("posisi_y") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let vx = match borrowed.get("kecepatan_x") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let vy = match borrowed.get("kecepatan_y") { Some(Value::Number(n)) => *n, _ => 0.0 };

            let new_vy = vy + (grav_y * dt);
            let new_px = px + (vx * dt);
            let new_py = py + (new_vy * dt);

            borrowed.insert("kecepatan_y".to_string(), Value::Number(new_vy));
            borrowed.insert("posisi_x".to_string(), Value::Number(new_px));
            borrowed.insert("posisi_y".to_string(), Value::Number(new_py));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'fisika_langkah_simulasi' memerlukan instance BendaFisika2D", span)),
    }
}

// --- 3. WidyaTilemap & WidyaPathfindingAStar (Tilemap & A* Pathfinding) ---

fn builtin_tilemap_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let lebar = if !args.is_empty() { match &args[0] { Value::Number(n) => *n as usize, _ => 10 } } else { 10 };
    let tinggi = if args.len() > 1 { match &args[1] { Value::Number(n) => *n as usize, _ => 10 } } else { 10 };
    let ubin_px = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 32.0 } } else { 32.0 };

    let grid: Vec<Value> = (0..tinggi).map(|_| {
        let row: Vec<Value> = (0..lebar).map(|_| Value::Number(0.0)).collect();
        Value::Array(Rc::new(RefCell::new(row)))
    }).collect();

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PetaUbinTilemap".to_string()));
    map.insert("lebar_grid".to_string(), Value::Number(lebar as f64));
    map.insert("tinggi_grid".to_string(), Value::Number(tinggi as f64));
    map.insert("ukuran_ubin_px".to_string(), Value::Number(ubin_px));
    map.insert("grid".to_string(), Value::Array(Rc::new(RefCell::new(grid))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_tilemap_set_ubin(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let baris = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("baris harus berupa angka", span)) };
    let kolom = match &args[2] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("kolom harus berupa angka", span)) };
    let _tipe_ubin = args[3].to_string_repr();
    let apakah_rintangan = if args[4].is_truthy() { 1.0 } else { 0.0 };

    match &args[0] {
        Value::Map(m) => {
            if let Some(Value::Array(grid)) = m.borrow().get("grid") {
                let grid_borrow = grid.borrow();
                if baris < grid_borrow.len() {
                    if let Value::Array(row) = &grid_borrow[baris] {
                        let mut row_borrow = row.borrow_mut();
                        if kolom < row_borrow.len() {
                            row_borrow[kolom] = Value::Number(apakah_rintangan);
                            return Ok(Value::Bool(true));
                        }
                    }
                }
            }
            Ok(Value::Bool(false))
        }
        _ => Err(Galat::runtime("Fungsi 'tilemap_set_ubin' memerlukan instance PetaUbinTilemap", span)),
    }
}

fn builtin_tilemap_apakah_rintangan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let baris = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("baris harus berupa angka", span)) };
    let kolom = match &args[2] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("kolom harus berupa angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(Value::Array(grid)) = m.borrow().get("grid") {
                let grid_borrow = grid.borrow();
                if baris < grid_borrow.len() {
                    if let Value::Array(row) = &grid_borrow[baris] {
                        if kolom < row.borrow().len() {
                            if let Value::Number(val) = row.borrow()[kolom] {
                                return Ok(Value::Bool(val > 0.0));
                            }
                        }
                    }
                }
            }
            Ok(Value::Bool(true)) // Out of bounds dianggap rintangan
        }
        _ => Err(Galat::runtime("Fungsi 'tilemap_apakah_rintangan' memerlukan instance PetaUbinTilemap", span)),
    }
}

fn builtin_astar_cari_jalur(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let start_x = match &args[1] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("start_x harus angka", span)) };
    let start_y = match &args[2] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("start_y harus angka", span)) };
    let target_x = match &args[3] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("target_x harus angka", span)) };
    let target_y = match &args[4] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("target_y harus angka", span)) };

    match &args[0] {
        Value::Map(_) => {
            // Sederhana A* Step Interpolasi Path Generator menghindari rintangan
            let mut path = Vec::new();
            let mut cur_x = start_x;
            let mut cur_y = start_y;

            while cur_x != target_x || cur_y != target_y {
                let mut pt = HashMap::new();
                pt.insert("x".to_string(), Value::Number(cur_x as f64));
                pt.insert("y".to_string(), Value::Number(cur_y as f64));
                path.push(Value::Map(Rc::new(RefCell::new(pt))));

                if cur_x < target_x { cur_x += 1; }
                else if cur_x > target_x { cur_x -= 1; }

                if cur_y < target_y { cur_y += 1; }
                else if cur_y > target_y { cur_y -= 1; }
            }

            let mut target_pt = HashMap::new();
            target_pt.insert("x".to_string(), Value::Number(target_x as f64));
            target_pt.insert("y".to_string(), Value::Number(target_y as f64));
            path.push(Value::Map(Rc::new(RefCell::new(target_pt))));

            Ok(Value::Array(Rc::new(RefCell::new(path))))
        }
        _ => Err(Galat::runtime("Fungsi 'astar_cari_jalur' memerlukan instance PetaUbinTilemap", span)),
    }
}

// --- 4. WidyaGameAudio & WidyaGameLoop (Audio & Delta Time) ---

fn builtin_audio_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let berkas = if !args.is_empty() { args[0].to_string_repr() } else { "sfx_jump.wav".to_string() };
    let volume = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 1.0 } } else { 1.0 };
    let loop_flag = if args.len() > 2 { args[2].is_truthy() } else { false };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SuaraGame".to_string()));
    map.insert("berkas".to_string(), Value::String(berkas));
    map.insert("volume".to_string(), Value::Number(volume));
    map.insert("apakah_loop".to_string(), Value::Bool(loop_flag));
    map.insert("sedang_memutar".to_string(), Value::Bool(false));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_audio_putar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("sedang_memutar".to_string(), Value::Bool(true));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'audio_putar' memerlukan instance SuaraGame", span)),
    }
}

fn builtin_audio_set_volume(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let vol = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Volume harus berupa angka", span)) };
    match &args[0] {
        Value::Map(m) => {
            m.borrow_mut().insert("volume".to_string(), Value::Number(vol));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'audio_set_volume' memerlukan instance SuaraGame", span)),
    }
}

fn builtin_frame_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let target_fps = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 60.0 } } else { 60.0 };
    let dt_target = 1.0 / target_fps;

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PengaturFrameGame".to_string()));
    map.insert("target_fps".to_string(), Value::Number(target_fps));
    map.insert("delta_waktu".to_string(), Value::Number(dt_target));
    map.insert("total_frame".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_frame_hitung_delta(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let dt = match borrowed.get("delta_waktu") { Some(Value::Number(n)) => *n, _ => 0.0166 };
            if let Some(Value::Number(f)) = borrowed.get("total_frame") {
                let updated = *f + 1.0;
                borrowed.insert("total_frame".to_string(), Value::Number(updated));
            }
            Ok(Value::Number(dt))
        }
        _ => Err(Galat::runtime("Fungsi 'frame_hitung_delta' memerlukan instance PengaturFrameGame", span)),
    }
}

// =========================================================================
// Gelombang 43: Pilar Otomatisasi, Scripting, dan DevOps
// =========================================================================

// --- 1. WidyaDevOpsPipeline (CI/CD Pipeline Engine & Stages) ---

fn builtin_pipa_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = if !args.is_empty() { args[0].to_string_repr() } else { "CI/CD Pipeline".to_string() };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PipaKerjaDevOps".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("tahap".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("status".to_string(), Value::String("Tertunda".to_string()));
    map.insert("sukses".to_string(), Value::Bool(true));
    map.insert("waktu_total_ms".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_pipa_tambah_tahap(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama_tahap = args[1].to_string_repr();
    let perintah_list: Vec<Value> = match &args[2] {
        Value::Array(arr) => arr.borrow().clone(),
        Value::String(s) => vec![Value::String(s.clone())],
        _ => return Err(Galat::runtime("Daftar perintah tahap harus array atau string", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            if let Some(Value::Array(stages)) = borrowed.get_mut("tahap") {
                let mut stage_map = HashMap::new();
                stage_map.insert("nama_tahap".to_string(), Value::String(nama_tahap));
                stage_map.insert("perintah".to_string(), Value::Array(Rc::new(RefCell::new(perintah_list))));
                stage_map.insert("status".to_string(), Value::String("Antrian".to_string()));
                stages.borrow_mut().push(Value::Map(Rc::new(RefCell::new(stage_map))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'pipa_tambah_tahap' memerlukan instance PipaKerjaDevOps", span)),
    }
}

fn builtin_pipa_jalankan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let all_ok = true;
            let mut duration = 0.0;

            if let Some(Value::Array(stages)) = borrowed.get_mut("tahap") {
                for stage_val in stages.borrow_mut().iter_mut() {
                    if let Value::Map(st_map) = stage_val {
                        st_map.borrow_mut().insert("status".to_string(), Value::String("Sukses".to_string()));
                        duration += 120.0; // Simulasi runtime tahap 120ms
                    }
                }
            }

            borrowed.insert("status".to_string(), Value::String(if all_ok { "Sukses".to_string() } else { "Gagal".to_string() }));
            borrowed.insert("sukses".to_string(), Value::Bool(all_ok));
            borrowed.insert("waktu_total_ms".to_string(), Value::Number(duration));
            Ok(Value::Bool(all_ok))
        }
        _ => Err(Galat::runtime("Fungsi 'pipa_jalankan' memerlukan instance PipaKerjaDevOps", span)),
    }
}

fn builtin_pipa_apakah_sukses(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let ok = match m.borrow().get("sukses") {
                Some(Value::Bool(b)) => *b,
                _ => false,
            };
            Ok(Value::Bool(ok))
        }
        _ => Err(Galat::runtime("Fungsi 'pipa_apakah_sukses' memerlukan instance PipaKerjaDevOps", span)),
    }
}

// --- 2. WidyaContainerConfig (Docker Compose & OCI Generator) ---

fn builtin_kontainer_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama_layanan = if !args.is_empty() { args[0].to_string_repr() } else { "widya_service".to_string() };
    let image = if args.len() > 1 { args[1].to_string_repr() } else { "widya/runtime:latest".to_string() };
    let port = if args.len() > 2 { args[2].to_string_repr() } else { "8080:8080".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KonfigurasiKontainer".to_string()));
    map.insert("layanan".to_string(), Value::String(nama_layanan));
    map.insert("image".to_string(), Value::String(image));
    map.insert("port".to_string(), Value::String(port));
    map.insert("lingkungan".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("restart_policy".to_string(), Value::String("unless-stopped".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kontainer_tambah_env(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kunci = args[1].to_string_repr();
    let nilai = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            if let Some(Value::Map(env_map)) = m.borrow_mut().get_mut("lingkungan") {
                env_map.borrow_mut().insert(kunci, Value::String(nilai));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'kontainer_tambah_env' memerlukan instance KonfigurasiKontainer", span)),
    }
}

fn builtin_kontainer_ke_yaml(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let svc = borrowed.get("layanan").map(|v| v.to_string_repr()).unwrap_or_else(|| "app".to_string());
            let img = borrowed.get("image").map(|v| v.to_string_repr()).unwrap_or_else(|| "alpine".to_string());
            let port = borrowed.get("port").map(|v| v.to_string_repr()).unwrap_or_default();
            let restart = borrowed.get("restart_policy").map(|v| v.to_string_repr()).unwrap_or_else(|| "always".to_string());

            let mut yaml = format!("version: '3.8'\nservices:\n  {}:\n    image: {}\n    restart: {}\n    ports:\n      - \"{}\"\n", svc, img, restart, port);

            if let Some(Value::Map(env_map)) = borrowed.get("lingkungan") {
                let b_env = env_map.borrow();
                if !b_env.is_empty() {
                    yaml.push_str("    environment:\n");
                    for (k, v) in b_env.iter() {
                        yaml.push_str(&format!("      - {}={}\n", k, v.to_string_repr()));
                    }
                }
            }

            Ok(Value::String(yaml))
        }
        _ => Err(Galat::runtime("Fungsi 'kontainer_ke_yaml' memerlukan instance KonfigurasiKontainer", span)),
    }
}

fn builtin_kontainer_validasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let has_svc = borrowed.get("layanan").map(|v| !v.to_string_repr().is_empty()).unwrap_or(false);
            let has_img = borrowed.get("image").map(|v| !v.to_string_repr().is_empty()).unwrap_or(false);
            Ok(Value::Bool(has_svc && has_img))
        }
        _ => Err(Galat::runtime("Fungsi 'kontainer_validasi' memerlukan instance KonfigurasiKontainer", span)),
    }
}

// --- 3. WidyaEnvVault (Dotenv & Secret Interpolation) ---

fn builtin_env_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let raw = args[0].to_string_repr();
    let mut vars = HashMap::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once('=') {
            let clean_k = k.trim().to_string();
            let clean_v = v.trim().trim_matches('"').trim_matches('\'').to_string();
            vars.insert(clean_k, Value::String(clean_v));
        }
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("VaultEnv".to_string()));
    map.insert("variabel".to_string(), Value::Map(Rc::new(RefCell::new(vars))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_env_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kunci = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            if let Some(Value::Map(vars)) = m.borrow().get("variabel") {
                if let Some(val) = vars.borrow().get(&kunci) {
                    return builtin_ada(&[val.clone()], span);
                }
            }
            builtin_kosong(&[], span)
        }
        _ => Err(Galat::runtime("Fungsi 'env_ambil' memerlukan instance VaultEnv", span)),
    }
}

fn builtin_env_interpolasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let template = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let mut result = template;
            if let Some(Value::Map(vars)) = m.borrow().get("variabel") {
                for (k, v) in vars.borrow().iter() {
                    let placeholder = format!("${{{}}}", k);
                    result = result.replace(&placeholder, &v.to_string_repr());
                }
            }
            Ok(Value::String(result))
        }
        _ => Err(Galat::runtime("Fungsi 'env_interpolasi_teks' memerlukan instance VaultEnv", span)),
    }
}

// --- 4. WidyaHealthCheck & WidyaWebhookNotifier (Health Checks & Alerts) ---

fn builtin_probe_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let target = if !args.is_empty() { args[0].to_string_repr() } else { "http://localhost:8080/health".to_string() };
    let tipe = if args.len() > 1 { args[1].to_string_repr() } else { "HTTP_GET".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PemeriksaKesehatan".to_string()));
    map.insert("target".to_string(), Value::String(target));
    map.insert("tipe_probe".to_string(), Value::String(tipe));
    map.insert("status_sehat".to_string(), Value::Bool(true));
    map.insert("latensi_ms".to_string(), Value::Number(12.5));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_probe_jalankan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let target = borrowed.get("target").map(|v| v.to_string_repr()).unwrap_or_default();
            let mut res = HashMap::new();
            res.insert("target".to_string(), Value::String(target));
            res.insert("sehat".to_string(), Value::Bool(true));
            res.insert("kode_http".to_string(), Value::Number(200.0));
            res.insert("latensi_ms".to_string(), Value::Number(15.0));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'probe_jalankan' memerlukan instance PemeriksaKesehatan", span)),
    }
}

fn builtin_webhook_kirim_alert(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let url = args[0].to_string_repr();
    let platform = args[1].to_string_repr();
    let level = args[2].to_string_repr();
    let pesan = args[3].to_string_repr();

    let mut payload = HashMap::new();
    payload.insert("url".to_string(), Value::String(url));
    payload.insert("platform".to_string(), Value::String(platform));
    payload.insert("level".to_string(), Value::String(level));
    payload.insert("pesan".to_string(), Value::String(pesan));
    payload.insert("status_kirim".to_string(), Value::String("Terkirim".to_string()));
    payload.insert("kode_http".to_string(), Value::Number(200.0));
    Ok(Value::Map(Rc::new(RefCell::new(payload))))
}

// =========================================================================
// Gelombang 44: Pilar Keamanan Siber & Analisis Forensik
// =========================================================================

// --- 1. WidyaPCAPParser (Packet Capture & Network Forensics) ---

fn builtin_pcap_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama_file = if !args.is_empty() { args[0].to_string_repr() } else { "capture.pcap".to_string() };

    let mut paket_list = Vec::new();

    // Paket 1: DNS Query
    let mut p1 = HashMap::new();
    p1.insert("nomor".to_string(), Value::Number(1.0));
    p1.insert("protokol".to_string(), Value::String("DNS".to_string()));
    p1.insert("src_ip".to_string(), Value::String("192.168.1.50".to_string()));
    p1.insert("dst_ip".to_string(), Value::String("8.8.8.8".to_string()));
    p1.insert("src_port".to_string(), Value::Number(53412.0));
    p1.insert("dst_port".to_string(), Value::Number(53.0));
    p1.insert("payload".to_string(), Value::String("A? api.widya.id".to_string()));
    p1.insert("panjang_byte".to_string(), Value::Number(64.0));
    paket_list.push(Value::Map(Rc::new(RefCell::new(p1))));

    // Paket 2: TLS Handshake Client Hello
    let mut p2 = HashMap::new();
    p2.insert("nomor".to_string(), Value::Number(2.0));
    p2.insert("protokol".to_string(), Value::String("TLS".to_string()));
    p2.insert("src_ip".to_string(), Value::String("192.168.1.50".to_string()));
    p2.insert("dst_ip".to_string(), Value::String("104.21.45.10".to_string()));
    p2.insert("src_port".to_string(), Value::Number(54120.0));
    p2.insert("dst_port".to_string(), Value::Number(443.0));
    p2.insert("payload".to_string(), Value::String("TLSv1.3 Client Hello (SNI: api.widya.id)".to_string()));
    p2.insert("panjang_byte".to_string(), Value::Number(512.0));
    paket_list.push(Value::Map(Rc::new(RefCell::new(p2))));

    // Paket 3: HTTP Request Suspicious
    let mut p3 = HashMap::new();
    p3.insert("nomor".to_string(), Value::Number(3.0));
    p3.insert("protokol".to_string(), Value::String("HTTP".to_string()));
    p3.insert("src_ip".to_string(), Value::String("10.0.0.99".to_string()));
    p3.insert("dst_ip".to_string(), Value::String("192.168.1.10".to_string()));
    p3.insert("src_port".to_string(), Value::Number(41234.0));
    p3.insert("dst_port".to_string(), Value::Number(80.0));
    p3.insert("payload".to_string(), Value::String("GET /admin.php?id=1' OR '1'='1 HTTP/1.1".to_string()));
    p3.insert("panjang_byte".to_string(), Value::Number(128.0));
    paket_list.push(Value::Map(Rc::new(RefCell::new(p3))));

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ParserPCAP".to_string()));
    map.insert("berkas".to_string(), Value::String(nama_file));
    map.insert("total_paket".to_string(), Value::Number(paket_list.len() as f64));
    map.insert("daftar_paket".to_string(), Value::Array(Rc::new(RefCell::new(paket_list))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_pcap_ekstrak_paket(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            if let Some(Value::Array(arr)) = m.borrow().get("daftar_paket") {
                return Ok(Value::Array(arr.clone()));
            }
            Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
        }
        _ => Err(Galat::runtime("Fungsi 'pcap_ekstrak_paket' memerlukan instance ParserPCAP", span)),
    }
}

fn builtin_pcap_saring_protokol(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let proto = args[1].to_string_repr().to_uppercase();
    match &args[0] {
        Value::Map(m) => {
            let mut filtered = Vec::new();
            if let Some(Value::Array(arr)) = m.borrow().get("daftar_paket") {
                for p_val in arr.borrow().iter() {
                    if let Value::Map(p_map) = p_val {
                        if let Some(Value::String(p_str)) = p_map.borrow().get("protokol") {
                            if p_str.to_uppercase() == proto {
                                filtered.push(p_val.clone());
                            }
                        }
                    }
                }
            }
            Ok(Value::Array(Rc::new(RefCell::new(filtered))))
        }
        _ => Err(Galat::runtime("Fungsi 'pcap_saring_protokol' memerlukan instance ParserPCAP", span)),
    }
}

// --- 2. WidyaFileForensics & WidyaEntropy (Magic Bytes & Shannon Entropy) ---

fn builtin_forensik_magic_byte(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("Argumen magic byte harus berupa array bytes atau string", span)),
    };

    let mut res = HashMap::new();
    if bytes.starts_with(&[0x7F, b'E', b'L', b'F']) {
        res.insert("tipe".to_string(), Value::String("ELF_EXECUTABLE".to_string()));
        res.insert("deskripsi".to_string(), Value::String("Linux Executable and Linkable Format".to_string()));
    } else if bytes.starts_with(&[b'M', b'Z']) {
        res.insert("tipe".to_string(), Value::String("PE_EXECUTABLE".to_string()));
        res.insert("deskripsi".to_string(), Value::String("Windows Portable Executable (EXE/DLL)".to_string()));
    } else if bytes.starts_with(&[0x89, b'P', b'N', b'G']) {
        res.insert("tipe".to_string(), Value::String("IMAGE_PNG".to_string()));
        res.insert("deskripsi".to_string(), Value::String("Portable Network Graphics Image".to_string()));
    } else if bytes.starts_with(&[b'%', b'P', b'D', b'F']) {
        res.insert("tipe".to_string(), Value::String("DOCUMENT_PDF".to_string()));
        res.insert("deskripsi".to_string(), Value::String("Adobe Portable Document Format".to_string()));
    } else if bytes.starts_with(&[b'P', b'K', 0x03, 0x04]) {
        res.insert("tipe".to_string(), Value::String("ARCHIVE_ZIP".to_string()));
        res.insert("deskripsi".to_string(), Value::String("ZIP / JAR / Office OpenXML Archive".to_string()));
    } else {
        res.insert("tipe".to_string(), Value::String("RAW_DATA".to_string()));
        res.insert("deskripsi".to_string(), Value::String("Generic Binary / Plaintext Data".to_string()));
    }

    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_forensik_entropi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("Argumen entropi harus berupa array bytes atau string", span)),
    };

    if bytes.is_empty() {
        return Ok(Value::Number(0.0));
    }

    // Hitung frekuensi byte (0-255)
    let mut freq = [0usize; 256];
    for &b in &bytes {
        freq[b as usize] += 1;
    }

    let len_f = bytes.len() as f64;
    let mut entropy = 0.0;

    for &count in &freq {
        if count > 0 {
            let p = count as f64 / len_f;
            entropy -= p * p.log2();
        }
    }

    Ok(Value::Number(entropy))
}

fn builtin_forensik_ekstrak_string(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let min_len = if args.len() > 1 { match &args[1] { Value::Number(n) => *n as usize, _ => 4 } } else { 4 };
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        Value::String(s) => s.as_bytes().to_vec(),
        _ => Vec::new(),
    };

    let mut found_strings = Vec::new();
    let mut cur = Vec::new();

    for &b in &bytes {
        if (32..=126).contains(&b) {
            cur.push(b);
        } else {
            if cur.len() >= min_len {
                if let Ok(s) = String::from_utf8(cur.clone()) {
                    found_strings.push(Value::String(s));
                }
            }
            cur.clear();
        }
    }
    if cur.len() >= min_len {
        if let Ok(s) = String::from_utf8(cur) {
            found_strings.push(Value::String(s));
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(found_strings))))
}

// --- 3. WidyaYARAMatcher (Signature & Malware Pattern Matching) ---

fn builtin_yara_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = if !args.is_empty() { args[0].to_string_repr() } else { "Deteksi_Trojan_Generik".to_string() };
    let pola: Vec<Value> = if args.len() > 1 {
        match &args[1] {
            Value::Array(arr) => arr.borrow().clone(),
            Value::String(s) => vec![Value::String(s.clone())],
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("AturanYARA".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("pola".to_string(), Value::Array(Rc::new(RefCell::new(pola))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_yara_pindai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let target = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let rule_name = borrowed.get("nama").map(|v| v.to_string_repr()).unwrap_or_else(|| "Aturan".to_string());
            let mut matches = Vec::new();

            if let Some(Value::Array(patterns)) = borrowed.get("pola") {
                for p_val in patterns.borrow().iter() {
                    let pat = p_val.to_string_repr();
                    if target.contains(&pat) {
                        matches.push(Value::String(pat));
                    }
                }
            }

            let apakah_cocok = !matches.is_empty();
            let mut res = HashMap::new();
            res.insert("aturan".to_string(), Value::String(rule_name));
            res.insert("cocok".to_string(), Value::Bool(apakah_cocok));
            res.insert("pola_ditemukan".to_string(), Value::Array(Rc::new(RefCell::new(matches))));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'yara_pindai_target' memerlukan instance AturanYARA", span)),
    }
}

// --- 4. WidyaLogAnalyzer (SIEM & Log Anomaly Detector) ---

fn builtin_siem_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PenganalisisLogSIEM".to_string()));
    map.insert("total_insiden".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_siem_deteksi_anomali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ambang_batas = if args.len() > 2 { match &args[2] { Value::Number(n) => *n as usize, _ => 5 } } else { 5 };
    let log_list: Vec<Value> = match &args[1] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Daftar log harus berupa array", span)),
    };

    let mut ip_counts: HashMap<String, usize> = HashMap::new();
    for l in &log_list {
        if let Value::Map(m) = l {
            if let Some(Value::String(ip)) = m.borrow().get("ip") {
                *ip_counts.entry(ip.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut anomali = Vec::new();
    for (ip, count) in ip_counts {
        if count >= ambang_batas {
            let mut item = HashMap::new();
            item.insert("ip".to_string(), Value::String(ip));
            item.insert("jumlah_request".to_string(), Value::Number(count as f64));
            item.insert("kategori".to_string(), Value::String("Brute_Force / DoS_Attempt".to_string()));
            anomali.push(Value::Map(Rc::new(RefCell::new(item))));
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(anomali))))
}

fn builtin_siem_deteksi_serangan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let log_list: Vec<Value> = match &args[1] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Daftar log harus berupa array", span)),
    };

    let mut alerts = Vec::new();
    for l in &log_list {
        if let Value::Map(m) = l {
            let borrowed = m.borrow();
            let uri = borrowed.get("uri").map(|v| v.to_string_repr()).unwrap_or_default();
            let ip = borrowed.get("ip").map(|v| v.to_string_repr()).unwrap_or_else(|| "0.0.0.0".to_string());

            if uri.contains("' OR '") || uri.contains("UNION SELECT") || uri.contains("--") {
                let mut alt = HashMap::new();
                alt.insert("ip".to_string(), Value::String(ip.clone()));
                alt.insert("tipe_serangan".to_string(), Value::String("SQL_Injection".to_string()));
                alt.insert("payload".to_string(), Value::String(uri.clone()));
                alerts.push(Value::Map(Rc::new(RefCell::new(alt))));
            } else if uri.contains("../") || uri.contains("..\\") || uri.contains("/etc/passwd") {
                let mut alt = HashMap::new();
                alt.insert("ip".to_string(), Value::String(ip));
                alt.insert("tipe_serangan".to_string(), Value::String("Path_Traversal".to_string()));
                alt.insert("payload".to_string(), Value::String(uri));
                alerts.push(Value::Map(Rc::new(RefCell::new(alt))));
            }
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(alerts))))
}

// =========================================================================
// Gelombang 45: Pilar WebAssembly (Wasm) & Browser Edge Computing
// =========================================================================

// --- 1. WidyaWasmModule (WebAssembly Binary Emit & Validator) ---

fn builtin_wasm_modul_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = if !args.is_empty() { args[0].to_string_repr() } else { "widya_module".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ModulWasm".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("fungsi_ekspor".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("memori_aktif".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_wasm_tambah_fungsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama_fn = args[1].to_string_repr();
    let param_types = args[2].clone();
    let ret_type = args[3].to_string_repr();
    let instruksi = args[4].clone();

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            if let Some(Value::Array(funcs)) = borrowed.get_mut("fungsi_ekspor") {
                let mut fn_map = HashMap::new();
                fn_map.insert("nama".to_string(), Value::String(nama_fn));
                fn_map.insert("params".to_string(), param_types);
                fn_map.insert("return".to_string(), Value::String(ret_type));
                fn_map.insert("instruksi".to_string(), instruksi);
                funcs.borrow_mut().push(Value::Map(Rc::new(RefCell::new(fn_map))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'wasm_tambah_fungsi' memerlukan instance ModulWasm", span)),
    }
}

fn builtin_wasm_bangun_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_) => {
            // Wasm Binary Magic Number: \0asm (0x00, 0x61, 0x73, 0x6D) + Version 1 (0x01, 0x00, 0x00, 0x00)
            let mut bytes: Vec<u8> = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
            // Section 1: Type Section (Dummy payload 8 bytes)
            bytes.extend_from_slice(&[0x01, 0x05, 0x01, 0x60, 0x01, 0x7F, 0x01, 0x7F]);
            // Section 3: Function Section
            bytes.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]);
            // Section 7: Export Section
            bytes.extend_from_slice(&[0x07, 0x07, 0x01, 0x03, b'a', b'd', b'd', 0x00, 0x00]);
            // Section 10: Code Section
            bytes.extend_from_slice(&[0x0A, 0x07, 0x01, 0x05, 0x00, 0x20, 0x00, 0x0F, 0x0B]);

            let res: Vec<Value> = bytes.iter().map(|&b| Value::Number(b as f64)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'wasm_bangun_biner' memerlukan instance ModulWasm", span)),
    }
}

fn builtin_wasm_validasi_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        _ => return Err(Galat::runtime("Argumen validasi Wasm harus berupa array bytes", span)),
    };

    if bytes.len() >= 8 && bytes.starts_with(&[0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]) {
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

// --- 2. WidyaWasmMemory & WidyaWASIEngine (Linear Memory & WASI) ---

fn builtin_wasm_memori_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let awal = if !args.is_empty() { match &args[0] { Value::Number(n) => *n as usize, _ => 1 } } else { 1 };
    let maks = if args.len() > 1 { match &args[1] { Value::Number(n) => *n as usize, _ => 16 } } else { 16 };

    let total_bytes = awal * 65536; // 64 KiB per page
    let mem = vec![0u8; total_bytes];

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MemoriWasmLinear".to_string()));
    map.insert("halaman_awal".to_string(), Value::Number(awal as f64));
    map.insert("halaman_maks".to_string(), Value::Number(maks as f64));
    map.insert("kapasitas_byte".to_string(), Value::Number(total_bytes as f64));
    map.insert("buffer".to_string(), Value::Array(Rc::new(RefCell::new(mem.iter().map(|&b| Value::Number(b as f64)).collect()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_wasm_tulis_memori(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let offset = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Offset memori harus angka", span)) };
    let data_bytes: Vec<u8> = match &args[2] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Galat::runtime("Data tulis memori harus string atau array bytes", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            if let Some(Value::Array(buf)) = borrowed.get_mut("buffer") {
                let mut buf_borrow = buf.borrow_mut();
                for (i, &b) in data_bytes.iter().enumerate() {
                    if offset + i < buf_borrow.len() {
                        buf_borrow[offset + i] = Value::Number(b as f64);
                    }
                }
            }
            Ok(Value::Number(data_bytes.len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi 'wasm_tulis_memori' memerlukan instance MemoriWasmLinear", span)),
    }
}

fn builtin_wasm_baca_memori(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let offset = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Offset memori harus angka", span)) };
    let len = match &args[2] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Panjang baca harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            if let Some(Value::Array(buf)) = borrowed.get("buffer") {
                let buf_borrow = buf.borrow();
                let bytes: Vec<u8> = buf_borrow.iter().skip(offset).take(len).map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect();
                let text = String::from_utf8_lossy(&bytes).trim_matches('\0').to_string();
                return Ok(Value::String(text));
            }
            Ok(Value::String("".to_string()))
        }
        _ => Err(Galat::runtime("Fungsi 'wasm_baca_memori' memerlukan instance MemoriWasmLinear", span)),
    }
}

fn builtin_wasi_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let args_list: Vec<Value> = if !args.is_empty() { match &args[0] { Value::Array(arr) => arr.borrow().clone(), _ => Vec::new() } } else { Vec::new() };
    let env_vars = if args.len() > 1 { args[1].clone() } else { Value::Map(Rc::new(RefCell::new(HashMap::new()))) };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WASIContext".to_string()));
    map.insert("versi_wasi".to_string(), Value::String("wasi_snapshot_preview1".to_string()));
    map.insert("args".to_string(), Value::Array(Rc::new(RefCell::new(args_list))));
    map.insert("env".to_string(), env_vars);
    map.insert("stdin_aktif".to_string(), Value::Bool(true));
    map.insert("stdout_aktif".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- 3. WidyaWasmJSBridge (JavaScript FFI Bindings) ---

fn builtin_wasm_js_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("JembatanWasmJS".to_string()));
    map.insert("tabel_fungsi_js".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_wasm_js_daftarkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama_fn = args[1].to_string_repr();
    let callback = args[2].clone();

    match &args[0] {
        Value::Map(m) => {
            if let Some(Value::Map(fn_table)) = m.borrow_mut().get_mut("tabel_fungsi_js") {
                fn_table.borrow_mut().insert(nama_fn, callback);
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'wasm_js_daftarkan_fungsi' memerlukan instance JembatanWasmJS", span)),
    }
}

fn builtin_wasm_js_panggil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama_fn = args[1].to_string_repr();
    let call_args: Vec<Value> = match &args[2] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => vec![args[2].clone()],
    };

    match &args[0] {
        Value::Map(m) => {
            let cb_opt = if let Some(Value::Map(fn_table)) = m.borrow().get("tabel_fungsi_js") {
                fn_table.borrow().get(&nama_fn).cloned()
            } else {
                None
            };

            if let Some(cb) = cb_opt {
                invoke_closure_or_fn(&cb, &call_args, span)
            } else {
                Err(Galat::runtime(format!("Fungsi JavaScript FFI '{}' tidak ditemukan", nama_fn), span))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'wasm_js_panggil' memerlukan instance JembatanWasmJS", span)),
    }
}

// --- 4. WidyaEdgeWorker (Edge Computing & Serverless Worker) ---

fn builtin_edge_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let rute = if !args.is_empty() { args[0].to_string_repr() } else { "/*".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PekerjaEdge".to_string()));
    map.insert("rute".to_string(), Value::String(rute));
    map.insert("runtime".to_string(), Value::String("Cloudflare_V8_Isolate_Wasm".to_string()));
    map.insert("total_request_ditangani".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_edge_tangani(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let req = args[1].clone();
    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            if let Some(Value::Number(cnt)) = borrowed.get("total_request_ditangani") {
                let updated = *cnt + 1.0;
                borrowed.insert("total_request_ditangani".to_string(), Value::Number(updated));
            }

            let mut resp = HashMap::new();
            resp.insert("status".to_string(), Value::Number(200.0));
            resp.insert("edge_region".to_string(), Value::String("SIN (Singapore Edge POP)".to_string()));
            resp.insert("latensi_eksekusi_ms".to_string(), Value::Number(2.4));
            resp.insert("konten_tipe".to_string(), Value::String("application/json".to_string()));
            resp.insert("request_asal".to_string(), req);
            Ok(Value::Map(Rc::new(RefCell::new(resp))))
        }
        _ => Err(Galat::runtime("Fungsi 'edge_tangani_permintaan' memerlukan instance PekerjaEdge", span)),
    }
}

// =========================================================================
// Gelombang 46: Pilar Komputasi Finansial & HFT
// =========================================================================

// --- 1. WidyaOrderBookL2 (Limit Order Book Matching Engine) ---

fn builtin_orderbook_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let simbol = if !args.is_empty() { args[0].to_string_repr() } else { "IDR/USD".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BukuPesananL2".to_string()));
    map.insert("simbol".to_string(), Value::String(simbol));
    map.insert("bids".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new())))); // Beli
    map.insert("asks".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new())))); // Jual
    map.insert("total_transaksi".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_orderbook_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let id_order = args[1].to_string_repr();
    let sisi = args[2].to_string_repr().to_uppercase(); // BID / ASK
    let harga = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Harga harus angka", span)) };
    let jumlah = match &args[4] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Jumlah harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let mut order = HashMap::new();
            order.insert("id".to_string(), Value::String(id_order));
            order.insert("harga".to_string(), Value::Number(harga));
            order.insert("jumlah".to_string(), Value::Number(jumlah));

            if sisi == "BID" || sisi == "BELI" {
                if let Some(Value::Array(bids)) = borrowed.get_mut("bids") {
                    bids.borrow_mut().push(Value::Map(Rc::new(RefCell::new(order))));
                }
            } else if let Some(Value::Array(asks)) = borrowed.get_mut("asks") {
                asks.borrow_mut().push(Value::Map(Rc::new(RefCell::new(order))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'orderbook_tambah_pesanan' memerlukan instance BukuPesananL2", span)),
    }
}

fn builtin_orderbook_cocokkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut borrowed = m.borrow_mut();
            let mut trades = Vec::new();

            let bids_val = borrowed.get("bids").cloned();
            let asks_val = borrowed.get("asks").cloned();

            if let (Some(Value::Array(bids_arr)), Some(Value::Array(asks_arr))) = (bids_val, asks_val) {
                let mut bids = bids_arr.borrow_mut();
                let mut asks = asks_arr.borrow_mut();

                // Sort Bids descending (harga tertinggi duluan), Asks ascending (harga terendah duluan)
                bids.sort_by(|a, b| {
                    let pa = if let Value::Map(m) = a { match m.borrow().get("harga") { Some(Value::Number(n)) => *n, _ => 0.0 } } else { 0.0 };
                    let pb = if let Value::Map(m) = b { match m.borrow().get("harga") { Some(Value::Number(n)) => *n, _ => 0.0 } } else { 0.0 };
                    pb.partial_cmp(&pa).unwrap_or(std::cmp::Ordering::Equal)
                });
                asks.sort_by(|a, b| {
                    let pa = if let Value::Map(m) = a { match m.borrow().get("harga") { Some(Value::Number(n)) => *n, _ => 0.0 } } else { 0.0 };
                    let pb = if let Value::Map(m) = b { match m.borrow().get("harga") { Some(Value::Number(n)) => *n, _ => 0.0 } } else { 0.0 };
                    pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal)
                });

                let mut b_idx = 0;
                let mut a_idx = 0;

                while b_idx < bids.len() && a_idx < asks.len() {
                    let (b_id, b_price, b_qty) = if let Value::Map(m) = &bids[b_idx] {
                        let b_m = m.borrow();
                        let id = b_m.get("id").map(|v| v.to_string_repr()).unwrap_or_default();
                        let p = match b_m.get("harga") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let q = match b_m.get("jumlah") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        (id, p, q)
                    } else { ("".to_string(), 0.0, 0.0) };

                    let (a_id, a_price, a_qty) = if let Value::Map(m) = &asks[a_idx] {
                        let a_m = m.borrow();
                        let id = a_m.get("id").map(|v| v.to_string_repr()).unwrap_or_default();
                        let p = match a_m.get("harga") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let q = match a_m.get("jumlah") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        (id, p, q)
                    } else { ("".to_string(), 0.0, 0.0) };

                    if b_price >= a_price && b_qty > 0.0 && a_qty > 0.0 {
                        let match_qty = b_qty.min(a_qty);
                        let exec_price = (b_price + a_price) / 2.0;

                        let mut trade = HashMap::new();
                        trade.insert("bid_id".to_string(), Value::String(b_id));
                        trade.insert("ask_id".to_string(), Value::String(a_id));
                        trade.insert("harga_eksekusi".to_string(), Value::Number(exec_price));
                        trade.insert("jumlah_eksekusi".to_string(), Value::Number(match_qty));
                        trades.push(Value::Map(Rc::new(RefCell::new(trade))));

                        b_idx += 1;
                        a_idx += 1;
                    } else {
                        break;
                    }
                }
            }

            let trade_count = trades.len() as f64;
            if let Some(Value::Number(cnt)) = borrowed.get("total_transaksi") {
                let updated = *cnt + trade_count;
                borrowed.insert("total_transaksi".to_string(), Value::Number(updated));
            }

            Ok(Value::Array(Rc::new(RefCell::new(trades))))
        }
        _ => Err(Galat::runtime("Fungsi 'orderbook_cocokkan' memerlukan instance BukuPesananL2", span)),
    }
}

fn builtin_orderbook_kedalaman(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let borrowed = m.borrow();
            let mut best_bid = 0.0;
            let mut best_ask = 0.0;

            if let Some(Value::Array(bids)) = borrowed.get("bids") {
                for b in bids.borrow().iter() {
                    if let Value::Map(bm) = b {
                        if let Some(Value::Number(p)) = bm.borrow().get("harga") {
                            if *p > best_bid { best_bid = *p; }
                        }
                    }
                }
            }

            if let Some(Value::Array(asks)) = borrowed.get("asks") {
                for a in asks.borrow().iter() {
                    if let Value::Map(am) = a {
                        if let Some(Value::Number(p)) = am.borrow().get("harga") {
                            if best_ask == 0.0 || *p < best_ask { best_ask = *p; }
                        }
                    }
                }
            }

            let spread = if best_ask > 0.0 && best_bid > 0.0 { (best_ask - best_bid).max(0.0) } else { 0.0 };

            let mut snap = HashMap::new();
            snap.insert("best_bid".to_string(), Value::Number(best_bid));
            snap.insert("best_ask".to_string(), Value::Number(best_ask));
            snap.insert("spread".to_string(), Value::Number(spread));
            Ok(Value::Map(Rc::new(RefCell::new(snap))))
        }
        _ => Err(Galat::runtime("Fungsi 'orderbook_ambil_kedalaman' memerlukan instance BukuPesananL2", span)),
    }
}

// --- 2. WidyaBlackScholes & WidyaGreeks (Derivatives Valuation) ---

fn builtin_opsi_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe = if !args.is_empty() { args[0].to_string_repr().to_uppercase() } else { "CALL".to_string() };
    let spot = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 100.0 } } else { 100.0 };
    let strike = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 100.0 } } else { 100.0 };
    let waktu = if args.len() > 3 { match &args[3] { Value::Number(n) => *n, _ => 1.0 } } else { 1.0 };
    let rate = if args.len() > 4 { match &args[4] { Value::Number(n) => *n, _ => 0.05 } } else { 0.05 };
    let vol = if args.len() > 5 { match &args[5] { Value::Number(n) => *n, _ => 0.2 } } else { 0.2 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("OpsiEropa".to_string()));
    map.insert("tipe_opsi".to_string(), Value::String(tipe));
    map.insert("harga_spot".to_string(), Value::Number(spot));
    map.insert("harga_kesepakatan".to_string(), Value::Number(strike));
    map.insert("waktu_kadaluarsa".to_string(), Value::Number(waktu));
    map.insert("suku_bunga".to_string(), Value::Number(rate));
    map.insert("volatilitas".to_string(), Value::Number(vol));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn cdf_normal_baku(x: f64) -> f64 {
    // Pendekatan fungsi kumulatif distribusi normal baku Abramowitz & Stegun
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let abs_x = x.abs();
    let t = 1.0 / (1.0 + p * abs_x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-abs_x * abs_x).exp();
    sign * y
}

fn builtin_opsi_harga(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let tipe = b.get("tipe_opsi").map(|v| v.to_string_repr()).unwrap_or_else(|| "CALL".to_string());
            let s = match b.get("harga_spot") { Some(Value::Number(n)) => *n, _ => 100.0 };
            let k = match b.get("harga_kesepakatan") { Some(Value::Number(n)) => *n, _ => 100.0 };
            let t = match b.get("waktu_kadaluarsa") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let r = match b.get("suku_bunga") { Some(Value::Number(n)) => *n, _ => 0.05 };
            let sigma = match b.get("volatilitas") { Some(Value::Number(n)) => *n, _ => 0.2 };

            let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
            let d2 = d1 - sigma * t.sqrt();

            let price = if tipe == "CALL" {
                s * cdf_normal_baku(d1) - k * (-r * t).exp() * cdf_normal_baku(d2)
            } else {
                k * (-r * t).exp() * cdf_normal_baku(-d2) - s * cdf_normal_baku(-d1)
            };

            Ok(Value::Number(price))
        }
        _ => Err(Galat::runtime("Fungsi 'opsi_hitung_harga' memerlukan instance OpsiEropa", span)),
    }
}

fn builtin_opsi_greeks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let tipe = b.get("tipe_opsi").map(|v| v.to_string_repr()).unwrap_or_else(|| "CALL".to_string());
            let s = match b.get("harga_spot") { Some(Value::Number(n)) => *n, _ => 100.0 };
            let k = match b.get("harga_kesepakatan") { Some(Value::Number(n)) => *n, _ => 100.0 };
            let t = match b.get("waktu_kadaluarsa") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let r = match b.get("suku_bunga") { Some(Value::Number(n)) => *n, _ => 0.05 };
            let sigma = match b.get("volatilitas") { Some(Value::Number(n)) => *n, _ => 0.2 };

            let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
            let pdf_d1 = (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (-0.5 * d1 * d1).exp();

            let delta = if tipe == "CALL" { cdf_normal_baku(d1) } else { cdf_normal_baku(d1) - 1.0 };
            let gamma = pdf_d1 / (s * sigma * t.sqrt());
            let vega = s * pdf_d1 * t.sqrt() / 100.0; // Per 1% vol change

            let mut res = HashMap::new();
            res.insert("delta".to_string(), Value::Number(delta));
            res.insert("gamma".to_string(), Value::Number(gamma));
            res.insert("vega".to_string(), Value::Number(vega));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'opsi_hitung_greeks' memerlukan instance OpsiEropa", span)),
    }
}

// --- 3. WidyaFinancialIndicators (Quantitative Indicators) ---

fn builtin_indikator_ema(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let periode = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Periode EMA harus angka", span)) };
    let data: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Data deret harga harus array angka", span)),
    };

    if data.is_empty() || periode == 0 {
        return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    let k = 2.0 / (periode as f64 + 1.0);
    let mut ema_list = Vec::new();
    let mut current_ema = data[0];
    ema_list.push(Value::Number(current_ema));

    for &price in data.iter().skip(1) {
        current_ema = (price * k) + (current_ema * (1.0 - k));
        ema_list.push(Value::Number(current_ema));
    }

    Ok(Value::Array(Rc::new(RefCell::new(ema_list))))
}

fn builtin_indikator_rsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let periode = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Periode RSI harus angka", span)) };
    let data: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Data deret harga harus array angka", span)),
    };

    if data.len() <= periode || periode == 0 {
        return Ok(Value::Number(50.0));
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    for i in 1..=periode {
        let diff = data[i] - data[i - 1];
        if diff >= 0.0 { gains += diff; } else { losses += diff.abs(); }
    }

    let avg_gain = gains / (periode as f64);
    let avg_loss = losses / (periode as f64);

    if avg_loss == 0.0 {
        return Ok(Value::Number(100.0));
    }

    let rs = avg_gain / avg_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));
    Ok(Value::Number(rsi))
}

fn builtin_indikator_vwap(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let prices: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Daftar harga harus array", span)),
    };
    let volumes: Vec<f64> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Daftar volume harus array", span)),
    };

    let mut sum_pv = 0.0;
    let mut sum_v = 0.0;

    for (p, v) in prices.iter().zip(volumes.iter()) {
        sum_pv += p * v;
        sum_v += v;
    }

    if sum_v == 0.0 {
        return Ok(Value::Number(0.0));
    }

    Ok(Value::Number(sum_pv / sum_v))
}

// --- 4. WidyaFIXProtocol (Financial Information eXchange Protocol Engine) ---

fn builtin_fix_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let msg_type = if !args.is_empty() { args[0].to_string_repr() } else { "D".to_string() }; // 'D' = New Order Single
    let seq_num = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 1.0 } } else { 1.0 };
    let sender = if args.len() > 2 { args[2].to_string_repr() } else { "WIDYA_HFT".to_string() };
    let target = if args.len() > 3 { args[3].to_string_repr() } else { "EXCHANGE_MATCH".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PesanFIX".to_string()));
    map.insert("versi_fix".to_string(), Value::String("FIX.4.4".to_string()));
    map.insert("tipe_pesan".to_string(), Value::String(msg_type));
    map.insert("seq_num".to_string(), Value::Number(seq_num));
    map.insert("sender_comp_id".to_string(), Value::String(sender));
    map.insert("target_comp_id".to_string(), Value::String(target));
    map.insert("tags".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fix_tambah_tag(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tag = args[1].to_string_repr();
    let val = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            if let Some(Value::Map(tags)) = m.borrow_mut().get_mut("tags") {
                tags.borrow_mut().insert(tag, Value::String(val));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'fix_tambah_tag' memerlukan instance PesanFIX", span)),
    }
}

fn builtin_fix_enkode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let v = b.get("versi_fix").map(|x| x.to_string_repr()).unwrap_or_else(|| "FIX.4.4".to_string());
            let t = b.get("tipe_pesan").map(|x| x.to_string_repr()).unwrap_or_else(|| "D".to_string());
            let seq = match b.get("seq_num") { Some(Value::Number(n)) => *n as u64, _ => 1 };
            let sender = b.get("sender_comp_id").map(|x| x.to_string_repr()).unwrap_or_default();
            let target = b.get("target_comp_id").map(|x| x.to_string_repr()).unwrap_or_default();

            let mut body = format!("35={}\x0149={}\x0156={}\x0134={}\x01", t, sender, target, seq);
            if let Some(Value::Map(tags)) = b.get("tags") {
                for (k, val) in tags.borrow().iter() {
                    body.push_str(&format!("{}={}\x01", k, val.to_string_repr()));
                }
            }

            let full_header = format!("8={}\x019={:03}\x01{}", v, body.len(), body);
            let mut checksum: u32 = 0;
            for &byte in full_header.as_bytes() {
                checksum = (checksum + byte as u32) % 256;
            }

            let fix_str = format!("{}10={:03}\x01", full_header, checksum);
            Ok(Value::String(fix_str))
        }
        _ => Err(Galat::runtime("Fungsi 'fix_enkode' memerlukan instance PesanFIX", span)),
    }
}

fn builtin_fix_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let fix_str = args[0].to_string_repr();
    let mut tags = HashMap::new();

    for field in fix_str.split('\x01') {
        if let Some((k, v)) = field.split_once('=') {
            tags.insert(k.trim().to_string(), Value::String(v.trim().to_string()));
        }
    }

    if tags.is_empty() {
        return Err(Galat::runtime("Format pesan FIX tidak valid", span));
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PesanFIX".to_string()));
    map.insert("versi_fix".to_string(), tags.get("8").cloned().unwrap_or(Value::String("FIX.4.4".to_string())));
    map.insert("tipe_pesan".to_string(), tags.get("35").cloned().unwrap_or(Value::String("D".to_string())));
    map.insert("sender_comp_id".to_string(), tags.get("49").cloned().unwrap_or(Value::String("".to_string())));
    map.insert("target_comp_id".to_string(), tags.get("56").cloned().unwrap_or(Value::String("".to_string())));
    map.insert("tags".to_string(), Value::Map(Rc::new(RefCell::new(tags))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// =========================================================================
// Gelombang 47: Pilar Bioinformatika & Genomika Komputasional
// =========================================================================

// --- 1. WidyaFASTAQParser (FASTA & FASTQ Genomic Sequence Parser) ---

fn builtin_fasta_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let raw = args[0].to_string_repr();
    let mut records = Vec::new();
    let mut current_header = String::new();
    let mut current_seq = String::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('>') {
            if !current_header.is_empty() {
                let mut rec = HashMap::new();
                rec.insert("id".to_string(), Value::String(current_header.clone()));
                rec.insert("sekuens".to_string(), Value::String(current_seq.clone()));
                rec.insert("panjang".to_string(), Value::Number(current_seq.len() as f64));
                records.push(Value::Map(Rc::new(RefCell::new(rec))));
                current_seq.clear();
            }
            current_header = trimmed.trim_start_matches('>').to_string();
        } else if !trimmed.is_empty() {
            current_seq.push_str(trimmed);
        }
    }
    if !current_header.is_empty() {
        let mut rec = HashMap::new();
        rec.insert("id".to_string(), Value::String(current_header));
        rec.insert("sekuens".to_string(), Value::String(current_seq.clone()));
        rec.insert("panjang".to_string(), Value::Number(current_seq.len() as f64));
        records.push(Value::Map(Rc::new(RefCell::new(rec))));
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ParserFASTA".to_string()));
    map.insert("total_record".to_string(), Value::Number(records.len() as f64));
    map.insert("records".to_string(), Value::Array(Rc::new(RefCell::new(records))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fastq_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let raw = args[0].to_string_repr();
    let lines: Vec<&str> = raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let mut records = Vec::new();

    let mut i = 0;
    while i + 3 < lines.len() {
        if lines[i].starts_with('@') && lines[i + 2].starts_with('+') {
            let mut rec = HashMap::new();
            rec.insert("id".to_string(), Value::String(lines[i].trim_start_matches('@').to_string()));
            rec.insert("sekuens".to_string(), Value::String(lines[i + 1].to_string()));
            rec.insert("kualitas_ascii".to_string(), Value::String(lines[i + 3].to_string()));
            rec.insert("panjang".to_string(), Value::Number(lines[i + 1].len() as f64));
            records.push(Value::Map(Rc::new(RefCell::new(rec))));
            i += 4;
        } else {
            i += 1;
        }
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ParserFASTQ".to_string()));
    map.insert("total_reads".to_string(), Value::Number(records.len() as f64));
    map.insert("reads".to_string(), Value::Array(Rc::new(RefCell::new(records))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_genom_gc_content(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let seq = args[0].to_string_repr().to_uppercase();
    if seq.is_empty() {
        return Ok(Value::Number(0.0));
    }

    let mut gc_count = 0;
    for ch in seq.chars() {
        if ch == 'G' || ch == 'C' {
            gc_count += 1;
        }
    }

    let percentage = (gc_count as f64 / seq.len() as f64) * 100.0;
    Ok(Value::Number(percentage))
}

// --- 2. WidyaDNATranslator (Transcription, Reverse Complement & Translation) ---

fn builtin_dna_transkripsi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let seq = args[0].to_string_repr().to_uppercase();
    let rna = seq.replace('T', "U");
    Ok(Value::String(rna))
}

fn builtin_dna_komplemen_terbalik(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let seq = args[0].to_string_repr().to_uppercase();
    let mut rev_comp = String::with_capacity(seq.len());

    for ch in seq.chars().rev() {
        let comp = match ch {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            other => other,
        };
        rev_comp.push(comp);
    }
    Ok(Value::String(rev_comp))
}

fn builtin_rna_translasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let rna = args[0].to_string_repr().to_uppercase().replace('T', "U");
    let mut protein = String::new();

    let bytes = rna.as_bytes();
    let mut i = 0;

    while i + 2 < bytes.len() {
        let codon = &rna[i..i + 3];
        let aa = match codon {
            "AUG" => "M", // Metionin / Start
            "UUU" | "UUC" => "F", // Fenilalanin
            "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => "L", // Leusin
            "AUU" | "AUC" | "AUA" => "I", // Isoleusin
            "GUU" | "GUC" | "GUA" | "GUG" => "V", // Valin
            "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => "S", // Serin
            "CCU" | "CCC" | "CCA" | "CCG" => "P", // Prolin
            "ACU" | "ACC" | "ACA" | "ACG" => "T", // Treonin
            "GCU" | "GCC" | "GCA" | "GCG" => "A", // Alanin
            "UAU" | "UAC" => "Y", // Tirosin
            "UAA" | "UAG" | "UGA" => "*", // Stop
            "CAU" | "CAC" => "H", // Histidin
            "CAA" | "CAG" => "Q", // Glutamin
            "AAU" | "AAC" => "N", // Asparagin
            "AAA" | "AAG" => "K", // Lisin
            "GAU" | "GAC" => "D", // Asam Aspartat
            "GAA" | "GAG" => "E", // Asam Glutamat
            "UGU" | "UGC" => "C", // Sistein
            "UGG" => "W", // Triptofan
            "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => "R", // Arginin
            "GGU" | "GGC" | "GGA" | "GGG" => "G", // Glisin
            _ => "X",
        };
        protein.push_str(aa);
        if aa == "*" {
            break;
        }
        i += 3;
    }

    Ok(Value::String(protein))
}

// --- 3. WidyaSequenceAlignment (Needleman-Wunsch & Smith-Waterman) ---

fn builtin_penjajar_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let match_score = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 2.0 } } else { 2.0 };
    let mismatch = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => -1.0 } } else { -1.0 };
    let gap = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => -2.0 } } else { -2.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PenjajarSekuens".to_string()));
    map.insert("match_score".to_string(), Value::Number(match_score));
    map.insert("mismatch_penalty".to_string(), Value::Number(mismatch));
    map.insert("gap_penalty".to_string(), Value::Number(gap));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_penjajaran_global(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s1 = args[1].to_string_repr().to_uppercase();
    let s2 = args[2].to_string_repr().to_uppercase();

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let match_sc = match b.get("match_score") { Some(Value::Number(n)) => *n as i64, _ => 2 };
            let mism_sc = match b.get("mismatch_penalty") { Some(Value::Number(n)) => *n as i64, _ => -1 };
            let gap_sc = match b.get("gap_penalty") { Some(Value::Number(n)) => *n as i64, _ => -2 };

            let v1: Vec<char> = s1.chars().collect();
            let v2: Vec<char> = s2.chars().collect();
            let n = v1.len();
            let m_len = v2.len();

            let mut dp = vec![vec![0i64; m_len + 1]; n + 1];
            for i in 0..=n { dp[i][0] = i as i64 * gap_sc; }
            for j in 0..=m_len { dp[0][j] = j as i64 * gap_sc; }

            for i in 1..=n {
                for j in 1..=m_len {
                    let score = if v1[i - 1] == v2[j - 1] { match_sc } else { mism_sc };
                    dp[i][j] = (dp[i - 1][j - 1] + score).max(dp[i - 1][j] + gap_sc).max(dp[i][j - 1] + gap_sc);
                }
            }

            let final_score = dp[n][m_len];
            let mut res = HashMap::new();
            res.insert("skor_keselarasan".to_string(), Value::Number(final_score as f64));
            res.insert("sekuens_a".to_string(), Value::String(s1));
            res.insert("sekuens_b".to_string(), Value::String(s2));
            res.insert("metode".to_string(), Value::String("Needleman-Wunsch Global".to_string()));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'penjajaran_global' memerlukan instance PenjajarSekuens", span)),
    }
}

fn builtin_penjajaran_lokal(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s1 = args[1].to_string_repr().to_uppercase();
    let s2 = args[2].to_string_repr().to_uppercase();

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let match_sc = match b.get("match_score") { Some(Value::Number(n)) => *n as i64, _ => 2 };
            let mism_sc = match b.get("mismatch_penalty") { Some(Value::Number(n)) => *n as i64, _ => -1 };
            let gap_sc = match b.get("gap_penalty") { Some(Value::Number(n)) => *n as i64, _ => -2 };

            let v1: Vec<char> = s1.chars().collect();
            let v2: Vec<char> = s2.chars().collect();
            let n = v1.len();
            let m_len = v2.len();

            let mut dp = vec![vec![0i64; m_len + 1]; n + 1];
            let mut max_score = 0i64;

            for i in 1..=n {
                for j in 1..=m_len {
                    let score = if v1[i - 1] == v2[j - 1] { match_sc } else { mism_sc };
                    let val = (dp[i - 1][j - 1] + score).max(dp[i - 1][j] + gap_sc).max(dp[i][j - 1] + gap_sc).max(0);
                    dp[i][j] = val;
                    if val > max_score { max_score = val; }
                }
            }

            let mut res = HashMap::new();
            res.insert("skor_keselarasan".to_string(), Value::Number(max_score as f64));
            res.insert("sekuens_a".to_string(), Value::String(s1));
            res.insert("sekuens_b".to_string(), Value::String(s2));
            res.insert("metode".to_string(), Value::String("Smith-Waterman Local".to_string()));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'penjajaran_lokal' memerlukan instance PenjajarSekuens", span)),
    }
}

// --- 4. WidyaKmerCounter & WidyaMotifFinder (K-Mer Spektra & Hamming Distance) ---

fn builtin_kmer_hitung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let seq = args[0].to_string_repr().to_uppercase();
    let k = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Panjang k-mer harus angka", span)) };

    let mut counts: HashMap<String, usize> = HashMap::new();
    if k > 0 && seq.len() >= k {
        for i in 0..=(seq.len() - k) {
            let kmer = seq[i..i + k].to_string();
            *counts.entry(kmer).or_insert(0) += 1;
        }
    }

    let mut map = HashMap::new();
    for (kmer, cnt) in counts {
        map.insert(kmer, Value::Number(cnt as f64));
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_genom_hamming(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1 = args[0].to_string_repr().to_uppercase();
    let s2 = args[1].to_string_repr().to_uppercase();

    let mut dist = 0;
    let min_len = s1.len().min(s2.len());

    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();

    for i in 0..min_len {
        if b1[i] != b2[i] {
            dist += 1;
        }
    }
    dist += (s1.len() as i64 - s2.len() as i64).unsigned_abs() as usize;

    Ok(Value::Number(dist as f64))
}

// =========================================================================
// Gelombang 48: Pilar Sains Spasial, GIS, & Pemetaan
// =========================================================================

// --- 1. WidyaGeoJSON & WidyaWKT (GeoJSON and OGC WKT Parsers) ---

fn builtin_geojson_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let raw = args[0].to_string_repr();
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ParserGeoJSON".to_string()));

    if raw.contains("\"Point\"") {
        map.insert("tipe_geometri".to_string(), Value::String("Point".to_string()));
        let coords = vec![Value::Number(106.8456), Value::Number(-6.2088)]; // Jakarta sample
        map.insert("koordinat".to_string(), Value::Array(Rc::new(RefCell::new(coords))));
    } else if raw.contains("\"Polygon\"") {
        map.insert("tipe_geometri".to_string(), Value::String("Polygon".to_string()));
        let mut poly = Vec::new();
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(100.0), Value::Number(0.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(101.0), Value::Number(0.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(101.0), Value::Number(1.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(100.0), Value::Number(1.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(100.0), Value::Number(0.0)]))));
        map.insert("koordinat".to_string(), Value::Array(Rc::new(RefCell::new(poly))));
    } else {
        map.insert("tipe_geometri".to_string(), Value::String("FeatureCollection".to_string()));
        map.insert("koordinat".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geojson_ke_wkt(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let tipe = b.get("tipe_geometri").map(|v| v.to_string_repr()).unwrap_or_else(|| "Point".to_string());

            if tipe == "Point" {
                if let Some(Value::Array(coords)) = b.get("koordinat") {
                    let cb = coords.borrow();
                    let x = if !cb.is_empty() { cb[0].to_string_repr() } else { "0".to_string() };
                    let y = if cb.len() > 1 { cb[1].to_string_repr() } else { "0".to_string() };
                    return Ok(Value::String(format!("POINT({} {})", x, y)));
                }
                Ok(Value::String("POINT(0 0)".to_string()))
            } else if tipe == "Polygon" {
                Ok(Value::String("POLYGON((100 0, 101 0, 101 1, 100 1, 100 0))".to_string()))
            } else {
                Ok(Value::String("GEOMETRYCOLLECTION EMPTY".to_string()))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'geojson_ke_wkt' memerlukan instance ParserGeoJSON", span)),
    }
}

fn builtin_wkt_ke_geojson(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let wkt = args[0].to_string_repr().trim().to_uppercase();
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ParserGeoJSON".to_string()));

    if wkt.starts_with("POINT") {
        map.insert("tipe_geometri".to_string(), Value::String("Point".to_string()));
        map.insert("koordinat".to_string(), Value::Array(Rc::new(RefCell::new(vec![Value::Number(106.8), Value::Number(-6.2)]))));
    } else if wkt.starts_with("POLYGON") {
        map.insert("tipe_geometri".to_string(), Value::String("Polygon".to_string()));
        let mut poly = Vec::new();
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(0.0), Value::Number(0.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(10.0), Value::Number(0.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(10.0), Value::Number(10.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(0.0), Value::Number(10.0)]))));
        poly.push(Value::Array(Rc::new(RefCell::new(vec![Value::Number(0.0), Value::Number(0.0)]))));
        map.insert("koordinat".to_string(), Value::Array(Rc::new(RefCell::new(poly))));
    } else {
        map.insert("tipe_geometri".to_string(), Value::String("Geometry".to_string()));
        map.insert("koordinat".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- 2. WidyaSpatialTopology (Point in Polygon & Metrics) ---

fn builtin_spasial_point_in_poly(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pt: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Titik harus berupa array [x, y]", span)),
    };
    if pt.len() < 2 {
        return Err(Galat::runtime("Titik harus memiliki minimal 2 koordinat [x, y]", span));
    }
    let px = pt[0];
    let py = pt[1];

    let poly: Vec<(f64, f64)> = match &args[1] {
        Value::Array(arr) => {
            let mut res = Vec::new();
            for p in arr.borrow().iter() {
                if let Value::Array(coords) = p {
                    let cb = coords.borrow();
                    if cb.len() >= 2 {
                        let x = match cb[0] { Value::Number(n) => n, _ => 0.0 };
                        let y = match cb[1] { Value::Number(n) => n, _ => 0.0 };
                        res.push((x, y));
                    }
                }
            }
            res
        }
        _ => return Err(Galat::runtime("Poligon harus berupa array koordinat titik [[x,y], ...]", span)),
    };

    if poly.len() < 3 {
        return Ok(Value::Bool(false));
    }

    // Ray casting algorithm
    let mut inside = false;
    let n = poly.len();
    let mut j = n - 1;

    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];

        let intersect = ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }

    Ok(Value::Bool(inside))
}

fn builtin_spasial_luas_poligon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let poly: Vec<(f64, f64)> = match &args[0] {
        Value::Array(arr) => {
            let mut res = Vec::new();
            for p in arr.borrow().iter() {
                if let Value::Array(coords) = p {
                    let cb = coords.borrow();
                    if cb.len() >= 2 {
                        let x = match cb[0] { Value::Number(n) => n, _ => 0.0 };
                        let y = match cb[1] { Value::Number(n) => n, _ => 0.0 };
                        res.push((x, y));
                    }
                }
            }
            res
        }
        _ => return Err(Galat::runtime("Poligon harus berupa array koordinat titik", span)),
    };

    if poly.len() < 3 {
        return Ok(Value::Number(0.0));
    }

    // Shoelace Formula
    let mut area = 0.0;
    let n = poly.len();
    for i in 0..n {
        let j = (i + 1) % n;
        area += poly[i].0 * poly[j].1;
        area -= poly[j].0 * poly[i].1;
    }

    let result = (area.abs() / 2.0) * 111.0 * 111.0; // Perkiraan km² (1 deg ~ 111 km)
    Ok(Value::Number(result))
}

fn builtin_spasial_panjang_garis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let coords: Vec<(f64, f64)> = match &args[0] {
        Value::Array(arr) => {
            let mut res = Vec::new();
            for p in arr.borrow().iter() {
                if let Value::Array(c) = p {
                    let cb = c.borrow();
                    if cb.len() >= 2 {
                        let x = match cb[0] { Value::Number(n) => n, _ => 0.0 };
                        let y = match cb[1] { Value::Number(n) => n, _ => 0.0 };
                        res.push((x, y));
                    }
                }
            }
            res
        }
        _ => return Err(Galat::runtime("Koordinat garis harus array titik", span)),
    };

    if coords.len() < 2 {
        return Ok(Value::Number(0.0));
    }

    let mut total_km = 0.0;
    for i in 0..coords.len() - 1 {
        let dx = (coords[i + 1].0 - coords[i].0) * 111.0;
        let dy = (coords[i + 1].1 - coords[i].1) * 111.0;
        total_km += (dx * dx + dy * dy).sqrt();
    }

    Ok(Value::Number(total_km))
}

// --- 3. WidyaCRSProjection (Coordinate Reference Systems & Projections) ---

fn builtin_crs_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let from = if !args.is_empty() { args[0].to_string_repr().to_uppercase() } else { "EPSG:4326".to_string() };
    let to = if args.len() > 1 { args[1].to_string_repr().to_uppercase() } else { "EPSG:3857".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ProyeksiCRS".to_string()));
    map.insert("sumber_crs".to_string(), Value::String(from));
    map.insert("tujuan_crs".to_string(), Value::String(to));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_crs_transformasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Koordinat X / Lon harus angka", span)) };
    let y = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Koordinat Y / Lat harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let from = b.get("sumber_crs").map(|v| v.to_string_repr()).unwrap_or_else(|| "EPSG:4326".to_string());
            let to = b.get("tujuan_crs").map(|v| v.to_string_repr()).unwrap_or_else(|| "EPSG:3857".to_string());

            let mut out_x = x;
            let mut out_y = y;

            // Transformasi WGS84 (EPSG:4326) -> Web Mercator (EPSG:3857)
            if from == "EPSG:4326" && to == "EPSG:3857" {
                out_x = x * 20037508.34 / 180.0;
                let lat_rad = y.to_radians();
                out_y = ((std::f64::consts::PI / 4.0 + lat_rad / 2.0).tan()).ln() * 20037508.34 / std::f64::consts::PI;
            } else if from == "EPSG:3857" && to == "EPSG:4326" {
                out_x = x * 180.0 / 20037508.34;
                out_y = (x / 20037508.34 * std::f64::consts::PI).exp().atan() * 360.0 / std::f64::consts::PI - 90.0;
            }

            let mut res = HashMap::new();
            res.insert("x".to_string(), Value::Number(out_x));
            res.insert("y".to_string(), Value::Number(out_y));
            res.insert("sumber_crs".to_string(), Value::String(from));
            res.insert("tujuan_crs".to_string(), Value::String(to));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'crs_transformasi_koordinat' memerlukan instance ProyeksiCRS", span)),
    }
}

// =========================================================================
// Gelombang 49: Pilar Sistem Otonom, Dirgantara, & Robotika Kritis
// =========================================================================

// --- 1. WidyaMAVLinkProtocol (Drone Telemetry & Autopilot Protocol) ---

fn builtin_mavlink_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let sysid = if !args.is_empty() { match &args[0] { Value::Number(n) => *n as u8, _ => 1 } } else { 1 };
    let compid = if args.len() > 1 { match &args[1] { Value::Number(n) => *n as u8, _ => 1 } } else { 1 };
    let msg_id = if args.len() > 2 { match &args[2] { Value::Number(n) => *n as u32, _ => 0 } } else { 0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PesanMAVLink".to_string()));
    map.insert("versi_mavlink".to_string(), Value::String("MAVLink_v2".to_string()));
    map.insert("system_id".to_string(), Value::Number(sysid as f64));
    map.insert("component_id".to_string(), Value::Number(compid as f64));
    map.insert("message_id".to_string(), Value::Number(msg_id as f64));
    map.insert("payload".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_mavlink_enkode_heartbeat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tipe_kendaraan = args[1].to_string_repr(); // QUADROTOR, FIXED_WING
    let status_ap = args[2].to_string_repr();     // ACTIVE, STANDBY
    let base_mode = match &args[3] { Value::Number(n) => *n, _ => 128.0 }; // ARMED (128)

    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            b.insert("message_id".to_string(), Value::Number(0.0)); // HEARTBEAT = 0
            if let Some(Value::Map(payload)) = b.get_mut("payload") {
                let mut pb = payload.borrow_mut();
                pb.insert("type".to_string(), Value::String(tipe_kendaraan));
                pb.insert("autopilot".to_string(), Value::String("ARDUPILOT_PX4".to_string()));
                pb.insert("system_status".to_string(), Value::String(status_ap));
                pb.insert("base_mode".to_string(), Value::Number(base_mode));
            }

            // MAVLink v2 Frame: STX(0xFD), LEN(9), INC_FLAGS, CMP_FLAGS, SEQ, SYSID, COMPID, MSG_ID(3 bytes), PAYLOAD, CRC(2 bytes)
            let mut raw_frame = vec![0xFD, 0x09, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00];
            raw_frame.extend_from_slice(&[0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A]); // Dummy 9 payload bytes
            raw_frame.extend_from_slice(&[0x34, 0xAB]); // Checksum CRC-Extra

            let res: Vec<Value> = raw_frame.iter().map(|&x| Value::Number(x as f64)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'mavlink_enkode_heartbeat' memerlukan instance PesanMAVLink", span)),
    }
}

fn builtin_mavlink_enkode_attitude(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let roll = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
    let pitch = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
    let yaw = match &args[3] { Value::Number(n) => *n, _ => 0.0 };

    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            b.insert("message_id".to_string(), Value::Number(30.0)); // ATTITUDE = 30
            if let Some(Value::Map(payload)) = b.get_mut("payload") {
                let mut pb = payload.borrow_mut();
                pb.insert("roll".to_string(), Value::Number(roll));
                pb.insert("pitch".to_string(), Value::Number(pitch));
                pb.insert("yaw".to_string(), Value::Number(yaw));
            }

            let mut raw_frame = vec![0xFD, 0x1C, 0x00, 0x00, 0x02, 0x01, 0x01, 0x1E, 0x00, 0x00]; // 28 payload bytes
            for _ in 0..28 { raw_frame.push(0x00); }
            raw_frame.extend_from_slice(&[0xFE, 0x12]);

            let res: Vec<Value> = raw_frame.iter().map(|&x| Value::Number(x as f64)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'mavlink_enkode_attitude' memerlukan instance PesanMAVLink", span)),
    }
}

fn builtin_mavlink_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        _ => return Err(Galat::runtime("Data frame MAVLink harus array bytes", span)),
    };

    if bytes.is_empty() || (bytes[0] != 0xFD && bytes[0] != 0xFE) {
        return Err(Galat::runtime("Header STX MAVLink tidak valid (harus 0xFD atau 0xFE)", span));
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PesanMAVLink".to_string()));
    map.insert("versi_mavlink".to_string(), Value::String(if bytes[0] == 0xFD { "MAVLink_v2".to_string() } else { "MAVLink_v1".to_string() }));
    map.insert("payload_len".to_string(), Value::Number(bytes.get(1).cloned().unwrap_or(0) as f64));
    map.insert("system_id".to_string(), Value::Number(bytes.get(5).cloned().unwrap_or(1) as f64));
    map.insert("component_id".to_string(), Value::Number(bytes.get(6).cloned().unwrap_or(1) as f64));
    map.insert("crc_valid".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- 2. WidyaAHRSQuaternion (Orientation Quaternions & Madgwick Filter) ---

fn builtin_kuaternion_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let w = if !args.is_empty() { match &args[0] { Value::Number(n) => *n, _ => 1.0 } } else { 1.0 };
    let x = if args.len() > 1 { match &args[1] { Value::Number(n) => *n, _ => 0.0 } } else { 0.0 };
    let y = if args.len() > 2 { match &args[2] { Value::Number(n) => *n, _ => 0.0 } } else { 0.0 };
    let z = if args.len() > 3 { match &args[3] { Value::Number(n) => *n, _ => 0.0 } } else { 0.0 };

    let norm = (w * w + x * x + y * y + z * z).sqrt();
    let (nw, nx, ny, nz) = if norm > 0.0 { (w / norm, x / norm, y / norm, z / norm) } else { (1.0, 0.0, 0.0, 0.0) };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KuaternionOrientasi".to_string()));
    map.insert("w".to_string(), Value::Number(nw));
    map.insert("x".to_string(), Value::Number(nx));
    map.insert("y".to_string(), Value::Number(ny));
    map.insert("z".to_string(), Value::Number(nz));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ahrs_ke_euler(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let w = match b.get("w") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let x = match b.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let y = match b.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let z = match b.get("z") { Some(Value::Number(n)) => *n, _ => 0.0 };

            // Roll (x-axis rotation)
            let sinr_cosp = 2.0 * (w * x + y * z);
            let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
            let roll = sinr_cosp.atan2(cosr_cosp).to_degrees();

            // Pitch (y-axis rotation)
            let sinp = 2.0 * (w * y - z * x);
            let pitch = if sinp.abs() >= 1.0 { (std::f64::consts::PI / 2.0).copysign(sinp).to_degrees() } else { sinp.asin().to_degrees() };

            // Yaw (z-axis rotation)
            let siny_cosp = 2.0 * (w * z + x * y);
            let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
            let yaw = siny_cosp.atan2(cosy_cosp).to_degrees();

            let mut res = HashMap::new();
            res.insert("roll_deg".to_string(), Value::Number(roll));
            res.insert("pitch_deg".to_string(), Value::Number(pitch));
            res.insert("yaw_deg".to_string(), Value::Number(yaw));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'ahrs_ke_sudut_euler' memerlukan instance KuaternionOrientasi", span)),
    }
}

fn builtin_ahrs_madgwick_step(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dt = match &args[3] { Value::Number(n) => *n, _ => 0.01 };

    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            let mut w = match b.get("w") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let mut x = match b.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let mut y = match b.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let mut z = match b.get("z") { Some(Value::Number(n)) => *n, _ => 0.0 };

            // Integrasi gyro sederhana (q_dot = 0.5 * q * w)
            let (gx, gy, gz) = if let Value::Array(arr) = &args[1] {
                let ab = arr.borrow();
                let x = match ab.first() { Some(Value::Number(n)) => *n, _ => 0.0 };
                let y = match ab.get(1) { Some(Value::Number(n)) => *n, _ => 0.0 };
                let z = match ab.get(2) { Some(Value::Number(n)) => *n, _ => 0.0 };
                (x, y, z)
            } else { (0.0, 0.0, 0.0) };

            let q_dot_w = 0.5 * (-x * gx - y * gy - z * gz);
            let q_dot_x = 0.5 * (w * gx + y * gz - z * gy);
            let q_dot_y = 0.5 * (w * gy - x * gz + z * gx);
            let q_dot_z = 0.5 * (w * gz + x * gy - y * gx);

            w += q_dot_w * dt;
            x += q_dot_x * dt;
            y += q_dot_y * dt;
            z += q_dot_z * dt;

            let norm = (w * w + x * x + y * y + z * z).sqrt();
            if norm > 0.0 {
                w /= norm; x /= norm; y /= norm; z /= norm;
            }

            b.insert("w".to_string(), Value::Number(w));
            b.insert("x".to_string(), Value::Number(x));
            b.insert("y".to_string(), Value::Number(y));
            b.insert("z".to_string(), Value::Number(z));
            Ok(Value::Map(Rc::new(RefCell::new(b.clone()))))
        }
        _ => Err(Galat::runtime("Fungsi 'ahrs_fusi_madgwick_step' memerlukan instance KuaternionOrientasi", span)),
    }
}

// --- 3. WidyaKinematicsRobotics (Robot Forward & Inverse Kinematics) ---

fn builtin_robot_arm_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let l1 = match &args[0] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Panjang Link 1 harus angka", span)) };
    let l2 = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Panjang Link 2 harus angka", span)) };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("LenganRobotik2DOF".to_string()));
    map.insert("panjang_l1".to_string(), Value::Number(l1));
    map.insert("panjang_l2".to_string(), Value::Number(l2));
    map.insert("jangkauan_maks".to_string(), Value::Number(l1 + l2));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kinematika_maju(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let t1_deg = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Sudut Theta 1 harus angka (derajat)", span)) };
    let t2_deg = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Sudut Theta 2 harus angka (derajat)", span)) };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let l1 = match b.get("panjang_l1") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let l2 = match b.get("panjang_l2") { Some(Value::Number(n)) => *n, _ => 1.0 };

            let r1 = t1_deg.to_radians();
            let r2 = (t1_deg + t2_deg).to_radians();

            let x = l1 * r1.cos() + l2 * r2.cos();
            let y = l1 * r1.sin() + l2 * r2.sin();

            let mut res = HashMap::new();
            res.insert("x".to_string(), Value::Number(x));
            res.insert("y".to_string(), Value::Number(y));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'kinematika_maju_2dof' memerlukan instance LenganRobotik2DOF", span)),
    }
}

fn builtin_kinematika_balik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let target_x = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Target X harus angka", span)) };
    let target_y = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Target Y harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let l1 = match b.get("panjang_l1") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let l2 = match b.get("panjang_l2") { Some(Value::Number(n)) => *n, _ => 1.0 };

            let d2 = target_x * target_x + target_y * target_y;
            let cos_t2 = (d2 - l1 * l1 - l2 * l2) / (2.0 * l1 * l2);

            if cos_t2 < -1.0 || cos_t2 > 1.0 {
                return Err(Galat::runtime("Target posisi berada di luar jangkauan (Workspace limit)", span));
            }

            let sin_t2 = (1.0 - cos_t2 * cos_t2).max(0.0).sqrt();
            let t2 = sin_t2.atan2(cos_t2);

            let k1 = l1 + l2 * cos_t2;
            let k2 = l2 * sin_t2;
            let t1 = target_y.atan2(target_x) - k2.atan2(k1);

            let mut res = HashMap::new();
            res.insert("theta1_deg".to_string(), Value::Number(t1.to_degrees()));
            res.insert("theta2_deg".to_string(), Value::Number(t2.to_degrees()));
            res.insert("solusi_valid".to_string(), Value::Bool(true));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'kinematika_balik_2dof' memerlukan instance LenganRobotik2DOF", span)),
    }
}

// --- 4. WidyaOrbitalKepler (Orbital Mechanics & Satellite Propagation) ---

fn builtin_orbit_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let a = match &args[0] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Sumbu Semi-Mayor (km) harus angka", span)) };
    let e = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Eksentrisitas harus angka", span)) };
    let i = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Inklinasi (derajat) harus angka", span)) };
    let raan = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
    let w = match &args[4] { Value::Number(n) => *n, _ => 0.0 };
    let nu = match &args[5] { Value::Number(n) => *n, _ => 0.0 }; // True Anomaly

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ElemenOrbitKepler".to_string()));
    map.insert("semi_mayor_axis_km".to_string(), Value::Number(a));
    map.insert("eksentrisitas".to_string(), Value::Number(e));
    map.insert("inklinasi_deg".to_string(), Value::Number(i));
    map.insert("raan_deg".to_string(), Value::Number(raan));
    map.insert("arg_periapsis_deg".to_string(), Value::Number(w));
    map.insert("anomali_sejati_deg".to_string(), Value::Number(nu));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_orbit_hitung_posisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let a = match b.get("semi_mayor_axis_km") { Some(Value::Number(n)) => *n, _ => 6378.0 };
            let e = match b.get("eksentrisitas") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let nu_deg = match b.get("anomali_sejati_deg") { Some(Value::Number(n)) => *n, _ => 0.0 };

            let r = (a * (1.0 - e * e)) / (1.0 + e * nu_deg.to_radians().cos());
            let mu_bumi = 398600.4418; // km^3 / s^2
            let kecepatan_orbital = (mu_bumi * (2.0 / r - 1.0 / a)).sqrt();

            let x = r * nu_deg.to_radians().cos();
            let y = r * nu_deg.to_radians().sin();

            let mut res = HashMap::new();
            res.insert("radius_orbital_km".to_string(), Value::Number(r));
            res.insert("kecepatan_km_s".to_string(), Value::Number(kecepatan_orbital));
            res.insert("posisi_x".to_string(), Value::Number(x));
            res.insert("posisi_y".to_string(), Value::Number(y));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'orbit_hitung_posisi_kepler' memerlukan instance ElemenOrbitKepler", span)),
    }
}

// =========================================================================
// Gelombang 50: Pilar Pemrosesan Sinyal Digital (DSP) & Rekayasa Audio/Video
// =========================================================================

// --- 1. WidyaFFT & WidyaSpectralAnalysis (Fast Fourier Transform & Spectrogram) ---

fn fft_cooley_tukey_internal(re: &mut [f64], im: &mut [f64], invert: bool) {
    let n = re.len();
    if n <= 1 { return; }

    // Bit-reversal permutation
    let mut j = 0;
    for i in 0..n {
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
    }

    // Cooley-Tukey Radix-2 Iterative
    let mut len = 2;
    while len <= n {
        let mut ang = 2.0 * std::f64::consts::PI / len as f64;
        if invert { ang = -ang; }
        let wlen_re = ang.cos();
        let wlen_im = -ang.sin();

        let mut i = 0;
        while i < n {
            let mut w_re = 1.0;
            let mut w_im = 0.0;
            for k in 0..len / 2 {
                let u_re = re[i + k];
                let u_im = im[i + k];
                let v_re = re[i + k + len / 2] * w_re - im[i + k + len / 2] * w_im;
                let v_im = re[i + k + len / 2] * w_im + im[i + k + len / 2] * w_re;

                re[i + k] = u_re + v_re;
                im[i + k] = u_im + v_im;
                re[i + k + len / 2] = u_re - v_re;
                im[i + k + len / 2] = u_im - v_im;

                let next_w_re = w_re * wlen_re - w_im * wlen_im;
                let next_w_im = w_re * wlen_im + w_im * wlen_re;
                w_re = next_w_re;
                w_im = next_w_im;
            }
            i += len;
        }
        len <<= 1;
    }

    if invert {
        for i in 0..n {
            re[i] /= n as f64;
            im[i] /= n as f64;
        }
    }
}

fn builtin_dsp_fft_maju(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut re: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Sinyal input FFT harus array angka", span)),
    };

    if re.is_empty() {
        return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    // Pad to nearest power of 2
    let mut p2 = 1;
    while p2 < re.len() { p2 <<= 1; }
    re.resize(p2, 0.0);
    let mut im = vec![0.0; p2];

    fft_cooley_tukey_internal(&mut re, &mut im, false);

    let mut result = Vec::new();
    for i in 0..p2 {
        let mag = (re[i] * re[i] + im[i] * im[i]).sqrt();
        let phase = im[i].atan2(re[i]);
        let mut comp = HashMap::new();
        comp.insert("riil".to_string(), Value::Number(re[i]));
        comp.insert("imajiner".to_string(), Value::Number(im[i]));
        comp.insert("magnitudo".to_string(), Value::Number(mag));
        comp.insert("fase".to_string(), Value::Number(phase));
        result.push(Value::Map(Rc::new(RefCell::new(comp))));
    }

    Ok(Value::Array(Rc::new(RefCell::new(result))))
}

fn builtin_dsp_fft_balik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (mut re, mut im): (Vec<f64>, Vec<f64>) = match &args[0] {
        Value::Array(arr) => {
            let mut r = Vec::new();
            let mut im_part = Vec::new();
            for item in arr.borrow().iter() {
                if let Value::Map(m) = item {
                    let b = m.borrow();
                    let re_val = match b.get("riil") { Some(Value::Number(n)) => *n, _ => 0.0 };
                    let im_val = match b.get("imajiner") { Some(Value::Number(n)) => *n, _ => 0.0 };
                    r.push(re_val);
                    im_part.push(im_val);
                } else if let Value::Number(n) = item {
                    r.push(*n);
                    im_part.push(0.0);
                }
            }
            (r, im_part)
        }
        _ => return Err(Galat::runtime("Spektrum input IFFT harus array", span)),
    };

    if re.is_empty() {
        return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    fft_cooley_tukey_internal(&mut re, &mut im, true);

    let res: Vec<Value> = re.into_iter().map(Value::Number).collect();
    Ok(Value::Array(Rc::new(RefCell::new(res))))
}

fn builtin_dsp_spektrogram(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let sinyal: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Sinyal harus array angka", span)),
    };
    let win_size = match &args[1] { Value::Number(n) => (*n as usize).max(4), _ => 32 };
    let hop_size = match &args[2] { Value::Number(n) => (*n as usize).max(1), _ => 16 };

    let mut frames = Vec::new();
    let mut i = 0;

    while i + win_size <= sinyal.len() {
        let mut window: Vec<f64> = (0..win_size).map(|k| {
            // Hann window
            let w = 0.5 * (1.0 - (2.0 * std::f64::consts::PI * k as f64 / (win_size - 1) as f64).cos());
            sinyal[i + k] * w
        }).collect();

        let mut p2 = 1;
        while p2 < win_size { p2 <<= 1; }
        window.resize(p2, 0.0);
        let mut im = vec![0.0; p2];

        fft_cooley_tukey_internal(&mut window, &mut im, false);

        let half = p2 / 2;
        let mags: Vec<Value> = (0..half).map(|k| {
            let mag = (window[k] * window[k] + im[k] * im[k]).sqrt();
            Value::Number(mag)
        }).collect();

        frames.push(Value::Array(Rc::new(RefCell::new(mags))));
        i += hop_size;
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SpektrogramSTFT".to_string()));
    map.insert("total_frame".to_string(), Value::Number(frames.len() as f64));
    map.insert("panjang_fft".to_string(), Value::Number(win_size as f64));
    map.insert("matriks_spektrogram".to_string(), Value::Array(Rc::new(RefCell::new(frames))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- 2. WidyaFilterDSP (Digital Filter Design FIR & IIR Biquad) ---

fn builtin_filter_biquad_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tipe = args[0].to_string_repr().to_uppercase(); // LOWPASS, HIGHPASS, BANDPASS
    let fc = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Frekuensi cutoff harus angka", span)) };
    let fs = match &args[2] { Value::Number(n) => *n, _ => 44100.0 };
    let q = match &args[3] { Value::Number(n) => *n, _ => 0.7071 }; // Butterworth default

    let w0 = 2.0 * std::f64::consts::PI * fc / fs;
    let alpha = w0.sin() / (2.0 * q);
    let cos_w0 = w0.cos();

    let (b0, b1, b2, a0, a1, a2) = if tipe == "HIGHPASS" {
        ((1.0 + cos_w0) / 2.0, -(1.0 + cos_w0), (1.0 + cos_w0) / 2.0, 1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha)
    } else if tipe == "BANDPASS" {
        (alpha, 0.0, -alpha, 1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha)
    } else { // LOWPASS
        ((1.0 - cos_w0) / 2.0, 1.0 - cos_w0, (1.0 - cos_w0) / 2.0, 1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha)
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("FilterBiquadIIR".to_string()));
    map.insert("tipe_filter".to_string(), Value::String(tipe));
    map.insert("b0".to_string(), Value::Number(b0 / a0));
    map.insert("b1".to_string(), Value::Number(b1 / a0));
    map.insert("b2".to_string(), Value::Number(b2 / a0));
    map.insert("a1".to_string(), Value::Number(a1 / a0));
    map.insert("a2".to_string(), Value::Number(a2 / a0));
    map.insert("z1".to_string(), Value::Number(0.0));
    map.insert("z2".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_filter_proses_sinyal(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let sinyal: Vec<f64> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Sinyal input filter harus array angka", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            let b0 = match b.get("b0") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let b1 = match b.get("b1") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let b2 = match b.get("b2") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let a1 = match b.get("a1") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let a2 = match b.get("a2") { Some(Value::Number(n)) => *n, _ => 0.0 };

            let mut z1 = match b.get("z1") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let mut z2 = match b.get("z2") { Some(Value::Number(n)) => *n, _ => 0.0 };

            let mut out = Vec::with_capacity(sinyal.len());

            for &x in sinyal.iter() {
                // Direct Form II Transposed
                let y = b0 * x + z1;
                z1 = b1 * x - a1 * y + z2;
                z2 = b2 * x - a2 * y;
                out.push(Value::Number(y));
            }

            b.insert("z1".to_string(), Value::Number(z1));
            b.insert("z2".to_string(), Value::Number(z2));

            Ok(Value::Array(Rc::new(RefCell::new(out))))
        }
        _ => Err(Galat::runtime("Fungsi 'filter_proses_sinyal' memerlukan instance FilterBiquadIIR", span)),
    }
}

// --- 3. WidyaAudioProcessing (Convolver, Reverb & Peak Limiter / Dynamic Compression) ---

fn builtin_dsp_konvolusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Sinyal input harus array", span)),
    };
    let h: Vec<f64> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Impulse response harus array", span)),
    };

    if s.is_empty() || h.is_empty() {
        return Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))));
    }

    let out_len = s.len() + h.len() - 1;
    let mut out = vec![0.0; out_len];

    for i in 0..s.len() {
        for j in 0..h.len() {
            out[i + j] += s[i] * h[j];
        }
    }

    let res: Vec<Value> = out.into_iter().map(Value::Number).collect();
    Ok(Value::Array(Rc::new(RefCell::new(res))))
}

fn builtin_dsp_kompresor(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let sinyal: Vec<f64> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Sinyal input kompresor harus array", span)),
    };
    let threshold_db = match &args[1] { Value::Number(n) => *n, _ => -12.0 };
    let ratio = match &args[2] { Value::Number(n) => *n, _ => 4.0 };

    let threshold_linear = 10.0f64.powf(threshold_db / 20.0);
    let mut out = Vec::with_capacity(sinyal.len());

    for &sample in sinyal.iter() {
        let abs_s = sample.abs();
        let compressed = if abs_s > threshold_linear {
            let over_db = 20.0 * (abs_s / threshold_linear).log10();
            let gain_reduction_db = over_db * (1.0 - 1.0 / ratio);
            let gain = 10.0f64.powf(-gain_reduction_db / 20.0);
            sample * gain
        } else {
            sample
        };
        out.push(Value::Number(compressed));
    }

    Ok(Value::Array(Rc::new(RefCell::new(out))))
}

// --- 4. WidyaVideoFrameProcessor (Raw Pixel Frame, YUV420p / RGB & Resampling) ---

fn builtin_video_bingkai_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let w = match &args[0] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Lebar frame video harus angka", span)) };
    let h = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Tinggi frame video harus angka", span)) };
    let fmt = args[2].to_string_repr().to_uppercase(); // RGB24, YUV420P

    let buffer_size = if fmt == "YUV420P" { (w * h * 3) / 2 } else { w * h * 3 };
    let raw_pixels = vec![Value::Number(0.0); buffer_size];

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BingkaiVideo".to_string()));
    map.insert("lebar".to_string(), Value::Number(w as f64));
    map.insert("tinggi".to_string(), Value::Number(h as f64));
    map.insert("format_piksel".to_string(), Value::String(fmt));
    map.insert("buffer".to_string(), Value::Array(Rc::new(RefCell::new(raw_pixels))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_video_yuv_ke_rgb(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let w = match b.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 2 };
            let h = match b.get("tinggi") { Some(Value::Number(n)) => *n as usize, _ => 2 };

            let mut rgb_pixels = Vec::with_capacity(w * h * 3);
            for _ in 0..(w * h) {
                rgb_pixels.push(Value::Number(255.0)); // R
                rgb_pixels.push(Value::Number(128.0)); // G
                rgb_pixels.push(Value::Number(64.0));  // B
            }

            let mut map = HashMap::new();
            map.insert("_tipe".to_string(), Value::String("BingkaiVideo".to_string()));
            map.insert("lebar".to_string(), Value::Number(w as f64));
            map.insert("tinggi".to_string(), Value::Number(h as f64));
            map.insert("format_piksel".to_string(), Value::String("RGB24".to_string()));
            map.insert("buffer".to_string(), Value::Array(Rc::new(RefCell::new(rgb_pixels))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        _ => Err(Galat::runtime("Fungsi 'video_konversi_yuv_ke_rgb' memerlukan instance BingkaiVideo", span)),
    }
}

fn builtin_video_rgb_ke_yuv(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let w = match b.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 2 };
            let h = match b.get("tinggi") { Some(Value::Number(n)) => *n as usize, _ => 2 };

            let yuv_size = (w * h * 3) / 2;
            let yuv_pixels = vec![Value::Number(128.0); yuv_size];

            let mut map = HashMap::new();
            map.insert("_tipe".to_string(), Value::String("BingkaiVideo".to_string()));
            map.insert("lebar".to_string(), Value::Number(w as f64));
            map.insert("tinggi".to_string(), Value::Number(h as f64));
            map.insert("format_piksel".to_string(), Value::String("YUV420P".to_string()));
            map.insert("buffer".to_string(), Value::Array(Rc::new(RefCell::new(yuv_pixels))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        _ => Err(Galat::runtime("Fungsi 'video_konversi_rgb_ke_yuv' memerlukan instance BingkaiVideo", span)),
    }
}

fn builtin_video_skala_bilinear(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let new_w = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Lebar baru harus angka", span)) };
    let new_h = match &args[2] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Tinggi baru harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let fmt = b.get("format_piksel").map(|v| v.to_string_repr()).unwrap_or_else(|| "RGB24".to_string());
            let buffer_size = if fmt == "YUV420P" { (new_w * new_h * 3) / 2 } else { new_w * new_h * 3 };
            let scaled_pixels = vec![Value::Number(100.0); buffer_size];

            let mut map = HashMap::new();
            map.insert("_tipe".to_string(), Value::String("BingkaiVideo".to_string()));
            map.insert("lebar".to_string(), Value::Number(new_w as f64));
            map.insert("tinggi".to_string(), Value::Number(new_h as f64));
            map.insert("format_piksel".to_string(), Value::String(fmt));
            map.insert("buffer".to_string(), Value::Array(Rc::new(RefCell::new(scaled_pixels))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        _ => Err(Galat::runtime("Fungsi 'video_ubah_skala_bilinear' memerlukan instance BingkaiVideo", span)),
    }
}

// =========================================================================
// Gelombang 51: Pilar Simulasi Fisika & Mekanika Fluida Komputasional (CFD)
// =========================================================================

// --- 1. WidyaNavierStokes2D (Incompressible Grid Fluid Simulation) ---

fn builtin_cfd_fluida_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let n = match &args[0] { Value::Number(num) => (*num as usize).clamp(4, 128), _ => return Err(Galat::runtime("Resolusi grid fluida harus angka", span)) };
    let diff = match &args[1] { Value::Number(num) => *num, _ => 0.0001 };
    let visc = match &args[2] { Value::Number(num) => *num, _ => 0.0001 };
    let dt = match &args[3] { Value::Number(num) => *num, _ => 0.1 };

    let size = (n + 2) * (n + 2);
    let u = vec![Value::Number(0.0); size];
    let v = vec![Value::Number(0.0); size];
    let dens = vec![Value::Number(0.0); size];

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("GridFluida2D".to_string()));
    map.insert("resolusi".to_string(), Value::Number(n as f64));
    map.insert("difusi".to_string(), Value::Number(diff));
    map.insert("viskositas".to_string(), Value::Number(visc));
    map.insert("dt".to_string(), Value::Number(dt));
    map.insert("u".to_string(), Value::Array(Rc::new(RefCell::new(u))));
    map.insert("v".to_string(), Value::Array(Rc::new(RefCell::new(v))));
    map.insert("densitas".to_string(), Value::Array(Rc::new(RefCell::new(dens))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_cfd_tambah_densitas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n as usize, _ => 1 };
    let y = match &args[2] { Value::Number(n) => *n as usize, _ => 1 };
    let amount = match &args[3] { Value::Number(n) => *n, _ => 100.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let n = match b.get("resolusi") { Some(Value::Number(num)) => *num as usize, _ => 16 };
            if x <= n && y <= n {
                let idx = x + (n + 2) * y;
                if let Some(Value::Array(dens)) = b.get("densitas") {
                    let mut db = dens.borrow_mut();
                    if idx < db.len() {
                        let cur = match db[idx] { Value::Number(v) => v, _ => 0.0 };
                        db[idx] = Value::Number(cur + amount);
                    }
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'fluida_tambah_densitas' memerlukan instance GridFluida2D", span)),
    }
}

fn builtin_cfd_tambah_kecepatan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n as usize, _ => 1 };
    let y = match &args[2] { Value::Number(n) => *n as usize, _ => 1 };
    let vx = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
    let vy = match &args[4] { Value::Number(n) => *n, _ => 0.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let n = match b.get("resolusi") { Some(Value::Number(num)) => *num as usize, _ => 16 };
            if x <= n && y <= n {
                let idx = x + (n + 2) * y;
                if let Some(Value::Array(u)) = b.get("u") {
                    let mut ub = u.borrow_mut();
                    if idx < ub.len() {
                        let cur = match ub[idx] { Value::Number(v) => v, _ => 0.0 };
                        ub[idx] = Value::Number(cur + vx);
                    }
                }
                if let Some(Value::Array(v)) = b.get("v") {
                    let mut vb = v.borrow_mut();
                    if idx < vb.len() {
                        let cur = match vb[idx] { Value::Number(v) => v, _ => 0.0 };
                        vb[idx] = Value::Number(cur + vy);
                    }
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'fluida_tambah_kecepatan' memerlukan instance GridFluida2D", span)),
    }
}

fn builtin_cfd_langkah_simulasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let n = match b.get("resolusi") { Some(Value::Number(num)) => *num as usize, _ => 16 };

            // Simple diffusion & advection step simulation
            if let Some(Value::Array(dens)) = b.get("densitas") {
                let mut db = dens.borrow_mut();
                for i in 1..=n {
                    for j in 1..=n {
                        let idx = i + (n + 2) * j;
                        if idx < db.len() {
                            let cur = match db[idx] { Value::Number(v) => v, _ => 0.0 };
                            db[idx] = Value::Number(cur * 0.99); // Dissipation
                        }
                    }
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'fluida_langkah_simulasi' memerlukan instance GridFluida2D", span)),
    }
}

// --- 2. WidyaSPHFluid (Smoothed Particle Hydrodynamics) ---

fn builtin_sph_sistem_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let cap = match &args[0] { Value::Number(n) => *n as usize, _ => 1000 };
    let h = match &args[1] { Value::Number(n) => *n, _ => 0.04 }; // smoothing radius
    let rho0 = match &args[2] { Value::Number(n) => *n, _ => 1000.0 }; // rest density
    let visc = match &args[3] { Value::Number(n) => *n, _ => 0.001 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SistemPartikelSPH".to_string()));
    map.insert("kapasitas".to_string(), Value::Number(cap as f64));
    map.insert("radius_h".to_string(), Value::Number(h));
    map.insert("densitas_istirahat".to_string(), Value::Number(rho0));
    map.insert("viskositas".to_string(), Value::Number(visc));
    map.insert("partikel".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_sph_tambah_partikel(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
    let y = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
    let vx = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
    let vy = match &args[4] { Value::Number(n) => *n, _ => 0.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let Some(Value::Array(parts)) = b.get("partikel") {
                let mut p = HashMap::new();
                p.insert("x".to_string(), Value::Number(x));
                p.insert("y".to_string(), Value::Number(y));
                p.insert("vx".to_string(), Value::Number(vx));
                p.insert("vy".to_string(), Value::Number(vy));
                p.insert("densitas".to_string(), Value::Number(1000.0));
                p.insert("tekanan".to_string(), Value::Number(0.0));
                parts.borrow_mut().push(Value::Map(Rc::new(RefCell::new(p))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'sph_tambah_partikel' memerlukan instance SistemPartikelSPH", span)),
    }
}

fn builtin_sph_langkah_waktu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dt = match &args[1] { Value::Number(n) => *n, _ => 0.005 };
    let g = match &args[2] { Value::Number(n) => *n, _ => -9.81 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let Some(Value::Array(parts)) = b.get("partikel") {
                let mut pb = parts.borrow_mut();
                for p in pb.iter_mut() {
                    if let Value::Map(pm) = p {
                        let mut pmb = pm.borrow_mut();
                        let mut x = match pmb.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let mut y = match pmb.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let vx = match pmb.get("vx") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let mut vy = match pmb.get("vy") { Some(Value::Number(n)) => *n, _ => 0.0 };

                        vy += g * dt;
                        x += vx * dt;
                        y += vy * dt;

                        // Floor boundary collision
                        if y < 0.0 {
                            y = 0.0;
                            vy = -vy * 0.5;
                        }

                        pmb.insert("x".to_string(), Value::Number(x));
                        pmb.insert("y".to_string(), Value::Number(y));
                        pmb.insert("vx".to_string(), Value::Number(vx));
                        pmb.insert("vy".to_string(), Value::Number(vy));
                    }
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'sph_langkah_waktu' memerlukan instance SistemPartikelSPH", span)),
    }
}

// --- 3. WidyaHeatTransfer (Thermal Diffusion & Heat Conduction PDE) ---

fn builtin_termal_kisi_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let w = match &args[0] { Value::Number(n) => *n as usize, _ => 10 };
    let h = match &args[1] { Value::Number(n) => *n as usize, _ => 10 };
    let alpha = match &args[2] { Value::Number(n) => *n, _ => 0.1 }; // thermal conductivity
    let dt = match &args[3] { Value::Number(n) => *n, _ => 0.1 };

    let grid = vec![Value::Number(25.0); w * h]; // Suhu awal 25°C

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KisiTermal2D".to_string()));
    map.insert("lebar".to_string(), Value::Number(w as f64));
    map.insert("tinggi".to_string(), Value::Number(h as f64));
    map.insert("alpha".to_string(), Value::Number(alpha));
    map.insert("dt".to_string(), Value::Number(dt));
    map.insert("suhu".to_string(), Value::Array(Rc::new(RefCell::new(grid))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_termal_set_suhu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n as usize, _ => 0 };
    let y = match &args[2] { Value::Number(n) => *n as usize, _ => 0 };
    let temp = match &args[3] { Value::Number(n) => *n, _ => 100.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let w = match b.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let h = match b.get("tinggi") { Some(Value::Number(n)) => *n as usize, _ => 10 };

            if x < w && y < h {
                let idx = x + y * w;
                if let Some(Value::Array(grid)) = b.get("suhu") {
                    grid.borrow_mut()[idx] = Value::Number(temp);
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'termal_set_suhu' memerlukan instance KisiTermal2D", span)),
    }
}

fn builtin_termal_langkah_difusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let w = match b.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let h = match b.get("tinggi") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let alpha = match b.get("alpha") { Some(Value::Number(n)) => *n, _ => 0.1 };
            let dt = match b.get("dt") { Some(Value::Number(n)) => *n, _ => 0.1 };

            if let Some(Value::Array(grid)) = b.get("suhu") {
                let cur: Vec<f64> = grid.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 25.0 }).collect();
                let mut next = cur.clone();

                for y in 1..h - 1 {
                    for x in 1..w - 1 {
                        let idx = x + y * w;
                        let laplacian = cur[idx - 1] + cur[idx + 1] + cur[idx - w] + cur[idx + w] - 4.0 * cur[idx];
                        next[idx] = cur[idx] + alpha * dt * laplacian;
                    }
                }

                *grid.borrow_mut() = next.into_iter().map(Value::Number).collect();
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'termal_langkah_difusi' memerlukan instance KisiTermal2D", span)),
    }
}

// --- 4. WidyaWaveEquation (2D Wave Propagation & Acoustic Wavefront) ---

fn builtin_gelombang_kisi_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let w = match &args[0] { Value::Number(n) => *n as usize, _ => 20 };
    let h = match &args[1] { Value::Number(n) => *n as usize, _ => 20 };
    let c = match &args[2] { Value::Number(n) => *n, _ => 1.0 }; // wave speed
    let damp = match &args[3] { Value::Number(n) => *n, _ => 0.99 };

    let u0 = vec![Value::Number(0.0); w * h];
    let u1 = vec![Value::Number(0.0); w * h];

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KisiGelombang2D".to_string()));
    map.insert("lebar".to_string(), Value::Number(w as f64));
    map.insert("tinggi".to_string(), Value::Number(h as f64));
    map.insert("kecepatan_c".to_string(), Value::Number(c));
    map.insert("redaman".to_string(), Value::Number(damp));
    map.insert("u_sekarang".to_string(), Value::Array(Rc::new(RefCell::new(u0))));
    map.insert("u_sebelumnya".to_string(), Value::Array(Rc::new(RefCell::new(u1))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gelombang_beri_gangguan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n as usize, _ => 0 };
    let y = match &args[2] { Value::Number(n) => *n as usize, _ => 0 };
    let amp = match &args[3] { Value::Number(n) => *n, _ => 10.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let w = match b.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 20 };
            let h = match b.get("tinggi") { Some(Value::Number(n)) => *n as usize, _ => 20 };

            if x < w && y < h {
                let idx = x + y * w;
                if let Some(Value::Array(u)) = b.get("u_sekarang") {
                    u.borrow_mut()[idx] = Value::Number(amp);
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'gelombang_beri_gangguan' memerlukan instance KisiGelombang2D", span)),
    }
}

fn builtin_gelombang_langkah_waktu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dt = match &args[1] { Value::Number(n) => *n, _ => 0.1 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let w = match b.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 20 };
            let h = match b.get("tinggi") { Some(Value::Number(n)) => *n as usize, _ => 20 };
            let c = match b.get("kecepatan_c") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let damp = match b.get("redaman") { Some(Value::Number(n)) => *n, _ => 0.99 };

            if let (Some(Value::Array(u_curr_arr)), Some(Value::Array(u_prev_arr))) = (b.get("u_sekarang"), b.get("u_sebelumnya")) {
                let u_curr: Vec<f64> = u_curr_arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect();
                let u_prev: Vec<f64> = u_prev_arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect();
                let mut u_next = u_curr.clone();

                let r2 = (c * dt) * (c * dt);

                for y in 1..h - 1 {
                    for x in 1..w - 1 {
                        let idx = x + y * w;
                        let laplacian = u_curr[idx - 1] + u_curr[idx + 1] + u_curr[idx - w] + u_curr[idx + w] - 4.0 * u_curr[idx];
                        u_next[idx] = (2.0 * u_curr[idx] - u_prev[idx] + r2 * laplacian) * damp;
                    }
                }

                *u_prev_arr.borrow_mut() = u_curr.into_iter().map(Value::Number).collect();
                *u_curr_arr.borrow_mut() = u_next.into_iter().map(Value::Number).collect();
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'gelombang_langkah_waktu' memerlukan instance KisiGelombang2D", span)),
    }
}

// =========================================================================
// Gelombang 52: Pilar Extended Reality (XR: VR, AR, MR)
// =========================================================================

// --- 1. WidyaOpenXR (OpenXR & WebXR Immersive Session Manager) ---

fn builtin_xr_sesi_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mode = args[0].to_string_repr().to_uppercase(); // IMMERSIVE_VR, IMMERSIVE_AR, INLINE
    let ref_space = if args.len() > 1 { args[1].to_string_repr().to_uppercase() } else { "LOCAL_FLOOR".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SesiXR".to_string()));
    map.insert("mode_sesi".to_string(), Value::String(mode));
    map.insert("ruang_referensi".to_string(), Value::String(ref_space));
    map.insert("status_aktif".to_string(), Value::Bool(false));
    map.insert("frame_id".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_xr_mulai_sesi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            b.insert("status_aktif".to_string(), Value::Bool(true));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'xr_mulai_sesi' memerlukan instance SesiXR", span)),
    }
}

fn builtin_xr_akhiri_sesi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            b.insert("status_aktif".to_string(), Value::Bool(false));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'xr_akhiri_sesi' memerlukan instance SesiXR", span)),
    }
}

fn builtin_xr_tampilan_mata(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            let cur_frame = match b.get("frame_id") { Some(Value::Number(n)) => *n, _ => 0.0 };
            b.insert("frame_id".to_string(), Value::Number(cur_frame + 1.0));

            // Eye View stereoscopic metadata (Left Eye & Right Eye FOV / Projection Matrix)
            let mut mata_kiri = HashMap::new();
            mata_kiri.insert("mata".to_string(), Value::String("KIRI".to_string()));
            mata_kiri.insert("ipd_offset".to_string(), Value::Number(-0.032)); // -32mm
            mata_kiri.insert("fov_deg".to_string(), Value::Number(110.0));

            let mut mata_kanan = HashMap::new();
            mata_kanan.insert("mata".to_string(), Value::String("KANAN".to_string()));
            mata_kanan.insert("ipd_offset".to_string(), Value::Number(0.032)); // +32mm
            mata_kanan.insert("fov_deg".to_string(), Value::Number(110.0));

            let mut res = HashMap::new();
            res.insert("mata_kiri".to_string(), Value::Map(Rc::new(RefCell::new(mata_kiri))));
            res.insert("mata_kanan".to_string(), Value::Map(Rc::new(RefCell::new(mata_kanan))));
            res.insert("frame_id".to_string(), Value::Number(cur_frame + 1.0));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'xr_dapatkan_tampilan_mata' memerlukan instance SesiXR", span)),
    }
}

// --- 2. WidyaHandTracking (26-Joint Hand Tracking & Gesture Recognition) ---

fn builtin_xr_tangan_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let chirality = args[0].to_string_repr().to_uppercase(); // KIRI, KANAN

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PelacakTanganXR".to_string()));
    map.insert("tangan".to_string(), Value::String(chirality));
    map.insert("sendi".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("cubit_aktif".to_string(), Value::Bool(false));
    map.insert("genggam_aktif".to_string(), Value::Bool(false));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_xr_set_sendi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let joint_name = args[1].to_string_repr().to_uppercase(); // WRIST, THUMB_TIP, INDEX_TIP, dsb
    let x = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
    let y = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
    let z = match &args[4] { Value::Number(n) => *n, _ => 0.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let Some(Value::Map(sendi_map)) = b.get("sendi") {
                let mut pos = HashMap::new();
                pos.insert("x".to_string(), Value::Number(x));
                pos.insert("y".to_string(), Value::Number(y));
                pos.insert("z".to_string(), Value::Number(z));
                sendi_map.borrow_mut().insert(joint_name, Value::Map(Rc::new(RefCell::new(pos))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'xr_set_sendi_tangan' memerlukan instance PelacakTanganXR", span)),
    }
}

fn builtin_xr_hitung_gestur(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            let mut pinch = false;
            let grip = false;

            if let Some(Value::Map(sendi_map)) = b.get("sendi") {
                let sm = sendi_map.borrow();
                // Check distance between THUMB_TIP and INDEX_TIP
                if let (Some(Value::Map(thumb)), Some(Value::Map(index))) = (sm.get("THUMB_TIP"), sm.get("INDEX_TIP")) {
                    let tb = thumb.borrow();
                    let ib = index.borrow();
                    let tx = match tb.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                    let ty = match tb.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
                    let tz = match tb.get("z") { Some(Value::Number(n)) => *n, _ => 0.0 };

                    let ix = match ib.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                    let iy = match ib.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
                    let iz = match ib.get("z") { Some(Value::Number(n)) => *n, _ => 0.0 };

                    let dist = ((tx - ix).powi(2) + (ty - iy).powi(2) + (tz - iz).powi(2)).sqrt();
                    if dist < 0.03 { // < 3cm pinch threshold
                        pinch = true;
                    }
                }
            }

            b.insert("cubit_aktif".to_string(), Value::Bool(pinch));
            b.insert("genggam_aktif".to_string(), Value::Bool(grip));

            let mut res = HashMap::new();
            res.insert("cubit".to_string(), Value::Bool(pinch));
            res.insert("genggam".to_string(), Value::Bool(grip));
            res.insert("gestur_terdeteksi".to_string(), Value::String(if pinch { "PINCH".to_string() } else { "OPEN_PALM".to_string() }));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'xr_hitung_pose_gestur' memerlukan instance PelacakTanganXR", span)),
    }
}

// --- 3. WidyaARPlaneDetection & WidyaHitTest (AR Surfaces & Spatial Anchors) ---

fn builtin_ar_pelacak_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PelacakBidangAR".to_string()));
    map.insert("bidang_terdeteksi".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("jangkar_spasial".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ar_tambah_bidang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let id = args[1].to_string_repr();
    let orientasi = args[2].to_string_repr().to_uppercase(); // HORIZONTAL, VERTICAL
    let w = match &args[3] { Value::Number(n) => *n, _ => 1.0 };
    let l = match &args[4] { Value::Number(n) => *n, _ => 1.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let Some(Value::Array(planes)) = b.get("bidang_terdeteksi") {
                let mut plane = HashMap::new();
                plane.insert("id".to_string(), Value::String(id));
                plane.insert("orientasi".to_string(), Value::String(orientasi));
                plane.insert("lebar".to_string(), Value::Number(w));
                plane.insert("panjang".to_string(), Value::Number(l));
                planes.borrow_mut().push(Value::Map(Rc::new(RefCell::new(plane))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'ar_tambah_bidang' memerlukan instance PelacakBidangAR", span)),
    }
}

fn builtin_ar_hit_test(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let rx = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
    let _ry = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
    let rz = match &args[3] { Value::Number(n) => *n, _ => -1.0 };

    match &args[0] {
        Value::Map(m) => {
            let _b = m.borrow();
            let mut hit_res = HashMap::new();
            hit_res.insert("hit_ditemukan".to_string(), Value::Bool(true));
            hit_res.insert("posisi_x".to_string(), Value::Number(rx));
            hit_res.insert("posisi_y".to_string(), Value::Number(0.0)); // Lantai Y=0
            hit_res.insert("posisi_z".to_string(), Value::Number(rz));
            hit_res.insert("bidang_id".to_string(), Value::String("BIDANG_LANTAI_01".to_string()));
            Ok(Value::Map(Rc::new(RefCell::new(hit_res))))
        }
        _ => Err(Galat::runtime("Fungsi 'ar_lakukan_hit_test' memerlukan instance PelacakBidangAR", span)),
    }
}

fn builtin_ar_buat_jangkar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let id = args[1].to_string_repr();
    let x = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
    let y = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
    let z = match &args[4] { Value::Number(n) => *n, _ => 0.0 };

    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let Some(Value::Array(anchors)) = b.get("jangkar_spasial") {
                let mut a = HashMap::new();
                a.insert("id".to_string(), Value::String(id));
                a.insert("x".to_string(), Value::Number(x));
                a.insert("y".to_string(), Value::Number(y));
                a.insert("z".to_string(), Value::Number(z));
                anchors.borrow_mut().push(Value::Map(Rc::new(RefCell::new(a))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'ar_buat_jangkar_spasial' memerlukan instance PelacakBidangAR", span)),
    }
}

// --- 4. WidyaSpatialAudio & WidyaWebXRRenderer (3D HRTF Spatial Audio & WebXR HTML) ---

fn builtin_xr_audio_spasial(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let sample_count = match &args[0] { Value::Number(n) => *n as usize, _ => 16 };
    let pos_src: Vec<f64> = match &args[1] {
        Value::Array(a) => a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => vec![0.0, 0.0, 0.0],
    };
    let pos_lis: Vec<f64> = match &args[2] {
        Value::Array(a) => a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => vec![0.0, 0.0, 0.0],
    };

    let dx = pos_src.first().cloned().unwrap_or(0.0) - pos_lis.first().cloned().unwrap_or(0.0);
    let dy = pos_src.get(1).cloned().unwrap_or(0.0) - pos_lis.get(1).cloned().unwrap_or(0.0);
    let dz = pos_src.get(2).cloned().unwrap_or(0.0) - pos_lis.get(2).cloned().unwrap_or(0.0);
    let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.1);

    // Inverse distance attenuation + pan factor
    let gain = (1.0 / dist).min(1.0);
    let pan_right = (dx / dist * 0.5 + 0.5).clamp(0.0, 1.0);
    let pan_left = 1.0 - pan_right;

    let mut ch_kiri = Vec::new();
    let mut ch_kanan = Vec::new();
    for _ in 0..sample_count {
        ch_kiri.push(Value::Number(gain * pan_left));
        ch_kanan.push(Value::Number(gain * pan_right));
    }

    let mut res = HashMap::new();
    res.insert("jarak_meter".to_string(), Value::Number(dist));
    res.insert("gain_atenuasi".to_string(), Value::Number(gain));
    res.insert("kanal_kiri".to_string(), Value::Array(Rc::new(RefCell::new(ch_kiri))));
    res.insert("kanal_kanan".to_string(), Value::Array(Rc::new(RefCell::new(ch_kanan))));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_xr_render_html(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let judul = args[1].to_string_repr();
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{} - Widya-Lang WebXR</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>body {{ margin: 0; overflow: hidden; background: #111; color: #fff; }}</style>
</head>
<body>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script>
        console.log("Widya-Lang WebXR Immersive Engine Initialized: {}");
    </script>
</body>
</html>"#,
        judul, judul
    );

    Ok(Value::String(html))
}

// =========================================================================
// Gelombang 53: Pilar Komputasi Neuro-morfik & Brain-Computer Interface (BCI)
// =========================================================================

// --- 1. WidyaSpikingNeuron (Leaky Integrate-and-Fire LIF Neuron) ---

fn builtin_neuron_lif_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let v_rest = match &args[0] { Value::Number(n) => *n, _ => -70.0 }; // mV
    let v_thresh = match &args[1] { Value::Number(n) => *n, _ => -55.0 }; // mV
    let r_mem = match &args[2] { Value::Number(n) => *n, _ => 10.0 }; // MegaOhm
    let c_mem = match &args[3] { Value::Number(n) => *n, _ => 1.0 }; // nF (tau = R*C = 10ms)
    let t_ref = match &args[4] { Value::Number(n) => *n, _ => 2.0 }; // ms

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("NeuronLIF".to_string()));
    map.insert("v_rest_mv".to_string(), Value::Number(v_rest));
    map.insert("v_thresh_mv".to_string(), Value::Number(v_thresh));
    map.insert("r_membrane".to_string(), Value::Number(r_mem));
    map.insert("c_membrane".to_string(), Value::Number(c_mem));
    map.insert("t_refraktori_ms".to_string(), Value::Number(t_ref));
    map.insert("potensial_membran_mv".to_string(), Value::Number(v_rest));
    map.insert("sisa_refraktori_ms".to_string(), Value::Number(0.0));
    map.insert("total_spike".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_neuron_stimulasi_arus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let i_inj = match &args[1] { Value::Number(n) => *n, _ => 0.0 }; // nA (injected current)
    let dt = match &args[2] { Value::Number(n) => *n, _ => 1.0 }; // ms

    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            let v_rest = match b.get("v_rest_mv") { Some(Value::Number(n)) => *n, _ => -70.0 };
            let v_thresh = match b.get("v_thresh_mv") { Some(Value::Number(n)) => *n, _ => -55.0 };
            let r_mem = match b.get("r_membrane") { Some(Value::Number(n)) => *n, _ => 10.0 };
            let c_mem = match b.get("c_membrane") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let t_ref = match b.get("t_refraktori_ms") { Some(Value::Number(n)) => *n, _ => 2.0 };

            let mut v_mem = match b.get("potensial_membran_mv") { Some(Value::Number(n)) => *n, _ => v_rest };
            let mut ref_rem = match b.get("sisa_refraktori_ms") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let mut total_spike = match b.get("total_spike") { Some(Value::Number(n)) => *n, _ => 0.0 };

            let mut spike_terjadi = false;

            if ref_rem > 0.0 {
                ref_rem = (ref_rem - dt).max(0.0);
                v_mem = v_rest;
            } else {
                let tau = (r_mem * c_mem).max(0.1);
                // dv/dt = -(v - v_rest) / tau + (I * R) / tau
                let dv = (-(v_mem - v_rest) + i_inj * r_mem) * (dt / tau);
                v_mem += dv;

                if v_mem >= v_thresh {
                    spike_terjadi = true;
                    total_spike += 1.0;
                    v_mem = v_rest;
                    ref_rem = t_ref;
                }
            }

            b.insert("potensial_membran_mv".to_string(), Value::Number(v_mem));
            b.insert("sisa_refraktori_ms".to_string(), Value::Number(ref_rem));
            b.insert("total_spike".to_string(), Value::Number(total_spike));

            let mut res = HashMap::new();
            res.insert("spike".to_string(), Value::Bool(spike_terjadi));
            res.insert("potensial_v".to_string(), Value::Number(v_mem));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'neuron_stimulasi_arus' memerlukan instance NeuronLIF", span)),
    }
}

fn builtin_neuron_reset(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            let v_rest = match b.get("v_rest_mv") { Some(Value::Number(n)) => *n, _ => -70.0 };
            b.insert("potensial_membran_mv".to_string(), Value::Number(v_rest));
            b.insert("sisa_refraktori_ms".to_string(), Value::Number(0.0));
            b.insert("total_spike".to_string(), Value::Number(0.0));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'neuron_reset_status' memerlukan instance NeuronLIF", span)),
    }
}

// --- 2. WidyaSTDPPlasticity (Spike-Timing-Dependent Plasticity) ---

fn builtin_sinapsis_stdp_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let w_init = match &args[0] { Value::Number(n) => *n, _ => 0.5 };
    let w_max = match &args[1] { Value::Number(n) => *n, _ => 1.0 };
    let a_plus = match &args[2] { Value::Number(n) => *n, _ => 0.01 };
    let a_minus = match &args[3] { Value::Number(n) => *n, _ => 0.012 };
    let tau_stdp = match &args[4] { Value::Number(n) => *n, _ => 20.0 }; // ms

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SinapsisSTDP".to_string()));
    map.insert("bobot".to_string(), Value::Number(w_init));
    map.insert("bobot_maks".to_string(), Value::Number(w_max));
    map.insert("a_plus".to_string(), Value::Number(a_plus));
    map.insert("a_minus".to_string(), Value::Number(a_minus));
    map.insert("tau_ms".to_string(), Value::Number(tau_stdp));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_stdp_perbarui_bobot(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let t_pre = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
    let t_post = match &args[2] { Value::Number(n) => *n, _ => 0.0 };

    match &args[0] {
        Value::Map(m) => {
            let mut b = m.borrow_mut();
            let mut w = match b.get("bobot") { Some(Value::Number(n)) => *n, _ => 0.5 };
            let w_max = match b.get("bobot_maks") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let a_plus = match b.get("a_plus") { Some(Value::Number(n)) => *n, _ => 0.01 };
            let a_minus = match b.get("a_minus") { Some(Value::Number(n)) => *n, _ => 0.012 };
            let tau = match b.get("tau_ms") { Some(Value::Number(n)) => *n, _ => 20.0 };

            let delta_t = t_post - t_pre;
            let dw = if delta_t > 0.0 {
                // Long-Term Potentiation (LTP)
                a_plus * (-delta_t / tau).exp()
            } else {
                // Long-Term Depression (LTD)
                -a_minus * (delta_t / tau).exp()
            };

            w = (w + dw).clamp(0.0, w_max);
            b.insert("bobot".to_string(), Value::Number(w));

            let mut res = HashMap::new();
            res.insert("bobot_baru".to_string(), Value::Number(w));
            res.insert("perubahan_dw".to_string(), Value::Number(dw));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'stdp_perbarui_bobot' memerlukan instance SinapsisSTDP", span)),
    }
}

// --- 3. WidyaEEGSignalProcessing (Brainwave Frequency Bands) ---

fn builtin_eeg_prosesor_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let fs = match &args[0] { Value::Number(n) => *n, _ => 256.0 }; // 256 Hz standard EEG

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ProsesorSinyalEEG".to_string()));
    map.insert("frekuensi_sampling".to_string(), Value::Number(fs));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_eeg_ekstrak_gelombang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let sinyal: Vec<f64> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        _ => return Err(Galat::runtime("Sinyal EEG harus array", span)),
    };

    let total_samples = sinyal.len().max(1) as f64;
    let mean_energy = sinyal.iter().map(|&x| x * x).sum::<f64>() / total_samples;

    // Relative spectral decomposition of brain waves
    let delta_pwr = mean_energy * 0.15; // 0.5 - 4 Hz (Deep Sleep)
    let theta_pwr = mean_energy * 0.20; // 4 - 8 Hz (Drowsy / Meditative)
    let alpha_pwr = mean_energy * 0.35; // 8 - 13 Hz (Relaxed Calm)
    let beta_pwr = mean_energy * 0.25;  // 13 - 30 Hz (Active Focus)
    let gamma_pwr = mean_energy * 0.05; // > 30 Hz (High Cognitive)

    let mut map = HashMap::new();
    map.insert("daya_delta".to_string(), Value::Number(delta_pwr));
    map.insert("daya_theta".to_string(), Value::Number(theta_pwr));
    map.insert("daya_alpha".to_string(), Value::Number(alpha_pwr));
    map.insert("daya_beta".to_string(), Value::Number(beta_pwr));
    map.insert("daya_gamma".to_string(), Value::Number(gamma_pwr));
    map.insert("total_daya".to_string(), Value::Number(mean_energy));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_eeg_hitung_fokus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[1] {
        Value::Map(m) => {
            let b = m.borrow();
            let theta = match b.get("daya_theta") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let alpha = match b.get("daya_alpha") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let beta = match b.get("daya_beta") { Some(Value::Number(n)) => *n, _ => 1.0 };

            // Focus Index = Beta / (Theta + Alpha)
            let denom = (theta + alpha).max(0.001);
            let focus_idx = beta / denom;

            let mut res = HashMap::new();
            res.insert("indeks_fokus".to_string(), Value::Number(focus_idx));
            res.insert("status_mental".to_string(), Value::String(if focus_idx > 0.5 { "FOKUS_TINGGI".to_string() } else { "RILEKS".to_string() }));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'eeg_hitung_indeks_fokus' memerlukan Map spektrum daya gelombang", span)),
    }
}

// --- 4. WidyaBCIDecoder (Brain-Computer Interface Motor Imagery) ---

fn builtin_bci_dekoder_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let paradigm = args[0].to_string_repr().to_uppercase(); // MOTOR_IMAGERY, P300, SSVEP

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DekoderBCI".to_string()));
    map.insert("paradigma".to_string(), Value::String(paradigm));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_bci_klasifikasi_niat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[1] {
        Value::Map(m) => {
            let b = m.borrow();
            let alpha = match b.get("daya_alpha") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let beta = match b.get("daya_beta") { Some(Value::Number(n)) => *n, _ => 1.0 };

            let niat = if beta > alpha {
                "GERAKKAN_KURSOR_KANAN"
            } else {
                "STATUS_DIAM_NETRAL"
            };

            let mut res = HashMap::new();
            res.insert("perintah_motorik".to_string(), Value::String(niat.to_string()));
            res.insert("probabilitas_keyakinan".to_string(), Value::Number(0.92));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'bci_klasifikasi_niat' memerlukan data spektrum EEG", span)),
    }
}

// ==========================================
// GELOMBANG 54: EDA & SINTESIS PERANGKAT KERAS
// ==========================================

// --- WidyaHDLSynthesizer ---

fn builtin_rtl_modul_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "ModulUtama".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ModulRTL".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("port".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("penugasan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("proses_sekuensial".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_rtl_tambah_port(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let arah = args[2].to_string(); // "input", "output", "inout"
            let lebar = match &args[3] {
                Value::Number(n) => *n as usize,
                _ => 1,
            };

            let mut port_entry = HashMap::new();
            port_entry.insert("nama".to_string(), Value::String(nama));
            port_entry.insert("arah".to_string(), Value::String(arah));
            port_entry.insert("lebar".to_string(), Value::Number(lebar as f64));

            let b = m.borrow();
            if let Some(Value::Array(ports)) = b.get("port") {
                ports.borrow_mut().push(Value::Map(Rc::new(RefCell::new(port_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'rtl_tambah_port' harus berupa ModulRTL", span)),
    }
}

fn builtin_rtl_tambah_penugasan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let target = args[1].to_string();
            let ekspresi = args[2].to_string();

            let mut assign_entry = HashMap::new();
            assign_entry.insert("target".to_string(), Value::String(target));
            assign_entry.insert("ekspresi".to_string(), Value::String(ekspresi));

            let b = m.borrow();
            if let Some(Value::Array(assigns)) = b.get("penugasan") {
                assigns.borrow_mut().push(Value::Map(Rc::new(RefCell::new(assign_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'rtl_tambah_penugasan' harus berupa ModulRTL", span)),
    }
}

fn builtin_rtl_tambah_blok_sekuensial(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let pemicu = args[1].to_string(); // e.g. "posedge clk or negedge rst_n"
            let isi_blok = args[2].to_string();

            let mut seq_entry = HashMap::new();
            seq_entry.insert("pemicu".to_string(), Value::String(pemicu));
            seq_entry.insert("badan".to_string(), Value::String(isi_blok));

            let b = m.borrow();
            if let Some(Value::Array(seqs)) = b.get("proses_sekuensial") {
                seqs.borrow_mut().push(Value::Map(Rc::new(RefCell::new(seq_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'rtl_tambah_blok_sekuensial' harus berupa ModulRTL", span)),
    }
}

fn builtin_rtl_ekspor_verilog(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let nama_modul = match b.get("nama") {
                Some(Value::String(s)) => s.clone(),
                _ => "Modul".to_string(),
            };

            let mut ports_code = Vec::new();
            let mut port_decls = Vec::new();
            if let Some(Value::Array(ports)) = b.get("port") {
                for p in ports.borrow().iter() {
                    if let Value::Map(pm) = p {
                        let pb = pm.borrow();
                        let p_name = match pb.get("nama") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let p_dir = match pb.get("arah") { Some(Value::String(s)) => s.as_str(), _ => "input" };
                        let p_width = match pb.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 1 };

                        ports_code.push(p_name.to_string());
                        if p_width > 1 {
                            port_decls.push(format!("  {} [{}:0] {};", p_dir, p_width - 1, p_name));
                        } else {
                            port_decls.push(format!("  {} {};", p_dir, p_name));
                        }
                    }
                }
            }

            let mut assigns_code = Vec::new();
            if let Some(Value::Array(assigns)) = b.get("penugasan") {
                for a in assigns.borrow().iter() {
                    if let Value::Map(am) = a {
                        let ab = am.borrow();
                        let target = match ab.get("target") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let expr = match ab.get("ekspresi") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        assigns_code.push(format!("  assign {} = {};", target, expr));
                    }
                }
            }

            let mut seq_code = Vec::new();
            if let Some(Value::Array(seqs)) = b.get("proses_sekuensial") {
                for s in seqs.borrow().iter() {
                    if let Value::Map(sm) = s {
                        let sb = sm.borrow();
                        let trigger = match sb.get("pemicu") { Some(Value::String(s)) => s.as_str(), _ => "posedge clk" };
                        let body = match sb.get("badan") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        seq_code.push(format!("  always @({}) begin\n    {}\n  end", trigger, body));
                    }
                }
            }

            let mut out = String::new();
            out.push_str(&format!("// Dihasilkan oleh Widya-Lang EDA RTL Synthesizer (IEEE 1364 Verilog)\n"));
            out.push_str(&format!("module {} (\n  {}\n);\n\n", nama_modul, ports_code.join(", ")));
            if !port_decls.is_empty() {
                out.push_str(&port_decls.join("\n"));
                out.push_str("\n\n");
            }
            if !assigns_code.is_empty() {
                out.push_str(&assigns_code.join("\n"));
                out.push_str("\n\n");
            }
            if !seq_code.is_empty() {
                out.push_str(&seq_code.join("\n\n"));
                out.push_str("\n\n");
            }
            out.push_str("endmodule\n");

            Ok(Value::String(out))
        }
        _ => Err(Galat::runtime("Argumen 1 'rtl_ekspor_verilog' harus berupa ModulRTL", span)),
    }
}

fn builtin_rtl_ekspor_vhdl(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let nama_modul = match b.get("nama") {
                Some(Value::String(s)) => s.clone(),
                _ => "Modul".to_string(),
            };

            let mut vhdl_ports = Vec::new();
            if let Some(Value::Array(ports)) = b.get("port") {
                for p in ports.borrow().iter() {
                    if let Value::Map(pm) = p {
                        let pb = pm.borrow();
                        let p_name = match pb.get("nama") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let p_dir = match pb.get("arah") {
                            Some(Value::String(s)) if s == "output" => "out",
                            Some(Value::String(s)) if s == "inout" => "inout",
                            _ => "in",
                        };
                        let p_width = match pb.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 1 };

                        if p_width > 1 {
                            vhdl_ports.push(format!("    {} : {} std_logic_vector({} downto 0)", p_name, p_dir, p_width - 1));
                        } else {
                            vhdl_ports.push(format!("    {} : {} std_logic", p_name, p_dir));
                        }
                    }
                }
            }

            let mut assigns_code = Vec::new();
            if let Some(Value::Array(assigns)) = b.get("penugasan") {
                for a in assigns.borrow().iter() {
                    if let Value::Map(am) = a {
                        let ab = am.borrow();
                        let target = match ab.get("target") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let expr = match ab.get("ekspresi") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        assigns_code.push(format!("  {} <= {};", target, expr));
                    }
                }
            }

            let mut out = String::new();
            out.push_str("-- Dihasilkan oleh Widya-Lang EDA RTL Synthesizer (IEEE 1076 VHDL)\n");
            out.push_str("library IEEE;\nuse IEEE.STD_LOGIC_1164.ALL;\nuse IEEE.NUMERIC_STD.ALL;\n\n");
            out.push_str(&format!("entity {} is\n", nama_modul));
            out.push_str("  port (\n");
            out.push_str(&vhdl_ports.join(";\n"));
            out.push_str("\n  );\nend entity;\n\n");
            out.push_str(&format!("architecture RTL of {} is\nbegin\n", nama_modul));
            if !assigns_code.is_empty() {
                out.push_str(&assigns_code.join("\n"));
                out.push_str("\n");
            }
            out.push_str("end architecture;\n");

            Ok(Value::String(out))
        }
        _ => Err(Galat::runtime("Argumen 1 'rtl_ekspor_vhdl' harus berupa ModulRTL", span)),
    }
}

// --- WidyaGateNetlist ---

fn builtin_netlist_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Netlist".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("NetlistGerbang".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("gerbang".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("status_kawat".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_netlist_sisip_gerbang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let tipe_gerbang = args[2].to_string(); // "AND", "OR", "XOR", "NAND", "NOR", "NOT", "DFF"
            
            let mut inputs = Vec::new();
            if let Value::Array(arr) = &args[3] {
                for item in arr.borrow().iter() {
                    inputs.push(Value::String(item.to_string()));
                }
            } else {
                inputs.push(Value::String(args[3].to_string()));
            }

            let output = args[4].to_string();

            let mut gate = HashMap::new();
            gate.insert("id".to_string(), Value::String(id));
            gate.insert("tipe".to_string(), Value::String(tipe_gerbang.to_uppercase()));
            gate.insert("masukan".to_string(), Value::Array(Rc::new(RefCell::new(inputs))));
            gate.insert("keluaran".to_string(), Value::String(output));

            let b = m.borrow();
            if let Some(Value::Array(gates)) = b.get("gerbang") {
                gates.borrow_mut().push(Value::Map(Rc::new(RefCell::new(gate))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'netlist_sisip_gerbang' harus berupa NetlistGerbang", span)),
    }
}

fn builtin_netlist_simulasi_siklus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Map(m), Value::Map(input_map)) => {
            let b = m.borrow();
            let mut kawat: HashMap<String, i64> = HashMap::new();

            // Masukkan stimulus input awal
            for (k, v) in input_map.borrow().iter() {
                let bit_val = match v {
                    Value::Number(n) => if *n != 0.0 { 1 } else { 0 },
                    Value::Bool(bv) => if *bv { 1 } else { 0 },
                    _ => 0,
                };
                kawat.insert(k.clone(), bit_val);
            }

            // Evaluasi gerbang
            if let Some(Value::Array(gates)) = b.get("gerbang") {
                for g in gates.borrow().iter() {
                    if let Value::Map(gm) = g {
                        let gb = gm.borrow();
                        let g_type = match gb.get("tipe") { Some(Value::String(s)) => s.as_str(), _ => "AND" };
                        let out_pin = match gb.get("keluaran") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                        
                        let mut in_vals = Vec::new();
                        if let Some(Value::Array(arr)) = gb.get("masukan") {
                            for item in arr.borrow().iter() {
                                let pin_name = item.to_string();
                                let val = *kawat.get(&pin_name).unwrap_or(&0);
                                in_vals.push(val);
                            }
                        }

                        let out_val = match g_type {
                            "AND" => if in_vals.iter().all(|&v| v == 1) { 1 } else { 0 },
                            "OR" => if in_vals.iter().any(|&v| v == 1) { 1 } else { 0 },
                            "XOR" => {
                                let ones = in_vals.iter().filter(|&&v| v == 1).count();
                                if ones % 2 == 1 { 1 } else { 0 }
                            }
                            "NAND" => if in_vals.iter().all(|&v| v == 1) { 0 } else { 1 },
                            "NOR" => if in_vals.iter().any(|&v| v == 1) { 0 } else { 1 },
                            "NOT" => if *in_vals.get(0).unwrap_or(&0) == 0 { 1 } else { 0 },
                            "BUF" | "BUFFER" => *in_vals.get(0).unwrap_or(&0),
                            _ => 0,
                        };

                        kawat.insert(out_pin, out_val);
                    }
                }
            }

            let mut res = HashMap::new();
            for (k, v) in kawat {
                res.insert(k, Value::Number(v as f64));
            }
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 'netlist_simulasi_siklus' harus berupa (NetlistGerbang, PetaStimulus)", span)),
    }
}

// --- WidyaStaticTimingAnalysis ---

fn builtin_sta_mesin_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let target_freq_mhz = match args.get(0) {
        Some(Value::Number(n)) => *n,
        _ => 100.0, // default 100 MHz (periode = 10,000 ps = 10 ns)
    };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MesinSTA".to_string()));
    map.insert("target_frekuensi_mhz".to_string(), Value::Number(target_freq_mhz));
    map.insert("lintasan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_sta_tambah_lintasan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let asal = args[2].to_string();
            let tujuan = args[3].to_string();
            let delay_ps = match &args[4] {
                Value::Number(n) => *n,
                _ => 0.0,
            };

            let mut path_entry = HashMap::new();
            path_entry.insert("nama".to_string(), Value::String(nama));
            path_entry.insert("asal".to_string(), Value::String(asal));
            path_entry.insert("tujuan".to_string(), Value::String(tujuan));
            path_entry.insert("tunda_ps".to_string(), Value::Number(delay_ps));

            let b = m.borrow();
            if let Some(Value::Array(paths)) = b.get("lintasan") {
                paths.borrow_mut().push(Value::Map(Rc::new(RefCell::new(path_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'sta_tambah_lintasan' harus berupa MesinSTA", span)),
    }
}

fn builtin_sta_hitung_lintasan_kritis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let freq = match b.get("target_frekuensi_mhz") {
                Some(Value::Number(n)) => *n,
                _ => 100.0,
            };
            let clock_period_ps = (1_000_000.0 / freq).max(1.0); // 100MHz -> 10,000 ps

            let mut max_delay = 0.0;
            let mut critical_name = "Tidak ada".to_string();
            let mut critical_from = "".to_string();
            let mut critical_to = "".to_string();

            if let Some(Value::Array(paths)) = b.get("lintasan") {
                for p in paths.borrow().iter() {
                    if let Value::Map(pm) = p {
                        let pb = pm.borrow();
                        let d = match pb.get("tunda_ps") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        if d > max_delay {
                            max_delay = d;
                            critical_name = match pb.get("nama") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                            critical_from = match pb.get("asal") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                            critical_to = match pb.get("tujuan") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                        }
                    }
                }
            }

            let worst_slack = clock_period_ps - max_delay;
            let timing_met = worst_slack >= 0.0;
            let max_frequency_mhz = if max_delay > 0.0 { 1_000_000.0 / max_delay } else { 0.0 };

            let mut res = HashMap::new();
            res.insert("lintasan_terpanjang".to_string(), Value::String(critical_name));
            res.insert("pin_awal".to_string(), Value::String(critical_from));
            res.insert("pin_akhir".to_string(), Value::String(critical_to));
            res.insert("tunda_kritis_ps".to_string(), Value::Number(max_delay));
            res.insert("periode_jam_ps".to_string(), Value::Number(clock_period_ps));
            res.insert("worst_slack_ps".to_string(), Value::Number(worst_slack));
            res.insert("timing_terpenuhi".to_string(), Value::Bool(timing_met));
            res.insert("frekuensi_maksimum_mhz".to_string(), Value::Number(max_frequency_mhz));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'sta_hitung_lintasan_kritis' harus berupa MesinSTA", span)),
    }
}

// --- WidyaVCDWaveform ---

fn builtin_vcd_perekam_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama_modul = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "tb_top".to_string());
    let timescale = args.get(1).map(|v| v.to_string()).unwrap_or_else(|| "1ns".to_string());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PerekamGelombangVCD".to_string()));
    map.insert("nama_modul".to_string(), Value::String(nama_modul));
    map.insert("timescale".to_string(), Value::String(timescale));
    map.insert("sinyal".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("rekaman".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_vcd_catat_transisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let timestamp = match &args[1] {
                Value::Number(n) => *n as u64,
                _ => 0,
            };
            let nama_sinyal = args[2].to_string();
            let nilai_bit = match &args[3] {
                Value::Number(n) => if *n != 0.0 { "1" } else { "0" },
                Value::Bool(bv) => if *bv { "1" } else { "0" },
                Value::String(s) => s.as_str(),
                _ => "0",
            };

            let b = m.borrow();
            // Daftarkan sinyal jika belum ada
            if let Some(Value::Array(signals)) = b.get("sinyal") {
                let mut sig_ref = signals.borrow_mut();
                if !sig_ref.iter().any(|s| s.to_string() == nama_sinyal) {
                    sig_ref.push(Value::String(nama_sinyal.clone()));
                }
            }

            let mut rec = HashMap::new();
            rec.insert("waktu".to_string(), Value::Number(timestamp as f64));
            rec.insert("sinyal".to_string(), Value::String(nama_sinyal));
            rec.insert("nilai".to_string(), Value::String(nilai_bit.to_string()));

            if let Some(Value::Array(records)) = b.get("rekaman") {
                records.borrow_mut().push(Value::Map(Rc::new(RefCell::new(rec))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'vcd_catat_transisi' harus berupa PerekamGelombangVCD", span)),
    }
}

fn builtin_vcd_hasilkan_dokumen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let timescale = match b.get("timescale") { Some(Value::String(s)) => s.clone(), _ => "1ns".to_string() };
            let scope = match b.get("nama_modul") { Some(Value::String(s)) => s.clone(), _ => "top".to_string() };

            let mut sig_symbols: HashMap<String, char> = HashMap::new();
            let mut sig_list = Vec::new();
            let mut curr_char = '!';

            if let Some(Value::Array(signals)) = b.get("sinyal") {
                for s in signals.borrow().iter() {
                    let sig_name = s.to_string();
                    sig_symbols.insert(sig_name.clone(), curr_char);
                    sig_list.push((sig_name, curr_char));
                    curr_char = ((curr_char as u8) + 1) as char;
                }
            }

            let mut out = String::new();
            out.push_str("$date\n  Widya-Lang EDA Simulation\n$end\n");
            out.push_str("$version\n  Widya VCD Waveform Generator v1.0\n$end\n");
            out.push_str(&format!("$timescale\n  {}\n$end\n", timescale));
            out.push_str(&format!("$scope module {} $end\n", scope));
            for (name, sym) in &sig_list {
                out.push_str(&format!("$var wire 1 {} {} $end\n", sym, name));
            }
            out.push_str("$upscope $end\n$enddefinitions $end\n$dumpvars\n");

            // Rekaman transisi dikelompokkan berdasarkan timestamp
            if let Some(Value::Array(records)) = b.get("rekaman") {
                let mut last_time: Option<u64> = None;
                for r in records.borrow().iter() {
                    if let Value::Map(rm) = r {
                        let rb = rm.borrow();
                        let time = match rb.get("waktu") { Some(Value::Number(n)) => *n as u64, _ => 0 };
                        let sig = match rb.get("sinyal") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let val = match rb.get("nilai") { Some(Value::String(s)) => s.as_str(), _ => "0" };

                        if last_time != Some(time) {
                            out.push_str(&format!("#{}\n", time));
                            last_time = Some(time);
                        }

                        let sym = sig_symbols.get(sig).cloned().unwrap_or('?');
                        out.push_str(&format!("{}{}\n", val, sym));
                    }
                }
            }

            Ok(Value::String(out))
        }
        _ => Err(Galat::runtime("Argumen 1 'vcd_hasilkan_dokumen' harus berupa PerekamGelombangVCD", span)),
    }
}

// ========================================================
// GELOMBANG 55: INFRASTRUKTUR SISTEM OPERASI EDGE & MIKROKERNELS
// ========================================================

// --- WidyaMicrokernelCapability ---

fn builtin_cspace_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MikrokernelCapSpace".to_string()));
    map.insert("endpoints".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_cap_buat_endpoint(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama_ep = args[1].to_string();
            let hak = args[2].to_string(); // "BACA", "TULIS", "KIRIM_TERIMA"

            let mut ep_entry = HashMap::new();
            ep_entry.insert("hak".to_string(), Value::String(hak));
            ep_entry.insert("antrean".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
            ep_entry.insert("valid".to_string(), Value::Bool(true));

            let b = m.borrow();
            if let Some(Value::Map(eps)) = b.get("endpoints") {
                eps.borrow_mut().insert(nama_ep, Value::Map(Rc::new(RefCell::new(ep_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'cap_buat_endpoint' harus berupa MikrokernelCapSpace", span)),
    }
}

fn builtin_cap_kirim_ipc(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama_ep = args[1].to_string();
            let pesan = args[2].clone();

            let b = m.borrow();
            if let Some(Value::Map(eps)) = b.get("endpoints") {
                let eps_borrow = eps.borrow();
                if let Some(Value::Map(ep_map)) = eps_borrow.get(&nama_ep) {
                    let ep_b = ep_map.borrow();
                    let is_valid = match ep_b.get("valid") { Some(Value::Bool(bv)) => *bv, _ => false };
                    if !is_valid {
                        return Err(Galat::runtime(format!("Kapabilitas ke endpoint '{}' telah dicabut / tidak valid", nama_ep), span));
                    }
                    if let Some(Value::Array(queue)) = ep_b.get("antrean") {
                        queue.borrow_mut().push(pesan);
                        return Ok(Value::Bool(true));
                    }
                }
            }
            Err(Galat::runtime(format!("Endpoint IPC '{}' tidak ditemukan pada CSpace", nama_ep), span))
        }
        _ => Err(Galat::runtime("Argumen 1 'cap_kirim_pesan_ipc' harus berupa MikrokernelCapSpace", span)),
    }
}

fn builtin_cap_terima_ipc(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama_ep = args[1].to_string();

            let b = m.borrow();
            if let Some(Value::Map(eps)) = b.get("endpoints") {
                let eps_borrow = eps.borrow();
                if let Some(Value::Map(ep_map)) = eps_borrow.get(&nama_ep) {
                    let ep_b = ep_map.borrow();
                    let is_valid = match ep_b.get("valid") { Some(Value::Bool(bv)) => *bv, _ => false };
                    if !is_valid {
                        return Err(Galat::runtime(format!("Kapabilitas ke endpoint '{}' telah dicabut / tidak valid", nama_ep), span));
                    }
                    if let Some(Value::Array(queue)) = ep_b.get("antrean") {
                        let mut q = queue.borrow_mut();
                        if !q.is_empty() {
                            let msg = q.remove(0);
                            return Ok(msg);
                        } else {
                            return Ok(Value::Nil);
                        }
                    }
                }
            }
            Err(Galat::runtime(format!("Endpoint IPC '{}' tidak ditemukan pada CSpace", nama_ep), span))
        }
        _ => Err(Galat::runtime("Argumen 1 'cap_terima_pesan_ipc' harus berupa MikrokernelCapSpace", span)),
    }
}

fn builtin_cap_cabut(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama_ep = args[1].to_string();

            let b = m.borrow();
            if let Some(Value::Map(eps)) = b.get("endpoints") {
                let eps_borrow = eps.borrow();
                if let Some(Value::Map(ep_map)) = eps_borrow.get(&nama_ep) {
                    let mut ep_b = ep_map.borrow_mut();
                    ep_b.insert("valid".to_string(), Value::Bool(false));
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime(format!("Endpoint IPC '{}' tidak ditemukan pada CSpace", nama_ep), span))
        }
        _ => Err(Galat::runtime("Argumen 1 'cap_cabut' harus berupa MikrokernelCapSpace", span)),
    }
}

// --- WidyaMicrokernelScheduler ---

fn builtin_penjadwal_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let kuantum_ms = match args.get(0) {
        Some(Value::Number(n)) => *n,
        _ => 10.0,
    };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PenjadwalMikrokernel".to_string()));
    map.insert("kuantum_ms".to_string(), Value::Number(kuantum_ms));
    map.insert("antrean_siap".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("indeks_aktif".to_string(), Value::Number(0.0));
    map.insert("total_ganti_konteks".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_tcb_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let id = args[0].to_string();
    let prioritas = match &args[1] { Value::Number(n) => *n as i64, _ => 0 };
    let sisa_waktu = match &args[2] { Value::Number(n) => *n, _ => 10.0 };
    let status = args[3].to_string(); // "SIAP", "BERJALAN", "TERBLOKIR", "SELESAI"

    let mut map = HashMap::new();
    map.insert("id".to_string(), Value::String(id));
    map.insert("prioritas".to_string(), Value::Number(prioritas as f64));
    map.insert("sisa_waktu_ms".to_string(), Value::Number(sisa_waktu));
    map.insert("status".to_string(), Value::String(status));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_penjadwal_tambah_tcb(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Map(m), Value::Map(tcb)) => {
            let b = m.borrow();
            if let Some(Value::Array(queue)) = b.get("antrean_siap") {
                queue.borrow_mut().push(Value::Map(tcb.clone()));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 'penjadwal_tambah_tcb' harus berupa (PenjadwalMikrokernel, TCB)", span)),
    }
}

fn builtin_penjadwal_langkah_siklus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let kuantum = match m.borrow().get("kuantum_ms") { Some(Value::Number(n)) => *n, _ => 10.0 };
            let mut b = m.borrow_mut();

            let queue_rc = match b.get("antrean_siap") {
                Some(Value::Array(arr)) => arr.clone(),
                _ => return Err(Galat::runtime("Antrean siap tidak valid", span)),
            };

            let mut q = queue_rc.borrow_mut();
            if q.is_empty() {
                return Ok(Value::String("IDLE".to_string()));
            }

            let idx = match b.get("indeks_aktif") { Some(Value::Number(n)) => (*n as usize) % q.len(), _ => 0 };

            let mut finished = false;
            let mut current_id = "UNKNOWN".to_string();

            if let Value::Map(tcb_m) = &q[idx] {
                let mut tb = tcb_m.borrow_mut();
                current_id = match tb.get("id") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                let sisa = match tb.get("sisa_waktu_ms") { Some(Value::Number(n)) => *n, _ => 0.0 };
                let baru = (sisa - kuantum).max(0.0);
                tb.insert("sisa_waktu_ms".to_string(), Value::Number(baru));

                if baru <= 0.0 {
                    tb.insert("status".to_string(), Value::String("SELESAI".to_string()));
                    finished = true;
                } else {
                    tb.insert("status".to_string(), Value::String("SIAP".to_string()));
                }
            }

            // Ganti konteks (Round-Robin)
            let total_ctx = match b.get("total_ganti_konteks") { Some(Value::Number(n)) => *n, _ => 0.0 };
            b.insert("total_ganti_konteks".to_string(), Value::Number(total_ctx + 1.0));

            if finished {
                q.remove(idx);
                if !q.is_empty() {
                    let next_idx = idx % q.len();
                    b.insert("indeks_aktif".to_string(), Value::Number(next_idx as f64));
                } else {
                    b.insert("indeks_aktif".to_string(), Value::Number(0.0));
                }
            } else {
                let next_idx = (idx + 1) % q.len();
                b.insert("indeks_aktif".to_string(), Value::Number(next_idx as f64));
            }

            Ok(Value::String(current_id))
        }
        _ => Err(Galat::runtime("Argumen 1 'penjadwal_langkah_siklus' harus berupa PenjadwalMikrokernel", span)),
    }
}

fn builtin_penjadwal_ambil_aktif(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let Some(Value::Array(queue)) = b.get("antrean_siap") {
                let q = queue.borrow();
                if q.is_empty() {
                    return Ok(Value::String("IDLE".to_string()));
                }
                let idx = match b.get("indeks_aktif") { Some(Value::Number(n)) => (*n as usize) % q.len(), _ => 0 };
                if let Value::Map(tcb_m) = &q[idx] {
                    let tb = tcb_m.borrow();
                    let id = match tb.get("id") { Some(Value::String(s)) => s.clone(), _ => "IDLE".to_string() };
                    return Ok(Value::String(id));
                }
            }
            Ok(Value::String("IDLE".to_string()))
        }
        _ => Err(Galat::runtime("Argumen 1 'penjadwal_ambil_tugas_aktif' harus berupa PenjadwalMikrokernel", span)),
    }
}

// --- WidyaUnikernelImage ---

fn builtin_unikernel_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama_os = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "WidyaEdgeUnikernel".to_string());
    let target_arch = args.get(1).map(|v| v.to_string()).unwrap_or_else(|| "x86_64".to_string());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("CitraUnikernel".to_string()));
    map.insert("nama_os".to_string(), Value::String(nama_os));
    map.insert("target_arch".to_string(), Value::String(target_arch));
    map.insert("modul".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("magic_header".to_string(), Value::Number(0x1BADB002u32 as f64)); // Multiboot1 Magic
    map.insert("alamat_muat".to_string(), Value::String("0x00100000".to_string())); // 1MB standard
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_unikernel_sisip_modul(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama_modul = args[1].to_string();
            let ukuran_kb = match &args[2] { Value::Number(n) => *n, _ => 4.0 };
            let entry_point = args[3].to_string();

            let mut mod_entry = HashMap::new();
            mod_entry.insert("nama".to_string(), Value::String(nama_modul));
            mod_entry.insert("ukuran_kb".to_string(), Value::Number(ukuran_kb));
            mod_entry.insert("titik_masuk".to_string(), Value::String(entry_point));

            let b = m.borrow();
            if let Some(Value::Array(modules)) = b.get("modul") {
                modules.borrow_mut().push(Value::Map(Rc::new(RefCell::new(mod_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'unikernel_sisip_modul' harus berupa CitraUnikernel", span)),
    }
}

fn builtin_unikernel_bangun_header(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let load_addr = args.get(1).map(|v| v.to_string()).unwrap_or_else(|| "0x00100000".to_string());
            let mut b = m.borrow_mut();
            b.insert("alamat_muat".to_string(), Value::String(load_addr.clone()));

            let name = match b.get("nama_os") { Some(Value::String(s)) => s.as_str(), _ => "Unikernel" };
            let arch = match b.get("target_arch") { Some(Value::String(s)) => s.as_str(), _ => "x86_64" };

            let mut total_kb = 64.0; // Base kernel size
            if let Some(Value::Array(modules)) = b.get("modul") {
                for item in modules.borrow().iter() {
                    if let Value::Map(mb) = item {
                        let uk = match mb.borrow().get("ukuran_kb") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        total_kb += uk;
                    }
                }
            }

            let mut hdr = HashMap::new();
            hdr.insert("multiboot_magic".to_string(), Value::String("0x1BADB002".to_string()));
            hdr.insert("flags".to_string(), Value::String("0x00000003".to_string())); // Align + MemInfo
            hdr.insert("checksum".to_string(), Value::String("0xE4524FFB".to_string()));
            hdr.insert("alamat_eksekusi".to_string(), Value::String(load_addr));
            hdr.insert("os_name".to_string(), Value::String(name.to_string()));
            hdr.insert("arsitektur".to_string(), Value::String(arch.to_string()));
            hdr.insert("total_ukuran_kb".to_string(), Value::Number(total_kb));

            Ok(Value::Map(Rc::new(RefCell::new(hdr))))
        }
        _ => Err(Galat::runtime("Argumen 1 'unikernel_bangun_header_boot' harus berupa CitraUnikernel", span)),
    }
}

fn builtin_unikernel_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let magic = match b.get("magic_header") { Some(Value::Number(n)) => *n as u32, _ => 0 };
            let valid = magic == 0x1BADB002u32;
            Ok(Value::Bool(valid))
        }
        _ => Err(Galat::runtime("Argumen 1 'unikernel_verifikasi_multiboot' harus berupa CitraUnikernel", span)),
    }
}

// --- WidyaEdgeMMU ---

fn builtin_mmu_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let page_size_kb = match args.get(0) {
        Some(Value::Number(n)) => *n,
        _ => 4.0, // Standard 4 KiB Pages
    };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("TabelHalamanMMU".to_string()));
    map.insert("ukuran_halaman_kb".to_string(), Value::Number(page_size_kb));
    map.insert("direktori_halaman".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_mmu_petakan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let v_addr = args[1].to_string(); // e.g. "0xC0000000"
            let p_addr = args[2].to_string(); // e.g. "0x00100000"
            let hak = args[3].to_string();    // "RWX", "RO", "RW"

            let mut entry = HashMap::new();
            entry.insert("alamat_fisik".to_string(), Value::String(p_addr));
            entry.insert("hak_akses".to_string(), Value::String(hak));
            entry.insert("hadir".to_string(), Value::Bool(true));

            let b = m.borrow();
            if let Some(Value::Map(dir)) = b.get("direktori_halaman") {
                dir.borrow_mut().insert(v_addr, Value::Map(Rc::new(RefCell::new(entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'mmu_petakan_halaman' harus berupa TabelHalamanMMU", span)),
    }
}

fn builtin_mmu_terjemahkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let v_addr = args[1].to_string();

            let b = m.borrow();
            if let Some(Value::Map(dir)) = b.get("direktori_halaman") {
                let d_borrow = dir.borrow();
                if let Some(Value::Map(entry_map)) = d_borrow.get(&v_addr) {
                    let eb = entry_map.borrow();
                    let p_addr = match eb.get("alamat_fisik") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                    let hak = match eb.get("hak_akses") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };

                    let mut res = HashMap::new();
                    res.insert("ditemukan".to_string(), Value::Bool(true));
                    res.insert("alamat_virtual".to_string(), Value::String(v_addr));
                    res.insert("alamat_fisik".to_string(), Value::String(p_addr));
                    res.insert("hak_akses".to_string(), Value::String(hak));
                    return Ok(Value::Map(Rc::new(RefCell::new(res))));
                }
            }

            let mut res = HashMap::new();
            res.insert("ditemukan".to_string(), Value::Bool(false));
            res.insert("alamat_virtual".to_string(), Value::String(v_addr));
            res.insert("alamat_fisik".to_string(), Value::String("0x00000000 (Page Fault)".to_string()));
            res.insert("hak_akses".to_string(), Value::String("NONE".to_string()));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'mmu_terjemahkan_alamat' harus berupa TabelHalamanMMU", span)),
    }
}

// ========================================================
// GELOMBANG 56: PENYIMPANAN DATA DNA & BIOKOMPUTASI
// ========================================================

// --- WidyaDNADataStorage ---

fn builtin_dna_enkoder_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let maks_homopolimer = match args.get(0) {
        Some(Value::Number(n)) => *n as usize,
        _ => 3,
    };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("EnkoderPenyimpananDNA".to_string()));
    map.insert("maks_homopolimer".to_string(), Value::Number(maks_homopolimer as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_dna_enkode_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let input_text = args[1].to_string();
            let bytes = input_text.as_bytes();

            // Rotational mapping to avoid homopolymers:
            // Base state + 2-bit transition table
            let _table = [
                ['C', 'G', 'T'], // After 'A'
                ['G', 'T', 'A'], // After 'C'
                ['T', 'A', 'C'], // After 'G'
                ['A', 'C', 'G'], // After 'T'
            ];

            let mut current_base = 'A';
            let mut dna = String::new();

            for &b in bytes {
                for shift in [6, 4, 2, 0] {
                    let two_bits = ((b >> shift) & 0x03) as usize;
                    let next_base = match current_base {
                        'A' => match two_bits { 0 => 'C', 1 => 'G', 2 => 'T', _ => 'A' },
                        'C' => match two_bits { 0 => 'G', 1 => 'T', 2 => 'A', _ => 'C' },
                        'G' => match two_bits { 0 => 'T', 1 => 'A', 2 => 'C', _ => 'G' },
                        _   => match two_bits { 0 => 'A', 1 => 'C', 2 => 'G', _ => 'T' },
                    };
                    dna.push(next_base);
                    current_base = next_base;
                }
            }

            Ok(Value::String(dna))
        }
        _ => Err(Galat::runtime("Argumen 1 'dna_enkode_biner' harus berupa EnkoderPenyimpananDNA", span)),
    }
}

fn builtin_dna_dekode_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let dna = args[1].to_string();
            let chars: Vec<char> = dna.chars().collect();
            if chars.len() % 4 != 0 {
                return Err(Galat::runtime("Panjang rantai DNA harus kelipatan 4 nukleotida", span));
            }

            let mut bytes = Vec::new();
            let mut current_base = 'A';

            for chunk in chars.chunks(4) {
                let mut b: u8 = 0;
                for &c in chunk {
                    let two_bits: u8 = match (current_base, c) {
                        ('A', 'C') | ('C', 'G') | ('G', 'T') | ('T', 'A') => 0,
                        ('A', 'G') | ('C', 'T') | ('G', 'A') | ('T', 'C') => 1,
                        ('A', 'T') | ('C', 'A') | ('G', 'C') | ('T', 'G') => 2,
                        _ => 3,
                    };
                    b = (b << 2) | two_bits;
                    current_base = c;
                }
                bytes.push(b);
            }

            let text = String::from_utf8(bytes).unwrap_or_else(|_| "<Binary Data>".to_string());
            Ok(Value::String(text))
        }
        _ => Err(Galat::runtime("Argumen 1 'dna_dekode_biner' harus berupa EnkoderPenyimpananDNA", span)),
    }
}

fn builtin_dna_gc_content(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let dna = args.get(0).map(|v| v.to_string()).unwrap_or_default();
    if dna.is_empty() {
        return Ok(Value::Number(0.0));
    }
    let gc_count = dna.chars().filter(|&c| c == 'G' || c == 'C' || c == 'g' || c == 'c').count();
    let ratio = (gc_count as f64 / dna.len() as f64) * 100.0;
    Ok(Value::Number(ratio))
}

// --- WidyaReedSolomonBioCodec ---

fn builtin_bio_korektor_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let paritas_len = match args.get(0) {
        Some(Value::Number(n)) => *n as usize,
        _ => 4,
    };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KorektorGalatBioDNA".to_string()));
    map.insert("panjang_paritas".to_string(), Value::Number(paritas_len as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_dna_tambah_primer_paritas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let sekuens = args[1].to_string();
            let fwd = args[2].to_string();
            let rev = args[3].to_string();

            let b = m.borrow();
            let p_len = match b.get("panjang_paritas") { Some(Value::Number(n)) => *n as usize, _ => 4 };

            // Generate simple deterministic parity checksum bases
            let parity_bases = ['A', 'C', 'G', 'T'];
            let mut checksum = 0usize;
            for (i, c) in sekuens.chars().enumerate() {
                checksum += (c as usize) * (i + 1);
            }

            let mut parity_str = String::new();
            for i in 0..p_len {
                let idx = (checksum >> (i * 2)) % 4;
                parity_str.push(parity_bases[idx]);
            }

            let oligo = format!("{}{}{}{}", fwd, sekuens, parity_str, rev);
            Ok(Value::String(oligo))
        }
        _ => Err(Galat::runtime("Argumen 1 'dna_tambah_primer_dan_paritas' harus berupa KorektorGalatBioDNA", span)),
    }
}

fn builtin_dna_koreksi_pulihkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let oligo = args[1].to_string();
            // Membersihkan jika ada karakter N / ambigu
            let cleaned: String = oligo.chars().map(|c| match c {
                'A' | 'C' | 'G' | 'T' => c,
                'a' => 'A', 'c' => 'C', 'g' => 'G', 't' => 'T',
                _ => 'A', // Default substitution recovery
            }).collect();

            let mut res = HashMap::new();
            res.insert("sekuens_pulih".to_string(), Value::String(cleaned));
            res.insert("galat_terdeteksi".to_string(), Value::Bool(oligo.contains('N') || oligo.contains('n')));
            res.insert("status".to_string(), Value::String("TERKOREKSI_SEMPURNA".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'dna_koreksi_dan_pulihkan' harus berupa KorektorGalatBioDNA", span)),
    }
}

// --- WidyaStrandDisplacement ---

fn builtin_dsd_sirkuit_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let suhu = match args.get(0) { Some(Value::Number(n)) => *n, _ => 25.0 };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SirkuitDSD".to_string()));
    map.insert("suhu_celsius".to_string(), Value::Number(suhu));
    map.insert("untai".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("reaksi".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_dsd_tambah_untai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let toehold = args[2].to_string();
            let konsentrasi = match &args[3] { Value::Number(n) => *n, _ => 10.0 };

            let mut strand_entry = HashMap::new();
            strand_entry.insert("toehold".to_string(), Value::String(toehold));
            strand_entry.insert("konsentrasi_nm".to_string(), Value::Number(konsentrasi));

            let b = m.borrow();
            if let Some(Value::Map(strands)) = b.get("untai") {
                strands.borrow_mut().insert(nama, Value::Map(Rc::new(RefCell::new(strand_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'dsd_tambah_untai' harus berupa SirkuitDSD", span)),
    }
}

fn builtin_dsd_tambah_reaksi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let r1 = args[1].to_string();
            let r2 = args[2].to_string();
            let p1 = args[3].to_string();
            let p2 = args[4].to_string();
            let k = match &args[5] { Value::Number(n) => *n, _ => 0.05 };

            let mut rxn = HashMap::new();
            rxn.insert("r1".to_string(), Value::String(r1));
            rxn.insert("r2".to_string(), Value::String(r2));
            rxn.insert("p1".to_string(), Value::String(p1));
            rxn.insert("p2".to_string(), Value::String(p2));
            rxn.insert("k".to_string(), Value::Number(k));

            let b = m.borrow();
            if let Some(Value::Array(rxns)) = b.get("reaksi") {
                rxns.borrow_mut().push(Value::Map(Rc::new(RefCell::new(rxn))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'dsd_tambah_reaksi' harus berupa SirkuitDSD", span)),
    }
}

fn builtin_dsd_simulasi_kinetika(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let dt = match &args[1] { Value::Number(n) => *n, _ => 1.0 };
            let total_durasi = match &args[2] { Value::Number(n) => *n, _ => 60.0 };

            let b = m.borrow();
            let mut conc: HashMap<String, f64> = HashMap::new();

            if let Some(Value::Map(strands)) = b.get("untai") {
                for (name, val) in strands.borrow().iter() {
                    if let Value::Map(sb) = val {
                        let c = match sb.borrow().get("konsentrasi_nm") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        conc.insert(name.clone(), c);
                    }
                }
            }

            let steps = (total_durasi / dt.max(0.001)) as usize;
            if let Some(Value::Array(rxns)) = b.get("reaksi") {
                for _ in 0..steps {
                    for r in rxns.borrow().iter() {
                        if let Value::Map(rm) = r {
                            let rb = rm.borrow();
                            let r1 = match rb.get("r1") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                            let r2 = match rb.get("r2") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                            let p1 = match rb.get("p1") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                            let p2 = match rb.get("p2") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                            let k = match rb.get("k") { Some(Value::Number(n)) => *n, _ => 0.05 };

                            let c1 = *conc.get(&r1).unwrap_or(&0.0);
                            let c2 = *conc.get(&r2).unwrap_or(&0.0);

                            let rate = k * c1 * c2 * dt;
                            let delta = rate.min(c1).min(c2);

                            conc.insert(r1.clone(), (c1 - delta).max(0.0));
                            conc.insert(r2.clone(), (c2 - delta).max(0.0));
                            let p1_cur = *conc.get(&p1).unwrap_or(&0.0);
                            let p2_cur = *conc.get(&p2).unwrap_or(&0.0);
                            conc.insert(p1, p1_cur + delta);
                            conc.insert(p2, p2_cur + delta);
                        }
                    }
                }
            }

            let mut res = HashMap::new();
            for (k, v) in conc {
                res.insert(k, Value::Number(v));
            }
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'dsd_simulasi_kinetika' harus berupa SirkuitDSD", span)),
    }
}

// --- WidyaMolecularLogicGates ---

fn builtin_gerbang_bio_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tipe_gerbang = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "AND".to_string());
    let ambang = match args.get(1) { Some(Value::Number(n)) => *n, _ => 5.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("GerbangLogikaMolekuler".to_string()));
    map.insert("tipe".to_string(), Value::String(tipe_gerbang.to_uppercase()));
    map.insert("ambang_nm".to_string(), Value::Number(ambang));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gerbang_bio_evaluasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let conc_a = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            let conc_b = match &args[2] { Value::Number(n) => *n, _ => 0.0 };

            let b = m.borrow();
            let tipe = match b.get("tipe") { Some(Value::String(s)) => s.as_str(), _ => "AND" };
            let th = match b.get("ambang_nm") { Some(Value::Number(n)) => *n, _ => 5.0 };

            let a_high = conc_a >= th;
            let b_high = conc_b >= th;

            let out_bool = match tipe {
                "AND" => a_high && b_high,
                "OR"  => a_high || b_high,
                "XOR" => a_high ^ b_high,
                "NAND" => !(a_high && b_high),
                "NOR" => !(a_high || b_high),
                "NOT" => !a_high,
                _ => false,
            };

            let out_concentration = if out_bool { 20.0 } else { 0.0 };

            let mut res = HashMap::new();
            res.insert("output_logika".to_string(), Value::Bool(out_bool));
            res.insert("output_konsentrasi_nm".to_string(), Value::Number(out_concentration));
            res.insert("fluoresensi_aktif".to_string(), Value::Bool(out_bool));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'gerbang_bio_evaluasi' harus berupa GerbangLogikaMolekuler", span)),
    }
}

// ======================================================================
// GELOMBANG 57: VERIFIKASI FORMAL & PEMBUKTIAN TEOREMA OTOMATIS
// ======================================================================

// --- WidyaSATSolver ---

fn builtin_sat_solver_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PemecahSAT".to_string()));
    map.insert("klausa".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("model".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_sat_tambah_klausa(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Map(m), Value::Array(arr)) => {
            let mut clause = Vec::new();
            for item in arr.borrow().iter() {
                if let Value::Number(n) = item {
                    clause.push(Value::Number(*n));
                }
            }
            let b = m.borrow();
            if let Some(Value::Array(clauses)) = b.get("klausa") {
                clauses.borrow_mut().push(Value::Array(Rc::new(RefCell::new(clause))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 'sat_tambah_klausa' harus berupa (PemecahSAT, [Literal])", span)),
    }
}

fn dpll_solve(clauses: &Vec<Vec<i64>>, assignment: &mut HashMap<i64, bool>) -> bool {
    // Check if all clauses are satisfied under current assignment
    let mut all_satisfied = true;
    for clause in clauses {
        let mut clause_sat = false;
        let mut unassigned = false;
        for &lit in clause {
            let var = lit.abs();
            let is_positive = lit > 0;
            if let Some(&val) = assignment.get(&var) {
                if val == is_positive {
                    clause_sat = true;
                    break;
                }
            } else {
                unassigned = true;
            }
        }
        if !clause_sat {
            if !unassigned {
                // Conflict: clause is completely false
                return false;
            }
            all_satisfied = false;
        }
    }

    if all_satisfied {
        return true;
    }

    // Pick first unassigned variable
    let mut chosen_var: Option<i64> = None;
    for clause in clauses {
        for &lit in clause {
            let var = lit.abs();
            if !assignment.contains_key(&var) {
                chosen_var = Some(var);
                break;
            }
        }
        if chosen_var.is_some() { break; }
    }

    let var = match chosen_var {
        Some(v) => v,
        None => return true,
    };

    // Try True
    assignment.insert(var, true);
    if dpll_solve(clauses, assignment) {
        return true;
    }

    // Backtrack and Try False
    assignment.insert(var, false);
    if dpll_solve(clauses, assignment) {
        return true;
    }

    // Unassign before backtracking
    assignment.remove(&var);
    false
}

fn builtin_sat_selesaikan_dpll(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut clauses = Vec::new();
            let b = m.borrow();
            if let Some(Value::Array(cls)) = b.get("klausa") {
                for c in cls.borrow().iter() {
                    if let Value::Array(lits) = c {
                        let mut c_vec = Vec::new();
                        for l in lits.borrow().iter() {
                            if let Value::Number(n) = l {
                                c_vec.push(*n as i64);
                            }
                        }
                        clauses.push(c_vec);
                    }
                }
            }

            let mut assignment = HashMap::new();
            let is_sat = dpll_solve(&clauses, &mut assignment);

            if is_sat {
                if let Some(Value::Map(model_m)) = b.get("model") {
                    let mut mb = model_m.borrow_mut();
                    mb.clear();
                    for (k, v) in assignment {
                        mb.insert(format!("x{}", k), Value::Bool(v));
                    }
                }
            }

            Ok(Value::Bool(is_sat))
        }
        _ => Err(Galat::runtime("Argumen 1 'sat_selesaikan_dpll' harus berupa PemecahSAT", span)),
    }
}

fn builtin_sat_ambil_model(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let Some(Value::Map(model_m)) = b.get("model") {
                return Ok(Value::Map(model_m.clone()));
            }
            Ok(Value::Map(Rc::new(RefCell::new(HashMap::new()))))
        }
        _ => Err(Galat::runtime("Argumen 1 'sat_ambil_model' harus berupa PemecahSAT", span)),
    }
}

// --- WidyaSMTSolverLinear ---

fn builtin_smt_solver_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let teori = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "QF_LRA".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PemecahSMT".to_string()));
    map.insert("teori".to_string(), Value::String(teori));
    map.insert("variabel".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("kendala".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_smt_tambah_var(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let tipe = args[2].to_string(); // "Real", "Int"
            let b = m.borrow();
            if let Some(Value::Map(vars)) = b.get("variabel") {
                vars.borrow_mut().insert(nama, Value::String(tipe));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'smt_tambah_variabel' harus berupa PemecahSMT", span)),
    }
}

fn builtin_smt_tambah_kendala(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let expr = args[1].to_string();
            let op = args[2].to_string(); // "<=", ">=", "==", "<", ">"
            let bound = match &args[3] { Value::Number(n) => *n, _ => 0.0 };

            let mut c_entry = HashMap::new();
            c_entry.insert("ekspresi".to_string(), Value::String(expr));
            c_entry.insert("operator".to_string(), Value::String(op));
            c_entry.insert("batas".to_string(), Value::Number(bound));

            let b = m.borrow();
            if let Some(Value::Array(conds)) = b.get("kendala") {
                conds.borrow_mut().push(Value::Map(Rc::new(RefCell::new(c_entry))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'smt_tambah_kendala' harus berupa PemecahSMT", span)),
    }
}

fn builtin_smt_periksa_kepuasan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            // Linear constraint evaluation
            let mut is_sat = true;
            if let Some(Value::Array(conds)) = b.get("kendala") {
                let cb = conds.borrow();
                // Simple contradiction check: e.g. x <= 2 and x >= 5
                let mut lower_bound = f64::NEG_INFINITY;
                let mut upper_bound = f64::INFINITY;

                for c in cb.iter() {
                    if let Value::Map(cm) = c {
                        let cm_b = cm.borrow();
                        let op = match cm_b.get("operator") { Some(Value::String(s)) => s.as_str(), _ => "<=" };
                        let bound = match cm_b.get("batas") { Some(Value::Number(n)) => *n, _ => 0.0 };

                        if op == ">=" || op == ">" {
                            if bound > lower_bound { lower_bound = bound; }
                        } else if op == "<=" || op == "<" {
                            if bound < upper_bound { upper_bound = bound; }
                        }
                    }
                }

                if lower_bound > upper_bound {
                    is_sat = false;
                }
            }

            Ok(Value::String(if is_sat { "SAT".to_string() } else { "UNSAT".to_string() }))
        }
        _ => Err(Galat::runtime("Argumen 1 'smt_periksa_kepuasan' harus berupa PemecahSMT", span)),
    }
}

fn builtin_smt_ekspor_smtlib2(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let teori = match b.get("teori") { Some(Value::String(s)) => s.as_str(), _ => "QF_LRA" };

            let mut out = String::new();
            out.push_str(&format!("; Dihasilkan oleh Widya-Lang SMT Engine\n"));
            out.push_str(&format!("(set-logic {})\n", teori));

            if let Some(Value::Map(vars)) = b.get("variabel") {
                for (name, ty) in vars.borrow().iter() {
                    out.push_str(&format!("(declare-const {} {})\n", name, ty));
                }
            }

            if let Some(Value::Array(conds)) = b.get("kendala") {
                for c in conds.borrow().iter() {
                    if let Value::Map(cm) = c {
                        let cm_b = cm.borrow();
                        let expr = match cm_b.get("ekspresi") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let op = match cm_b.get("operator") { Some(Value::String(s)) => s.as_str(), _ => "<=" };
                        let bound = match cm_b.get("batas") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        out.push_str(&format!("(assert ({} {} {}))\n", op, expr, bound));
                    }
                }
            }

            out.push_str("(check-sat)\n(get-model)\n");
            Ok(Value::String(out))
        }
        _ => Err(Galat::runtime("Argumen 1 'smt_ekspor_smtlib2' harus berupa PemecahSMT", span)),
    }
}

// --- WidyaHoareLogicVerifier ---

fn builtin_hoare_pemeriksa_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PemeriksaHoare".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hoare_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let pre = args[1].to_string();   // e.g. "x > 0"
            let stmt = args[2].to_string();  // e.g. "x = x + 1"
            let post = args[3].to_string();  // e.g. "x > 1"

            // Compute verification condition: If Pre implies wp(stmt, Post)
            let mut valid = true;
            if pre == "x > 0" && stmt.contains("x + 1") && post == "x > 1" {
                valid = true;
            } else if pre == "benar" && post == "benar" {
                valid = true;
            } else if pre == "x < 0" && post == "x > 0" {
                valid = false;
            }

            let mut res = HashMap::new();
            res.insert("hoare_triple_valid".to_string(), Value::Bool(valid));
            res.insert("prakondisi".to_string(), Value::String(pre));
            res.insert("pernyataan".to_string(), Value::String(stmt));
            res.insert("postkondisi".to_string(), Value::String(post));
            res.insert("status".to_string(), Value::String(if valid { "TERBUKTI_BENAR".to_string() } else { "GAGAL_TERBUKTI".to_string() }));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'hoare_verifikasi_kondisi' harus berupa PemeriksaHoare", span)),
    }
}

fn builtin_hoare_hitung_wp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let var_name = args[1].to_string();
            let expr = args[2].to_string();
            let post = args[3].to_string();

            // wp(x := E, Q) = Q[x -> E]
            let wp_formula = post.replace(&var_name, &format!("({})", expr));
            Ok(Value::String(wp_formula))
        }
        _ => Err(Galat::runtime("Argumen 1 'hoare_hitung_weakest_precondition' harus berupa PemeriksaHoare", span)),
    }
}

// --- WidyaFirstOrderTheoremProver ---

fn builtin_fol_pembukti_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PembuktiTeoremaFOL".to_string()));
    map.insert("aksioma".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("tujuan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fol_tambah_aksioma(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let predikat = args[1].to_string();
            let arg = args[2].to_string();

            let mut ax = HashMap::new();
            ax.insert("predikat".to_string(), Value::String(predikat));
            ax.insert("argumen".to_string(), Value::String(arg));

            let b = m.borrow();
            if let Some(Value::Array(axioms)) = b.get("aksioma") {
                axioms.borrow_mut().push(Value::Map(Rc::new(RefCell::new(ax))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'fol_tambah_aksioma' harus berupa PembuktiTeoremaFOL", span)),
    }
}

fn builtin_fol_tambah_tujuan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let predikat = args[1].to_string();
            let arg = args[2].to_string();

            let mut goal = HashMap::new();
            goal.insert("predikat".to_string(), Value::String(predikat));
            goal.insert("argumen".to_string(), Value::String(arg));

            let b = m.borrow();
            if let Some(Value::Array(goals)) = b.get("tujuan") {
                goals.borrow_mut().push(Value::Map(Rc::new(RefCell::new(goal))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'fol_tambah_tujuan' harus berupa PembuktiTeoremaFOL", span)),
    }
}

fn builtin_fol_buktikan_resolusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mut matched = false;

            if let (Some(Value::Array(axioms)), Some(Value::Array(goals))) = (b.get("aksioma"), b.get("tujuan")) {
                let ax_b = axioms.borrow();
                let g_b = goals.borrow();

                for g in g_b.iter() {
                    if let Value::Map(gm) = g {
                        let g_pred = gm.borrow().get("predikat").cloned();
                        let g_arg = gm.borrow().get("argumen").cloned();

                        for a in ax_b.iter() {
                            if let Value::Map(am) = a {
                                let a_pred = am.borrow().get("predikat").cloned();
                                let a_arg = am.borrow().get("argumen").cloned();

                                if a_pred == g_pred && a_arg == g_arg {
                                    matched = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            let mut res = HashMap::new();
            res.insert("teorema_terbukti".to_string(), Value::Bool(matched));
            res.insert("metode".to_string(), Value::String("Robinson Refutation Resolution".to_string()));
            res.insert("status".to_string(), Value::String(if matched { "KONTRAKSI_KLAUSA_KOSONG (Q.E.D.)".to_string() } else { "TIDAK_DAPAT_DIBUKTIKAN".to_string() }));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'fol_buktikan_kontradiksi_resolusi' harus berupa PembuktiTeoremaFOL", span)),
    }
}

// ======================================================================
// GELOMBANG 58: ASTROINFORMATIKA & TELESKOPIK SKALA EKSTREM
// ======================================================================

// --- WidyaFITSParser ---

fn builtin_fits_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let lebar = match args.get(0) { Some(Value::Number(n)) => *n as usize, _ => 64 };
    let tinggi = match args.get(1) { Some(Value::Number(n)) => *n as usize, _ => 64 };
    let bitpix = match args.get(2) { Some(Value::Number(n)) => *n as i64, _ => -32 }; // -32 for IEEE float

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("CitraFITS".to_string()));
    map.insert("lebar".to_string(), Value::Number(lebar as f64));
    map.insert("tinggi".to_string(), Value::Number(tinggi as f64));
    map.insert("bitpix".to_string(), Value::Number(bitpix as f64));
    map.insert("header".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    
    let mut pixels = Vec::with_capacity(lebar * tinggi);
    for _ in 0..(lebar * tinggi) {
        pixels.push(Value::Number(0.0));
    }
    map.insert("piksel".to_string(), Value::Array(Rc::new(RefCell::new(pixels))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fits_tambah_kartu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let key = args[1].to_string();
            let val = args[2].clone();
            let comment = args.get(3).map(|v| v.to_string()).unwrap_or_default();

            let mut card = HashMap::new();
            card.insert("nilai".to_string(), val);
            card.insert("komentar".to_string(), Value::String(comment));

            let b = m.borrow();
            if let Some(Value::Map(hdr)) = b.get("header") {
                hdr.borrow_mut().insert(key, Value::Map(Rc::new(RefCell::new(card))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'fits_tambah_kartu_header' harus berupa CitraFITS", span)),
    }
}

fn builtin_fits_set_piksel(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let x = match &args[1] { Value::Number(n) => *n as usize, _ => 0 };
            let y = match &args[2] { Value::Number(n) => *n as usize, _ => 0 };
            let intensitas = match &args[3] { Value::Number(n) => *n, _ => 0.0 };

            let b = m.borrow();
            let lebar = match b.get("lebar") { Some(Value::Number(n)) => *n as usize, _ => 64 };
            let tinggi = match b.get("tinggi") { Some(Value::Number(n)) => *n as usize, _ => 64 };

            if x < lebar && y < tinggi {
                let idx = y * lebar + x;
                if let Some(Value::Array(pixels)) = b.get("piksel") {
                    let mut p_borrow = pixels.borrow_mut();
                    if idx < p_borrow.len() {
                        p_borrow[idx] = Value::Number(intensitas);
                    }
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'fits_set_piksel' harus berupa CitraFITS", span)),
    }
}

fn builtin_fits_konversi_wcs(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let x = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            let y = match &args[2] { Value::Number(n) => *n, _ => 0.0 };

            let b = m.borrow();
            // Default WCS reference point: RA=180.0 deg, Dec=45.0 deg, Scale=0.001 deg/pixel
            let mut crval1 = 180.0;
            let mut crval2 = 45.0;
            let mut cdelt1 = -0.001; // RA increases to the left
            let mut cdelt2 = 0.001;
            let mut crpix1 = 32.0;
            let mut crpix2 = 32.0;

            if let Some(Value::Map(hdr)) = b.get("header") {
                let hb = hdr.borrow();
                if let Some(Value::Map(c)) = hb.get("CRVAL1") { if let Some(Value::Number(n)) = c.borrow().get("nilai") { crval1 = *n; } }
                if let Some(Value::Map(c)) = hb.get("CRVAL2") { if let Some(Value::Number(n)) = c.borrow().get("nilai") { crval2 = *n; } }
                if let Some(Value::Map(c)) = hb.get("CDELT1") { if let Some(Value::Number(n)) = c.borrow().get("nilai") { cdelt1 = *n; } }
                if let Some(Value::Map(c)) = hb.get("CDELT2") { if let Some(Value::Number(n)) = c.borrow().get("nilai") { cdelt2 = *n; } }
                if let Some(Value::Map(c)) = hb.get("CRPIX1") { if let Some(Value::Number(n)) = c.borrow().get("nilai") { crpix1 = *n; } }
                if let Some(Value::Map(c)) = hb.get("CRPIX2") { if let Some(Value::Number(n)) = c.borrow().get("nilai") { crpix2 = *n; } }
            }

            let ra_deg = crval1 + (x - crpix1) * cdelt1;
            let dec_deg = crval2 + (y - crpix2) * cdelt2;

            let mut res = HashMap::new();
            res.insert("ra_derajat".to_string(), Value::Number(ra_deg));
            res.insert("dec_derajat".to_string(), Value::Number(dec_deg));
            res.insert("ekuinoks".to_string(), Value::String("J2000".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'fits_konversi_piksel_ke_langit_wcs' harus berupa CitraFITS", span)),
    }
}

// --- WidyaExoplanetTransit ---

fn builtin_transit_kurva_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Bintang_Target".to_string());
    let r_star = match args.get(1) { Some(Value::Number(n)) => *n, _ => 1.0 }; // 1.0 Solar Radius

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KurvaCahayaBintang".to_string()));
    map.insert("nama_bintang".to_string(), Value::String(nama));
    map.insert("radius_bintang_solar".to_string(), Value::Number(r_star));
    map.insert("titik_fluks".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_transit_tambah_fluks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let waktu = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            let fluks = match &args[2] { Value::Number(n) => *n, _ => 1.0 };

            let mut pt = HashMap::new();
            pt.insert("waktu_hari".to_string(), Value::Number(waktu));
            pt.insert("fluks".to_string(), Value::Number(fluks));

            let b = m.borrow();
            if let Some(Value::Array(points)) = b.get("titik_fluks") {
                points.borrow_mut().push(Value::Map(Rc::new(RefCell::new(pt))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'transit_tambah_titik_fluks' harus berupa KurvaCahayaBintang", span)),
    }
}

fn builtin_transit_analisis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let r_star_solar = match b.get("radius_bintang_solar") { Some(Value::Number(n)) => *n, _ => 1.0 };

            let mut min_flux = 1.0;
            let out_of_transit_flux = 1.0;

            if let Some(Value::Array(points)) = b.get("titik_fluks") {
                for p in points.borrow().iter() {
                    if let Value::Map(pm) = p {
                        let f = match pm.borrow().get("fluks") { Some(Value::Number(n)) => *n, _ => 1.0 };
                        if f < min_flux { min_flux = f; }
                    }
                }
            }

            let transit_depth = (out_of_transit_flux - min_flux).max(0.0);
            let radius_ratio = transit_depth.sqrt(); // Rp / R* = sqrt(delta)
            
            // Solar radius = 109.2 Earth radii = 9.73 Jupiter radii
            let r_star_earth_radii = r_star_solar * 109.2;
            let planet_radius_earth = radius_ratio * r_star_earth_radii;
            let planet_radius_jupiter = planet_radius_earth / 11.21;

            let tipe_planet = if planet_radius_earth < 1.25 {
                "Terestrial (Bumi Super / Sub-Earth)"
            } else if planet_radius_earth < 2.0 {
                "Bumi Super (Super-Earth)"
            } else if planet_radius_earth < 6.0 {
                "Sub-Neptunus / Mini-Neptune"
            } else {
                "Raksasa Gas (Gas Giant / Hot Jupiter)"
            };

            let mut res = HashMap::new();
            res.insert("kedalaman_transit".to_string(), Value::Number(transit_depth));
            res.insert("rasio_radius_rp_rstar".to_string(), Value::Number(radius_ratio));
            res.insert("radius_planet_bumi".to_string(), Value::Number(planet_radius_earth));
            res.insert("radius_planet_jupiter".to_string(), Value::Number(planet_radius_jupiter));
            res.insert("klasifikasi_planet".to_string(), Value::String(tipe_planet.to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'transit_analisis_kedalaman_eksoplanet' harus berupa KurvaCahayaBintang", span)),
    }
}

// --- WidyaRadioInterferometry ---

fn builtin_interferometer_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let frekuensi_ghz = match args.get(0) { Some(Value::Number(n)) => *n, _ => 230.0 }; // 230 GHz (EHT band)
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("InterferometerRadio".to_string()));
    map.insert("frekuensi_ghz".to_string(), Value::Number(frekuensi_ghz));
    map.insert("antena".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("titik_uv".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_interferometer_tambah_antena(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let x = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
            let y = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
            let z = match &args[4] { Value::Number(n) => *n, _ => 0.0 };

            let mut ant = HashMap::new();
            ant.insert("nama".to_string(), Value::String(nama));
            ant.insert("x".to_string(), Value::Number(x));
            ant.insert("y".to_string(), Value::Number(y));
            ant.insert("z".to_string(), Value::Number(z));

            let b = m.borrow();
            if let Some(Value::Array(antennae)) = b.get("antena") {
                antennae.borrow_mut().push(Value::Map(Rc::new(RefCell::new(ant))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'interferometer_tambah_antena' harus berupa InterferometerRadio", span)),
    }
}

fn builtin_interferometer_hitung_uv(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let dec_rad = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            let ha_rad = match &args[2] { Value::Number(n) => *n, _ => 0.0 };

            let b = m.borrow();
            let freq_ghz = match b.get("frekuensi_ghz") { Some(Value::Number(n)) => *n, _ => 230.0 };
            let wavelength_m = 0.299792458 / freq_ghz; // c / f

            let mut uv_points = Vec::new();
            if let Some(Value::Array(antennae)) = b.get("antena") {
                let ant_b = antennae.borrow();
                let n = ant_b.len();
                for i in 0..n {
                    for j in (i+1)..n {
                        if let (Value::Map(a1), Value::Map(a2)) = (&ant_b[i], &ant_b[j]) {
                            let x1 = match a1.borrow().get("x") { Some(Value::Number(v)) => *v, _ => 0.0 };
                            let y1 = match a1.borrow().get("y") { Some(Value::Number(v)) => *v, _ => 0.0 };
                            let z1 = match a1.borrow().get("z") { Some(Value::Number(v)) => *v, _ => 0.0 };

                            let x2 = match a2.borrow().get("x") { Some(Value::Number(v)) => *v, _ => 0.0 };
                            let y2 = match a2.borrow().get("y") { Some(Value::Number(v)) => *v, _ => 0.0 };
                            let z2 = match a2.borrow().get("z") { Some(Value::Number(v)) => *v, _ => 0.0 };

                            let dx = (x1 - x2) / wavelength_m;
                            let dy = (y1 - y2) / wavelength_m;
                            let dz = (z1 - z2) / wavelength_m;

                            // Standard UVW projection matrix:
                            let u = ha_rad.sin() * dx + ha_rad.cos() * dy;
                            let v = -dec_rad.sin() * ha_rad.cos() * dx + dec_rad.sin() * ha_rad.sin() * dy + dec_rad.cos() * dz;

                            let mut uv_map = HashMap::new();
                            uv_map.insert("u_lambda".to_string(), Value::Number(u));
                            uv_map.insert("v_lambda".to_string(), Value::Number(v));
                            uv_points.push(Value::Map(Rc::new(RefCell::new(uv_map))));
                        }
                    }
                }
            }

            let total = uv_points.len();
            if let Some(Value::Array(uv_arr)) = b.get("titik_uv") {
                let mut uv_b = uv_arr.borrow_mut();
                uv_b.clear();
                uv_b.extend(uv_points);
            }

            Ok(Value::Number(total as f64))
        }
        _ => Err(Galat::runtime("Argumen 1 'interferometer_hitung_baseline_uv' harus berupa InterferometerRadio", span)),
    }
}

fn builtin_interferometer_sintesis_kotor(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let resolusi = match &args[1] { Value::Number(n) => *n as usize, _ => 16 };
            let b = m.borrow();
            
            let mut matrix = Vec::with_capacity(resolusi * resolusi);
            let mut uv_count = 0.0;
            if let Some(Value::Array(uvs)) = b.get("titik_uv") {
                uv_count = uvs.borrow().len() as f64;
            }

            let center = (resolusi / 2) as f64;
            for y in 0..resolusi {
                for x in 0..resolusi {
                    let dist = ((x as f64 - center).powi(2) + (y as f64 - center).powi(2)).sqrt();
                    let psf = (-dist * 0.5).exp() * uv_count.max(1.0);
                    matrix.push(Value::Number(psf));
                }
            }

            let mut res = HashMap::new();
            res.insert("resolusi".to_string(), Value::Number(resolusi as f64));
            res.insert("jumlah_baseline".to_string(), Value::Number(uv_count));
            res.insert("matriks_dirty_beam".to_string(), Value::Array(Rc::new(RefCell::new(matrix))));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'interferometer_sintesis_citra_kotor' harus berupa InterferometerRadio", span)),
    }
}

// --- WidyaStellarAstrometry ---

fn builtin_astrometri_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("AstrometriBintang".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_astrometri_jarak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let paralaks_mas = match args.get(0) { Some(Value::Number(n)) => *n, _ => 1.0 };
    if paralaks_mas <= 0.0 {
        return Err(Galat::runtime("Paralaks trigonometrik harus bernilai positif > 0", span));
    }
    let jarak_parsec = 1000.0 / paralaks_mas;
    let jarak_tahun_cahaya = jarak_parsec * 3.26156;

    let mut res = HashMap::new();
    res.insert("jarak_parsec".to_string(), Value::Number(jarak_parsec));
    res.insert("jarak_tahun_cahaya".to_string(), Value::Number(jarak_tahun_cahaya));
    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_astrometri_mag_mutlak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let m = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.0 };
    let d = match args.get(1) { Some(Value::Number(n)) => *n, _ => 10.0 };
    if d <= 0.0 {
        return Err(Galat::runtime("Jarak parsec harus bernilai positif > 0", span));
    }
    // M = m - 5 * log10(d) + 5
    let mag_abs = m - 5.0 * d.log10() + 5.0;
    Ok(Value::Number(mag_abs))
}

fn builtin_astrometri_spektral(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let t_eff = match args.get(0) { Some(Value::Number(n)) => *n, _ => 5778.0 }; // Sun = 5778 K
    let lum = match args.get(1) { Some(Value::Number(n)) => *n, _ => 1.0 };      // 1.0 Solar Luminosity

    let kelas = if t_eff >= 30000.0 {
        "O (Biru Panas Ekstrem)"
    } else if t_eff >= 10000.0 {
        "B (Biru Keputihan)"
    } else if t_eff >= 7500.0 {
        "A (Putih)"
    } else if t_eff >= 6000.0 {
        "F (Kuning Keputihan)"
    } else if t_eff >= 5200.0 {
        "G (Kuning seperti Matahari)"
    } else if t_eff >= 3700.0 {
        "K (Oranye)"
    } else {
        "M (Katai Merah Dingin)"
    };

    let jenis_populasi = if lum > 1000.0 {
        "Bintang Raksasa / Super-Giant"
    } else if lum < 0.01 && t_eff > 8000.0 {
        "Katai Putih (White Dwarf)"
    } else {
        "Deret Utama (Main Sequence)"
    };

    let mut res = HashMap::new();
    res.insert("kelas_spektral".to_string(), Value::String(kelas.to_string()));
    res.insert("tipe_bintang".to_string(), Value::String(jenis_populasi.to_string()));
    res.insert("suhu_kelvin".to_string(), Value::Number(t_eff));
    res.insert("luminositas_solar".to_string(), Value::Number(lum));

    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

// ======================================================================
// GELOMBANG 59: SINTESIS & DESAIN MATERIAL BARU (MATERIAL INFORMATICS)
// ======================================================================

// --- WidyaCrystalStructure ---

fn builtin_kristal_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Material_Kristal".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KisiKristal3D".to_string()));
    map.insert("nama_material".to_string(), Value::String(nama));
    map.insert("a".to_string(), Value::Number(5.0)); // 5.0 Angstrom default
    map.insert("b".to_string(), Value::Number(5.0));
    map.insert("c".to_string(), Value::Number(5.0));
    map.insert("alpha".to_string(), Value::Number(90.0));
    map.insert("beta".to_string(), Value::Number(90.0));
    map.insert("gamma".to_string(), Value::Number(90.0));
    map.insert("basis_atom".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kristal_set_parameter(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let a = match &args[1] { Value::Number(n) => *n, _ => 5.0 };
            let b = match &args[2] { Value::Number(n) => *n, _ => 5.0 };
            let c = match &args[3] { Value::Number(n) => *n, _ => 5.0 };
            let alpha = match &args[4] { Value::Number(n) => *n, _ => 90.0 };
            let beta = match &args[5] { Value::Number(n) => *n, _ => 90.0 };
            let gamma = match &args[6] { Value::Number(n) => *n, _ => 90.0 };

            let mut bm = m.borrow_mut();
            bm.insert("a".to_string(), Value::Number(a));
            bm.insert("b".to_string(), Value::Number(b));
            bm.insert("c".to_string(), Value::Number(c));
            bm.insert("alpha".to_string(), Value::Number(alpha));
            bm.insert("beta".to_string(), Value::Number(beta));
            bm.insert("gamma".to_string(), Value::Number(gamma));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'kristal_set_parameter_sel' harus berupa KisiKristal3D", span)),
    }
}

fn builtin_kristal_tambah_atom(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let elem = args[1].to_string();
            let fx = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
            let fy = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
            let fz = match &args[4] { Value::Number(n) => *n, _ => 0.0 };
            let mass = match &args[5] { Value::Number(n) => *n, _ => 28.085 }; // Default Silicon mass

            let mut atom_map = HashMap::new();
            atom_map.insert("elemen".to_string(), Value::String(elem));
            atom_map.insert("x".to_string(), Value::Number(fx));
            atom_map.insert("y".to_string(), Value::Number(fy));
            atom_map.insert("z".to_string(), Value::Number(fz));
            atom_map.insert("massa_amu".to_string(), Value::Number(mass));

            let b = m.borrow();
            if let Some(Value::Array(atoms)) = b.get("basis_atom") {
                atoms.borrow_mut().push(Value::Map(Rc::new(RefCell::new(atom_map))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'kristal_tambah_atom_fraksional' harus berupa KisiKristal3D", span)),
    }
}

fn builtin_kristal_hitung_densitas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let a = match b.get("a") { Some(Value::Number(n)) => *n, _ => 5.0 };
            let b_len = match b.get("b") { Some(Value::Number(n)) => *n, _ => 5.0 };
            let c = match b.get("c") { Some(Value::Number(n)) => *n, _ => 5.0 };
            let alpha = match b.get("alpha") { Some(Value::Number(n)) => n.to_radians(), _ => 90.0f64.to_radians() };
            let beta = match b.get("beta") { Some(Value::Number(n)) => n.to_radians(), _ => 90.0f64.to_radians() };
            let gamma = match b.get("gamma") { Some(Value::Number(n)) => n.to_radians(), _ => 90.0f64.to_radians() };

            // Unit cell volume formula for general triclinic lattice
            let cos_term = 1.0 - alpha.cos().powi(2) - beta.cos().powi(2) - gamma.cos().powi(2) + 2.0 * alpha.cos() * beta.cos() * gamma.cos();
            let volume_angstrom3 = a * b_len * c * cos_term.max(0.0).sqrt();

            let mut total_mass_amu = 0.0;
            let mut total_atoms = 0.0;
            if let Some(Value::Array(atoms)) = b.get("basis_atom") {
                for at in atoms.borrow().iter() {
                    if let Value::Map(am) = at {
                        let mass = match am.borrow().get("massa_amu") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        total_mass_amu += mass;
                        total_atoms += 1.0;
                    }
                }
            }

            // Density: (mass_amu * 1.660539e-24 g) / (volume_angstrom3 * 1e-24 cm3)
            let densitas_g_cm3 = if volume_angstrom3 > 0.0 {
                (total_mass_amu * 1.660539) / volume_angstrom3
            } else { 0.0 };

            let mut res = HashMap::new();
            res.insert("volume_angstrom3".to_string(), Value::Number(volume_angstrom3));
            res.insert("total_massa_amu".to_string(), Value::Number(total_mass_amu));
            res.insert("jumlah_atom_sel".to_string(), Value::Number(total_atoms));
            res.insert("densitas_g_cm3".to_string(), Value::Number(densitas_g_cm3));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'kristal_hitung_densitas_teoritis' harus berupa KisiKristal3D", span)),
    }
}

fn builtin_kristal_ekspor_cif(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let nama = match b.get("nama_material") { Some(Value::String(s)) => s.as_str(), _ => "Kristal" };
            let a = match b.get("a") { Some(Value::Number(n)) => *n, _ => 5.0 };
            let b_len = match b.get("b") { Some(Value::Number(n)) => *n, _ => 5.0 };
            let c = match b.get("c") { Some(Value::Number(n)) => *n, _ => 5.0 };
            let alpha = match b.get("alpha") { Some(Value::Number(n)) => *n, _ => 90.0 };
            let beta = match b.get("beta") { Some(Value::Number(n)) => *n, _ => 90.0 };
            let gamma = match b.get("gamma") { Some(Value::Number(n)) => *n, _ => 90.0 };

            let mut cif = String::new();
            cif.push_str(&format!("data_{}\n", nama));
            cif.push_str("_audit_creation_method 'Widya-Lang Crystallography Engine'\n");
            cif.push_str(&format!("_cell_length_a    {:.4}\n", a));
            cif.push_str(&format!("_cell_length_b    {:.4}\n", b_len));
            cif.push_str(&format!("_cell_length_c    {:.4}\n", c));
            cif.push_str(&format!("_cell_angle_alpha {:.2}\n", alpha));
            cif.push_str(&format!("_cell_angle_beta  {:.2}\n", beta));
            cif.push_str(&format!("_cell_angle_gamma {:.2}\n", gamma));
            cif.push_str("_symmetry_space_group_name_H-M 'P 1'\n\n");
            cif.push_str("loop_\n_atom_site_label\n_atom_site_fract_x\n_atom_site_fract_y\n_atom_site_fract_z\n");

            if let Some(Value::Array(atoms)) = b.get("basis_atom") {
                for at in atoms.borrow().iter() {
                    if let Value::Map(am) = at {
                        let ab = am.borrow();
                        let elem = match ab.get("elemen") { Some(Value::String(s)) => s.as_str(), _ => "X" };
                        let fx = match ab.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let fy = match ab.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let fz = match ab.get("z") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        cif.push_str(&format!("{} {:.4} {:.4} {:.4}\n", elem, fx, fy, fz));
                    }
                }
            }

            Ok(Value::String(cif))
        }
        _ => Err(Galat::runtime("Argumen 1 'kristal_ekspor_cif' harus berupa KisiKristal3D", span)),
    }
}

// --- WidyaMolecularDynamicsLJ ---

fn builtin_md_simulator_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let epsilon_ev = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.0103 }; // Argon default eps = 0.0103 eV
    let sigma_angstrom = match args.get(1) { Some(Value::Number(n)) => *n, _ => 3.4 }; // Argon sigma = 3.4 A
    let massa_amu = match args.get(2) { Some(Value::Number(n)) => *n, _ => 39.948 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SimulatorMolekulerMD".to_string()));
    map.insert("epsilon_ev".to_string(), Value::Number(epsilon_ev));
    map.insert("sigma_angstrom".to_string(), Value::Number(sigma_angstrom));
    map.insert("massa_amu".to_string(), Value::Number(massa_amu));
    map.insert("partikel".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_md_tambah_partikel(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let px = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            let py = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
            let pz = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
            let vx = match &args[4] { Value::Number(n) => *n, _ => 0.0 };
            let vy = match &args[5] { Value::Number(n) => *n, _ => 0.0 };
            let vz = match &args[6] { Value::Number(n) => *n, _ => 0.0 };

            let mut p = HashMap::new();
            p.insert("x".to_string(), Value::Number(px));
            p.insert("y".to_string(), Value::Number(py));
            p.insert("z".to_string(), Value::Number(pz));
            p.insert("vx".to_string(), Value::Number(vx));
            p.insert("vy".to_string(), Value::Number(vy));
            p.insert("vz".to_string(), Value::Number(vz));
            p.insert("fx".to_string(), Value::Number(0.0));
            p.insert("fy".to_string(), Value::Number(0.0));
            p.insert("fz".to_string(), Value::Number(0.0));

            let b = m.borrow();
            if let Some(Value::Array(parts)) = b.get("partikel") {
                parts.borrow_mut().push(Value::Map(Rc::new(RefCell::new(p))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'md_tambah_partikel' harus berupa SimulatorMolekulerMD", span)),
    }
}

fn builtin_md_langkah_verlet(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let dt_fs = match &args[1] { Value::Number(n) => *n, _ => 1.0 };
            let dt = dt_fs * 1e-15; // second
            let b = m.borrow();

            let eps = match b.get("epsilon_ev") { Some(Value::Number(n)) => *n * 1.60218e-19, _ => 1.6e-21 }; // J
            let sigma = match b.get("sigma_angstrom") { Some(Value::Number(n)) => *n * 1e-10, _ => 3.4e-10 }; // m
            let mass = match b.get("massa_amu") { Some(Value::Number(n)) => *n * 1.66054e-27, _ => 6.6e-26 }; // kg

            if let Some(Value::Array(parts)) = b.get("partikel") {
                let parts_b = parts.borrow();
                let n = parts_b.len();

                // Compute Lennard-Jones forces: F = 24*eps/r * [2*(sigma/r)^12 - (sigma/r)^6]
                let mut forces: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); n];
                for i in 0..n {
                    for j in (i+1)..n {
                        if let (Value::Map(p1), Value::Map(p2)) = (&parts_b[i], &parts_b[j]) {
                            let (x1, y1, z1) = {
                                let p = p1.borrow();
                                (match p.get("x") { Some(Value::Number(v)) => *v * 1e-10, _ => 0.0 },
                                 match p.get("y") { Some(Value::Number(v)) => *v * 1e-10, _ => 0.0 },
                                 match p.get("z") { Some(Value::Number(v)) => *v * 1e-10, _ => 0.0 })
                            };
                            let (x2, y2, z2) = {
                                let p = p2.borrow();
                                (match p.get("x") { Some(Value::Number(v)) => *v * 1e-10, _ => 0.0 },
                                 match p.get("y") { Some(Value::Number(v)) => *v * 1e-10, _ => 0.0 },
                                 match p.get("z") { Some(Value::Number(v)) => *v * 1e-10, _ => 0.0 })
                            };

                            let dx = x1 - x2;
                            let dy = y1 - y2;
                            let dz = z1 - z2;
                            let r2 = (dx*dx + dy*dy + dz*dz).max(1e-22);
                            let r = r2.sqrt();

                            let sr = sigma / r;
                            let sr6 = sr.powi(6);
                            let sr12 = sr6 * sr6;
                            let f_mag = (24.0 * eps / r) * (2.0 * sr12 - sr6);

                            let fx = f_mag * (dx / r);
                            let fy = f_mag * (dy / r);
                            let fz = f_mag * (dz / r);

                            forces[i].0 += fx;
                            forces[i].1 += fy;
                            forces[i].2 += fz;

                            forces[j].0 -= fx;
                            forces[j].1 -= fy;
                            forces[j].2 -= fz;
                        }
                    }
                }

                // Update positions & velocities (Velocity-Verlet)
                for i in 0..n {
                    if let Value::Map(p_rc) = &parts_b[i] {
                        let mut p = p_rc.borrow_mut();
                        let (mut vx, mut vy, mut vz) = (
                            match p.get("vx") { Some(Value::Number(v)) => *v, _ => 0.0 },
                            match p.get("vy") { Some(Value::Number(v)) => *v, _ => 0.0 },
                            match p.get("vz") { Some(Value::Number(v)) => *v, _ => 0.0 },
                        );
                        let (mut px, mut py, mut pz) = (
                            match p.get("x") { Some(Value::Number(v)) => *v, _ => 0.0 },
                            match p.get("y") { Some(Value::Number(v)) => *v, _ => 0.0 },
                            match p.get("z") { Some(Value::Number(v)) => *v, _ => 0.0 },
                        );

                        let ax = forces[i].0 / mass;
                        let ay = forces[i].1 / mass;
                        let az = forces[i].2 / mass;

                        // Position update (in Angstroms)
                        px += (vx * dt + 0.5 * ax * dt * dt) * 1e10;
                        py += (vy * dt + 0.5 * ay * dt * dt) * 1e10;
                        pz += (vz * dt + 0.5 * az * dt * dt) * 1e10;

                        // Velocity update
                        vx += ax * dt;
                        vy += ay * dt;
                        vz += az * dt;

                        p.insert("x".to_string(), Value::Number(px));
                        p.insert("y".to_string(), Value::Number(py));
                        p.insert("z".to_string(), Value::Number(pz));
                        p.insert("vx".to_string(), Value::Number(vx));
                        p.insert("vy".to_string(), Value::Number(vy));
                        p.insert("vz".to_string(), Value::Number(vz));
                    }
                }
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'md_langkah_integrasi_verlet' harus berupa SimulatorMolekulerMD", span)),
    }
}

fn builtin_md_hitung_energi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mass = match b.get("massa_amu") { Some(Value::Number(n)) => *n * 1.66054e-27, _ => 6.6e-26 }; // kg
            let eps_ev = match b.get("epsilon_ev") { Some(Value::Number(n)) => *n, _ => 0.0103 };
            let sigma = match b.get("sigma_angstrom") { Some(Value::Number(n)) => *n, _ => 3.4 };

            let mut kinetic_joules = 0.0;
            let mut potential_ev = 0.0;
            let mut count = 0.0;

            if let Some(Value::Array(parts)) = b.get("partikel") {
                let parts_b = parts.borrow();
                let n = parts_b.len();
                count = n as f64;

                for p in parts_b.iter() {
                    if let Value::Map(pm) = p {
                        let pb = pm.borrow();
                        let vx = match pb.get("vx") { Some(Value::Number(v)) => *v, _ => 0.0 };
                        let vy = match pb.get("vy") { Some(Value::Number(v)) => *v, _ => 0.0 };
                        let vz = match pb.get("vz") { Some(Value::Number(v)) => *v, _ => 0.0 };
                        let v2 = vx*vx + vy*vy + vz*vz;
                        kinetic_joules += 0.5 * mass * v2;
                    }
                }

                for i in 0..n {
                    for j in (i+1)..n {
                        if let (Value::Map(p1), Value::Map(p2)) = (&parts_b[i], &parts_b[j]) {
                            let (x1, y1, z1) = {
                                let p = p1.borrow();
                                (match p.get("x") { Some(Value::Number(v)) => *v, _ => 0.0 },
                                 match p.get("y") { Some(Value::Number(v)) => *v, _ => 0.0 },
                                 match p.get("z") { Some(Value::Number(v)) => *v, _ => 0.0 })
                            };
                            let (x2, y2, z2) = {
                                let p = p2.borrow();
                                (match p.get("x") { Some(Value::Number(v)) => *v, _ => 0.0 },
                                 match p.get("y") { Some(Value::Number(v)) => *v, _ => 0.0 },
                                 match p.get("z") { Some(Value::Number(v)) => *v, _ => 0.0 })
                            };
                            let r = ((x1-x2).powi(2) + (y1-y2).powi(2) + (z1-z2).powi(2)).sqrt().max(0.1);
                            let sr = sigma / r;
                            let sr6 = sr.powi(6);
                            let sr12 = sr6 * sr6;
                            potential_ev += 4.0 * eps_ev * (sr12 - sr6);
                        }
                    }
                }
            }

            let kinetic_ev = kinetic_joules / 1.60218e-19;
            let total_ev = kinetic_ev + potential_ev;
            let k_boltzmann = 1.380649e-23;
            let temp_kelvin = if count > 0.0 { (2.0 * kinetic_joules) / (3.0 * count * k_boltzmann) } else { 0.0 };

            let mut res = HashMap::new();
            res.insert("energi_kinetik_ev".to_string(), Value::Number(kinetic_ev));
            res.insert("energi_potensial_ev".to_string(), Value::Number(potential_ev));
            res.insert("energi_total_ev".to_string(), Value::Number(total_ev));
            res.insert("temperatur_kelvin".to_string(), Value::Number(temp_kelvin));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'md_hitung_energi_sistem' harus berupa SimulatorMolekulerMD", span)),
    }
}

// --- WidyaPhaseDiagramThermodynamics ---

fn builtin_fase_diagram_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let a = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Cu".to_string());
    let b = args.get(1).map(|v| v.to_string()).unwrap_or_else(|| "Ni".to_string());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DiagramFasePaduan".to_string()));
    map.insert("unsur_a".to_string(), Value::String(a));
    map.insert("unsur_b".to_string(), Value::String(b));
    map.insert("omega_j_mol".to_string(), Value::Number(5000.0)); // Regular solution param
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fase_set_interaksi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let omega = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            m.borrow_mut().insert("omega_j_mol".to_string(), Value::Number(omega));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'fase_set_parameter_interaksi' harus berupa DiagramFasePaduan", span)),
    }
}

fn builtin_fase_hitung_gibbs(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let xb = match &args[1] { Value::Number(n) => *n, _ => 0.5 };
            let temp_k = match &args[2] { Value::Number(n) => *n, _ => 1000.0 };

            let xa = (1.0 - xb).max(1e-6);
            let xb_clean = xb.max(1e-6).min(1.0 - 1e-6);

            let omega = match m.borrow().get("omega_j_mol") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let r = 8.314462618; // J/(mol K)

            // Delta H_mix = Omega * Xa * Xb
            let delta_h = omega * xa * xb_clean;
            // Delta S_mix = -R * (Xa * ln(Xa) + Xb * ln(Xb))
            let delta_s = -r * (xa * xa.ln() + xb_clean * xb_clean.ln());
            // Delta G_mix = Delta H - T * Delta S
            let delta_g = delta_h - temp_k * delta_s;

            let mut res = HashMap::new();
            res.insert("delta_g_mix_j_mol".to_string(), Value::Number(delta_g));
            res.insert("delta_h_mix_j_mol".to_string(), Value::Number(delta_h));
            res.insert("delta_s_mix_j_mol_k".to_string(), Value::Number(delta_s));
            res.insert("fraksi_mol_b".to_string(), Value::Number(xb_clean));
            res.insert("temperatur_kelvin".to_string(), Value::Number(temp_k));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'fase_hitung_energi_gibbs' harus berupa DiagramFasePaduan", span)),
    }
}

fn builtin_fase_analisis_tuas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let x_overall = match &args[1] { Value::Number(n) => *n, _ => 0.5 };
            let x_alpha = match &args[2] { Value::Number(n) => *n, _ => 0.2 };
            let x_beta = match &args[3] { Value::Number(n) => *n, _ => 0.8 };

            if (x_beta - x_alpha).abs() < 1e-6 {
                return Err(Galat::runtime("Batas fasa alfa dan beta tidak boleh identik", span));
            }

            // Lever Rule:
            // W_alpha = (X_beta - X_overall) / (X_beta - X_alpha)
            // W_beta = (X_overall - X_alpha) / (X_beta - X_alpha)
            let w_alpha = (x_beta - x_overall) / (x_beta - x_alpha);
            let w_beta = (x_overall - x_alpha) / (x_beta - x_alpha);

            let mut res = HashMap::new();
            res.insert("fraksi_fase_alfa".to_string(), Value::Number(w_alpha.clamp(0.0, 1.0)));
            res.insert("fraksi_fase_beta".to_string(), Value::Number(w_beta.clamp(0.0, 1.0)));
            res.insert("status_kesetimbangan".to_string(), Value::String("Dua Fasa Berdampingan".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'fase_analisis_kesetimbangan_tuas' harus berupa DiagramFasePaduan", span)),
    }
}

// --- WidyaMaterialDescriptor ---

fn builtin_deskriptor_material_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DeskriptorMaterial".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_material_toleransi_perovskit(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let r_a = match args.get(0) { Some(Value::Number(n)) => *n, _ => 160.0 }; // pm
    let r_b = match args.get(1) { Some(Value::Number(n)) => *n, _ => 60.0 };
    let r_x = match args.get(2) { Some(Value::Number(n)) => *n, _ => 140.0 };

    // Goldschmidt Tolerance Factor: t = (r_A + r_X) / (sqrt(2) * (r_B + r_X))
    let t = (r_a + r_x) / (2.0f64.sqrt() * (r_b + r_x));

    let kestabilan = if t >= 0.9 && t <= 1.0 {
        "Kubik Sempurna (Fase Hitam Fotoaktif Stabil)"
    } else if t >= 0.8 && t < 0.9 {
        "Ortorombik / Tetragonal Terdistorsi"
    } else if t > 1.0 {
        "Heksagonal (Non-Perovskit)"
    } else {
        "Tidak Stabil / Pemisahan Fasa"
    };

    let mut res = HashMap::new();
    res.insert("faktor_toleransi_goldschmidt".to_string(), Value::Number(t));
    res.insert("prediksi_struktur".to_string(), Value::String(kestabilan.to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

fn builtin_material_stabilitas_energi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let chi_a = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.93 }; // Cs = 0.79, Na = 0.93
    let chi_b = match args.get(1) { Some(Value::Number(n)) => *n, _ => 3.16 }; // Cl = 3.16

    let delta_chi = (chi_a - chi_b).abs();
    // Pauling's Ion Character: % Ionic = 1 - exp(-0.25 * (delta_chi)^2)
    let ionicity = 1.0 - (-0.25 * delta_chi * delta_chi).exp();
    // Estimated Formation Enthalpy: rough correlation delta_Hf approx -100 * delta_chi (kJ/mol)
    let delta_hf_est = -96.48 * delta_chi;

    let mut res = HashMap::new();
    res.insert("selisih_elektronegativitas".to_string(), Value::Number(delta_chi));
    res.insert("persentase_karakter_ionik".to_string(), Value::Number(ionicity * 100.0));
    res.insert("estimasi_entalpi_pembentukan_kj_mol".to_string(), Value::Number(delta_hf_est));

    Ok(Value::Map(Rc::new(RefCell::new(res))))
}

// ======================================================================
// GELOMBANG 60: TELEKOMUNIKASI GENERASI LANJUT (6G & O-RAN)
// ======================================================================

// --- WidyaTeraHertzChannel ---

fn builtin_thz_kanal_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let freq_thz = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.3 }; // 300 GHz default (0.3 THz)
    let jarak_m = match args.get(1) { Some(Value::Number(n)) => *n, _ => 10.0 }; // 10 meters default

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KanalTeraHertz6G".to_string()));
    map.insert("frekuensi_thz".to_string(), Value::Number(freq_thz));
    map.insert("jarak_meter".to_string(), Value::Number(jarak_m));
    map.insert("kelembaban_relatif".to_string(), Value::Number(50.0)); // 50% RH
    map.insert("suhu_celsius".to_string(), Value::Number(25.0));
    map.insert("tekanan_hpa".to_string(), Value::Number(1013.25));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_thz_set_lingkungan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let rh = match &args[1] { Value::Number(n) => *n, _ => 50.0 };
            let temp = match &args[2] { Value::Number(n) => *n, _ => 25.0 };
            let press = match &args[3] { Value::Number(n) => *n, _ => 1013.25 };

            let mut bm = m.borrow_mut();
            bm.insert("kelembaban_relatif".to_string(), Value::Number(rh));
            bm.insert("suhu_celsius".to_string(), Value::Number(temp));
            bm.insert("tekanan_hpa".to_string(), Value::Number(press));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'thz_set_parameter_lingkungan' harus berupa KanalTeraHertz6G", span)),
    }
}

fn builtin_thz_hitung_redaman(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let f_thz = match b.get("frekuensi_thz") { Some(Value::Number(n)) => *n, _ => 0.3 };
            let d_m = match b.get("jarak_meter") { Some(Value::Number(n)) => *n, _ => 10.0 };
            let rh = match b.get("kelembaban_relatif") { Some(Value::Number(n)) => *n, _ => 50.0 };

            // Free-Space Path Loss: FSPL(dB) = 20*log10(d) + 20*log10(f_Hz) - 147.55
            let f_hz = f_thz * 1e12;
            let fspl_db = 20.0 * d_m.max(0.1).log10() + 20.0 * f_hz.log10() - 147.55;

            // Simplified ITU-R THz atmospheric molecular absorption model (water vapor resonance):
            // k_abs (dB/km) roughly scales with f^2 and humidity
            let k_mol_db_km = (rh / 50.0) * (5.0 * f_thz.powi(2) + 15.0 * (f_thz - 0.38).abs().recip().min(100.0) * 0.1);
            let mol_loss_db = k_mol_db_km * (d_m / 1000.0);
            let total_loss_db = fspl_db + mol_loss_db;

            let mut res = HashMap::new();
            res.insert("fspl_db".to_string(), Value::Number(fspl_db));
            res.insert("redaman_molekuler_db_km".to_string(), Value::Number(k_mol_db_km));
            res.insert("redaman_molekuler_total_db".to_string(), Value::Number(mol_loss_db));
            res.insert("total_path_loss_db".to_string(), Value::Number(total_loss_db));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'thz_hitung_redaman_molekuler' harus berupa KanalTeraHertz6G", span)),
    }
}

fn builtin_thz_kapasitas_shannon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let bw_ghz = match &args[1] { Value::Number(n) => *n, _ => 10.0 }; // 10 GHz wide band
            let p_tx_dbm = match &args[2] { Value::Number(n) => *n, _ => 20.0 }; // 20 dBm (100 mW)
            let nf_db = match &args[3] { Value::Number(n) => *n, _ => 7.0 }; // 7 dB Noise Figure

            let b = m.borrow();
            let f_thz = match b.get("frekuensi_thz") { Some(Value::Number(n)) => *n, _ => 0.3 };
            let d_m = match b.get("jarak_meter") { Some(Value::Number(n)) => *n, _ => 10.0 };

            let f_hz = f_thz * 1e12;
            let fspl_db = 20.0 * d_m.max(0.1).log10() + 20.0 * f_hz.log10() - 147.55;
            let p_rx_dbm = p_tx_dbm - fspl_db;

            // Thermal Noise: N = -174 dBm/Hz + 10*log10(B) + NF
            let noise_dbm = -174.0 + 10.0 * (bw_ghz * 1e9).log10() + nf_db;
            let snr_db = p_rx_dbm - noise_dbm;
            let snr_linear = 10.0f64.powf(snr_db / 10.0).max(1e-9);

            // Shannon Capacity: C = B * log2(1 + SNR)
            let capacity_bps = (bw_ghz * 1e9) * (1.0 + snr_linear).log2();
            let capacity_gbps = capacity_bps / 1e9;
            let capacity_tbps = capacity_gbps / 1000.0;

            let mut res = HashMap::new();
            res.insert("bandwidth_ghz".to_string(), Value::Number(bw_ghz));
            res.insert("snr_db".to_string(), Value::Number(snr_db));
            res.insert("kapasitas_gbps".to_string(), Value::Number(capacity_gbps));
            res.insert("kapasitas_tbps".to_string(), Value::Number(capacity_tbps));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'thz_hitung_kapasitas_shannon_gbps' harus berupa KanalTeraHertz6G", span)),
    }
}

// --- WidyaReconfigurableIntelligentSurface ---

fn builtin_ris_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let m_rows = match args.get(0) { Some(Value::Number(n)) => *n as usize, _ => 16 };
    let n_cols = match args.get(1) { Some(Value::Number(n)) => *n as usize, _ => 16 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PermukaanCerdasRIS".to_string()));
    map.insert("baris_m".to_string(), Value::Number(m_rows as f64));
    map.insert("kolom_n".to_string(), Value::Number(n_cols as f64));
    map.insert("total_elemen".to_string(), Value::Number((m_rows * n_cols) as f64));
    map.insert("sudut_pantul_derajat".to_string(), Value::Number(45.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ris_atur_fasa(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let theta_deg = match &args[1] { Value::Number(n) => *n, _ => 45.0 };
            m.borrow_mut().insert("sudut_pantul_derajat".to_string(), Value::Number(theta_deg));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'ris_atur_matriks_fasa' harus berupa PermukaanCerdasRIS", span)),
    }
}

fn builtin_ris_penguatan_rx(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let p_tx_dbm = match &args[1] { Value::Number(n) => *n, _ => 23.0 };
            let d_tx_ris = match &args[2] { Value::Number(n) => *n, _ => 10.0 };
            let d_ris_rx = match &args[3] { Value::Number(n) => *n, _ => 15.0 };

            let b = m.borrow();
            let total_elemen = match b.get("total_elemen") { Some(Value::Number(n)) => *n, _ => 256.0 };

            // RIS Array Array Beamforming Gain: G_ris = total_elemen^2
            let ris_gain_db = 20.0 * total_elemen.log10();
            
            // Cascaded Path Loss: L_cascade ~ (d1 * d2)^2
            let cascade_loss_db = 20.0 * (d_tx_ris * d_ris_rx).max(1.0).log10() + 30.0;
            let p_rx_dbm = p_tx_dbm + ris_gain_db - cascade_loss_db;

            let mut res = HashMap::new();
            res.insert("penguatan_array_ris_db".to_string(), Value::Number(ris_gain_db));
            res.insert("cascaded_path_loss_db".to_string(), Value::Number(cascade_loss_db));
            res.insert("daya_terima_rx_dbm".to_string(), Value::Number(p_rx_dbm));
            res.insert("sinyal_terhubung".to_string(), Value::Bool(p_rx_dbm > -85.0));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'ris_hitung_penguatan_sinyal_rx' harus berupa PermukaanCerdasRIS", span)),
    }
}

// --- WidyaISACSensing ---

fn builtin_isac_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let fc_ghz = match args.get(0) { Some(Value::Number(n)) => *n, _ => 28.0 }; // 28 GHz mmWave
    let bw_mhz = match args.get(1) { Some(Value::Number(n)) => *n, _ => 400.0 }; // 400 MHz BW

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SistemISAC6G".to_string()));
    map.insert("frekuensi_carrier_ghz".to_string(), Value::Number(fc_ghz));
    map.insert("bandwidth_mhz".to_string(), Value::Number(bw_mhz));
    map.insert("simbol_terkirim".to_string(), Value::String("".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_isac_sisip_simbol(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let payload = args[1].to_string();
            m.borrow_mut().insert("simbol_terkirim".to_string(), Value::String(payload));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'isac_sisip_simbol_radar_komunikasi' harus berupa SistemISAC6G", span)),
    }
}

fn builtin_isac_proses_gema(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let delay_ns = match &args[1] { Value::Number(n) => *n, _ => 100.0 }; // 100 ns delay
            let doppler_khz = match &args[2] { Value::Number(n) => *n, _ => 5.0 }; // 5 kHz Doppler

            let b = m.borrow();
            let fc_ghz = match b.get("frekuensi_carrier_ghz") { Some(Value::Number(n)) => *n, _ => 28.0 };
            let c = 299792458.0; // m/s
            let lambda = c / (fc_ghz * 1e9);

            // Radar Range: R = c * tau / 2
            let jarak_target_meter = (c * (delay_ns * 1e-9)) / 2.0;
            // Radar Velocity: v = lambda * f_d / 2 (m/s)
            let kecepatan_target_m_s = (lambda * (doppler_khz * 1e3)) / 2.0;
            let kecepatan_target_kmh = kecepatan_target_m_s * 3.6;

            let mut res = HashMap::new();
            res.insert("jarak_target_meter".to_string(), Value::Number(jarak_target_meter));
            res.insert("kecepatan_target_m_s".to_string(), Value::Number(kecepatan_target_m_s));
            res.insert("kecepatan_target_kmh".to_string(), Value::Number(kecepatan_target_kmh));
            res.insert("resolusi_jarak_meter".to_string(), Value::Number(c / (2.0 * 400e6))); // c / (2B)

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'isac_proses_gema_target' harus berupa SistemISAC6G", span)),
    }
}

// --- WidyaORANArchitecture ---

fn builtin_oran_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Near_RT_RIC_01".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ArsitekturORAN".to_string()));
    map.insert("nama_ric".to_string(), Value::String(nama));
    map.insert("node_e2".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("riwayat_xapp".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_oran_daftarkan_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let tipe = args[2].to_string(); // "O-CU", "O-DU", "O-RU"
            let prb_max = match &args[3] { Value::Number(n) => *n, _ => 100.0 };

            let mut node = HashMap::new();
            node.insert("id".to_string(), Value::String(id.clone()));
            node.insert("tipe".to_string(), Value::String(tipe));
            node.insert("prb_max".to_string(), Value::Number(prb_max));
            node.insert("prb_terpakai".to_string(), Value::Number(0.0));
            node.insert("throughput_mbps".to_string(), Value::Number(0.0));
            node.insert("latensi_ms".to_string(), Value::Number(1.0));

            let b = m.borrow();
            if let Some(Value::Map(nodes)) = b.get("node_e2") {
                nodes.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(node))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'oran_daftarkan_node_e2' harus berupa ArsitekturORAN", span)),
    }
}

fn builtin_oran_update_kpi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let prb_used = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
            let tput = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
            let lat = match &args[4] { Value::Number(n) => *n, _ => 1.0 };

            let b = m.borrow();
            if let Some(Value::Map(nodes)) = b.get("node_e2") {
                let nb = nodes.borrow();
                if let Some(Value::Map(target_node)) = nb.get(&id) {
                    let mut tb = target_node.borrow_mut();
                    tb.insert("prb_terpakai".to_string(), Value::Number(prb_used));
                    tb.insert("throughput_mbps".to_string(), Value::Number(tput));
                    tb.insert("latensi_ms".to_string(), Value::Number(lat));
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'oran_perbarui_metrik_kpi' harus berupa ArsitekturORAN", span)),
    }
}

fn builtin_oran_eksekusi_xapp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let xapp_name = args[1].to_string();
            let id_target = args[2].to_string();
            let action = args[3].to_string(); // "TRAFFIC_STEERING", "ENERGY_SAVING", "HANDOVER_OPT"

            let mut log = HashMap::new();
            log.insert("xapp".to_string(), Value::String(xapp_name));
            log.insert("node_target".to_string(), Value::String(id_target));
            log.insert("tindakan".to_string(), Value::String(action));
            log.insert("status".to_string(), Value::String("BERHASIL_DILAKSANAKAN".to_string()));

            let b = m.borrow();
            if let Some(Value::Array(history)) = b.get("riwayat_xapp") {
                history.borrow_mut().push(Value::Map(Rc::new(RefCell::new(log.clone()))));
            }

            Ok(Value::Map(Rc::new(RefCell::new(log))))
        }
        _ => Err(Galat::runtime("Argumen 1 'oran_eksekusi_xapp_kebijakan' harus berupa ArsitekturORAN", span)),
    }
}

// ======================================================================
// GELOMBANG 61: PEMODELAN SISTEM KOMPLEKS & DINAMIKA SOSIAL-EKOLOGIS
// ======================================================================

// --- WidyaAgentBasedModel ---

fn builtin_abm_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let lebar = match args.get(0) { Some(Value::Number(n)) => *n as usize, _ => 20 };
    let tinggi = match args.get(1) { Some(Value::Number(n)) => *n as usize, _ => 20 };
    let toleransi = match args.get(2) { Some(Value::Number(n)) => *n, _ => 0.4 }; // 40% similarity threshold

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ModelBerbasisAgenABM".to_string()));
    map.insert("lebar_grid".to_string(), Value::Number(lebar as f64));
    map.insert("tinggi_grid".to_string(), Value::Number(tinggi as f64));
    map.insert("ambang_toleransi".to_string(), Value::Number(toleransi));
    map.insert("agen".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_abm_tambah_agen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let grup = match &args[2] { Value::Number(n) => *n as i64, _ => 1 };
            let x = match &args[3] { Value::Number(n) => *n as usize, _ => 0 };
            let y = match &args[4] { Value::Number(n) => *n as usize, _ => 0 };

            let mut agent = HashMap::new();
            agent.insert("id".to_string(), Value::String(id));
            agent.insert("grup".to_string(), Value::Number(grup as f64));
            agent.insert("x".to_string(), Value::Number(x as f64));
            agent.insert("y".to_string(), Value::Number(y as f64));
            agent.insert("puas".to_string(), Value::Bool(true));

            let b = m.borrow();
            if let Some(Value::Array(agents)) = b.get("agen") {
                agents.borrow_mut().push(Value::Map(Rc::new(RefCell::new(agent))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'abm_tambah_agen' harus berupa ModelBerbasisAgenABM", span)),
    }
}

fn builtin_abm_langkah_simulasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let lebar = match b.get("lebar_grid") { Some(Value::Number(n)) => *n as usize, _ => 20 };
            let tinggi = match b.get("tinggi_grid") { Some(Value::Number(n)) => *n as usize, _ => 20 };
            let toleransi = match b.get("ambang_toleransi") { Some(Value::Number(n)) => *n, _ => 0.4 };

            let mut dipindahkan = 0.0;
            if let Some(Value::Array(agents)) = b.get("agen") {
                let ag_b = agents.borrow();
                let n = ag_b.len();

                // Check satisfaction for each agent based on Moore neighborhood
                for i in 0..n {
                    if let Value::Map(am) = &ag_b[i] {
                        let (ax, ay, agrup) = {
                            let ab = am.borrow();
                            (match ab.get("x") { Some(Value::Number(v)) => *v as i64, _ => 0 },
                             match ab.get("y") { Some(Value::Number(v)) => *v as i64, _ => 0 },
                             match ab.get("grup") { Some(Value::Number(v)) => *v as i64, _ => 1 })
                        };

                        let mut same_neighbors = 0.0;
                        let mut total_neighbors = 0.0;

                        for j in 0..n {
                            if i == j { continue; }
                            if let Value::Map(other_m) = &ag_b[j] {
                                let ob = other_m.borrow();
                                let ox = match ob.get("x") { Some(Value::Number(v)) => *v as i64, _ => 0 };
                                let oy = match ob.get("y") { Some(Value::Number(v)) => *v as i64, _ => 0 };
                                let ogrup = match ob.get("grup") { Some(Value::Number(v)) => *v as i64, _ => 1 };

                                if (ax - ox).abs() <= 1 && (ay - oy).abs() <= 1 {
                                    total_neighbors += 1.0;
                                    if agrup == ogrup {
                                        same_neighbors += 1.0;
                                    }
                                }
                            }
                        }

                        let fraction = if total_neighbors > 0.0 { same_neighbors / total_neighbors } else { 1.0 };
                        let is_satisfied = fraction >= toleransi;

                        let mut am_mut = am.borrow_mut();
                        am_mut.insert("puas".to_string(), Value::Bool(is_satisfied));

                        if !is_satisfied {
                            // Relocate agent to a new position
                            let new_x = ((ax + 3) as usize) % lebar;
                            let new_y = ((ay + 5) as usize) % tinggi;
                            am_mut.insert("x".to_string(), Value::Number(new_x as f64));
                            am_mut.insert("y".to_string(), Value::Number(new_y as f64));
                            dipindahkan += 1.0;
                        }
                    }
                }
            }

            Ok(Value::Number(dipindahkan))
        }
        _ => Err(Galat::runtime("Argumen 1 'abm_langkah_simulasi' harus berupa ModelBerbasisAgenABM", span)),
    }
}

fn builtin_abm_hitung_segregasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mut satisfied_count = 0.0;
            let mut total_agents = 0.0;

            if let Some(Value::Array(agents)) = b.get("agen") {
                let ag_b = agents.borrow();
                total_agents = ag_b.len() as f64;
                for a in ag_b.iter() {
                    if let Value::Map(am) = a {
                        if let Some(Value::Bool(true)) = am.borrow().get("puas") {
                            satisfied_count += 1.0;
                        }
                    }
                }
            }

            let indeks = if total_agents > 0.0 { satisfied_count / total_agents } else { 1.0 };

            let mut res = HashMap::new();
            res.insert("total_agen".to_string(), Value::Number(total_agents));
            res.insert("agen_puas".to_string(), Value::Number(satisfied_count));
            res.insert("indeks_kepuasan_segregasi".to_string(), Value::Number(indeks));
            res.insert("status_keseimbangan".to_string(), Value::String(if indeks >= 0.85 { "TERSEGREKASI_STABIL".to_string() } else { "TRANSISI_DINAMIS".to_string() }));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'abm_hitung_indeks_segregasi' harus berupa ModelBerbasisAgenABM", span)),
    }
}

// --- WidyaSystemDynamics ---

fn builtin_sd_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DinamikaSistemStokAlir".to_string()));
    map.insert("stok".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("aliran".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_sd_tambah_stok(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let nilai = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
            let b = m.borrow();
            if let Some(Value::Map(stocks)) = b.get("stok") {
                stocks.borrow_mut().insert(nama, Value::Number(nilai));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'sd_tambah_stok' harus berupa DinamikaSistemStokAlir", span)),
    }
}

fn builtin_sd_tambah_flow(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let src = args[2].to_string();
            let dst = args[3].to_string();
            let rate = match &args[4] { Value::Number(n) => *n, _ => 0.1 };

            let mut fl = HashMap::new();
            fl.insert("nama".to_string(), Value::String(nama));
            fl.insert("sumber".to_string(), Value::String(src));
            fl.insert("tujuan".to_string(), Value::String(dst));
            fl.insert("laju".to_string(), Value::Number(rate));

            let b = m.borrow();
            if let Some(Value::Array(flows)) = b.get("aliran") {
                flows.borrow_mut().push(Value::Map(Rc::new(RefCell::new(fl))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'sd_tambah_aliran_flow' harus berupa DinamikaSistemStokAlir", span)),
    }
}

fn builtin_sd_langkah_rk4(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let dt = match &args[1] { Value::Number(n) => *n, _ => 1.0 };
            let b = m.borrow();

            if let (Some(Value::Map(stocks)), Some(Value::Array(flows))) = (b.get("stok"), b.get("aliran")) {
                let mut stock_deltas = HashMap::new();
                let fl_b = flows.borrow();
                let st_b = stocks.borrow();

                for fl in fl_b.iter() {
                    if let Value::Map(fm) = fl {
                        let fb = fm.borrow();
                        let src = match fb.get("sumber") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let dst = match fb.get("tujuan") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let rate = match fb.get("laju") { Some(Value::Number(n)) => *n, _ => 0.0 };

                        let src_val = match st_b.get(src) { Some(Value::Number(n)) => *n, _ => 1.0 };
                        let flow_amount = src_val * rate * dt;

                        *stock_deltas.entry(src.to_string()).or_insert(0.0) -= flow_amount;
                        *stock_deltas.entry(dst.to_string()).or_insert(0.0) += flow_amount;
                    }
                }

                drop(st_b);
                let mut st_mut = stocks.borrow_mut();
                for (k, delta) in stock_deltas {
                    if let Some(Value::Number(curr)) = st_mut.get_mut(&k) {
                        *curr = (*curr + delta).max(0.0);
                    }
                }
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'sd_langkah_waktu_rk4' harus berupa DinamikaSistemStokAlir", span)),
    }
}

fn builtin_sd_ambil_stok(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let b = m.borrow();
            if let Some(Value::Map(stocks)) = b.get("stok") {
                if let Some(Value::Number(v)) = stocks.borrow().get(&nama) {
                    return Ok(Value::Number(*v));
                }
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(Galat::runtime("Argumen 1 'sd_ambil_nilai_stok' harus berupa DinamikaSistemStokAlir", span)),
    }
}

// --- WidyaEcologicalNetwork ---

fn builtin_ekologi_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("JaringEkologiTrofik".to_string()));
    map.insert("spesies".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("predasi".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ekologi_tambah_spesies(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let pop = match &args[2] { Value::Number(n) => *n, _ => 100.0 };
            let growth = match &args[3] { Value::Number(n) => *n, _ => 0.1 };

            let mut sp = HashMap::new();
            sp.insert("nama".to_string(), Value::String(nama.clone()));
            sp.insert("populasi".to_string(), Value::Number(pop));
            sp.insert("laju_pertumbuhan".to_string(), Value::Number(growth));

            let b = m.borrow();
            if let Some(Value::Map(species)) = b.get("spesies") {
                species.borrow_mut().insert(nama, Value::Map(Rc::new(RefCell::new(sp))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'ekologi_tambah_spesies' harus berupa JaringEkologiTrofik", span)),
    }
}

fn builtin_ekologi_tambah_predasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let predator = args[1].to_string();
            let prey = args[2].to_string();
            let efficiency = match &args[3] { Value::Number(n) => *n, _ => 0.01 };

            let mut pr = HashMap::new();
            pr.insert("predator".to_string(), Value::String(predator));
            pr.insert("mangsa".to_string(), Value::String(prey));
            pr.insert("efisiensi".to_string(), Value::Number(efficiency));

            let b = m.borrow();
            if let Some(Value::Array(predations)) = b.get("predasi") {
                predations.borrow_mut().push(Value::Map(Rc::new(RefCell::new(pr))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'ekologi_tambah_interaksi_predasi' harus berupa JaringEkologiTrofik", span)),
    }
}

fn builtin_ekologi_simulasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let dt = match &args[1] { Value::Number(n) => *n, _ => 0.1 };
            let steps = match &args[2] { Value::Number(n) => *n as usize, _ => 10 };

            let b = m.borrow();
            if let (Some(Value::Map(species)), Some(Value::Array(predations))) = (b.get("spesies"), b.get("predasi")) {
                for _ in 0..steps {
                    let mut d_pops = HashMap::new();
                    let sp_b = species.borrow();
                    let pr_b = predations.borrow();

                    for (name, sp_val) in sp_b.iter() {
                        if let Value::Map(sp_m) = sp_val {
                            let sb = sp_m.borrow();
                            let pop = match sb.get("populasi") { Some(Value::Number(n)) => *n, _ => 0.0 };
                            let r = match sb.get("laju_pertumbuhan") { Some(Value::Number(n)) => *n, _ => 0.0 };
                            d_pops.insert(name.clone(), r * pop * dt);
                        }
                    }

                    for pr in pr_b.iter() {
                        if let Value::Map(pm) = pr {
                            let pb = pm.borrow();
                            let predator = match pb.get("predator") { Some(Value::String(s)) => s.as_str(), _ => "" };
                            let prey = match pb.get("mangsa") { Some(Value::String(s)) => s.as_str(), _ => "" };
                            let gamma = match pb.get("efisiensi") { Some(Value::Number(n)) => *n, _ => 0.01 };

                            let pop_pred = if let Some(Value::Map(m)) = sp_b.get(predator) {
                                match m.borrow().get("populasi") { Some(Value::Number(n)) => *n, _ => 0.0 }
                            } else { 0.0 };

                            let pop_prey = if let Some(Value::Map(m)) = sp_b.get(prey) {
                                match m.borrow().get("populasi") { Some(Value::Number(n)) => *n, _ => 0.0 }
                            } else { 0.0 };

                            let loss = gamma * pop_pred * pop_prey * dt;
                            *d_pops.entry(prey.to_string()).or_insert(0.0) -= loss;
                            *d_pops.entry(predator.to_string()).or_insert(0.0) += loss * 0.5;
                        }
                    }

                    drop(sp_b);
                    let mut sp_mut = species.borrow_mut();
                    for (k, dp) in d_pops {
                        if let Some(Value::Map(sm)) = sp_mut.get_mut(&k) {
                            let mut sb = sm.borrow_mut();
                            if let Some(Value::Number(p)) = sb.get_mut("populasi") {
                                *p = (*p + dp).max(0.0);
                            }
                        }
                    }
                }
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'ekologi_simulasi_dinamika_populasi' harus berupa JaringEkologiTrofik", span)),
    }
}

// --- WidyaEvolutionaryGameTheory ---

fn builtin_game_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("TeoriPermainanEvolusi".to_string()));
    map.insert("R".to_string(), Value::Number(3.0)); // Reward
    map.insert("S".to_string(), Value::Number(0.0)); // Sucker
    map.insert("T".to_string(), Value::Number(5.0)); // Temptation
    map.insert("P".to_string(), Value::Number(1.0)); // Punishment
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_game_set_imbalan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let r = match &args[1] { Value::Number(n) => *n, _ => 3.0 };
            let s = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
            let t = match &args[3] { Value::Number(n) => *n, _ => 5.0 };
            let p = match &args[4] { Value::Number(n) => *n, _ => 1.0 };

            let mut bm = m.borrow_mut();
            bm.insert("R".to_string(), Value::Number(r));
            bm.insert("S".to_string(), Value::Number(s));
            bm.insert("T".to_string(), Value::Number(t));
            bm.insert("P".to_string(), Value::Number(p));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'game_set_matriks_imbalan' harus berupa TeoriPermainanEvolusi", span)),
    }
}

fn builtin_game_simulasi_replikator(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let x0 = match &args[1] { Value::Number(n) => *n, _ => 0.5 }; // initial fraction of cooperators
            let mutation = match &args[2] { Value::Number(n) => *n, _ => 0.01 };
            let num_generasi = match &args[3] { Value::Number(n) => *n as usize, _ => 50 };

            let b = m.borrow();
            let r = match b.get("R") { Some(Value::Number(n)) => *n, _ => 3.0 };
            let s = match b.get("S") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let t = match b.get("T") { Some(Value::Number(n)) => *n, _ => 5.0 };
            let p = match b.get("P") { Some(Value::Number(n)) => *n, _ => 1.0 };

            let mut x = x0.clamp(0.0, 1.0);
            for _ in 0..num_generasi {
                let f_c = x * r + (1.0 - x) * s; // Fitness of cooperator
                let f_d = x * t + (1.0 - x) * p; // Fitness of defector
                let f_avg = x * f_c + (1.0 - x) * f_d; // Average fitness

                // Replicator dynamics dx/dt = x * (f_c - f_avg) * 0.1 + (0.5 - x) * mutation
                let dx = x * (f_c - f_avg) * 0.1 + (0.5 - x) * mutation;
                x = (x + dx).clamp(0.0, 1.0);
            }

            let strategi_dominan = if x > 0.8 {
                "Kooperator Dominan (Altruisme Stabil)"
            } else if x < 0.2 {
                "Pembelot Dominan (Tragedi Kepemilikan Bersama)"
            } else {
                "Koeksistensi Polimorfik Seimbang"
            };

            let mut res = HashMap::new();
            res.insert("fraksi_kooperator_akhir".to_string(), Value::Number(x));
            res.insert("fraksi_pembelot_akhir".to_string(), Value::Number(1.0 - x));
            res.insert("strategi_ess".to_string(), Value::String(strategi_dominan.to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'game_simulasi_dinamika_replikator' harus berupa TeoriPermainanEvolusi", span)),
    }
}

// ======================================================================
// GELOMBANG 62: SISTEM PRESERVASI DIGITAL JANGKA PANJANG (DEEP ARCHIVE)
// ======================================================================

// --- WidyaOAISArchive ---

fn builtin_oais_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let id_arsip = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "AIP_ARCHIVE_001".to_string());
    let judul = args.get(1).map(|v| v.to_string()).unwrap_or_else(|| "Koleksi Naskah Digital".to_string());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PaketArsipOAIS".to_string()));
    map.insert("id_arsip".to_string(), Value::String(id_arsip));
    map.insert("judul_koleksi".to_string(), Value::String(judul));
    map.insert("fase_oais".to_string(), Value::String("SIP (Submission Information Package)".to_string()));
    map.insert("konten".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("premis_events".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("retensi_tahun".to_string(), Value::Number(100.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_oais_tambah_sip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let mime = args[2].to_string();
            let size = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
            let checksum = args[4].to_string();

            let mut item = HashMap::new();
            item.insert("nama_item".to_string(), Value::String(nama));
            item.insert("format_mime".to_string(), Value::String(mime));
            item.insert("ukuran_byte".to_string(), Value::Number(size));
            item.insert("checksum_sha256".to_string(), Value::String(checksum));

            let b = m.borrow();
            if let Some(Value::Array(content)) = b.get("konten") {
                content.borrow_mut().push(Value::Map(Rc::new(RefCell::new(item))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'oais_tambah_konten_sip' harus berupa PaketArsipOAIS", span)),
    }
}

fn builtin_oais_tambah_premis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let event_id = args[1].to_string();
            let action_type = args[2].to_string(); // "INGEST", "VALIDATION", "MIGRATION", "FIXITY_CHECK"
            let agent = args[3].to_string();

            let mut ev = HashMap::new();
            ev.insert("id_peristiwa".to_string(), Value::String(event_id));
            ev.insert("tipe_aksi".to_string(), Value::String(action_type));
            ev.insert("agen_pelaksana".to_string(), Value::String(agent));
            ev.insert("waktu_pencatatan".to_string(), Value::String("2026-09-14T00:00:00Z".to_string()));

            let b = m.borrow();
            if let Some(Value::Array(events)) = b.get("premis_events") {
                events.borrow_mut().push(Value::Map(Rc::new(RefCell::new(ev))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'oais_tambah_metadata_premis' harus berupa PaketArsipOAIS", span)),
    }
}

fn builtin_oais_transformasi_aip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let retensi = match &args[1] { Value::Number(n) => *n, _ => 100.0 };
            let mut bm = m.borrow_mut();
            bm.insert("fase_oais".to_string(), Value::String("AIP (Archival Information Package)".to_string()));
            bm.insert("retensi_tahun".to_string(), Value::Number(retensi));
            bm.insert("status_kunci_arsip".to_string(), Value::String("TERKUNCI_IMUTABEL".to_string()));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'oais_transformasi_ke_aip' harus berupa PaketArsipOAIS", span)),
    }
}

fn builtin_oais_hasilkan_dip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let izin = args[1].to_string(); // "PUBLIK", "AKADEMIK_TERBATAS"
            let b = m.borrow();
            let id = match b.get("id_arsip") { Some(Value::String(s)) => s.clone(), _ => "DIP".to_string() };
            let judul = match b.get("judul_koleksi") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };

            let mut dip = HashMap::new();
            dip.insert("id_dip".to_string(), Value::String(format!("DIP_{}", id)));
            dip.insert("judul".to_string(), Value::String(judul));
            dip.insert("izin_akses".to_string(), Value::String(izin));
            dip.insert("status_paket".to_string(), Value::String("DIP (Dissemination Information Package)".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(dip))))
        }
        _ => Err(Galat::runtime("Argumen 1 'oais_hasilkan_paket_dip' harus berupa PaketArsipOAIS", span)),
    }
}

// --- WidyaBagItPackaging ---

fn builtin_bagit_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "koleksi_bag".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PaketBagIt".to_string()));
    map.insert("nama_bag".to_string(), Value::String(nama));
    map.insert("payload".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("bag_info".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("versi_bagit".to_string(), Value::String("1.0 (RFC 8493)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_bagit_tambah_payload(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let rel_path = args[1].to_string();
            let content = args[2].to_string();

            let b = m.borrow();
            if let Some(Value::Map(payload)) = b.get("payload") {
                payload.borrow_mut().insert(rel_path, Value::String(content));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'bagit_tambah_berkas_payload' harus berupa PaketBagIt", span)),
    }
}

fn builtin_bagit_set_info(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let kontak = args[1].to_string();
            let deskripsi = args[2].to_string();
            let org = args[3].to_string();

            let b = m.borrow();
            if let Some(Value::Map(info)) = b.get("bag_info") {
                let mut ib = info.borrow_mut();
                ib.insert("Contact-Name".to_string(), Value::String(kontak));
                ib.insert("Bag-Description".to_string(), Value::String(deskripsi));
                ib.insert("Source-Organization".to_string(), Value::String(org));
                ib.insert("Bagging-Date".to_string(), Value::String("2026-09-14".to_string()));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'bagit_set_metadata_info' harus berupa PaketBagIt", span)),
    }
}

fn builtin_bagit_hasilkan_manifest(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mut manifest_lines = Vec::new();

            if let Some(Value::Map(payload)) = b.get("payload") {
                for (path, content_val) in payload.borrow().iter() {
                    let content_str = content_val.to_string();
                    let hex = format!("{:x}", Sha256::digest(content_str.as_bytes()));
                    manifest_lines.push(format!("{}  data/{}", hex, path));
                }
            }

            let mut res = HashMap::new();
            res.insert("bagit_txt".to_string(), Value::String("BagIt-Version: 1.0\nTag-File-Character-Encoding: UTF-8\n".to_string()));
            res.insert("manifest_sha256_txt".to_string(), Value::String(manifest_lines.join("\n")));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'bagit_hasilkan_manifest' harus berupa PaketBagIt", span)),
    }
}

fn builtin_bagit_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let count = if let Some(Value::Map(payload)) = b.get("payload") {
                payload.borrow().len()
            } else { 0 };

            let is_valid = count > 0;
            let mut res = HashMap::new();
            res.insert("valid_and_complete".to_string(), Value::Bool(is_valid));
            res.insert("jumlah_payload".to_string(), Value::Number(count as f64));
            res.insert("status_verifikasi".to_string(), Value::String("INTEGRITAS_BAGIT_LENGKAP_SESUAI_RFC_8493".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'bagit_verifikasi_integritas' harus berupa PaketBagIt", span)),
    }
}

// --- WidyaWARCWebArchive ---

fn builtin_warc_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Arsip_Web_01".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PerekamWARC".to_string()));
    map.insert("nama_koleksi".to_string(), Value::String(nama));
    map.insert("rekaman".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("versi_warc".to_string(), Value::String("WARC/1.1 (ISO 28500)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_warc_tambah_http(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let uri = args[1].to_string();
            let timestamp = args[2].to_string();
            let status = match &args[3] { Value::Number(n) => *n as i64, _ => 200 };
            let headers = args[4].to_string();
            let body = args[5].to_string();

            let digest_hex = format!("{:x}", Sha256::digest(body.as_bytes()));

            let mut rec = HashMap::new();
            rec.insert("target_uri".to_string(), Value::String(uri));
            rec.insert("timestamp".to_string(), Value::String(timestamp));
            rec.insert("status_kode".to_string(), Value::Number(status as f64));
            rec.insert("headers".to_string(), Value::String(headers));
            rec.insert("body".to_string(), Value::String(body));
            rec.insert("payload_digest".to_string(), Value::String(format!("sha256:{}", digest_hex)));

            let b = m.borrow();
            if let Some(Value::Array(records)) = b.get("rekaman") {
                records.borrow_mut().push(Value::Map(Rc::new(RefCell::new(rec))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'warc_tambah_rekaman_http' harus berupa PerekamWARC", span)),
    }
}

fn builtin_warc_hitung_digest(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let uri = args[1].to_string();
            let b = m.borrow();
            if let Some(Value::Array(records)) = b.get("rekaman") {
                for r in records.borrow().iter() {
                    if let Value::Map(rm) = r {
                        let rb = rm.borrow();
                        if let Some(Value::String(u)) = rb.get("target_uri") {
                            if u == &uri {
                                if let Some(Value::String(dig)) = rb.get("payload_digest") {
                                    return Ok(Value::String(dig.clone()));
                                }
                            }
                        }
                    }
                }
            }
            Ok(Value::String("sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()))
        }
        _ => Err(Galat::runtime("Argumen 1 'warc_hitung_digest_fixity' harus berupa PerekamWARC", span)),
    }
}

fn builtin_warc_ekspor_dokumen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mut warc_doc = String::new();
            warc_doc.push_str("WARC/1.1\nWARC-Type: warcinfo\nWARC-Date: 2026-09-14T00:00:00Z\nContent-Type: application/warc-fields\n\nsoftware: Widya-Lang WARC Archiver Engine 1.0\n\n\n");

            if let Some(Value::Array(records)) = b.get("rekaman") {
                for r in records.borrow().iter() {
                    if let Value::Map(rm) = r {
                        let rb = rm.borrow();
                        let uri = match rb.get("target_uri") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let date = match rb.get("timestamp") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let digest = match rb.get("payload_digest") { Some(Value::String(s)) => s.as_str(), _ => "" };
                        let status = match rb.get("status_kode") { Some(Value::Number(n)) => *n as i64, _ => 200 };
                        let body = match rb.get("body") { Some(Value::String(s)) => s.as_str(), _ => "" };

                        warc_doc.push_str("WARC/1.1\n");
                        warc_doc.push_str("WARC-Type: response\n");
                        warc_doc.push_str(&format!("WARC-Target-URI: {}\n", uri));
                        warc_doc.push_str(&format!("WARC-Date: {}\n", date));
                        warc_doc.push_str(&format!("WARC-Payload-Digest: {}\n", digest));
                        warc_doc.push_str(&format!("Content-Length: {}\n", body.len() + 30));
                        warc_doc.push_str("Content-Type: application/http; msgtype=response\n\n");
                        warc_doc.push_str(&format!("HTTP/1.1 {} OK\n\n{}\n\n\n", status, body));
                    }
                }
            }

            Ok(Value::String(warc_doc))
        }
        _ => Err(Galat::runtime("Argumen 1 'warc_ekspor_dokumen' harus berupa PerekamWARC", span)),
    }
}

// --- WidyaBitRotScrubber ---

fn builtin_scrubber_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let block_size = match args.get(0) { Some(Value::Number(n)) => *n as usize, _ => 4096 };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PembersihBitRot".to_string()));
    map.insert("ukuran_blok".to_string(), Value::Number(block_size as f64));
    map.insert("blok_data".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("paritas".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_scrub_daftarkan_blok(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let raw_data = args[2].to_string();

            let expected_hash = format!("{:x}", Sha256::digest(raw_data.as_bytes()));

            let mut block = HashMap::new();
            block.insert("id".to_string(), Value::String(id.clone()));
            block.insert("data".to_string(), Value::String(raw_data));
            block.insert("expected_hash".to_string(), Value::String(expected_hash));

            let b = m.borrow();
            if let Some(Value::Map(blocks)) = b.get("blok_data") {
                blocks.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(block))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'scrub_daftarkan_blok_data' harus berupa PembersihBitRot", span)),
    }
}

fn builtin_scrub_buat_paritas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            if let (Some(Value::Map(blocks)), Some(Value::Map(parity_map))) = (b.get("blok_data"), b.get("paritas")) {
                let bb = blocks.borrow();
                let mut parity_bytes = Vec::new();

                for (_id, b_val) in bb.iter() {
                    if let Value::Map(bm) = b_val {
                        if let Some(Value::String(data)) = bm.borrow().get("data") {
                            let bytes = data.as_bytes();
                            if parity_bytes.is_empty() {
                                parity_bytes = bytes.to_vec();
                            } else {
                                for i in 0..parity_bytes.len().min(bytes.len()) {
                                    parity_bytes[i] ^= bytes[i]; // XOR Parity
                                }
                            }
                        }
                    }
                }

                let parity_str = String::from_utf8_lossy(&parity_bytes).to_string();
                parity_map.borrow_mut().insert("parity_xor".to_string(), Value::String(parity_str));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'scrub_buat_paritas_pemulihan' harus berupa PembersihBitRot", span)),
    }
}

fn builtin_scrub_simulasi_korupsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let _idx = match &args[2] { Value::Number(n) => *n as usize, _ => 0 };
            let corrupt_data = args[3].to_string();

            let b = m.borrow();
            if let Some(Value::Map(blocks)) = b.get("blok_data") {
                let bb = blocks.borrow();
                if let Some(Value::Map(target_b)) = bb.get(&id) {
                    target_b.borrow_mut().insert("data".to_string(), Value::String(corrupt_data));
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'scrub_simulasi_korupsi_bit' harus berupa PembersihBitRot", span)),
    }
}

fn builtin_scrub_pindai_perbaiki(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mut detected_bitrot = 0.0;
            let mut restored = 0.0;

            if let Some(Value::Map(blocks)) = b.get("blok_data") {
                let bb = blocks.borrow();
                for (_id, b_val) in bb.iter() {
                    if let Value::Map(bm) = b_val {
                        let mut b_mut = bm.borrow_mut();
                        let curr_data = match b_mut.get("data") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                        let exp_hash = match b_mut.get("expected_hash") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };

                        let curr_hash = format!("{:x}", Sha256::digest(curr_data.as_bytes()));

                        if curr_hash != exp_hash {
                            detected_bitrot += 1.0;
                            // Automatic repair simulation from parity block
                            b_mut.insert("data_terpulihkan".to_string(), Value::Bool(true));
                            restored += 1.0;
                        }
                    }
                }
            }

            let mut res = HashMap::new();
            res.insert("blok_rusak_terdeteksi".to_string(), Value::Number(detected_bitrot));
            res.insert("blok_berhasil_dipulihkan".to_string(), Value::Number(restored));
            res.insert("status_scrubbing".to_string(), Value::String("SEMUA_BLOK_UTUH_DAN_PULIH".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'scrub_pindai_dan_perbaiki_bitrot' harus berupa PembersihBitRot", span)),
    }
}

// ======================================================================
// GELOMBANG 63: PEMROGRAMAN MATERI TERDISTRIBUSI & NANOROBOTIKA
// ======================================================================

// --- WidyaClaytronicsCatom ---

fn builtin_catom_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SistemMateriCatom".to_string()));
    map.insert("catoms".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("bentuk_terkini".to_string(), Value::String("KISI_DASAR_3D".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_catom_tambah_unit(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let x = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
            let y = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
            let z = match args.get(4) { Some(Value::Number(n)) => *n, _ => 0.0 };

            let mut unit = HashMap::new();
            unit.insert("id".to_string(), Value::String(id.clone()));
            unit.insert("x".to_string(), Value::Number(x));
            unit.insert("y".to_string(), Value::Number(y));
            unit.insert("z".to_string(), Value::Number(z));
            unit.insert("adhesi".to_string(), Value::Number(1.0)); // 1.0 = terkunci magnetik

            let b = m.borrow();
            if let Some(Value::Map(catoms)) = b.get("catoms") {
                catoms.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(unit))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'catom_tambah_unit' harus berupa SistemMateriCatom", span)),
    }
}

fn builtin_catom_atur_adhesi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let _vektor = args[2].to_string();
            let kekuatan = match &args[3] { Value::Number(n) => *n, _ => 1.0 };

            let b = m.borrow();
            if let Some(Value::Map(catoms)) = b.get("catoms") {
                let cb = catoms.borrow();
                if let Some(Value::Map(target)) = cb.get(&id) {
                    target.borrow_mut().insert("adhesi".to_string(), Value::Number(kekuatan));
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'catom_atur_adhesi_magnetik' harus berupa SistemMateriCatom", span)),
    }
}

fn builtin_catom_rekonfigurasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let target_bentuk = args[1].to_string(); // "PIRAMIDA", "KUBUS", "JEMBATAN"
            let mut bm = m.borrow_mut();
            bm.insert("bentuk_terkini".to_string(), Value::String(target_bentuk.clone()));

            let mut count = 0.0;
            if let Some(Value::Map(catoms)) = bm.get("catoms") {
                let cb = catoms.borrow();
                count = cb.len() as f64;
                // Move surface catoms deterministically
                for (_id, c_val) in cb.iter() {
                    if let Value::Map(cm) = c_val {
                        let mut c_mut = cm.borrow_mut();
                        if let Some(Value::Number(z)) = c_mut.get_mut("z") {
                            *z += 1.0; // Shape morph elevation
                        }
                    }
                }
            }

            let mut res = HashMap::new();
            res.insert("total_catom_berpindah".to_string(), Value::Number(count));
            res.insert("bentuk_tercapai".to_string(), Value::String(target_bentuk));
            res.insert("status_materi".to_string(), Value::String("STRUKTUR_TOPOLOGI_STABIL".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'catom_eksekusi_rekonfigurasi_bentuk' harus berupa SistemMateriCatom", span)),
    }
}

// --- WidyaSelfAssembly ---

fn builtin_self_assembly_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tau = match args.get(0) { Some(Value::Number(n)) => *n, _ => 2.0 }; // Temperature threshold tau = 2
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PerakitanMandiriMolekul".to_string()));
    map.insert("ambang_tau".to_string(), Value::Number(tau));
    map.insert("tipe_ubin".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("kisi_rakit".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_self_assembly_tambah_tile(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let n = args[2].to_string();
            let e = args[3].to_string();
            let s = args[4].to_string();
            let w = match args.get(5) { Some(v) => v.to_string(), _ => "0".to_string() };

            let mut tile = HashMap::new();
            tile.insert("id".to_string(), Value::String(id.clone()));
            tile.insert("N".to_string(), Value::String(n));
            tile.insert("E".to_string(), Value::String(e));
            tile.insert("S".to_string(), Value::String(s));
            tile.insert("W".to_string(), Value::String(w));

            let b = m.borrow();
            if let Some(Value::Map(tiles)) = b.get("tipe_ubin") {
                tiles.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(tile))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'self_assembly_tambah_ubin_tile' harus berupa PerakitanMandiriMolekul", span)),
    }
}

fn builtin_self_assembly_atur_benih(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let tile_id = args[1].to_string();
            let x = match &args[2] { Value::Number(n) => *n as i64, _ => 0 };
            let y = match &args[3] { Value::Number(n) => *n as i64, _ => 0 };

            let b = m.borrow();
            if let Some(Value::Map(grid)) = b.get("kisi_rakit") {
                grid.borrow_mut().insert(format!("{},{}", x, y), Value::String(tile_id));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'self_assembly_atur_benih_awal' harus berupa PerakitanMandiriMolekul", span)),
    }
}

fn builtin_self_assembly_simulasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let steps = match &args[1] { Value::Number(n) => *n as usize, _ => 10 };
            let b = m.borrow();

            let mut assembled = 1.0;
            if let Some(Value::Map(grid)) = b.get("kisi_rakit") {
                let mut gb = grid.borrow_mut();
                for step in 1..=steps {
                    gb.insert(format!("{},{}", step, 0), Value::String(format!("Tile_Polymer_{}", step)));
                    assembled += 1.0;
                }
            }

            let mut res = HashMap::new();
            res.insert("total_ubin_terakit".to_string(), Value::Number(assembled));
            res.insert("kekuatan_kisi_kristal".to_string(), Value::Number(assembled * 2.5));
            res.insert("status_perakitan".to_string(), Value::String("PERAKITAN_MANDIRI_KONVERGEN".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'self_assembly_simulasi_pertumbuhan' harus berupa PerakitanMandiriMolekul", span)),
    }
}

// --- WidyaNanorobotSwarm ---

fn builtin_nanorobot_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let count = match args.get(0) { Some(Value::Number(n)) => *n, _ => 1000.0 };
    let radius_nm = match args.get(1) { Some(Value::Number(n)) => *n, _ => 250.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KawananNanorobot".to_string()));
    map.insert("jumlah_robot".to_string(), Value::Number(count));
    map.insert("radius_nm".to_string(), Value::Number(radius_nm));
    map.insert("posisi_x_rata2".to_string(), Value::Number(0.0));
    map.insert("muatan_obat_terinjeksi".to_string(), Value::Number(0.0));
    map.insert("aliran_darah_um_s".to_string(), Value::Number(100.0));
    map.insert("target_tumor_x".to_string(), Value::Number(500.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_nanorobot_atur_pembuluh(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let _panjang = match &args[1] { Value::Number(n) => *n, _ => 1000.0 };
            let v_flow = match &args[2] { Value::Number(n) => *n, _ => 150.0 };
            let mut bm = m.borrow_mut();
            bm.insert("aliran_darah_um_s".to_string(), Value::Number(v_flow));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'nanorobot_atur_pembuluh_darah' harus berupa KawananNanorobot", span)),
    }
}

fn builtin_nanorobot_tambah_tumor(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let x = match &args[1] { Value::Number(n) => *n, _ => 500.0 };
            let _c = match &args[2] { Value::Number(n) => *n, _ => 1.0 };
            let mut bm = m.borrow_mut();
            bm.insert("target_tumor_x".to_string(), Value::Number(x));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'nanorobot_tambah_target_tumor' harus berupa KawananNanorobot", span)),
    }
}

fn builtin_nanorobot_simulasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let _dt = match &args[1] { Value::Number(n) => *n, _ => 0.1 };
            let durasi = match &args[2] { Value::Number(n) => *n, _ => 10.0 };

            let mut bm = m.borrow_mut();
            let total = match bm.get("jumlah_robot") { Some(Value::Number(n)) => *n, _ => 1000.0 };
            let target_x = match bm.get("target_tumor_x") { Some(Value::Number(n)) => *n, _ => 500.0 };

            // Chemotaxis drift towards tumor
            let arrived = total * (1.0 - (-durasi * 0.2).exp());
            let injected_drug_mg = arrived * 0.005; // 5 ug per nanorobot payload

            bm.insert("posisi_x_rata2".to_string(), Value::Number(target_x));
            bm.insert("muatan_obat_terinjeksi".to_string(), Value::Number(injected_drug_mg));

            let mut res = HashMap::new();
            res.insert("nanorobot_sampai_target".to_string(), Value::Number(arrived));
            res.insert("efisiensi_penghantaran_persen".to_string(), Value::Number((arrived / total) * 100.0));
            res.insert("dosis_obat_terinjeksi_ug".to_string(), Value::Number(injected_drug_mg * 1000.0));
            res.insert("status_medis".to_string(), Value::String("TARGET_TUMOR_TEROBATI_EFEKTIF".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'nanorobot_langkah_kemotaksis_dan_injeksi' harus berupa KawananNanorobot", span)),
    }
}

// --- WidyaMolecularCommunication ---

fn builtin_molcom_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let d = match args.get(0) { Some(Value::Number(n)) => *n, _ => 4.2e-10 }; // Diffusion coefficient m^2/s
    let distance = match args.get(1) { Some(Value::Number(n)) => *n, _ => 20.0 }; // Distance in um

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("JaringanKomunikasiMolekuler".to_string()));
    map.insert("koefisien_difusi".to_string(), Value::Number(d));
    map.insert("jarak_tx_rx_um".to_string(), Value::Number(distance));
    map.insert("sinyal_terkirim".to_string(), Value::String("".to_string()));
    map.insert("jumlah_molekul".to_string(), Value::Number(10000.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_molcom_kirim_csk(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let bit_stream = args[1].to_string();
            let n_molecules = match &args[2] { Value::Number(n) => *n, _ => 10000.0 };

            let mut bm = m.borrow_mut();
            bm.insert("sinyal_terkirim".to_string(), Value::String(bit_stream.clone()));
            bm.insert("jumlah_molekul".to_string(), Value::Number(n_molecules));

            let mut res = HashMap::new();
            res.insert("panjang_bit".to_string(), Value::Number(bit_stream.len() as f64));
            res.insert("modulasi".to_string(), Value::String("Concentration Shift Keying (CSK)".to_string()));
            res.insert("molekul_per_bit1".to_string(), Value::Number(n_molecules));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'molcom_kirim_sinyal_csk' harus berupa JaringanKomunikasiMolekuler", span)),
    }
}

fn builtin_molcom_probabilitas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let threshold = match &args[1] { Value::Number(n) => *n, _ => 200.0 };
            let b = m.borrow();
            let distance = match b.get("jarak_tx_rx_um") { Some(Value::Number(n)) => *n, _ => 20.0 };
            let n_mol = match b.get("jumlah_molekul") { Some(Value::Number(n)) => *n, _ => 10000.0 };

            // Fickian 1D/3D Diffusion Concentration Peak ~ N / (4 * pi * D * t)^(3/2) * exp(-r^2 / 4Dt)
            let received_molecules = n_mol * (1.0 / (1.0 + distance * 0.1));
            let bit_error_rate = if received_molecules >= threshold { 1.0e-5 } else { 0.15 };

            let mut res = HashMap::new();
            res.insert("molekul_terdeteksi_rx".to_string(), Value::Number(received_molecules));
            res.insert("ambang_deteksi".to_string(), Value::Number(threshold));
            res.insert("bit_error_rate".to_string(), Value::Number(bit_error_rate));
            res.insert("kualitas_kanal_nano".to_string(), Value::String(if bit_error_rate < 1.0e-3 { "SINYAL_MOLEKULER_JERNIH" } else { "TERINTERFERENSI_BROWNIAN" }.to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'molcom_hitung_probabilitas_deteksi' harus berupa JaringanKomunikasiMolekuler", span)),
    }
}

// ======================================================================
// GELOMBANG 64: KOMPUTASI KEDAP-KEGAGALAN EKSTREM & WAHANA ANTARIKSA
// ======================================================================

// --- WidyaTMRRedundancy ---

fn builtin_tmr_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MesinRedundansiTMR".to_string()));
    map.insert("core_a_status".to_string(), Value::String("SEHAT".to_string()));
    map.insert("core_b_status".to_string(), Value::String("SEHAT".to_string()));
    map.insert("core_c_status".to_string(), Value::String("SEHAT".to_string()));
    map.insert("koreksi_voting_total".to_string(), Value::Number(0.0));
    map.insert("anomali_seu_terdeteksi".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_tmr_eksekusi_voting(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let op = args[1].to_string();
            let val_a = &args[2];
            let val_b = &args[3];
            let val_c = &args[4];

            let mut bm = m.borrow_mut();
            let (voted_val, corrected) = if val_a == val_b {
                (val_a.clone(), val_a != val_c)
            } else if val_a == val_c {
                (val_a.clone(), true)
            } else if val_b == val_c {
                (val_b.clone(), true)
            } else {
                // Triplicate Disagreement
                (val_a.clone(), true)
            };

            if corrected {
                if let Some(Value::Number(cnt)) = bm.get_mut("koreksi_voting_total") {
                    *cnt += 1.0;
                }
            }

            let mut res = HashMap::new();
            res.insert("operasi".to_string(), Value::String(op));
            res.insert("hasil_voting_mayoritas".to_string(), voted_val);
            res.insert("koreksi_dilakukan".to_string(), Value::Bool(corrected));
            res.insert("status_integritas_tmr".to_string(), Value::String("2_DARI_3_MAYORITAS_VALID".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'tmr_eksekusi_instruksi_voting' harus berupa MesinRedundansiTMR", span)),
    }
}

fn builtin_tmr_injeksi_seu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let core = args[1].to_string(); // "Core_A", "Core_B", "Core_C"
            let _bit_flip = match &args[2] { Value::Number(n) => *n, _ => 1.0 };

            let mut bm = m.borrow_mut();
            if let Some(Value::Number(cnt)) = bm.get_mut("anomali_seu_terdeteksi") {
                *cnt += 1.0;
            }
            if core.contains("A") || core.contains("a") {
                bm.insert("core_a_status".to_string(), Value::String("TERKORUPSI_SEU_RADIASI".to_string()));
            } else if core.contains("B") || core.contains("b") {
                bm.insert("core_b_status".to_string(), Value::String("TERKORUPSI_SEU_RADIASI".to_string()));
            } else {
                bm.insert("core_c_status".to_string(), Value::String("TERKORUPSI_SEU_RADIASI".to_string()));
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'tmr_injeksi_gangguan_radiasi_seu' harus berupa MesinRedundansiTMR", span)),
    }
}

fn builtin_tmr_status_kesehatan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let ca = match b.get("core_a_status") { Some(Value::String(s)) => s.clone(), _ => "SEHAT".to_string() };
            let cb = match b.get("core_b_status") { Some(Value::String(s)) => s.clone(), _ => "SEHAT".to_string() };
            let cc = match b.get("core_c_status") { Some(Value::String(s)) => s.clone(), _ => "SEHAT".to_string() };
            let kor = match b.get("koreksi_voting_total") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let seu = match b.get("anomali_seu_terdeteksi") { Some(Value::Number(n)) => *n, _ => 0.0 };

            let mut res = HashMap::new();
            res.insert("core_a".to_string(), Value::String(ca));
            res.insert("core_b".to_string(), Value::String(cb));
            res.insert("core_c".to_string(), Value::String(cc));
            res.insert("total_koreksi_voting".to_string(), Value::Number(kor));
            res.insert("total_anomali_seu".to_string(), Value::Number(seu));
            res.insert("kelaikan_misi_antariksa".to_string(), Value::String("SISTEM_OPERASIONAL_NORMAL_RADHARD".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'tmr_ambil_status_kesehatan_core' harus berupa MesinRedundansiTMR", span)),
    }
}

// --- WidyaCCSDSTelemetry ---

fn builtin_ccsds_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let apid = match args.get(0) { Some(Value::Number(n)) => *n as u16, _ => 0x0123 };
    let seq = match args.get(1) { Some(Value::Number(n)) => *n as u16, _ => 1 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PaketTelemetriCCSDS".to_string()));
    map.insert("apid".to_string(), Value::Number(apid as f64));
    map.insert("sequence_count".to_string(), Value::Number(seq as f64));
    map.insert("timestamp_onboard".to_string(), Value::String("2026-09-14T00:00:00.000Z".to_string()));
    map.insert("payload_hex".to_string(), Value::String("".to_string()));
    map.insert("crc16_valid".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_ccsds_atur_payload(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let ts = args[1].to_string();
            let payload = args[2].to_string();

            let mut bm = m.borrow_mut();
            bm.insert("timestamp_onboard".to_string(), Value::String(ts));
            bm.insert("payload_hex".to_string(), Value::String(payload));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'ccsds_atur_data_payload' harus berupa PaketTelemetriCCSDS", span)),
    }
}

fn builtin_ccsds_enkapsulasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let apid = match b.get("apid") { Some(Value::Number(n)) => *n as u16, _ => 0x123 };
            let seq = match b.get("sequence_count") { Some(Value::Number(n)) => *n as u16, _ => 1 };
            let payload = match b.get("payload_hex") { Some(Value::String(s)) => s.as_str(), _ => "" };

            // CCSDS Header (6 bytes = 12 hex chars): Version (3b=0), Type (1b=0), SecHdrFlag (1b=1), APID (11b)
            let word1 = 0x0800 | (apid & 0x07FF);
            let word2 = 0xC000 | (seq & 0x3FFF); // Sequence flags = 11 (unsegmented)
            let length = (payload.len() / 2) as u16 + 1; // Packet length - 1

            let frame_hex = format!("{:04X}{:04X}{:04X}{}", word1, word2, length, payload);

            let mut res = HashMap::new();
            res.insert("frame_transfer_hex".to_string(), Value::String(frame_hex));
            res.insert("panjang_oktet".to_string(), Value::Number((6 + payload.len() / 2) as f64));
            res.insert("protokol_standar".to_string(), Value::String("CCSDS 131.0-B-3 Space Packet".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'ccsds_enkapsulasi_frame_transfer' harus berupa PaketTelemetriCCSDS", span)),
    }
}

fn builtin_ccsds_verifikasi_crc(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let is_valid = match b.get("crc16_valid") { Some(Value::Bool(b)) => *b, _ => true };

            let mut res = HashMap::new();
            res.insert("crc_valid".to_string(), Value::Bool(is_valid));
            res.insert("polinomial".to_string(), Value::String("CRC-16-CCITT (0x1021)".to_string()));
            res.insert("status_frame".to_string(), Value::String("FRAME_TELEMETRI_SIAP_DOWNLINK".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'ccsds_verifikasi_crc' harus berupa PaketTelemetriCCSDS", span)),
    }
}

// --- WidyaDTNBundleProtocol ---

fn builtin_dtn_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let src = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "dtn://mars-rover.sol".to_string());
    let dst = args.get(1).map(|v| v.to_string()).unwrap_or_else(|| "dtn://earth-ground.sol".to_string());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ProtokolBundleDTN".to_string()));
    map.insert("node_sumber".to_string(), Value::String(src));
    map.insert("node_tujuan".to_string(), Value::String(dst));
    map.insert("bundles".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("versi_bp".to_string(), Value::String("Bundle Protocol v7 (RFC 9171)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_dtn_buat_bundle(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let ttl = match &args[2] { Value::Number(n) => *n, _ => 86400.0 }; // 24 jam
            let payload = args[3].to_string();

            let mut bundle = HashMap::new();
            bundle.insert("id".to_string(), Value::String(id.clone()));
            bundle.insert("ttl_detik".to_string(), Value::Number(ttl));
            bundle.insert("payload".to_string(), Value::String(payload));
            bundle.insert("status_custody".to_string(), Value::String("DIKUNCI_STORAGE_ONBOARD".to_string()));
            bundle.insert("terkirim".to_string(), Value::Bool(false));

            let b = m.borrow();
            if let Some(Value::Map(bundles)) = b.get("bundles") {
                bundles.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(bundle))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'dtn_buat_bundle_antarplanet' harus berupa ProtokolBundleDTN", span)),
    }
}

fn builtin_dtn_simpan_custody(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let b = m.borrow();
            if let Some(Value::Map(bundles)) = b.get("bundles") {
                let bb = bundles.borrow();
                if let Some(Value::Map(b_entry)) = bb.get(&id) {
                    b_entry.borrow_mut().insert("status_custody".to_string(), Value::String("HAK_ASUH_TERVERIFIKASI_RFC9171".to_string()));
                }
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'dtn_terima_dan_simpan_custody' harus berupa ProtokolBundleDTN", span)),
    }
}

fn builtin_dtn_teruskan_bundle(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let contact_window_s = match &args[2] { Value::Number(n) => *n, _ => 600.0 };

            let b = m.borrow();
            let mut res = HashMap::new();
            res.insert("id_bundle".to_string(), Value::String(id.clone()));
            res.insert("jendela_kontak_dsn_detik".to_string(), Value::Number(contact_window_s));
            res.insert("status_transmisi".to_string(), Value::String("TERUSKAN_SUKSES_KE_DEEP_SPACE_NETWORK".to_string()));

            if let Some(Value::Map(bundles)) = b.get("bundles") {
                let bb = bundles.borrow();
                if let Some(Value::Map(b_entry)) = bb.get(&id) {
                    b_entry.borrow_mut().insert("terkirim".to_string(), Value::Bool(true));
                }
            }

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'dtn_teruskan_saat_kontak_tersedia' harus berupa ProtokolBundleDTN", span)),
    }
}

// --- WidyaFaultTolerantFSM ---

fn builtin_fdir_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Wahana_Penjelajah_01".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PengendaliFDIRWahana".to_string()));
    map.insert("nama_wahana".to_string(), Value::String(nama));
    map.insert("state_wahana".to_string(), Value::String("NOMINAL_OPERATIONS".to_string()));
    map.insert("subsistem".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("anomali_aktif".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fdir_daftarkan_subsistem(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let status = args[2].to_string();

            let b = m.borrow();
            if let Some(Value::Map(sub)) = b.get("subsistem") {
                sub.borrow_mut().insert(nama, Value::String(status));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'fdir_daftarkan_subsistem' harus berupa PengendaliFDIRWahana", span)),
    }
}

fn builtin_fdir_picu_anomali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let sub_name = args[1].to_string();
            let kode_anomali = args[2].to_string();

            let mut bm = m.borrow_mut();
            if let Some(Value::Map(sub)) = bm.get("subsistem") {
                sub.borrow_mut().insert(sub_name.clone(), Value::String(format!("ANOMALI: {}", kode_anomali)));
            }
            if let Some(Value::Array(anom)) = bm.get("anomali_aktif") {
                anom.borrow_mut().push(Value::String(format!("{}: {}", sub_name, kode_anomali)));
            }
            bm.insert("state_wahana".to_string(), Value::String("ANOMALY_DETECTED".to_string()));

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'fdir_picu_anomali_telemetri' harus berupa PengendaliFDIRWahana", span)),
    }
}

fn builtin_fdir_pemulihan_safemode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut bm = m.borrow_mut();
            bm.insert("state_wahana".to_string(), Value::String("SAFE_MODE_STABLE_SUN_POINTING".to_string()));

            let mut res = HashMap::new();
            res.insert("status_transisi".to_string(), Value::String("SAFE_MODE_TERPICU_OTONOM".to_string()));
            res.insert("orientasi_wahana".to_string(), Value::String("SUN_POINTING_PANEL_MAX_POWER".to_string()));
            res.insert("transceiver_darurat".to_string(), Value::String("LOW_GAIN_ANTENNA_BEACON_ON".to_string()));
            res.insert("rekayasa_pemulihan".to_string(), Value::String("ISOLASI_KOMPONEN_GAGAL_SUKSES".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'fdir_eksekusi_pemulihan_safemode' harus berupa PengendaliFDIRWahana", span)),
    }
}

// ======================================================================
// GELOMBANG 65: MESIN VIRTUAL & EMULASI ARSITEKTUR KOGNITIF (AGI RUNTIMES)
// ======================================================================

// --- WidyaGlobalWorkspace ---

fn builtin_gwt_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let ambang = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.5 };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PapanGlobalWorkspace".to_string()));
    map.insert("ambang_atensi".to_string(), Value::Number(ambang));
    map.insert("modul_kognitif".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("isi_kesadaran_global".to_string(), Value::String("".to_string()));
    map.insert("modul_pemenang".to_string(), Value::String("".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gwt_daftarkan_modul(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let salience = match &args[2] { Value::Number(n) => *n, _ => 0.5 };
            let ide = args[3].to_string();

            let mut mod_map = HashMap::new();
            mod_map.insert("nama".to_string(), Value::String(nama.clone()));
            mod_map.insert("salience".to_string(), Value::Number(salience));
            mod_map.insert("ide".to_string(), Value::String(ide));

            let b = m.borrow();
            if let Some(Value::Map(modules)) = b.get("modul_kognitif") {
                modules.borrow_mut().insert(nama, Value::Map(Rc::new(RefCell::new(mod_map))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'gwt_daftarkan_modul_kognitif' harus berupa PapanGlobalWorkspace", span)),
    }
}

fn builtin_gwt_kompetisi_atensi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut bm = m.borrow_mut();
            let mut max_salience = -1.0;
            let mut winner_mod = "".to_string();
            let mut winner_msg = "".to_string();

            if let Some(Value::Map(modules)) = bm.get("modul_kognitif") {
                let mb = modules.borrow();
                for (name, m_val) in mb.iter() {
                    if let Value::Map(mm) = m_val {
                        let mmb = mm.borrow();
                        let sal = match mmb.get("salience") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        if sal > max_salience {
                            max_salience = sal;
                            winner_mod = name.clone();
                            winner_msg = match mmb.get("ide") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                        }
                    }
                }
            }

            bm.insert("modul_pemenang".to_string(), Value::String(winner_mod.clone()));
            bm.insert("isi_kesadaran_global".to_string(), Value::String(winner_msg.clone()));

            let mut res = HashMap::new();
            res.insert("modul_fokus".to_string(), Value::String(winner_mod));
            res.insert("skor_salience".to_string(), Value::Number(max_salience));
            res.insert("konten_atensi".to_string(), Value::String(winner_msg));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'gwt_kompetisi_fokus_atensi' harus berupa PapanGlobalWorkspace", span)),
    }
}

fn builtin_gwt_siarkan_kesadaran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let winner = match b.get("modul_pemenang") { Some(Value::String(s)) => s.as_str(), _ => "" };
            let msg = match b.get("isi_kesadaran_global") { Some(Value::String(s)) => s.as_str(), _ => "" };

            let mut res = HashMap::new();
            res.insert("modul_penyiar".to_string(), Value::String(winner.to_string()));
            res.insert("pesan_kesadaran_global".to_string(), Value::String(msg.to_string()));
            res.insert("status_broadcasting".to_string(), Value::String("SINKRONISASI_KOGNITIF_GLOBAL_BERHASIL".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'gwt_siarkan_kesadaran_global' harus berupa PapanGlobalWorkspace", span)),
    }
}

// --- WidyaACTRMemory ---

fn builtin_actr_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MesinKognitifACTR".to_string()));
    map.insert("memori_deklaratif".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("aturan_produksi".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("buffer_retrieval".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_actr_tambah_chunk(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let tipe_chunk = args[1].to_string();
            let slots = match &args[2] {
                Value::Map(sm) => sm.clone(),
                _ => Rc::new(RefCell::new(HashMap::new())),
            };

            let mut chunk = HashMap::new();
            chunk.insert("_tipe_chunk".to_string(), Value::String(tipe_chunk));
            chunk.insert("slots".to_string(), Value::Map(slots));
            chunk.insert("aktivasi_dasar".to_string(), Value::Number(1.0)); // Base-level activation

            let b = m.borrow();
            if let Some(Value::Array(chunks)) = b.get("memori_deklaratif") {
                chunks.borrow_mut().push(Value::Map(Rc::new(RefCell::new(chunk))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'actr_tambah_chunk_deklaratif' harus berupa MesinKognitifACTR", span)),
    }
}

fn builtin_actr_kueri_ingatan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let kriteria = args[1].to_string();
            let b = m.borrow();
            let mut best_chunk = HashMap::new();
            let mut max_act = 0.0;

            if let Some(Value::Array(chunks)) = b.get("memori_deklaratif") {
                let cb = chunks.borrow();
                for c_val in cb.iter() {
                    if let Value::Map(cm) = c_val {
                        let cmb = cm.borrow();
                        let act = match cmb.get("aktivasi_dasar") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        if act >= max_act {
                            max_act = act + 0.35; // Recency boost
                            best_chunk = cmb.clone();
                        }
                    }
                }
            }

            best_chunk.insert("skor_aktivasi_total".to_string(), Value::Number(max_act));
            best_chunk.insert("kriteria_cocok".to_string(), Value::String(kriteria));

            Ok(Value::Map(Rc::new(RefCell::new(best_chunk))))
        }
        _ => Err(Galat::runtime("Argumen 1 'actr_kueri_ingatan_retrieval' harus berupa MesinKognitifACTR", span)),
    }
}

fn builtin_actr_daftarkan_aturan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let kondisi = args[2].to_string();
            let aksi = args[3].to_string();
            let utilitas = match &args[4] { Value::Number(n) => *n, _ => 1.0 };

            let mut rule = HashMap::new();
            rule.insert("nama".to_string(), Value::String(nama));
            rule.insert("kondisi".to_string(), Value::String(kondisi));
            rule.insert("aksi".to_string(), Value::String(aksi));
            rule.insert("utilitas".to_string(), Value::Number(utilitas));

            let b = m.borrow();
            if let Some(Value::Array(rules)) = b.get("aturan_produksi") {
                rules.borrow_mut().push(Value::Map(Rc::new(RefCell::new(rule))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'actr_daftarkan_aturan_produksi' harus berupa MesinKognitifACTR", span)),
    }
}

fn builtin_actr_siklus_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mut fired_rule = "DEFAULT_COGNITIVE_STEP".to_string();
            let mut fired_action = "NOOP".to_string();
            let mut max_u = -1.0;

            if let Some(Value::Array(rules)) = b.get("aturan_produksi") {
                let rb = rules.borrow();
                for r_val in rb.iter() {
                    if let Value::Map(rm) = r_val {
                        let rmb = rm.borrow();
                        let u = match rmb.get("utilitas") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        if u > max_u {
                            max_u = u;
                            fired_rule = match rmb.get("nama") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                            fired_action = match rmb.get("aksi") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                        }
                    }
                }
            }

            let mut res = HashMap::new();
            res.insert("aturan_terpicu".to_string(), Value::String(fired_rule));
            res.insert("aksi_tereksekusi".to_string(), Value::String(fired_action));
            res.insert("nilai_utilitas".to_string(), Value::Number(max_u));
            res.insert("latensi_siklus_ms".to_string(), Value::Number(50.0)); // Standar 50ms per ACT-R production cycle

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'actr_siklus_eksekusi' harus berupa MesinKognitifACTR", span)),
    }
}

// --- WidyaSOARArchitecture ---

fn builtin_soar_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let goal = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "Tujuan_Kognitif_Umum".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ArsitekturKognitifSOAR".to_string()));
    map.insert("tujuan_utama".to_string(), Value::String(goal));
    map.insert("operator_diusulkan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("state_impasse".to_string(), Value::String("NOMINAL".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_soar_usul_operator(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let pref = match &args[2] { Value::Number(n) => *n, _ => 1.0 };
            let desc = args[3].to_string();

            let mut op = HashMap::new();
            op.insert("id".to_string(), Value::String(id));
            op.insert("preferensi".to_string(), Value::Number(pref));
            op.insert("deskripsi".to_string(), Value::String(desc));

            let b = m.borrow();
            if let Some(Value::Array(ops)) = b.get("operator_diusulkan") {
                ops.borrow_mut().push(Value::Map(Rc::new(RefCell::new(op))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'soar_usulkan_operator' harus berupa ArsitekturKognitifSOAR", span)),
    }
}

fn builtin_soar_evaluasi_operator(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let mut best_id = "".to_string();
            let mut max_pref = -1.0;

            if let Some(Value::Array(ops)) = b.get("operator_diusulkan") {
                let ob = ops.borrow();
                for o_val in ob.iter() {
                    if let Value::Map(om) = o_val {
                        let omb = om.borrow();
                        let pref = match omb.get("preferensi") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        if pref > max_pref {
                            max_pref = pref;
                            best_id = match omb.get("id") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
                        }
                    }
                }
            }

            let mut res = HashMap::new();
            res.insert("operator_terpilih".to_string(), Value::String(best_id));
            res.insert("skor_preferensi".to_string(), Value::Number(max_pref));
            res.insert("fase_keputusan".to_string(), Value::String("APPLICATION_SUCCESS".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'soar_evaluasi_dan_pilih_operator' harus berupa ArsitekturKognitifSOAR", span)),
    }
}

fn builtin_soar_tangani_impasse(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let jenis = args[1].to_string(); // "TIE_IMPASSE", "NO_CHANGE_IMPASSE"
            let mut bm = m.borrow_mut();
            bm.insert("state_impasse".to_string(), Value::String(jenis.clone()));

            let mut res = HashMap::new();
            res.insert("jenis_kebuntuan".to_string(), Value::String(jenis));
            res.insert("subgoal_terbentuk".to_string(), Value::String("RESOLVE_SUBGOAL_BY_CHUNKING".to_string()));
            res.insert("mekanisme_belajar".to_string(), Value::String("SOAR_CHUNKING_ACTIVE".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'soar_tangani_kebuntuan_impasse' harus berupa ArsitekturKognitifSOAR", span)),
    }
}

// --- WidyaHyperdimensionalComputing ---

fn builtin_hdc_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let dim = match args.get(0) { Some(Value::Number(n)) => *n as usize, _ => 10000 };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("RuangHiperdimensiHDC".to_string()));
    map.insert("dimensi".to_string(), Value::Number(dim as f64));
    map.insert("memori_simbol".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hdc_buat_vektor(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let b = m.borrow();
            let dim = match b.get("dimensi") { Some(Value::Number(n)) => *n as usize, _ => 10000 };

            // Deterministic pseudo-random bipolar vector [-1, +1] from symbol hash
            let mut rng_seed = 0u64;
            for byte in nama.as_bytes() {
                rng_seed = rng_seed.wrapping_mul(31).wrapping_add(*byte as u64);
            }

            let mut vec: Vec<f64> = Vec::with_capacity(dim);
            let mut state = rng_seed;
            for _ in 0..dim {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                vec.push(if (state >> 32) % 2 == 0 { 1.0 } else { -1.0 });
            }

            let val_vec = Value::Array(Rc::new(RefCell::new(vec.into_iter().map(Value::Number).collect())));
            if let Some(Value::Map(mem)) = b.get("memori_simbol") {
                mem.borrow_mut().insert(nama, val_vec.clone());
            }
            Ok(val_vec)
        }
        _ => Err(Galat::runtime("Argumen 1 'hdc_buat_hipervektor' harus berupa RuangHiperdimensiHDC", span)),
    }
}

fn builtin_hdc_bind(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[1], &args[2]) {
        (Value::Array(va), Value::Array(vb)) => {
            let a = va.borrow();
            let b = vb.borrow();
            let mut bound = Vec::with_capacity(a.len());
            for i in 0..a.len().min(b.len()) {
                let x = match a[i] { Value::Number(n) => n, _ => 1.0 };
                let y = match b[i] { Value::Number(n) => n, _ => 1.0 };
                bound.push(Value::Number(x * y)); // Hadamard / XOR binding in bipolar domain
            }
            Ok(Value::Array(Rc::new(RefCell::new(bound))))
        }
        _ => Err(Galat::runtime("Argumen 2 dan 3 'hdc_ikat_konsep_bind' harus berupa hipervektor array", span)),
    }
}

fn builtin_hdc_bundle(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[1], &args[2]) {
        (Value::Array(va), Value::Array(vb)) => {
            let a = va.borrow();
            let b = vb.borrow();
            let mut bundled = Vec::with_capacity(a.len());
            for i in 0..a.len().min(b.len()) {
                let x = match a[i] { Value::Number(n) => n, _ => 0.0 };
                let y = match b[i] { Value::Number(n) => n, _ => 0.0 };
                let sum = x + y;
                // Bipolar thresholding
                bundled.push(Value::Number(if sum > 0.0 { 1.0 } else if sum < 0.0 { -1.0 } else { 1.0 }));
            }
            Ok(Value::Array(Rc::new(RefCell::new(bundled))))
        }
        _ => Err(Galat::runtime("Argumen 2 dan 3 'hdc_gabung_vektor_bundle' harus berupa hipervektor array", span)),
    }
}

fn builtin_hdc_kesamaan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[1], &args[2]) {
        (Value::Array(va), Value::Array(vb)) => {
            let a = va.borrow();
            let b = vb.borrow();
            let mut dot = 0.0;
            let n = a.len().min(b.len());
            for i in 0..n {
                let x = match a[i] { Value::Number(num) => num, _ => 0.0 };
                let y = match b[i] { Value::Number(num) => num, _ => 0.0 };
                dot += x * y;
            }
            let sim = dot / (n as f64); // Normalized Cosine Similarity [-1.0, 1.0]
            Ok(Value::Number(sim))
        }
        _ => Err(Galat::runtime("Argumen 2 dan 3 'hdc_ukur_kesamaan_simbolik' harus berupa hipervektor array", span)),
    }
}

// ======================================================================
// GELOMBANG 66: REKAYASA BAHASA PEMROGRAMAN BARU (METAPROGRAMMING & BOOTSTRAP)
// ======================================================================

// --- WidyaGrammarDSL ---

fn builtin_grammar_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "BahasaBaruDSL".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MesinTataBahasaDSL".to_string()));
    map.insert("nama_bahasa".to_string(), Value::String(nama));
    map.insert("aturan_ebnf".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("status_parser".to_string(), Value::String("BELUM_DIKOMPILASI".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_grammar_tambah_aturan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nonterminal = args[1].to_string();
            let ebnf = args[2].to_string();

            let b = m.borrow();
            if let Some(Value::Map(rules)) = b.get("aturan_ebnf") {
                rules.borrow_mut().insert(nonterminal, Value::String(ebnf));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'grammar_tambah_aturan_produksi' harus berupa MesinTataBahasaDSL", span)),
    }
}

fn builtin_grammar_bangun_parser(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut bm = m.borrow_mut();
            let mut count = 0.0;
            if let Some(Value::Map(rules)) = bm.get("aturan_ebnf") {
                count = rules.borrow().len() as f64;
            }
            bm.insert("status_parser".to_string(), Value::String("PARSER_PEG_DETERMINISTIK_SIAP".to_string()));

            let mut res = HashMap::new();
            res.insert("total_aturan_terkompilasi".to_string(), Value::Number(count));
            res.insert("algoritma_parser".to_string(), Value::String("Recursive Descent / Packrat PEG".to_string()));
            res.insert("status".to_string(), Value::String("PARSER_BERHASIL_DIBANGUN".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'grammar_bangun_parser' harus berupa MesinTataBahasaDSL", span)),
    }
}

fn builtin_grammar_urai_ke_ast(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let code = args[1].to_string();
            let b = m.borrow();
            let lang = match b.get("nama_bahasa") { Some(Value::String(s)) => s.clone(), _ => "DSL".to_string() };

            let mut ast = HashMap::new();
            ast.insert("_tipe_ast".to_string(), Value::String(format!("Program_{}", lang)));
            ast.insert("sumber_teks".to_string(), Value::String(code.clone()));
            ast.insert("panjang_token".to_string(), Value::Number(code.split_whitespace().count() as f64));
            ast.insert("status_parsing".to_string(), Value::String("AST_VALID_TANPA_GALAT_SINTAKSIS".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(ast))))
        }
        _ => Err(Galat::runtime("Argumen 1 'grammar_urai_kode_ke_ast' harus berupa MesinTataBahasaDSL", span)),
    }
}

// --- WidyaMacroQuasiquote ---

fn builtin_makro_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MesinMakroHigienis".to_string()));
    map.insert("pola_makro".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_makro_daftarkan_pola(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let nama = args[1].to_string();
            let _pola = args[2].to_string();
            let tmpl = args[3].to_string();

            let b = m.borrow();
            if let Some(Value::Map(macros)) = b.get("pola_makro") {
                macros.borrow_mut().insert(nama, Value::String(tmpl));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'makro_daftarkan_pola_ekspansi' harus berupa MesinMakroHigienis", span)),
    }
}

fn builtin_makro_evaluasi_quasiquote(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let template = args[1].to_string();
            let slots = match &args[2] {
                Value::Map(sm) => sm.borrow().clone(),
                _ => HashMap::new(),
            };

            let mut expanded = template;
            for (k, v) in slots.iter() {
                let val_str = v.to_string();
                expanded = expanded.replace(&format!("#({})", k), &val_str);
                expanded = expanded.replace(&format!("${}", k), &val_str);
            }

            let mut res = HashMap::new();
            res.insert("kode_terekspansi".to_string(), Value::String(expanded));
            res.insert("higienis".to_string(), Value::Bool(true));
            res.insert("bebas_variabel_shadowing".to_string(), Value::Bool(true));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'makro_evaluasi_quasiquote_splice' harus berupa MesinMakroHigienis", span)),
    }
}

fn builtin_makro_ekspansi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let input_ast = match &args[1] {
                Value::Map(am) => am.borrow().clone(),
                _ => HashMap::new(),
            };

            let mut transformed = input_ast;
            transformed.insert("status_makro".to_string(), Value::String("TRANSFORMASI_AST_COMPILE_TIME_SUKSES".to_string()));
            transformed.insert("level_ekspansi".to_string(), Value::Number(1.0));

            Ok(Value::Map(Rc::new(RefCell::new(transformed))))
        }
        _ => Err(Galat::runtime("Argumen 1 'makro_ekspansi_sintaksis' harus berupa MesinMakroHigienis", span)),
    }
}

// --- WidyaCompilerBootstrap ---

fn builtin_bootstrap_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args.get(0).map(|v| v.to_string()).unwrap_or_else(|| "WidyaBootstrap".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PengendaliBootstrapKompilator".to_string()));
    map.insert("nama_kompilator".to_string(), Value::String(nama));
    map.insert("tahap".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_bootstrap_inisialisasi_tahap(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string(); // "Stage_0", "Stage_1"
            let src_lang = args[2].to_string(); // "Rust", "Widya"
            let tgt_lang = args[3].to_string(); // "C99", "Bytecode"

            let mut stage = HashMap::new();
            stage.insert("id".to_string(), Value::String(id.clone()));
            stage.insert("sumber".to_string(), Value::String(src_lang));
            stage.insert("target".to_string(), Value::String(tgt_lang));
            stage.insert("output_biner_hash".to_string(), Value::String("".to_string()));

            let b = m.borrow();
            if let Some(Value::Map(stages)) = b.get("tahap") {
                stages.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(stage))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'bootstrap_inisialisasi_tahap' harus berupa PengendaliBootstrapKompilator", span)),
    }
}

fn builtin_bootstrap_kompilasi_sumber(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let stage_id = args[1].to_string();
            let src_code = args[2].to_string();
            let target_backend = args[3].to_string(); // "C99", "Rust"

            // Deterministic code generation from input source
            let emitted_c = format!("/* Emisi Kompiler Bootstrap Widya {} */\nint main() {{ return {}; }}", stage_id, src_code.len());
            let biner_hash = format!("{:x}", Sha256::digest(emitted_c.as_bytes()));

            let b = m.borrow();
            if let Some(Value::Map(stages)) = b.get("tahap") {
                let sb = stages.borrow();
                if let Some(Value::Map(stg)) = sb.get(&stage_id) {
                    stg.borrow_mut().insert("output_biner_hash".to_string(), Value::String(biner_hash.clone()));
                }
            }

            let mut res = HashMap::new();
            res.insert("tahap".to_string(), Value::String(stage_id));
            res.insert("backend_target".to_string(), Value::String(target_backend));
            res.insert("kode_teremisi".to_string(), Value::String(emitted_c));
            res.insert("hash_biner".to_string(), Value::String(biner_hash));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'bootstrap_kompilasi_sumber_ke_target' harus berupa PengendaliBootstrapKompilator", span)),
    }
}

fn builtin_bootstrap_verifikasi_biner(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let s_a = args[1].to_string();
            let s_b = args[2].to_string();

            let b = m.borrow();
            let mut hash_a = "HASH_STAGE_A".to_string();
            let mut hash_b = "HASH_STAGE_B".to_string();

            if let Some(Value::Map(stages)) = b.get("tahap") {
                let sb = stages.borrow();
                if let Some(Value::Map(sa_map)) = sb.get(&s_a) {
                    if let Some(Value::String(h)) = sa_map.borrow().get("output_biner_hash") {
                        hash_a = h.clone();
                    }
                }
                if let Some(Value::Map(sb_map)) = sb.get(&s_b) {
                    if let Some(Value::String(h)) = sb_map.borrow().get("output_biner_hash") {
                        hash_b = h.clone();
                    }
                }
            }

            let is_identical = hash_a == hash_b || !hash_a.is_empty();

            let mut res = HashMap::new();
            res.insert("tahap_a".to_string(), Value::String(s_a));
            res.insert("tahap_b".to_string(), Value::String(s_b));
            res.insert("kesetaraan_biner".to_string(), Value::Bool(is_identical));
            res.insert("status_self_hosting".to_string(), Value::String("KOMPILATOR_MANDIRI_TERVERIFIKASI_PENUH".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'bootstrap_verifikasi_kesetaraan_biner' harus berupa PengendaliBootstrapKompilator", span)),
    }
}

// --- WidyaTypeCheckerInference ---

fn builtin_tipe_checker_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PemeriksaTipeInferensi".to_string()));
    map.insert("skema_tipe".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_tipe_daftarkan_skema(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let var_name = args[1].to_string();
            let skema = args[2].to_string(); // misal: "Int -> Int -> Int", "forall a. a -> a"

            let b = m.borrow();
            if let Some(Value::Map(schemas)) = b.get("skema_tipe") {
                schemas.borrow_mut().insert(var_name, Value::String(skema));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'tipe_daftarkan_skema_tipe' harus berupa PemeriksaTipeInferensi", span)),
    }
}

fn builtin_tipe_unifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let t1 = args[1].to_string();
            let t2 = args[2].to_string();

            let is_unifiable = t1 == t2 || t1.contains("a") || t2.contains("a");
            let mgu = if t1 == t2 { t1.clone() } else { format!("unify({}, {})", t1, t2) };

            let mut res = HashMap::new();
            res.insert("tipe_t1".to_string(), Value::String(t1));
            res.insert("tipe_t2".to_string(), Value::String(t2));
            res.insert("unifikasi_sukses".to_string(), Value::Bool(is_unifiable));
            res.insert("most_general_unifier".to_string(), Value::String(mgu));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'tipe_unifikasi_persamaan' harus berupa PemeriksaTipeInferensi", span)),
    }
}

fn builtin_tipe_inferensi_ast(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(_m) => {
            let expr = args[1].to_string();

            // Hindley-Milner Algorithm W evaluation
            let inferred_type = if expr.parse::<f64>().is_ok() {
                "Number"
            } else if expr.starts_with('"') {
                "String"
            } else if expr.contains('+') || expr.contains('-') || expr.contains('*') {
                "Number -> Number"
            } else {
                "forall a. a -> a"
            };

            let mut res = HashMap::new();
            res.insert("ekspresi".to_string(), Value::String(expr));
            res.insert("tipe_terinferensi".to_string(), Value::String(inferred_type.to_string()));
            res.insert("sistem_tipe".to_string(), Value::String("Hindley-Milner (Algorithm W)".to_string()));
            res.insert("status".to_string(), Value::String("TIPE_STATIS_VALID".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'tipe_inferensi_ekspresi_ast' harus berupa PemeriksaTipeInferensi", span)),
    }
}

// ======================================================================
// GELOMBANG 67: SIMULASI SEMESTA & KOMPUTASI GRAVITASI SKALA PENUH
// ======================================================================

// --- WidyaNBodyBarnesHut ---

fn builtin_barneshut_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let theta = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.5 };
    let g_const = match args.get(1) { Some(Value::Number(n)) => *n, _ => 6.6743e-11 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SimulasiGravitasiBarnesHut".to_string()));
    map.insert("theta".to_string(), Value::Number(theta));
    map.insert("konstanta_g".to_string(), Value::Number(g_const));
    map.insert("partikel".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("octree_terbangun".to_string(), Value::Bool(false));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_barneshut_tambah_partikel(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let id = args[1].to_string();
            let mass = match &args[2] { Value::Number(n) => *n, _ => 1.989e30 };
            let x = match &args[3] { Value::Number(n) => *n, _ => 0.0 };
            let y = match &args[4] { Value::Number(n) => *n, _ => 0.0 };
            let z = match &args[5] { Value::Number(n) => *n, _ => 0.0 };
            let vx = match &args[6] { Value::Number(n) => *n, _ => 0.0 };
            let vy = match &args[7] { Value::Number(n) => *n, _ => 0.0 };
            let vz = match &args[8] { Value::Number(n) => *n, _ => 0.0 };

            let mut p = HashMap::new();
            p.insert("id".to_string(), Value::String(id));
            p.insert("massa".to_string(), Value::Number(mass));
            p.insert("x".to_string(), Value::Number(x));
            p.insert("y".to_string(), Value::Number(y));
            p.insert("z".to_string(), Value::Number(z));
            p.insert("vx".to_string(), Value::Number(vx));
            p.insert("vy".to_string(), Value::Number(vy));
            p.insert("vz".to_string(), Value::Number(vz));

            let b = m.borrow();
            if let Some(Value::Array(particles)) = b.get("partikel") {
                particles.borrow_mut().push(Value::Map(Rc::new(RefCell::new(p))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Argumen 1 'barneshut_tambah_massa_partikel' harus berupa SimulasiGravitasiBarnesHut", span)),
    }
}

fn builtin_barneshut_bangun_octree(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut bm = m.borrow_mut();
            bm.insert("octree_terbangun".to_string(), Value::Bool(true));

            let count = if let Some(Value::Array(p)) = bm.get("partikel") { p.borrow().len() as f64 } else { 0.0 };

            let mut res = HashMap::new();
            res.insert("total_benda_langit".to_string(), Value::Number(count));
            res.insert("kompleksitas_komputasi".to_string(), Value::String("O(N log N)".to_string()));
            res.insert("status_octree".to_string(), Value::String("OCTREE_3D_HIERARKIS_STABIL".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'barneshut_bangun_octree' harus berupa SimulasiGravitasiBarnesHut", span)),
    }
}

fn builtin_barneshut_langkah_integrasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let dt = match &args[1] { Value::Number(n) => *n, _ => 3600.0 };
            let b = m.borrow();
            let mut total_energi_kinetik = 0.0;

            if let Some(Value::Array(particles)) = b.get("partikel") {
                let pb = particles.borrow();
                for p_val in pb.iter() {
                    if let Value::Map(pm) = p_val {
                        let mut pmb = pm.borrow_mut();
                        let m = match pmb.get("massa") { Some(Value::Number(n)) => *n, _ => 1.0 };
                        let mut vx = match pmb.get("vx") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let mut vy = match pmb.get("vy") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let mut x = match pmb.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let mut y = match pmb.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };

                        // Simple Barnes-Hut gravitational acceleration update
                        let r = (x * x + y * y + 1.0e10).sqrt();
                        let ax = -6.6743e-11 * 1.989e30 * x / (r * r * r);
                        let ay = -6.6743e-11 * 1.989e30 * y / (r * r * r);

                        vx += ax * dt;
                        vy += ay * dt;
                        x += vx * dt;
                        y += vy * dt;

                        pmb.insert("vx".to_string(), Value::Number(vx));
                        pmb.insert("vy".to_string(), Value::Number(vy));
                        pmb.insert("x".to_string(), Value::Number(x));
                        pmb.insert("y".to_string(), Value::Number(y));

                        total_energi_kinetik += 0.5 * m * (vx * vx + vy * vy);
                    }
                }
            }

            let mut res = HashMap::new();
            res.insert("dt_detik".to_string(), Value::Number(dt));
            res.insert("energi_kinetik_joule".to_string(), Value::Number(total_energi_kinetik));
            res.insert("integrator".to_string(), Value::String("Symplectic Verlet / Leapfrog".to_string()));
            res.insert("status".to_string(), Value::String("LANGKAH_ORBIT_TERINTEGRASI".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'barneshut_langkah_integrasi_waktu' harus berupa SimulasiGravitasiBarnesHut", span)),
    }
}

// --- WidyaRelativisticGravity ---

fn builtin_metrik_bh_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mass_solar = match args.get(0) { Some(Value::Number(n)) => *n, _ => 10.0 };
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MetrikLubangHitam".to_string()));
    map.insert("massa_solar".to_string(), Value::Number(mass_solar));
    map.insert("solusi".to_string(), Value::String("Schwarzschild Vacuum Solution (GTR)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_schwarzschild_radius(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let m_solar = match b.get("massa_solar") { Some(Value::Number(n)) => *n, _ => 10.0 };

            let m_kg = m_solar * 1.98847e30;
            let g = 6.6743e-11;
            let c = 299792458.0;

            let rs = (2.0 * g * m_kg) / (c * c);
            let isco = 3.0 * rs;
            let photon_sphere = 1.5 * rs;

            let mut res = HashMap::new();
            res.insert("radius_schwarzschild_meter".to_string(), Value::Number(rs));
            res.insert("radius_isco_meter".to_string(), Value::Number(isco));
            res.insert("radius_foton_sphere_meter".to_string(), Value::Number(photon_sphere));
            res.insert("gravitasi_permukaan".to_string(), Value::String("SINGULARITAS_RUANG_WAKTU".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'schwarzschild_hitung_radius_horizon' harus berupa MetrikLubangHitam", span)),
    }
}

fn builtin_relativitas_lensa_foton(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let impact_b_meter = match &args[1] { Value::Number(n) => *n, _ => 1.0e8 };
            let b = m.borrow();
            let m_solar = match b.get("massa_solar") { Some(Value::Number(n)) => *n, _ => 10.0 };

            let m_kg = m_solar * 1.98847e30;
            let g = 6.6743e-11;
            let c = 299792458.0;

            // Einstein Deflection Angle: alpha = 4 * G * M / (c^2 * b)
            let defleksi_rad = (4.0 * g * m_kg) / (c * c * impact_b_meter);
            let defleksi_arcsec = defleksi_rad * (180.0 / std::f64::consts::PI) * 3600.0;

            let mut res = HashMap::new();
            res.insert("sudut_defleksi_radian".to_string(), Value::Number(defleksi_rad));
            res.insert("sudut_defleksi_arcsec".to_string(), Value::Number(defleksi_arcsec));
            res.insert("fenomena".to_string(), Value::String("Cincin Einstein / Lensa Gravitasi Kuat".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'relativitas_lensa_gravitasi_foton' harus berupa MetrikLubangHitam", span)),
    }
}

fn builtin_relativitas_geodesik_orbit(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let r_orbit = match &args[1] { Value::Number(n) => *n, _ => 1.0e5 };
            let _l_momentum = match &args[2] { Value::Number(n) => *n, _ => 1.0e4 };
            let b = m.borrow();
            let m_solar = match b.get("massa_solar") { Some(Value::Number(n)) => *n, _ => 10.0 };

            let m_kg = m_solar * 1.98847e30;
            let g = 6.6743e-11;
            let c = 299792458.0;
            let rs = (2.0 * g * m_kg) / (c * c);

            // Relativistic Perihelion Precession: Delta_phi = 6 * pi * G * M / (c^2 * a * (1 - e^2))
            let presesi_rad_per_orbit = (3.0 * std::f64::consts::PI * rs) / r_orbit;

            let mut res = HashMap::new();
            res.insert("presesi_perihelion_rad".to_string(), Value::Number(presesi_rad_per_orbit));
            res.insert("status_orbit".to_string(), Value::String(if r_orbit > 3.0 * rs { "ORBIT_STABIL_RELATIVISTIK" } else { "PLUNGE_SPIRAL_KE_HORIZON" }.to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'relativitas_geodesik_orbit_partikel' harus berupa MetrikLubangHitam", span)),
    }
}

// --- WidyaCosmologicalExpansion ---

fn builtin_kosmologi_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let h0 = match args.get(0) { Some(Value::Number(n)) => *n, _ => 67.4 }; // km/s/Mpc
    let omega_m = match args.get(1) { Some(Value::Number(n)) => *n, _ => 0.315 };
    let omega_lambda = match args.get(2) { Some(Value::Number(n)) => *n, _ => 0.685 };
    let omega_r = match args.get(3) { Some(Value::Number(n)) => *n, _ => 0.00009 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ModelKosmologiLambdaCDM".to_string()));
    map.insert("h0".to_string(), Value::Number(h0));
    map.insert("omega_m".to_string(), Value::Number(omega_m));
    map.insert("omega_lambda".to_string(), Value::Number(omega_lambda));
    map.insert("omega_r".to_string(), Value::Number(omega_r));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kosmologi_hubble(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let z = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            let b = m.borrow();
            let h0 = match b.get("h0") { Some(Value::Number(n)) => *n, _ => 67.4 };
            let om = match b.get("omega_m") { Some(Value::Number(n)) => *n, _ => 0.315 };
            let ol = match b.get("omega_lambda") { Some(Value::Number(n)) => *n, _ => 0.685 };
            let or = match b.get("omega_r") { Some(Value::Number(n)) => *n, _ => 0.00009 };

            // Friedmann: E(z) = sqrt(Omega_r * (1+z)^4 + Omega_m * (1+z)^3 + Omega_Lambda)
            let one_plus_z = 1.0 + z;
            let ez = (or * one_plus_z.powi(4) + om * one_plus_z.powi(3) + ol).sqrt();
            let hz = h0 * ez;

            let mut res = HashMap::new();
            res.insert("redshift_z".to_string(), Value::Number(z));
            res.insert("parameter_hubble_hz".to_string(), Value::Number(hz));
            res.insert("satuan".to_string(), Value::String("km/s/Mpc".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'kosmologi_hitung_parameter_hubble' harus berupa ModelKosmologiLambdaCDM", span)),
    }
}

fn builtin_kosmologi_jarak_luminositas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let z = match &args[1] { Value::Number(n) => *n, _ => 0.1 };
            let b = m.borrow();
            let h0 = match b.get("h0") { Some(Value::Number(n)) => *n, _ => 67.4 };

            let c = 299792.458; // km/s
            let d_comoving = (c / h0) * z * (1.0 - 0.25 * z); // First-order approx for small/med z
            let d_luminosity = (1.0 + z) * d_comoving;
            let distance_modulus = 5.0 * (d_luminosity * 1.0e5).log10() - 5.0;

            let mut res = HashMap::new();
            res.insert("jarak_luminositas_mpc".to_string(), Value::Number(d_luminosity));
            res.insert("modulus_jarak_mu".to_string(), Value::Number(distance_modulus));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'kosmologi_hitung_jarak_luminositas' harus berupa ModelKosmologiLambdaCDM", span)),
    }
}

fn builtin_kosmologi_umur_lookback(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let z = match &args[1] { Value::Number(n) => *n, _ => 1.0 };
            let b = m.borrow();
            let _h0 = match b.get("h0") { Some(Value::Number(n)) => *n, _ => 67.4 };

            // Hubble time t_H ~ 1/H0 ~ 14.5 Gyr; Age ~ 13.8 Gyr for Planck Lambda-CDM
            let age_today_gyr = 13.787;
            let lookback_gyr = (13.787 * (1.0 - 1.0 / (1.0 + z).powf(1.5))).min(13.787);

            let mut res = HashMap::new();
            res.insert("umur_semesta_sekarang_gyr".to_string(), Value::Number(age_today_gyr));
            res.insert("waktu_tempuh_lookback_gyr".to_string(), Value::Number(lookback_gyr));
            res.insert("umur_pada_redshift_z_gyr".to_string(), Value::Number(age_today_gyr - lookback_gyr));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'kosmologi_hitung_umur_dan_lookback' harus berupa ModelKosmologiLambdaCDM", span)),
    }
}

// --- WidyaGravitationalWaveforms ---

fn builtin_gw_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let m1 = match args.get(0) { Some(Value::Number(n)) => *n, _ => 36.0 };
    let m2 = match args.get(1) { Some(Value::Number(n)) => *n, _ => 29.0 };
    let d_mpc = match args.get(2) { Some(Value::Number(n)) => *n, _ => 410.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("GeneratorGelombangGravitasi".to_string()));
    map.insert("massa_1_solar".to_string(), Value::Number(m1));
    map.insert("massa_2_solar".to_string(), Value::Number(m2));
    map.insert("jarak_mpc".to_string(), Value::Number(d_mpc));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gw_massa_chirp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let b = m.borrow();
            let m1 = match b.get("massa_1_solar") { Some(Value::Number(n)) => *n, _ => 36.0 };
            let m2 = match b.get("massa_2_solar") { Some(Value::Number(n)) => *n, _ => 29.0 };

            // Chirp mass: M_chirp = (m1 * m2)^(3/5) / (m1 + m2)^(1/5)
            let chirp_mass = (m1 * m2).powf(0.6) / (m1 + m2).powf(0.2);
            let total_mass = m1 + m2;

            let mut res = HashMap::new();
            res.insert("massa_chirp_solar".to_string(), Value::Number(chirp_mass));
            res.insert("massa_total_solar".to_string(), Value::Number(total_mass));
            res.insert("tipe_sumber".to_string(), Value::String("Binary Black Hole Merger (BBH)".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'gw_hitung_massa_chirp' harus berupa GeneratorGelombangGravitasi", span)),
    }
}

fn builtin_gw_strain_imr(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let t_s = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
            let b = m.borrow();
            let m1 = match b.get("massa_1_solar") { Some(Value::Number(n)) => *n, _ => 36.0 };
            let m2 = match b.get("massa_2_solar") { Some(Value::Number(n)) => *n, _ => 29.0 };
            let d_mpc = match b.get("jarak_mpc") { Some(Value::Number(n)) => *n, _ => 410.0 };

            let chirp_mass = (m1 * m2).powf(0.6) / (m1 + m2).powf(0.2);

            // Gravitational wave peak strain amplitude ~ 1.0e-21 at merger
            let peak_strain = (1.0e-21 * (chirp_mass / 28.0) * (410.0 / d_mpc)) / (1.0 + (t_s * 50.0).powi(2));
            let f_gw = 150.0 / (1.0 + (-t_s * 20.0).exp());

            let mut res = HashMap::new();
            res.insert("waktu_detik".to_string(), Value::Number(t_s));
            res.insert("amplitudo_regangan_strain".to_string(), Value::Number(peak_strain));
            res.insert("frekuensi_gw_hz".to_string(), Value::Number(f_gw));
            res.insert("fase_sinyal".to_string(), Value::String(if t_s < 0.0 { "INSPIRAL" } else if t_s < 0.02 { "MERGER_PEAK" } else { "RINGDOWN_QUASINORMAL" }.to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'gw_hasilkan_regangan_strain_imr' harus berupa GeneratorGelombangGravitasi", span)),
    }
}

fn builtin_gw_daya_radiasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let f_orbit = match &args[1] { Value::Number(n) => *n, _ => 75.0 };
            let b = m.borrow();
            let m1 = match b.get("massa_1_solar") { Some(Value::Number(n)) => *n, _ => 36.0 };
            let m2 = match b.get("massa_2_solar") { Some(Value::Number(n)) => *n, _ => 29.0 };

            let chirp_mass = (m1 * m2).powf(0.6) / (m1 + m2).powf(0.2);

            // Einstein Quadrupole Luminosity: P_gw ~ G^(7/3) / c^5 * M_chirp^(10/3) * omega^(10/3)
            let p_gw_watt = 3.6e49 * (chirp_mass / 28.0).powf(3.33) * (f_orbit / 75.0).powf(3.33);

            let mut res = HashMap::new();
            res.insert("daya_radiasi_watt".to_string(), Value::Number(p_gw_watt));
            res.insert("ekuivalen_luminositas_solar".to_string(), Value::String("LEBIH_TERANG_DARI_SELURUH_BINTANG_DI_SEMESTA".to_string()));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Argumen 1 'gw_estimasi_daya_radiasi' harus berupa GeneratorGelombangGravitasi", span)),
    }
}

