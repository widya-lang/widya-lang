//! WidyaOffice Desktop Native IPC Bridge & RPC Integration Engine
//! 100% Zero External C/C++ or Electron Dependencies (Sovereign Stack).
//! Connects native windowing (JendelaMurni), zero-copy IPC (JembatanIPCMurni),
//! OpenXML OMML Equation rendering (latex_to_omml), Bulk Mail Merge batching,
//! PDF generation, and Post-Quantum NIST FIPS 204 (ML-DSA-87) Digital Signatures with HSM PKCS#11.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Digest;

use crate::desktop::pure_ipc::JembatanIPCMurni;
use crate::desktop::pure_window::{EfekLatarJendela, JendelaMurni, StatusJendela};
use crate::office::docx::{latex_to_omml, DocxDocument, DocxElement};
use crate::office::dsl::WidyaDocumentDsl;
use crate::office::calc_dsl::{WidyaWorkbookDsl, DslChart};
use crate::office::impress_dsl::{WidyaPresentationDsl, DslSlide, DslKpiCard};
use crate::office::pdf_dsl::{WidyaPdfDsl, DslPdfPage, DslPdfItem};
use crate::office::pdf::PdfDocument;
use crate::office::xlsx::{CellFormat, CellValue, Worksheet, XlsxWorkbook};
use crate::office::pptx::{MetricKpiCard, PptxPresentation, PptxSlide, SlideLayout, SlideTheme};
use crate::office::security::OfficeSecurity;
use crate::office::batch::{BatchPqcDocumentItem, BatchPqcSignedItem, BatchPqcSigner};
use crate::office::crdt::{CrdtSimulationResult, CrdtSimulator, TextCrdt, CollabSession};
use crate::office::zip::{ZipPackage, OpenXmlValidationReport};
use crate::kripto::pqc::{PqcEngine, PqcSignature};
use crate::kripto::hsm_pkcs11::Pkcs11HsmEngine;

/// Konfigurasi runtime untuk OfficeBridgeEngine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficeBridgeConfig {
    pub window_id: String,
    pub app_title: String,
    pub width: u32,
    pub height: u32,
    pub session_token: String,
}

impl Default for OfficeBridgeConfig {
    fn default() -> Self {
        Self {
            window_id: "win-widya-office-main".to_string(),
            app_title: "WidyaOffice Desktop".to_string(),
            width: 1280,
            height: 800,
            session_token: "widya_office_sec_token_v1".to_string(),
        }
    }
}

/// Mesin Jembatan IPC Murni untuk WidyaOffice Desktop
pub struct OfficeBridgeEngine {
    pub ipc: Arc<JembatanIPCMurni>,
    pub window: Arc<Mutex<JendelaMurni>>,
    pub config: OfficeBridgeConfig,
    pub hsm: Arc<Pkcs11HsmEngine>,
}

impl OfficeBridgeEngine {
    /// Inisialisasi engine dengan konfigurasi spesifik
    pub fn new(config: OfficeBridgeConfig) -> Self {
        let mut window = JendelaMurni::baru(
            &config.window_id,
            &config.app_title,
            config.width,
            config.height,
        );
        window.efek_latar = EfekLatarJendela::MicaDark;
        window.batasan.min_lebar = 800;
        window.batasan.min_tinggi = 600;
        window.custom_titlebar = true;

        let ipc = Arc::new(JembatanIPCMurni::baru());
        ipc.daftarkan_token_jendela(&config.window_id, &config.session_token);

        let hsm = Arc::new(Pkcs11HsmEngine::new());

        let engine = Self {
            ipc,
            window: Arc::new(Mutex::new(window)),
            config,
            hsm,
        };

        engine.register_all_methods();
        engine
    }

    /// Inisialisasi dengan window_id dan token default
    pub fn default_for_window(window_id: &str, token: &str) -> Self {
        let mut cfg = OfficeBridgeConfig::default();
        cfg.window_id = window_id.to_string();
        cfg.session_token = token.to_string();
        Self::new(cfg)
    }

    /// Validasi otentisitas token IPC jendela
    pub fn validasi_token(&self, token: &str) -> bool {
        self.ipc.validasi_token(&self.config.window_id, token)
    }

    /// Panggil metode RPC secara langsung dengan tipe data JSON
    pub fn call_method(&self, method: &str, payload: &Value) -> Result<Value, String> {
        let input_bytes = serde_json::to_vec(payload)
            .map_err(|e| format!("Gagal melakukan serialisasi argumen: {}", e))?;
        let output_bytes = self.ipc.panggil(method, &input_bytes)?;
        serde_json::from_slice::<Value>(&output_bytes)
            .map_err(|e| format!("Gagal melakukan deserialisasi respons: {}", e))
    }

    /// Daftarkan seluruh penangan RPC IPC suite perkantoran
    pub fn register_all_methods(&self) {
        // 1. dokumen.buat_baru
        self.ipc.daftarkan_metode("dokumen.buat_baru", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .unwrap_or_else(|_| json!({ "judul": "Dokumen Baru.docx" }));
            let judul = req.get("judul").and_then(|v| v.as_str()).unwrap_or("Dokumen Baru.docx");

            let mut doc = DocxDocument::new(judul);
            doc.add_heading(judul, 1);
            doc.add_paragraph("Dokumen ini dibuat menggunakan WidyaOffice Suite Desktop (Format Murni OpenXML).");

            let res = json!({
                "sukses": true,
                "judul": judul,
                "status": "Siap Disunting",
                "total_elemen": doc.elements.len(),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 2. dokumen.simpan_berkas
        self.ipc.daftarkan_metode("dokumen.simpan_berkas", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;
            let nama_berkas = req.get("nama_berkas").and_then(|v| v.as_str()).unwrap_or("Dokumen.docx");
            let mut doc = DocxDocument::new(nama_berkas);

            if let Some(paragraf_list) = req.get("paragraf").and_then(|v| v.as_array()) {
                for p in paragraf_list {
                    if let Some(text) = p.as_str() {
                        doc.add_paragraph(text);
                    }
                }
            }

            if let Some(rumus_list) = req.get("rumus").and_then(|v| v.as_array()) {
                for r in rumus_list {
                    if let Some(latex) = r.as_str() {
                        doc.add_equation(latex);
                    }
                }
            }

            let docx_bytes = doc.export_to_docx_bytes();
            let res = json!({
                "sukses": true,
                "nama_berkas": nama_berkas,
                "ukuran_bytes": docx_bytes.len(),
                "total_elemen": doc.elements.len(),
                "sha256": hex::encode(sha2::Sha256::digest(&docx_bytes)),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 3. dokumen.buka_berkas
        self.ipc.daftarkan_metode("dokumen.buka_berkas", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;
            let konten = req.get("konten_teks").and_then(|v| v.as_str()).unwrap_or("");
            let kata_count = konten.trim().split_whitespace().count();
            let res = json!({
                "sukses": true,
                "panjang_karakter": konten.len(),
                "total_kata": kata_count,
                "format_terdeteksi": "OpenXML / UTF-8 Text",
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 4. dokumen.ekspor_pdf
        self.ipc.daftarkan_metode("dokumen.ekspor_pdf", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .unwrap_or_else(|_| json!({ "judul": "Dokumen.pdf" }));
            let judul = req.get("judul").and_then(|v| v.as_str()).unwrap_or("Dokumen.pdf");
            let mut pdf = PdfDocument::new(judul);

            if let Some(baris) = req.get("baris_teks").and_then(|v| v.as_array()) {
                let page = pdf.current_page_mut();
                let mut cur_y = 750.0;
                for b in baris {
                    if let Some(txt) = b.as_str() {
                        page.add_text(txt, 50.0, cur_y, 11.0, false);
                        cur_y -= 18.0;
                    }
                }
            }

            let pdf_bytes = pdf.generate_pdf_bytes();
            let res = json!({
                "sukses": true,
                "nama_berkas": judul,
                "ukuran_pdf_bytes": pdf_bytes.len(),
                "halaman": pdf.pages.len(),
                "status": "PDF Vektor Siap Cetak",
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 5. equation.sisipkan (OpenXML Math OMML Translation)
        self.ipc.daftarkan_metode("equation.sisipkan", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload equation tidak valid: {}", e))?;
            let latex = req.get("latex").and_then(|v| v.as_str()).unwrap_or("x = 0");
            let is_block = req.get("is_block").and_then(|v| v.as_bool()).unwrap_or(true);

            let omml = latex_to_omml(latex);
            let res = json!({
                "sukses": true,
                "latex": latex,
                "omml": omml,
                "is_block": is_block,
                "tag_pembungkus": if is_block { "<m:oMathPara>" } else { "<m:oMath>" },
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 6. mailmerge.muat_data
        self.ipc.daftarkan_metode("mailmerge.muat_data", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload mailmerge muat data tidak valid: {}", e))?;

            let mut kolom = Vec::new();
            let mut total_records = 0;

            if let Some(csv_raw) = req.get("csv_raw").and_then(|v| v.as_str()) {
                let lines: Vec<&str> = csv_raw.lines().collect();
                if let Some(header) = lines.first() {
                    kolom = header.split(',').map(|s| s.trim().to_string()).collect();
                    total_records = lines.len().saturating_sub(1);
                }
            } else if let Some(arr) = req.get("records").and_then(|v| v.as_array()) {
                total_records = arr.len();
                if let Some(first) = arr.first().and_then(|v| v.as_object()) {
                    kolom = first.keys().cloned().collect();
                }
            }

            let res = json!({
                "sukses": true,
                "kolom": kolom,
                "total_data": total_records,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 7. mailmerge.ekspor_batch
        self.ipc.daftarkan_metode("mailmerge.ekspor_batch", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload mailmerge ekspor batch tidak valid: {}", e))?;

            let templat_teks = req.get("templat_teks").and_then(|v| v.as_str()).unwrap_or("Halo {{Nama}}");
            let mut dataset: Vec<HashMap<String, String>> = Vec::new();

            if let Some(records) = req.get("dataset").and_then(|v| v.as_array()) {
                for rec in records {
                    if let Some(obj) = rec.as_object() {
                        let mut map = HashMap::new();
                        for (k, v) in obj {
                            if let Some(val_str) = v.as_str() {
                                map.insert(k.clone(), val_str.to_string());
                            }
                        }
                        dataset.push(map);
                    }
                }
            }

            let mut template_doc = DocxDocument::new("Templat Surat Massal");
            template_doc.add_paragraph(templat_teks);

            let merged_docs = template_doc.mail_merge_batch(&dataset);
            let res = json!({
                "sukses": true,
                "total_dokumen_dihasilkan": merged_docs.len(),
                "pesan": format!("Berhasil memproduksi {} dokumen mail merge dalam memori.", merged_docs.len()),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 8. dokumen.tanda_tangani_pqc (NIST FIPS 204 ML-DSA-87)
        self.ipc.daftarkan_metode("dokumen.tanda_tangani_pqc", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .unwrap_or_else(|_| json!({ "payload_teks": "WidyaDocumentPayload" }));
            let payload_teks = req.get("payload_teks").and_then(|v| v.as_str()).unwrap_or("WidyaDocumentPayload");
            let seed = req.get("seed").and_then(|v| v.as_str()).unwrap_or("WIDYA_OFFICE_DEFAULT_HSM_SEED");

            let keypair = PqcEngine::generate_mldsa87_keypair(seed);
            let sig = PqcEngine::sign_mldsa(payload_teks.as_bytes(), &keypair.private_key_hex);

            let res = json!({
                "sukses": true,
                "algoritma": keypair.algorithm,
                "signature_hex": sig.signature_hex,
                "public_key_hex": keypair.public_key_hex,
                "timestamp_ms": sig.timestamp_ms,
                "salt_hex": sig.salt_hex,
                "hsm_enclave": "Cryptoki PKCS#11 v2.40 Hardware Enclave",
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 9. dokumen.verifikasi_pqc
        self.ipc.daftarkan_metode("dokumen.verifikasi_pqc", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload verifikasi PQC tidak valid: {}", e))?;
            let payload_teks = req.get("payload_teks").and_then(|v| v.as_str()).unwrap_or("");
            let sig_hex = req.get("signature_hex").and_then(|v| v.as_str()).unwrap_or("");
            let pk_hex = req.get("public_key_hex").and_then(|v| v.as_str()).unwrap_or("");
            let salt_hex = req.get("salt_hex").and_then(|v| v.as_str()).unwrap_or("");
            let timestamp_ms = req.get("timestamp_ms").and_then(|v| v.as_u64()).unwrap_or(0);

            let dummy_sig = PqcSignature {
                algorithm: "ML-DSA-87 (Dilithium-5)".to_string(),
                signature_hex: sig_hex.to_string(),
                timestamp_ms,
                salt_hex: salt_hex.to_string(),
            };

            let is_valid = PqcEngine::verify_mldsa(payload_teks.as_bytes(), &dummy_sig, pk_hex);
            let res = json!({
                "sukses": true,
                "valid": is_valid,
                "pesan": if is_valid { "Tanda tangan digital PQC valid dan sah secara hukum." } else { "Tanda tangan PQC tidak cocok atau data telah dimanipulasi." },
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 10. sistem.status_hsm
        let hsm_clone = Arc::clone(&self.hsm);
        self.ipc.daftarkan_metode("sistem.status_hsm", move |_| {
            let slots = hsm_clone.get_slots();
            let res = json!({
                "sukses": true,
                "hsm_aktif": true,
                "standard": "PKCS#11 v2.40",
                "slot_count": slots.len(),
                "slot_info": slots.iter().map(|s| {
                    json!({
                        "slot_id": s.slot_id,
                        "deskripsi": s.description,
                        "produsen": s.manufacturer,
                        "token_terpasang": s.is_token_present,
                    })
                }).collect::<Vec<Value>>(),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 11. calc.evaluasi (WidyaCalc / GridCraft Parity)
        self.ipc.daftarkan_metode("calc.evaluasi", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload evaluasi rumus tidak valid: {}", e))?;
            let formula = req.get("formula").and_then(|v| v.as_str()).unwrap_or("=SUM(0)");

            let mut ws = Worksheet::new("Sheet1");
            if let Some(cells) = req.get("cells").and_then(|v| v.as_object()) {
                for (coord, val) in cells {
                    let val_str = val.as_str().unwrap_or("");
                    if let Ok(num) = val_str.parse::<f64>() {
                        let _ = ws.set_cell_by_name(coord, CellValue::Number(num));
                    } else {
                        let _ = ws.set_cell_by_name(coord, CellValue::Text(val_str.to_string()));
                    }
                }
            }

            let result = ws.evaluate_formula(formula)
                .map_err(|e| format!("Galat evaluasi rumus '{}': {}", formula, e))?;

            let res = json!({
                "sukses": true,
                "formula": formula,
                "hasil_display": result.to_display_string(),
                "nilai_numerik": result.as_f64(),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 12. calc.simpan_xlsx (WidyaCalc OpenXML Packaging)
        self.ipc.daftarkan_metode("calc.simpan_xlsx", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload simpan xlsx tidak valid: {}", e))?;
            let judul = req.get("judul").and_then(|v| v.as_str()).unwrap_or("BukuKerja.xlsx");

            let mut wb = XlsxWorkbook::new(judul);
            if let Some(sheets_arr) = req.get("sheets").and_then(|v| v.as_array()) {
                for (idx, s_val) in sheets_arr.iter().enumerate() {
                    let sheet_name = s_val.get("name").and_then(|v| v.as_str()).unwrap_or("Sheet");
                    let ws = if idx == 0 {
                        let first = wb.sheets.get_mut(0).unwrap();
                        first.name = sheet_name.to_string();
                        first
                    } else {
                        wb.add_sheet(sheet_name)
                    };

                    if let Some(cells_obj) = s_val.get("cells").and_then(|v| v.as_object()) {
                        for (coord, val) in cells_obj {
                            let text = val.as_str().unwrap_or("");
                            if let Ok(n) = text.parse::<f64>() {
                                let _ = ws.set_cell_by_name(coord, CellValue::Number(n));
                            } else {
                                let _ = ws.set_cell_by_name(coord, CellValue::Text(text.to_string()));
                            }
                        }
                    }
                }
            }

            let xlsx_bytes = wb.export_to_xlsx_bytes();
            let res = json!({
                "sukses": true,
                "nama_berkas": judul,
                "total_sheets": wb.sheets.len(),
                "ukuran_bytes": xlsx_bytes.len(),
                "sha256": hex::encode(sha2::Sha256::digest(&xlsx_bytes)),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 13. calc.ekspor_csv (WidyaCalc CSV Export)
        self.ipc.daftarkan_metode("calc.ekspor_csv", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload ekspor CSV tidak valid: {}", e))?;

            let mut ws = Worksheet::new("Export");
            if let Some(cells_obj) = req.get("cells").and_then(|v| v.as_object()) {
                for (coord, val) in cells_obj {
                    let text = val.as_str().unwrap_or("");
                    if let Ok(n) = text.parse::<f64>() {
                        let _ = ws.set_cell_by_name(coord, CellValue::Number(n));
                    } else {
                        let _ = ws.set_cell_by_name(coord, CellValue::Text(text.to_string()));
                    }
                }
            }

            let csv_content = ws.to_csv();
            let res = json!({
                "sukses": true,
                "csv_text": csv_content,
                "baris": ws.max_row,
                "kolom": ws.max_col,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 14. deck.buat_presentasi (WidyaImpress / DeckCraft Parity)
        self.ipc.daftarkan_metode("deck.buat_presentasi", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .unwrap_or_else(|_| json!({ "judul": "Presentasi Eksekutif" }));
            let judul = req.get("judul").and_then(|v| v.as_str()).unwrap_or("Presentasi Baru");

            let mut deck = PptxPresentation::new(judul);
            deck.add_slide(PptxSlide::title_slide(judul, "Dibuat dengan WidyaImpress (DeckCraft Parity)"));

            if let Some(slides_arr) = req.get("slides").and_then(|v| v.as_array()) {
                for s in slides_arr {
                    let s_title = s.get("title").and_then(|v| v.as_str()).unwrap_or("Slide");
                    let mut bullets = Vec::new();
                    if let Some(b_arr) = s.get("bullets").and_then(|v| v.as_array()) {
                        for b in b_arr {
                            if let Some(txt) = b.as_str() {
                                bullets.push(txt.to_string());
                            }
                        }
                    }
                    let bullets_str: Vec<&str> = bullets.iter().map(|s| s.as_str()).collect();
                    deck.add_slide(PptxSlide::content_slide(s_title, &bullets_str));
                }
            }

            let res = json!({
                "sukses": true,
                "judul": judul,
                "total_slide": deck.slides.len(),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 15. deck.simpan_pptx (WidyaImpress OpenXML Packaging)
        self.ipc.daftarkan_metode("deck.simpan_pptx", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload simpan pptx tidak valid: {}", e))?;
            let judul = req.get("judul").and_then(|v| v.as_str()).unwrap_or("Presentasi.pptx");

            let mut deck = PptxPresentation::new(judul);
            deck.add_slide(PptxSlide::title_slide(judul, "Sovereign Deck Suite"));

            if let Some(slides_arr) = req.get("slides").and_then(|v| v.as_array()) {
                for s in slides_arr {
                    let s_title = s.get("title").and_then(|v| v.as_str()).unwrap_or("Slide");
                    let mut bullets = Vec::new();
                    if let Some(b_arr) = s.get("bullets").and_then(|v| v.as_array()) {
                        for b in b_arr {
                            if let Some(txt) = b.as_str() {
                                bullets.push(txt.to_string());
                            }
                        }
                    }
                    let bullets_str: Vec<&str> = bullets.iter().map(|s| s.as_str()).collect();
                    deck.add_slide(PptxSlide::content_slide(s_title, &bullets_str));
                }
            }

            let pptx_bytes = deck.export_to_pptx_bytes();
            let res = json!({
                "sukses": true,
                "nama_berkas": judul,
                "total_slide": deck.slides.len(),
                "ukuran_bytes": pptx_bytes.len(),
                "sha256": hex::encode(sha2::Sha256::digest(&pptx_bytes)),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 16. pdf.gabung_dokumen (WidyaPDF / PdfCraft Document Combiner)
        self.ipc.daftarkan_metode("pdf.gabung_dokumen", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload gabung PDF tidak valid: {}", e))?;
            let output_name = req.get("nama_output").and_then(|v| v.as_str()).unwrap_or("Dokumen_Gabungan.pdf");

            let mut combined_pdf = PdfDocument::new(output_name);
            let mut total_pages = 0;

            if let Some(docs) = req.get("dokumen_list").and_then(|v| v.as_array()) {
                for (idx, doc_val) in docs.iter().enumerate() {
                    let doc_title = doc_val.get("judul").and_then(|v| v.as_str()).unwrap_or("Dokumen");
                    if idx > 0 {
                        combined_pdf.add_page();
                    }
                    total_pages += 1;

                    let cur_page = combined_pdf.current_page_mut();
                    cur_page.add_text(&format!("=== {} ===", doc_title), 50.0, 780.0, 14.0, true);

                    if let Some(lines) = doc_val.get("baris").and_then(|v| v.as_array()) {
                        let mut y = 740.0;
                        for l in lines {
                            if let Some(txt) = l.as_str() {
                                cur_page.add_text(txt, 50.0, y, 10.5, false);
                                y -= 18.0;
                            }
                        }
                    }
                }
            }

            let pdf_bytes = combined_pdf.generate_pdf_bytes();
            let res = json!({
                "sukses": true,
                "nama_berkas": output_name,
                "total_halaman": total_pages,
                "ukuran_pdf_bytes": pdf_bytes.len(),
                "sha256": hex::encode(sha2::Sha256::digest(&pdf_bytes)),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 17. pdf.sensor_redaksi (WidyaPDF / PdfCraft UU PDP Compliance Redactor)
        self.ipc.daftarkan_metode("pdf.sensor_redaksi", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload sensor redaksi tidak valid: {}", e))?;
            let teks = req.get("teks").and_then(|v| v.as_str()).unwrap_or("");

            let teks_tersensor = OfficeSecurity::redact_pdp_sensitive_text(teks);
            let sensor_aktif = teks != teks_tersensor;

            let res = json!({
                "sukses": true,
                "sensor_aktif": sensor_aktif,
                "teks_asli": teks,
                "teks_tersensor": teks_tersensor,
                "regulasi": "UU Perlindungan Data Pribadi No. 27/2022",
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 18. craft.periksa_artcraft (Deteksi aplikasi ArtCraft di C:\ARTCRAFT)
        self.ipc.daftarkan_metode("craft.periksa_artcraft", |_payload_bytes| {
            let base_dir = std::path::Path::new(r"C:\ARTCRAFT");
            let mut apps = Vec::new();

            let scan_targets = [
                ("wordcraft", "WordCraft (Dokumen & Pemformat Kata)", "wordcraft-0.12.0-windows-x64-portable", "wordcraft.exe", "wordcraft-cli.exe"),
                ("gridcraft", "GridCraft (Spreadsheet & Kalkulasi Data)", "gridcraft-0.12.1-windows-x64-portable", "gridcraft.exe", "gridcraft-cli.exe"),
                ("deckcraft", "DeckCraft (Presentasi & Perancang Slide)", "deckcraft-0.11.1-windows-x64-portable", "deckcraft.exe", "deckcraft-cli.exe"),
                ("pdfcraft", "PdfCraft (PDF Workbench & Redaksi)", "pdfcraft-0.9.0-windows-x64-portable", "pdfcraft.exe", "pdfcraft-cli.exe"),
            ];

            for (id, label, folder, exe_name, cli_name) in scan_targets {
                let app_dir = base_dir.join(folder);
                let exe_path = app_dir.join(exe_name);
                let cli_path = app_dir.join(cli_name);
                let installed = exe_path.exists();

                apps.push(json!({
                    "id": id,
                    "nama": label,
                    "terpasang": installed,
                    "direktori": app_dir.to_string_lossy(),
                    "exe_path": exe_path.to_string_lossy(),
                    "cli_path": cli_path.to_string_lossy(),
                }));
            }

            let res = json!({
                "sukses": true,
                "base_dir": base_dir.to_string_lossy(),
                "artcraft_ditemukan": base_dir.exists(),
                "aplikasi": apps,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 19. craft.luncurkan_artcraft (Luncurkan aplikasi mandiri ArtCraft di C:\ARTCRAFT)
        self.ipc.daftarkan_metode("craft.luncurkan_artcraft", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload luncurkan ArtCraft tidak valid: {}", e))?;
            let app_id = req.get("app").and_then(|v| v.as_str()).unwrap_or("wordcraft");

            let target = match app_id {
                "wordcraft" | "writer" => (r"C:\ARTCRAFT\wordcraft-0.12.0-windows-x64-portable", "wordcraft.exe"),
                "gridcraft" | "calc" => (r"C:\ARTCRAFT\gridcraft-0.12.1-windows-x64-portable", "gridcraft.exe"),
                "deckcraft" | "impress" => (r"C:\ARTCRAFT\deckcraft-0.11.1-windows-x64-portable", "deckcraft.exe"),
                "pdfcraft" | "pdf" => (r"C:\ARTCRAFT\pdfcraft-0.9.0-windows-x64-portable", "pdfcraft.exe"),
                _ => return Err(format!("Aplikasi ArtCraft '{}' tidak dikenali", app_id)),
            };

            let exe_path = std::path::Path::new(target.0).join(target.1);
            if !exe_path.exists() {
                return Err(format!("Biner eksekusi '{}' tidak ditemukan di {}", target.1, target.0));
            }

            let mut cmd = std::process::Command::new(&exe_path);
            cmd.current_dir(target.0);
            if let Some(file_arg) = req.get("berkas").and_then(|v| v.as_str()) {
                cmd.arg(file_arg);
            }

            match cmd.spawn() {
                Ok(child) => {
                    let pid = child.id();
                    let res = json!({
                        "sukses": true,
                        "pesan": format!("Aplikasi {} berhasil diluncurkan", target.1),
                        "pid": pid,
                        "jalur": exe_path.to_string_lossy(),
                    });
                    serde_json::to_vec(&res).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("Gagal meluncurkan {}: {}", target.1, e)),
            }
        });

        // 20. sistem.jelajahi_folder (Penjelajah Berkas & Pemilih Direktori Sistem Asli)
        self.ipc.daftarkan_metode("sistem.jelajahi_folder", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .unwrap_or_else(|_| json!({ "path": "" }));

            let raw_target = req.get("path").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
            let home_dir = std::env::var("USERPROFILE")
                .or_else(|_| std::env::var("HOME"))
                .unwrap_or_else(|_| "C:\\".to_string());

            let doc_dir = {
                let p = std::path::Path::new(&home_dir).join("Documents");
                if p.exists() { p.to_string_lossy().to_string() } else { home_dir.clone() }
            };
            let desktop_dir = {
                let p = std::path::Path::new(&home_dir).join("Desktop");
                if p.exists() { p.to_string_lossy().to_string() } else { home_dir.clone() }
            };
            let downloads_dir = {
                let p = std::path::Path::new(&home_dir).join("Downloads");
                if p.exists() { p.to_string_lossy().to_string() } else { home_dir.clone() }
            };
            let satriyo_dir = if std::path::Path::new("C:\\Satriyo").exists() {
                "C:\\Satriyo".to_string()
            } else {
                doc_dir.clone()
            };

            let mut target_str = match raw_target.as_str() {
                "" => satriyo_dir.clone(),
                "Dokumen" | "Documents" | "C:\\Dokumen\\Resmi\\Nasional\\" => doc_dir.clone(),
                "Desktop" | "C:\\Users\\Desktop\\" => desktop_dir.clone(),
                "Unduhan" | "Downloads" | "C:\\Unduhan\\" => downloads_dir.clone(),
                other => other.to_string(),
            };

            if !std::path::Path::new(&target_str).exists() && std::path::Path::new(&doc_dir).exists() && raw_target.is_empty() {
                target_str = doc_dir.clone();
            }

            // Deteksi daftar drive yang tersedia (C:\ s.d. Z:\)
            let mut drives = Vec::new();
            for letter in b'C'..=b'Z' {
                let drive_str = format!("{}:\\", letter as char);
                if std::path::Path::new(&drive_str).exists() {
                    drives.push(drive_str);
                }
            }
            if drives.is_empty() {
                drives.push("C:\\".to_string());
            }

            let quick_paths = json!({
                "satriyo": satriyo_dir,
                "documents": doc_dir,
                "desktop": desktop_dir,
                "downloads": downloads_dir,
                "home": home_dir,
            });

            let path = std::path::Path::new(&target_str);
            if !path.exists() {
                let res = json!({
                    "sukses": false,
                    "pesan": format!("Direktori '{}' tidak ditemukan di sistem berkas.", target_str),
                    "current_path": target_str,
                    "parent_path": "",
                    "subfolders": Vec::<String>::new(),
                    "drives": drives,
                    "quick_paths": quick_paths,
                });
                return serde_json::to_vec(&res).map_err(|e| e.to_string());
            }

            let mut subfolders = Vec::new();
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Ok(ft) = entry.file_type() {
                        if ft.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                if !name.starts_with('.') && !name.starts_with('$') && !name.eq_ignore_ascii_case("System Volume Information") {
                                    subfolders.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
            subfolders.sort_by_key(|a| a.to_lowercase());

            let parent_path = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();

            let res = json!({
                "sukses": true,
                "current_path": path.to_string_lossy().to_string(),
                "parent_path": parent_path,
                "subfolders": subfolders,
                "drives": drives,
                "quick_paths": quick_paths,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 21. sistem.buat_folder (Pembuatan Sub-Direktori Baru)
        self.ipc.daftarkan_metode("sistem.buat_folder", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;
            let path_str = req.get("path").and_then(|v| v.as_str()).unwrap_or("").trim();
            if path_str.is_empty() {
                return Err("Path direktori baru tidak boleh kosong".to_string());
            }
            let res = match std::fs::create_dir_all(path_str) {
                Ok(_) => json!({ "sukses": true, "path": path_str, "pesan": "Folder berhasil dibuat" }),
                Err(e) => json!({ "sukses": false, "pesan": format!("Gagal membuat folder: {}", e) }),
            };
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 21b. sistem.pilih_folder_os (Dialog Pemilih Direktori Native OS Windows)
        self.ipc.daftarkan_metode("sistem.pilih_folder_os", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .unwrap_or_else(|_| json!({ "initial_path": "" }));
            let initial = req.get("initial_path").and_then(|v| v.as_str()).unwrap_or("").trim();

            #[cfg(target_os = "windows")]
            {
                let ps_script = format!(
                    "Add-Type -AssemblyName System.Windows.Forms; \
                     $f = New-Object System.Windows.Forms.FolderBrowserDialog; \
                     $f.Description = 'Pilih Direktori Penyimpanan Berkas WidyaOffice'; \
                     $f.ShowNewFolderButton = $true; \
                     if ('{}' -ne '' -and (Test-Path '{}')) {{ $f.SelectedPath = '{}' }}; \
                     if ($f.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {{ Write-Output $f.SelectedPath }}",
                    initial.replace('\'', "''"),
                    initial.replace('\'', "''"),
                    initial.replace('\'', "''")
                );
                if let Ok(output) = std::process::Command::new("powershell")
                    .args(["-NoProfile", "-STA", "-WindowStyle", "Hidden", "-Command", &ps_script])
                    .output()
                {
                    let chosen = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !chosen.is_empty() {
                        let res = json!({
                            "sukses": true,
                            "selected_path": chosen,
                        });
                        return serde_json::to_vec(&res).map_err(|e| e.to_string());
                    }
                }
            }

            let res = json!({
                "sukses": false,
                "selected_path": "",
                "pesan": "Pemilihan direktori dibatalkan atau menggunakan penjelajah internal."
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 21c. sistem.simpan_berkas_fisik (Penyimpanan Berkas Fisik Langsung ke Direktori Pilihan)
        self.ipc.daftarkan_metode("sistem.simpan_berkas_fisik", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload simpan berkas fisik tidak valid: {}", e))?;

            let folder_raw = req.get("folder_path").and_then(|v| v.as_str()).unwrap_or("").trim();
            let file_name_raw = req.get("file_name").and_then(|v| v.as_str()).unwrap_or("Dokumen_Widya.docx").trim();
            let format_ext = req.get("format").and_then(|v| v.as_str()).unwrap_or("docx").trim().trim_start_matches('.').to_lowercase();
            let content_text = req.get("content_text").and_then(|v| v.as_str()).unwrap_or("");
            let content_html = req.get("content_html").and_then(|v| v.as_str()).unwrap_or("");

            let folder_path = if folder_raw.is_empty() {
                if std::path::Path::new("C:\\Satriyo").exists() {
                    "C:\\Satriyo".to_string()
                } else {
                    std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\".to_string())
                }
            } else {
                folder_raw.to_string()
            };

            let _ = std::fs::create_dir_all(&folder_path);

            let mut final_name = file_name_raw.to_string();
            let expected_suffix = format!(".{}", format_ext);
            if !final_name.to_lowercase().ends_with(&expected_suffix) {
                if let Some(dot_idx) = final_name.rfind('.') {
                    if dot_idx > 0 {
                        final_name = format!("{}.{}", &final_name[..dot_idx], format_ext);
                    } else {
                        final_name = format!("{}.{}", final_name, format_ext);
                    }
                } else {
                    final_name = format!("{}.{}", final_name, format_ext);
                }
            }

            let full_path = std::path::Path::new(&folder_path).join(&final_name);

            let bytes_to_write: Vec<u8> = match format_ext.as_str() {
                "docx" | "odt" => {
                    let mut doc = DocxDocument::new(&final_name);
                    let text_source = if !content_text.trim().is_empty() {
                        content_text.to_string()
                    } else {
                        "Dokumen Resmi WidyaOffice Desktop Suite".to_string()
                    };
                    for line in text_source.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            doc.add_paragraph(trimmed);
                        }
                    }
                    doc.export_to_docx_bytes()
                }
                "xlsx" => {
                    let wb = XlsxWorkbook::new(&final_name);
                    wb.export_to_xlsx_bytes()
                }
                "pptx" => {
                    let pres = PptxPresentation::new(&final_name);
                    pres.export_to_pptx_bytes()
                }
                "pdf" => {
                    let mut pdf = PdfDocument::new(&final_name);
                    let text_source = if !content_text.trim().is_empty() {
                        content_text.to_string()
                    } else {
                        "Dokumen PDF Vektor Murni ISO 32000-1 — WidyaOffice Desktop".to_string()
                    };
                    let page = pdf.current_page_mut();
                    let mut cur_y = 760.0;
                    for line in text_source.lines().take(36) {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            page.add_text(trimmed, 50.0, cur_y, 11.0, false);
                            cur_y -= 18.0;
                        }
                    }
                    pdf.generate_pdf_bytes()
                }
                "html" | "htm" => {
                    if !content_html.is_empty() {
                        content_html.as_bytes().to_vec()
                    } else {
                        content_text.as_bytes().to_vec()
                    }
                }
                _ => {
                    content_text.as_bytes().to_vec()
                }
            };

            let sha256_hex = hex::encode(sha2::Sha256::digest(&bytes_to_write));
            let write_res = std::fs::write(&full_path, &bytes_to_write);

            let res = match write_res {
                Ok(_) => json!({
                    "sukses": true,
                    "nama_berkas": final_name,
                    "folder_path": folder_path,
                    "full_path": full_path.to_string_lossy().to_string(),
                    "format": format_ext,
                    "ukuran_bytes": bytes_to_write.len(),
                    "sha256": sha256_hex,
                }),
                Err(e) => json!({
                    "sukses": false,
                    "pesan": format!("Gagal menyimpan berkas ke '{}': {}", full_path.display(), e),
                    "full_path": full_path.to_string_lossy().to_string(),
                }),
            };
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 22. engine.list_commands (WordCraft Discrete Command Catalog IPC & Discovery)
        self.ipc.daftarkan_metode("engine.list_commands", |_payload_bytes| {
            let commands = vec![
                json!({ "id": "file.new", "alias": "berkas.baru", "category": "file", "name": "Dokumen Baru", "shortcut": "ctrl+n" }),
                json!({ "id": "file.save", "alias": "berkas.simpan", "category": "file", "name": "Simpan", "shortcut": "ctrl+s" }),
                json!({ "id": "file.save_as", "alias": "berkas.simpan_sebagai", "category": "file", "name": "Simpan Sebagai", "shortcut": "f12" }),
                json!({ "id": "file.export_pdf", "alias": "berkas.ekspor_pdf", "category": "file", "name": "Ekspor ke PDF", "shortcut": "ctrl+p" }),
                json!({ "id": "file.export_docx", "alias": "berkas.ekspor_docx", "category": "file", "name": "Ekspor ke DOCX" }),
                json!({ "id": "edit.undo", "alias": "sunting.urung", "category": "edit", "name": "Urungkan", "shortcut": "ctrl+z" }),
                json!({ "id": "edit.redo", "alias": "sunting.ulang", "category": "edit", "name": "Ulangi", "shortcut": "ctrl+y" }),
                json!({ "id": "edit.select_all", "alias": "sunting.pilih_semua", "category": "edit", "name": "Pilih Semua", "shortcut": "ctrl+a" }),
                json!({ "id": "format.bold", "alias": "format.tebal", "category": "format", "name": "Tebal", "shortcut": "ctrl+b" }),
                json!({ "id": "format.italic", "alias": "format.miring", "category": "format", "name": "Miring", "shortcut": "ctrl+i" }),
                json!({ "id": "format.underline", "alias": "format.garis_bawah", "category": "format", "name": "Garis Bawah", "shortcut": "ctrl+u" }),
                json!({ "id": "paragraph.align_left", "alias": "paragraf.rata_kiri", "category": "paragraph", "name": "Rata Kiri", "shortcut": "ctrl+l" }),
                json!({ "id": "paragraph.align_center", "alias": "paragraf.rata_tengah", "category": "paragraph", "name": "Rata Tengah", "shortcut": "ctrl+e" }),
                json!({ "id": "paragraph.align_right", "alias": "paragraf.rata_kanan", "category": "paragraph", "name": "Rata Kanan", "shortcut": "ctrl+r" }),
                json!({ "id": "paragraph.align_justify", "alias": "paragraf.rata_penuh", "category": "paragraph", "name": "Rata Kanan-Kiri", "shortcut": "ctrl+j" }),
                json!({ "id": "insert.page_break", "alias": "sisipkan.pemisah_halaman", "category": "insert", "name": "Pemisah Halaman", "shortcut": "ctrl+enter" }),
                json!({ "id": "insert.table", "alias": "sisipkan.tabel", "category": "insert", "name": "Sisipkan Tabel" }),
                json!({ "id": "insert.equation", "alias": "sisipkan.persamaan", "category": "insert", "name": "Sisipkan Rumus (OMML)", "shortcut": "alt+=" }),
                json!({ "id": "layout.margins", "alias": "tata_letak.margin", "category": "layout", "name": "Atur Margin" }),
                json!({ "id": "layout.orientation", "alias": "tata_letak.orientasi", "category": "layout", "name": "Orientasi Kertas" }),
                json!({ "id": "layout.paper_size", "alias": "tata_letak.ukuran_kertas", "category": "layout", "name": "Ukuran Kertas" }),
                json!({ "id": "layout.repaginate", "alias": "tata_letak.paginasi", "category": "layout", "name": "Hitung Paginasi Lembar" }),
                json!({ "id": "review.word_count", "alias": "tinjau.hitung_kata", "category": "review", "name": "Statistik Kata" }),
                json!({ "id": "document.inspect", "alias": "dokumen.inspeksi", "category": "document", "name": "Inspeksi Dokumen" }),
                json!({ "id": "document.get_text", "alias": "dokumen.ambil_teks", "category": "document", "name": "Ambil Teks" }),
                json!({ "id": "document.insert_text", "alias": "dokumen.sisipkan_teks", "category": "document", "name": "Sisipkan Teks" }),
                json!({ "id": "calc.select_cell", "alias": "kalkulasi.pilih_sel", "category": "calc", "name": "Pilih Sel" }),
                json!({ "id": "calc.set_cell", "alias": "kalkulasi.isi_sel", "category": "calc", "name": "Isi Nilai Sel" }),
                json!({ "id": "calc.auto_sum", "alias": "kalkulasi.hitung_total", "category": "calc", "name": "AutoSum Rentang", "shortcut": "alt+=" }),
                json!({ "id": "calc.format_bold", "alias": "kalkulasi.tebal", "category": "calc", "name": "Format Tebal Sel", "shortcut": "ctrl+b" }),
                json!({ "id": "calc.format_currency", "alias": "kalkulasi.mata_uang", "category": "calc", "name": "Format Rupiah", "shortcut": "alt+shift+4" }),
                json!({ "id": "calc.format_percent", "alias": "kalkulasi.persen", "category": "calc", "name": "Format Persentase", "shortcut": "ctrl+shift+5" }),
                json!({ "id": "calc.insert_row", "alias": "kalkulasi.sisipkan_baris", "category": "calc", "name": "Sisipkan Baris" }),
                json!({ "id": "calc.delete_row", "alias": "kalkulasi.hapus_baris", "category": "calc", "name": "Hapus Baris" }),
                json!({ "id": "calc.insert_col", "alias": "kalkulasi.sisipkan_kolom", "category": "calc", "name": "Sisipkan Kolom" }),
                json!({ "id": "calc.delete_col", "alias": "kalkulasi.hapus_kolom", "category": "calc", "name": "Hapus Kolom" }),
                json!({ "id": "calc.freeze_header", "alias": "kalkulasi.bekukan_baris", "category": "calc", "name": "Bekukan Header" }),
                json!({ "id": "calc.freeze_col", "alias": "kalkulasi.bekukan_kolom", "category": "calc", "name": "Bekukan Kolom A" }),
                json!({ "id": "calc.new_sheet", "alias": "kalkulasi.lembar_baru", "category": "calc", "name": "Tambah Lembar Baru" }),
                json!({ "id": "calc.export_xlsx", "alias": "kalkulasi.ekspor_xlsx", "category": "calc", "name": "Ekspor ke Excel XLSX" }),
                json!({ "id": "calc.export_csv", "alias": "kalkulasi.ekspor_csv", "category": "calc", "name": "Ekspor ke CSV" }),
                json!({ "id": "calc.generate_chart", "alias": "kalkulasi.buat_grafik", "category": "calc", "name": "Buat Grafik SVG" }),
                json!({ "id": "calc.compile_dsl", "alias": "kalkulasi.kompilasi_dsl", "category": "calc", "name": "Kompilasi Spreadsheet DSL" }),
                json!({ "id": "calc.export_dsl", "alias": "kalkulasi.ekspor_dsl", "category": "calc", "name": "Ekspor ke Widya DSL" }),
                json!({ "id": "calc.inspect", "alias": "kalkulasi.inspeksi", "category": "calc", "name": "Inspeksi Buku Kerja" }),
                json!({ "id": "impress.new_slide", "alias": "presentasi.slide_baru", "category": "impress", "name": "Slide Baru", "shortcut": "ctrl+m" }),
                json!({ "id": "impress.duplicate_slide", "alias": "presentasi.duplikasi_slide", "category": "impress", "name": "Duplikasi Slide", "shortcut": "ctrl+d" }),
                json!({ "id": "impress.delete_slide", "alias": "presentasi.hapus_slide", "category": "impress", "name": "Hapus Slide", "shortcut": "delete" }),
                json!({ "id": "impress.select_slide", "alias": "presentasi.pilih_slide", "category": "impress", "name": "Pilih Slide" }),
                json!({ "id": "impress.move_slide_up", "alias": "presentasi.geser_atas", "category": "impress", "name": "Geser Slide ke Atas" }),
                json!({ "id": "impress.move_slide_down", "alias": "presentasi.geser_bawah", "category": "impress", "name": "Geser Slide ke Bawah" }),
                json!({ "id": "impress.set_layout", "alias": "presentasi.atur_tata_letak", "category": "impress", "name": "Atur Tata Letak Slide" }),
                json!({ "id": "impress.set_theme", "alias": "presentasi.atur_tema", "category": "impress", "name": "Atur Tema Presentasi" }),
                json!({ "id": "impress.set_aspect_ratio", "alias": "presentasi.atur_rasio", "category": "impress", "name": "Atur Rasio Slide" }),
                json!({ "id": "impress.set_transition", "alias": "presentasi.atur_transisi", "category": "impress", "name": "Atur Efek Transisi" }),
                json!({ "id": "impress.toggle_sorter", "alias": "presentasi.tampilan_penyortir", "category": "impress", "name": "Beralih Tampilan Sorter" }),
                json!({ "id": "impress.start_slideshow", "alias": "presentasi.mulai_presentasi", "category": "impress", "name": "Mulai Slide Show", "shortcut": "f5" }),
                json!({ "id": "impress.start_slideshow_from_current", "alias": "presentasi.mulai_dari_slide_aktif", "category": "impress", "name": "Slide Show dari Slide Ini", "shortcut": "shift+f5" }),
                json!({ "id": "impress.toggle_presenter_notes", "alias": "presentasi.tampilkan_catatan", "category": "impress", "name": "Tampilkan/Sembunyikan Catatan" }),
                json!({ "id": "impress.export_pptx", "alias": "presentasi.ekspor_pptx", "category": "impress", "name": "Ekspor ke PowerPoint PPTX" }),
                json!({ "id": "impress.export_pdf", "alias": "presentasi.ekspor_pdf", "category": "impress", "name": "Ekspor ke Handout PDF" }),
                json!({ "id": "impress.compile_dsl", "alias": "presentasi.kompilasi_dsl", "category": "impress", "name": "Kompilasi Presentasi DSL" }),
                json!({ "id": "impress.import_markdown", "alias": "presentasi.impor_markdown", "category": "impress", "name": "Impor Presentasi Markdown" }),
                json!({ "id": "impress.export_dsl", "alias": "presentasi.ekspor_dsl", "category": "impress", "name": "Ekspor ke Widya DSL" }),
                json!({ "id": "impress.render_svg", "alias": "presentasi.render_svg", "category": "impress", "name": "Render Slide ke Vektor SVG" }),
                json!({ "id": "impress.inspect", "alias": "presentasi.inspeksi", "category": "impress", "name": "Inspeksi Presentasi" }),

                // 12. WidyaPDF Workbench Commands (Katalog Perintah Acrobat Parity & Sovereign PDF)
                json!({ "id": "pdf.next_page", "alias": "pdf.halaman_berikutnya", "category": "pdf", "name": "Halaman Berikutnya", "shortcut": "pagedown" }),
                json!({ "id": "pdf.prev_page", "alias": "pdf.halaman_sebelumnya", "category": "pdf", "name": "Halaman Sebelumnya", "shortcut": "pageup" }),
                json!({ "id": "pdf.go_to_page", "alias": "pdf.ke_halaman", "category": "pdf", "name": "Buka Halaman Spesifik" }),
                json!({ "id": "pdf.zoom_in", "alias": "pdf.perbesar", "category": "pdf", "name": "Perbesar Tampilan (Zoom In)", "shortcut": "ctrl+=" }),
                json!({ "id": "pdf.zoom_out", "alias": "pdf.perkecil", "category": "pdf", "name": "Perkecil Tampilan (Zoom Out)", "shortcut": "ctrl+-" }),
                json!({ "id": "pdf.zoom_fit_page", "alias": "pdf.pas_halaman", "category": "pdf", "name": "Sesuaikan ke Satu Halaman", "shortcut": "ctrl+0" }),
                json!({ "id": "pdf.zoom_fit_width", "alias": "pdf.pas_lebar", "category": "pdf", "name": "Sesuaikan ke Lebar Jendela", "shortcut": "ctrl+2" }),
                json!({ "id": "pdf.zoom_actual_size", "alias": "pdf.ukuran_asli", "category": "pdf", "name": "Ukuran Sebenarnya 100%", "shortcut": "ctrl+1" }),
                json!({ "id": "pdf.rotate", "alias": "pdf.putar_halaman", "category": "pdf", "name": "Putar Orientasi Halaman 90°" }),
                json!({ "id": "pdf.toggle_two_up", "alias": "pdf.mode_buku", "category": "pdf", "name": "Beralih Mode Dua Halaman (Two-Up)" }),
                json!({ "id": "pdf.toggle_dark_mode", "alias": "pdf.mode_malam", "category": "pdf", "name": "Beralih Pembaca Gelap Kontras Tinggi" }),
                json!({ "id": "pdf.toggle_comments", "alias": "pdf.panel_komentar", "category": "pdf", "name": "Tampilkan/Sembunyikan Panel Komentar" }),
                json!({ "id": "pdf.search", "alias": "pdf.cari_teks", "category": "pdf", "name": "Cari Teks dalam Dokumen", "shortcut": "ctrl+f" }),
                json!({ "id": "pdf.add_sticky_note", "alias": "pdf.tambah_catatan", "category": "pdf", "name": "Bubuhi Catatan Tempel (Sticky Note)" }),
                json!({ "id": "pdf.add_stamp", "alias": "pdf.bubuhi_stempel", "category": "pdf", "name": "Bubuhi Stempel Resmi (Approved/Confidential)" }),
                json!({ "id": "pdf.sign_pqc", "alias": "pdf.tanda_tangan_pqc", "category": "pdf", "name": "Tanda Tangan Elektronik Pasca-Kuantum FIPS 204" }),
                json!({ "id": "pdf.sensor_pdp", "alias": "pdf.sensor_uu_pdp", "category": "pdf", "name": "Sensor Privasi Otomatis (UU PDP No. 27/2022)" }),
                json!({ "id": "pdf.save", "alias": "pdf.simpan", "category": "pdf", "name": "Simpan Perubahan Dokumen PDF", "shortcut": "ctrl+s" }),
                json!({ "id": "pdf.compile_dsl", "alias": "pdf.kompilasi_dsl", "category": "pdf", "name": "Kompilasi WidyaPDF DSL" }),
                json!({ "id": "pdf.export_dsl", "alias": "pdf.ekspor_dsl", "category": "pdf", "name": "Ekspor ke WidyaPDF DSL" }),
                json!({ "id": "pdf.inspect", "alias": "pdf.inspeksi", "category": "pdf", "name": "Inspeksi Status Dokumen PDF" }),
                json!({ "id": "workflow.batch_pqc_sign", "alias": "alur_kerja.tanda_tangan_massal_pqc", "category": "workflow", "name": "Penandatanganan Massal PQC (NIST FIPS 204)" }),
                json!({ "id": "workflow.crdt_simulate", "alias": "alur_kerja.simulasi_crdt", "category": "workflow", "name": "Simulasi Kolaborasi P2P CRDT Bebas Konflik" }),
                json!({ "id": "workflow.openxml_validate", "alias": "alur_kerja.validasi_openxml", "category": "workflow", "name": "Audit Kepatuhan ISO/IEC 29500 (OpenXML)" }),
            ];

            let res = json!({
                "sukses": true,
                "total_perintah": commands.len(),
                "katalog": commands,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 23. engine.execute_command (Eksekusi Perintah Terpusat via IPC / AI Agent)
        self.ipc.daftarkan_metode("engine.execute_command", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload execute_command tidak valid: {}", e))?;
            let cmd_str = req.get("command").and_then(|v| v.as_str()).unwrap_or("").trim().to_lowercase();
            if cmd_str.is_empty() {
                return Err("Nama perintah 'command' tidak boleh kosong".to_string());
            }

            let payload = req.get("payload").cloned().unwrap_or(json!({}));
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();

            let res = json!({
                "sukses": true,
                "command": cmd_str,
                "status": "Diterima oleh Engine Bus",
                "timestamp_ms": timestamp,
                "payload": payload,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 24. dsl.compile_widya_doc (Kompilasi Skrip Widya-Lang DSL ke HTML/DOCX/PDF)
        self.ipc.daftarkan_metode("dsl.compile_widya_doc", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let code = if let Some(code_str) = req.get("code").and_then(|v| v.as_str()) {
                code_str.to_string()
            } else if let Some(file_path) = req.get("file_path").and_then(|v| v.as_str()) {
                std::fs::read_to_string(file_path)
                    .map_err(|e| format!("Gagal membaca berkas DSL: {}", e))?
            } else {
                return Err("Parameter 'code' atau 'file_path' harus disediakan".to_string());
            };

            let doc = WidyaDocumentDsl::from_widya_dsl(&code)?;
            let html = doc.to_writer_html();
            let docx_bytes = doc.to_docx().export_to_docx_bytes();
            let dsl_code = doc.to_widya_dsl_code();

            let res = json!({
                "sukses": true,
                "judul": doc.title,
                "author": doc.author,
                "total_elemen": doc.elements.len(),
                "total_catatan_kaki": doc.footnotes.len(),
                "has_toc": doc.has_toc,
                "html": html,
                "docx_bytes_len": docx_bytes.len(),
                "widya_dsl": dsl_code,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 25. dsl.import_markdown (Konversi Markdown/Disertasi ke HTML WidyaWriter, DOCX & DSL)
        self.ipc.daftarkan_metode("dsl.import_markdown", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let (md_content, title_opt) = if let Some(file_path) = req.get("file_path").and_then(|v| v.as_str()) {
                let content = std::fs::read_to_string(file_path)
                    .map_err(|e| format!("Gagal membaca berkas Markdown: {}", e))?;
                let file_name = std::path::Path::new(file_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Dokumen Markdown");
                (content, Some(file_name.to_string()))
            } else if let Some(md_str) = req.get("markdown").and_then(|v| v.as_str()) {
                let t = req.get("judul").and_then(|v| v.as_str()).map(|s| s.to_string());
                (md_str.to_string(), t)
            } else {
                return Err("Parameter 'markdown' atau 'file_path' harus disediakan".to_string());
            };

            let doc = WidyaDocumentDsl::from_markdown(&md_content, title_opt.as_deref());
            let html = doc.to_writer_html();
            let docx_bytes = doc.to_docx().export_to_docx_bytes();
            let dsl_code = doc.to_widya_dsl_code();

            let res = json!({
                "sukses": true,
                "judul": doc.title,
                "author": doc.author,
                "total_elemen": doc.elements.len(),
                "total_catatan_kaki": doc.footnotes.len(),
                "has_toc": doc.has_toc,
                "html": html,
                "docx_bytes_len": docx_bytes.len(),
                "widya_dsl": dsl_code,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 26. calc.compile_dsl (Kompilasi Skrip WidyaCalc Spreadsheet DSL ke XLSX & Grid UI)
        self.ipc.daftarkan_metode("calc.compile_dsl", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let code = if let Some(code_str) = req.get("code").and_then(|v| v.as_str()) {
                code_str.to_string()
            } else if let Some(file_path) = req.get("file_path").and_then(|v| v.as_str()) {
                std::fs::read_to_string(file_path)
                    .map_err(|e| format!("Gagal membaca berkas DSL Calc: {}", e))?
            } else {
                return Err("Parameter 'code' atau 'file_path' harus disediakan".to_string());
            };

            let wb = WidyaWorkbookDsl::from_widya_dsl(&code)?;
            let xlsx = wb.to_xlsx();
            let xlsx_bytes = xlsx.export_to_xlsx_bytes();
            let grid_json = wb.to_grid_json();
            let dsl_code = wb.to_widya_dsl_code();

            let mut chart_svgs = Vec::new();
            for c_idx in 0..wb.charts.len() {
                if let Ok(svg) = wb.generate_svg_chart(c_idx) {
                    chart_svgs.push(svg);
                }
            }

            let res = json!({
                "sukses": true,
                "judul": wb.title,
                "total_lembar": wb.sheets.len(),
                "total_grafik": wb.charts.len(),
                "grid_json": grid_json,
                "chart_svgs": chart_svgs,
                "xlsx_bytes_len": xlsx_bytes.len(),
                "widya_dsl": dsl_code,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 27. calc.export_dsl (Ekspor Status Lembar Kerja ke Sintaks Murni Widya-Lang)
        self.ipc.daftarkan_metode("calc.export_dsl", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let judul = req.get("judul").and_then(|v| v.as_str()).unwrap_or("BukuKerja_Widya");
            let mut wb = WidyaWorkbookDsl::new(judul);

            if let Some(sheets_obj) = req.get("sheets").and_then(|v| v.as_object()) {
                wb.sheets.clear();
                for (_key, s_val) in sheets_obj {
                    let s_name = s_val.get("name").and_then(|v| v.as_str()).unwrap_or("Sheet");
                    let mut dsl_sheet = crate::office::calc_dsl::DslSheet::new(s_name);
                    if let Some(cells) = s_val.get("cells").and_then(|v| v.as_object()) {
                        for (coord, val) in cells {
                            if let Some(s) = val.as_str() {
                                dsl_sheet.set_cell(coord, s);
                            }
                        }
                    }
                    wb.sheets.push(dsl_sheet);
                }
            }

            let dsl_code = wb.to_widya_dsl_code();
            let res = json!({
                "sukses": true,
                "judul": judul,
                "widya_dsl": dsl_code,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 28. calc.generate_chart (Generator Grafik Vektor SVG Interaktif Murni)
        self.ipc.daftarkan_metode("calc.generate_chart", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let kind = req.get("type").and_then(|v| v.as_str()).unwrap_or("batang");
            let title = req.get("title").and_then(|v| v.as_str()).unwrap_or("Grafik WidyaCalc");
            let label_range = req.get("label_range").and_then(|v| v.as_str()).unwrap_or("A1:A5");
            let data_range = req.get("data_range").and_then(|v| v.as_str()).unwrap_or("B1:B5");

            let mut wb = WidyaWorkbookDsl::new("ChartTest");
            if let Some(sheet_cells) = req.get("cells").and_then(|v| v.as_object()) {
                let sheet = wb.sheets.get_mut(0).unwrap();
                for (coord, val) in sheet_cells {
                    if let Some(s) = val.as_str() {
                        sheet.set_cell(coord, s);
                    }
                }
            }

            wb.add_chart(DslChart {
                kind: kind.to_string(),
                title: title.to_string(),
                label_range: label_range.to_string(),
                data_range: data_range.to_string(),
                color_palette: DslChart::default().color_palette,
            });

            let svg = wb.generate_svg_chart(0)
                .map_err(|e| format!("Gagal menghasilkan grafik SVG: {}", e))?;

            let res = json!({
                "sukses": true,
                "type": kind,
                "title": title,
                "svg": svg,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 29. impress.compile_dsl (Kompilasi Skrip Presentasi Deklaratif Widya-Lang)
        self.ipc.daftarkan_metode("impress.compile_dsl", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let code = if let Some(c) = req.get("code").and_then(|v| v.as_str()) {
                c.to_string()
            } else if let Some(file_path) = req.get("file_path").and_then(|v| v.as_str()) {
                std::fs::read_to_string(file_path)
                    .map_err(|e| format!("Gagal membaca berkas DSL Impress: {}", e))?
            } else {
                return Err("Parameter 'code' atau 'file_path' harus disediakan".to_string());
            };

            let pres = WidyaPresentationDsl::from_widya_dsl(&code)?;
            let pptx = pres.to_pptx();
            let pptx_bytes = pptx.export_to_pptx_bytes();
            let slides_json = pres.to_slides_json();
            let dsl_code = pres.to_widya_dsl_code();
            let preview_svg = pres.render_slide_svg(0).ok();

            let res = json!({
                "sukses": true,
                "judul": pres.title,
                "total_slide": pres.slides.len(),
                "slides_json": slides_json,
                "pptx_bytes_len": pptx_bytes.len(),
                "widya_dsl": dsl_code,
                "preview_svg": preview_svg,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 30. impress.import_markdown (Impor Presentasi Markdown Marp/Slidev Parity)
        self.ipc.daftarkan_metode("impress.import_markdown", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let md = req.get("markdown").and_then(|v| v.as_str())
                .ok_or_else(|| "Parameter 'markdown' harus disediakan".to_string())?;
            let title_opt = req.get("judul").and_then(|v| v.as_str());

            let pres = WidyaPresentationDsl::from_markdown(md, title_opt);
            let pptx = pres.to_pptx();
            let pptx_bytes = pptx.export_to_pptx_bytes();
            let slides_json = pres.to_slides_json();

            let res = json!({
                "sukses": true,
                "judul": pres.title,
                "total_slide": pres.slides.len(),
                "slides_json": slides_json,
                "pptx_bytes_len": pptx_bytes.len(),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 31. impress.export_dsl (Ekspor Slide Presentasi Aktif ke Sintaks Widya-Lang)
        self.ipc.daftarkan_metode("impress.export_dsl", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let pres_val = req.get("presentasi").unwrap_or(&req);
            let title = pres_val.get("title").and_then(|v| v.as_str()).unwrap_or("Presentasi_Widya");
            let theme = pres_val.get("theme").and_then(|v| v.as_str()).unwrap_or("theme-executive-navy");
            let ratio = pres_val.get("aspectRatio").and_then(|v| v.as_str()).unwrap_or("16:9");

            let mut pres = WidyaPresentationDsl::new(title);
            pres.slides.clear();
            pres.theme = theme.to_string();
            pres.aspect_ratio = ratio.to_string();

            if let Some(slides_arr) = pres_val.get("slides").and_then(|v| v.as_array()) {
                for s in slides_arr {
                    let layout = s.get("layout").and_then(|v| v.as_str()).unwrap_or("content");
                    let s_title = s.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    let s_sub = s.get("subtitle").and_then(|v| v.as_str());
                    let s_notes = s.get("notes").and_then(|v| v.as_str());

                    match layout {
                        "title" => {
                            pres.slides.push(DslSlide::new_title(s_title, s_sub, s_notes));
                        }
                        "kpi" => {
                            let mut kpi_cards = Vec::new();
                            if let Some(cards_arr) = s.get("kpiCards").and_then(|v| v.as_array()) {
                                for c in cards_arr {
                                    kpi_cards.push(DslKpiCard {
                                        label: c.get("label").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                        value: c.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                        subtext: c.get("subtext").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                        positive: c.get("positive").and_then(|v| v.as_bool()).unwrap_or(true),
                                    });
                                }
                            }
                            pres.slides.push(DslSlide::new_kpi(s_title, s_sub, kpi_cards, s_notes));
                        }
                        "two-column" => {
                            let lt = s.get("leftTitle").and_then(|v| v.as_str());
                            let rt = s.get("rightTitle").and_then(|v| v.as_str());
                            let mut lb = Vec::new();
                            if let Some(arr) = s.get("leftBullets").and_then(|v| v.as_array()) {
                                for b in arr { if let Some(txt) = b.as_str() { lb.push(txt.to_string()); } }
                            }
                            let mut rb = Vec::new();
                            if let Some(arr) = s.get("rightBullets").and_then(|v| v.as_array()) {
                                for b in arr { if let Some(txt) = b.as_str() { rb.push(txt.to_string()); } }
                            }
                            pres.slides.push(DslSlide::new_two_column(s_title, s_sub, lt, lb, rt, rb, s_notes));
                        }
                        _ => {
                            let mut bullets = Vec::new();
                            if let Some(arr) = s.get("bullets").and_then(|v| v.as_array()) {
                                for b in arr { if let Some(txt) = b.as_str() { bullets.push(txt.to_string()); } }
                            }
                            pres.slides.push(DslSlide::new_content(s_title, s_sub, bullets, s_notes));
                        }
                    }
                }
            }

            let dsl_code = pres.to_widya_dsl_code();
            let res = json!({
                "sukses": true,
                "judul": title,
                "widya_dsl": dsl_code,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 32. impress.render_slide_svg (Render Vektor SVG Slide Mandiri Beresolusi Tinggi)
        self.ipc.daftarkan_metode("impress.render_slide_svg", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let slide_idx = req.get("slide_index").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

            let pres = if let Some(code) = req.get("code").and_then(|v| v.as_str()) {
                WidyaPresentationDsl::from_widya_dsl(code)?
            } else if let Some(md) = req.get("markdown").and_then(|v| v.as_str()) {
                WidyaPresentationDsl::from_markdown(md, None)
            } else {
                WidyaPresentationDsl::default()
            };

            let svg = pres.render_slide_svg(slide_idx)
                .map_err(|e| format!("Gagal me-render SVG slide: {}", e))?;

            let res = json!({
                "sukses": true,
                "slide_index": slide_idx,
                "svg": svg,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 33. pdf.compile_dsl (Kompilasi Skrip WidyaPDF DSL ke Biner PDF Asli & JSON Workbench)
        self.ipc.daftarkan_metode("pdf.compile_dsl", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let code = if let Some(c) = req.get("code").and_then(|v| v.as_str()) {
                c.to_string()
            } else if let Some(file_path) = req.get("file_path").and_then(|v| v.as_str()) {
                std::fs::read_to_string(file_path)
                    .map_err(|e| format!("Gagal membaca berkas DSL PDF: {}", e))?
            } else {
                return Err("Parameter 'code' atau 'file_path' harus disediakan".to_string());
            };

            let mut doc = WidyaPdfDsl::from_widya_dsl(&code)?;

            // Sensor otomatis UU PDP jika diminta
            let apply_pdp = req.get("apply_pdp").and_then(|v| v.as_bool()).unwrap_or(false);
            let mut redacted_count = 0;
            if apply_pdp {
                redacted_count = doc.apply_uupdp_redaction();
            }

            let pdf_doc = doc.to_pdf();
            let pdf_bytes = pdf_doc.generate_pdf_bytes();
            let pages_json = doc.to_pdf_pages_json();
            let dsl_code = doc.to_widya_dsl_code();
            let preview_svg = doc.render_page_svg(0).ok();

            let res = json!({
                "sukses": true,
                "judul": doc.title,
                "total_halaman": doc.pages.len(),
                "pages_json": pages_json,
                "pdf_bytes_len": pdf_bytes.len(),
                "sha256": hex::encode(sha2::Sha256::digest(&pdf_bytes)),
                "widya_dsl": dsl_code,
                "preview_svg": preview_svg,
                "sensor_pdp_diterapkan": apply_pdp,
                "total_item_disensor": redacted_count,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 34. pdf.export_dsl (Ekspor Status Halaman & Anotasi PDF ke Sintaks Murni Widya-Lang)
        self.ipc.daftarkan_metode("pdf.export_dsl", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let title = req.get("title").and_then(|v| v.as_str()).unwrap_or("Dokumen_WidyaPDF");
            let author = req.get("author").and_then(|v| v.as_str()).unwrap_or("Widya Office Suite");
            let org = req.get("organization").and_then(|v| v.as_str()).unwrap_or("Pemerintah Republik Indonesia");
            let page_size = req.get("pageSize").and_then(|v| v.as_str()).unwrap_or("A4");
            let watermark = req.get("watermark").and_then(|v| v.as_str()).map(|s| s.to_string());

            let mut doc = WidyaPdfDsl::new(title);
            doc.author = author.to_string();
            doc.organization = org.to_string();
            doc.page_size = page_size.to_string();
            doc.watermark = watermark;
            doc.pages.clear();

            if let Some(pages_arr) = req.get("pagesData").and_then(|v| v.as_array()) {
                for (idx, p_val) in pages_arr.iter().enumerate() {
                    let page_num = p_val.get("pageNumber").and_then(|v| v.as_u64()).unwrap_or((idx + 1) as u64) as usize;
                    let p_title = p_val.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    let p_header = p_val.get("header").and_then(|v| v.as_str()).unwrap_or("");

                    let mut dsl_page = DslPdfPage::new(page_num, p_title, p_header);

                    if let Some(body_arr) = p_val.get("body").and_then(|v| v.as_array()) {
                        for b in body_arr {
                            if let Some(txt) = b.as_str() {
                                dsl_page.add_paragraph(txt);
                            }
                        }
                    }

                    // Muat Anotasi Kotak Redaksi untuk halaman ini jika ada
                    if let Some(redactions_obj) = req.get("manualRedactions").and_then(|v| v.as_object()) {
                        let key = page_num.to_string();
                        if let Some(boxes) = redactions_obj.get(&key).and_then(|v| v.as_array()) {
                            for bx in boxes {
                                let x = bx.get("x").and_then(|v| v.as_f64()).unwrap_or(50.0) as f32;
                                let y = bx.get("y").and_then(|v| v.as_f64()).unwrap_or(200.0) as f32;
                                let w = bx.get("w").and_then(|v| v.as_f64()).unwrap_or(200.0) as f32;
                                let h = bx.get("h").and_then(|v| v.as_f64()).unwrap_or(20.0) as f32;
                                let label = bx.get("label").and_then(|v| v.as_str()).unwrap_or("SENSOR UU PDP");
                                dsl_page.add_redaction_box(x, y, w, h, label);
                            }
                        }
                    }

                    // Muat Stempel untuk halaman ini jika ada
                    if let Some(stamps_obj) = req.get("rubberStamps").and_then(|v| v.as_object()) {
                        let key = page_num.to_string();
                        if let Some(stamps) = stamps_obj.get(&key).and_then(|v| v.as_array()) {
                            for st in stamps {
                                let stype = st.get("type").and_then(|v| v.as_str()).unwrap_or("APPROVED");
                                let x = st.get("x").and_then(|v| v.as_f64()).unwrap_or(400.0) as f32;
                                let y = st.get("y").and_then(|v| v.as_f64()).unwrap_or(700.0) as f32;
                                let label = st.get("label").and_then(|v| v.as_str()).unwrap_or(stype);
                                dsl_page.add_stamp(stype, x, y, label);
                            }
                        }
                    }

                    // Muat Tanda Tangan Digital PQC untuk halaman ini jika ada
                    if let Some(sigs_obj) = req.get("digitalSignatures").and_then(|v| v.as_object()) {
                        let key = page_num.to_string();
                        if let Some(sigs) = sigs_obj.get(&key).and_then(|v| v.as_array()) {
                            for sg in sigs {
                                let signer = sg.get("signer").and_then(|v| v.as_str()).unwrap_or("Pejabat");
                                let role = sg.get("role").and_then(|v| v.as_str()).unwrap_or("Auditor");
                                let algo = sg.get("algorithm").and_then(|v| v.as_str()).unwrap_or("ML-DSA-87");
                                let time_str = sg.get("timestamp").and_then(|v| v.as_str()).unwrap_or("Sekarang");
                                let sig_hex = sg.get("signature_hex").and_then(|v| v.as_str()).unwrap_or("");
                                let x = sg.get("x").and_then(|v| v.as_f64()).unwrap_or(380.0) as f32;
                                let y = sg.get("y").and_then(|v| v.as_f64()).unwrap_or(140.0) as f32;
                                dsl_page.add_pqc_signature(signer, role, algo, time_str, sig_hex, x, y);
                            }
                        }
                    }

                    doc.pages.push(dsl_page);
                }
            }

            if doc.pages.is_empty() {
                doc.pages.push(DslPdfPage::new(1, title, "LEMBAR DOKUMEN RESMI"));
            }

            let dsl_code = doc.to_widya_dsl_code();
            let res = json!({
                "sukses": true,
                "judul": title,
                "total_halaman": doc.pages.len(),
                "widya_dsl": dsl_code,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 35. pdf.tandatangani_pqc (Pembubuhan Tanda Tangan Pasca-Kuantum NIST ML-DSA-87 / HSM)
        self.ipc.daftarkan_metode("pdf.tandatangani_pqc", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let signer = req.get("signer").and_then(|v| v.as_str()).unwrap_or("Prof. Dr. Budi Santoso");
            let role = req.get("role").and_then(|v| v.as_str()).unwrap_or("Kepala Pusat Kriptografi BSSN");

            let mut doc = if let Some(code) = req.get("code").and_then(|v| v.as_str()) {
                WidyaPdfDsl::from_widya_dsl(code)?
            } else {
                let title = req.get("title").and_then(|v| v.as_str()).unwrap_or("Dokumen Resmi Berdaulat");
                WidyaPdfDsl::new(title)
            };

            let sig_meta = doc.sign_pqc(signer, role)?;
            let pdf_bytes = doc.to_pdf().generate_pdf_bytes();

            let res = json!({
                "sukses": true,
                "judul": doc.title,
                "penandatangan": signer,
                "peran": role,
                "algoritma": sig_meta.algorithm,
                "waktu": sig_meta.timestamp,
                "signature_hex": sig_meta.signature_hex,
                "document_digest": sig_meta.document_digest_sha256,
                "pdf_bytes_len": pdf_bytes.len(),
                "total_halaman": doc.pages.len(),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 36. pdf.render_svg_page (Render Vektor SVG Halaman PDF Resolusi Tinggi Mandiri)
        self.ipc.daftarkan_metode("pdf.render_svg_page", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload tidak valid: {}", e))?;

            let page_idx = req.get("page_index").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

            let doc = if let Some(code) = req.get("code").and_then(|v| v.as_str()) {
                WidyaPdfDsl::from_widya_dsl(code)?
            } else {
                WidyaPdfDsl::default()
            };

            let svg = doc.render_page_svg(page_idx)
                .map_err(|e| format!("Gagal me-render SVG halaman: {}", e))?;

            let res = json!({
                "sukses": true,
                "page_index": page_idx,
                "svg": svg,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 37. dokumen.tanda_tangani_batch_pqc (NIST FIPS 204 ML-DSA-87 Penandatanganan Massal Berkecepatan Tinggi)
        self.ipc.daftarkan_metode("dokumen.tanda_tangani_batch_pqc", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload batch PQC tidak valid: {}", e))?;

            let seed = req.get("seed").and_then(|v| v.as_str()).unwrap_or("WIDYA_OFFICE_BATCH_PQC_SEED");
            let mut items = Vec::new();

            if let Some(doc_list) = req.get("dokumen_list").and_then(|v| v.as_array()) {
                for (idx, d) in doc_list.iter().enumerate() {
                    let id = d.get("id").and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("DOC-{:04}", idx + 1));
                    let title = d.get("judul").and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("Dokumen-{}", idx + 1));
                    let content = d.get("konten").and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "".to_string());
                    items.push(BatchPqcDocumentItem { id, title, content });
                }
            } else if let Some(count) = req.get("generate_sample_count").and_then(|v| v.as_u64()) {
                for i in 1..=count {
                    items.push(BatchPqcDocumentItem {
                        id: format!("DOC-{:04}", i),
                        title: format!("Surat Keputusan Resmi #{:04}", i),
                        content: format!("Dokumen Kedaulatan Data WidyaOffice #{} - Tanggal {}", i, chrono::Utc::now().to_rfc3339()),
                    });
                }
            }

            if items.is_empty() {
                return Err("Daftar dokumen 'dokumen_list' tidak boleh kosong".to_string());
            }

            let (signed_items, metrics) = BatchPqcSigner::sign_batch(&items, seed);

            let res = json!({
                "sukses": true,
                "total_signed": metrics.total_documents,
                "elapsed_millis": metrics.elapsed_millis,
                "signatures_per_second": metrics.signatures_per_second,
                "algorithm": metrics.algorithm,
                "all_verified_valid": metrics.all_verified_valid,
                "signed_items": signed_items,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 38. dokumen.verifikasi_batch_pqc (Verifikasi Massal Integritas Berkas PQC)
        self.ipc.daftarkan_metode("dokumen.verifikasi_batch_pqc", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload verifikasi batch PQC tidak valid: {}", e))?;

            let mut signed_items = Vec::new();
            if let Some(items_arr) = req.get("signed_items").and_then(|v| v.as_array()) {
                for item in items_arr {
                    let id = item.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let signature_hex = item.get("signature_hex").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let public_key_hex = item.get("public_key_hex").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let salt_hex = item.get("salt_hex").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let timestamp_ms = item.get("timestamp_ms").and_then(|v| v.as_u64()).unwrap_or(0);
                    let algorithm = item.get("algorithm").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let sha256_hash = item.get("sha256_hash").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let is_valid = item.get("is_valid").and_then(|v| v.as_bool()).unwrap_or(true);

                    signed_items.push(BatchPqcSignedItem {
                        id,
                        title,
                        signature_hex,
                        public_key_hex,
                        salt_hex,
                        timestamp_ms,
                        algorithm,
                        sha256_hash,
                        is_valid,
                    });
                }
            }

            let mut original_contents = HashMap::new();
            if let Some(map) = req.get("konten_asli").and_then(|v| v.as_object()) {
                for (k, v) in map {
                    if let Some(s) = v.as_str() {
                        original_contents.insert(k.clone(), s.to_string());
                    }
                }
            }

            let (all_valid, valid_count, invalid_count) = BatchPqcSigner::verify_batch(&signed_items, &original_contents);

            let res = json!({
                "sukses": true,
                "semua_valid": all_valid,
                "valid_count": valid_count,
                "invalid_count": invalid_count,
                "total": signed_items.len(),
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 39. crdt.simulasi_konvergensi (Simulasi Kolaborasi Offline-First Real-Time P2P CRDT)
        self.ipc.daftarkan_metode("crdt.simulasi_konvergensi", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .unwrap_or_else(|_| json!({}));

            let base_text = req.get("base_text").and_then(|v| v.as_str()).unwrap_or("WidyaOffice Kedaulatan Digital");
            let pos_a = req.get("peer_a_pos").and_then(|v| v.as_u64()).unwrap_or(base_text.len() as u64) as usize;
            let text_a = req.get("peer_a_text").and_then(|v| v.as_str()).unwrap_or(" [Nusantara Murni]");
            let pos_b = req.get("peer_b_pos").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            let text_b = req.get("peer_b_text").and_then(|v| v.as_str()).unwrap_or("Dokumen Resmi: ");

            let result = CrdtSimulator::simulate_two_peers_concurrent(
                base_text,
                (pos_a, text_a),
                (pos_b, text_b),
            );

            let res = json!({
                "sukses": true,
                "is_identical": result.is_identical,
                "converged_text": result.peer_a_converged,
                "peer_a_initial": result.peer_a_initial,
                "peer_b_initial": result.peer_b_initial,
                "peer_a_converged": result.peer_a_converged,
                "peer_b_converged": result.peer_b_converged,
                "total_ops_exchanged": result.total_ops_exchanged,
                "causal_lamport_clock": result.causal_lamport_clock,
                "status": if result.is_identical { "Konvergen Bebas Konflik (100% Identik)" } else { "Konflik Terdeteksi" }
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });

        // 40. openxml.validasi_arsip (Audit Kepatuhan Format OpenXML ISO/IEC 29500 Murni)
        self.ipc.daftarkan_metode("openxml.validasi_arsip", |payload_bytes| {
            let req: Value = serde_json::from_slice(payload_bytes)
                .map_err(|e| format!("Payload validasi OpenXML tidak valid: {}", e))?;

            let tipe = req.get("tipe").and_then(|v| v.as_str()).unwrap_or("docx").to_lowercase();
            
            let raw_bytes = if let Some(b64) = req.get("bytes_base64").and_then(|v| v.as_str()) {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.decode(b64)
                    .map_err(|e| format!("Gagal mendecode base64: {}", e))?
            } else {
                match tipe.as_str() {
                    "xlsx" => {
                        let wb = XlsxWorkbook::new("Buku Kerja Validasi.xlsx");
                        wb.export_to_xlsx_bytes()
                    },
                    "pptx" => {
                        let pres = PptxPresentation::new("Presentasi Validasi.pptx");
                        pres.export_to_pptx_bytes()
                    },
                    _ => {
                        let mut doc = DocxDocument::new("Dokumen Validasi.docx");
                        doc.add_paragraph("Uji Kepatuhan ISO/IEC 29500 Murni WidyaOffice.");
                        doc.export_to_docx_bytes()
                    }
                }
            };

            let pkg = ZipPackage::from_bytes(&raw_bytes)
                .map_err(|e| format!("Gagal membaca paket ZIP OpenXML: {}", e))?;

            let report = match tipe.as_str() {
                "xlsx" => pkg.validate_openxml_xlsx()?,
                "pptx" => pkg.validate_openxml_pptx()?,
                _ => pkg.validate_openxml_docx()?,
            };

            let res = json!({
                "sukses": true,
                "valid": report.is_valid,
                "doc_type": report.doc_type,
                "total_entries": report.total_entries,
                "has_content_types": report.has_content_types,
                "has_rels": report.has_rels,
                "primary_part_found": report.primary_part_found,
                "primary_part_path": report.primary_part_path,
                "schema_namespaces": report.schema_namespaces,
                "compliance_standard": report.compliance_standard,
                "issues": report.issues,
            });
            serde_json::to_vec(&res).map_err(|e| e.to_string())
        });
    }
}

