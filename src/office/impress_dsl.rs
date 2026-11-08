//! Widya-Lang Presentation DSL & Universal Slide Engine (WidyaImpress)
//! Sovereign DeckCraft-grade Presentation Model for OpenXML PPTX, Markdown, and Interactive Vector SVG Slides.

use crate::office::pptx::{MetricKpiCard, PptxPresentation, PptxSlide, SlideLayout, SlideTheme};
use serde::{Deserialize, Serialize};

/// Kartu Metrik KPI untuk Slide Presentasi Deklaratif
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslKpiCard {
    pub label: String,
    pub value: String,
    pub subtext: String,
    pub positive: bool,
}

impl Default for DslKpiCard {
    fn default() -> Self {
        Self {
            label: "Metrik Kunci".to_string(),
            value: "100%".to_string(),
            subtext: "Standar Kedaulatan".to_string(),
            positive: true,
        }
    }
}

/// Struktur Model Slide Tunggal (WidyaImpress Slide Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslSlide {
    pub layout: String, // "title", "kpi", "content", "two-column", "blank"
    pub title: String,
    pub subtitle: Option<String>,
    pub bullets: Vec<String>,
    pub left_title: Option<String>,
    pub left_bullets: Vec<String>,
    pub right_title: Option<String>,
    pub right_bullets: Vec<String>,
    pub kpi_cards: Vec<DslKpiCard>,
    pub notes: Option<String>,
    pub hidden: bool,
    pub custom_bg: Option<String>,
}

impl DslSlide {
    pub fn new_title(title: &str, subtitle: Option<&str>, notes: Option<&str>) -> Self {
        Self {
            layout: "title".to_string(),
            title: title.to_string(),
            subtitle: subtitle.map(|s| s.to_string()),
            bullets: Vec::new(),
            left_title: None,
            left_bullets: Vec::new(),
            right_title: None,
            right_bullets: Vec::new(),
            kpi_cards: Vec::new(),
            notes: notes.map(|s| s.to_string()),
            hidden: false,
            custom_bg: None,
        }
    }

    pub fn new_content(title: &str, subtitle: Option<&str>, bullets: Vec<String>, notes: Option<&str>) -> Self {
        Self {
            layout: "content".to_string(),
            title: title.to_string(),
            subtitle: subtitle.map(|s| s.to_string()),
            bullets,
            left_title: None,
            left_bullets: Vec::new(),
            right_title: None,
            right_bullets: Vec::new(),
            kpi_cards: Vec::new(),
            notes: notes.map(|s| s.to_string()),
            hidden: false,
            custom_bg: None,
        }
    }

    pub fn new_kpi(title: &str, subtitle: Option<&str>, cards: Vec<DslKpiCard>, notes: Option<&str>) -> Self {
        Self {
            layout: "kpi".to_string(),
            title: title.to_string(),
            subtitle: subtitle.map(|s| s.to_string()),
            bullets: Vec::new(),
            left_title: None,
            left_bullets: Vec::new(),
            right_title: None,
            right_bullets: Vec::new(),
            kpi_cards: cards,
            notes: notes.map(|s| s.to_string()),
            hidden: false,
            custom_bg: None,
        }
    }

    pub fn new_two_column(
        title: &str,
        subtitle: Option<&str>,
        left_title: Option<&str>,
        left_bullets: Vec<String>,
        right_title: Option<&str>,
        right_bullets: Vec<String>,
        notes: Option<&str>,
    ) -> Self {
        Self {
            layout: "two-column".to_string(),
            title: title.to_string(),
            subtitle: subtitle.map(|s| s.to_string()),
            bullets: Vec::new(),
            left_title: left_title.map(|s| s.to_string()),
            left_bullets,
            right_title: right_title.map(|s| s.to_string()),
            right_bullets,
            kpi_cards: Vec::new(),
            notes: notes.map(|s| s.to_string()),
            hidden: false,
            custom_bg: None,
        }
    }
}

/// Dokumen Presentasi Berdaulat (WidyaPresentationDsl / DeckCraft Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaPresentationDsl {
    pub title: String,
    pub author: String,
    pub theme: String, // "theme-executive-navy", "theme-dark-modern", "theme-emerald-fintech", "theme-sovereign-gold"
    pub aspect_ratio: String, // "16:9" atau "4:3"
    pub slides: Vec<DslSlide>,
}

impl Default for WidyaPresentationDsl {
    fn default() -> Self {
        Self::new("Presentasi_WidyaImpress")
    }
}

impl WidyaPresentationDsl {
    pub fn new(title: &str) -> Self {
        let mut pres = Self {
            title: title.to_string(),
            author: "Widya Sovereign Presentation Engine".to_string(),
            theme: "theme-executive-navy".to_string(),
            aspect_ratio: "16:9".to_string(),
            slides: Vec::new(),
        };

        // Tambahkan slide pembuka default
        pres.slides.push(DslSlide::new_title(
            title,
            Some("Peta Jalan Transformasi Sistem Komputasi Berdaulat"),
            Some("Slide pembuka eksekutif."),
        ));

        pres
    }

    pub fn add_slide(&mut self, slide: DslSlide) {
        self.slides.push(slide);
    }

    /// Parser dari Skrip Kode Bahasa Widya-Lang (`.widya` / `.wpres`)
    pub fn from_widya_dsl(code: &str) -> Result<Self, String> {
        let mut title = "Presentasi Widya".to_string();
        let mut author = "Tim Riset Widya".to_string();
        let mut theme = "theme-executive-navy".to_string();
        let mut aspect_ratio = "16:9".to_string();
        let mut slides = Vec::new();

        // 1. Deteksi blok presentasi { ... }
        let block_content = if let Some(start_idx) = code.find("presentasi {") {
            let inner = &code[start_idx + 12..];
            if let Some(end_idx) = inner.rfind('}') {
                &inner[..end_idx]
            } else {
                inner
            }
        } else {
            code
        };

        // 2. Ekstraksi Metadata Header (hanya sebelum pemanggilan slide pertama)
        let header_part = if let Some((_, first_slide_pos)) = find_next_keyword(block_content, &["slide_judul(", "slide_konten(", "slide_kpi(", "slide_dua_kolom("]) {
            &block_content[..first_slide_pos]
        } else {
            block_content
        };

        for line in header_part.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("judul:") {
                title = extract_quoted_string(trimmed).unwrap_or(title);
            } else if trimmed.starts_with("penulis:") {
                author = extract_quoted_string(trimmed).unwrap_or(author);
            } else if trimmed.starts_with("tema:") {
                let raw_theme = extract_quoted_string(trimmed).unwrap_or_else(|| "navy".to_string());
                theme = match raw_theme.to_lowercase().as_str() {
                    "navy" | "executive" | "theme-executive-navy" => "theme-executive-navy".to_string(),
                    "dark" | "modern" | "theme-dark-modern" => "theme-dark-modern".to_string(),
                    "emerald" | "fintech" | "theme-emerald-fintech" => "theme-emerald-fintech".to_string(),
                    "gold" | "sovereign" | "theme-sovereign-gold" => "theme-sovereign-gold".to_string(),
                    other => other.to_string(),
                };
            } else if trimmed.starts_with("rasio:") {
                aspect_ratio = extract_quoted_string(trimmed).unwrap_or(aspect_ratio);
            }
        }

        // 3. Ekstraksi Slide-Slide Deklaratif
        let slide_blocks = split_slide_blocks(block_content);
        for (kind, content) in slide_blocks {
            match kind.as_str() {
                "slide_judul" => {
                    let s_title = extract_field_str(&content, "judul").unwrap_or_else(|| "Judul Slide".to_string());
                    let s_sub = extract_field_str(&content, "subjudul");
                    let s_notes = extract_field_str(&content, "catatan");
                    slides.push(DslSlide::new_title(&s_title, s_sub.as_deref(), s_notes.as_deref()));
                }
                "slide_konten" => {
                    let s_title = extract_field_str(&content, "judul").unwrap_or_else(|| "Judul Konten".to_string());
                    let s_sub = extract_field_str(&content, "subjudul");
                    let s_notes = extract_field_str(&content, "catatan");
                    let bullets = extract_string_array(&content, "poin");
                    slides.push(DslSlide::new_content(&s_title, s_sub.as_deref(), bullets, s_notes.as_deref()));
                }
                "slide_kpi" => {
                    let s_title = extract_field_str(&content, "judul").unwrap_or_else(|| "Dashboard Metrik".to_string());
                    let s_sub = extract_field_str(&content, "subjudul");
                    let s_notes = extract_field_str(&content, "catatan");
                    let cards = extract_kpi_cards(&content);
                    slides.push(DslSlide::new_kpi(&s_title, s_sub.as_deref(), cards, s_notes.as_deref()));
                }
                "slide_dua_kolom" => {
                    let s_title = extract_field_str(&content, "judul").unwrap_or_else(|| "Perbandingan Kolom".to_string());
                    let s_sub = extract_field_str(&content, "subjudul");
                    let s_notes = extract_field_str(&content, "catatan");

                    let left_title = extract_field_str(&content, "kiri_judul");
                    let left_bullets = extract_string_array(&content, "kiri_poin");

                    let right_title = extract_field_str(&content, "kanan_judul");
                    let right_bullets = extract_string_array(&content, "kanan_poin");

                    slides.push(DslSlide::new_two_column(
                        &s_title,
                        s_sub.as_deref(),
                        left_title.as_deref(),
                        left_bullets,
                        right_title.as_deref(),
                        right_bullets,
                        s_notes.as_deref(),
                    ));
                }
                _ => {}
            }
        }

        if slides.is_empty() {
            slides.push(DslSlide::new_title(&title, None, None));
        }

        Ok(Self {
            title,
            author,
            theme,
            aspect_ratio,
            slides,
        })
    }

    /// Parser dari Markdown Presentasi Standar (Marp / Slidev Parity)
    /// Pemisah slide: `---`
    /// Judul slide: `# Judul`
    /// Subjudul: `## Subjudul`
    /// Poin peluru: `- Poin` atau `* Poin`
    /// Catatan pembicara: `<!-- notes: ... -->` atau `> Catatan: ...`
    pub fn from_markdown(md_text: &str, title_opt: Option<&str>) -> Self {
        let default_title = title_opt.unwrap_or("Presentasi Markdown");
        let mut presentation = Self::new(default_title);
        presentation.slides.clear();

        let raw_sections: Vec<&str> = md_text.split("\n---").collect();

        for (sec_idx, sec) in raw_sections.iter().enumerate() {
            let lines: Vec<&str> = sec.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
            if lines.is_empty() {
                continue;
            }

            let mut slide_title = String::new();
            let mut slide_sub = None;
            let mut bullets = Vec::new();
            let mut notes = String::new();

            for line in lines {
                if line.starts_with("# ") {
                    slide_title = line[2..].trim().to_string();
                } else if line.starts_with("## ") && slide_sub.is_none() {
                    slide_sub = Some(line[3..].trim().to_string());
                } else if line.starts_with("- ") || line.starts_with("* ") {
                    bullets.push(line[2..].trim().to_string());
                } else if line.starts_with("<!--") && line.contains("notes:") {
                    let note_content = line.replace("<!--", "").replace("-->", "").replace("notes:", "");
                    notes.push_str(note_content.trim());
                    notes.push('\n');
                } else if line.starts_with("> Catatan:") || line.starts_with("> Note:") {
                    let colon_idx = line.find(':').unwrap_or(8);
                    notes.push_str(line[colon_idx + 1..].trim());
                    notes.push('\n');
                } else if !slide_title.is_empty() && bullets.is_empty() && slide_sub.is_none() && !line.starts_with('#') {
                    slide_sub = Some(line.to_string());
                }
            }

            if slide_title.is_empty() {
                slide_title = format!("Slide {}", sec_idx + 1);
            }

            let notes_opt = if notes.trim().is_empty() { None } else { Some(notes.trim().to_string()) };

            if sec_idx == 0 && bullets.is_empty() {
                presentation.slides.push(DslSlide::new_title(&slide_title, slide_sub.as_deref(), notes_opt.as_deref()));
            } else {
                presentation.slides.push(DslSlide::new_content(&slide_title, slide_sub.as_deref(), bullets, notes_opt.as_deref()));
            }
        }

        if presentation.slides.is_empty() {
            presentation.slides.push(DslSlide::new_title(default_title, None, None));
        }

        presentation
    }

    /// Konversi ke Paket OpenXML PPTX Murni (ECMA-376 Standard)
    pub fn to_pptx(&self) -> PptxPresentation {
        let mut pptx = PptxPresentation::new(&self.title);
        pptx.author = self.author.clone();

        pptx.theme = match self.theme.as_str() {
            "theme-dark-modern" => SlideTheme::DarkModern,
            "theme-emerald-fintech" => SlideTheme::EmeraldFintech,
            "theme-sovereign-gold" => SlideTheme::SovereignGold,
            _ => SlideTheme::ExecutiveNavy,
        };

        for s in &self.slides {
            let mut slide = match s.layout.as_str() {
                "title" => PptxSlide::title_slide(&s.title, s.subtitle.as_deref().unwrap_or("")),
                "kpi" => {
                    let kpi_metrics: Vec<MetricKpiCard> = s.kpi_cards.iter().map(|c| {
                        MetricKpiCard {
                            label: c.label.clone(),
                            value: c.value.clone(),
                            subtext: c.subtext.clone(),
                            is_positive: c.positive,
                        }
                    }).collect();
                    PptxSlide::kpi_slide(&s.title, kpi_metrics)
                }
                "two-column" => {
                    let left_refs: Vec<&str> = s.left_bullets.iter().map(|b| b.as_str()).collect();
                    let right_refs: Vec<&str> = s.right_bullets.iter().map(|b| b.as_str()).collect();
                    PptxSlide::two_column_slide(&s.title, &left_refs, &right_refs)
                }
                _ => {
                    let bullet_refs: Vec<&str> = s.bullets.iter().map(|b| b.as_str()).collect();
                    PptxSlide::content_slide(&s.title, &bullet_refs)
                }
            };

            if let Some(notes) = &s.notes {
                slide = slide.with_notes(notes);
            }

            pptx.add_slide(slide);
        }

        pptx
    }

    /// Serialisasi Slide ke Format Objek JSON untuk Antarmuka WidyaImpress UI Core
    pub fn to_slides_json(&self) -> serde_json::Value {
        let slides_val: Vec<serde_json::Value> = self.slides.iter().map(|s| {
            let kpi_cards_val: Vec<serde_json::Value> = s.kpi_cards.iter().map(|c| {
                serde_json::json!({
                    "label": c.label,
                    "value": c.value,
                    "subtext": c.subtext,
                    "positive": c.positive,
                })
            }).collect();

            serde_json::json!({
                "layout": s.layout,
                "title": s.title,
                "subtitle": s.subtitle.clone().unwrap_or_default(),
                "bullets": s.bullets,
                "leftTitle": s.left_title.clone().unwrap_or_default(),
                "leftBullets": s.left_bullets,
                "rightTitle": s.right_title.clone().unwrap_or_default(),
                "rightBullets": s.right_bullets,
                "kpiCards": kpi_cards_val,
                "notes": s.notes.clone().unwrap_or_default(),
                "hidden": s.hidden,
                "customBg": s.custom_bg,
            })
        }).collect();

        serde_json::json!({
            "title": self.title,
            "author": self.author,
            "theme": self.theme,
            "aspectRatio": self.aspect_ratio,
            "slides": slides_val,
        })
    }

    /// Ekspor Struktur Presentasi ke Kode Sumber Deklaratif Bahasa Widya-Lang (.widya)
    pub fn to_widya_dsl_code(&self) -> String {
        let mut out = String::new();
        out.push_str("presentasi {\n");
        out.push_str(&format!("    judul: \"{}\",\n", self.title.replace('"', "\\\"")));
        out.push_str(&format!("    penulis: \"{}\",\n", self.author.replace('"', "\\\"")));
        out.push_str(&format!("    tema: \"{}\",\n", self.theme));
        out.push_str(&format!("    rasio: \"{}\",\n\n", self.aspect_ratio));

        for s in &self.slides {
            match s.layout.as_str() {
                "title" => {
                    out.push_str("    slide_judul(\n");
                    out.push_str(&format!("        judul: \"{}\"", s.title.replace('"', "\\\"")));
                    if let Some(sub) = &s.subtitle {
                        out.push_str(&format!(",\n        subjudul: \"{}\"", sub.replace('"', "\\\"")));
                    }
                    if let Some(notes) = &s.notes {
                        out.push_str(&format!(",\n        catatan: \"{}\"", notes.replace('"', "\\\"")));
                    }
                    out.push_str("\n    )\n\n");
                }
                "kpi" => {
                    out.push_str("    slide_kpi(\n");
                    out.push_str(&format!("        judul: \"{}\"", s.title.replace('"', "\\\"")));
                    if let Some(sub) = &s.subtitle {
                        out.push_str(&format!(",\n        subjudul: \"{}\"", sub.replace('"', "\\\"")));
                    }
                    out.push_str(",\n        kartu: [\n");
                    for (c_idx, c) in s.kpi_cards.iter().enumerate() {
                        let comma = if c_idx + 1 < s.kpi_cards.len() { "," } else { "" };
                        out.push_str(&format!(
                            "            {{ label: \"{}\", nilai: \"{}\", subteks: \"{}\", positif: {} }}{}\n",
                            c.label.replace('"', "\\\""),
                            c.value.replace('"', "\\\""),
                            c.subtext.replace('"', "\\\""),
                            if c.positive { "benar" } else { "salah" },
                            comma
                        ));
                    }
                    out.push_str("        ]");
                    if let Some(notes) = &s.notes {
                        out.push_str(&format!(",\n        catatan: \"{}\"", notes.replace('"', "\\\"")));
                    }
                    out.push_str("\n    )\n\n");
                }
                "two-column" => {
                    out.push_str("    slide_dua_kolom(\n");
                    out.push_str(&format!("        judul: \"{}\"", s.title.replace('"', "\\\"")));
                    if let Some(sub) = &s.subtitle {
                        out.push_str(&format!(",\n        subjudul: \"{}\"", sub.replace('"', "\\\"")));
                    }
                    if let Some(lt) = &s.left_title {
                        out.push_str(&format!(",\n        kiri_judul: \"{}\"", lt.replace('"', "\\\"")));
                    }
                    let left_pts = s.left_bullets.iter().map(|b| format!("\"{}\"", b.replace('"', "\\\""))).collect::<Vec<_>>().join(", ");
                    out.push_str(&format!(",\n        kiri_poin: [{}]", left_pts));

                    if let Some(rt) = &s.right_title {
                        out.push_str(&format!(",\n        kanan_judul: \"{}\"", rt.replace('"', "\\\"")));
                    }
                    let right_pts = s.right_bullets.iter().map(|b| format!("\"{}\"", b.replace('"', "\\\""))).collect::<Vec<_>>().join(", ");
                    out.push_str(&format!(",\n        kanan_poin: [{}]", right_pts));

                    if let Some(notes) = &s.notes {
                        out.push_str(&format!(",\n        catatan: \"{}\"", notes.replace('"', "\\\"")));
                    }
                    out.push_str("\n    )\n\n");
                }
                _ => {
                    out.push_str("    slide_konten(\n");
                    out.push_str(&format!("        judul: \"{}\"", s.title.replace('"', "\\\"")));
                    if let Some(sub) = &s.subtitle {
                        out.push_str(&format!(",\n        subjudul: \"{}\"", sub.replace('"', "\\\"")));
                    }
                    let pts = s.bullets.iter().map(|b| format!("\"{}\"", b.replace('"', "\\\""))).collect::<Vec<_>>().join(", ");
                    out.push_str(&format!(",\n        poin: [{}]", pts));
                    if let Some(notes) = &s.notes {
                        out.push_str(&format!(",\n        catatan: \"{}\"", notes.replace('"', "\\\"")));
                    }
                    out.push_str("\n    )\n\n");
                }
            }
        }

        out.push_str("}\n");
        out
    }

    /// Generator Grafik Vektor SVG Slide Mandiri Beresolusi Tinggi (Standalone Slide Vector SVG)
    pub fn render_slide_svg(&self, slide_idx: usize) -> Result<String, String> {
        let slide = self.slides.get(slide_idx)
            .ok_or_else(|| format!("Slide indeks {} tidak ditemukan", slide_idx))?;

        let (width, height) = if self.aspect_ratio == "4:3" { (960.0, 720.0) } else { (960.0, 540.0) };

        // Palet warna berdasarkan tema
        let (bg_color, bg_gradient_end, title_color, sub_color, text_color, card_bg, card_border, accent_color) = match self.theme.as_str() {
            "theme-dark-modern" => ("#090d16", "#111827", "#f8fafc", "#94a3b8", "#e2e8f0", "#1e293b", "#334155", "#38bdf8"),
            "theme-emerald-fintech" => ("#06231a", "#0f3d2e", "#ecfdf5", "#a7f3d0", "#d1fae5", "#134e3a", "#059669", "#34d399"),
            "theme-sovereign-gold" => ("#1c1917", "#292524", "#fef3c7", "#fde68a", "#f5f5f4", "#44403c", "#78716c", "#fbbf24"),
            _ => ("#071b3b", "#0f2e5a", "#f8fafc", "#93c5fd", "#e0e7ff", "#13386b", "#1d4ed8", "#60a5fa"), // executive navy
        };

        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="100%" height="auto" style="border-radius:8px; box-shadow:0 8px 24px rgba(0,0,0,0.3); font-family:-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;">
  <defs>
    <linearGradient id="bgGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="{bg_color}" />
      <stop offset="100%" stop-color="{bg_gradient_end}" />
    </linearGradient>
    <filter id="cardShadow" x="-10%" y="-10%" width="120%" height="120%">
      <feDropShadow dx="0" dy="4" stdDeviation="6" flood-color="black" flood-opacity="0.25"/>
    </filter>
  </defs>
  <!-- Background Canvas -->
  <rect width="{width}" height="{height}" fill="url(#bgGrad)" rx="8" />
  <!-- Sovereign Watermark Emblem -->
  <circle cx="{cx}" cy="{cy}" r="220" fill="none" stroke="{accent_color}" stroke-opacity="0.04" stroke-width="60" />
"#,
            width = width,
            height = height,
            bg_color = bg_color,
            bg_gradient_end = bg_gradient_end,
            accent_color = accent_color,
            cx = width - 120.0,
            cy = height - 100.0,
        );

        match slide.layout.as_str() {
            "title" => {
                // Layout Slide Pembuka
                svg.push_str(&format!(
                    r#"  <!-- Title Slide -->
  <rect x="80" y="{accent_bar_y}" width="6" height="110" fill="{accent_color}" rx="3" />
  <text x="105" y="{title_y}" font-size="34" font-weight="800" fill="{title_color}" letter-spacing="-0.5">{title}</text>
  <text x="105" y="{sub_y}" font-size="18" font-weight="400" fill="{sub_color}">{subtitle}</text>
  <line x1="80" y1="{line_y}" x2="400" y2="{line_y}" stroke="{card_border}" stroke-width="1.5" />
  <text x="80" y="{footer_y}" font-size="13" font-weight="600" fill="{accent_color}">{author} • WIDYAIMPRESS DECKCRAFT</text>
"#,
                    accent_bar_y = height / 2.0 - 65.0,
                    accent_color = accent_color,
                    title_y = height / 2.0 - 20.0,
                    title_color = title_color,
                    title = escape_xml(&slide.title),
                    sub_y = height / 2.0 + 25.0,
                    sub_color = sub_color,
                    subtitle = escape_xml(slide.subtitle.as_deref().unwrap_or("")),
                    line_y = height / 2.0 + 60.0,
                    card_border = card_border,
                    footer_y = height / 2.0 + 85.0,
                    author = escape_xml(&self.author),
                ));
            }
            "kpi" => {
                // Layout Slide Dashboard Metrik KPI
                svg.push_str(&format!(
                    r#"  <!-- Header -->
  <text x="60" y="70" font-size="26" font-weight="800" fill="{title_color}">{title}</text>
  <text x="60" y="98" font-size="15" font-weight="400" fill="{sub_color}">{subtitle}</text>
  <line x1="60" y1="115" x2="{width_pad}" y2="115" stroke="{card_border}" stroke-width="1" />
"#,
                    title_color = title_color,
                    title = escape_xml(&slide.title),
                    sub_color = sub_color,
                    subtitle = escape_xml(slide.subtitle.as_deref().unwrap_or("")),
                    width_pad = width - 60.0,
                    card_border = card_border,
                ));

                let card_count = slide.kpi_cards.len().max(1);
                let available_w = width - 120.0;
                let gap = 20.0;
                let card_w = (available_w - (gap * (card_count as f64 - 1.0))) / card_count as f64;
                let card_h = 240.0;
                let card_y = 150.0;

                for (idx, card) in slide.kpi_cards.iter().enumerate() {
                    let card_x = 60.0 + (idx as f64 * (card_w + gap));
                    let badge_color = if card.positive { "#22c55e" } else { "#ef4444" };
                    let badge_text = if card.positive { "▲ Positif" } else { "▼ Perlu Perhatian" };

                    svg.push_str(&format!(
                        r#"  <!-- KPI Card {idx} -->
  <rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{card_bg}" stroke="{card_border}" stroke-width="1.5" rx="10" filter="url(#cardShadow)" />
  <text x="{pad_x}" y="{label_y}" font-size="13" font-weight="600" fill="{sub_color}">{label}</text>
  <text x="{pad_x}" y="{val_y}" font-size="36" font-weight="900" fill="{title_color}">{val}</text>
  <rect x="{pad_x}" y="{badge_y}" width="80" height="20" fill="{badge_color}" fill-opacity="0.15" rx="4" />
  <text x="{badge_text_x}" y="{badge_text_y}" font-size="10" font-weight="700" fill="{badge_color}">{badge_text}</text>
  <text x="{pad_x}" y="{subtext_y}" font-size="12" font-weight="400" fill="{text_color}">{subtext}</text>
"#,
                        idx = idx,
                        x = card_x,
                        y = card_y,
                        w = card_w,
                        h = card_h,
                        card_bg = card_bg,
                        card_border = card_border,
                        pad_x = card_x + 20.0,
                        label_y = card_y + 40.0,
                        sub_color = sub_color,
                        label = escape_xml(&card.label),
                        val_y = card_y + 95.0,
                        title_color = title_color,
                        val = escape_xml(&card.value),
                        badge_y = card_y + 120.0,
                        badge_color = badge_color,
                        badge_text_x = card_x + 26.0,
                        badge_text_y = card_y + 134.0,
                        badge_text = badge_text,
                        subtext_y = card_y + 175.0,
                        text_color = text_color,
                        subtext = escape_xml(&card.subtext),
                    ));
                }
            }
            "two-column" => {
                // Layout Dua Kolom
                svg.push_str(&format!(
                    r#"  <!-- Header -->
  <text x="60" y="70" font-size="26" font-weight="800" fill="{title_color}">{title}</text>
  <text x="60" y="98" font-size="15" font-weight="400" fill="{sub_color}">{subtitle}</text>
  <line x1="60" y1="115" x2="{width_pad}" y2="115" stroke="{card_border}" stroke-width="1" />
"#,
                    title_color = title_color,
                    title = escape_xml(&slide.title),
                    sub_color = sub_color,
                    subtitle = escape_xml(slide.subtitle.as_deref().unwrap_or("")),
                    width_pad = width - 60.0,
                    card_border = card_border,
                ));

                let col_w = (width - 150.0) / 2.0;
                let col_y = 145.0;
                let col_h = height - col_y - 60.0;

                // Kolom Kiri
                svg.push_str(&format!(
                    r#"  <!-- Kolom Kiri -->
  <rect x="60" y="{y}" width="{w}" height="{h}" fill="{card_bg}" stroke="{card_border}" stroke-width="1" rx="8" />
  <text x="85" y="{head_y}" font-size="16" font-weight="700" fill="{accent_color}">{lt}</text>
"#,
                    y = col_y,
                    w = col_w,
                    h = col_h,
                    card_bg = card_bg,
                    card_border = card_border,
                    head_y = col_y + 35.0,
                    accent_color = accent_color,
                    lt = escape_xml(slide.left_title.as_deref().unwrap_or("Kolom Kiri")),
                ));

                for (idx, b) in slide.left_bullets.iter().enumerate() {
                    let by = col_y + 70.0 + (idx as f64 * 32.0);
                    svg.push_str(&format!(
                        r#"  <circle cx="85" cy="{cy}" r="3.5" fill="{accent_color}" />
  <text x="98" y="{by}" font-size="13" fill="{text_color}">{text}</text>
"#,
                        cy = by - 4.0,
                        accent_color = accent_color,
                        by = by,
                        text_color = text_color,
                        text = escape_xml(b),
                    ));
                }

                // Kolom Kanan
                let right_x = 60.0 + col_w + 30.0;
                svg.push_str(&format!(
                    r#"  <!-- Kolom Kanan -->
  <rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{card_bg}" stroke="{card_border}" stroke-width="1" rx="8" />
  <text x="{head_x}" y="{head_y}" font-size="16" font-weight="700" fill="{accent_color}">{rt}</text>
"#,
                    x = right_x,
                    y = col_y,
                    w = col_w,
                    h = col_h,
                    card_bg = card_bg,
                    card_border = card_border,
                    head_x = right_x + 25.0,
                    head_y = col_y + 35.0,
                    accent_color = accent_color,
                    rt = escape_xml(slide.right_title.as_deref().unwrap_or("Kolom Kanan")),
                ));

                for (idx, b) in slide.right_bullets.iter().enumerate() {
                    let by = col_y + 70.0 + (idx as f64 * 32.0);
                    svg.push_str(&format!(
                        r#"  <circle cx="{cx}" cy="{cy}" r="3.5" fill="{accent_color}" />
  <text x="{tx}" y="{by}" font-size="13" fill="{text_color}">{text}</text>
"#,
                        cx = right_x + 25.0,
                        cy = by - 4.0,
                        accent_color = accent_color,
                        tx = right_x + 38.0,
                        by = by,
                        text_color = text_color,
                        text = escape_xml(b),
                    ));
                }
            }
            _ => {
                // Layout Konten Standar (Title & Bullets)
                svg.push_str(&format!(
                    r#"  <!-- Header -->
  <text x="60" y="70" font-size="26" font-weight="800" fill="{title_color}">{title}</text>
  <text x="60" y="98" font-size="15" font-weight="400" fill="{sub_color}">{subtitle}</text>
  <line x1="60" y1="115" x2="{width_pad}" y2="115" stroke="{card_border}" stroke-width="1" />
"#,
                    title_color = title_color,
                    title = escape_xml(&slide.title),
                    sub_color = sub_color,
                    subtitle = escape_xml(slide.subtitle.as_deref().unwrap_or("")),
                    width_pad = width - 60.0,
                    card_border = card_border,
                ));

                let card_w = width - 120.0;
                let card_y = 145.0;
                let card_h = height - card_y - 60.0;

                svg.push_str(&format!(
                    r#"  <rect x="60" y="{y}" width="{w}" height="{h}" fill="{card_bg}" stroke="{card_border}" stroke-width="1" rx="8" />
"#,
                    y = card_y,
                    w = card_w,
                    h = card_h,
                    card_bg = card_bg,
                    card_border = card_border,
                ));

                for (idx, b) in slide.bullets.iter().enumerate() {
                    let by = card_y + 45.0 + (idx as f64 * 40.0);
                    svg.push_str(&format!(
                        r#"  <circle cx="95" cy="{cy}" r="4" fill="{accent_color}" />
  <text x="115" y="{by}" font-size="15" fill="{text_color}">{text}</text>
"#,
                        cy = by - 5.0,
                        accent_color = accent_color,
                        by = by,
                        text_color = text_color,
                        text = escape_xml(b),
                    ));
                }
            }
        }

        // Footer Nomor Slide & Branding
        svg.push_str(&format!(
            r#"  <!-- Footer -->
  <text x="60" y="{footer_y}" font-size="11" fill="{sub_color}">WidyaOffice Suite • Slide {curr} / {total}</text>
  <text x="{foot_right_x}" y="{footer_y}" font-size="11" text-anchor="end" fill="{sub_color}">Kedaulatan Digital Indonesia</text>
</svg>
"#,
            footer_y = height - 25.0,
            sub_color = sub_color,
            curr = slide_idx + 1,
            total = self.slides.len(),
            foot_right_x = width - 60.0,
        ));

        Ok(svg)
    }
}

// ----------------------------------------------------
// Helper Parser & String Manipulators
// ----------------------------------------------------

fn extract_quoted_string(line: &str) -> Option<String> {
    let start = line.find('"')?;
    let end = line[start + 1..].find('"')?;
    Some(line[start + 1..start + 1 + end].to_string())
}

fn extract_field_str(content: &str, field_name: &str) -> Option<String> {
    let pattern = format!("{}:", field_name);
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&pattern) {
            return extract_quoted_string(trimmed);
        }
    }
    None
}

fn extract_string_array(content: &str, field_name: &str) -> Vec<String> {
    let pattern = format!("{}:", field_name);
    let mut results = Vec::new();

    let mut in_array = false;
    let mut accumulated = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&pattern) {
            in_array = true;
            accumulated.push_str(trimmed);
            if trimmed.contains(']') {
                break;
            }
            continue;
        }

        if in_array {
            accumulated.push(' ');
            accumulated.push_str(trimmed);
            if trimmed.contains(']') {
                break;
            }
        }
    }

    if let (Some(start), Some(end)) = (accumulated.find('['), accumulated.find(']')) {
        let inside = &accumulated[start + 1..end];
        let mut cur = String::new();
        let mut in_quotes = false;
        for ch in inside.chars() {
            if ch == '"' {
                if in_quotes {
                    results.push(cur.clone());
                    cur.clear();
                    in_quotes = false;
                } else {
                    in_quotes = true;
                }
            } else if in_quotes {
                cur.push(ch);
            }
        }
    }

    results
}

fn extract_kpi_cards(content: &str) -> Vec<DslKpiCard> {
    let mut cards = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut inside_cards = false;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("kartu:") {
            inside_cards = true;
            continue;
        }
        if inside_cards && trimmed.starts_with(']') {
            break;
        }

        if inside_cards && (trimmed.contains('{') && trimmed.contains('}')) {
            let label = extract_field_str_inline(trimmed, "label").unwrap_or_else(|| "Metrik".to_string());
            let value = extract_field_str_inline(trimmed, "nilai").unwrap_or_else(|| "0".to_string());
            let subtext = extract_field_str_inline(trimmed, "subteks").unwrap_or_default();
            let positive = !trimmed.contains("positif: salah") && !trimmed.contains("positif: false");

            cards.push(DslKpiCard {
                label,
                value,
                subtext,
                positive,
            });
        }
    }

    cards
}

fn extract_field_str_inline(chunk: &str, field_name: &str) -> Option<String> {
    let pattern = format!("{}:", field_name);
    let start_key = chunk.find(&pattern)?;
    let after_key = &chunk[start_key + pattern.len()..];
    let start_quote = after_key.find('"')?;
    let end_quote = after_key[start_quote + 1..].find('"')?;
    Some(after_key[start_quote + 1..start_quote + 1 + end_quote].to_string())
}

fn split_slide_blocks(code: &str) -> Vec<(String, String)> {
    let mut blocks = Vec::new();
    let keywords = ["slide_judul(", "slide_konten(", "slide_kpi(", "slide_dua_kolom("];

    let mut remaining = code;
    while let Some((kw, idx)) = find_next_keyword(remaining, &keywords) {
        let after_kw = &remaining[idx + kw.len()..];
        // Temukan penutup kurung pemanggilan ')'
        if let Some(close_idx) = find_matching_paren(after_kw) {
            let block_content = &after_kw[..close_idx];
            let kind = kw.trim_end_matches('(').to_string();
            blocks.push((kind, block_content.to_string()));
            remaining = &after_kw[close_idx + 1..];
        } else {
            break;
        }
    }

    blocks
}

fn find_next_keyword(s: &str, keywords: &[&'static str]) -> Option<(&'static str, usize)> {
    let mut min_pos: Option<(&'static str, usize)> = None;
    for &kw in keywords {
        if let Some(pos) = s.find(kw) {
            match min_pos {
                None => min_pos = Some((kw, pos)),
                Some((_, cur_min)) if pos < cur_min => min_pos = Some((kw, pos)),
                _ => {}
            }
        }
    }
    min_pos
}

fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    let mut in_str = false;
    let mut prev_char = ' ';

    for (idx, ch) in s.char_indices() {
        if ch == '"' && prev_char != '\\' {
            in_str = !in_str;
        } else if !in_str {
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
                if depth == 0 {
                    return Some(idx);
                }
            }
        }
        prev_char = ch;
    }

    None
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
