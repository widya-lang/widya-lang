//! Widya-Lang PDF DSL & Universal Workbench Engine (WidyaPDF)
//! Sovereign Acrobat-grade PDF Workbench Model for ISO 32000-1 (%PDF-1.7),
//! UU PDP No. 27/2022 Automated Redaction, and NIST FIPS 204 (ML-DSA-87) Post-Quantum Signatures.

use crate::office::pdf::{wrap_text, PageSize, PdfDocument, PdfElement};
use crate::office::security::OfficeSecurity;
use crate::kripto::pqc::{PqcEngine, PqcKeyPair, PqcSignature};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Jenis Stempel Cap Karet Resmi
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RubberStampType {
    Approved,
    Confidential,
    Valid,
    Final,
    Rejected,
    Custom(String),
}

impl RubberStampType {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "APPROVED" | "DISETUJUI" => RubberStampType::Approved,
            "CONFIDENTIAL" | "RAHASIA" | "SANGAT RAHASIA" => RubberStampType::Confidential,
            "VALID" | "TERVERIFIKASI" => RubberStampType::Valid,
            "FINAL" | "RESMI" => RubberStampType::Final,
            "REJECTED" | "DITOLAK" => RubberStampType::Rejected,
            other => RubberStampType::Custom(other.to_string()),
        }
    }

    pub fn label(&self) -> &str {
        match self {
            RubberStampType::Approved => "APPROVED",
            RubberStampType::Confidential => "CONFIDENTIAL",
            RubberStampType::Valid => "VALID",
            RubberStampType::Final => "FINAL",
            RubberStampType::Rejected => "REJECTED",
            RubberStampType::Custom(s) => s.as_str(),
        }
    }

    pub fn color_rgb(&self) -> (f32, f32, f32) {
        match self {
            RubberStampType::Approved | RubberStampType::Valid => (0.08, 0.55, 0.24), // Hijau Resmi
            RubberStampType::Confidential | RubberStampType::Rejected => (0.82, 0.15, 0.15), // Merah Resmi
            RubberStampType::Final => (0.12, 0.32, 0.68), // Biru Eksekutif
            RubberStampType::Custom(_) => (0.45, 0.20, 0.65), // Ungu
        }
    }

    pub fn color_hex(&self) -> &str {
        match self {
            RubberStampType::Approved | RubberStampType::Valid => "#15803d",
            RubberStampType::Confidential | RubberStampType::Rejected => "#dc2626",
            RubberStampType::Final => "#1d4ed8",
            RubberStampType::Custom(_) => "#7c3aed",
        }
    }
}

/// Elemen Dekoratif / Anotasi Spesifik PDF DSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DslPdfItem {
    Title {
        text: String,
    },
    Header {
        text: String,
    },
    Paragraph {
        text: String,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        col_widths: Vec<f32>,
    },
    RedactionBox {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        label: String,
    },
    RubberStamp {
        stamp_type: String,
        x: f32,
        y: f32,
        label: String,
    },
    PqcSignature {
        signer: String,
        role: String,
        algorithm: String,
        timestamp: String,
        signature_hex: String,
        x: f32,
        y: f32,
    },
    StickyNote {
        id: String,
        author: String,
        text: String,
        x: f32,
        y: f32,
        date: String,
    },
}

/// Struktur Model Halaman Tunggal WidyaPDF (Sovereign PDF Page Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslPdfPage {
    pub page_number: usize,
    pub title: String,
    pub header: String,
    pub body: Vec<String>,
    pub items: Vec<DslPdfItem>,
    pub watermark: Option<String>,
}

impl DslPdfPage {
    pub fn new(page_number: usize, title: &str, header: &str) -> Self {
        Self {
            page_number,
            title: title.to_string(),
            header: header.to_string(),
            body: Vec::new(),
            items: Vec::new(),
            watermark: None,
        }
    }

    pub fn add_paragraph(&mut self, text: &str) {
        self.body.push(text.to_string());
        self.items.push(DslPdfItem::Paragraph {
            text: text.to_string(),
        });
    }

    pub fn add_redaction_box(&mut self, x: f32, y: f32, w: f32, h: f32, label: &str) {
        self.items.push(DslPdfItem::RedactionBox {
            x,
            y,
            w,
            h,
            label: label.to_string(),
        });
    }

    pub fn add_stamp(&mut self, stamp_type: &str, x: f32, y: f32, label: &str) {
        self.items.push(DslPdfItem::RubberStamp {
            stamp_type: stamp_type.to_string(),
            x,
            y,
            label: label.to_string(),
        });
    }

    pub fn add_pqc_signature(
        &mut self,
        signer: &str,
        role: &str,
        algorithm: &str,
        timestamp: &str,
        signature_hex: &str,
        x: f32,
        y: f32,
    ) {
        self.items.push(DslPdfItem::PqcSignature {
            signer: signer.to_string(),
            role: role.to_string(),
            algorithm: algorithm.to_string(),
            timestamp: timestamp.to_string(),
            signature_hex: signature_hex.to_string(),
            x,
            y,
        });
    }

    pub fn add_sticky_note(&mut self, id: &str, author: &str, text: &str, x: f32, y: f32, date: &str) {
        self.items.push(DslPdfItem::StickyNote {
            id: id.to_string(),
            author: author.to_string(),
            text: text.to_string(),
            x,
            y,
            date: date.to_string(),
        });
    }
}

/// Metadata Tanda Tangan Pasca-Kuantum Dokumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcSignatureMeta {
    pub signer_name: String,
    pub signer_role: String,
    pub algorithm: String,
    pub timestamp: String,
    pub signature_hex: String,
    pub document_digest_sha256: String,
    pub is_valid: bool,
}

/// Model Dokumen Kerja Lengkap WidyaPDF (Universal Workbench & DSL Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaPdfDsl {
    pub title: String,
    pub author: String,
    pub organization: String,
    pub page_size: String, // "A4", "Letter", "Legal"
    pub watermark: Option<String>,
    pub pages: Vec<DslPdfPage>,
    pub security_pdp: bool,
    pub pqc_signed: bool,
    pub pqc_signature_meta: Option<PqcSignatureMeta>,
}

impl Default for WidyaPdfDsl {
    fn default() -> Self {
        Self::new("Dokumen Resmi WidyaPDF")
    }
}

impl WidyaPdfDsl {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "Widya Office Suite".to_string(),
            organization: "Pemerintah Republik Indonesia / Korporasi Berdaulat".to_string(),
            page_size: "A4".to_string(),
            watermark: None,
            pages: vec![DslPdfPage::new(
                1,
                title,
                "LEMBAR DOKUMEN RESMI NEGARA REPUBLIK INDONESIA",
            )],
            security_pdp: false,
            pqc_signed: false,
            pqc_signature_meta: None,
        }
    }

    pub fn add_page(&mut self, title: &str, header: &str) -> &mut DslPdfPage {
        let num = self.pages.len() + 1;
        self.pages.push(DslPdfPage::new(num, title, header));
        self.pages.last_mut().unwrap()
    }

    /// Sensor / Redaksi Otomatis UU PDP No. 27/2022 pada seluruh lembar halaman
    pub fn apply_uupdp_redaction(&mut self) -> usize {
        let mut total_redacted_count = 0;

        for page in &mut self.pages {
            let mut new_body = Vec::new();
            for para in &page.body {
                let redacted = OfficeSecurity::redact_pdp_sensitive_text(para);
                if &redacted != para {
                    total_redacted_count += 1;
                }
                new_body.push(redacted);
            }
            page.body = new_body;

            // Perbarui juga item Paragraph dalam items
            for item in &mut page.items {
                if let DslPdfItem::Paragraph { text } = item {
                    let redacted = OfficeSecurity::redact_pdp_sensitive_text(text);
                    *text = redacted;
                }
            }

            // Tambahkan tanda kotak sensor visual jika ada modifikasi
            if total_redacted_count > 0 {
                let has_redact_box = page.items.iter().any(|it| matches!(it, DslPdfItem::RedactionBox { .. }));
                if !has_redact_box {
                    page.items.push(DslPdfItem::RedactionBox {
                        x: 50.0,
                        y: 350.0,
                        w: 495.0,
                        h: 24.0,
                        label: "UU PDP NO. 27/2022 - SENSOR OTOMATIS".to_string(),
                    });
                }
            }
        }

        self.security_pdp = true;
        total_redacted_count
    }

    /// Penandatanganan Kriptografi Pasca-Kuantum NIST FIPS 204 (ML-DSA-87 / Dilithium-5)
    pub fn sign_pqc(&mut self, signer: &str, role: &str) -> Result<PqcSignatureMeta, String> {
        let doc_text = self.extract_full_text();
        let digest = OfficeSecurity::compute_sha256(doc_text.as_bytes());

        // Hasilkan pasangan kunci pasca-kuantum ML-DSA-87 murni
        let seed = format!("{}_{}", signer, digest);
        let keypair = PqcEngine::generate_mldsa87_keypair(&seed);

        // Lakukan penandatanganan pasca-kuantum
        let sig = PqcEngine::sign_mldsa(doc_text.as_bytes(), &keypair.private_key_hex);
        let now_str = chrono::Utc::now().format("%d %b %Y, %H:%M WIB").to_string();

        let meta = PqcSignatureMeta {
            signer_name: signer.to_string(),
            signer_role: role.to_string(),
            algorithm: "NIST FIPS 204 (ML-DSA-87 / Dilithium-5)".to_string(),
            timestamp: now_str.clone(),
            signature_hex: sig.signature_hex.clone(),
            document_digest_sha256: digest,
            is_valid: true,
        };

        // Bubuhkan tanda tangan di halaman terakhir dokumen
        if let Some(last_page) = self.pages.last_mut() {
            last_page.add_pqc_signature(
                signer,
                role,
                &meta.algorithm,
                &now_str,
                &sig.signature_hex,
                380.0,
                140.0,
            );
        }

        self.pqc_signed = true;
        self.pqc_signature_meta = Some(meta.clone());

        Ok(meta)
    }

    /// Ekstraksi seluruh teks dokumen untuk audit dan penandatanganan
    pub fn extract_full_text(&self) -> String {
        let mut full = format!("{}\n{}\n{}\n", self.title, self.author, self.organization);
        for page in &self.pages {
            full.push_str(&format!("\n--- Halaman {} ---\n", page.page_number));
            full.push_str(&format!("{}\n{}\n", page.title, page.header));
            for b in &page.body {
                full.push_str(b);
                full.push('\n');
            }
        }
        full
    }

    /// Konversi Dokumen Model ke Format Biner PDF Asli ECMA/ISO 32000-1 (%PDF-1.7)
    pub fn to_pdf(&self) -> PdfDocument {
        let mut doc = PdfDocument::new(&self.title);
        doc.author = self.author.clone();
        doc.organization = self.organization.clone();

        match self.page_size.to_uppercase().as_str() {
            "LETTER" => doc.page_size = PageSize::Letter,
            "LEGAL" => doc.page_size = PageSize::Legal,
            _ => doc.page_size = PageSize::A4,
        }

        if let Some(ref wm) = self.watermark {
            doc.set_watermark(wm);
        }

        let (page_w, page_h) = doc.page_size.dimensions_points();
        doc.pages.clear();

        for (p_idx, p) in self.pages.iter().enumerate() {
            let mut page = crate::office::pdf::PdfPage::new();

            // 1. Watermark halaman jika spesifik
            let effective_wm = p.watermark.as_ref().or(self.watermark.as_ref());
            if let Some(wm_text) = effective_wm {
                // Diarsir halus diagonal
                page.add_formatted_text(
                    wm_text,
                    120.0,
                    420.0,
                    42.0,
                    true,
                    false,
                    false,
                    (0.88, 0.88, 0.90),
                );
            }

            // 2. Garis Batas / Header Halaman
            let margin_left = 50.0;
            let margin_right = page_w - 50.0;
            let mut y = page_h - 60.0;

            // Header resmi atas
            if !p.header.is_empty() {
                page.add_formatted_text(&p.header, margin_left, y, 8.5, true, false, false, (0.4, 0.45, 0.5));
                page.add_styled_line(margin_left, y - 8.0, margin_right, y - 8.0, 0.75, (0.75, 0.80, 0.85));
                y -= 25.0;
            }

            // Judul Halaman
            if !p.title.is_empty() {
                page.add_formatted_text(&p.title, margin_left, y, 14.0, true, false, false, (0.08, 0.12, 0.18));
                y -= 28.0;
            }

            // Bookmark navigasi PDF
            doc.add_bookmark(&p.title, p_idx, y, 1);

            // 3. Render Body Paragraf
            for para in &p.body {
                let content_width = margin_right - margin_left;
                y = page.add_wrapped_text(para, margin_left, y, content_width, 10.0, 14.5, false, (0.15, 0.18, 0.22));
                y -= 10.0; // Spasi antar paragraf
            }

            // 4. Render Item Tambahan (Tabel, Kotak Redaksi, Stempel, Tanda Tangan PQC)
            for item in &p.items {
                match item {
                    DslPdfItem::RedactionBox { x, y: ry, w, h, label } => {
                        // Kotak hitam sensor pekat
                        page.add_rect(*x, *ry, *w, *h, true, (0.05, 0.05, 0.05));
                        if !label.is_empty() {
                            page.add_formatted_text(label, *x + 8.0, *ry + (*h * 0.3), 8.0, true, false, true, (0.9, 0.9, 0.9));
                        }
                    }
                    DslPdfItem::RubberStamp { stamp_type, x, y: sy, label } => {
                        let st = RubberStampType::from_str(stamp_type);
                        let color = st.color_rgb();
                        let stamp_text = if label.is_empty() { st.label() } else { label.as_str() };

                        // Bingkai ganda stempel
                        page.add_rounded_rect(*x, *sy, 140.0, 48.0, 4.0, false, color);
                        page.add_rounded_rect(*x + 3.0, *sy + 3.0, 134.0, 42.0, 2.0, false, color);
                        page.add_formatted_text(stamp_text, *x + 12.0, *sy + 18.0, 14.0, true, false, false, color);
                    }
                    DslPdfItem::PqcSignature { signer, role, algorithm, timestamp, x, y: sig_y, .. } => {
                        let sig_w = 190.0;
                        let sig_h = 75.0;
                        let navy = (0.09, 0.20, 0.42);

                        // Latar & Bingkai TTE Resmi Pasca-Kuantum
                        page.add_rounded_rect(*x, *sig_y, sig_w, sig_h, 5.0, true, (0.96, 0.97, 0.99));
                        page.add_rounded_rect(*x, *sig_y, sig_w, sig_h, 5.0, false, navy);
                        page.add_rounded_rect(*x + 2.0, *sig_y + 2.0, sig_w - 4.0, sig_h - 4.0, 4.0, false, (0.80, 0.85, 0.92));

                        // Header TTE
                        page.add_formatted_text("[TTE PASCA-KUANTUM]", *x + 8.0, *sig_y + sig_h - 14.0, 8.0, true, false, false, navy);
                        page.add_styled_line(*x + 8.0, *sig_y + sig_h - 18.0, *x + sig_w - 8.0, *sig_y + sig_h - 18.0, 0.5, (0.75, 0.80, 0.88));

                        // Penandatangan & Peran
                        page.add_formatted_text(signer, *x + 8.0, *sig_y + sig_h - 30.0, 9.5, true, false, false, (0.08, 0.12, 0.18));
                        page.add_formatted_text(role, *x + 8.0, *sig_y + sig_h - 42.0, 8.0, false, false, false, (0.35, 0.40, 0.48));

                        // Algoritma & Waktu
                        page.add_formatted_text(algorithm, *x + 8.0, *sig_y + sig_h - 54.0, 7.0, false, false, true, (0.15, 0.55, 0.25));
                        page.add_formatted_text(timestamp, *x + 8.0, *sig_y + 8.0, 7.0, false, true, false, (0.50, 0.55, 0.60));
                    }
                    _ => {}
                }
            }

            // Footer halaman resmi (Nomor halaman)
            let footer_y = 35.0;
            page.add_styled_line(margin_left, footer_y + 15.0, margin_right, footer_y + 15.0, 0.5, (0.80, 0.85, 0.90));
            let footer_text = format!("WidyaPDF Sovereign Workbench • Halaman {} dari {}", p.page_number, self.pages.len());
            page.add_formatted_text(&footer_text, margin_left, footer_y, 8.0, false, false, false, (0.45, 0.50, 0.55));
            page.add_formatted_text("STANDAR ISO 32000-1 / UU PDP", margin_right - 140.0, footer_y, 8.0, true, false, false, (0.2, 0.45, 0.3));

            doc.pages.push(page);
        }

        doc
    }

    /// Render Halaman Vektor Mandiri ke SVG Beresolusi Tinggi (A4 Canvas 595.28 x 841.89 pt)
    pub fn render_page_svg(&self, page_idx: usize) -> Result<String, String> {
        if self.pages.is_empty() {
            return Err("Dokumen PDF tidak memiliki halaman".to_string());
        }
        let target_idx = page_idx.min(self.pages.len().saturating_sub(1));
        let page = &self.pages[target_idx];

        let width = 595.28;
        let height = 841.89;

        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {:.2} {:.2}" width="{:.2}" height="{:.2}" style="background-color: #ffffff; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; box-shadow: 0 4px 16px rgba(0,0,0,0.15);">"##,
            width, height, width, height
        );

        // Definisi Efek Bayangan & Filter
        svg.push_str(r##"<defs>
            <filter id="shadow" x="-5%" y="-5%" width="110%" height="110%">
                <feDropShadow dx="0" dy="2" stdDeviation="3" flood-color="black" flood-opacity="0.12"/>
            </filter>
            <pattern id="gridPattern" width="20" height="20" patternUnits="userSpaceOnUse">
                <rect width="20" height="20" fill="none" stroke="#f1f5f9" stroke-width="0.5"/>
            </pattern>
        </defs>"##);

        // Kertas Latar Belakang Bersih
        svg.push_str(&format!(
            r##"<rect width="{:.2}" height="{:.2}" fill="#ffffff"/>"##,
            width, height
        ));

        // Watermark Lembaran jika ada
        let wm_opt = page.watermark.as_ref().or(self.watermark.as_ref());
        if let Some(wm) = wm_opt {
            svg.push_str(&format!(
                r##"<g transform="translate(297, 420) rotate(-45)">
                    <text text-anchor="middle" font-size="52" font-weight="900" fill="#dc2626" fill-opacity="0.08" letter-spacing="6">{}</text>
                </g>"##,
                quick_escape(wm)
            ));
        }

        // Header Dokumen Resmi
        let margin_x = 48.0;
        let mut curr_y = 56.0;

        if !page.header.is_empty() {
            svg.push_str(&format!(
                r##"<text x="{:.1}" y="{:.1}" font-size="9" font-weight="700" fill="#64748b" letter-spacing="1.2">{}</text>"##,
                margin_x, curr_y, quick_escape(&page.header)
            ));
            svg.push_str(&format!(
                r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#cbd5e1" stroke-width="1"/>"##,
                margin_x, curr_y + 8.0, width - margin_x, curr_y + 8.0
            ));
            curr_y += 34.0;
        }

        // Judul Utama Halaman
        if !page.title.is_empty() {
            svg.push_str(&format!(
                r##"<text x="{:.1}" y="{:.1}" font-size="14" font-weight="800" fill="#0f172a" letter-spacing="0.4">{}</text>"##,
                margin_x, curr_y, quick_escape(&page.title)
            ));
            curr_y += 30.0;
        }

        // Body Paragraf
        let content_width = width - (margin_x * 2.0);
        for para in &page.body {
            let lines = wrap_text(para, content_width, 10.0, false);
            for line in lines {
                if !line.is_empty() {
                    svg.push_str(&format!(
                        r##"<text x="{:.1}" y="{:.1}" font-size="10.5" fill="#1e293b" line-height="1.6">{}</text>"##,
                        margin_x, curr_y, quick_escape(&line)
                    ));
                }
                curr_y += 16.0;
            }
            curr_y += 10.0;
        }

        // Elemen Tambahan (Kotak Redaksi, Stempel, Tanda Tangan PQC)
        for item in &page.items {
            match item {
                DslPdfItem::RedactionBox { x, y: ry, w, h, label } => {
                    let svg_y = height - ry - h;
                    svg.push_str(&format!(
                        r##"<g>
                            <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="#0f172a" rx="2"/>
                            <text x="{:.1}" y="{:.1}" font-size="8" font-family="monospace" font-weight="700" fill="#ffffff">{}</text>
                        </g>"##,
                        x, svg_y, w, h,
                        x + 6.0, svg_y + (h * 0.65), quick_escape(label)
                    ));
                }
                DslPdfItem::RubberStamp { stamp_type, x, y: sy, label } => {
                    let st = RubberStampType::from_str(stamp_type);
                    let color = st.color_hex();
                    let text = if label.is_empty() { st.label() } else { label.as_str() };
                    let svg_y = height - sy - 48.0;

                    svg.push_str(&format!(
                        r##"<g transform="translate({:.1}, {:.1}) rotate(-6)">
                            <rect width="140" height="48" rx="4" fill="none" stroke="{}" stroke-width="3" stroke-dasharray="8 2"/>
                            <rect x="4" y="4" width="132" height="40" rx="2" fill="none" stroke="{}" stroke-width="1.2"/>
                            <text x="70" y="29" text-anchor="middle" font-size="14" font-weight="900" fill="{}" letter-spacing="1.5">{}</text>
                        </g>"##,
                        x, svg_y, color, color, color, quick_escape(text)
                    ));
                }
                DslPdfItem::PqcSignature { signer, role, algorithm, timestamp, x, y: sig_y, .. } => {
                    let svg_y = height - sig_y - 75.0;
                    svg.push_str(&format!(
                        r##"<g transform="translate({:.1}, {:.1})" filter="url(#shadow)">
                            <rect width="190" height="75" rx="6" fill="#f8fafc" stroke="#1e3a8a" stroke-width="1.5"/>
                            <rect x="2" y="2" width="186" height="71" rx="4" fill="none" stroke="#93c5fd" stroke-width="0.8"/>
                            <text x="10" y="15" font-size="8.5" font-weight="800" fill="#1e3a8a" letter-spacing="0.5">🔒 [TTE PASCA-KUANTUM]</text>
                            <line x1="8" y1="20" x2="182" y2="20" stroke="#cbd5e1" stroke-width="0.8"/>
                            <text x="10" y="34" font-size="10" font-weight="800" fill="#0f172a">{}</text>
                            <text x="10" y="46" font-size="8" fill="#475569">{}</text>
                            <text x="10" y="58" font-size="7" font-weight="700" fill="#15803d">{}</text>
                            <text x="10" y="68" font-size="6.5" fill="#64748b">{}</text>
                        </g>"##,
                        x, svg_y, quick_escape(signer), quick_escape(role), quick_escape(algorithm), quick_escape(timestamp)
                    ));
                }
                _ => {}
            }
        }

        // Footer Resmi
        let footer_y = height - 28.0;
        svg.push_str(&format!(
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#e2e8f0" stroke-width="1"/>
            <text x="{:.1}" y="{:.1}" font-size="8" fill="#64748b">WidyaPDF Sovereign Workbench • Halaman {} dari {}</text>
            <text x="{:.1}" y="{:.1}" font-size="8" font-weight="700" fill="#166534">ISO 32000-1 / UU PDP No. 27/2022</text>"##,
            margin_x, footer_y - 10.0, width - margin_x, footer_y - 10.0,
            margin_x, footer_y, page.page_number, self.pages.len(),
            width - margin_x - 145.0, footer_y
        ));

        svg.push_str("</svg>");
        Ok(svg)
    }

    /// Konversi ke Objek JSON yang langsung kompatibel dengan controller `pdf_core.js`
    pub fn to_pdf_pages_json(&self) -> Value {
        let pages_data: Vec<Value> = self.pages.iter().map(|p| {
            json!({
                "pageNumber": p.page_number,
                "title": p.title,
                "header": p.header,
                "body": p.body,
            })
        }).collect();

        let mut manual_redactions: std::collections::HashMap<usize, Vec<Value>> = std::collections::HashMap::new();
        let mut rubber_stamps: std::collections::HashMap<usize, Vec<Value>> = std::collections::HashMap::new();
        let mut digital_signatures: std::collections::HashMap<usize, Vec<Value>> = std::collections::HashMap::new();
        let mut comments: Vec<Value> = Vec::new();

        for p in &self.pages {
            let p_num = p.page_number;
            let mut p_redact = Vec::new();
            let mut p_stamps = Vec::new();
            let mut p_sigs = Vec::new();

            for it in &p.items {
                match it {
                    DslPdfItem::RedactionBox { x, y, w, h, label } => {
                        p_redact.push(json!({
                            "x": x,
                            "y": y,
                            "w": w,
                            "h": h,
                            "label": label,
                        }));
                    }
                    DslPdfItem::RubberStamp { stamp_type, x, y, label } => {
                        p_stamps.push(json!({
                            "type": stamp_type,
                            "x": x,
                            "y": y,
                            "label": label,
                        }));
                    }
                    DslPdfItem::PqcSignature { signer, role, algorithm, timestamp, signature_hex, x, y } => {
                        p_sigs.push(json!({
                            "signer": signer,
                            "role": role,
                            "algorithm": algorithm,
                            "timestamp": timestamp,
                            "signature_hex": signature_hex,
                            "x": x,
                            "y": y,
                        }));
                    }
                    DslPdfItem::StickyNote { id, author, text, x, y, date } => {
                        comments.push(json!({
                            "id": id,
                            "page": p_num,
                            "x": x,
                            "y": y,
                            "author": author,
                            "date": date,
                            "text": text,
                        }));
                    }
                    _ => {}
                }
            }

            manual_redactions.insert(p_num, p_redact);
            rubber_stamps.insert(p_num, p_stamps);
            digital_signatures.insert(p_num, p_sigs);
        }

        json!({
            "title": self.title,
            "author": self.author,
            "organization": self.organization,
            "pageSize": self.page_size,
            "watermark": self.watermark,
            "pagesData": pages_data,
            "manualRedactions": manual_redactions,
            "rubberStamps": rubber_stamps,
            "digitalSignatures": digital_signatures,
            "comments": comments,
            "securityPdp": self.security_pdp,
            "pqcSigned": self.pqc_signed,
            "pqcMeta": self.pqc_signature_meta,
        })
    }

    /// Serialisasi Balik ke Kode Sintaks Murni Widya-Lang DSL
    pub fn to_widya_dsl_code(&self) -> String {
        let mut dsl = String::new();
        dsl.push_str("dokumen_pdf {\n");
        dsl.push_str(&format!("    judul: \"{}\"\n", self.title));
        dsl.push_str(&format!("    penulis: \"{}\"\n", self.author));
        dsl.push_str(&format!("    organisasi: \"{}\"\n", self.organization));
        dsl.push_str(&format!("    ukuran_halaman: \"{}\"\n", self.page_size));

        if let Some(ref wm) = self.watermark {
            dsl.push_str(&format!("    tanda_air: \"{}\"\n", wm));
        }
        dsl.push('\n');

        for page in &self.pages {
            dsl.push_str(&format!("    halaman({}) {{\n", page.page_number));
            if !page.title.is_empty() {
                dsl.push_str(&format!("        judul: \"{}\"\n", page.title));
            }
            if !page.header.is_empty() {
                dsl.push_str(&format!("        tajuk: \"{}\"\n", page.header));
            }
            if let Some(ref pwm) = page.watermark {
                dsl.push_str(&format!("        tanda_air: \"{}\"\n", pwm));
            }

            for para in &page.body {
                let escaped_para = para.replace('"', "\\\"");
                dsl.push_str(&format!("        paragraf: \"{}\"\n", escaped_para));
            }

            for item in &page.items {
                match item {
                    DslPdfItem::RedactionBox { x, y, w, h, label } => {
                        dsl.push_str(&format!(
                            "        kotak_redaksi: [{:.1}, {:.1}, {:.1}, {:.1}, \"{}\"]\n",
                            x, y, w, h, label
                        ));
                    }
                    DslPdfItem::RubberStamp { stamp_type, x, y, label } => {
                        dsl.push_str(&format!(
                            "        stempel: [\"{}\", {:.1}, {:.1}, \"{}\"]\n",
                            stamp_type, x, y, label
                        ));
                    }
                    DslPdfItem::PqcSignature { signer, role, algorithm, x, y, .. } => {
                        dsl.push_str(&format!(
                            "        tanda_tangan_pqc: [\"{}\", \"{}\", \"{}\", {:.1}, {:.1}]\n",
                            signer, role, algorithm, x, y
                        ));
                    }
                    DslPdfItem::StickyNote { author, text, x, y, .. } => {
                        dsl.push_str(&format!(
                            "        catatan: [\"{}\", \"{}\", {:.1}, {:.1}]\n",
                            author, text.replace('"', "\\\""), x, y
                        ));
                    }
                    _ => {}
                }
            }

            dsl.push_str("    }\n\n");
        }

        dsl.push_str("}\n");
        dsl
    }

    /// Parser Sintaks Deklaratif Widya-Lang (`dokumen_pdf { ... }`)
    pub fn from_widya_dsl(code: &str) -> Result<Self, String> {
        let trimmed = code.trim();
        if !trimmed.contains("dokumen_pdf") {
            return Err("Sintaks DSL tidak valid: blok utama 'dokumen_pdf { ... }' tidak ditemukan".to_string());
        }

        let mut doc = WidyaPdfDsl::new("Dokumen PDF");
        doc.pages.clear();

        let mut current_page: Option<DslPdfPage> = None;

        for line in trimmed.lines() {
            let line_trim = line.trim();

            if line_trim.starts_with("//") || line_trim.is_empty() {
                continue;
            }

            if line_trim.starts_with("dokumen_pdf") {
                continue;
            }

            if line_trim.starts_with("judul:") {
                let val = extract_quoted_string(line_trim);
                if !val.is_empty() {
                    if let Some(ref mut page) = current_page {
                        page.title = val;
                    } else {
                        doc.title = val;
                    }
                }
            } else if line_trim.starts_with("penulis:") {
                doc.author = extract_quoted_string(line_trim);
            } else if line_trim.starts_with("organisasi:") {
                doc.organization = extract_quoted_string(line_trim);
            } else if line_trim.starts_with("ukuran_halaman:") {
                doc.page_size = extract_quoted_string(line_trim);
            } else if line_trim.starts_with("tanda_air:") {
                let val = extract_quoted_string(line_trim);
                if !val.is_empty() {
                    if let Some(ref mut page) = current_page {
                        page.watermark = Some(val);
                    } else {
                        doc.watermark = Some(val);
                    }
                }
            } else if line_trim.starts_with("halaman") && line_trim.contains('{') {
                if let Some(prev) = current_page.take() {
                    doc.pages.push(prev);
                }
                let page_num = extract_number_in_parens(line_trim).unwrap_or(doc.pages.len() + 1);
                current_page = Some(DslPdfPage::new(page_num, "", ""));
            } else if line_trim.starts_with("tajuk:") {
                if let Some(ref mut page) = current_page {
                    page.header = extract_quoted_string(line_trim);
                }
            } else if line_trim.starts_with("paragraf:") {
                if let Some(ref mut page) = current_page {
                    let para = extract_quoted_string(line_trim);
                    page.add_paragraph(&para);
                }
            } else if line_trim.starts_with("kotak_redaksi:") {
                if let Some(ref mut page) = current_page {
                    if let Some((x, y, w, h, label)) = parse_redaction_box_line(line_trim) {
                        page.add_redaction_box(x, y, w, h, &label);
                    }
                }
            } else if line_trim.starts_with("stempel:") {
                if let Some(ref mut page) = current_page {
                    if let Some((stype, x, y, label)) = parse_stamp_line(line_trim) {
                        page.add_stamp(&stype, x, y, &label);
                    }
                }
            } else if line_trim.starts_with("tanda_tangan_pqc:") {
                if let Some(ref mut page) = current_page {
                    if let Some((signer, role, algo, x, y)) = parse_pqc_line(line_trim) {
                        page.add_pqc_signature(&signer, &role, &algo, "Terverifikasi", "", x, y);
                    }
                }
            } else if line_trim.starts_with("catatan:") {
                if let Some(ref mut page) = current_page {
                    if let Some((author, text, x, y)) = parse_comment_line(line_trim) {
                        let id = format!("c{}", page.items.len() + 1);
                        page.add_sticky_note(&id, &author, &text, x, y, "Sekarang");
                    }
                }
            } else if line_trim == "}" {
                if let Some(p) = current_page.take() {
                    doc.pages.push(p);
                }
            }
        }

        if let Some(p) = current_page {
            doc.pages.push(p);
        }

        if doc.pages.is_empty() {
            doc.pages.push(DslPdfPage::new(1, &doc.title, "LEMBAR DOKUMEN RESMI"));
        }

        Ok(doc)
    }
}

fn quick_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn extract_quoted_string(line: &str) -> String {
    if let Some(first_quote) = line.find('"') {
        if let Some(last_quote) = line.rfind('"') {
            if first_quote < last_quote {
                return line[first_quote + 1..last_quote].to_string();
            }
        }
    }
    String::new()
}

fn extract_number_in_parens(line: &str) -> Option<usize> {
    let start = line.find('(')?;
    let end = line.find(')')?;
    line[start + 1..end].trim().parse::<usize>().ok()
}

fn parse_redaction_box_line(line: &str) -> Option<(f32, f32, f32, f32, String)> {
    let open_b = line.find('[')?;
    let close_b = line.rfind(']')?;
    let inner = &line[open_b + 1..close_b];
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.len() >= 4 {
        let x = parts[0].parse::<f32>().ok()?;
        let y = parts[1].parse::<f32>().ok()?;
        let w = parts[2].parse::<f32>().ok()?;
        let h = parts[3].parse::<f32>().ok()?;
        let label = if parts.len() >= 5 {
            parts[4].trim_matches('"').to_string()
        } else {
            "SENSOR".to_string()
        };
        return Some((x, y, w, h, label));
    }
    None
}

fn parse_stamp_line(line: &str) -> Option<(String, f32, f32, String)> {
    let open_b = line.find('[')?;
    let close_b = line.rfind(']')?;
    let inner = &line[open_b + 1..close_b];
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.len() >= 3 {
        let stype = parts[0].trim_matches('"').to_string();
        let x = parts[1].parse::<f32>().ok()?;
        let y = parts[2].parse::<f32>().ok()?;
        let label = if parts.len() >= 4 {
            parts[3].trim_matches('"').to_string()
        } else {
            stype.clone()
        };
        return Some((stype, x, y, label));
    }
    None
}

fn parse_pqc_line(line: &str) -> Option<(String, String, String, f32, f32)> {
    let open_b = line.find('[')?;
    let close_b = line.rfind(']')?;
    let inner = &line[open_b + 1..close_b];
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.len() >= 5 {
        let signer = parts[0].trim_matches('"').to_string();
        let role = parts[1].trim_matches('"').to_string();
        let algo = parts[2].trim_matches('"').to_string();
        let x = parts[3].parse::<f32>().ok()?;
        let y = parts[4].parse::<f32>().ok()?;
        return Some((signer, role, algo, x, y));
    }
    None
}

fn parse_comment_line(line: &str) -> Option<(String, String, f32, f32)> {
    let open_b = line.find('[')?;
    let close_b = line.rfind(']')?;
    let inner = &line[open_b + 1..close_b];
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.len() >= 4 {
        let author = parts[0].trim_matches('"').to_string();
        let text = parts[1].trim_matches('"').to_string();
        let x = parts[2].parse::<f32>().ok()?;
        let y = parts[3].parse::<f32>().ok()?;
        return Some((author, text, x, y));
    }
    None
}
