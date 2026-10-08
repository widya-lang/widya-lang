use std::fs;
use widya::jalankan;

#[test]
fn test_contoh_187_to_194_i18n_nusantara_and_time() {
    let contoh_files = [
        "contoh/187_pengaturan_editing_teks_arab_rtl.wya",
        "contoh/188_sistem_lokalisasi_i18n_multibahasa.wya",
        "contoh/189_bahasa_daerah_dan_aksara_nusantara.wya",
        "contoh/190_pengaturan_keyboard_aksara_nusantara.wya",
        "contoh/191_aksara_dunia_dan_transliterasi.wya",
        "contoh/192_ensiklopedia_aksara_dunia_dan_kuno.wya",
        "contoh/193_pengaturan_waktu_dunia_dan_timezone.wya",
        "contoh/194_arsitektur_waktu_utc_monotonic_dst_smearing.wya",
    ];

    for path in contoh_files {
        let code = fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));
        let res = jalankan(&code);
        assert!(res.is_ok(), "Execution failed for {}: {:?}", path, res.err());
    }
}

#[test]
fn test_contoh_195_to_200_frontier_system_pillars() {
    let contoh_files = [
        "contoh/195_lima_pilar_rekayasa_sistem_dan_arsitektur_modern.wya",
        "contoh/196_infrastruktur_protokol_observabilitas_dan_tooling_lsp.wya",
        "contoh/197_kompilasi_wasm_simd_gpu_shaping_dan_verifikasi_formal.wya",
        "contoh/198_komputasi_kuantum_crdt_green_coding_dan_dna.wya",
        "contoh/199_ai_native_autodiff_slsa_dan_neuromorphic_fotonik.wya",
        "contoh/200_spatial_bci_biocomputing_dan_smt_solver.wya",
    ];

    for path in contoh_files {
        let code = fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));
        let res = jalankan(&code);
        assert!(res.is_ok(), "Execution failed for {}: {:?}", path, res.err());
    }
}

#[test]
fn test_contoh_201_to_205_web_desktop_mobile_enterprise() {
    let contoh_files = [
        "contoh/201_widyaweb_enterprise_fullstack_dan_keamanan.wya",
        "contoh/202_widyaweb_saas_caching_task_queue_dan_sharding.wya",
        "contoh/203_widyadesktop_native_multi_window_dan_hardware.wya",
        "contoh/204_widyadesktop_deep_shell_power_dan_embedded_db.wya",
        "contoh/205_widyamobile_touch_lifecycle_hardware_graphics_dan_security.wya",
    ];

    for path in contoh_files {
        let code = fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));
        let res = jalankan(&code);
        assert!(res.is_ok(), "Execution failed for {}: {:?}", path, res.err());
    }
}

#[test]
fn test_contoh_206_to_207_widyaos_and_widyablok() {
    let contoh_files = [
        "contoh/206_widyaos_baremetal_kernel_6_pilar.wya",
        "contoh/207_widyablok_visual_scratch_engine.wya",
    ];

    for path in contoh_files {
        let code = fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));
        let res = jalankan(&code);
        assert!(res.is_ok(), "Execution failed for {}: {:?}", path, res.err());
    }
}
