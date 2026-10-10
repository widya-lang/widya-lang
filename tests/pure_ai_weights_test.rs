// ============================================================================
// Integration Test: Pure Sovereign SafeTensors & GGUF v3 Model Weight Engine
// ============================================================================
// Menguji serialisasi dan deserialisasi biner model bobot AI SafeTensors (.safetensors)
// dan GGUF v3 (.gguf) dengan kuantisasi Q8_0 tanpa PyTorch/Python.
// ============================================================================

use std::fs;
use widya::ai::{
    PureGgufEngine, PureGgufModel, PureGgufTensor, PureGgufType, PureGgufValue,
    PureSafeTensorsEngine, SafeTensorRecord, SafeTensorsModel,
};

#[test]
fn test_safetensors_serialization_and_roundtrip() {
    let weight_vals = vec![
        1.0f32, 2.5, -3.2, 0.0,
        0.5, -1.2, 4.4, 3.1,
        -0.7, 8.2, 1.1, -2.0,
        5.5, 0.1, -0.4, 2.2,
    ];
    let bias_vals = vec![0.1f32, -0.2, 0.3, -0.4];

    let t_weight = SafeTensorRecord::baru_f32("model.layers.0.weight", vec![4, 4], &weight_vals);
    let t_bias = SafeTensorRecord::baru_f32("model.layers.0.bias", vec![4], &bias_vals);

    let model = SafeTensorsModel::baru()
        .dengan_metadata("format", "widya_pure")
        .dengan_metadata("author", "Widya-Lang AI Foundation")
        .tambah_tensor(t_weight)
        .tambah_tensor(t_bias);

    // 1. Serialisasi ke biner
    let bytes = PureSafeTensorsEngine::tulis_ke_bytes(&model)
        .expect("Serialisasi SafeTensors harus berhasil");

    // 2. Verifikasi header 8-byte LE
    assert!(bytes.len() > 8);
    let header_len = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
    assert!(header_len > 0);
    assert!(8 + header_len <= bytes.len());

    // 3. Deserialisasi kembali
    let parsed_model = PureSafeTensorsEngine::baca_dari_bytes(&bytes)
        .expect("Deserialisasi SafeTensors harus berhasil");

    assert_eq!(parsed_model.metadata.get("format").unwrap(), "widya_pure");
    assert_eq!(parsed_model.metadata.get("author").unwrap(), "Widya-Lang AI Foundation");
    assert_eq!(parsed_model.tensors.len(), 2);

    let w_parsed = parsed_model.tensors.get("model.layers.0.weight").unwrap();
    assert_eq!(w_parsed.shape, vec![4, 4]);
    assert_eq!(w_parsed.dtype, "F32");
    assert_eq!(w_parsed.to_f32_vec().unwrap(), weight_vals);

    let b_parsed = parsed_model.tensors.get("model.layers.0.bias").unwrap();
    assert_eq!(b_parsed.shape, vec![4]);
    assert_eq!(b_parsed.dtype, "F32");
    assert_eq!(b_parsed.to_f32_vec().unwrap(), bias_vals);
}

#[test]
fn test_safetensors_file_export_and_import() {
    let t = SafeTensorRecord::baru_f32("embedding.weight", vec![2, 3], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let model = SafeTensorsModel::baru().tambah_tensor(t);

    let temp_dir = std::env::temp_dir().join("widya_safetensors_test");
    let target_file = temp_dir.join("model_test.safetensors");

    let exported = PureSafeTensorsEngine::ekspor_berkas(&model, &target_file)
        .expect("Ekspor SafeTensors harus berhasil");
    assert_eq!(exported, target_file);
    assert!(target_file.exists());

    let imported = PureSafeTensorsEngine::impor_berkas(&target_file)
        .expect("Impor SafeTensors harus berhasil");
    assert_eq!(imported.tensors.len(), 1);
    assert_eq!(
        imported.tensors.get("embedding.weight").unwrap().shape,
        vec![2, 3]
    );

    let _ = fs::remove_file(&target_file);
    let _ = fs::remove_dir(&temp_dir);
}

#[test]
fn test_gguf_v3_serialization_and_roundtrip() {
    // 64 elemen float untuk uji kuantisasi Q8_0
    let f32_weights: Vec<f32> = (0..64).map(|i| (i as f32) * 0.1 - 3.2).collect();
    let q8_bytes = PureGgufEngine::kuantisasi_q8_0(&f32_weights);
    // 64 elemen = 2 blok 32 elemen -> 2 * 34 = 68 bytes
    assert_eq!(q8_bytes.len(), 68);

    // Raw F32 bytes untuk tensor embedding (32 elemen * 4 byte = 128 bytes)
    let embd_f32: Vec<f32> = (0..32).map(|i| i as f32).collect();
    let mut embd_bytes = Vec::new();
    for &v in &embd_f32 {
        embd_bytes.extend_from_slice(&v.to_le_bytes());
    }

    let t1 = PureGgufTensor {
        name: "token_embd.weight".to_string(),
        shape: vec![32, 1],
        tensor_type: PureGgufType::F32,
        data: embd_bytes,
    };

    let t2 = PureGgufTensor {
        name: "output.weight".to_string(),
        shape: vec![64, 1],
        tensor_type: PureGgufType::Q8_0,
        data: q8_bytes,
    };

    let model = PureGgufModel::baru()
        .dengan_meta("general.architecture", PureGgufValue::String("widya_slm".to_string()))
        .dengan_meta("widya_slm.context_length", PureGgufValue::UInt32(8192))
        .dengan_meta("widya_slm.is_sovereign", PureGgufValue::Bool(true))
        .tambah_tensor(t1)
        .tambah_tensor(t2);

    // 1. Serialisasi ke GGUF v3 biner
    let gguf_bytes = PureGgufEngine::tulis_ke_bytes(&model)
        .expect("Serialisasi GGUF harus berhasil");

    // 2. Verifikasi magic GGUF (0x46554747) dan Version 3
    assert!(gguf_bytes.len() >= 24);
    assert_eq!(&gguf_bytes[0..4], b"GGUF");
    let version = u32::from_le_bytes(gguf_bytes[4..8].try_into().unwrap());
    assert_eq!(version, 3);

    // 3. Deserialisasi kembali dari GGUF biner
    let parsed_model = PureGgufEngine::baca_dari_bytes(&gguf_bytes)
        .expect("Deserialisasi GGUF harus berhasil");

    // Verifikasi metadata
    assert_eq!(
        parsed_model.metadata.get("general.architecture"),
        Some(&PureGgufValue::String("widya_slm".to_string()))
    );
    assert_eq!(
        parsed_model.metadata.get("widya_slm.context_length"),
        Some(&PureGgufValue::UInt32(8192))
    );
    assert_eq!(
        parsed_model.metadata.get("widya_slm.is_sovereign"),
        Some(&PureGgufValue::Bool(true))
    );

    // Verifikasi tensor
    assert_eq!(parsed_model.tensors.len(), 2);
    let p_t1 = &parsed_model.tensors[0];
    assert_eq!(p_t1.name, "token_embd.weight");
    assert_eq!(p_t1.tensor_type, PureGgufType::F32);
    assert_eq!(p_t1.shape, vec![32, 1]);
    assert_eq!(p_t1.data.len(), 128);

    let p_t2 = &parsed_model.tensors[1];
    assert_eq!(p_t2.name, "output.weight");
    assert_eq!(p_t2.tensor_type, PureGgufType::Q8_0);
    assert_eq!(p_t2.shape, vec![64, 1]);
    assert_eq!(p_t2.data.len(), 68);
}
