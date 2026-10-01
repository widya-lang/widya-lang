use widya::office::crdt::{CollabSession, TextCrdt};

use widya::smartcity::{
    analisis_bauran_energi, hitung_ispu_pm25, jarak_haversine_km, optimasi_lalu_lintas,
    titik_di_dalam_poligon, KategoriIspu, LevelOfService, TitikKoordinat,
};

#[test]
fn test_crdt_text_operations_and_multi_client_convergence() {
    let mut doc_alice = TextCrdt::new("alice");
    let mut doc_bob = TextCrdt::new("bob");

    // Alice mengetik "Widya"
    doc_alice.insert_text(0, "Widya");
    assert_eq!(doc_alice.read(), "Widya");

    // Bob melakukan insert_text
    doc_bob.insert_text(0, "Widya");
    assert_eq!(doc_bob.read(), "Widya");

    // Alice menyisipkan " Bahasa" di akhir
    doc_alice.insert_text(5, " Bahasa");
    assert_eq!(doc_alice.read(), "Widya Bahasa");

    // Alice menghapus 5 karakter awal
    doc_alice.delete_range(0, 5);
    assert_eq!(doc_alice.read(), " Bahasa");
}

#[test]
fn test_collab_session_and_html_generation() {
    let mut session = CollabSession::new("sesi-nusantara", "UUD1945.docx", "presiden", "Bapak Presiden RI");
    session.crdt_engine.insert_text(0, "Undang-Undang Dasar Negara Republik Indonesia");

    let html = session.render_html_editor();
    assert!(html.contains("sesi-nusantara"));
    assert!(html.contains("UUD1945.docx"));
    assert!(html.contains("Undang-Undang Dasar"));
}

#[test]
fn test_smartcity_ispu_calculation() {
    let baik = hitung_ispu_pm25(12.0);
    assert_eq!(baik.kategori, KategoriIspu::Baik);
    assert!(baik.nilai_ispu <= 50);

    let sedang = hitung_ispu_pm25(35.0);
    assert_eq!(sedang.kategori, KategoriIspu::Sedang);
    assert!(sedang.nilai_ispu > 50 && sedang.nilai_ispu <= 100);

    let tidak_sehat = hitung_ispu_pm25(90.0);
    assert_eq!(tidak_sehat.kategori, KategoriIspu::TidakSehat);
    assert!(tidak_sehat.nilai_ispu > 100 && tidak_sehat.nilai_ispu <= 200);

    let sangat_tidak_sehat = hitung_ispu_pm25(180.0);
    assert_eq!(sangat_tidak_sehat.kategori, KategoriIspu::SangatTidakSehat);
    assert!(sangat_tidak_sehat.nilai_ispu > 200 && sangat_tidak_sehat.nilai_ispu <= 300);

    let berbahaya = hitung_ispu_pm25(300.0);
    assert_eq!(berbahaya.kategori, KategoriIspu::Berbahaya);
    assert!(berbahaya.nilai_ispu > 300);
}

#[test]
fn test_smartcity_geospatial_haversine_and_geofencing() {
    // Monas Jakarta (-6.1754, 106.8272) ke Gedung Sate Bandung (-6.9025, 107.6187)
    let monas = TitikKoordinat::new(-6.1754, 106.8272);
    let bandung = TitikKoordinat::new(-6.9025, 107.6187);
    let jarak = jarak_haversine_km(&monas, &bandung);

    // Jarak garis lurus Jakarta-Bandung sekitar 115 - 130 km
    assert!(jarak > 110.0 && jarak < 140.0, "Jarak terhitung: {} km", jarak);

    // Uji Geofencing Poligon Segitiga Wilayah
    let p1 = TitikKoordinat::new(0.0, 0.0);
    let p2 = TitikKoordinat::new(10.0, 0.0);
    let p3 = TitikKoordinat::new(5.0, 10.0);
    let poligon = vec![p1, p2, p3];

    let di_dalam = TitikKoordinat::new(5.0, 3.0);
    let di_luar = TitikKoordinat::new(15.0, 15.0);

    assert!(titik_di_dalam_poligon(&di_dalam, &poligon));
    assert!(!titik_di_dalam_poligon(&di_luar, &poligon));
}

#[test]
fn test_smartcity_traffic_and_energy_analytics() {
    // Rekayasa lalu lintas
    let lalin = optimasi_lalu_lintas("Jl. MH Thamrin", 1400.0, 1500.0, 120);
    assert_eq!(lalin.level_of_service, LevelOfService::E);
    assert!(lalin.durasi_lampu_hijau_detik >= 50);

    // Analisis bauran energi mikrogrid
    let energi = analisis_bauran_energi(20.0, 30.0, 50.0, 0.0); // 100% EBT
    assert_eq!(energi.persentase_ebt, 100.0);
    assert!(energi.reduksi_emisi_ton_co2_harian > 2000.0);
}

#[test]
fn test_widya_script_crdt_and_smartcity_execution() {
    let script = r##"
        // 1. Uji CRDT Real-time
        biar crdt = crdt_teks_buat("server_node")
        biar crdt_update = crdt_teks_sisip(crdt, 0, "Halo Indonesia Maju!")
        biar teks_awal = crdt_teks_baca(crdt_update)
        pastikan_sama(teks_awal, "Halo Indonesia Maju!")

        // 2. Uji Smart City ISPU
        biar ispu = smartcity_hitung_ispu_pm25(45.2)
        pastikan_sama(ispu["parameter"], "PM2.5")

        // 3. Uji Jarak Haversine
        biar jarak = smartcity_jarak_haversine(-6.2088, 106.8456, -6.9175, 107.6191)
        pastikan_benar(jarak > 100.0)

        // 4. Uji Energi EBT
        biar energi = smartcity_analisis_bauran_energi(25.0, 25.0, 50.0, 0.0)
        pastikan_sama(energi["persentase_ebt"], 100.0)
    "##;

    let res = widya::jalankan(script);
    assert!(res.is_ok(), "Eksekusi skrip CRDT & Smartcity gagal: {:?}", res.err());
}

