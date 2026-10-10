/// Mesin Isolasi Gaya CSS (Scoped CSS Engine)
pub struct ScopedCSSEngine;
pub type CssScopedEngine = ScopedCSSEngine;

impl ScopedCSSEngine {

    /// Hitung hash unik untuk nama file komponen (misal: "w-scope-a8f9c1")
    pub fn buat_scope_id(nama_komponen: &str) -> String {
        let mut hash: u32 = 5381;
        for b in nama_komponen.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u32);
        }
        format!("w-scope-{:x}", hash)
    }

    /// Transformasi CSS dan return (css_termodifikasi, scope_id)
    pub fn transformasi_scoped(css_mentah: &str, nama_komponen: &str) -> (String, String) {
        let scope_id = Self::buat_scope_id(nama_komponen);
        let scoped_css = Self::terapkan_scope_css(css_mentah, &scope_id);
        (scoped_css, scope_id)
    }

    /// Terapkan atribut scope pada selector CSS
    pub fn terapkan_scope_css(css_mentah: &str, scope_id: &str) -> String {
        let mut hasil = String::new();
        for baris in css_mentah.lines() {
            let trim = baris.trim();
            if trim.contains('{') && !trim.starts_with('@') {
                let parts: Vec<&str> = trim.split('{').collect();
                let selectors: Vec<String> = parts[0]
                    .split(',')
                    .map(|s| format!("{}[{}]", s.trim(), scope_id))
                    .collect();
                hasil.push_str(&format!("{} {{\n", selectors.join(", ")));
            } else if !trim.is_empty() {
                hasil.push_str(baris);
                hasil.push('\n');
            }
        }
        hasil
    }
}

/// Mesin Minifikasi Aset Web Murni
pub struct WebMinifier;
pub type MinifierWebMurni = WebMinifier;

impl WebMinifier {
    /// Minifikasi kode HTML
    pub fn minifikasi_html(html: &str) -> String {
        let mut out = String::with_capacity(html.len());
        let mut prev_is_ws = false;

        for c in html.chars() {
            if c == '<' || c == '>' {
                prev_is_ws = false;
                out.push(c);
            } else if c.is_whitespace() {
                if !prev_is_ws {
                    out.push(' ');
                    prev_is_ws = true;
                }
            } else {
                prev_is_ws = false;
                out.push(c);
            }
        }
        out.trim().to_string()
    }

    pub fn minify_html(html: &str) -> String {
        Self::minifikasi_html(html)
    }

    /// Minifikasi kode CSS
    pub fn minifikasi_css(css: &str) -> String {
        css.replace('\n', "")
            .replace('\r', "")
            .replace("  ", " ")
            .replace(": ", ":")
            .replace(" {", "{")
            .replace("{ ", "{")
            .replace("; ", ";")
            .replace(" }", "}")
    }

    pub fn minify_css(css: &str) -> String {
        Self::minifikasi_css(css)
    }
}

