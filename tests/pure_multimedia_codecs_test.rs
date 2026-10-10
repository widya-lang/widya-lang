use widya::multimedia::pure_codecs::{BmpCodec, QoiCodec, WavCodec};
use widya::multimedia::{AudioBuffer, AudioFrame, ImageBuffer, RgbaPixel};

#[test]
fn test_qoi_codec_lossless_roundtrip() {
    let width = 8;
    let height = 8;
    let mut pixels = Vec::new();

    // Create a pattern that triggers RUN, DIFF, LUMA, and INDEX:
    for y in 0..height {
        for x in 0..width {
            if y < 2 {
                // RUN sequence
                pixels.push(RgbaPixel::new(255, 0, 0, 255));
            } else if y < 4 {
                // Small DIFF
                pixels.push(RgbaPixel::new(100 + (x as u8), 100 + (y as u8), 50, 255));
            } else if y < 6 {
                // LUMA (larger green delta)
                pixels.push(RgbaPixel::new(100 + (x as u8 * 2), 150 + (y as u8 * 5), 80, 255));
            } else {
                // RGBA with alpha variation
                pixels.push(RgbaPixel::new((x * 30) as u8, (y * 30) as u8, 128, 128 + (x * 10) as u8));
            }
        }
    }

    // 1. Encode QOI RGBA
    let qoi_bytes = QoiCodec::encode(&pixels, width, height, 4).expect("Failed to encode QOI");
    assert!(qoi_bytes.starts_with(b"qoif"));
    assert_eq!(&qoi_bytes[qoi_bytes.len() - 8..], &[0, 0, 0, 0, 0, 0, 0, 1]);

    // 2. Decode QOI
    let (header, decoded_pixels) = QoiCodec::decode(&qoi_bytes).expect("Failed to decode QOI");
    assert_eq!(header.width, width);
    assert_eq!(header.height, height);
    assert_eq!(header.channels, 4);
    assert_eq!(decoded_pixels.len(), pixels.len());
    assert_eq!(decoded_pixels, pixels);

    // 3. Test through ImageBuffer helper methods
    let img_buf = ImageBuffer {
        width,
        height,
        pixels: pixels.clone(),
    };
    let encoded = img_buf.to_qoi(true).unwrap();
    let restored = ImageBuffer::from_qoi(&encoded).unwrap();
    assert_eq!(restored.width, width);
    assert_eq!(restored.height, height);
    assert_eq!(restored.pixels, pixels);
}

#[test]
fn test_bmp_codec_24bit_and_32bit_with_padding() {
    // Width 5 requires 1 byte padding for 24-bit (5 * 3 = 15 bytes -> padded to 16)
    let width = 5;
    let height = 4;
    let mut pixels = Vec::new();

    for y in 0..height {
        for x in 0..width {
            pixels.push(RgbaPixel::new(
                (x * 40) as u8,
                (y * 50) as u8,
                ((x + y) * 20) as u8,
                255,
            ));
        }
    }

    // 1. Test 24-bit BMP
    let bmp24_bytes = BmpCodec::encode(&pixels, width, height, 24).expect("Encode BMP 24 failed");
    assert_eq!(&bmp24_bytes[0..2], b"BM");
    let (w24, h24, decoded24) = BmpCodec::decode(&bmp24_bytes).expect("Decode BMP 24 failed");
    assert_eq!(w24, width);
    assert_eq!(h24, height);
    assert_eq!(decoded24, pixels);

    // 2. Test 32-bit BMP with alpha
    let mut alpha_pixels = pixels.clone();
    for p in &mut alpha_pixels {
        p.a = 180;
    }
    let bmp32_bytes = BmpCodec::encode(&alpha_pixels, width, height, 32).expect("Encode BMP 32 failed");
    let (w32, h32, decoded32) = BmpCodec::decode(&bmp32_bytes).expect("Decode BMP 32 failed");
    assert_eq!(w32, width);
    assert_eq!(h32, height);
    assert_eq!(decoded32, alpha_pixels);

    // 3. Test ImageBuffer BMP helpers
    let img_buf = ImageBuffer {
        width,
        height,
        pixels: alpha_pixels.clone(),
    };
    let bmp_data = img_buf.to_bmp(32).unwrap();
    let restored_img = ImageBuffer::from_bmp(&bmp_data).unwrap();
    assert_eq!(restored_img.pixels, alpha_pixels);
}

#[test]
fn test_wav_codec_audio_roundtrip() {
    let sample_rate = 44100;
    let duration = 0.05; // 50ms
    let sine_buffer = AudioBuffer::generate_sine(sample_rate, 440.0, duration, 0.8);

    // 1. Encode to WAV
    let wav_bytes = sine_buffer.to_wav();
    assert!(wav_bytes.starts_with(b"RIFF"));
    assert_eq!(&wav_bytes[8..12], b"WAVE");

    // 2. Decode from WAV
    let decoded_buffer = AudioBuffer::from_wav(&wav_bytes).expect("Failed to decode WAV");
    assert_eq!(decoded_buffer.sample_rate, sample_rate);
    assert_eq!(decoded_buffer.frames.len(), sine_buffer.frames.len());

    // 3. Verify sample fidelity (16-bit PCM quantization error is < 1/32767)
    for (orig, dec) in sine_buffer.frames.iter().zip(decoded_buffer.frames.iter()) {
        assert!((orig.left - dec.left).abs() < 0.001);
        assert!((orig.right - dec.right).abs() < 0.001);
    }
}
