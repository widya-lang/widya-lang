use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_office_suite(env: &mut Environment) {
    // Office Suite (DOCX, XLSX, PPTX, PDF & Universal Converter)
    register_fn(env, "office_buat_docx", Some(1), builtin_office_buat_docx);
    register_fn(env, "office_docx_tambah_paragraf", Some(5), builtin_office_docx_tambah_paragraf);
    register_fn(env, "office_docx_ekstrak_teks", Some(1), builtin_office_docx_ekstrak_teks);
    register_fn(env, "office_docx_simpan", Some(2), builtin_office_docx_simpan);
    register_fn(env, "office_docx_terapkan_template", Some(2), builtin_office_docx_terapkan_template);
    register_fn(env, "office_docx_tanda_tangan", Some(4), builtin_office_docx_tanda_tangan);
    register_fn(env, "office_buat_xlsx", Some(1), builtin_office_buat_xlsx);
    register_fn(env, "office_xlsx_simpan", Some(2), builtin_office_xlsx_simpan);
    register_fn(env, "office_xlsx_evaluasi_rumus", Some(2), builtin_office_xlsx_evaluasi_rumus);
    register_fn(env, "office_buat_pptx", Some(1), builtin_office_buat_pptx);
    register_fn(env, "office_pptx_simpan", Some(2), builtin_office_pptx_simpan);
    register_fn(env, "office_pptx_render_slideshow", Some(1), builtin_office_pptx_render_slideshow);
    register_fn(env, "office_docx_ke_pdf", Some(2), builtin_office_docx_ke_pdf);
    register_fn(env, "office_xlsx_ke_pdf", Some(2), builtin_office_xlsx_ke_pdf);
    register_fn(env, "office_markdown_ke_docx", Some(2), builtin_office_markdown_ke_docx);
    register_fn(env, "office_markdown_ke_pdf", Some(3), builtin_office_markdown_ke_pdf);
    register_fn(env, "office_pptx_ke_pdf", Some(2), builtin_office_pptx_ke_pdf);
    register_fn(env, "office_buat_epub", Some(4), builtin_office_buat_epub);
    register_fn(env, "office_buat_qrcode_svg", Some(1), builtin_office_buat_qrcode_svg);
    register_fn(env, "office_buat_qrcode_pdf", Some(2), builtin_office_buat_qrcode_pdf);
    register_fn(env, "office_pdf_ekstrak_teks", Some(1), builtin_office_pdf_ekstrak_teks);
    register_fn(env, "office_sensor_pdp", Some(1), builtin_office_sensor_pdp);
    // PDF, LaTeX, Markdown & WidyaDrive / Workspace
    register_fn(env, "office_pdf_buat", Some(1), builtin_office_pdf_buat);
    register_fn(env, "office_pdf_simpan", Some(2), builtin_office_pdf_simpan);
    register_fn(env, "office_pdf_tambah_teks", Some(5), builtin_office_pdf_tambah_teks);
    register_fn(env, "office_latex_buat", Some(2), builtin_office_latex_buat);
    register_fn(env, "office_latex_tambah_persamaan", Some(2), builtin_office_latex_tambah_persamaan);
    register_fn(env, "office_markdown_ke_html", Some(1), builtin_office_markdown_ke_html);
    register_fn(env, "widyadrive_buat", Some(1), builtin_widyadrive_buat);
    register_fn(env, "widyadrive_unggah", Some(3), builtin_widyadrive_unggah);
    register_fn(env, "widyaworkspace_buat", Some(2), builtin_widyaworkspace_buat);
    register_fn(env, "widyaworkspace_render_dashboard", Some(1), builtin_widyaworkspace_render_dashboard);
    // CRDT Real-time Collaboration Engine
    register_fn(env, "crdt_teks_buat", Some(1), builtin_crdt_teks_buat);
    register_fn(env, "crdt_teks_sisip", Some(3), builtin_crdt_teks_sisip);
    register_fn(env, "crdt_teks_hapus", Some(3), builtin_crdt_teks_hapus);
    register_fn(env, "crdt_teks_baca", Some(1), builtin_crdt_teks_baca);
    register_fn(env, "crdt_sesi_buat", Some(3), builtin_crdt_sesi_buat);
    register_fn(env, "crdt_sesi_render_html", Some(1), builtin_crdt_sesi_render_html);
}

// --- Office Suite (DOCX, XLSX, PPTX) Builtin Implementations ---

fn builtin_office_buat_docx(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let title = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "Dokumen Baru",
    };
    let doc = crate::office::DocxDocument::new(title);
    let json_val = serde_json::to_value(&doc).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_docx_tambah_paragraf(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut doc: crate::office::DocxDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::DocxDocument::new("Dokumen"))
        }
    };
    let text = match &args[1] { Value::String(s) => s.as_str(), _ => "" };
    let bold = match &args[2] { Value::Bool(b) => *b, _ => false };
    let italic = match &args[3] { Value::Bool(b) => *b, _ => false };
    let font_size = match &args[4] { Value::Number(n) => *n as f32, _ => 11.0 };

    doc.add_styled_paragraph(text, bold, italic, font_size, None);
    let json_val = serde_json::to_value(&doc).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_docx_ekstrak_teks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let doc: crate::office::DocxDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::DocxDocument::new("Dokumen"))
        }
    };
    Ok(Value::String(doc.extract_plain_text()))
}

fn builtin_office_docx_simpan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let doc: crate::office::DocxDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::DocxDocument::new("Dokumen"))
        }
    };
    let path = match &args[1] { Value::String(s) => s.as_str(), _ => "dokumen.docx" };
    match doc.save_as_docx(path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal menyimpan DOCX: {}", e), span)),
    }
}

fn builtin_office_docx_terapkan_template(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut doc: crate::office::DocxDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::DocxDocument::new("Dokumen"))
        }
    };
    let json_vars = widya_val_ke_json(&args[1]);
    let mut vars = std::collections::HashMap::new();
    if let serde_json::Value::Object(map) = json_vars {
        for (k, v) in map {
            vars.insert(k, v.as_str().unwrap_or(&v.to_string()).to_string());
        }
    }
    doc.apply_template(&vars);
    let json_val = serde_json::to_value(&doc).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_docx_tanda_tangan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut doc: crate::office::DocxDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::DocxDocument::new("Dokumen"))
        }
    };
    let nama = match &args[1] { Value::String(s) => s.as_str(), _ => "Penandatangan" };
    let peran = match &args[2] { Value::String(s) => s.as_str(), _ => "Pejabat Berwenang" };
    let kunci = match &args[3] { Value::String(s) => s.as_str(), _ => "widya_secret" };

    let sig = doc.sign_document(nama, peran, kunci);
    let json_val = serde_json::to_value(&sig).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_buat_xlsx(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let title = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "BukuKerja",
    };
    let wb = crate::office::XlsxWorkbook::new(title);
    let json_val = serde_json::to_value(&wb).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_xlsx_simpan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wb: crate::office::XlsxWorkbook = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::XlsxWorkbook::new("Workbook"))
        }
    };
    let path = match &args[1] { Value::String(s) => s.as_str(), _ => "laporan.xlsx" };
    match wb.save_as_xlsx(path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal menyimpan XLSX: {}", e), span)),
    }
}

fn builtin_office_xlsx_evaluasi_rumus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let csv_data = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    let formula = match &args[1] { Value::String(s) => s.as_str(), _ => "=SUM(A1:A10)" };

    let mut sheet = crate::office::Worksheet::new("Sheet1");
    sheet.from_csv(csv_data);

    match sheet.evaluate_formula(formula) {
        Ok(crate::office::CellValue::Number(n)) => Ok(Value::Number(n)),
        Ok(crate::office::CellValue::Text(s)) => Ok(Value::String(s)),
        Ok(crate::office::CellValue::Bool(b)) => Ok(Value::Bool(b)),
        _ => Ok(Value::Number(0.0)),
    }
}

fn builtin_office_buat_pptx(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let title = match &args[0] { Value::String(s) => s.as_str(), _ => "Presentasi Baru" };
    let mut pptx = crate::office::PptxPresentation::new(title);
    pptx.add_slide(crate::office::PptxSlide::title_slide(title, "Dibuat dengan Widya Office"));
    let json_val = serde_json::to_value(&pptx).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_pptx_simpan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pptx: crate::office::PptxPresentation = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::PptxPresentation::new("Presentasi"))
        }
    };
    let path = match &args[1] { Value::String(s) => s.as_str(), _ => "presentasi.pptx" };
    match pptx.save_as_pptx(path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal menyimpan PPTX: {}", e), span)),
    }
}

fn builtin_office_pptx_render_slideshow(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pptx: crate::office::PptxPresentation = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::PptxPresentation::new("Presentasi"))
        }
    };
    Ok(Value::String(pptx.render_html_slideshow()))
}

fn builtin_office_docx_ke_pdf(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let doc: crate::office::DocxDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::DocxDocument::new("Dokumen"))
        }
    };
    let path = match &args[1] { Value::String(s) => s.as_str(), _ => "dokumen.pdf" };
    let pdf = crate::office::DocumentConverter::docx_to_pdf(&doc);
    match pdf.save_to_file(path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal konversi DOCX ke PDF: {}", e), span)),
    }
}

fn builtin_office_xlsx_ke_pdf(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wb: crate::office::XlsxWorkbook = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::XlsxWorkbook::new("Workbook"))
        }
    };
    let path = match &args[1] { Value::String(s) => s.as_str(), _ => "laporan.pdf" };
    let pdf = crate::office::DocumentConverter::xlsx_to_pdf(&wb);
    match pdf.save_to_file(path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal konversi XLSX ke PDF: {}", e), span)),
    }
}

fn builtin_office_markdown_ke_docx(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let md_text = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    let path = match &args[1] { Value::String(s) => s.as_str(), _ => "output.docx" };
    let md = crate::office::MarkdownDocument::parse(md_text);
    let doc = crate::office::DocumentConverter::markdown_to_docx(&md);
    match doc.save_as_docx(path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal konversi Markdown ke DOCX: {}", e), span)),
    }
}

fn builtin_office_sensor_pdp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    let redacted = crate::office::OfficeSecurity::redact_pdp_sensitive_text(text);
    Ok(Value::String(redacted))
}

fn builtin_office_pdf_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let title = match &args[0] { Value::String(s) => s.as_str(), _ => "Dokumen PDF" };
    let pdf = crate::office::PdfDocument::new(title);
    let json_val = serde_json::to_value(&pdf).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_pdf_simpan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pdf: crate::office::PdfDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::PdfDocument::new("Dokumen"))
        }
    };
    let path = match &args[1] { Value::String(s) => s.as_str(), _ => "dokumen.pdf" };
    match pdf.save_to_file(path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal menyimpan PDF: {}", e), span)),
    }
}

fn builtin_office_pdf_tambah_teks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut pdf: crate::office::PdfDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::PdfDocument::new("Dokumen"))
        }
    };
    let text = match &args[1] { Value::String(s) => s.as_str(), _ => "" };
    let x = match &args[2] { Value::Number(n) => *n as f32, _ => 50.0 };
    let y = match &args[3] { Value::Number(n) => *n as f32, _ => 750.0 };
    let size = match &args[4] { Value::Number(n) => *n as f32, _ => 12.0 };

    pdf.current_page_mut().add_text(text, x, y, size, false);
    let json_val = serde_json::to_value(&pdf).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_latex_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let title = match &args[0] { Value::String(s) => s.as_str(), _ => "Makalah Riset" };
    let author = match &args[1] { Value::String(s) => s.as_str(), _ => "Peneliti Widya" };
    let tex = crate::office::LatexDocument::new(title, author);
    let json_val = serde_json::to_value(&tex).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_latex_tambah_persamaan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut tex: crate::office::LatexDocument = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::LatexDocument::new("Doc", "Author"))
        }
    };
    let eq = match &args[1] { Value::String(s) => s.as_str(), _ => "E = mc^2" };
    tex.add_equation(eq);
    let json_val = serde_json::to_value(&tex).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_office_markdown_ke_html(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let md_text = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    let doc = crate::office::MarkdownDocument::parse(md_text);
    Ok(Value::String(doc.to_html()))
}

fn builtin_widyadrive_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let name = match &args[0] { Value::String(s) => s.as_str(), _ => "DriveUtama" };
    let drive = crate::office::WidyaDrive::new(name);
    let json_val = serde_json::to_value(&drive).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_widyadrive_unggah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut drive: crate::office::WidyaDrive = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::WidyaDrive::new("Drive"))
        }
    };
    let filename = match &args[1] { Value::String(s) => s.as_str(), _ => "berkas.txt" };
    let content = match &args[2] { Value::String(s) => s.as_bytes(), _ => b"" };

    let file_id = drive.upload_file(filename, content, "admin").unwrap_or_default();
    let mut map = HashMap::new();
    map.insert("file_id".to_string(), Value::String(file_id));
    map.insert("total_berkas".to_string(), Value::Number(drive.files.len() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_widyaworkspace_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ws_name = match &args[0] { Value::String(s) => s.as_str(), _ => "WidyaWorkspace" };
    let org = match &args[1] { Value::String(s) => s.as_str(), _ => "Widya Enterprise" };
    let ws = crate::office::WidyaWorkspace::new(ws_name, org);
    let json_val = serde_json::to_value(&ws).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_widyaworkspace_render_dashboard(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ws: crate::office::WidyaWorkspace = match &args[0] {
        v => {
            let json_v = widya_val_ke_json(v);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::WidyaWorkspace::new("WS", "Org"))
        }
    };
    Ok(Value::String(ws.render_workspace_dashboard()))
}

// ============================================================================
// CRDT BUILTIN FUNCTIONS
// ============================================================================

fn builtin_crdt_teks_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let client_id = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "klien-1",
    };
    let crdt = crate::office::TextCrdt::new(client_id);
    let json_val = serde_json::to_value(&crdt).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_crdt_teks_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let json_v = widya_val_ke_json(&args[0]);
    let mut crdt: crate::office::TextCrdt = serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::TextCrdt::new("klien"));
    let pos = match &args[1] { Value::Number(n) => *n as usize, _ => 0 };
    let text = match &args[2] { Value::String(s) => s.as_str(), _ => "" };

    crdt.insert_text(pos, text);
    let json_res = serde_json::to_value(&crdt).unwrap_or_default();
    Ok(json_ke_widya_val(json_res))
}

fn builtin_crdt_teks_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let json_v = widya_val_ke_json(&args[0]);
    let mut crdt: crate::office::TextCrdt = serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::TextCrdt::new("klien"));
    let pos = match &args[1] { Value::Number(n) => *n as usize, _ => 0 };
    let len = match &args[2] { Value::Number(n) => *n as usize, _ => 1 };

    crdt.delete_range(pos, len);
    let json_res = serde_json::to_value(&crdt).unwrap_or_default();
    Ok(json_ke_widya_val(json_res))
}

fn builtin_crdt_teks_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let json_v = widya_val_ke_json(&args[0]);
    let crdt: crate::office::TextCrdt = serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::TextCrdt::new("klien"));
    Ok(Value::String(crdt.read_text()))
}

fn builtin_crdt_sesi_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let room_id = match &args[0] { Value::String(s) => s.as_str(), _ => "room-1" };
    let doc_name = match &args[1] { Value::String(s) => s.as_str(), _ => "Dokumen" };
    let host_id = match &args[2] { Value::String(s) => s.as_str(), _ => "host" };

    let session = crate::office::CollabSession::new(room_id, doc_name, host_id, host_id);
    let json_val = serde_json::to_value(&session).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_crdt_sesi_render_html(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let json_v = widya_val_ke_json(&args[0]);
    let session: crate::office::CollabSession = serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::CollabSession::new("room", "doc", "host", "host"));
    Ok(Value::String(session.render_html_editor()))
}

fn builtin_office_markdown_ke_pdf(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let md_text = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    let out_path = match &args[1] { Value::String(s) => s.as_str(), _ => "output.pdf" };
    let sertakan_toc = match &args[2] { Value::Bool(b) => *b, _ => false };

    let md = crate::office::MarkdownDocument::parse(md_text);
    let pdf = if sertakan_toc {
        crate::office::DocumentConverter::markdown_to_pdf_with_toc(&md)
    } else {
        crate::office::DocumentConverter::markdown_to_pdf(&md)
    };

    match pdf.save_to_file(out_path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal konversi Markdown ke PDF: {}", e), span)),
    }
}

fn builtin_office_pptx_ke_pdf(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let in_val = &args[0];
    let out_path = match &args[1] { Value::String(s) => s.as_str(), _ => "presentation.pdf" };

    let pres = match in_val {
        Value::String(path) => crate::office::PptxPresentation::new(path),
        _ => {
            let json_v = widya_val_ke_json(in_val);
            serde_json::from_value(json_v).unwrap_or_else(|_| crate::office::PptxPresentation::new("Presentasi"))
        }
    };

    let pdf = crate::office::DocumentConverter::pptx_to_pdf(&pres);
    match pdf.save_to_file(out_path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal konversi PPTX ke PDF: {}", e), span)),
    }
}

fn builtin_office_buat_epub(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let title = match &args[0] { Value::String(s) => s.as_str(), _ => "Buku" };
    let author = match &args[1] { Value::String(s) => s.as_str(), _ => "Penulis" };
    let md_text = match &args[2] { Value::String(s) => s.as_str(), _ => "" };
    let out_path = match &args[3] { Value::String(s) => s.as_str(), _ => "output.epub" };

    let md = crate::office::MarkdownDocument::parse(md_text);
    let epub = crate::office::DocumentConverter::markdown_to_epub(&md, Some(title), Some(author));
    match epub.save_to_file(out_path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal mengekspor buku EPUB: {}", e), span)),
    }
}

fn builtin_office_buat_qrcode_svg(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    match crate::office::WidyaQrCode::encode(data) {
        Ok(qr) => Ok(Value::String(qr.to_svg(4, 2))),
        Err(e) => Err(Galat::runtime(format!("Gagal membuat QR Code SVG: {}", e), span)),
    }
}

fn builtin_office_buat_qrcode_pdf(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    let out_path = match &args[1] { Value::String(s) => s.as_str(), _ => "qrcode.pdf" };

    let qr = crate::office::WidyaQrCode::encode(data)
        .map_err(|e| Galat::runtime(format!("Gagal encode QR Code: {}", e), span))?;

    let mut pdf = crate::office::PdfDocument::new("QR Code Matrix");
    pdf.current_page_mut().add_styled_text("QR Code Verifikasi Resmi", 50.0, 750.0, 14.0, true, (0.1, 0.2, 0.4));
    qr.render_to_pdf(&mut pdf, 50.0, 700.0, 200.0);

    match pdf.save_to_file(out_path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(Galat::runtime(format!("Gagal menyimpan PDF QR Code: {}", e), span)),
    }
}

fn builtin_office_pdf_ekstrak_teks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let path_or_text = match &args[0] { Value::String(s) => s.as_str(), _ => "" };
    let path = std::path::Path::new(path_or_text);
    if path.exists() {
        match crate::office::PdfDocument::extract_text_from_file(path) {
            Ok(txt) => Ok(Value::String(txt)),
            Err(e) => Err(Galat::runtime(format!("Gagal ekstrak teks dari PDF: {}", e), span)),
        }
    } else {
        match crate::office::PdfDocument::extract_text_from_bytes(path_or_text.as_bytes()) {
            Ok(txt) => Ok(Value::String(txt)),
            Err(e) => Err(Galat::runtime(format!("Gagal ekstrak teks dari data PDF: {}", e), span)),
        }
    }
}

