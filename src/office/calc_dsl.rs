//! Widya-Lang Spreadsheet DSL & Universal Calculation Engine (WidyaCalc)
//! Sovereign GridCraft-grade Spreadsheet Model for OpenXML XLSX, CSV, and Interactive SVG Charts.

use crate::office::xlsx::{
    col_name_to_index, format_idr_number, index_to_col_name, parse_cell_reference, CellFormat,
    CellValue, Worksheet, XlsxWorkbook,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definisi Grafik Spreadsheet Interaktif (Interactive SVG Chart)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslChart {
    pub kind: String,        // "batang" (bar), "garis" (line), "lingkaran" (pie)
    pub title: String,
    pub label_range: String, // e.g. "A2:A6"
    pub data_range: String,  // e.g. "B2:B6"
    pub color_palette: Vec<String>,
}

impl Default for DslChart {
    fn default() -> Self {
        Self {
            kind: "batang".to_string(),
            title: "Grafik Data WidyaCalc".to_string(),
            label_range: "A2:A6".to_string(),
            data_range: "B2:B6".to_string(),
            color_palette: vec![
                "#185abd".to_string(),
                "#107c41".to_string(),
                "#d83b01".to_string(),
                "#8764b8".to_string(),
                "#008272".to_string(),
            ],
        }
    }
}

/// Definisi Lembar Kerja Spreadsheet (Sheet)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslSheet {
    pub name: String,
    pub cells: HashMap<String, String>, // coord -> raw_value (formula or text)
    pub formats: HashMap<String, CellFormat>,
    pub max_row: u32,
    pub max_col: u32,
}

impl DslSheet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cells: HashMap::new(),
            formats: HashMap::new(),
            max_row: 30,
            max_col: 12,
        }
    }

    pub fn set_cell(&mut self, coord: &str, raw_val: &str) {
        let upper_coord = coord.to_uppercase();
        if let Ok((r, c)) = parse_cell_reference(&upper_coord) {
            if r > self.max_row {
                self.max_row = r;
            }
            if c > self.max_col {
                self.max_col = c;
            }
        }
        self.cells.insert(upper_coord, raw_val.to_string());
    }

    pub fn set_format(&mut self, coord: &str, fmt: CellFormat) {
        self.formats.insert(coord.to_uppercase(), fmt);
    }
}

/// Model Buku Kerja Spreadsheet Berdaulat (WidyaWorkbookDsl / GridCraft Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaWorkbookDsl {
    pub title: String,
    pub author: String,
    pub sheets: Vec<DslSheet>,
    pub charts: Vec<DslChart>,
}

impl Default for WidyaWorkbookDsl {
    fn default() -> Self {
        Self::new("BukuKerja_WidyaCalc")
    }
}

impl WidyaWorkbookDsl {
    pub fn new(title: &str) -> Self {
        let mut default_sheet = DslSheet::new("Sheet1");
        default_sheet.name = "Ringkasan Keuangan".to_string();
        Self {
            title: title.to_string(),
            author: "Widya Sovereign Calc Engine".to_string(),
            sheets: vec![default_sheet],
            charts: Vec::new(),
        }
    }

    pub fn get_sheet_mut(&mut self, name: &str) -> Option<&mut DslSheet> {
        self.sheets.iter_mut().find(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn add_sheet(&mut self, name: &str) -> &mut DslSheet {
        let sheet = DslSheet::new(name);
        self.sheets.push(sheet);
        self.sheets.last_mut().unwrap()
    }

    pub fn add_chart(&mut self, chart: DslChart) {
        self.charts.push(chart);
    }

    /// Parser dari Skrip Kode Bahasa Widya-Lang (`.widya` / `.wcalc`)
    pub fn from_widya_dsl(code: &str) -> Result<Self, String> {
        let mut wb = Self::new("BukuKerja_Widya");
        let lines: Vec<&str> = code.lines().collect();
        let mut i = 0;
        let mut current_sheet_idx = 0usize;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with("//") {
                i += 1;
                continue;
            }

            if line.starts_with("judul:") {
                let val = extract_quoted_value(line);
                if !val.is_empty() {
                    wb.title = val;
                }
            } else if line.starts_with("penulis:") {
                let val = extract_quoted_value(line);
                if !val.is_empty() {
                    wb.author = val;
                }
            } else if line.starts_with("lembar(") {
                let sheet_name = extract_arg_between(line, "lembar(", ")");
                if !sheet_name.is_empty() {
                    if wb.sheets.len() == 1 && wb.sheets[0].cells.is_empty() {
                        wb.sheets[0].name = sheet_name;
                        current_sheet_idx = 0;
                    } else {
                        wb.add_sheet(&sheet_name);
                        current_sheet_idx = wb.sheets.len() - 1;
                    }
                }
            } else if line.starts_with("baris_header(") || line.starts_with("header(") {
                let headers = extract_array_strings(line, "");
                let sheet = &mut wb.sheets[current_sheet_idx];
                for (col_idx, h) in headers.iter().enumerate() {
                    let col_letter = index_to_col_name((col_idx + 1) as u32);
                    let coord = format!("{}1", col_letter);
                    sheet.set_cell(&coord, h);
                    let mut fmt = CellFormat::default();
                    fmt.bold = true;
                    fmt.background_hex = Some("#f1f5f9".to_string());
                    sheet.set_format(&coord, fmt);
                }
            } else if line.starts_with("baris(") {
                let values = extract_array_strings(line, "");
                let sheet = &mut wb.sheets[current_sheet_idx];
                let next_row = sheet.max_row.max(1) + 1;
                for (col_idx, val) in values.iter().enumerate() {
                    let col_letter = index_to_col_name((col_idx + 1) as u32);
                    let coord = format!("{}{}", col_letter, next_row);
                    sheet.set_cell(&coord, val);
                }
            } else if line.starts_with("sel(") {
                // Sintaks eksplisit: sel("B4", "45000000") atau sel("F4", "=SUM(B4:E4)")
                let parts = extract_two_args(line, "sel(", ")");
                if parts.len() >= 2 {
                    let sheet = &mut wb.sheets[current_sheet_idx];
                    sheet.set_cell(&parts[0], &parts[1]);
                }
            } else if line.starts_with("grafik(") {
                let mut kind = "batang".to_string();
                let mut title = "Visualisasi Data".to_string();
                let mut label_range = "A2:A5".to_string();
                let mut data_range = "B2:B5".to_string();

                let mut j = i;
                while j < lines.len() {
                    let subline = lines[j].trim();
                    if subline.contains("jenis:") {
                        let k = extract_quoted_value(subline);
                        if !k.is_empty() {
                            kind = k;
                        }
                    } else if subline.contains("judul:") {
                        let t = extract_quoted_value(subline);
                        if !t.is_empty() {
                            title = t;
                        }
                    } else if subline.contains("rentang_label:") || subline.contains("label_range:") {
                        let lr = extract_quoted_value(subline);
                        if !lr.is_empty() {
                            label_range = lr;
                        }
                    } else if subline.contains("rentang_data:") || subline.contains("data_range:") {
                        let dr = extract_quoted_value(subline);
                        if !dr.is_empty() {
                            data_range = dr;
                        }
                    } else if (subline.starts_with('}') || subline.starts_with(')')) && j > i {
                        break;
                    }
                    j += 1;
                }
                i = j;

                wb.add_chart(DslChart {
                    kind,
                    title,
                    label_range,
                    data_range,
                    color_palette: DslChart::default().color_palette,
                });
            }

            i += 1;
        }

        Ok(wb)
    }

    /// Konversi Dokumen DSL ke Buku Kerja Excel Murni (`XlsxWorkbook`)
    pub fn to_xlsx(&self) -> XlsxWorkbook {
        let mut wb = XlsxWorkbook::new(&self.title);

        for (idx, dsl_sheet) in self.sheets.iter().enumerate() {
            let ws = if idx == 0 {
                let first = wb.sheets.get_mut(0).unwrap();
                first.name = dsl_sheet.name.clone();
                first
            } else {
                wb.add_sheet(&dsl_sheet.name)
            };

            for (coord, raw_val) in &dsl_sheet.cells {
                let clean_val = raw_val.trim();
                if clean_val.starts_with('=') {
                    let _ = ws.set_cell_by_name(coord, CellValue::Formula(clean_val[1..].to_string()));
                } else if let Ok(num) = clean_val.parse::<f64>() {
                    let _ = ws.set_cell_by_name(coord, CellValue::Number(num));
                } else if clean_val.eq_ignore_ascii_case("true") || clean_val.eq_ignore_ascii_case("benar") {
                    let _ = ws.set_cell_by_name(coord, CellValue::Bool(true));
                } else if clean_val.eq_ignore_ascii_case("false") || clean_val.eq_ignore_ascii_case("salah") {
                    let _ = ws.set_cell_by_name(coord, CellValue::Bool(false));
                } else {
                    let _ = ws.set_cell_by_name(coord, CellValue::Text(clean_val.to_string()));
                }
            }

            for (coord, fmt) in &dsl_sheet.formats {
                if let Ok((r, c)) = parse_cell_reference(coord) {
                    ws.set_cell_format(r, c, fmt.clone());
                }
            }
        }

        wb
    }

    /// Serialisasi Status Lembar Kerja ke Objek JSON untuk Antarmuka WidyaCalc Desktop
    pub fn to_grid_json(&self) -> serde_json::Value {
        let mut sheets_obj = serde_json::Map::new();

        for (idx, s) in self.sheets.iter().enumerate() {
            let key = format!("Sheet{}", idx + 1);
            let sheet_val = serde_json::json!({
                "name": s.name,
                "cells": s.cells,
                "formats": s.formats,
                "maxRow": s.max_row.max(30),
                "maxCol": s.max_col.max(12),
            });
            sheets_obj.insert(key, sheet_val);
        }

        serde_json::json!({
            "activeSheet": "Sheet1",
            "sheets": sheets_obj,
            "charts": self.charts,
            "title": self.title,
            "author": self.author,
        })
    }

    /// Ekspor Lembar Tertentu ke Teks CSV
    pub fn to_csv(&self, sheet_name: &str) -> String {
        let sheet = self.sheets.iter().find(|s| s.name.eq_ignore_ascii_case(sheet_name))
            .or_else(|| self.sheets.first());

        let s = match sheet {
            Some(s) => s,
            None => return String::new(),
        };

        let mut lines = Vec::new();
        for r in 1..=s.max_row {
            let mut row_cells = Vec::new();
            for c in 1..=s.max_col {
                let coord = format!("{}{}", index_to_col_name(c), r);
                let val = s.cells.get(&coord).cloned().unwrap_or_default();
                if val.contains(',') || val.contains('"') || val.contains('\n') {
                    row_cells.push(format!("\"{}\"", val.replace('"', "\"\"")));
                } else {
                    row_cells.push(val);
                }
            }
            lines.push(row_cells.join(","));
        }

        lines.join("\n")
    }

    /// Buat Grafik Vektor SVG Interaktif Murni dari Definisi Grafik
    pub fn generate_svg_chart(&self, chart_idx: usize) -> Result<String, String> {
        let chart = self.charts.get(chart_idx)
            .ok_or_else(|| format!("Grafik indeks {} tidak ditemukan", chart_idx))?;

        let first_sheet = self.sheets.first().ok_or_else(|| "Buku kerja tidak memiliki lembar data".to_string())?;

        // Ekstraksi data & label dari sel
        let labels = extract_range_values_from_sheet(first_sheet, &chart.label_range);
        let raw_values = extract_range_values_from_sheet(first_sheet, &chart.data_range);

        let numeric_values: Vec<f64> = raw_values
            .iter()
            .map(|s| s.parse::<f64>().unwrap_or(0.0))
            .collect();

        if labels.is_empty() || numeric_values.is_empty() {
            return Err("Rentang data atau label kosong untuk grafik".to_string());
        }

        let max_val = numeric_values.iter().cloned().fold(f64::NAN, f64::max).max(1.0);
        let width = 640.0f64;
        let height = 360.0f64;
        let pad_top = 50.0;
        let pad_bottom = 60.0;
        let pad_left = 70.0;
        let pad_right = 40.0;
        let chart_w = width - pad_left - pad_right;
        let chart_h = height - pad_top - pad_bottom;

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" style=\"width:100%; height:auto; background:#ffffff; border-radius:6px; box-shadow:0 2px 8px rgba(0,0,0,0.08); font-family:var(--font-ui, sans-serif);\">\n",
            width, height
        );

        // Judul Grafik
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"30\" text-anchor=\"middle\" font-size=\"16\" font-weight=\"bold\" fill=\"#0f172a\">{}</text>\n",
            width / 2.0, html_escape(&chart.title)
        ));

        match chart.kind.as_str() {
            "lingkaran" | "pie" => {
                // Diagram Lingkaran (Pie Chart)
                let cx = width / 2.0;
                let cy = (height + 20.0) / 2.0;
                let radius = 110.0;
                let total_sum: f64 = numeric_values.iter().sum::<f64>().max(1.0);

                let mut current_angle = 0.0f64;
                for (idx, &val) in numeric_values.iter().enumerate() {
                    let slice_angle = (val / total_sum) * 2.0 * std::f64::consts::PI;
                    let x1 = cx + radius * current_angle.cos();
                    let y1 = cy + radius * current_angle.sin();
                    let next_angle = current_angle + slice_angle;
                    let x2 = cx + radius * next_angle.cos();
                    let y2 = cy + radius * next_angle.sin();

                    let large_arc = if slice_angle > std::f64::consts::PI { 1 } else { 0 };
                    let color = chart.color_palette.get(idx % chart.color_palette.len()).unwrap();

                    svg.push_str(&format!(
                        "  <path d=\"M {},{} L {},{} A {},{} 0 {},1 {},{} Z\" fill=\"{}\" stroke=\"#ffffff\" stroke-width=\"2\" />\n",
                        cx, cy, x1, y1, radius, radius, large_arc, x2, y2, color
                    ));

                    current_angle = next_angle;
                }
            }
            "garis" | "line" => {
                // Diagram Garis (Line Chart)
                let count = numeric_values.len();
                let step_x = if count > 1 { chart_w / (count - 1) as f64 } else { chart_w };

                let mut points = Vec::new();
                for (idx, &val) in numeric_values.iter().enumerate() {
                    let px = pad_left + idx as f64 * step_x;
                    let py = pad_top + chart_h - (val / max_val * chart_h);
                    points.push((px, py));
                }

                // Grid Horizontal Lines
                for step in 0..=4 {
                    let gy = pad_top + (step as f64 * chart_h / 4.0);
                    let val_step = max_val * (4 - step) as f64 / 4.0;
                    svg.push_str(&format!(
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#e2e8f0\" stroke-dasharray=\"3,3\" />\n",
                        pad_left, gy, pad_left + chart_w, gy
                    ));
                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"11\" fill=\"#64748b\">{:.0}</text>\n",
                        pad_left - 8.0, gy + 4.0, val_step
                    ));
                }

                // Polyline
                let poly_pts: Vec<String> = points.iter().map(|(x, y)| format!("{:.1},{:.1}", x, y)).collect();
                svg.push_str(&format!(
                    "  <polyline points=\"{}\" fill=\"none\" stroke=\"#185abd\" stroke-width=\"3\" stroke-linecap=\"round\" />\n",
                    poly_pts.join(" ")
                ));

                // Circles & Labels
                for (idx, (px, py)) in points.iter().enumerate() {
                    svg.push_str(&format!(
                        "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"#185abd\" stroke=\"#ffffff\" stroke-width=\"2\" />\n",
                        px, py
                    ));
                    if let Some(lbl) = labels.get(idx) {
                        svg.push_str(&format!(
                            "  <text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" font-size=\"11\" fill=\"#334155\">{}</text>\n",
                            px, height - 25.0, html_escape(lbl)
                        ));
                    }
                }
            }
            _ => {
                // Diagram Batang (Bar Chart)
                let count = numeric_values.len();
                let bar_slot = chart_w / count as f64;
                let bar_width = (bar_slot * 0.65).min(50.0);

                // Grid Horizontal Lines
                for step in 0..=4 {
                    let gy = pad_top + (step as f64 * chart_h / 4.0);
                    let val_step = max_val * (4 - step) as f64 / 4.0;
                    svg.push_str(&format!(
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#e2e8f0\" stroke-dasharray=\"3,3\" />\n",
                        pad_left, gy, pad_left + chart_w, gy
                    ));
                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"11\" fill=\"#64748b\">{:.0}</text>\n",
                        pad_left - 8.0, gy + 4.0, val_step
                    ));
                }

                for (idx, &val) in numeric_values.iter().enumerate() {
                    let bar_h = (val / max_val * chart_h).max(2.0);
                    let bx = pad_left + idx as f64 * bar_slot + (bar_slot - bar_width) / 2.0;
                    let by = pad_top + chart_h - bar_h;
                    let color = chart.color_palette.get(idx % chart.color_palette.len()).unwrap();

                    svg.push_str(&format!(
                        "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" rx=\"3\" />\n",
                        bx, by, bar_width, bar_h, color
                    ));

                    // Label Nilai di Atas Bar
                    svg.push_str(&format!(
                        "  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"10\" font-weight=\"600\" fill=\"#1e293b\">{:.0}</text>\n",
                        bx + bar_width / 2.0, by - 6.0, val
                    ));

                    // Label Kategori di Bawah Sumbu X
                    if let Some(lbl) = labels.get(idx) {
                        svg.push_str(&format!(
                            "  <text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" font-size=\"11\" fill=\"#334155\">{}</text>\n",
                            bx + bar_width / 2.0, height - 25.0, html_escape(lbl)
                        ));
                    }
                }
            }
        }

        svg.push_str("</svg>\n");
        Ok(svg)
    }

    /// Ekspor Kembali ke Kode Sintaks Murni Widya-Lang DSL (`.widya` / `.wcalc`)
    pub fn to_widya_dsl_code(&self) -> String {
        let mut code = String::new();
        code.push_str("// Widya-Lang Sovereign Spreadsheet DSL\n");
        code.push_str("buku_kerja {\n");
        code.push_str(&format!("    judul: \"{}\",\n", self.title));
        code.push_str(&format!("    penulis: \"{}\",\n\n", self.author));

        for s in &self.sheets {
            code.push_str(&format!("    lembar(\"{}\") {{\n", s.name));
            for r in 1..=s.max_row {
                let mut row_vals = Vec::new();
                let mut has_val = false;
                for c in 1..=s.max_col {
                    let coord = format!("{}{}", index_to_col_name(c), r);
                    if let Some(v) = s.cells.get(&coord) {
                        row_vals.push(v.clone());
                        if !v.is_empty() {
                            has_val = true;
                        }
                    } else {
                        row_vals.push(String::new());
                    }
                }
                if has_val {
                    code.push_str(&format!("        baris({:?})\n", row_vals));
                }
            }
            code.push_str("    }\n\n");
        }

        for ch in &self.charts {
            code.push_str("    grafik(\n");
            code.push_str(&format!("        jenis: \"{}\",\n", ch.kind));
            code.push_str(&format!("        judul: \"{}\",\n", ch.title));
            code.push_str(&format!("        rentang_label: \"{}\",\n", ch.label_range));
            code.push_str(&format!("        rentang_data: \"{}\"\n", ch.data_range));
            code.push_str("    )\n\n");
        }

        code.push_str("}\n");
        code
    }
}

// -----------------------------------------------------------------------------
// Helper Fungsi
// -----------------------------------------------------------------------------

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn extract_quoted_value(line: &str) -> String {
    if let Some(first_quote) = line.find('"') {
        if let Some(second_quote) = line[first_quote + 1..].find('"') {
            return line[first_quote + 1..first_quote + 1 + second_quote].to_string();
        }
    }
    String::new()
}

fn extract_arg_between(line: &str, start_token: &str, end_token: &str) -> String {
    if let Some(pos) = line.find(start_token) {
        let rest = &line[pos + start_token.len()..];
        if let Some(end_pos) = rest.rfind(end_token) {
            let val = rest[..end_pos].trim();
            return val.trim_matches(|c| c == '"' || c == '\'').to_string();
        }
    }
    String::new()
}

fn extract_two_args(line: &str, start_token: &str, end_token: &str) -> Vec<String> {
    let inner = extract_arg_between(line, start_token, end_token);
    inner
        .split(',')
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\'').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn extract_array_strings(line: &str, key: &str) -> Vec<String> {
    let search_area = if key.is_empty() {
        line
    } else if let Some(p) = line.find(key) {
        &line[p + key.len()..]
    } else {
        return Vec::new();
    };

    if let (Some(b_open), Some(b_close)) = (search_area.find('['), search_area.rfind(']')) {
        let content = &search_area[b_open + 1..b_close];
        return content
            .split(',')
            .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\'' || c == ' ').to_string())
            .collect();
    }
    Vec::new()
}

fn extract_range_values_from_sheet(sheet: &DslSheet, range_str: &str) -> Vec<String> {
    let mut results = Vec::new();
    let parts: Vec<&str> = range_str.split(':').collect();
    if parts.len() != 2 {
        return results;
    }

    let p1 = match parse_cell_reference(parts[0].trim()) {
        Ok(p) => p,
        Err(_) => return results,
    };
    let p2 = match parse_cell_reference(parts[1].trim()) {
        Ok(p) => p,
        Err(_) => return results,
    };

    let min_col = p1.1.min(p2.1);
    let max_col = p1.1.max(p2.1);
    let min_row = p1.0.min(p2.0);
    let max_row = p1.0.max(p2.0);

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let coord = format!("{}{}", index_to_col_name(c), r);
            let val = sheet.cells.get(&coord).cloned().unwrap_or_default();
            results.push(val);
        }
    }

    results
}
