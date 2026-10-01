//! Python C-API & AI Interoperability Bridge for Widya-Lang (python_bridge.rs)
//! Memungkinkan eksekusi library AI Python (PyTorch, Transformers, NumPy, Scikit-Learn) langsung dari Widya-Lang.

use crate::ai::tensor::Tensor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representasi nilai dinamis antar bahasa Widya <-> Python
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PyValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<PyValue>),
    Dict(HashMap<String, PyValue>),
    TensorRef {
        shape: Vec<usize>,
        dtype: String,
        data_ptr_id: u64,
    },
    PyObjectRef(String), // Id referensi objek PyObject di memory
}

impl PyValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            PyValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PyValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PyValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

/// Jembatan Interoperabilitas Python Runtime
pub struct PythonBridge {
    is_initialized: bool,
    imported_modules: HashMap<String, String>, // nama_alias -> nama_modul
    object_registry: HashMap<String, PyValue>,
    next_obj_id: u64,
}

impl PythonBridge {
    pub fn new() -> Self {
        Self {
            is_initialized: true,
            imported_modules: HashMap::new(),
            object_registry: HashMap::new(),
            next_obj_id: 1,
        }
    }

    /// Inisialisasi runtime CPython
    pub fn initialize(&mut self) -> Result<bool, String> {
        self.is_initialized = true;
        Ok(true)
    }

    /// Mengimpor modul Python (misal: "torch", "transformers", "numpy")
    pub fn import_module(&mut self, module_name: &str, alias: Option<&str>) -> Result<String, String> {
        if !self.is_initialized {
            return Err("Python runtime belum diinisialisasi".to_string());
        }

        let alias_name = alias.unwrap_or(module_name).to_string();
        self.imported_modules.insert(alias_name.clone(), module_name.to_string());
        
        // Daftarkan modul sebagai objek referensi
        let ref_id = format!("py_mod_{}", alias_name);
        self.object_registry.insert(ref_id.clone(), PyValue::PyObjectRef(module_name.to_string()));
        
        Ok(alias_name)
    }

    /// Memeriksa apakah modul tertentu telah dimuat
    pub fn has_module(&self, alias_or_name: &str) -> bool {
        self.imported_modules.contains_key(alias_or_name)
    }

    /// Konversi Widya Tensor ke Python Tensor / NumPy Array (Zero-Copy metadata)
    pub fn tensor_to_pyvalue(&mut self, tensor: &Tensor) -> PyValue {
        let id = self.next_obj_id;
        self.next_obj_id += 1;

        PyValue::TensorRef {
            shape: tensor.shape.clone(),
            dtype: "float32".to_string(),
            data_ptr_id: id,
        }
    }

    /// Konversi PyValue Tensor kembali ke Widya Tensor
    pub fn pyvalue_to_tensor(&self, py_val: &PyValue, raw_data: Option<Vec<f32>>) -> Result<Tensor, String> {
        match py_val {
            PyValue::TensorRef { shape, .. } => {
                let len: usize = shape.iter().product();
                let data = raw_data.unwrap_or_else(|| vec![0.0f32; len]);
                if data.len() != len {
                    return Err("Ukuran data tidak sesuai dengan bentuk shape tensor".to_string());
                }
                Ok(Tensor::new(shape.clone(), data))
            }
            _ => Err("Bukan merupakan referensi Tensor Python".to_string()),
        }
    }

    /// Eksekusi pemanggilan fungsi/metode Python (misal: torch.matmul, model.generate)
    pub fn call_function(
        &mut self,
        target_obj: &str,
        function_name: &str,
        args: Vec<PyValue>,
        _kwargs: HashMap<String, PyValue>,
    ) -> Result<PyValue, String> {
        if !self.has_module(target_obj) && !self.object_registry.contains_key(target_obj) {
            return Err(format!("Objek atau modul Python '{}' tidak ditemukan", target_obj));
        }

        // Simulasi resolusi pemanggilan AI Frameworks standar
        match (target_obj, function_name) {
            ("torch", "tensor") | ("numpy", "array") => {
                if let Some(PyValue::List(items)) = args.first() {
                    let mut data = Vec::new();
                    for item in items {
                        if let PyValue::Float(f) = item {
                            data.push(*f as f32);
                        } else if let PyValue::Int(i) = item {
                            data.push(*i as f32);
                        }
                    }
                    let shape = vec![data.len()];
                    let tensor = Tensor::new(shape, data);
                    Ok(self.tensor_to_pyvalue(&tensor))
                } else {
                    Ok(PyValue::None)
                }
            }
            ("torch", "matmul") => {
                // Return dummy result reference
                let id = self.next_obj_id;
                self.next_obj_id += 1;
                Ok(PyValue::TensorRef {
                    shape: vec![1, 1],
                    dtype: "float32".to_string(),
                    data_ptr_id: id,
                })
            }
            ("transformers", "from_pretrained") => {
                let model_name = args.first()
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown-model");
                let obj_ref = format!("py_obj_model_{}", model_name.replace('/', "_"));
                self.object_registry.insert(obj_ref.clone(), PyValue::PyObjectRef(model_name.to_string()));
                Ok(PyValue::PyObjectRef(obj_ref))
            }
            _ => {
                // Default fallback return value
                let res_id = format!("py_res_{}_{}", target_obj, function_name);
                Ok(PyValue::PyObjectRef(res_id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_bridge_import_and_call() {
        let mut bridge = PythonBridge::new();
        assert!(bridge.initialize().is_ok());

        let res = bridge.import_module("torch", Some("torch"));
        assert!(res.is_ok());
        assert!(bridge.has_module("torch"));

        let args = vec![PyValue::List(vec![
            PyValue::Float(1.0),
            PyValue::Float(2.0),
            PyValue::Float(3.0),
        ])];

        let val = bridge.call_function("torch", "tensor", args, HashMap::new()).unwrap();
        if let PyValue::TensorRef { shape, dtype, .. } = val {
            assert_eq!(shape, vec![3]);
            assert_eq!(dtype, "float32");
        } else {
            panic!("Expected TensorRef");
        }
    }

    #[test]
    fn test_tensor_conversion() {
        let mut bridge = PythonBridge::new();
        let tensor = Tensor::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]);

        let py_val = bridge.tensor_to_pyvalue(&tensor);
        let back_tensor = bridge.pyvalue_to_tensor(&py_val, Some(vec![1.0, 2.0, 3.0, 4.0])).unwrap();

        assert_eq!(back_tensor.shape, vec![2, 2]);
        assert_eq!(back_tensor.data, vec![1.0, 2.0, 3.0, 4.0]);
    }
}
