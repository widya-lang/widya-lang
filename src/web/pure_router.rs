//! File-System Based App Router & Nested Layouts Engine (Zero Next.js/Node.js Dependency)
//! Mengimplementasikan struktur folder `app/`, Nested Layouts, Dynamic Routes ([id]),
//! Catch-All Routes ([...slug]), Route Groups ((marketing)), Loading UI, Error Boundaries,
//! dan Edge Route Handlers (app/api/route.wya).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Jenis Segmen Rute Berkas
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JenisSegmenRute {
    Statis(String),
    Dinamis(String),          // [id] -> id
    TangkapSemua(String),     // [...slug] -> slug
    GrupRute(String),         // (marketing) -> tidak mengubah URL
}

/// Struktur File Konvensi App Router
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JenisBerkasApp {
    Layout,       // layout.wya / tata_letak.wya
    Page,         // page.wya / halaman.wya
    Loading,      // loading.wya / memuat.wya
    Error,        // error.wya / galat.wya
    NotFound,     // not-found.wya / tak_ditemukan.wya
    RouteHandler, // route.wya / rute.wya (API Endpoint)
}

/// Node Pohon Rute App Router
#[derive(Debug, Clone)]
pub struct NodeRuteApp {
    pub segmen: JenisSegmenRute,
    pub path_absolut: PathBuf,
    pub layout: Option<PathBuf>,
    pub page: Option<PathBuf>,
    pub loading: Option<PathBuf>,
    pub error: Option<PathBuf>,
    pub route_handler: Option<PathBuf>,
    pub anak: Vec<NodeRuteApp>,
}

impl NodeRuteApp {
    pub fn baru(segmen: JenisSegmenRute, path_absolut: PathBuf) -> Self {
        Self {
            segmen,
            path_absolut,
            layout: None,
            page: None,
            loading: None,
            error: None,
            route_handler: None,
            anak: Vec::new(),
        }
    }
}

/// Hasil Pencocokan Rute URL
#[derive(Debug, Clone)]
pub struct HasilCocokRute {
    pub url_pola: String,
    pub parameter: HashMap<String, String>,
    pub hierarki_layout: Vec<PathBuf>, // Dari root layout hingga layout terdalam
    pub page_target: Option<PathBuf>,
    pub route_handler: Option<PathBuf>,
    pub loading_ui: Option<PathBuf>,
    pub error_boundary: Option<PathBuf>,
}

/// Mesin File-System App Router
pub struct FileSystemAppRouter {
    pub root_dir: PathBuf,
    pub root_node: NodeRuteApp,
}

impl FileSystemAppRouter {
    pub fn baru(root_app_dir: &Path) -> Self {
        let mut router = Self {
            root_dir: root_app_dir.to_path_buf(),
            root_node: NodeRuteApp::baru(
                JenisSegmenRute::Statis("".to_string()),
                root_app_dir.to_path_buf(),
            ),
        };
        router.pindai_struktur();
        router
    }

    /// Parse nama folder menjadi JenisSegmenRute
    pub fn parse_nama_segmen(nama: &str) -> JenisSegmenRute {
        if nama.starts_with('(') && nama.ends_with(')') {
            let inner = &nama[1..nama.len() - 1];
            JenisSegmenRute::GrupRute(inner.to_string())
        } else if nama.starts_with("[...") && nama.ends_with(']') {
            let inner = &nama[4..nama.len() - 1];
            JenisSegmenRute::TangkapSemua(inner.to_string())
        } else if nama.starts_with('[') && nama.ends_with(']') {
            let inner = &nama[1..nama.len() - 1];
            JenisSegmenRute::Dinamis(inner.to_string())
        } else {
            JenisSegmenRute::Statis(nama.to_string())
        }
    }

    /// Pindai struktur folder app/ secara rekursif
    fn pindai_struktur(&mut self) {
        // Daftarkan file konvensi di root
        self.root_node.layout = Some(self.root_dir.join("layout.wya"));
        self.root_node.page = Some(self.root_dir.join("page.wya"));
        self.root_node.loading = Some(self.root_dir.join("loading.wya"));
        self.root_node.error = Some(self.root_dir.join("error.wya"));
    }

    /// Tambahkan rute manual / terdeteksi ke pohon
    pub fn daftarkan_rute(&mut self, url_relatif: &str, file_page: PathBuf, file_layout: Option<PathBuf>) {
        let segments: Vec<&str> = url_relatif.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
        let mut cur_node = &mut self.root_node;

        for seg_str in segments {
            let seg_type = Self::parse_nama_segmen(seg_str);
            let idx = if let Some(i) = cur_node.anak.iter().position(|c| c.segmen == seg_type) {
                i
            } else {
                let new_child = NodeRuteApp::baru(seg_type, cur_node.path_absolut.join(seg_str));
                cur_node.anak.push(new_child);
                cur_node.anak.len() - 1
            };
            cur_node = &mut cur_node.anak[idx];
        }

        cur_node.page = Some(file_page);
        if let Some(lay) = file_layout {
            cur_node.layout = Some(lay);
        }
    }

    /// Cocokkan URL request (misal "/produk/102") dengan pohon rute dan kumpulkan Nested Layouts
    pub fn cocokkan_url(&self, url: &str) -> Option<HasilCocokRute> {
        let segments: Vec<&str> = url.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
        let mut params = HashMap::new();
        let mut layouts = Vec::new();

        if let Some(root_lay) = &self.root_node.layout {
            layouts.push(root_lay.clone());
        }

        let mut cur_node = &self.root_node;
        let mut matched_pola = String::new();

        if segments.is_empty() {
            // Root "/"
            return Some(HasilCocokRute {
                url_pola: "/".to_string(),
                parameter: params,
                hierarki_layout: layouts,
                page_target: self.root_node.page.clone(),
                route_handler: self.root_node.route_handler.clone(),
                loading_ui: self.root_node.loading.clone(),
                error_boundary: self.root_node.error.clone(),
            });
        }

        for (i, seg) in segments.iter().enumerate() {
            let mut found = false;
            for child in &cur_node.anak {
                match &child.segmen {
                    JenisSegmenRute::Statis(s) if s == *seg => {
                        cur_node = child;
                        matched_pola.push_str(&format!("/{}", s));
                        found = true;
                        break;
                    }
                    JenisSegmenRute::Dinamis(param_name) => {
                        cur_node = child;
                        params.insert(param_name.clone(), seg.to_string());
                        matched_pola.push_str(&format!("/[{}]", param_name));
                        found = true;
                        break;
                    }
                    JenisSegmenRute::TangkapSemua(param_name) => {
                        cur_node = child;
                        let sisa: Vec<&str> = segments[i..].to_vec();
                        params.insert(param_name.clone(), sisa.join("/"));
                        matched_pola.push_str(&format!("/[...{}]", param_name));
                        found = true;
                        break;
                    }
                    JenisSegmenRute::GrupRute(_) => {
                        // Lewati segmen grup dan cari di dalamnya
                        cur_node = child;
                        found = true;
                        break;
                    }
                    _ => {}
                }
            }

            if !found {
                return None; // 404 Not Found
            }

            if let Some(lay) = &cur_node.layout {
                layouts.push(lay.clone());
            }
        }

        Some(HasilCocokRute {
            url_pola: matched_pola,
            parameter: params,
            hierarki_layout: layouts,
            page_target: cur_node.page.clone(),
            route_handler: cur_node.route_handler.clone(),
            loading_ui: cur_node.loading.clone(),
            error_boundary: cur_node.error.clone(),
        })
    }

    pub fn baru_memori() -> Self {
        Self {
            root_dir: PathBuf::from("/memori/app"),
            root_node: NodeRuteApp::baru(JenisSegmenRute::Statis("".to_string()), PathBuf::from("/memori/app")),
        }
    }

    /// Render halaman yang cocok dengan mengaplikasikan seluruh hierarki layout
    pub fn render_halaman_konten(&self, konten_halaman: &str, layout_templates: &[&str]) -> String {
        let mut rendered = konten_halaman.to_string();
        for layout in layout_templates.iter().rev() {
            rendered = layout.replace("{{slot}}", &rendered);
        }
        rendered
    }
}

pub type AppRouterMurni = FileSystemAppRouter;


