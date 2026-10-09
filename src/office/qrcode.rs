//! Pure Native Rust QR Code Generator Engine (ISO/IEC 18004 Standard)
//! 100% Zero External Dependencies. Menghasilkan matriks QR Code berkualitas tinggi
//! untuk disematkan ke PDF, DOCX, SVG, dan Segel TTE Elektronik Resmi.

use crate::office::pdf::PdfDocument;

/// Tingkat Koreksi Kesalahan QR Code (Error Correction Level)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrEcLevel {
    L, // ~7% pemulihan
    M, // ~15% pemulihan (standar bisnis & URL)
    Q, // ~25% pemulihan
    H, // ~30% pemulihan
}

impl QrEcLevel {
    pub fn format_bits(&self) -> u8 {
        match self {
            QrEcLevel::L => 0b01,
            QrEcLevel::M => 0b00,
            QrEcLevel::Q => 0b11,
            QrEcLevel::H => 0b10,
        }
    }
}

/// Struktur Informasi Versi QR Code
#[derive(Debug, Clone, Copy)]
struct QrVersionInfo {
    version: usize,
    total_codewords: usize,
    data_codewords: usize,
    ec_codewords: usize,
    alignment_coords: &'static [usize],
}

// Konfigurasi Versi 1 s.d. 6 untuk Level M
static VERSIONS_M: &[QrVersionInfo] = &[
    QrVersionInfo { version: 1, total_codewords: 26, data_codewords: 16, ec_codewords: 10, alignment_coords: &[] },
    QrVersionInfo { version: 2, total_codewords: 44, data_codewords: 28, ec_codewords: 16, alignment_coords: &[6, 18] },
    QrVersionInfo { version: 3, total_codewords: 70, data_codewords: 44, ec_codewords: 26, alignment_coords: &[6, 22] },
    QrVersionInfo { version: 4, total_codewords: 100, data_codewords: 64, ec_codewords: 36, alignment_coords: &[6, 26] },
    QrVersionInfo { version: 5, total_codewords: 134, data_codewords: 86, ec_codewords: 48, alignment_coords: &[6, 30] },
    QrVersionInfo { version: 6, total_codewords: 172, data_codewords: 108, ec_codewords: 64, alignment_coords: &[6, 34] },
];

/// Mesin Generator QR Code Mandiri
#[derive(Debug, Clone)]
pub struct WidyaQrCode {
    pub version: usize,
    pub size: usize,
    pub ec_level: QrEcLevel,
    pub modules: Vec<Vec<bool>>,
}

impl WidyaQrCode {
    /// Membuat QR Code dari teks UTF-8 menggunakan EC Level M
    pub fn encode(text: &str) -> Result<Self, String> {
        Self::encode_with_level(text, QrEcLevel::M)
    }

    /// Membuat QR Code dengan tingkat koreksi kesalahan tertentu
    pub fn encode_with_level(text: &str, ec: QrEcLevel) -> Result<Self, String> {
        let data_bytes = text.as_bytes();
        let data_len = data_bytes.len();

        // Cari versi minimum yang mampu menampung data
        let mut chosen_info = None;
        for info in VERSIONS_M {
            // Mode byte: 4 bit header mode + 8 bit panjang karakter = 12 bit (1.5 byte).
            // Tersedia data_codewords bytes.
            let max_data = info.data_codewords.saturating_sub(2);
            if data_len <= max_data {
                chosen_info = Some(*info);
                break;
            }
        }

        let info = chosen_info.ok_or_else(|| {
            format!("Panjang data ({} byte) melebihi kapasitas maksimum QR Code Versi 6", data_len)
        })?;

        let version = info.version;
        let size = 17 + version * 4;

        // 1. Bitstream Encoding (Byte Mode: 0100)
        let mut bits = Vec::new();

        // Mode indicator (4 bit: 0100 untuk 8-bit byte mode)
        push_bits(&mut bits, 0b0100, 4);

        // Character count indicator (8 bit untuk Versi 1-9)
        push_bits(&mut bits, data_len as u32, 8);

        // Payload data bytes
        for &b in data_bytes {
            push_bits(&mut bits, b as u32, 8);
        }

        // Terminator bits (hingga 4 bit 0)
        let total_data_bits = info.data_codewords * 8;
        let term_len = (total_data_bits.saturating_sub(bits.len())).min(4);
        push_bits(&mut bits, 0, term_len);

        // Bit padding ke kelipatan byte (8 bit)
        while bits.len() % 8 != 0 {
            bits.push(false);
        }

        // Convert bitstream to data codewords
        let mut data_codewords = Vec::with_capacity(info.data_codewords);
        for chunk in bits.chunks(8) {
            let mut byte = 0u8;
            for &bit in chunk {
                byte = (byte << 1) | (if bit { 1 } else { 0 });
            }
            data_codewords.push(byte);
        }

        // Byte padding selang-seling (0xEC, 0x11) hingga kapasitas penuh
        let pad_bytes = [0xEC, 0x11];
        let mut pad_idx = 0;
        while data_codewords.len() < info.data_codewords {
            data_codewords.push(pad_bytes[pad_idx]);
            pad_idx = (pad_idx + 1) % 2;
        }

        // 2. Reed-Solomon Error Correction Code
        let ec_codewords = calculate_reed_solomon_ec(&data_codewords, info.ec_codewords);

        // Gabungkan data codewords dan EC codewords
        let mut final_codewords = data_codewords;
        final_codewords.extend_from_slice(&ec_codewords);

        // 3. Matriks QR & Masking
        let mut modules = vec![vec![false; size]; size];
        let mut is_function = vec![vec![false; size]; size];

        // Letakkan Finder Patterns (3 pojok)
        place_finder_pattern(&mut modules, &mut is_function, 0, 0, size);
        place_finder_pattern(&mut modules, &mut is_function, size - 7, 0, size);
        place_finder_pattern(&mut modules, &mut is_function, 0, size - 7, size);

        // Letakkan Timing Patterns
        for i in 8..(size - 8) {
            let is_dark = i % 2 == 0;
            // Horizontal timing pattern
            if !is_function[6][i] {
                modules[6][i] = is_dark;
                is_function[6][i] = true;
            }
            // Vertical timing pattern
            if !is_function[i][6] {
                modules[i][6] = is_dark;
                is_function[i][6] = true;
            }
        }

        // Dark Module
        modules[size - 8][8] = true;
        is_function[size - 8][8] = true;

        // Alignment Patterns jika ada
        if !info.alignment_coords.is_empty() {
            let coords = info.alignment_coords;
            for &r in coords {
                for &c in coords {
                    // Jangan tumpuk dengan finder patterns
                    if is_function[r][c] {
                        continue;
                    }
                    place_alignment_pattern(&mut modules, &mut is_function, r, c);
                }
            }
        }

        // Reserved modules untuk Format Information
        reserve_format_info_areas(&mut is_function, size);

        // 4. Masukkan Codewords ke dalam Matrix (Dua kolom zig-zag dari kanan ke kiri)
        let mask_pattern = 0; // Mask pattern 0: (row + col) % 2 == 0
        place_codewords(
            &mut modules,
            &is_function,
            &final_codewords,
            size,
            mask_pattern,
        );

        // 5. Tulis Format Information
        let format_bits = get_format_bits(ec, mask_pattern);
        write_format_info(&mut modules, size, format_bits);

        Ok(Self {
            version,
            size,
            ec_level: ec,
            modules,
        })
    }

    /// Render Matriks QR Code ke format SVG murni (tanpa ketergantungan eksternal)
    pub fn to_svg(&self, module_size_px: u32, margin_modules: u32) -> String {
        let total_size = (self.size as u32 + margin_modules * 2) * module_size_px;
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}" shape-rendering="crispEdges">"#,
            total_size, total_size, total_size, total_size
        );
        svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>");

        for (r, row) in self.modules.iter().enumerate() {
            for (c, &is_dark) in row.iter().enumerate() {
                if is_dark {
                    let x = ((c as u32) + margin_modules) * module_size_px;
                    let y = ((r as u32) + margin_modules) * module_size_px;
                    svg.push_str(&format!(
                        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#000000\"/>",
                        x, y, module_size_px, module_size_px
                    ));
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }

    /// Render QR Code langsung ke Dokumen PDF Vektor Asli
    pub fn render_to_pdf(&self, pdf: &mut PdfDocument, x: f32, y: f32, size_pt: f32) {
        let margin_modules = 2.0;
        let total_modules = (self.size as f32) + (margin_modules * 2.0);
        let module_size = size_pt / total_modules;

        let page = pdf.current_page_mut();

        // Background putih
        page.add_rect(x, y - size_pt, size_pt, size_pt, true, (1.0, 1.0, 1.0));
        page.add_rect(x, y - size_pt, size_pt, size_pt, false, (0.85, 0.88, 0.92));

        // Modul-modul hitam
        for (r, row) in self.modules.iter().enumerate() {
            let row_y = y - ((margin_modules + r as f32 + 1.0) * module_size);
            for (c, &is_dark) in row.iter().enumerate() {
                if is_dark {
                    let col_x = x + ((margin_modules + c as f32) * module_size);
                    page.add_rect(col_x, row_y, module_size, module_size, true, (0.05, 0.10, 0.20));
                }
            }
        }
    }
}

// =========================================================================
// Logika Internal Standar QR Code (ISO/IEC 18004)
// =========================================================================

fn push_bits(bits: &mut Vec<bool>, val: u32, count: usize) {
    for i in (0..count).rev() {
        bits.push(((val >> i) & 1) == 1);
    }
}

fn place_finder_pattern(
    modules: &mut [Vec<bool>],
    is_function: &mut [Vec<bool>],
    start_r: usize,
    start_c: usize,
    size: usize,
) {
    // 7x7 Pattern + 1 module separator di sekitarnya
    let r_min = start_r.saturating_sub(1);
    let r_max = (start_r + 7).min(size - 1);
    let c_min = start_c.saturating_sub(1);
    let c_max = (start_c + 7).min(size - 1);

    for r in r_min..=r_max {
        for c in c_min..=c_max {
            is_function[r][c] = true;
            modules[r][c] = false; // default separator putih
        }
    }

    // 7x7 Finder Matrix
    for r in 0..7 {
        for c in 0..7 {
            let is_dark = r == 0 || r == 6 || c == 0 || c == 6 || (r >= 2 && r <= 4 && c >= 2 && c <= 4);
            modules[start_r + r][start_c + c] = is_dark;
            is_function[start_r + r][start_c + c] = true;
        }
    }
}

fn place_alignment_pattern(
    modules: &mut [Vec<bool>],
    is_function: &mut [Vec<bool>],
    center_r: usize,
    center_c: usize,
) {
    let start_r = center_r.saturating_sub(2);
    let start_c = center_c.saturating_sub(2);

    for r in 0..5 {
        for c in 0..5 {
            let curr_r = start_r + r;
            let curr_c = start_c + c;
            let is_dark = r == 0 || r == 4 || c == 0 || c == 4 || (r == 2 && c == 2);
            modules[curr_r][curr_c] = is_dark;
            is_function[curr_r][curr_c] = true;
        }
    }
}

fn reserve_format_info_areas(is_function: &mut [Vec<bool>], size: usize) {
    // Top-left
    for c in 0..=8 {
        is_function[8][c] = true;
    }
    for r in 0..=8 {
        is_function[r][8] = true;
    }

    // Top-right
    for c in (size - 8)..size {
        is_function[8][c] = true;
    }

    // Bottom-left
    for r in (size - 8)..size {
        is_function[r][8] = true;
    }
}

fn place_codewords(
    modules: &mut [Vec<bool>],
    is_function: &[Vec<bool>],
    codewords: &[u8],
    size: usize,
    mask_pattern: usize,
) {
    let mut bit_idx = 0;
    let total_bits = codewords.len() * 8;

    let mut going_up = true;
    let mut c = size - 1;

    while c > 0 {
        if c == 6 {
            // Kolom timing pattern vertikal dilewati
            c -= 1;
        }

        let row_range: Vec<usize> = if going_up {
            (0..size).rev().collect()
        } else {
            (0..size).collect()
        };

        for &r in &row_range {
            for &col in &[c, c - 1] {
                if !is_function[r][col] {
                    let bit = if bit_idx < total_bits {
                        let byte_val = codewords[bit_idx / 8];
                        let shift = 7 - (bit_idx % 8);
                        ((byte_val >> shift) & 1) == 1
                    } else {
                        false
                    };

                    // Masking (Mask 0: (r + col) % 2 == 0)
                    let mask = match mask_pattern {
                        0 => (r + col) % 2 == 0,
                        1 => r % 2 == 0,
                        2 => col % 3 == 0,
                        3 => (r + col) % 3 == 0,
                        _ => (r + col) % 2 == 0,
                    };

                    modules[r][col] = bit ^ mask;
                    bit_idx += 1;
                }
            }
        }

        going_up = !going_up;
        c = c.saturating_sub(2);
    }
}

fn get_format_bits(ec: QrEcLevel, mask: usize) -> u16 {
    // 5 bit data: 2 bit EC + 3 bit mask
    let data = ((ec.format_bits() as u32) << 3) | ((mask as u32) & 0x07);
    let mut rem = data << 10;
    let poly_gen = 0x537; // x^10 + x^8 + x^5 + x^4 + x^2 + x + 1

    for i in (0..5).rev() {
        if ((rem >> (i + 10)) & 1) == 1 {
            rem ^= poly_gen << i;
        }
    }

    let bch = (data << 10) | rem;
    // XOR mask: 101010000010010 (0x5412)
    (bch ^ 0x5412) as u16
}

fn write_format_info(modules: &mut [Vec<bool>], size: usize, format_bits: u16) {
    // 15 bit format information
    // Top-left horizontal & vertical
    let coords_tl = [
        (8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (8, 5), (8, 7), (8, 8),
        (7, 8), (5, 8), (4, 8), (3, 8), (2, 8), (1, 8), (0, 8),
    ];

    for (i, &(r, c)) in coords_tl.iter().enumerate() {
        let bit = ((format_bits >> i) & 1) == 1;
        modules[r][c] = bit;
    }

    // Split antara bottom-left (bit 0-6) dan top-right (bit 7-14)
    for i in 0..7 {
        let bit = ((format_bits >> i) & 1) == 1;
        modules[size - 1 - i][8] = bit;
    }

    for i in 0..8 {
        let bit = ((format_bits >> (7 + i)) & 1) == 1;
        modules[8][size - 8 + i] = bit;
    }
}

// =========================================================================
// Aritmatika Galois Field GF(256) & Reed-Solomon Codec
// =========================================================================

struct Gf256 {
    exp: [u8; 512],
    log: [u8; 256],
}

impl Gf256 {
    fn new() -> Self {
        let mut exp = [0u8; 512];
        let mut log = [0u8; 256];
        let mut x = 1u16;

        for i in 0..255 {
            exp[i] = x as u8;
            exp[i + 255] = x as u8;
            log[x as usize] = i as u8;
            x <<= 1;
            if x >= 256 {
                x ^= 0x11D; // Polinomial primitif x^8 + x^4 + x^3 + x^2 + 1
            }
        }

        Self { exp, log }
    }

    fn mul(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 {
            0
        } else {
            let idx = (self.log[a as usize] as usize) + (self.log[b as usize] as usize);
            self.exp[idx % 255]
        }
    }
}

fn calculate_reed_solomon_ec(data: &[u8], ec_count: usize) -> Vec<u8> {
    let gf = Gf256::new();

    // Buat generator polynomial g(x) = (x - a^0)(x - a^1)...(x - a^(ec_count-1))
    let mut poly_gen = vec![1u8];
    for i in 0..ec_count {
        let factor = [1u8, gf.exp[i]];
        let mut next_gen = vec![0u8; poly_gen.len() + 1];
        for (j, &g) in poly_gen.iter().enumerate() {
            next_gen[j] ^= g;
            next_gen[j + 1] ^= gf.mul(g, factor[1]);
        }
        poly_gen = next_gen;
    }

    // Pembagian sintetik polinomial modulo poly_gen(x)
    let mut rem = vec![0u8; ec_count];
    for &d in data {
        let factor = d ^ rem[0];
        rem.remove(0);
        rem.push(0);

        for (j, r) in rem.iter_mut().enumerate() {
            *r ^= gf.mul(poly_gen[j + 1], factor);
        }
    }

    rem
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_encode_and_matrix_generation() {
        let qr = WidyaQrCode::encode("https://widya-lang.org").expect("QR encode harus berhasil");
        assert_eq!(qr.version, 2);
        assert_eq!(qr.size, 25);
        assert!(!qr.modules.is_empty());

        // Verifikasi finder patterns (titik tengah pojok atas kiri harus hitam)
        assert!(qr.modules[3][3]);
        // Titik tengah pojok atas kanan harus hitam
        assert!(qr.modules[3][qr.size - 4]);
        // Titik tengah pojok bawah kiri harus hitam
        assert!(qr.modules[qr.size - 4][3]);
    }

    #[test]
    fn test_qr_to_svg_rendering() {
        let qr = WidyaQrCode::encode("TTE-VERIFIED-2026").unwrap();
        let svg = qr.to_svg(4, 2);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("fill=\"#000000\""));
        assert!(svg.ends_with("</svg>"));
    }
}
