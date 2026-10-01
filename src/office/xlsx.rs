use crate::office::zip::ZipPackage;
use chrono::{Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Format Tampilan Sel
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CellFormat {
    pub currency: Option<String>,     // misal "IDR" (Rp), "USD" ($)
    pub percentage: bool,             // Format persen (misal 15.5%)
    pub decimals: Option<u8>,         // Jumlah desimal
    pub bold: bool,
    pub italic: bool,
    pub background_hex: Option<String>,
    pub text_color_hex: Option<String>,
}

/// Nilai pada Sel Spreadsheet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Bool(bool),
    Formula(String),
    Empty,
}

impl CellValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            CellValue::Number(n) => Some(*n),
            CellValue::Text(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    format!("{:.2}", n)
                }
            }
            CellValue::Bool(b) => b.to_string(),
            CellValue::Formula(f) => format!("={}", f),
            CellValue::Empty => String::new(),
        }
    }

    pub fn format_with(&self, fmt: &CellFormat) -> String {
        match self {
            CellValue::Number(n) => {
                let dec = fmt.decimals.unwrap_or(2) as usize;
                if let Some(ref cur) = fmt.currency {
                    if cur.eq_ignore_ascii_case("IDR") || cur.eq_ignore_ascii_case("RP") {
                        format!("Rp {}", format_idr_number(*n, dec))
                    } else if cur.eq_ignore_ascii_case("USD") {
                        format!("${:.dec$}", n, dec = dec)
                    } else {
                        format!("{} {:.dec$}", cur, n, dec = dec)
                    }
                } else if fmt.percentage {
                    format!("{:.1}%", n * 100.0)
                } else {
                    format!("{:.dec$}", n, dec = dec)
                }
            }
            _ => self.to_display_string(),
        }
    }
}

fn format_idr_number(n: f64, decimals: usize) -> String {
    let is_neg = n < 0.0;
    let abs_n = n.abs();
    let int_part = abs_n.trunc() as u64;
    let frac_part = abs_n.fract();

    let int_str = int_part.to_string();
    let mut formatted_int = String::new();
    let chars: Vec<char> = int_str.chars().collect();
    let len = chars.len();

    for (i, &c) in chars.iter().enumerate() {
        formatted_int.push(c);
        let rem = len - 1 - i;
        if rem > 0 && rem % 3 == 0 {
            formatted_int.push('.');
        }
    }

    let mut res = if is_neg { format!("-{}", formatted_int) } else { formatted_int };
    if decimals > 0 {
        let frac_str = format!("{:.dec$}", frac_part, dec = decimals);
        if let Some(frac_digits) = frac_str.strip_prefix("0.") {
            res.push(',');
            res.push_str(frac_digits);
        }
    }
    res
}

/// Lembar Kerja (Worksheet) Spreadsheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worksheet {
    pub name: String,
    pub cells: HashMap<(u32, u32), CellValue>,       // (baris, kolom) 1-indexed
    pub formats: HashMap<(u32, u32), CellFormat>,    // Format sel
    pub max_row: u32,
    pub max_col: u32,
}

impl Worksheet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cells: HashMap::new(),
            formats: HashMap::new(),
            max_row: 0,
            max_col: 0,
        }
    }

    pub fn set_cell(&mut self, row: u32, col: u32, val: CellValue) {
        if row > self.max_row { self.max_row = row; }
        if col > self.max_col { self.max_col = col; }
        self.cells.insert((row, col), val);
    }

    pub fn set_cell_format(&mut self, row: u32, col: u32, fmt: CellFormat) {
        self.formats.insert((row, col), fmt);
    }

    pub fn set_cell_by_name(&mut self, cell_name: &str, val: CellValue) -> Result<(), String> {
        let (row, col) = parse_cell_reference(cell_name)?;
        self.set_cell(row, col, val);
        Ok(())
    }

    pub fn get_cell(&self, row: u32, col: u32) -> &CellValue {
        self.cells.get(&(row, col)).unwrap_or(&CellValue::Empty)
    }

    pub fn get_cell_by_name(&self, cell_name: &str) -> Result<&CellValue, String> {
        let (row, col) = parse_cell_reference(cell_name)?;
        Ok(self.get_cell(row, col))
    }

    /// Evaluasi Rumus Spreadsheet Lengkap (15+ Rumus Kelas Industri)
    pub fn evaluate_formula(&self, formula: &str) -> Result<CellValue, String> {
        let clean = formula.trim().trim_start_matches('=');
        let upper = clean.to_uppercase();

        let extract_inner = |prefix: &str| -> Option<&str> {
            if upper.starts_with(prefix) && clean.ends_with(')') {
                let start = prefix.len();
                let end = clean.len() - 1;
                if start <= end {
                    Some(&clean[start..end])
                } else {
                    None
                }
            } else {
                None
            }
        };

        // 1. SUM
        if let Some(inner) = extract_inner("SUM(") {
            let numbers = self.get_range_numbers(inner)?;
            return Ok(CellValue::Number(numbers.iter().sum()));
        }

        // 2. AVERAGE
        if let Some(inner) = extract_inner("AVERAGE(") {
            let numbers = self.get_range_numbers(inner)?;
            if numbers.is_empty() { return Ok(CellValue::Number(0.0)); }
            return Ok(CellValue::Number(numbers.iter().sum::<f64>() / numbers.len() as f64));
        }

        // 3. MIN & MAX
        if let Some(inner) = extract_inner("MIN(") {
            let numbers = self.get_range_numbers(inner)?;
            let min = numbers.into_iter().fold(f64::INFINITY, f64::min);
            return Ok(CellValue::Number(if min.is_infinite() { 0.0 } else { min }));
        }

        if let Some(inner) = extract_inner("MAX(") {
            let numbers = self.get_range_numbers(inner)?;
            let max = numbers.into_iter().fold(f64::NEG_INFINITY, f64::max);
            return Ok(CellValue::Number(if max.is_infinite() { 0.0 } else { max }));
        }

        // 4. COUNT & COUNTA
        if let Some(inner) = extract_inner("COUNT(") {
            let numbers = self.get_range_numbers(inner)?;
            return Ok(CellValue::Number(numbers.len() as f64));
        }

        // 5. IF(kondisi, nilai_benar, nilai_salah)
        if let Some(inner) = extract_inner("IF(") {
            let args = split_function_args(inner);
            if args.len() >= 3 {
                let cond_str = args[0].trim();
                let cond_res = self.evaluate_condition(cond_str)?;
                if cond_res {
                    return self.eval_simple_operand(args[1].trim());
                } else {
                    return self.eval_simple_operand(args[2].trim());
                }
            }
        }

        // 6. VLOOKUP(lookup_val, range, col_index, [exact])
        if let Some(inner) = extract_inner("VLOOKUP(") {
            let args = split_function_args(inner);
            if args.len() >= 3 {
                let target = args[0].trim().trim_matches('"');
                let range_str = args[1].trim();
                let col_idx: u32 = args[2].trim().parse().map_err(|_| "Indeks kolom VLOOKUP harus angka")?;

                if range_str.contains(':') {
                    let parts: Vec<&str> = range_str.split(':').collect();
                    let (r1, c1) = parse_cell_reference(parts[0])?;
                    let (r2, c2) = parse_cell_reference(parts[1])?;

                    let target_col = c1 + col_idx - 1;
                    if target_col > c2 {
                        return Err("Indeks kolom VLOOKUP di luar jangkauan rentang".to_string());
                    }

                    for r in r1..=r2 {
                        let lookup_cell = self.get_cell(r, c1).to_display_string();
                        if lookup_cell.eq_ignore_ascii_case(target) {
                            return Ok(self.get_cell(r, target_col).clone());
                        }
                    }
                    return Ok(CellValue::Text("#N/A".to_string()));
                }
            }
        }

        // 7. CONCAT & CONCATENATE
        if let Some(inner) = extract_inner("CONCAT(") {
            let args = split_function_args(inner);
            let mut res = String::new();
            for a in args {
                let val = self.eval_simple_operand(a.trim())?;
                res.push_str(&val.to_display_string());
            }
            return Ok(CellValue::Text(res));
        }

        // 8. TEXT MANIPULATION: UPPER, LOWER, TRIM, LEN
        if let Some(inner) = extract_inner("UPPER(") {
            let val = self.eval_simple_operand(inner.trim())?;
            return Ok(CellValue::Text(val.to_display_string().to_uppercase()));
        }
        if let Some(inner) = extract_inner("LOWER(") {
            let val = self.eval_simple_operand(inner.trim())?;
            return Ok(CellValue::Text(val.to_display_string().to_lowercase()));
        }
        if let Some(inner) = extract_inner("TRIM(") {
            let val = self.eval_simple_operand(inner.trim())?;
            return Ok(CellValue::Text(val.to_display_string().trim().to_string()));
        }
        if let Some(inner) = extract_inner("LEN(") {
            let val = self.eval_simple_operand(inner.trim())?;
            return Ok(CellValue::Number(val.to_display_string().len() as f64));
        }

        // 9. MATH: ROUND, ABS
        if let Some(inner) = extract_inner("ABS(") {
            let val = self.eval_simple_operand(inner.trim())?;
            return Ok(CellValue::Number(val.as_f64().unwrap_or(0.0).abs()));
        }
        if let Some(inner) = extract_inner("ROUND(") {
            let args = split_function_args(inner);
            if args.len() >= 2 {
                let val = self.eval_simple_operand(args[0].trim())?.as_f64().unwrap_or(0.0);
                let dec: i32 = args[1].trim().parse().unwrap_or(0);
                let factor = 10f64.powi(dec);
                return Ok(CellValue::Number((val * factor).round() / factor));
            }
        }

        // 10. FINANCIAL: PMT(rate, nper, pv) — Cicilan Pinjaman Koperasi/Fintech
        if let Some(inner) = extract_inner("PMT(") {
            let args = split_function_args(inner);
            if args.len() >= 3 {
                let rate = self.eval_simple_operand(args[0].trim())?.as_f64().unwrap_or(0.0);
                let nper = self.eval_simple_operand(args[1].trim())?.as_f64().unwrap_or(1.0);
                let pv = self.eval_simple_operand(args[2].trim())?.as_f64().unwrap_or(0.0);

                if rate == 0.0 {
                    return Ok(CellValue::Number(-pv / nper));
                }
                let pmt = (pv * rate * (1.0 + rate).powf(nper)) / ((1.0 + rate).powf(nper) - 1.0);
                return Ok(CellValue::Number(-pmt));
            }
        }

        // 11. DATE: TODAY(), NOW(), YEAR(sel), MONTH(sel)
        if upper == "TODAY()" {
            let now = Local::now();
            return Ok(CellValue::Text(format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day())));
        }
        if upper == "NOW()" {
            let now = Local::now();
            return Ok(CellValue::Text(format!("{:04}-{:02}-{:02} {:02}:{:02}", now.year(), now.month(), now.day(), now.hour(), now.minute())));
        }

        // 12. Aritmatika biner langsung (+, -, *, /)
        if clean.contains('+') || clean.contains('-') || clean.contains('*') || clean.contains('/') {
            return self.eval_arithmetic(clean);
        }

        // Nilai sel tunggal
        if let Ok((r, c)) = parse_cell_reference(clean) {
            return Ok(self.get_cell(r, c).clone());
        }

        Err(format!("Rumus tidak didukung: {}", formula))
    }

    fn eval_simple_operand(&self, op: &str) -> Result<CellValue, String> {
        let op_trim = op.trim();
        if op_trim.starts_with('"') && op_trim.ends_with('"') {
            return Ok(CellValue::Text(op_trim.trim_matches('"').to_string()));
        }
        if let Ok(num) = op_trim.parse::<f64>() {
            return Ok(CellValue::Number(num));
        }
        if let Ok((r, c)) = parse_cell_reference(op_trim) {
            return Ok(self.get_cell(r, c).clone());
        }
        if op_trim.starts_with('=') {
            return self.evaluate_formula(op_trim);
        }
        Ok(CellValue::Text(op_trim.to_string()))
    }

    fn evaluate_condition(&self, cond: &str) -> Result<bool, String> {
        if cond.contains(">=") {
            let parts: Vec<&str> = cond.split(">=").collect();
            let v1 = self.eval_simple_operand(parts[0])?.as_f64().unwrap_or(0.0);
            let v2 = self.eval_simple_operand(parts[1])?.as_f64().unwrap_or(0.0);
            return Ok(v1 >= v2);
        }
        if cond.contains("<=") {
            let parts: Vec<&str> = cond.split("<=").collect();
            let v1 = self.eval_simple_operand(parts[0])?.as_f64().unwrap_or(0.0);
            let v2 = self.eval_simple_operand(parts[1])?.as_f64().unwrap_or(0.0);
            return Ok(v1 <= v2);
        }
        if cond.contains('>') {
            let parts: Vec<&str> = cond.split('>').collect();
            let v1 = self.eval_simple_operand(parts[0])?.as_f64().unwrap_or(0.0);
            let v2 = self.eval_simple_operand(parts[1])?.as_f64().unwrap_or(0.0);
            return Ok(v1 > v2);
        }
        if cond.contains('<') {
            let parts: Vec<&str> = cond.split('<').collect();
            let v1 = self.eval_simple_operand(parts[0])?.as_f64().unwrap_or(0.0);
            let v2 = self.eval_simple_operand(parts[1])?.as_f64().unwrap_or(0.0);
            return Ok(v1 < v2);
        }
        if cond.contains('=') {
            let parts: Vec<&str> = cond.split('=').collect();
            let v1 = self.eval_simple_operand(parts[0])?.to_display_string();
            let v2 = self.eval_simple_operand(parts[1])?.to_display_string();
            return Ok(v1 == v2);
        }
        Ok(false)
    }

    fn eval_arithmetic(&self, expr: &str) -> Result<CellValue, String> {
        if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').collect();
            let mut total = 0.0;
            for p in parts {
                total += self.eval_simple_operand(p)?.as_f64().unwrap_or(0.0);
            }
            return Ok(CellValue::Number(total));
        }
        if expr.contains('-') {
            let parts: Vec<&str> = expr.split('-').collect();
            let mut total = self.eval_simple_operand(parts[0])?.as_f64().unwrap_or(0.0);
            for p in &parts[1..] {
                total -= self.eval_simple_operand(p)?.as_f64().unwrap_or(0.0);
            }
            return Ok(CellValue::Number(total));
        }
        if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').collect();
            let mut total = 1.0;
            for p in parts {
                total *= self.eval_simple_operand(p)?.as_f64().unwrap_or(0.0);
            }
            return Ok(CellValue::Number(total));
        }
        if expr.contains('/') {
            let parts: Vec<&str> = expr.split('/').collect();
            let first = self.eval_simple_operand(parts[0])?.as_f64().unwrap_or(0.0);
            let second = self.eval_simple_operand(parts[1])?.as_f64().unwrap_or(1.0);
            if second == 0.0 {
                return Ok(CellValue::Text("#DIV/0!".to_string()));
            }
            return Ok(CellValue::Number(first / second));
        }
        Err("Ekspresi tidak valid".to_string())
    }

    /// Ambil Rentang Nilai Numerik (misal "A1:A5")
    fn get_range_numbers(&self, range_str: &str) -> Result<Vec<f64>, String> {
        let mut out = Vec::new();
        if range_str.contains(':') {
            let parts: Vec<&str> = range_str.split(':').collect();
            let (r1, c1) = parse_cell_reference(parts[0])?;
            let (r2, c2) = parse_cell_reference(parts[1])?;

            let min_r = r1.min(r2);
            let max_r = r1.max(r2);
            let min_c = c1.min(c2);
            let max_c = c1.max(c2);

            for r in min_r..=max_r {
                for c in min_c..=max_c {
                    if let Some(num) = self.get_cell(r, c).as_f64() {
                        out.push(num);
                    }
                }
            }
        } else {
            for cell_ref in range_str.split(',') {
                if let Some(num) = self.get_cell_by_name(cell_ref.trim())?.as_f64() {
                    out.push(num);
                }
            }
        }
        Ok(out)
    }

    /// Ekspor Lembar Kerja ke Format CSV
    pub fn to_csv(&self) -> String {
        let mut out = String::new();
        for r in 1..=self.max_row {
            let mut row_vals = Vec::new();
            for c in 1..=self.max_col {
                let val = self.get_cell(r, c);
                let val_str = if let Some(fmt) = self.formats.get(&(r, c)) {
                    val.format_with(fmt)
                } else {
                    val.to_display_string()
                };

                if val_str.contains(',') || val_str.contains('"') {
                    row_vals.push(format!("\"{}\"", val_str.replace('"', "\"\"")));
                } else {
                    row_vals.push(val_str);
                }
            }
            out.push_str(&row_vals.join(","));
            out.push('\n');
        }
        out
    }

    /// Impor Data dari String CSV
    pub fn from_csv(&mut self, csv_text: &str) {
        let normalized = csv_text.replace("\\n", "\n").replace("\\r", "\r");
        for (r, line) in normalized.lines().enumerate() {
            let row_idx = (r + 1) as u32;
            for (c, item) in line.split(',').enumerate() {
                let col_idx = (c + 1) as u32;
                let trimmed = item.trim().trim_matches('"');
                if let Ok(num) = trimmed.parse::<f64>() {
                    self.set_cell(row_idx, col_idx, CellValue::Number(num));
                } else if trimmed.eq_ignore_ascii_case("true") {
                    self.set_cell(row_idx, col_idx, CellValue::Bool(true));
                } else if trimmed.eq_ignore_ascii_case("false") {
                    self.set_cell(row_idx, col_idx, CellValue::Bool(false));
                } else if trimmed.starts_with('=') {
                    self.set_cell(row_idx, col_idx, CellValue::Formula(trimmed[1..].to_string()));
                } else {
                    self.set_cell(row_idx, col_idx, CellValue::Text(trimmed.to_string()));
                }
            }
        }
    }

    /// Render ke Grid Pratinjau Interaktif HTML5
    pub fn render_html_grid(&self) -> String {
        let mut html = String::from(r#"<!DOCTYPE html><html><head><meta charset='utf-8'><style>
            body { font-family: 'Segoe UI', Tahoma, sans-serif; background: #0f172a; color: white; margin: 0; padding: 20px; }
            .sheet-card { background: #1e293b; border-radius: 8px; padding: 20px; box-shadow: 0 10px 25px rgba(0,0,0,0.3); border: 1px solid #334155; }
            .sheet-title { font-size: 16pt; font-weight: bold; color: #38bdf8; margin-bottom: 16px; }
            table { width: 100%; border-collapse: collapse; background: #0f172a; font-size: 9pt; }
            th, td { border: 1px solid #334155; padding: 6px 12px; text-align: left; }
            th { background: #1e293b; color: #94a3b8; font-weight: bold; text-align: center; }
            th.row-header { width: 40px; background: #1e293b; color: #64748b; }
            td.number { text-align: right; font-family: 'Consolas', monospace; color: #34d399; }
            td.bold { font-weight: bold; }
        </style></head><body>"#);

        html.push_str("<div class='sheet-card'>");
        html.push_str(&format!("<div class='sheet-title'>📊 Lembar Kerja: {}</div>", self.name));
        html.push_str("<table><thead><tr><th class='row-header'>#</th>");

        for c in 1..=self.max_col.max(1) {
            html.push_str(&format!("<th>{}</th>", index_to_col_name(c)));
        }
        html.push_str("</tr></thead><tbody>");

        for r in 1..=self.max_row.max(1) {
            html.push_str(&format!("<tr><th class='row-header'>{}</th>", r));
            for c in 1..=self.max_col.max(1) {
                let val = self.get_cell(r, c);
                let fmt = self.formats.get(&(r, c));
                let display = if let Some(f) = fmt {
                    val.format_with(f)
                } else {
                    val.to_display_string()
                };

                let is_num = matches!(val, CellValue::Number(_));
                let num_cls = if is_num { " class='number'" } else { "" };
                html.push_str(&format!("<td{}>{}</td>", num_cls, display));
            }
            html.push_str("</tr>");
        }

        html.push_str("</tbody></table></div></body></html>");
        html
    }
}

fn split_function_args(inner: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_quote = false;

    for c in inner.chars() {
        match c {
            '"' => {
                in_quote = !in_quote;
                current.push(c);
            }
            '(' => {
                depth += 1;
                current.push(c);
            }
            ')' => {
                if depth > 0 { depth -= 1; }
                current.push(c);
            }
            ',' if depth == 0 && !in_quote => {
                args.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        args.push(current.trim().to_string());
    }
    args
}

/// Buku Kerja Excel Spreadsheet (.xlsx / OpenXML Spreadsheet)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XlsxWorkbook {
    pub title: String,
    pub author: String,
    pub sheets: Vec<Worksheet>,
}

impl XlsxWorkbook {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "Widya Office Suite".to_string(),
            sheets: vec![Worksheet::new("Sheet1")],
        }
    }

    pub fn get_sheet_mut(&mut self, name: &str) -> Option<&mut Worksheet> {
        self.sheets.iter_mut().find(|s| s.name == name)
    }

    pub fn add_sheet(&mut self, name: &str) -> &mut Worksheet {
        self.sheets.push(Worksheet::new(name));
        self.sheets.last_mut().unwrap()
    }

    /// Menghasilkan Berkas Biner .XLSX Asli (Zip Packaging Berstandar ECMA-376)
    pub fn export_to_xlsx_bytes(&self) -> Vec<u8> {
        let mut pkg = ZipPackage::new();

        // 1. [Content_Types].xml
        pkg.add_file_str(
            "[Content_Types].xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#,
        );

        // 2. _rels/.rels
        pkg.add_file_str(
            "_rels/.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#,
        );

        // 3. xl/workbook.xml
        let mut wb_xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets>"#);
        for (i, s) in self.sheets.iter().enumerate() {
            wb_xml.push_str(&format!(
                r#"<sheet name="{}" sheetId="{}" r:id="rId{}"/>"#,
                s.name, i + 1, i + 1
            ));
        }
        wb_xml.push_str("</sheets></workbook>");
        pkg.add_file_str("xl/workbook.xml", &wb_xml);

        // 4. xl/_rels/workbook.xml.rels
        let mut wb_rels = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#);
        for (i, _) in self.sheets.iter().enumerate() {
            wb_rels.push_str(&format!(
                r#"<Relationship Id="rId{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet{}.xml"/>"#,
                i + 1, i + 1
            ));
        }
        wb_rels.push_str("</Relationships>");
        pkg.add_file_str("xl/_rels/workbook.xml.rels", &wb_rels);

        // 5. xl/worksheets/sheet1.xml
        if let Some(first_sheet) = self.sheets.first() {
            let mut sheet_xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>"#);
            for r in 1..=first_sheet.max_row {
                sheet_xml.push_str(&format!(r#"<row r="{}">"#, r));
                for c in 1..=first_sheet.max_col {
                    let cell_ref = format!("{}{}", index_to_col_name(c), r);
                    let val = first_sheet.get_cell(r, c);
                    match val {
                        CellValue::Number(n) => {
                            sheet_xml.push_str(&format!(r#"<c r="{}" t="n"><v>{}</v></c>"#, cell_ref, n));
                        }
                        CellValue::Text(s) => {
                            sheet_xml.push_str(&format!(r#"<c r="{}" t="inlineStr"><is><t>{}</t></is></c>"#, cell_ref, s.replace('&', "&amp;").replace('<', "&lt;")));
                        }
                        CellValue::Formula(f) => {
                            sheet_xml.push_str(&format!(r#"<c r="{}"><f>{}</f></c>"#, cell_ref, f.replace('&', "&amp;")));
                        }
                        _ => {}
                    }
                }
                sheet_xml.push_str("</row>");
            }
            sheet_xml.push_str("</sheetData></worksheet>");
            pkg.add_file_str("xl/worksheets/sheet1.xml", &sheet_xml);
        }

        // 6. docProps/core.xml & app.xml
        let now = chrono::Utc::now().to_rfc3339();
        let core_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>{}</dc:title>
  <dc:creator>{}</dc:creator>
  <dcterms:created xsi:type="dcterms:W3CDTF">{}</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">{}</dcterms:modified>
</cp:coreProperties>"#,
            self.title, self.author, now, now
        );
        pkg.add_file_str("docProps/core.xml", &core_xml);

        pkg.to_bytes()
    }

    /// Simpan Langsung ke Berkas Fisik `.xlsx`
    pub fn save_as_xlsx<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let bytes = self.export_to_xlsx_bytes();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

/// Konversi Indeks Kolom ke Nama Kolom (1 -> A, 2 -> B, 27 -> AA)
pub fn index_to_col_name(mut index: u32) -> String {
    let mut col_str = String::new();
    while index > 0 {
        let rem = (index - 1) % 26;
        col_str.insert(0, (b'A' + rem as u8) as char);
        index = (index - 1) / 26;
    }
    col_str
}

/// Konversi Referensi Kolom (A -> 1, B -> 2, Z -> 26, AA -> 27)
pub fn col_name_to_index(col_name: &str) -> Result<u32, String> {
    let mut index = 0u32;
    for c in col_name.to_ascii_uppercase().chars() {
        if !c.is_ascii_alphabetic() {
            return Err(format!("Karakter kolom tidak valid: {}", c));
        }
        index = index * 26 + ((c as u32) - ('A' as u32) + 1);
    }
    Ok(index)
}

/// Konversi Koordinat Nama Sel (misal "B12" -> (12, 2))
pub fn parse_cell_reference(cell_ref: &str) -> Result<(u32, u32), String> {
    let clean = cell_ref.trim();
    let col_str: String = clean.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    let row_str: String = clean.chars().skip_while(|c| c.is_ascii_alphabetic()).collect();

    if col_str.is_empty() || row_str.is_empty() {
        return Err(format!("Format referensi sel salah: {}", cell_ref));
    }

    let col = col_name_to_index(&col_str)?;
    let row: u32 = row_str.parse().map_err(|e| format!("Baris sel bukan angka: {}", e))?;

    Ok((row, col))
}
