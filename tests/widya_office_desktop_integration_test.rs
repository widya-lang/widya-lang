//! Test Integrasi Suite Perkantoran Berdaulat: WidyaOffice Desktop
//! 100% Zero External Dependencies (No Electron, No Qt, No Chromium Bloat).
//! Memvalidasi:
//! 1. Windowing & Geometri Murni (JendelaMurni, Efek MicaDark, Dirty Regions).
//! 2. Keamanan Token Anti-CSRF Jembatan IPC Murni (JembatanIPCMurni).
//! 3. Konversi Rumus Matematika OpenXML OMML (latex_to_omml).
//! 4. Wisaya Mail Merge Batch 1.000 Dokumen.
//! 5. Tanda Tangan Digital Pasca-Kuantum NIST FIPS 204 (ML-DSA-87) & HSM PKCS#11.
//! 6. Ekspor Berkas OpenXML (.docx) dan PDF Vektor Murni.
//! 7. Audit Keamanan & Kepatuhan Manifest (widya_desktop.json, skor 100 A+).

use std::collections::HashMap;
use std::path::Path;
use serde_json::json;
use widya::desktop::office_bridge::OfficeBridgeEngine;
use widya::desktop::pure_window::{EfekLatarJendela, StatusJendela};
use widya::desktop::periksa_aplikasi_desktop;

#[test]
fn test_widya_office_desktop_window_geometry_and_mica() {
    let bridge = OfficeBridgeEngine::default_for_window("win-test-office", "token-test-123");
    let mut win = bridge.window.lock().unwrap();

    assert_eq!(win.id, "win-test-office");
    assert_eq!(win.efek_latar, EfekLatarJendela::MicaDark);
    assert!(win.custom_titlebar);

    // Initial size
    assert_eq!(win.lebar, 1280);
    assert_eq!(win.tinggi, 800);

    // Test resize constraint: clamped to min 800x600
    win.ubah_ukuran(500, 300);
    assert_eq!(win.lebar, 800);
    assert_eq!(win.tinggi, 600);

    // Test dirty regions
    let dirty = win.bersihkan_dirty_regions();
    assert!(!dirty.is_empty());
    assert_eq!(dirty[0], (0, 0, 800, 600));

    // Lifecycle
    win.tampilkan();
    assert_eq!(win.status, StatusJendela::Ditampilkan);
}

#[test]
fn test_widya_office_desktop_ipc_bridge_and_token_security() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-auth", "secret-token-abc");

    // Token verification
    assert!(bridge.validasi_token("secret-token-abc"));
    assert!(!bridge.validasi_token("invalid-token-xyz"));
    assert!(!bridge.validasi_token(""));
}

#[test]
fn test_widya_office_desktop_omml_equation_rendering() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-omml", "token-omml");

    // 1. Quadratic formula
    let payload1 = json!({
        "latex": "\\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}",
        "is_block": true
    });
    let res1 = bridge.call_method("equation.sisipkan", &payload1).unwrap();
    let omml1 = res1.get("omml").and_then(|v| v.as_str()).unwrap();
    assert!(omml1.contains("<m:f>"));
    assert!(omml1.contains("<m:rad>"));
    assert!(omml1.contains("<m:sSup>"));

    // 2. Greek symbols
    let payload2 = json!({
        "latex": "\\alpha + \\beta = \\gamma",
        "is_block": false
    });
    let res2 = bridge.call_method("equation.sisipkan", &payload2).unwrap();
    let omml2 = res2.get("omml").and_then(|v| v.as_str()).unwrap();
    assert!(omml2.contains("α"));
    assert!(omml2.contains("β"));
    assert!(omml2.contains("γ"));
}

#[test]
fn test_widya_office_desktop_mail_merge_batch_large_dataset() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-mailmerge", "token-mm");

    // 1. Test CSV Parsing
    let csv_sample = "Nama,Instansi,NomorSurat\nBudi Santoso,Kemenkeu,UND-001\nSiti Aminah,Kominfo,UND-002\nAgus Wijaya,BRIN,UND-003";
    let load_payload = json!({ "csv_raw": csv_sample });
    let load_res = bridge.call_method("mailmerge.muat_data", &load_payload).unwrap();
    assert_eq!(load_res.get("total_data").and_then(|v| v.as_u64()), Some(3));

    // 2. Test 1,000 Documents Merge Batch
    let mut dataset = Vec::new();
    for i in 1..=1000 {
        let mut map = HashMap::new();
        map.insert("Nama".to_string(), format!("Penerima-{}", i));
        map.insert("Jabatan".to_string(), "Kepala Biro".to_string());
        map.insert("Nomor".to_string(), format!("SRT-{:04}", i));
        dataset.push(map);
    }

    let batch_payload = json!({
        "templat_teks": "Yth. {{Nama}} ({{Jabatan}}), Surat Resmi No: {{Nomor}}",
        "dataset": dataset
    });

    let batch_res = bridge.call_method("mailmerge.ekspor_batch", &batch_payload).unwrap();
    assert_eq!(batch_res.get("total_dokumen_dihasilkan").and_then(|v| v.as_u64()), Some(1000));
}

#[test]
fn test_widya_office_desktop_pqc_mldsa87_hsm_signature() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-pqc", "token-pqc");

    // 1. Sign
    let payload_doc = "Dokumen Negara Sangat Rahasia - Perpres Ketahanan Pangan & Energi 2026";
    let sign_payload = json!({
        "payload_teks": payload_doc,
        "seed": "SEED_TEST_PQC_INTEGRATION_SUITE"
    });
    let sign_res = bridge.call_method("dokumen.tanda_tangani_pqc", &sign_payload).unwrap();
    let sig_hex = sign_res.get("signature_hex").and_then(|v| v.as_str()).unwrap().to_string();
    let pk_hex = sign_res.get("public_key_hex").and_then(|v| v.as_str()).unwrap().to_string();
    let salt_hex = sign_res.get("salt_hex").and_then(|v| v.as_str()).unwrap().to_string();
    let ts = sign_res.get("timestamp_ms").and_then(|v| v.as_u64()).unwrap();

    assert!(sig_hex.starts_with("mldsa_sig_"));
    assert!(pk_hex.starts_with("mldsa_pk_"));

    // 2. Verify Valid Signature
    let verify_payload = json!({
        "payload_teks": payload_doc,
        "signature_hex": sig_hex,
        "public_key_hex": pk_hex,
        "salt_hex": salt_hex,
        "timestamp_ms": ts
    });
    let verify_res = bridge.call_method("dokumen.verifikasi_pqc", &verify_payload).unwrap();
    assert_eq!(verify_res.get("valid").and_then(|v| v.as_bool()), Some(true));

    // 3. Anti-Tamper Verification (Modified payload must be rejected)
    let tampered_payload = json!({
        "payload_teks": "Dokumen Telah Dimanipulasi Pihak Tidak Berwenang",
        "signature_hex": sig_hex,
        "public_key_hex": pk_hex,
        "salt_hex": salt_hex,
        "timestamp_ms": ts
    });
    let tampered_res = bridge.call_method("dokumen.verifikasi_pqc", &tampered_payload).unwrap();
    assert_eq!(tampered_res.get("valid").and_then(|v| v.as_bool()), Some(false));

    // 4. HSM Status
    let hsm_res = bridge.call_method("sistem.status_hsm", &json!({})).unwrap();
    assert_eq!(hsm_res.get("hsm_aktif").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(hsm_res.get("standard").and_then(|v| v.as_str()), Some("PKCS#11 v2.40"));
}

#[test]
fn test_widya_office_desktop_docx_and_pdf_generation() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-export", "token-exp");

    // 1. DOCX Packaging
    let save_payload = json!({
        "nama_berkas": "Laporan_Tahunan.docx",
        "paragraf": [
            "Laporan Tahunan Transformasi Digital dan Kedaulatan Data.",
            "Suite WidyaOffice beroperasi tanpa dependensi pihak ketiga."
        ],
        "rumus": [
            "E = mc^2",
            "a^2 + b^2 = c^2"
        ]
    });
    let save_res = bridge.call_method("dokumen.simpan_berkas", &save_payload).unwrap();
    let docx_bytes = save_res.get("ukuran_bytes").and_then(|v| v.as_u64()).unwrap();
    assert!(docx_bytes > 500);

    // 2. Vector PDF Generation
    let pdf_payload = json!({
        "judul": "Ringkasan_Eksekutif.pdf",
        "baris_teks": [
            "WidyaOffice PDF Generator Engine",
            "Sesuai Standar ISO 32000-2"
        ]
    });
    let pdf_res = bridge.call_method("dokumen.ekspor_pdf", &pdf_payload).unwrap();
    let pdf_bytes = pdf_res.get("ukuran_pdf_bytes").and_then(|v| v.as_u64()).unwrap();
    assert!(pdf_bytes > 200);
}

#[test]
fn test_widya_office_desktop_project_validation_score() {
    let project_dir = Path::new("projects/widya_office_desktop");
    let report = periksa_aplikasi_desktop(project_dir).expect("Gagal memvalidasi proyek desktop");

    assert!(report.valid, "Proyek WidyaOffice Desktop harus valid: {:?}", report.issues);
    assert_eq!(report.app_name, "WidyaOffice Desktop");
    assert_eq!(report.version, "1.0.0");
    assert!(report.manifest_exists);
    assert!(report.backend_exists);
    assert!(report.frontend_exists);
    assert!(report.security_lock_exists);
    assert_eq!(report.security_score, 100);
    assert_eq!(report.security_grade, "Enterprise Grade (A+)");
}

#[test]
fn test_widyacalc_gridcraft_formula_and_xlsx() {
    let bridge = OfficeBridgeEngine::default_for_window("win-calc-test", "token-calc");

    // 1. Formula evaluation: =SUM(A1:A3)
    let eval_payload = json!({
        "formula": "=SUM(A1:A3)",
        "cells": {
            "A1": "1000000",
            "A2": "2500000",
            "A3": "4000000"
        }
    });
    let eval_res = bridge.call_method("calc.evaluasi", &eval_payload).unwrap();
    let num = eval_res.get("nilai_numerik").and_then(|v| v.as_f64()).unwrap();
    assert_eq!(num, 7500000.0);

    // 2. OpenXML XLSX packaging
    let xlsx_payload = json!({
        "judul": "BukuKerja_WidyaCalc.xlsx",
        "sheets": [{
            "name": "Anggaran",
            "cells": {
                "A1": "Pos", "B1": "Anggaran",
                "A2": "Server", "B2": "50000000"
            }
        }]
    });
    let xlsx_res = bridge.call_method("calc.simpan_xlsx", &xlsx_payload).unwrap();
    let xlsx_bytes = xlsx_res.get("ukuran_bytes").and_then(|v| v.as_u64()).unwrap();
    assert!(xlsx_bytes > 500);

    // 3. CSV Export
    let csv_payload = json!({
        "cells": {
            "A1": "Nama", "B1": "Kota",
            "A2": "Daniswara", "B2": "Yogyakarta"
        }
    });
    let csv_res = bridge.call_method("calc.ekspor_csv", &csv_payload).unwrap();
    let csv_text = csv_res.get("csv_text").and_then(|v| v.as_str()).unwrap();
    assert!(csv_text.contains("Daniswara"));
}

#[test]
fn test_widyaimpress_deckcraft_presentation_pptx() {
    let bridge = OfficeBridgeEngine::default_for_window("win-impress-test", "token-impress");

    // 1. Create Deck
    let create_payload = json!({
        "judul": "Presentasi Transformasi Kedaulatan Digital",
        "slides": [
            {
                "title": "Arsitektur Zero Bloat",
                "bullets": ["Penggunaan RAM < 35 MB", "Cold-start < 15 ms"]
            },
            {
                "title": "Kriptografi Pasca-Kuantum",
                "bullets": ["Standar NIST FIPS 204", "Enklave HSM PKCS#11"]
            }
        ]
    });
    let create_res = bridge.call_method("deck.buat_presentasi", &create_payload).unwrap();
    let total_slides = create_res.get("total_slide").and_then(|v| v.as_u64()).unwrap();
    assert_eq!(total_slides, 3); // 1 title slide + 2 content slides

    // 2. OpenXML PPTX Export
    let pptx_res = bridge.call_method("deck.simpan_pptx", &create_payload).unwrap();
    let pptx_bytes = pptx_res.get("ukuran_bytes").and_then(|v| v.as_u64()).unwrap();
    assert!(pptx_bytes > 500);
}

#[test]
fn test_widyapdf_pdfcraft_merge_and_uupdp_redaction() {
    let bridge = OfficeBridgeEngine::default_for_window("win-pdf-test", "token-pdf");

    // 1. UU PDP Privacy Redaction
    let redact_payload = json!({
        "teks": "Data Sensitif: NIK 3271012345678901 dan Telepon 081234567890"
    });
    let redact_res = bridge.call_method("pdf.sensor_redaksi", &redact_payload).unwrap();
    let redacted = redact_res.get("teks_tersensor").and_then(|v| v.as_str()).unwrap();
    assert!(redacted.contains("327101******8901"));
    assert!(redacted.contains("0812****7890"));

    // 2. PDF Document Combiner (Merge)
    let merge_payload = json!({
        "nama_output": "Laporan_Konsolidasi.pdf",
        "dokumen_list": [
            { "judul": "Bab 1 Pendahuluan", "baris": ["Latar Belakang Proyek WidyaOffice"] },
            { "judul": "Bab 2 Metodologi", "baris": ["Desain Arsitektur Sovereign Clean-Room"] }
        ]
    });
    let merge_res = bridge.call_method("pdf.gabung_dokumen", &merge_payload).unwrap();
    let total_pages = merge_res.get("total_halaman").and_then(|v| v.as_u64()).unwrap();
    let pdf_bytes = merge_res.get("ukuran_pdf_bytes").and_then(|v| v.as_u64()).unwrap();
    assert_eq!(total_pages, 2);
    assert!(pdf_bytes > 300);
}

#[test]
fn test_artcraft_portable_integration() {
    let bridge = OfficeBridgeEngine::default_for_window("win-artcraft-test", "token-artcraft");

    let res = bridge.call_method("craft.periksa_artcraft", &json!({})).unwrap();
    let artcraft_found = res.get("artcraft_ditemukan").and_then(|v| v.as_bool()).unwrap_or(false);
    let apps = res.get("aplikasi").and_then(|v| v.as_array()).unwrap();

    assert_eq!(apps.len(), 4);
    if artcraft_found {
        for app in apps {
            let id = app.get("id").and_then(|v| v.as_str()).unwrap();
            let installed = app.get("terpasang").and_then(|v| v.as_bool()).unwrap();
            println!("   ArtCraft App {}: Terpasang = {}", id, installed);
        }
    }
}

#[test]
fn test_sistem_jelajahi_dan_buat_folder() {
    let bridge = OfficeBridgeEngine::default_for_window("win-folder-test", "token-folder");

    // 1. Jelajahi folder proyek saat ini
    let explore_payload = json!({ "path": "." });
    let explore_res = bridge.call_method("sistem.jelajahi_folder", &explore_payload).unwrap();
    assert_eq!(explore_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    let subfolders = explore_res.get("subfolders").and_then(|v| v.as_array()).unwrap();
    assert!(!subfolders.is_empty(), "Subfolders tidak boleh kosong pada direktori aktif");

    // 2. Buat folder sementara
    let test_dir = "target/widya_test_save_dir";
    let create_payload = json!({ "path": test_dir });
    let create_res = bridge.call_method("sistem.buat_folder", &create_payload).unwrap();
    assert_eq!(create_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(std::path::Path::new(test_dir).exists());

    // 3. Jelajahi folder yang baru dibuat
    let explore_new = bridge.call_method("sistem.jelajahi_folder", &create_payload).unwrap();
    assert_eq!(explore_new.get("sukses").and_then(|v| v.as_bool()), Some(true));

    // Bersihkan folder test
    let _ = std::fs::remove_dir(test_dir);
}

#[test]
fn test_engine_command_bus_methods() {
    let bridge = OfficeBridgeEngine::default_for_window("win-cmd-test", "token-cmd");

    // 1. Uji list_commands
    let list_res = bridge.call_method("engine.list_commands", &json!({})).unwrap();
    assert_eq!(list_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    let total = list_res.get("total_perintah").and_then(|v| v.as_u64()).unwrap();
    assert!(total >= 20, "Total perintah katalog harus minimal 20");

    let katalog = list_res.get("katalog").and_then(|v| v.as_array()).unwrap();
    let has_bold = katalog.iter().any(|c| c.get("id").and_then(|v| v.as_str()) == Some("format.bold"));
    let has_page_break = katalog.iter().any(|c| c.get("id").and_then(|v| v.as_str()) == Some("insert.page_break"));
    let has_inspect = katalog.iter().any(|c| c.get("id").and_then(|v| v.as_str()) == Some("document.inspect"));
    assert!(has_bold, "Katalog harus memuat perintah format.bold");
    assert!(has_page_break, "Katalog harus memuat perintah insert.page_break");
    assert!(has_inspect, "Katalog harus memuat perintah document.inspect");

    // 2. Uji execute_command
    let exec_payload = json!({
        "command": "format.bold",
        "payload": { "target": "selection" }
    });
    let exec_res = bridge.call_method("engine.execute_command", &exec_payload).unwrap();
    assert_eq!(exec_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(exec_res.get("command").and_then(|v| v.as_str()), Some("format.bold"));
}

#[test]
fn test_widya_document_dsl_and_markdown_compiler() {
    use widya::office::dsl::WidyaDocumentDsl;

    // 1. Uji Parsing & Kompilasi Skrip Widya-Lang Document DSL
    let widya_code = r#"
        dokumen {
            judul: "Metodologi Penelitian Sistem Komputasi Berdaulat",
            penulis: "Dr. Satriyo",
            institusi: "Laboratorium Riset Kedaulatan Digital",
            ukuran: "A4",
            daftar_isi: benar,

            bab("BAB I PENDAHULUAN")
            paragraf("Kedaulatan digital nasional merupakan landasan utama peradaban masa depan.")
            catatan_kaki("Rujukan Undang-Undang No. 27 Tahun 2022 tentang Pelindungan Data Pribadi.")

            bab("BAB II METODE RISET")
            paragraf("Penelitian ini menggunakan model ekonometrika spasial dan algoritma komputasi kuantum.")
            persamaan("E = m \cdot c^2")

            tabel(
                judul: "Tabel 1: Spesifikasi Parameter Uji",
                kolom: ["No", "Parameter", "Target", "Satuan"],
                baris: [
                    ["1", "Latensi IPC Native", "< 2", "ms"],
                    ["2", "PQC FIPS 204 Signature", "Valid", "Status"],
                    ["3", "Efisiensi Memori", "> 98", "%"]
                ]
            )
        }
    "#;

    let doc = WidyaDocumentDsl::from_widya_dsl(widya_code).expect("Gagal parse Widya DSL");
    assert_eq!(doc.title, "Metodologi Penelitian Sistem Komputasi Berdaulat");
    assert_eq!(doc.author, "Dr. Satriyo");
    assert!(doc.has_toc);
    assert_eq!(doc.footnotes.len(), 1);
    assert_eq!(doc.footnotes[0].text, "Rujukan Undang-Undang No. 27 Tahun 2022 tentang Pelindungan Data Pribadi.");

    // Validasi HTML untuk WidyaWriter
    let writer_html = doc.to_writer_html();
    assert!(writer_html.contains("doc-toc"), "HTML harus menyertakan Daftar Isi dinamis");
    assert!(writer_html.contains("doc-page-footnotes"), "HTML harus memuat kontainer footnote");
    assert!(writer_html.contains("doc-footnote-ref"), "HTML harus memuat rujukan footnote");
    assert!(writer_html.contains("<table"), "HTML harus menyertakan tabel terstruktur");

    // Validasi Kompilasi OpenXML DOCX & PDF
    let docx = doc.to_docx();
    let docx_bytes = docx.export_to_docx_bytes();
    assert!(docx_bytes.len() > 1000, "Ukuran berkas DOCX harus valid OpenXML zip");

    let pdf = doc.to_pdf();
    let pdf_bytes = pdf.generate_pdf_bytes();
    assert!(pdf_bytes.len() > 500, "Ukuran berkas PDF harus valid");

    // 2. Uji Konversi Markdown Disertasi Akademis ke WidyaDocumentDsl
    let dissertation_md = r#"# BAB III METODE PENELITIAN

## A. Rancangan Penelitian
Penelitian ini menggunakan pendekatan kuantitatif eksplanatori dengan desain panel longitudinal.[^1]

## B. Operasionalisasi Variabel
Pengukuran variabel dilakukan dengan indikator terstandarisasi.

| Klasifikasi | Simbol | Variabel | Satuan |
| :--- | :---: | :--- | :--- |
| Output | Y1 | Literasi Kognitif | 0-100 |
| Output | Y2 | Numerasi Kognitif | 0-100 |

[^1]: Sesuai metodologi Wooldridge (2010) untuk Two-Way Fixed Effects.
"#;

    let md_doc = WidyaDocumentDsl::from_markdown(dissertation_md, Some("BAB III METODE PENELITIAN"));
    assert_eq!(md_doc.title, "BAB III METODE PENELITIAN");
    assert_eq!(md_doc.footnotes.len(), 1);
    assert!(md_doc.footnotes[0].text.contains("Wooldridge"));

    let md_html = md_doc.to_writer_html();
    assert!(md_html.contains("BAB III METODE PENELITIAN"));
    assert!(md_html.contains("doc-footnote-ref"));
    assert!(md_html.contains("<table"));

    // 3. Uji Melalui Native IPC Bridge
    let bridge = OfficeBridgeEngine::default_for_window("win-dsl-test", "token-dsl");

    let dsl_res = bridge.call_method("dsl.compile_widya_doc", &json!({ "code": widya_code })).unwrap();
    assert_eq!(dsl_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(dsl_res.get("total_catatan_kaki").and_then(|v| v.as_u64()), Some(1));
    assert!(dsl_res.get("html").and_then(|v| v.as_str()).unwrap().contains("doc-toc"));

    let md_res = bridge.call_method("dsl.import_markdown", &json!({
        "markdown": dissertation_md,
        "judul": "Bab 3 Metode Disertasi"
    })).unwrap();
    assert_eq!(md_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(md_res.get("total_catatan_kaki").and_then(|v| v.as_u64()), Some(1));
    assert!(md_res.get("docx_bytes_len").and_then(|v| v.as_u64()).unwrap() > 1000);
}

#[test]
fn test_widyacalc_dsl_and_chart_engine() {
    use widya::office::calc_dsl::WidyaWorkbookDsl;

    // 1. Uji Parsing & Kompilasi Skrip Widya-Lang Calc DSL
    let widya_calc_code = r#"
        buku_kerja {
            judul: "Simulasi Finansial Riset Kedaulatan",
            mata_uang: "IDR",

            lembar("Ringkasan Anggaran") {
                baris_header(["Komponen", "Alokasi", "Realisasi", "Persentase"])
                baris(["Hardware Server PQC", "150000000", "145000000", "=C2/B2"])
                baris(["Audit Kriptografi", "50000000", "48000000", "=C3/B3"])
                baris(["Diseminasi Publik", "30000000", "29500000", "=C4/B4"])
                baris(["Total Pengeluaran", "=SUM(B2:B4)", "=SUM(C2:C4)", "=C5/B5"])

                grafik(
                    judul: "Realisasi Anggaran Komponen",
                    tipe: "batang",
                    label: "A2:A4",
                    data: "C2:C4"
                )
            }
        }
    "#;

    let wb = WidyaWorkbookDsl::from_widya_dsl(widya_calc_code).expect("Gagal parse Widya Calc DSL");
    assert_eq!(wb.title, "Simulasi Finansial Riset Kedaulatan");
    assert_eq!(wb.sheets.len(), 1);

    let sheet = &wb.sheets[0];
    assert_eq!(sheet.name, "Ringkasan Anggaran");
    assert!(!sheet.cells.is_empty(), "Sheet harus memuat sel-sel hasil parse");
    assert_eq!(wb.charts.len(), 1);
    assert_eq!(wb.charts[0].title, "Realisasi Anggaran Komponen");
    assert_eq!(wb.charts[0].kind, "batang");

    // 2. Uji Kompilasi ke OpenXML XLSX Murni
    let xlsx = wb.to_xlsx();
    let xlsx_bytes = xlsx.export_to_xlsx_bytes();
    assert!(xlsx_bytes.len() > 1000, "Ukuran berkas XLSX harus valid OpenXML zip binary");

    // 3. Uji Ekspor ke CSV
    let csv = wb.to_csv("Ringkasan Anggaran");
    assert!(csv.contains("Hardware Server PQC"));
    assert!(csv.contains("=SUM(B2:B4)"));

    // 4. Uji Konversi ke Grid JSON untuk Frontend WidyaCalc
    let grid_json = wb.to_grid_json();
    assert_eq!(grid_json.get("activeSheet").and_then(|v| v.as_str()), Some("Sheet1"));
    assert!(grid_json.get("sheets").is_some());
    assert_eq!(grid_json["sheets"]["Sheet1"]["name"].as_str(), Some("Ringkasan Anggaran"));

    // 5. Uji Generator Grafik Vektor SVG Interaktif Murni
    let svg = wb.generate_svg_chart(0).expect("Gagal generate SVG chart");
    assert!(svg.contains("<svg"), "Grafik harus dalam format SVG valid");
    assert!(svg.contains("Realisasi Anggaran Komponen"), "Judul grafik harus tertera di SVG");
    assert!(svg.contains("<rect"), "Grafik batang harus memuat elemen SVG rect");

    // 6. Uji Serialisasi Balik ke Kode Deklaratif Widya-Lang
    let dsl_roundtrip = wb.to_widya_dsl_code();
    assert!(dsl_roundtrip.contains("buku_kerja"));
    assert!(dsl_roundtrip.contains("Ringkasan Anggaran"));
    assert!(dsl_roundtrip.contains("grafik("));

    // 7. Uji Melalui Native IPC Bridge
    let bridge = OfficeBridgeEngine::default_for_window("win-calc-test", "token-calc");

    // Uji calc.compile_dsl
    let compile_res = bridge.call_method("calc.compile_dsl", &json!({ "code": widya_calc_code })).unwrap();
    assert_eq!(compile_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(compile_res.get("total_lembar").and_then(|v| v.as_u64()), Some(1));
    assert!(compile_res.get("xlsx_bytes_len").and_then(|v| v.as_u64()).unwrap() > 1000);
    assert!(compile_res.get("grid_json").is_some());

    // Uji calc.generate_chart
    let chart_res = bridge.call_method("calc.generate_chart", &json!({
        "type": "batang",
        "title": "Realisasi Anggaran Komponen",
        "label_range": "A2:A4",
        "data_range": "C2:C4",
        "cells": {
            "A2": "Hardware Server PQC", "C2": "145000000",
            "A3": "Audit Kriptografi", "C3": "48000000",
            "A4": "Diseminasi Publik", "C4": "29500000"
        }
    })).unwrap();
    assert_eq!(chart_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(chart_res.get("svg").and_then(|v| v.as_str()).unwrap().contains("<svg"));

    // Uji calc.export_dsl dari data grid state
    let export_res = bridge.call_method("calc.export_dsl", &json!({
        "judul": "BukuKerja_Widya",
        "sheets": grid_json["sheets"]
    })).unwrap();
    assert_eq!(export_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(export_res.get("widya_dsl").and_then(|v| v.as_str()).unwrap().contains("buku_kerja"));
}

#[test]
fn test_widyaimpress_dsl_and_vector_renderer() {
    use widya::office::impress_dsl::WidyaPresentationDsl;

    // 1. Uji Parsing & Kompilasi Skrip Widya-Lang Presentasi DSL
    let widya_impress_code = r#"
        presentasi {
            judul: "Transformasi Kedaulatan Digital 2026",
            penulis: "Dr. Satriyo",
            tema: "navy",
            rasio: "16:9",

            slide_judul(
                judul: "Kedaulatan Digital & Teknologi Nasional 2026",
                subjudul: "Peta Jalan Transformasi Perkantoran Berdaulat Tanpa Ketergantungan Asing",
                catatan: "Tekankan kemandirian teknologi nasional."
            )

            slide_kpi(
                judul: "Dashboard Metrik Strategis WidyaOffice",
                subjudul: "Efisiensi Kinerja & Benchmark Industri Terbuka",
                kartu: [
                    { label: "Penghematan RAM", nilai: "88%", subteks: "Dibandingkan Electron", positif: benar },
                    { label: "Waktu Cold-Start", nilai: "<15ms", subteks: "Biner Mandiri 64-bit AMD64", positif: benar },
                    { label: "Kriptografi", nilai: "FIPS 204", subteks: "NIST ML-DSA-87", positif: benar }
                ],
                catatan: "Jelaskan data empiris penghematan memori."
            )

            slide_konten(
                judul: "Pilar Utama Ekosistem WidyaOffice Suite",
                subjudul: "Arsitektur Bersih & Berstandar Terbuka Internasional",
                poin: [
                    "WidyaWriter: Pengolah kata berdaulat dengan editor rumus OMML",
                    "WidyaCalc: Mesin lembar kerja analitik berkecepatan tinggi",
                    "WidyaImpress: Perancang presentasi eksekutif dengan kartu metrik KPI"
                ],
                catatan: "Sebutkan keempat aplikasi terintegrasi."
            )

            slide_dua_kolom(
                judul: "Analisis Komparasi Kedaulatan",
                subjudul: "Office Konvensional vs Widya Stack",
                kiri_judul: "Office Konvensional",
                kiri_poin: ["Konsumsi RAM tinggi (>250MB)", "Kunci vendor proprietary"],
                kanan_judul: "Widya Sovereign Stack",
                kanan_poin: ["Penggunaan RAM <35MB", "100% Zero-External C Dependency"]
            )
        }
    "#;

    let pres = WidyaPresentationDsl::from_widya_dsl(widya_impress_code).expect("Gagal parse Widya Impress DSL");
    assert_eq!(pres.title, "Transformasi Kedaulatan Digital 2026");
    assert_eq!(pres.author, "Dr. Satriyo");
    assert_eq!(pres.slides.len(), 4);
    assert_eq!(pres.slides[0].layout, "title");
    assert_eq!(pres.slides[1].layout, "kpi");
    assert_eq!(pres.slides[1].kpi_cards.len(), 3);
    assert_eq!(pres.slides[2].layout, "content");
    assert_eq!(pres.slides[2].bullets.len(), 3);
    assert_eq!(pres.slides[3].layout, "two-column");
    assert_eq!(pres.slides[3].left_bullets.len(), 2);
    assert_eq!(pres.slides[3].right_bullets.len(), 2);

    // 2. Uji Kompilasi ke OpenXML PPTX Murni
    let pptx = pres.to_pptx();
    let pptx_bytes = pptx.export_to_pptx_bytes();
    assert!(pptx_bytes.len() > 1000, "Ukuran berkas PPTX harus valid ECMA-376 zip binary");

    // 3. Uji Parsing Markdown Presentasi (Marp/Slidev Parity)
    let md_slides = r#"# Slide Pembuka Eksekutif
Peta Jalan Kedaulatan Perkantoran Nasional

---

# Capaian Metrik Utama
- Latensi IPC < 2ms
- Tanda Tangan Digital PQC Valid
<!-- notes: Jelaskan pengujian laboratorium -->

---

# Kesimpulan & Langkah Strategis
- Implementasi menyeluruh instansi pemerintah
- Perlindungan data pribadi 100% lokal
"#;
    let md_pres = WidyaPresentationDsl::from_markdown(md_slides, Some("Paparan Eksekutif"));
    assert_eq!(md_pres.title, "Paparan Eksekutif");
    assert_eq!(md_pres.slides.len(), 3);
    assert_eq!(md_pres.slides[0].layout, "title");
    assert_eq!(md_pres.slides[1].layout, "content");
    assert_eq!(md_pres.slides[1].notes.as_deref(), Some("Jelaskan pengujian laboratorium"));

    // 4. Uji Render Vektor SVG Slide Mandiri
    let svg_title = pres.render_slide_svg(0).expect("Gagal render SVG slide title");
    assert!(svg_title.contains("<svg"), "Harus menghasilkan format SVG valid");
    assert!(svg_title.contains("Kedaulatan Digital"), "Judul slide harus tertera di SVG");

    let svg_kpi = pres.render_slide_svg(1).expect("Gagal render SVG slide kpi");
    assert!(svg_kpi.contains("Penghematan RAM"), "Label KPI harus ada di SVG");
    assert!(svg_kpi.contains("88%"), "Nilai KPI harus ada di SVG");

    // 5. Uji Roundtrip Serialisasi Kode Widya-Lang DSL
    let dsl_code = pres.to_widya_dsl_code();
    assert!(dsl_code.contains("presentasi {"));
    assert!(dsl_code.contains("slide_kpi("));
    assert!(dsl_code.contains("slide_dua_kolom("));

    // 6. Uji Melalui Native IPC Bridge
    let bridge = OfficeBridgeEngine::default_for_window("win-impress-dsl-test", "token-impress-dsl");

    // impress.compile_dsl
    let compile_res = bridge.call_method("impress.compile_dsl", &json!({ "code": widya_impress_code })).unwrap();
    assert_eq!(compile_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(compile_res.get("total_slide").and_then(|v| v.as_u64()), Some(4));
    assert!(compile_res.get("pptx_bytes_len").and_then(|v| v.as_u64()).unwrap() > 1000);
    assert!(compile_res.get("slides_json").is_some());

    // impress.import_markdown
    let md_res = bridge.call_method("impress.import_markdown", &json!({
        "markdown": md_slides,
        "judul": "Paparan Eksekutif"
    })).unwrap();
    assert_eq!(md_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(md_res.get("total_slide").and_then(|v| v.as_u64()), Some(3));

    // impress.export_dsl
    let export_res = bridge.call_method("impress.export_dsl", &json!({
        "presentasi": compile_res["slides_json"]
    })).unwrap();
    assert_eq!(export_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(export_res.get("widya_dsl").and_then(|v| v.as_str()).unwrap().contains("presentasi {"));

    // impress.render_slide_svg
    let render_res = bridge.call_method("impress.render_slide_svg", &json!({
        "code": widya_impress_code,
        "slide_index": 1
    })).unwrap();
    assert_eq!(render_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(render_res.get("svg").and_then(|v| v.as_str()).unwrap().contains("<svg"));
}

#[test]
fn test_widyapdf_dsl_and_pqc_workbench() {
    use widya::office::pdf_dsl::{WidyaPdfDsl, DslPdfItem, RubberStampType};
    use widya::desktop::office_bridge::OfficeBridgeEngine;
    use serde_json::json;

    // 1. Skrip Deklaratif WidyaPDF DSL
    let widya_pdf_code = r#"
dokumen_pdf {
    judul: "Keputusan Presiden tentang Standar Kedaulatan Komputasi"
    penulis: "Sekretariat Negara RI"
    organisasi: "Pemerintah Republik Indonesia"
    ukuran_halaman: "A4"
    tanda_air: "RAHASIA NEGARA"

    halaman(1) {
        judul: "KEPUTUSAN PRESIDEN REPUBLIK INDONESIA"
        tajuk: "LEMBAR NEGARA TAHUN 2026 NOMOR 104"
        paragraf: "MENIMBANG: Bahwa kedaulatan teknologi informasi dan komunikasi nasional memerlukan kemandirian penuh dalam ekosistem perangkat lunak perkantoran tingkat industri."
        paragraf: "MENGINGAT: Undang-Undang Nomor 27 Tahun 2022 tentang Pelindungan Data Pribadi (UU PDP)."
        kotak_redaksi: [50.0, 600.0, 250.0, 30.0, "UU PDP SENSOR"]
        stempel: ["CONFIDENTIAL", 420.0, 720.0, "SANGAT RAHASIA"]
        catatan: ["Sekretariat Negara", "Pastikan klausul zero-external dependency di Pasal 1 selaras dengan amanat kedaulatan.", 600.0, 180.0]
    }

    halaman(2) {
        judul: "LAMPIRAN I: DAFTAR PEJABAT PENANGGUNG JAWAB"
        tajuk: "KEMENTERIAN KOORDINATOR BIDANG DIGITAL"
        paragraf: "1. Penanggung Jawab Sistem: Prof. Dr. Budi Santoso (NIK: 3271012345678901, No Telp: 081234567890)."
        paragraf: "2. Auditor Kriptografi Pasca-Kuantum: Dr. Siti Aminah (Rekening Operasional: 1234567890 Bank Mandiri)."
        tanda_tangan_pqc: ["Prof. Dr. Budi Santoso", "Kepala Pusat Kriptografi BSSN", "NIST ML-DSA-87", 380.0, 140.0]
    }
}
"#;

    // 2. Uji Parsing DSL
    let mut doc = WidyaPdfDsl::from_widya_dsl(widya_pdf_code).expect("Gagal mem-parsing WidyaPDF DSL");
    assert_eq!(doc.title, "Keputusan Presiden tentang Standar Kedaulatan Komputasi");
    assert_eq!(doc.author, "Sekretariat Negara RI");
    assert_eq!(doc.pages.len(), 2);
    assert_eq!(doc.watermark.as_deref(), Some("RAHASIA NEGARA"));

    // Halaman 1
    assert_eq!(doc.pages[0].page_number, 1);
    assert_eq!(doc.pages[0].title, "KEPUTUSAN PRESIDEN REPUBLIK INDONESIA");
    assert_eq!(doc.pages[0].body.len(), 2);

    // 3. Uji Sensor Redaksi Privasi Otomatis (UU PDP No. 27/2022)
    let redacted_count = doc.apply_uupdp_redaction();
    assert!(redacted_count >= 2, "Harus menyamarkan minimal NIK, no telepon, atau no rekening");
    assert!(doc.security_pdp, "Flag keamanan UU PDP harus aktif");

    // Periksa apakah teks pada halaman 2 berhasil disensor
    let hal2_body = doc.pages[1].body.join(" ");
    assert!(hal2_body.contains("327101******8901"), "NIK harus tersensor 6 digit tengah");
    assert!(hal2_body.contains("0812****7890"), "Nomor telepon harus tersensor digit tengah");
    assert!(hal2_body.contains("1234****90"), "Nomor rekening bank harus tersensor");

    // 4. Uji Penandatanganan Pasca-Kuantum NIST FIPS 204 (ML-DSA-87)
    let sig_meta = doc.sign_pqc("Dr. Satriyo", "Kepala Badan Siber dan Sandi Negara")
        .expect("Gagal menandatangani dokumen secara kriptografis PQC");
    assert!(doc.pqc_signed);
    assert_eq!(sig_meta.signer_name, "Dr. Satriyo");
    assert!(sig_meta.algorithm.contains("ML-DSA-87"));
    assert!(sig_meta.signature_hex.starts_with("mldsa_sig_"));
    assert_eq!(sig_meta.document_digest_sha256.len(), 64);

    // 5. Uji Pembangkitan Berkas Biner Asli ISO 32000-1 (%PDF-1.7)
    let pdf = doc.to_pdf();
    let pdf_bytes = pdf.generate_pdf_bytes();
    assert!(pdf_bytes.starts_with(b"%PDF-1.7"), "Header PDF harus valid ISO 32000-1");
    let pdf_text = String::from_utf8_lossy(&pdf_bytes);
    assert!(pdf_text.contains("%%EOF"), "Trailer PDF harus berakhiran %%EOF");
    assert!(pdf_text.contains("/Root 1 0 R"), "Catalog root harus terdefinisi");
    assert!(pdf_text.contains("/Type /Page"), "Objek halaman harus ada");
    assert!(pdf_bytes.len() > 1500, "Ukuran biner PDF harus valid dan lengkap");

    // 6. Uji Render Vektor SVG Halaman Mandiri
    let svg_p1 = doc.render_page_svg(0).expect("Gagal render SVG halaman 1");
    assert!(svg_p1.contains("<svg"), "Harus menghasilkan format SVG valid");
    assert!(svg_p1.contains("KEPUTUSAN PRESIDEN"), "Judul harus ada di SVG");
    assert!(svg_p1.contains("RAHASIA NEGARA"), "Watermark harus ada di SVG");

    let svg_p2 = doc.render_page_svg(1).expect("Gagal render SVG halaman 2");
    assert!(svg_p2.contains("[TTE PASCA-KUANTUM]"), "Lencana TTE harus ada di SVG");
    assert!(svg_p2.contains("ML-DSA-87"), "Algoritma PQC harus ada di SVG");

    // 7. Uji Roundtrip Serialisasi Kode Widya-Lang DSL
    let exported_dsl = doc.to_widya_dsl_code();
    assert!(exported_dsl.contains("dokumen_pdf {"));
    assert!(exported_dsl.contains("halaman(1) {"));
    assert!(exported_dsl.contains("halaman(2) {"));

    // 8. Uji Melalui Native IPC Bridge
    let bridge = OfficeBridgeEngine::default_for_window("win-pdf-dsl-test", "token-pdf-dsl");

    // A. engine.list_commands (Memastikan 21 perintah pdf terdaftar di bus)
    let list_res = bridge.call_method("engine.list_commands", &json!({})).unwrap();
    let katalog = list_res.get("katalog").and_then(|v| v.as_array()).unwrap();
    let pdf_commands: Vec<_> = katalog.iter()
        .filter(|c| c.get("category").and_then(|v| v.as_str()) == Some("pdf"))
        .collect();
    assert_eq!(pdf_commands.len(), 21, "Harus ada 21 perintah kategori PDF yang terdaftar");

    // B. pdf.compile_dsl
    let compile_res = bridge.call_method("pdf.compile_dsl", &json!({
        "code": widya_pdf_code,
        "apply_pdp": true
    })).unwrap();
    assert_eq!(compile_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(compile_res.get("total_halaman").and_then(|v| v.as_u64()), Some(2));
    assert_eq!(compile_res.get("sensor_pdp_diterapkan").and_then(|v| v.as_bool()), Some(true));
    assert!(compile_res.get("pdf_bytes_len").and_then(|v| v.as_u64()).unwrap() > 1000);
    assert!(compile_res.get("pages_json").is_some());
    assert!(compile_res.get("preview_svg").is_some());

    // C. pdf.export_dsl
    let export_res = bridge.call_method("pdf.export_dsl", &json!({
        "pagesData": compile_res["pages_json"]["pagesData"],
        "manualRedactions": compile_res["pages_json"]["manualRedactions"],
        "rubberStamps": compile_res["pages_json"]["rubberStamps"],
        "digitalSignatures": compile_res["pages_json"]["digitalSignatures"],
        "comments": compile_res["pages_json"]["comments"]
    })).unwrap();
    assert_eq!(export_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(export_res.get("widya_dsl").and_then(|v| v.as_str()).unwrap().contains("dokumen_pdf {"));

    // D. pdf.tandatangani_pqc
    let sign_res = bridge.call_method("pdf.tandatangani_pqc", &json!({
        "code": widya_pdf_code,
        "signer": "Prof. Dr. Budi Santoso",
        "role": "Kepala Pusat Kriptografi BSSN"
    })).unwrap();
    assert_eq!(sign_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(sign_res.get("penandatangan").and_then(|v| v.as_str()), Some("Prof. Dr. Budi Santoso"));
    assert!(sign_res.get("signature_hex").and_then(|v| v.as_str()).unwrap().starts_with("mldsa_sig_"));

    // E. pdf.render_svg_page
    let render_res = bridge.call_method("pdf.render_svg_page", &json!({
        "code": widya_pdf_code,
        "page_index": 0
    })).unwrap();
    assert_eq!(render_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(render_res.get("svg").and_then(|v| v.as_str()).unwrap().contains("<svg"));
}

#[test]
fn test_batch_pqc_signer_mldsa87_high_throughput() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-pqc-batch", "token-pqc-batch");

    // 1. Siapkan 50 dokumen untuk ditandatangani secara massal
    let mut doc_list = Vec::new();
    let mut konten_asli = serde_json::Map::new();
    for i in 1..=50 {
        let id = format!("DOC-{:04}", i);
        let content = format!("Isi surat keputusan resmi kedaulatan data nomor {}/SK/BSSN/2026", i);
        doc_list.push(json!({
            "id": id,
            "judul": format!("Surat Keputusan #{:04}", i),
            "konten": content
        }));
        konten_asli.insert(id, json!(content));
    }

    let batch_req = json!({
        "dokumen_list": doc_list,
        "seed": "BSSN_SOVEREIGN_BATCH_SEED_2026"
    });

    let batch_res = bridge.call_method("dokumen.tanda_tangani_batch_pqc", &batch_req).unwrap();
    assert_eq!(batch_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(batch_res.get("total_signed").and_then(|v| v.as_u64()), Some(50));
    assert_eq!(batch_res.get("all_verified_valid").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(batch_res.get("algorithm").and_then(|v| v.as_str()), Some("ML-DSA-87 (Dilithium-5)"));

    let signed_items = batch_res.get("signed_items").and_then(|v| v.as_array()).unwrap();
    assert_eq!(signed_items.len(), 50);

    // 2. Verifikasi massal ke-50 dokumen
    let verify_req = json!({
        "signed_items": signed_items,
        "konten_asli": konten_asli
    });
    let verify_res = bridge.call_method("dokumen.verifikasi_batch_pqc", &verify_req).unwrap();
    assert_eq!(verify_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(verify_res.get("total").and_then(|v| v.as_u64()), Some(50));
    assert_eq!(verify_res.get("valid_count").and_then(|v| v.as_u64()), Some(50));
    assert_eq!(verify_res.get("invalid_count").and_then(|v| v.as_u64()), Some(0));
    assert_eq!(verify_res.get("semua_valid").and_then(|v| v.as_bool()), Some(true));

    // 3. Uji deteksi pemalsuan (Anti-Tampering) pada verifikasi massal
    let mut tampered_konten = konten_asli.clone();
    tampered_konten.insert("DOC-0025".to_string(), json!("Dokumen dipalsukan oleh peretas"));
    let tampered_req = json!({
        "signed_items": signed_items,
        "konten_asli": tampered_konten
    });
    let tampered_res = bridge.call_method("dokumen.verifikasi_batch_pqc", &tampered_req).unwrap();
    assert_eq!(tampered_res.get("semua_valid").and_then(|v| v.as_bool()), Some(false));
    assert_eq!(tampered_res.get("valid_count").and_then(|v| v.as_u64()), Some(49));
    assert_eq!(tampered_res.get("invalid_count").and_then(|v| v.as_u64()), Some(1));
}

#[test]
fn test_crdt_offline_first_p2p_collaboration_simulation() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-crdt", "token-crdt");

    let sim_payload = json!({
        "base_text": "Kedaulatan Digital Indonesia",
        "peer_a_pos": 28,
        "peer_a_text": " - Solusi Berdaulat Tanpa Cloud Asing.",
        "peer_b_pos": 0,
        "peer_b_text": "Resmi: "
    });

    let res = bridge.call_method("crdt.simulasi_konvergensi", &sim_payload).unwrap();
    assert_eq!(res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(res.get("is_identical").and_then(|v| v.as_bool()), Some(true));

    let converged = res.get("converged_text").and_then(|v| v.as_str()).unwrap();
    assert!(converged.starts_with("Resmi: Kedaulatan Digital Indonesia"));
    assert!(converged.ends_with("Solusi Berdaulat Tanpa Cloud Asing."));
    assert!(res.get("total_ops_exchanged").and_then(|v| v.as_u64()).unwrap() > 0);
    assert!(res.get("causal_lamport_clock").and_then(|v| v.as_u64()).unwrap() > 0);
}

#[test]
fn test_openxml_native_iso29500_validation_pure_rust() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-openxml", "token-openxml");

    // 1. DOCX validation
    let docx_res = bridge.call_method("openxml.validasi_arsip", &json!({ "tipe": "docx" })).unwrap();
    assert_eq!(docx_res.get("valid").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(docx_res.get("has_content_types").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(docx_res.get("has_rels").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(docx_res.get("primary_part_found").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(docx_res.get("primary_part_path").and_then(|v| v.as_str()), Some("word/document.xml"));
    assert!(docx_res.get("compliance_standard").and_then(|v| v.as_str()).unwrap().contains("ISO/IEC 29500"));

    // 2. XLSX validation
    let xlsx_res = bridge.call_method("openxml.validasi_arsip", &json!({ "tipe": "xlsx" })).unwrap();
    assert_eq!(xlsx_res.get("valid").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(xlsx_res.get("has_content_types").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(xlsx_res.get("has_rels").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(xlsx_res.get("primary_part_found").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(xlsx_res.get("primary_part_path").and_then(|v| v.as_str()), Some("xl/workbook.xml"));

    // 3. PPTX validation
    let pptx_res = bridge.call_method("openxml.validasi_arsip", &json!({ "tipe": "pptx" })).unwrap();
    assert_eq!(pptx_res.get("valid").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(pptx_res.get("has_content_types").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(pptx_res.get("has_rels").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(pptx_res.get("primary_part_found").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(pptx_res.get("primary_part_path").and_then(|v| v.as_str()), Some("ppt/presentation.xml"));
}

#[test]
fn test_widya_office_save_as_and_directory_picker_rpc() {
    let bridge = OfficeBridgeEngine::default_for_window("win-office-save-as", "token-save-as");

    // 1. Uji penjelajah direktori (sistem.jelajahi_folder)
    let browse_res = bridge.call_method("sistem.jelajahi_folder", &json!({ "path": "" })).unwrap();
    assert_eq!(browse_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    assert!(browse_res.get("drives").and_then(|v| v.as_array()).is_some());
    assert!(browse_res.get("quick_paths").and_then(|v| v.as_object()).is_some());

    // 2. Uji penyimpanan berkas fisik ke direktori pilihan (sistem.simpan_berkas_fisik)
    let temp_dir = std::env::temp_dir().join("widya_office_save_as_test");
    let save_res = bridge.call_method("sistem.simpan_berkas_fisik", &json!({
        "folder_path": temp_dir.to_string_lossy().to_string(),
        "file_name": "Dokumen_Uji_SaveAs",
        "format": "docx",
        "content_text": "Isi dokumen uji coba Save As ke direktori pilihan."
    })).unwrap();

    assert_eq!(save_res.get("sukses").and_then(|v| v.as_bool()), Some(true));
    let full_path = save_res.get("full_path").and_then(|v| v.as_str()).unwrap();
    assert!(Path::new(full_path).exists(), "Berkas harus benar-benar tersimpan di direktori pilihan");
    assert!(save_res.get("ukuran_bytes").and_then(|v| v.as_u64()).unwrap() > 0);
    let _ = std::fs::remove_dir_all(&temp_dir);
}





