// ============================================================================
// Integration Test: Pure Sovereign OpenDocument Format (ODF) Engine
// ============================================================================
// Menguji ISO/IEC 26300: OpenDocument Text (.odt), Spreadsheet (.ods),
// dan Presentation (.odp) 100% mandiri tanpa LibreOffice atau pustaka luar.
// ============================================================================

use std::fs;
use widya::office::{
    OdpDocument, OdsCellValue, OdsDocument, OdtDocument,
    ODF_MIME_PRESENTATION, ODF_MIME_SPREADSHEET, ODF_MIME_TEXT,
};

#[test]
fn test_odt_document_generation_and_zip_structure() {
    let mut doc = OdtDocument::baru("Laporan Kedaulatan Digital")
        .dengan_penulis("Pemerintah Republik Indonesia");

    doc.tambah_judul(1, "BAB I: KEDAULATAN PERANGKAT LUNAK NASIONAL");
    doc.tambah_paragraf("Widya-Lang adalah bahasa pemrograman dan ekosistem komputasi berdaulat.");
    doc.tambah_paragraf_tebal("Pernyataan Resmi: Standar terbuka menjamin keabadian akses arsip negara.");

    doc.tambah_tabel(
        vec!["Komponen", "Status", "Kemandirian"],
        vec![
            vec!["Compiler & Bytecode", "Selesai", "100%"],
            vec!["WidyaBase DBMS", "Selesai", "100%"],
            vec!["WidyaOffice Suite", "Selesai", "100%"],
        ],
    );

    let odt_bytes = doc.tulis_ke_bytes()
        .expect("Pembangkitan ODT harus berhasil");

    // 1. Verifikasi header ZIP: PK\x03\x04
    assert!(odt_bytes.len() >= 100);
    assert_eq!(&odt_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);

    // 2. Verifikasi mimetype
    let odt_str = String::from_utf8_lossy(&odt_bytes);
    assert!(odt_str.contains(ODF_MIME_TEXT));

    // 3. Verifikasi berkas manifest & content ODF
    assert!(odt_str.contains("META-INF/manifest.xml"));
    assert!(odt_str.contains("content.xml"));
    assert!(odt_str.contains("styles.xml"));
    assert!(odt_str.contains("meta.xml"));

    // 4. Verifikasi teks isi dokumen
    assert!(odt_str.contains("BAB I: KEDAULATAN PERANGKAT LUNAK NASIONAL"));
    assert!(odt_str.contains("Pemerintah Republik Indonesia"));
    assert!(odt_str.contains("WidyaBase DBMS"));
}

#[test]
fn test_ods_spreadsheet_generation() {
    let mut spreadsheet = OdsDocument::baru("Anggaran Kedaulatan 2026");

    let sheet = spreadsheet.tambah_sheet("Rekapitulasi");
    // Baris Header
    sheet.rows.push(vec![
        OdsCellValue::String("Pos Anggaran".to_string()),
        OdsCellValue::String("Alokasi (IDR)".to_string()),
        OdsCellValue::String("Realisasi (%)".to_string()),
    ]);
    // Baris Data
    sheet.rows.push(vec![
        OdsCellValue::String("Riset Microkernel Baremetal".to_string()),
        OdsCellValue::Currency(500_000_000.0),
        OdsCellValue::Number(98.5),
    ]);
    sheet.rows.push(vec![
        OdsCellValue::String("Infrastruktur Jaringan HTTP/3".to_string()),
        OdsCellValue::Currency(350_000_000.0),
        OdsCellValue::Number(100.0),
    ]);

    let ods_bytes = spreadsheet.tulis_ke_bytes()
        .expect("Pembangkitan ODS harus berhasil");

    // Verifikasi mimetype spreadsheet ODF
    assert_eq!(&ods_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
    let ods_str = String::from_utf8_lossy(&ods_bytes);
    assert!(ods_str.contains(ODF_MIME_SPREADSHEET));
    assert!(ods_str.contains("Rekapitulasi"));
    assert!(ods_str.contains("Riset Microkernel Baremetal"));
    assert!(ods_str.contains("500000000"));
}

#[test]
fn test_odp_presentation_generation() {
    let mut pres = OdpDocument::baru("Paparan Eksekutif Widya");

    pres.tambah_slide(
        "Visi Indonesia Berdaulat Digital 2045",
        &[
            "Kemandirian teknologi informasi hulu ke hilir",
            "Bebas ketergantungan lisensi dan cloud asing",
            "Keamanan nasional berbasis Post-Quantum Cryptography",
        ],
    );

    pres.tambah_slide(
        "Pencapaian Teknologi Widya-Lang",
        &[
            "Pure Desktop Packager (deb, rpm, run, wapp)",
            "Pure Mobile APK & iOS IPA Bundler",
            "Pure Live ISO & UEFI ESP Generator",
            "Pure Parquet, Arrow, SafeTensors & GGUF Engine",
        ],
    );

    let odp_bytes = pres.tulis_ke_bytes()
        .expect("Pembangkitan ODP harus berhasil");

    assert_eq!(&odp_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);
    let odp_str = String::from_utf8_lossy(&odp_bytes);
    assert!(odp_str.contains(ODF_MIME_PRESENTATION));
    assert!(odp_str.contains("Visi Indonesia Berdaulat Digital 2045"));
    assert!(odp_str.contains("Pure Live ISO &amp; UEFI ESP Generator"));
}

#[test]
fn test_export_odf_files_to_disk() {
    let temp_dir = std::env::temp_dir().join("widya_odf_suite_test");

    // 1. Ekspor ODT
    let mut doc = OdtDocument::baru("Uji Ekspor ODT");
    doc.tambah_paragraf("Pengujian berkas ODT fisik.");
    let odt_file = temp_dir.join("dokumen_uji.odt");
    let res_odt = doc.ekspor_berkas(&odt_file).unwrap();
    assert_eq!(res_odt, odt_file);
    assert!(odt_file.exists());

    // 2. Ekspor ODS
    let mut sheet_doc = OdsDocument::baru("Uji Ekspor ODS");
    let sheet = sheet_doc.tambah_sheet("Data1");
    sheet.rows.push(vec![OdsCellValue::String("Test".to_string())]);
    let ods_file = temp_dir.join("tabel_uji.ods");
    let res_ods = sheet_doc.ekspor_berkas(&ods_file).unwrap();
    assert_eq!(res_ods, ods_file);
    assert!(ods_file.exists());

    // 3. Ekspor ODP
    let mut pres_doc = OdpDocument::baru("Uji Ekspor ODP");
    pres_doc.tambah_slide("Slide Uji", &["Poin 1", "Poin 2"]);
    let odp_file = temp_dir.join("slide_uji.odp");
    let res_odp = pres_doc.ekspor_berkas(&odp_file).unwrap();
    assert_eq!(res_odp, odp_file);
    assert!(odp_file.exists());

    // Pembersihan
    let _ = fs::remove_file(&odt_file);
    let _ = fs::remove_file(&ods_file);
    let _ = fs::remove_file(&odp_file);
    let _ = fs::remove_dir(&temp_dir);
}
