//! Comprehensive AI Ecosystem Parity Integration Tests for Widya-Lang
//! Menguji Python Interoperability Bridge, DLPack, Autograd, Arrow DataFrame, CUDA Triton Emitter, dan Jupyter Protocol.

use widya::ai::{
    PythonBridge, PyValue,
    DLTensor, DLDataType, SafetensorsParser,
    AutoGradEngine,
    DataFrame, DataColumn,
    Tensor,
};
use widya::gpu::CudaTritonKernelEmitter;
use widya::tools::jupyter::WidyaJupyterKernel;
use std::collections::HashMap;

#[test]
fn test_python_ai_bridge_and_tensor_flow() {
    let mut bridge = PythonBridge::new();
    assert!(bridge.initialize().is_ok());

    // Import AI libraries
    assert!(bridge.import_module("torch", Some("torch")).is_ok());
    assert!(bridge.import_module("transformers", Some("transformers")).is_ok());

    // Call torch.tensor
    let args = vec![PyValue::List(vec![PyValue::Float(10.0), PyValue::Float(20.0), PyValue::Float(30.0)])];
    let res = bridge.call_function("torch", "tensor", args, HashMap::new()).unwrap();
    
    if let PyValue::TensorRef { shape, dtype, .. } = res {
        assert_eq!(shape, vec![3]);
        assert_eq!(dtype, "float32");
    } else {
        panic!("Diharapkan mengembalikan TensorRef");
    }

    // Call transformers model loading
    let model_args = vec![PyValue::String("deepseek-ai/DeepSeek-V3".to_string())];
    let model_res = bridge.call_function("transformers", "from_pretrained", model_args, HashMap::new()).unwrap();
    assert!(model_res.as_str().is_none());
}

#[test]
fn test_dlpack_and_safetensors_integration() {
    // 1. DLPack zero-copy tensor structure
    let dl_tensor = DLTensor::new_cpu_f32(vec![4, 512, 64]);
    assert_eq!(dl_tensor.total_elements(), 4 * 512 * 64);
    assert!(dl_tensor.is_contiguous());

    // 2. Safetensors metadata parsing
    let header_json = r#"{
        "transformer.layer.0.attn.q_proj.weight": {
            "dtype": "F32",
            "shape": [4096, 4096],
            "data_offsets": [0, 67108864]
        },
        "transformer.layer.0.attn.k_proj.weight": {
            "dtype": "F32",
            "shape": [4096, 1024],
            "data_offsets": [67108864, 83886080]
        }
    }"#;

    let parsed = SafetensorsParser::parse_header_json(header_json).unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed["transformer.layer.0.attn.q_proj.weight"].shape, vec![4096, 4096]);
    assert_eq!(parsed["transformer.layer.0.attn.k_proj.weight"].data_offsets, (67108864, 83886080));
}

#[test]
fn test_autograd_multilayer_perceptron_backward() {
    let mut autograd = AutoGradEngine::new();

    // Input x = [2.0, -1.0], Weights w = [0.5, 2.0]
    let x = autograd.variable(vec![2], vec![2.0, -1.0], false);
    let w = autograd.variable(vec![2], vec![0.5, 2.0], true);
    let target = autograd.variable(vec![2], vec![1.0, 0.0], false);

    // h = x * w = [1.0, -2.0]
    let h = autograd.mul(x, w);
    
    // a = relu(h) = [1.0, 0.0]
    let a = autograd.relu(h);
    assert_eq!(autograd.get_data(a), &[1.0, 0.0]);

    // loss = mse(a, target) = mse([1.0, 0.0], [1.0, 0.0]) = 0.0
    let loss = autograd.mse_loss(a, target);
    assert_eq!(autograd.get_data(loss)[0], 0.0);

    // Backward pass
    autograd.backward(loss);
    assert_eq!(autograd.get_grad(w), &[0.0, 0.0]);
}

#[test]
fn test_columnar_dataframe_analytics() {
    let csv_data = "user_id,loss,accuracy\n1,0.45,0.85\n2,0.12,0.95\n3,0.88,0.60\n4,0.05,0.98";
    let df = DataFrame::from_csv_str(csv_data, true).unwrap();

    assert_eq!(df.num_rows(), 4);
    assert_eq!(df.num_cols(), 3);

    // Filter model dengan akurasi > 0.80
    let high_acc = df.filter_f64("accuracy", |acc| acc > 0.80).unwrap();
    assert_eq!(high_acc.num_rows(), 3);

    let avg_loss = high_acc.mean("loss").unwrap();
    // Avg loss of 0.45, 0.12, 0.05 = 0.62 / 3 = 0.206666...
    assert!((avg_loss - 0.206666).abs() < 0.001);
}

#[test]
fn test_cuda_triton_kernel_emission() {
    let fused_code = CudaTritonKernelEmitter::emit_fused_matmul_gelu_cuda(64, 64, 64);
    assert!(fused_code.contains("extern \"C\" __global__ void widya_fused_matmul_gelu"));
    
    let flash_attn = CudaTritonKernelEmitter::emit_flash_attention_kernel(4096, 128);
    assert!(flash_attn.contains("widya_flash_attention_forward"));
}

#[test]
fn test_jupyter_interactive_notebook_protocol() {
    let mut kernel = WidyaJupyterKernel::new("session_notebook_01");
    let info = kernel.get_kernel_info();
    assert_eq!(info.language_info.name, "widya");

    let res = kernel.execute_cell("cetak(\"Widya AI Kernel Siap\")");
    assert_eq!(res.status, "ok");
    assert_eq!(res.stdout, "Widya AI Kernel Siap\n");
}
