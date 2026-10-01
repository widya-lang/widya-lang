use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hak Akses Pengguna pada WidyaDrive
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DriveRole {
    Owner,
    Editor,
    Viewer,
}

/// Metadata Berkas pada WidyaDrive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveFile {
    pub id: String,
    pub name: String,
    pub size_bytes: usize,
    pub sha256_hash: String,
    pub mime_type: String,
    pub payload_bytes: Vec<u8>,
    pub owner: String,
    pub version: u32,
    pub is_starred: bool,
}

/// Mesin Penyimpanan Cloud Sovereign: WidyaDrive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaDrive {
    pub storage_name: String,
    pub quota_max_bytes: usize,
    pub files: HashMap<String, DriveFile>,
    pub acl: HashMap<String, DriveRole>, // user_id -> role
}

impl WidyaDrive {
    pub fn new(storage_name: &str) -> Self {
        let mut acl = HashMap::new();
        acl.insert("admin".to_string(), DriveRole::Owner);

        Self {
            storage_name: storage_name.to_string(),
            quota_max_bytes: 10 * 1024 * 1024 * 1024, // 10 GB
            files: HashMap::new(),
            acl,
        }
    }

    /// Simpan Berkas ke WidyaDrive dengan Verifikasi Hash SHA-256
    pub fn upload_file(&mut self, name: &str, data: &[u8], owner: &str) -> Result<String, String> {
        use sha2::{Digest, Sha256};
        let file_id = format!("file_{:x}", self.files.len() + 1);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let sha256_hash = hex::encode(hasher.finalize()); // SHA-256 CAS check

        let current_usage: usize = self.files.values().map(|f| f.size_bytes).sum();
        if current_usage + data.len() > self.quota_max_bytes {
            return Err("Kapasitas penyimpanan WidyaDrive penuh".to_string());
        }

        let mime = if name.ends_with(".docx") {
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        } else if name.ends_with(".xlsx") {
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        } else if name.ends_with(".pptx") {
            "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        } else if name.ends_with(".pdf") {
            "application/pdf"
        } else if name.ends_with(".md") {
            "text/markdown"
        } else {
            "application/octet-stream"
        };

        let file = DriveFile {
            id: file_id.clone(),
            name: name.to_string(),
            size_bytes: data.len(),
            sha256_hash,
            mime_type: mime.to_string(),
            payload_bytes: data.to_vec(),
            owner: owner.to_string(),
            version: 1,
            is_starred: false,
        };

        self.files.insert(file_id.clone(), file);
        Ok(file_id)
    }

    /// Unduh Berkas dari WidyaDrive
    pub fn download_file(&self, file_id: &str) -> Option<&DriveFile> {
        self.files.get(file_id)
    }

    /// Total Penggunaan Penyimpanan (Bytes)
    pub fn total_used_bytes(&self) -> usize {
        self.files.values().map(|f| f.size_bytes).sum()
    }

    /// Render Antarmuka Web Explorer Cloud Storage WidyaDrive
    pub fn render_web_explorer(&self) -> String {
        let mut html = String::from("<!DOCTYPE html><html><head><meta charset='utf-8'>");
        html.push_str(&format!("<title>{} - WidyaDrive Sovereign Cloud</title>", self.storage_name));
        html.push_str(r#"<style>
            * { box-sizing: border-box; }
            body { font-family: 'Segoe UI', Tahoma, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; display: flex; height: 100vh; }
            .sidebar { width: 260px; background: #1e293b; padding: 24px; border-right: 1px solid #334155; display: flex; flex-direction: column; }
            .logo { font-size: 1.3rem; font-weight: bold; color: #38bdf8; margin-bottom: 30px; display: flex; align-items: center; gap: 10px; }
            .nav-item { padding: 12px 16px; border-radius: 8px; color: #94a3b8; text-decoration: none; margin-bottom: 6px; display: block; font-weight: 500; }
            .nav-item.active, .nav-item:hover { background: #334155; color: white; }
            .quota-box { margin-top: auto; background: #0f172a; padding: 16px; border-radius: 8px; border: 1px solid #334155; }
            .main { flex: 1; padding: 32px; overflow-y: auto; }
            .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 28px; }
            .btn-upload { background: #0284c7; color: white; border: none; padding: 10px 20px; border-radius: 8px; font-weight: bold; cursor: pointer; }
            .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 20px; }
            .file-card { background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 20px; transition: transform 0.2s; cursor: pointer; }
            .file-card:hover { transform: translateY(-4px); border-color: #38bdf8; }
            .file-icon { font-size: 2.2rem; margin-bottom: 12px; }
            .file-name { font-weight: 600; font-size: 0.95rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
            .file-meta { font-size: 0.8rem; color: #94a3b8; margin-top: 6px; }
        </style></head><body>"#);

        html.push_str("<div class='sidebar'>");
        html.push_str("<div class='logo'>☁️ WidyaDrive</div>");
        html.push_str("<a class='nav-item active' href='#'>📁 Semua Berkas</a>");
        html.push_str("<a class='nav-item' href='#'>⭐ Berbintang</a>");
        html.push_str("<a class='nav-item' href='#'>🕒 Riwayat Versi</a>");
        html.push_str("<a class='nav-item' href='#'>🔒 Keamanan Kripto</a>");

        let used_mb = self.total_used_bytes() as f64 / (1024.0 * 1024.0);
        html.push_str(&format!(r#"<div class='quota-box'>
            <div style='font-size:0.8rem; color:#94a3b8;'>Penyimpanan Sovereign</div>
            <div style='font-weight:bold; margin-top:4px;'>{:.2} MB terpakai</div>
        </div>"#, used_mb));
        html.push_str("</div>");

        html.push_str("<div class='main'>");
        html.push_str("<div class='header'>");
        html.push_str("<h2>Berkas Saya</h2>");
        html.push_str("<button class='btn-upload'>➕ Unggah Dokumen</button>");
        html.push_str("</div>");

        html.push_str("<div class='grid'>");
        for f in self.files.values() {
            let icon = if f.name.ends_with(".docx") { "📄" }
                else if f.name.ends_with(".xlsx") { "📊" }
                else if f.name.ends_with(".pptx") { "📽️" }
                else if f.name.ends_with(".pdf") { "📑" }
                else if f.name.ends_with(".md") { "📝" }
                else { "📦" };

            let kb = f.size_bytes as f64 / 1024.0;
            html.push_str(&format!(r#"<div class='file-card'>
                <div class='file-icon'>{}</div>
                <div class='file-name' title='{}'>{}</div>
                <div class='file-meta'>{:.1} KB • v{}</div>
            </div>"#, icon, f.name, f.name, kb, f.version));
        }
        html.push_str("</div></div></body></html>");
        html
    }
}
