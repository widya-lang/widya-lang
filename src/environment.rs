use crate::error::{Galat, Span};
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
struct Variable {
    value: Value,
    is_const: bool,
}

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Variable>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: String, value: Value, is_const: bool) {
        self.values.insert(name, Variable { value, is_const });
    }

    pub fn get(&self, name: &str, span: &Span) -> Result<Value, Galat> {
        if let Some(var) = self.values.get(name) {
            return Ok(var.value.clone());
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow().get(name, span);
        }

        Err(Galat::runtime(
            format!("Variabel '{}' belum didefinisikan", name),
            span,
        ))
    }

    pub fn assign(&mut self, name: &str, value: Value, span: &Span) -> Result<(), Galat> {
        if let Some(var) = self.values.get_mut(name) {
            if var.is_const {
                return Err(Galat::runtime(
                    format!(
                        "Tidak dapat mengubah nilai konstanta '{}' (didefinisikan dengan 'tetap')",
                        name
                    ),
                    span,
                ));
            }
            var.value = value;
            return Ok(());
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow_mut().assign(name, value, span);
        }

        Err(Galat::runtime(
            format!(
                "Variabel '{}' belum didefinisikan sebelum ditugaskan nilai",
                name
            ),
            span,
        ))
    }

    pub fn get_all_local(&self) -> HashMap<String, Value> {
        self.values
            .iter()
            .map(|(k, v)| (k.clone(), v.value.clone()))
            .collect()
    }

    /// Cek apakah variabel dengan nama ini ada DI LOCAL ENV INI SAJA (parent tidak dicari).
    pub fn contains_key_local(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Kumpulkan SEMUA variabel dari env ini + seluruh parent chain (urutan: parent dulu, lalu local override).
    /// Digunakan untuk inject seluruh fungsi / variabel ke Interpreter BARU sebelum execute handler.
    pub fn flatten_chain(&self) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        // Traverse parent DULU (agar local values bisa override parent)
        if let Some(ref parent) = self.parent {
            let parent_map = parent.borrow().flatten_chain();
            for (k, v) in parent_map {
                result.insert(k, v);
            }
        }
        // Local values override parent
        for (k, v) in &self.values {
            result.insert(k.clone(), v.value.clone());
        }
        result
    }
}
