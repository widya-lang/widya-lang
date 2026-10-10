//! Reactive State Management & Signal Engine (Zero External Dependencies)
//! Standar State Management: ValueNotifier, ChangeNotifier, Fine-Grained Signals,
//! dan Mekanisme Reaktivitas Subtree.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Objek Pendengar Perubahan Status (State Listener Callback)
pub type ListenerCallback<T> = Arc<dyn Fn(&T) + Send + Sync>;

/// Pemberitahu Nilai (ValueNotifier)
pub struct PemberitahuNilai<T: Clone> {
    pub id: String,
    nilai: Arc<Mutex<T>>,
    pendengar: Arc<Mutex<Vec<ListenerCallback<T>>>>,
}

impl<T: Clone + 'static> PemberitahuNilai<T> {
    pub fn baru(id: &str, nilai_awal: T) -> Self {
        Self {
            id: id.to_string(),
            nilai: Arc::new(Mutex::new(nilai_awal)),
            pendengar: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn ambil(&self) -> T {
        self.nilai.lock().unwrap().clone()
    }

    pub fn setel(&self, nilai_baru: T) {
        {
            let mut val = self.nilai.lock().unwrap();
            *val = nilai_baru.clone();
        }
        let list = self.pendengar.lock().unwrap().clone();
        for cb in list {
            cb(&nilai_baru);
        }
    }

    pub fn dengarkan<F: Fn(&T) + Send + Sync + 'static>(&self, callback: F) {
        self.pendengar.lock().unwrap().push(Arc::new(callback));
    }
}

/// Sinyal Reaktif Ringan (Fine-Grained Signal)
pub struct Sinyal<T: Clone> {
    pub notifier: PemberitahuNilai<T>,
}

impl<T: Clone + 'static> Sinyal<T> {
    pub fn baru(id: &str, nilai_awal: T) -> Self {
        Self {
            notifier: PemberitahuNilai::baru(id, nilai_awal),
        }
    }

    pub fn baca(&self) -> T {
        self.notifier.ambil()
    }

    pub fn tulis(&self, nilai_baru: T) {
        self.notifier.setel(nilai_baru);
    }
}
