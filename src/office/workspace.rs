use serde::{Deserialize, Serialize};
use super::widyadrive::WidyaDrive;

/// Modul Aplikasi di dalam Widya Workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkspaceApp {
    Docs,
    Sheets,
    Slides,
    Drive,
    LatexEditor,
    PdfStudio,
}

/// Ruang Kerja Terpadu Enterprise: Widya Workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaWorkspace {
    pub workspace_name: String,
    pub organization: String,
    pub drive: WidyaDrive,
    pub active_members: Vec<String>,
}

impl WidyaWorkspace {
    pub fn new(workspace_name: &str, organization: &str) -> Self {
        let mut drive = WidyaDrive::new(&format!("{}_Drive", workspace_name));
        // Buat berkas percontohan
        let _ = drive.upload_file("Proposal_Proyek.docx", b"Konten Dokumen Word Enterprise", "admin");
        let _ = drive.upload_file("Anggaran_Q1.xlsx", b"Kategori,Jumlah\nServer,50000000\nLisensi,25000000", "admin");
        let _ = drive.upload_file("Slide_PitchDeck.pptx", b"Presentasi Eksekutif Seri A", "admin");
        let _ = drive.upload_file("Makalah_Riset.tex", b"\\documentclass{article}\n\\title{Riset AI}", "admin");

        Self {
            workspace_name: workspace_name.to_string(),
            organization: organization.to_string(),
            drive,
            active_members: vec!["CEO Widya".to_string(), "Lead Architect".to_string(), "Developer".to_string()],
        }
    }

    /// Render Web Dashboard Widya Workspace Terpadu (Dark Glassmorphism UI)
    pub fn render_workspace_dashboard(&self) -> String {
        let mut html = String::from("<!DOCTYPE html><html><head><meta charset='utf-8'>");
        html.push_str(&format!("<title>{} - Widya Workspace Suite</title>", self.workspace_name));
        html.push_str(r#"<style>
            * { box-sizing: border-box; }
            body { font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif; background: #0b0f19; color: #f3f4f6; margin: 0; display: flex; height: 100vh; overflow: hidden; }
            .sidebar { width: 280px; background: rgba(17, 24, 39, 0.95); backdrop-filter: blur(12px); border-right: 1px solid rgba(255,255,255,0.08); padding: 24px; display: flex; flex-direction: column; }
            .brand { display: flex; align-items: center; gap: 12px; margin-bottom: 36px; font-size: 1.25rem; font-weight: 700; color: #38bdf8; }
            .app-btn { display: flex; align-items: center; gap: 14px; padding: 12px 16px; border-radius: 10px; color: #9ca3af; text-decoration: none; margin-bottom: 8px; font-weight: 500; transition: all 0.2s; }
            .app-btn:hover, .app-btn.active { background: rgba(56, 189, 248, 0.15); color: #38bdf8; }
            .main-content { flex: 1; padding: 36px; overflow-y: auto; display: flex; flex-direction: column; gap: 32px; }
            .hero-banner { background: linear-gradient(135deg, rgba(14, 165, 233, 0.2) 0%, rgba(99, 102, 241, 0.2) 100%); border: 1px solid rgba(56, 189, 248, 0.3); border-radius: 16px; padding: 32px 40px; }
            .quick-create { display: grid; grid-template-columns: repeat(4, 1fr); gap: 20px; }
            .create-card { background: #111827; border: 1px solid rgba(255,255,255,0.08); border-radius: 14px; padding: 24px; text-align: center; cursor: pointer; transition: all 0.2s; }
            .create-card:hover { transform: translateY(-4px); border-color: #38bdf8; box-shadow: 0 10px 25px -5px rgba(56, 189, 248, 0.2); }
            .card-icon { font-size: 2.5rem; margin-bottom: 12px; }
            .recent-section { background: #111827; border: 1px solid rgba(255,255,255,0.08); border-radius: 16px; padding: 24px; }
            .file-row { display: flex; justify-content: space-between; align-items: center; padding: 14px 16px; border-bottom: 1px solid rgba(255,255,255,0.05); }
            .file-row:last-child { border-bottom: none; }
            .badge-sovereign { background: rgba(16, 185, 129, 0.2); color: #34d399; padding: 4px 12px; border-radius: 20px; font-size: 0.8rem; font-weight: 600; }
        </style></head><body>"#);

        html.push_str("<div class='sidebar'>");
        html.push_str(&format!("<div class='brand'>🏢 {}</div>", self.workspace_name));
        html.push_str("<a class='app-btn active' href='#'>⚡ Dashboard Utama</a>");
        html.push_str("<a class='app-btn' href='#'>📄 WidyaDocs (.docx / .md)</a>");
        html.push_str("<a class='app-btn' href='#'>📊 WidyaSheets (.xlsx)</a>");
        html.push_str("<a class='app-btn' href='#'>📽️ WidyaSlides (.pptx)</a>");
        html.push_str("<a class='app-btn' href='#'>📑 WidyaPDF Studio</a>");
        html.push_str("<a class='app-btn' href='#'>📐 LaTeX Math Studio</a>");
        html.push_str("<a class='app-btn' href='#'>☁️ WidyaDrive Cloud</a>");

        html.push_str("<div style='margin-top:auto; padding-top:20px; border-top:1px solid rgba(255,255,255,0.08);'>");
        html.push_str(&format!("<div style='font-size:0.85rem; color:#9ca3af;'>Organisasi: <b>{}</b></div>", self.organization));
        html.push_str("<div style='margin-top:8px;'><span class='badge-sovereign'>🛡️ 100% Data Sovereign</span></div>");
        html.push_str("</div></div>");

        html.push_str("<div class='main-content'>");
        html.push_str(&format!(r#"<div class='hero-banner'>
            <h1 style='margin:0 0 8px 0; font-size:1.8rem; color:#f8fafc;'>Selamat Datang di {}</h1>
            <p style='margin:0; color:#94a3b8; font-size:1.05rem;'>Pusat produktivitas kantor mandiri untuk dokumen, spreadsheet, presentasi, dan komputasi awan berdaulat.</p>
        </div>"#, self.workspace_name));

        html.push_str("<div class='quick-create'>");
        html.push_str("<div class='create-card'><div class='card-icon'>📄</div><div style='font-weight:600;'>Dokumen Baru</div><div style='font-size:0.8rem; color:#9ca3af; margin-top:4px;'>Word / Markdown</div></div>");
        html.push_str("<div class='create-card'><div class='card-icon'>📊</div><div style='font-weight:600;'>Spreadsheet Baru</div><div style='font-size:0.8rem; color:#9ca3af; margin-top:4px;'>Excel dengan Rumus</div></div>");
        html.push_str("<div class='create-card'><div class='card-icon'>📽️</div><div style='font-weight:600;'>Slide Presentasi</div><div style='font-size:0.8rem; color:#9ca3af; margin-top:4px;'>PowerPoint Deck</div></div>");
        html.push_str("<div class='create-card'><div class='card-icon'>📐</div><div style='font-weight:600;'>Makalah TeX / PDF</div><div style='font-size:0.8rem; color:#9ca3af; margin-top:4px;'>Rumus Matematika</div></div>");
        html.push_str("</div>");

        html.push_str("<div class='recent-section'>");
        html.push_str("<h3 style='margin:0 0 16px 0;'>📁 Berkas Terkini di WidyaDrive</h3>");
        for f in self.drive.files.values() {
            let icon = if f.name.ends_with(".docx") { "📄" }
                else if f.name.ends_with(".xlsx") { "📊" }
                else if f.name.ends_with(".pptx") { "📽️" }
                else if f.name.ends_with(".tex") { "📐" }
                else { "📑" };

            html.push_str(&format!(r#"<div class='file-row'>
                <div style='display:flex; align-items:center; gap:12px;'>
                    <span style='font-size:1.5rem;'>{}</span>
                    <div>
                        <div style='font-weight:600;'>{}</div>
                        <div style='font-size:0.8rem; color:#9ca3af;'>Dimiliki oleh: {} • v{}</div>
                    </div>
                </div>
                <div style='display:flex; gap:10px;'>
                    <button style='background:rgba(56,189,248,0.2); color:#38bdf8; border:none; padding:6px 14px; border-radius:6px; font-weight:bold; cursor:pointer;'>Buka</button>
                </div>
            </div>"#, icon, f.name, f.owner, f.version));
        }
        html.push_str("</div></div></body></html>");
        html
    }
}
