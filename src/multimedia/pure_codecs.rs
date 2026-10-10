//! Pure Sovereign Zero-Dependency Multimedia Codecs (pure_codecs.rs)
//!
//! 100% In-House Implementations:
//! - QOI (Quite OK Image Format): RFC / Phobos spec lossless image codec.
//! - BMP (Windows Bitmap): Standard 24-bit RGB and 32-bit RGBA uncompressed raster codec.
//! - WAV (RIFF/WAVE): Canonical 16-bit PCM stereo & mono audio codec.

use crate::multimedia::audio::{AudioBuffer, AudioFrame};
use crate::multimedia::image::{ImageBuffer, RgbaPixel};

// ============================================================================
// 1. QOI (QUITE OK IMAGE FORMAT) ENGINE
// ============================================================================

pub const QOI_MAGIC: [u8; 4] = [b'q', b'o', b'i', b'f'];
pub const QOI_HEADER_SIZE: usize = 14;
pub const QOI_END_MARKER: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];

pub const QOI_OP_INDEX_TAG: u8 = 0x00;
pub const QOI_OP_DIFF_TAG: u8  = 0x40;
pub const QOI_OP_LUMA_TAG: u8  = 0x80;
pub const QOI_OP_RUN_TAG: u8   = 0xC0;
pub const QOI_OP_RGB_TAG: u8   = 0xFE;
pub const QOI_OP_RGBA_TAG: u8  = 0xFF;

#[inline]
fn qoi_color_hash(pixel: RgbaPixel) -> usize {
    ((pixel.r as usize * 3)
        + (pixel.g as usize * 5)
        + (pixel.b as usize * 7)
        + (pixel.a as usize * 11))
        % 64
}

/// Metadata header QOI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QoiHeader {
    pub width: u32,
    pub height: u32,
    pub channels: u8,   // 3 = RGB, 4 = RGBA
    pub colorspace: u8, // 0 = sRGB with linear alpha, 1 = all channels linear
}

pub struct QoiCodec;

impl QoiCodec {
    /// Encode buffer piksel ke format biner QOI
    pub fn encode(pixels: &[RgbaPixel], width: u32, height: u32, channels: u8) -> Result<Vec<u8>, String> {
        if pixels.len() != (width as usize) * (height as usize) {
            return Err("Panjang buffer piksel tidak cocok dengan dimensi gambar".to_string());
        }
        if channels != 3 && channels != 4 {
            return Err("Kanal QOI harus 3 (RGB) atau 4 (RGBA)".to_string());
        }

        let mut out = Vec::with_capacity(QOI_HEADER_SIZE + pixels.len() * 4 + 8);

        // Header: "qoif", width, height, channels, colorspace (0)
        out.extend_from_slice(&QOI_MAGIC);
        out.extend_from_slice(&width.to_be_bytes());
        out.extend_from_slice(&height.to_be_bytes());
        out.push(channels);
        out.push(0); // colorspace: sRGB with linear alpha

        let mut index = [RgbaPixel { r: 0, g: 0, b: 0, a: 0 }; 64];
        let mut prev = RgbaPixel { r: 0, g: 0, b: 0, a: 255 };
        let mut run: u8 = 0;

        for &px in pixels {
            if px == prev {
                run += 1;
                if run == 62 {
                    out.push(QOI_OP_RUN_TAG | (run - 1));
                    run = 0;
                }
            } else {
                if run > 0 {
                    out.push(QOI_OP_RUN_TAG | (run - 1));
                    run = 0;
                }

                let idx = qoi_color_hash(px);
                if index[idx] == px {
                    out.push(QOI_OP_INDEX_TAG | (idx as u8));
                } else {
                    index[idx] = px;

                    if px.a == prev.a {
                        let vr = (px.r as i16) - (prev.r as i16);
                        let vg = (px.g as i16) - (prev.g as i16);
                        let vb = (px.b as i16) - (prev.b as i16);

                        let vg_r = vr - vg;
                        let vg_b = vb - vg;

                        if vr >= -2 && vr <= 1 && vg >= -2 && vg <= 1 && vb >= -2 && vb <= 1 {
                            let byte = QOI_OP_DIFF_TAG
                                | (((vr + 2) as u8) << 4)
                                | (((vg + 2) as u8) << 2)
                                | ((vb + 2) as u8);
                            out.push(byte);
                        } else if vg >= -32 && vg <= 31 && vg_r >= -8 && vg_r <= 7 && vg_b >= -8 && vg_b <= 7 {
                            let byte1 = QOI_OP_LUMA_TAG | ((vg + 32) as u8);
                            let byte2 = (((vg_r + 8) as u8) << 4) | ((vg_b + 8) as u8);
                            out.push(byte1);
                            out.push(byte2);
                        } else {
                            out.push(QOI_OP_RGB_TAG);
                            out.push(px.r);
                            out.push(px.g);
                            out.push(px.b);
                        }
                    } else {
                        out.push(QOI_OP_RGBA_TAG);
                        out.push(px.r);
                        out.push(px.g);
                        out.push(px.b);
                        out.push(px.a);
                    }
                }
                prev = px;
            }
        }

        if run > 0 {
            out.push(QOI_OP_RUN_TAG | (run - 1));
        }

        out.extend_from_slice(&QOI_END_MARKER);
        Ok(out)
    }

    /// Decode stream biner QOI ke buffer piksel
    pub fn decode(bytes: &[u8]) -> Result<(QoiHeader, Vec<RgbaPixel>), String> {
        if bytes.len() < QOI_HEADER_SIZE + 8 {
            return Err("Ukuran data QOI terlalu pendek untuk header dan end marker".to_string());
        }

        if &bytes[0..4] != &QOI_MAGIC {
            return Err("Magic identifier QOI tidak valid".to_string());
        }

        let width = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let height = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let channels = bytes[12];
        let colorspace = bytes[13];

        if channels != 3 && channels != 4 {
            return Err(format!("Jumlah kanal QOI tidak valid: {}", channels));
        }

        let total_pixels = (width as usize) * (height as usize);
        let mut pixels = Vec::with_capacity(total_pixels);

        let mut index = [RgbaPixel { r: 0, g: 0, b: 0, a: 0 }; 64];
        let mut current = RgbaPixel { r: 0, g: 0, b: 0, a: 255 };

        let mut p = QOI_HEADER_SIZE;
        let limit = bytes.len() - 8; // sisa 8 byte end marker

        while p < limit && pixels.len() < total_pixels {
            let b1 = bytes[p];
            p += 1;

            if b1 == QOI_OP_RGB_TAG {
                if p + 3 > limit {
                    return Err("Data terpotong pada QOI_OP_RGB".to_string());
                }
                current.r = bytes[p];
                current.g = bytes[p + 1];
                current.b = bytes[p + 2];
                p += 3;
            } else if b1 == QOI_OP_RGBA_TAG {
                if p + 4 > limit {
                    return Err("Data terpotong pada QOI_OP_RGBA".to_string());
                }
                current.r = bytes[p];
                current.g = bytes[p + 1];
                current.b = bytes[p + 2];
                current.a = bytes[p + 3];
                p += 4;
            } else {
                let tag = b1 & 0xC0;
                match tag {
                    QOI_OP_INDEX_TAG => {
                        let idx = (b1 & 0x3F) as usize;
                        current = index[idx];
                    }
                    QOI_OP_DIFF_TAG => {
                        let dr = ((b1 >> 4) & 0x03) as i16 - 2;
                        let dg = ((b1 >> 2) & 0x03) as i16 - 2;
                        let db = (b1 & 0x03) as i16 - 2;
                        current.r = ((current.r as i16 + dr) as u8) & 0xFF;
                        current.g = ((current.g as i16 + dg) as u8) & 0xFF;
                        current.b = ((current.b as i16 + db) as u8) & 0xFF;
                    }
                    QOI_OP_LUMA_TAG => {
                        if p >= limit {
                            return Err("Data terpotong pada QOI_OP_LUMA".to_string());
                        }
                        let b2 = bytes[p];
                        p += 1;
                        let dg = (b1 & 0x3F) as i16 - 32;
                        let dr = (((b2 >> 4) & 0x0F) as i16 - 8) + dg;
                        let db = ((b2 & 0x0F) as i16 - 8) + dg;
                        current.r = ((current.r as i16 + dr) as u8) & 0xFF;
                        current.g = ((current.g as i16 + dg) as u8) & 0xFF;
                        current.b = ((current.b as i16 + db) as u8) & 0xFF;
                    }
                    QOI_OP_RUN_TAG => {
                        let run_len = (b1 & 0x3F) + 1;
                        for _ in 0..run_len {
                            pixels.push(current);
                        }
                        continue;
                    }
                    _ => unreachable!(),
                }
            }

            let idx = qoi_color_hash(current);
            index[idx] = current;
            pixels.push(current);
        }

        if pixels.len() != total_pixels {
            return Err(format!("Jumlah piksel terdecode ({}) tidak cocok dengan header ({})", pixels.len(), total_pixels));
        }

        Ok((QoiHeader { width, height, channels, colorspace }, pixels))
    }
}

// ============================================================================
// 2. BMP (WINDOWS BITMAP) ENGINE (24-BIT RGB & 32-BIT RGBA)
// ============================================================================

pub struct BmpCodec;

impl BmpCodec {
    /// Encode piksel ke Windows Bitmap (.bmp) 24-bit atau 32-bit
    pub fn encode(pixels: &[RgbaPixel], width: u32, height: u32, bits_per_pixel: u16) -> Result<Vec<u8>, String> {
        if pixels.len() != (width as usize) * (height as usize) {
            return Err("Dimensi piksel tidak valid untuk BMP".to_string());
        }
        if bits_per_pixel != 24 && bits_per_pixel != 32 {
            return Err("Hanya bits_per_pixel 24 (RGB) dan 32 (RGBA) yang didukung".to_string());
        }

        let bytes_per_pixel = (bits_per_pixel / 8) as usize;
        let row_stride_unpadded = (width as usize) * bytes_per_pixel;
        let row_padding = (4 - (row_stride_unpadded % 4)) % 4;
        let padded_row_size = row_stride_unpadded + row_padding;
        let image_data_size = (padded_row_size * (height as usize)) as u32;

        let file_header_size = 14u32;
        let info_header_size = 40u32;
        let data_offset = file_header_size + info_header_size;
        let total_file_size = data_offset + image_data_size;

        let mut out = Vec::with_capacity(total_file_size as usize);

        // BITMAPFILEHEADER (14 bytes)
        out.extend_from_slice(b"BM");                          // bfType
        out.extend_from_slice(&total_file_size.to_le_bytes()); // bfSize
        out.extend_from_slice(&0u16.to_le_bytes());            // bfReserved1
        out.extend_from_slice(&0u16.to_le_bytes());            // bfReserved2
        out.extend_from_slice(&data_offset.to_le_bytes());     // bfOffBits

        // BITMAPINFOHEADER (40 bytes)
        out.extend_from_slice(&info_header_size.to_le_bytes());// biSize
        out.extend_from_slice(&(width as i32).to_le_bytes());  // biWidth
        out.extend_from_slice(&(height as i32).to_le_bytes()); // biHeight (positif = bottom-up)
        out.extend_from_slice(&1u16.to_le_bytes());            // biPlanes
        out.extend_from_slice(&bits_per_pixel.to_le_bytes());  // biBitCount
        out.extend_from_slice(&0u32.to_le_bytes());            // biCompression (BI_RGB)
        out.extend_from_slice(&image_data_size.to_le_bytes()); // biSizeImage
        out.extend_from_slice(&2835i32.to_le_bytes());         // biXPelsPerMeter (~72 DPI)
        out.extend_from_slice(&2835i32.to_le_bytes());         // biYPelsPerMeter (~72 DPI)
        out.extend_from_slice(&0u32.to_le_bytes());            // biClrUsed
        out.extend_from_slice(&0u32.to_le_bytes());            // biClrImportant

        // Pixel data: baris diurutkan dari bawah ke atas (bottom-up)
        for y in (0..height).rev() {
            let row_start = (y * width) as usize;
            for x in 0..width {
                let px = pixels[row_start + x as usize];
                // BMP menyimpan warna dalam urutan B, G, R [, A]
                out.push(px.b);
                out.push(px.g);
                out.push(px.r);
                if bits_per_pixel == 32 {
                    out.push(px.a);
                }
            }
            for _ in 0..row_padding {
                out.push(0);
            }
        }

        Ok(out)
    }

    /// Decode file .bmp ke ImageBuffer piksel
    pub fn decode(bytes: &[u8]) -> Result<(u32, u32, Vec<RgbaPixel>), String> {
        if bytes.len() < 54 {
            return Err("Ukuran data BMP terlalu pendek (< 54 bytes)".to_string());
        }

        if &bytes[0..2] != b"BM" {
            return Err("Magic marker BMP tidak valid (harus 'BM')".to_string());
        }

        let data_offset = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]) as usize;
        let info_header_size = u32::from_le_bytes([bytes[14], bytes[15], bytes[16], bytes[17]]);

        if info_header_size < 40 {
            return Err("Format DIB header BMP kuno (< 40 bytes) tidak didukung".to_string());
        }

        let width_raw = i32::from_le_bytes([bytes[18], bytes[19], bytes[20], bytes[21]]);
        let height_raw = i32::from_le_bytes([bytes[22], bytes[23], bytes[24], bytes[25]]);
        let planes = u16::from_le_bytes([bytes[26], bytes[27]]);
        let bpp = u16::from_le_bytes([bytes[28], bytes[29]]);
        let compression = u32::from_le_bytes([bytes[30], bytes[31], bytes[32], bytes[33]]);

        if planes != 1 {
            return Err("Jumlah planes BMP harus 1".to_string());
        }
        if compression != 0 {
            return Err("Hanya kompresi BI_RGB (uncompressed) yang didukung".to_string());
        }
        if bpp != 24 && bpp != 32 {
            return Err(format!("Bits per pixel {} tidak didukung (harus 24 atau 32)", bpp));
        }

        let width = width_raw.unsigned_abs();
        let height = height_raw.unsigned_abs();
        let is_top_down = height_raw < 0;

        let bytes_per_pixel = (bpp / 8) as usize;
        let row_stride_unpadded = (width as usize) * bytes_per_pixel;
        let row_padding = (4 - (row_stride_unpadded % 4)) % 4;
        let padded_row_size = row_stride_unpadded + row_padding;

        let total_pixels = (width as usize) * (height as usize);
        let mut pixels = vec![RgbaPixel::TRANSPARENT; total_pixels];

        let mut offset = data_offset;
        for row_idx in 0..height {
            let target_y = if is_top_down {
                row_idx
            } else {
                height - 1 - row_idx
            };

            if offset + row_stride_unpadded > bytes.len() {
                return Err("Data BMP terpotong saat membaca baris piksel".to_string());
            }

            let row_bytes = &bytes[offset..offset + row_stride_unpadded];
            let target_start = (target_y * width) as usize;

            for x in 0..width as usize {
                let p_idx = x * bytes_per_pixel;
                let b = row_bytes[p_idx];
                let g = row_bytes[p_idx + 1];
                let r = row_bytes[p_idx + 2];
                let a = if bpp == 32 { row_bytes[p_idx + 3] } else { 255 };

                pixels[target_start + x] = RgbaPixel { r, g, b, a };
            }

            offset += padded_row_size;
        }

        Ok((width, height, pixels))
    }
}

// ============================================================================
// 3. WAV (RIFF/WAVE PCM 16-BIT) AUDIO ENGINE
// ============================================================================

pub struct WavCodec;

impl WavCodec {
    /// Encode AudioBuffer stereo float (-1.0 .. +1.0) ke biner WAV PCM 16-bit
    pub fn encode(audio: &AudioBuffer) -> Vec<u8> {
        let num_channels: u16 = 2; // Stereo
        let bits_per_sample: u16 = 16;
        let sample_rate = audio.sample_rate;
        let byte_rate = sample_rate * (num_channels as u32) * ((bits_per_sample as u32) / 8);
        let block_align = num_channels * (bits_per_sample / 8);

        let num_frames = audio.frames.len();
        let data_bytes = (num_frames * 2 * 2) as u32; // 2 channels * 2 bytes per sample
        let riff_chunk_size = 36 + data_bytes;

        let mut out = Vec::with_capacity((44 + data_bytes) as usize);

        // RIFF Header
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&riff_chunk_size.to_le_bytes());
        out.extend_from_slice(b"WAVE");

        // "fmt " Subchunk
        out.extend_from_slice(b"fmt ");
        out.extend_from_slice(&16u32.to_le_bytes());          // Subchunk1Size (16 for PCM)
        out.extend_from_slice(&1u16.to_le_bytes());           // AudioFormat (1 for PCM)
        out.extend_from_slice(&num_channels.to_le_bytes());
        out.extend_from_slice(&sample_rate.to_le_bytes());
        out.extend_from_slice(&byte_rate.to_le_bytes());
        out.extend_from_slice(&block_align.to_le_bytes());
        out.extend_from_slice(&bits_per_sample.to_le_bytes());

        // "data" Subchunk
        out.extend_from_slice(b"data");
        out.extend_from_slice(&data_bytes.to_le_bytes());

        // PCM 16-bit signed samples
        for frame in &audio.frames {
            let left_i16 = (frame.left.clamp(-1.0, 1.0) * 32767.0) as i16;
            let right_i16 = (frame.right.clamp(-1.0, 1.0) * 32767.0) as i16;
            out.extend_from_slice(&left_i16.to_le_bytes());
            out.extend_from_slice(&right_i16.to_le_bytes());
        }

        out
    }

    /// Decode biner WAV PCM (16-bit / 8-bit, mono / stereo) ke AudioBuffer
    pub fn decode(bytes: &[u8]) -> Result<AudioBuffer, String> {
        if bytes.len() < 44 {
            return Err("Ukuran data WAV terlalu pendek (< 44 bytes)".to_string());
        }

        if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err("Header RIFF WAVE tidak valid".to_string());
        }

        // Parsing chunks
        let mut pos = 12;
        let mut sample_rate: u32 = 44100;
        let mut channels: u16 = 2;
        let mut bits_per_sample: u16 = 16;
        let mut audio_format: u16 = 1;
        let mut data_slice: Option<&[u8]> = None;

        while pos + 8 <= bytes.len() {
            let chunk_id = &bytes[pos..pos + 4];
            let chunk_size = u32::from_le_bytes([
                bytes[pos + 4],
                bytes[pos + 5],
                bytes[pos + 6],
                bytes[pos + 7],
            ]) as usize;
            pos += 8;

            if chunk_id == b"fmt " {
                if chunk_size < 16 || pos + chunk_size > bytes.len() {
                    return Err("Format subchunk fmt WAV korup".to_string());
                }
                audio_format = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]);
                channels = u16::from_le_bytes([bytes[pos + 2], bytes[pos + 3]]);
                sample_rate = u32::from_le_bytes([bytes[pos + 4], bytes[pos + 5], bytes[pos + 6], bytes[pos + 7]]);
                bits_per_sample = u16::from_le_bytes([bytes[pos + 14], bytes[pos + 15]]);
                pos += chunk_size;
            } else if chunk_id == b"data" {
                let actual_size = chunk_size.min(bytes.len() - pos);
                data_slice = Some(&bytes[pos..pos + actual_size]);
                pos += chunk_size;
            } else {
                // Lewati chunk lain (misal: LIST, JUNK, dll.)
                pos += chunk_size;
            }
        }

        if audio_format != 1 {
            return Err(format!("Hanya format audio PCM (1) yang didukung, ditemukan {}", audio_format));
        }

        let raw_data = data_slice.ok_or_else(|| "Chunk 'data' tidak ditemukan dalam file WAV".to_string())?;

        let mut frames = Vec::new();
        match (channels, bits_per_sample) {
            (1, 16) => {
                let num_samples = raw_data.len() / 2;
                frames.reserve(num_samples);
                for i in 0..num_samples {
                    let sample_i16 = i16::from_le_bytes([raw_data[i * 2], raw_data[i * 2 + 1]]);
                    let val = sample_i16 as f32 / 32768.0;
                    frames.push(AudioFrame::mono(val));
                }
            }
            (2, 16) => {
                let num_frames = raw_data.len() / 4;
                frames.reserve(num_frames);
                for i in 0..num_frames {
                    let base = i * 4;
                    let l = i16::from_le_bytes([raw_data[base], raw_data[base + 1]]) as f32 / 32768.0;
                    let r = i16::from_le_bytes([raw_data[base + 2], raw_data[base + 3]]) as f32 / 32768.0;
                    frames.push(AudioFrame::new(l, r));
                }
            }
            (1, 8) => {
                frames.reserve(raw_data.len());
                for &b in raw_data {
                    let val = (b as f32 - 128.0) / 128.0;
                    frames.push(AudioFrame::mono(val));
                }
            }
            (2, 8) => {
                let num_frames = raw_data.len() / 2;
                frames.reserve(num_frames);
                for i in 0..num_frames {
                    let l = (raw_data[i * 2] as f32 - 128.0) / 128.0;
                    let r = (raw_data[i * 2 + 1] as f32 - 128.0) / 128.0;
                    frames.push(AudioFrame::new(l, r));
                }
            }
            _ => return Err(format!("Kombinasi kanal {} dan bit depth {} tidak didukung", channels, bits_per_sample)),
        }

        Ok(AudioBuffer { sample_rate, frames })
    }
}

// ============================================================================
// CONVENIENCE EXTENSIONS UNTUK ImageBuffer & AudioBuffer
// ============================================================================

impl ImageBuffer {
    pub fn to_qoi(&self, has_alpha: bool) -> Result<Vec<u8>, String> {
        let channels = if has_alpha { 4 } else { 3 };
        QoiCodec::encode(&self.pixels, self.width, self.height, channels)
    }

    pub fn from_qoi(bytes: &[u8]) -> Result<Self, String> {
        let (header, pixels) = QoiCodec::decode(bytes)?;
        Ok(Self {
            width: header.width,
            height: header.height,
            pixels,
        })
    }

    pub fn to_bmp(&self, bits_per_pixel: u16) -> Result<Vec<u8>, String> {
        BmpCodec::encode(&self.pixels, self.width, self.height, bits_per_pixel)
    }

    pub fn from_bmp(bytes: &[u8]) -> Result<Self, String> {
        let (width, height, pixels) = BmpCodec::decode(bytes)?;
        Ok(Self {
            width,
            height,
            pixels,
        })
    }
}

impl AudioBuffer {
    pub fn to_wav(&self) -> Vec<u8> {
        WavCodec::encode(self)
    }

    pub fn from_wav(bytes: &[u8]) -> Result<Self, String> {
        WavCodec::decode(bytes)
    }
}
