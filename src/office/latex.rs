use serde::{Deserialize, Serialize};

/// Elemen Dokumen TeX / LaTeX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatexElement {
    Section(String),
    Subsection(String),
    Paragraph(String),
    Equation(String), // e.g. E = mc^2 atau \int_{0}^{\infty} f(x) dx
    InlineMath(String),
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Itemize(Vec<String>),
}

/// Dokumen LaTeX / TeX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatexDocument {
    pub documentclass: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub elements: Vec<LatexElement>,
}

impl LatexDocument {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            documentclass: "article".to_string(),
            title: title.to_string(),
            author: author.to_string(),
            date: "\\today".to_string(),
            elements: Vec::new(),
        }
    }

    pub fn add_section(&mut self, title: &str) {
        self.elements.push(LatexElement::Section(title.to_string()));
    }

    pub fn add_subsection(&mut self, title: &str) {
        self.elements.push(LatexElement::Subsection(title.to_string()));
    }

    pub fn add_paragraph(&mut self, text: &str) {
        self.elements.push(LatexElement::Paragraph(text.to_string()));
    }

    pub fn add_equation(&mut self, math_tex: &str) {
        self.elements.push(LatexElement::Equation(math_tex.to_string()));
    }

    pub fn add_table(&mut self, headers: Vec<String>, rows: Vec<Vec<String>>) {
        self.elements.push(LatexElement::Table { headers, rows });
    }

    /// Hasilkan Kode Sumber LaTeX Asli (.tex)
    pub fn generate_tex_source(&self) -> String {
        let mut tex = String::new();
        tex.push_str(&format!("\\documentclass{{{}}}\n", self.documentclass));
        tex.push_str("\\usepackage[utf8]{inputenc}\n\\usepackage{amsmath,amssymb}\n\\usepackage{graphicx}\n\n");
        tex.push_str(&format!("\\title{{{}}}\n", self.title));
        tex.push_str(&format!("\\author{{{}}}\n", self.author));
        tex.push_str(&format!("\\date{{{}}}\n\n", self.date));
        tex.push_str("\\begin{document}\n\\maketitle\n\n");

        for el in &self.elements {
            match el {
                LatexElement::Section(s) => tex.push_str(&format!("\\section{{{}}}\n\n", s)),
                LatexElement::Subsection(s) => tex.push_str(&format!("\\subsection{{{}}}\n\n", s)),
                LatexElement::Paragraph(p) => tex.push_str(&format!("{}\n\n", p)),
                LatexElement::Equation(eq) => {
                    tex.push_str("\\begin{equation}\n");
                    tex.push_str(&format!("  {}\n", eq));
                    tex.push_str("\\end{equation}\n\n");
                }
                LatexElement::InlineMath(m) => tex.push_str(&format!("${}$", m)),
                LatexElement::Table { headers, rows } => {
                    let col_spec = "c|".repeat(headers.len());
                    tex.push_str(&format!("\\begin{{tabular}}{{|{}}}\n\\hline\n", col_spec));
                    tex.push_str(&format!("{} \\\\ \\hline\n", headers.join(" & ")));
                    for r in rows {
                        tex.push_str(&format!("{} \\\\ \\hline\n", r.join(" & ")));
                    }
                    tex.push_str("\\end{tabular}\n\n");
                }
                LatexElement::Itemize(items) => {
                    tex.push_str("\\begin{itemize}\n");
                    for it in items {
                        tex.push_str(&format!("  \\item {}\n", it));
                    }
                    tex.push_str("\\end{itemize}\n\n");
                }
            }
        }

        tex.push_str("\\end{document}\n");
        tex
    }

    /// Render ke Pratinjau Web Math KaTeX/MathJax Interaktif
    pub fn render_html_math_preview(&self) -> String {
        let mut html = String::from("<!DOCTYPE html><html><head><meta charset='utf-8'>");
        html.push_str(&format!("<title>{} - LaTeX Preview</title>", self.title));
        html.push_str(r#"
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.css">
        <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.js"></script>
        <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/contrib/auto-render.min.js" onload="renderMathInElement(document.body);"></script>
        <style>
            body { font-family: 'Latin Modern Roman', 'Times New Roman', serif; background: #fafafa; color: #111; margin: 0; padding: 40px; display: flex; justify-content: center; }
            .paper { background: white; width: 800px; min-height: 1000px; padding: 70px; box-shadow: 0 4px 15px rgba(0,0,0,0.08); border-radius: 4px; }
            h1.title { text-align: center; font-size: 2rem; margin-bottom: 8px; }
            p.meta { text-align: center; color: #555; margin-bottom: 40px; font-style: italic; }
            h2 { font-size: 1.4rem; border-bottom: 1px solid #ddd; padding-bottom: 6px; margin-top: 30px; }
            .eq-box { text-align: center; margin: 20px 0; font-size: 1.2rem; }
            table { width: 100%; border-collapse: collapse; margin: 20px 0; }
            th, td { border: 1px solid #333; padding: 8px 12px; text-align: center; }
            th { background: #eee; }
        </style></head><body>"#);

        html.push_str("<div class='paper'>");
        html.push_str(&format!("<h1 class='title'>{}</h1>", self.title));
        html.push_str(&format!("<p class='meta'>Penulis: {} | Tanggal: {}</p>", self.author, self.date));

        for el in &self.elements {
            match el {
                LatexElement::Section(s) => html.push_str(&format!("<h2>{}</h2>", s)),
                LatexElement::Subsection(s) => html.push_str(&format!("<h3>{}</h3>", s)),
                LatexElement::Paragraph(p) => html.push_str(&format!("<p>{}</p>", p)),
                LatexElement::Equation(eq) => {
                    html.push_str(&format!("<div class='eq-box'>$${}$$</div>", eq));
                }
                LatexElement::InlineMath(m) => html.push_str(&format!("${}$", m)),
                LatexElement::Table { headers, rows } => {
                    html.push_str("<table><tr>");
                    for h in headers { html.push_str(&format!("<th>{}</th>", h)); }
                    html.push_str("</tr>");
                    for r in rows {
                        html.push_str("<tr>");
                        for cell in r { html.push_str(&format!("<td>{}</td>", cell)); }
                        html.push_str("</tr>");
                    }
                    html.push_str("</table>");
                }
                LatexElement::Itemize(items) => {
                    html.push_str("<ul>");
                    for it in items { html.push_str(&format!("<li>{}</li>", it)); }
                    html.push_str("</ul>");
                }
            }
        }

        html.push_str("</div></body></html>");
        html
    }
}
