use std::fs;
use std::path::Path;
use widya::mobile::{
    inisialisasi_proyek_mobile_dengan_template,
    periksa_aplikasi_mobile,
    bangun_aplikasi_mobile,
    generate_pwa_manifest,
    generate_service_worker,
    MobileCompiler,
};

#[test]
fn test_mobile_project_scaffolding_all_templates() {
    let tmp_dir = std::env::temp_dir().join("widya_mobile_test_templates");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).unwrap();

    let templates = ["standard", "pos", "presensi", "dashboard"];

    for t in &templates {
        let app_name = format!("App_{}", t);
        let project_path = inisialisasi_proyek_mobile_dengan_template(&app_name, Some(&tmp_dir), t).unwrap();

        assert!(project_path.exists(), "Direktori proyek harus ada");
        assert!(project_path.join("src").join("main.wya").exists(), "main.wya harus ada");
        assert!(project_path.join("widya.json").exists(), "widya.json harus ada");
        assert!(project_path.join("android").exists(), "android scaffolding harus ada");
        assert!(project_path.join("ios").exists(), "ios scaffolding harus ada");

        let code = fs::read_to_string(project_path.join("src").join("main.wya")).unwrap();
        assert!(code.contains("fungsi utama()"));

        // Audit check
        let audit = periksa_aplikasi_mobile(&project_path).unwrap();
        assert!(audit.entry_exists, "Audit: entry harus ditemukan");
        assert!(audit.manifest_exists, "Audit: manifest harus ditemukan");
        assert_eq!(audit.app_name, app_name);
    }

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_mobile_pwa_manifest_and_sw() {
    let manifest = generate_pwa_manifest("TokoKu");
    assert!(manifest.contains("TokoKu"));
    assert!(manifest.contains("standalone"));
    assert!(manifest.contains("portrait-primary"));

    let sw = generate_service_worker();
    assert!(sw.contains("widya-mobile-cache-v1"));
    assert!(sw.contains("addEventListener('install'"));
    assert!(sw.contains("addEventListener('fetch'"));
}

#[test]
fn test_mobile_compilation_targets() {
    let tmp_dir = std::env::temp_dir().join("widya_mobile_test_build");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).unwrap();

    let project_path = inisialisasi_proyek_mobile_dengan_template("DemoKasir", Some(&tmp_dir), "pos").unwrap();
    let entry_file = project_path.join("src").join("main.wya");
    let out_dir = tmp_dir.join("dist");

    // Test Webview / PWA compilation
    let web_res = bangun_aplikasi_mobile(&entry_file, Some(&out_dir), "webview");
    assert!(web_res.is_ok(), "Kompilasi webview/PWA harus berhasil: {:?}", web_res);
    assert!(out_dir.join("index.html").exists());
    assert!(out_dir.join("manifest.webmanifest").exists());
    assert!(out_dir.join("sw.js").exists());

    // Test Android APK compilation (package generation)
    let android_res = bangun_aplikasi_mobile(&entry_file, Some(&out_dir), "android");
    assert!(android_res.is_ok(), "Kompilasi Android APK harus berhasil: {:?}", android_res);

    // Test iOS IPA compilation
    let ios_res = bangun_aplikasi_mobile(&entry_file, Some(&out_dir), "ios");
    assert!(ios_res.is_ok(), "Kompilasi iOS IPA harus berhasil: {:?}", ios_res);

    let _ = fs::remove_dir_all(&tmp_dir);
}
