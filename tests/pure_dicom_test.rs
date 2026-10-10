//! Unit & Integration Test: WidyaHealth Pure Medical Imaging DICOM Engine
//! Memverifikasi pembuatan file DICOM v3.0, encoding/decoding tag Explicit VR,
//! Hounsfield Unit (HU) scaling, radiologic contrast windowing presets, dan ekspor ke ImageBuffer.

use widya::healthtech::pure_dicom::{DicomWindowPreset, PureDicomFile};
use widya::healthtech::DicomMetadata;

#[test]
fn test_dicom_ct_encode_and_parse_roundtrip() {
    let rows = 4;
    let cols = 4;
    // Nilai piksel mentah simulasi: intercept = -1024, slope = 1.0
    // Jadi HU = raw - 1024
    // Misal: udara (-1000 HU) -> raw = 24
    // air (0 HU) -> raw = 1024
    // organ lunak (+40 HU) -> raw = 1064
    // tulang (+800 HU) -> raw = 1824
    let mut pixels = vec![0i16; (rows * cols) as usize];
    pixels[0] = 24;    // ~ -1000 HU (Air)
    pixels[1] = 1024;  // ~ 0 HU (Water)
    pixels[2] = 1064;  // ~ 40 HU (Soft Tissue)
    pixels[3] = 1824;  // ~ 800 HU (Bone)

    let dicom = PureDicomFile::new_ct(
        "PASIEN^SOEHARTO",
        "RM-2026-00123",
        rows,
        cols,
        -1024.0,
        1.0,
        pixels,
    );

    let bytes = dicom.encode_bytes();

    // Verifikasi header
    assert!(bytes.len() >= 132);
    assert_eq!(&bytes[128..132], b"DICM");

    // Parse kembali
    let parsed = PureDicomFile::parse_bytes(&bytes).expect("Parse DICOM bytes failed");
    assert_eq!(parsed.patient_name, "PASIEN^SOEHARTO");
    assert_eq!(parsed.patient_id, "RM-2026-00123");
    assert_eq!(parsed.modality, "CT");
    assert_eq!(parsed.rows, 4);
    assert_eq!(parsed.columns, 4);
    assert_eq!(parsed.rescale_intercept, -1024.0);
    assert_eq!(parsed.rescale_slope, 1.0);
    assert_eq!(parsed.pixel_data.len(), 16);
    assert_eq!(parsed.pixel_data[0], 24);
    assert_eq!(parsed.pixel_data[1], 1024);
}

#[test]
fn test_hounsfield_unit_and_contrast_windowing_to_image_buffer() {
    let rows = 2;
    let cols = 2;
    // Buat 4 piksel dengan densitas HU bervariasi
    // intercept = 0, slope = 1 -> raw = HU
    let pixels = vec![
        -1000, // Udara
        40,    // Soft tissue
        400,   // Tulang awal
        1000,  // Tulang padat
    ];

    let dicom = PureDicomFile::new_ct(
        "KARTINI^RADEN",
        "RM-9999",
        rows,
        cols,
        0.0,
        1.0,
        pixels,
    );

    // 1. Uji Preset Soft Tissue (C = 40, W = 400 -> rentang [-160, 240])
    let img_soft = dicom.to_image_buffer(DicomWindowPreset::SoftTissue);
    assert_eq!(img_soft.width, 2);
    assert_eq!(img_soft.height, 2);

    // Udara (-1000 HU <= -160) -> 0 (Hitam pekat)
    assert_eq!(img_soft.pixels[0].r, 0);
    // Soft tissue (+40 HU tepat di Center) -> ~128 (Abu-abu netral)
    assert!((img_soft.pixels[1].r as i32 - 128).abs() <= 2);
    // Tulang (+400 HU >= 240) -> 255 (Putih penuh jenuh)
    assert_eq!(img_soft.pixels[2].r, 255);

    // 2. Uji Preset Bone (C = 400, W = 1800 -> rentang [-500, 1300])
    let img_bone = dicom.to_image_buffer(DicomWindowPreset::Bone);
    // Soft tissue (+40 HU) tidak jenuh di window tulang
    assert!(img_bone.pixels[1].r > 0 && img_bone.pixels[1].r < 255);
    // Tulang padat (+1000 HU) terlihat dengan kontras abu-abu terang
    assert!(img_bone.pixels[3].r > img_bone.pixels[2].r);
}

#[test]
fn test_dicom_legacy_metadata_bridge() {
    let dicom = PureDicomFile::new_ct(
        "TEST^PASIEN",
        "ID123",
        512,
        512,
        -1024.0,
        1.0,
        vec![0; 512 * 512],
    );

    let meta: DicomMetadata = dicom.into();
    assert!(meta.is_valid_dicom);
    assert_eq!(meta.patient_name, Some("TEST^PASIEN".to_string()));
    assert_eq!(meta.patient_id, Some("ID123".to_string()));
    assert_eq!(meta.modality, "CT");
    assert_eq!(meta.rows, 512);
    assert_eq!(meta.columns, 512);
}
