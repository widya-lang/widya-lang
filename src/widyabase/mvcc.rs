// ============================================================================
// WIDYABASE MULTI-VERSION CONCURRENCY CONTROL (MVCC) (mvcc.rs)
// ============================================================================
// Snapshot Isolation: Readers never block Writers, Writers never block Readers.
// Mengelola visibilitas baris berdasarkan xmin (transaksi pembuat) dan
// xmax (transaksi pengubah/penghapus).
// ============================================================================

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    Active,
    Committed,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub xmin: u64,           // ID transaksi aktif terendah saat snapshot dibuat
    pub xmax: u64,           // ID transaksi tertinggi berikutnya saat snapshot dibuat
    pub active_tx: HashSet<u64>, // Daftar transaksi yang masih aktif saat snapshot dibuat
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MvccHeader {
    pub xmin: u64, // Transaksi yang menyisipkan baris ini
    pub xmax: u64, // Transaksi yang menghapus baris ini (0 jika masih hidup)
}

impl MvccHeader {
    pub fn to_bytes(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0..8].copy_from_slice(&self.xmin.to_le_bytes());
        buf[8..16].copy_from_slice(&self.xmax.to_le_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let xmin = u64::from_le_bytes(bytes[0..8].try_into().unwrap_or([0; 8]));
        let xmax = u64::from_le_bytes(bytes[8..16].try_into().unwrap_or([0; 8]));
        MvccHeader { xmin, xmax }
    }
}

pub struct TransactionManager {
    next_tx_id: Arc<Mutex<u64>>,
    tx_status: Arc<RwLock<HashMap<u64, TxStatus>>>,
}

impl TransactionManager {
    pub fn new() -> Self {
        let mut status = HashMap::new();
        status.insert(0, TxStatus::Committed); // Tx 0 is bootstrap committed

        TransactionManager {
            next_tx_id: Arc::new(Mutex::new(1)),
            tx_status: Arc::new(RwLock::new(status)),
        }
    }

    /// Memulai transaksi baru
    pub fn begin_transaction(&self) -> u64 {
        let mut next = self.next_tx_id.lock().unwrap();
        let tx_id = *next;
        *next += 1;

        let mut status = self.tx_status.write().unwrap();
        status.insert(tx_id, TxStatus::Active);

        tx_id
    }

    /// Mengonfirmasi transaksi (Commit)
    pub fn commit(&self, tx_id: u64) {
        let mut status = self.tx_status.write().unwrap();
        status.insert(tx_id, TxStatus::Committed);
    }

    /// Membatalkan transaksi (Rollback/Abort)
    pub fn rollback(&self, tx_id: u64) {
        let mut status = self.tx_status.write().unwrap();
        status.insert(tx_id, TxStatus::Aborted);
    }

    /// Mengambil status transaksi tertentu
    pub fn get_status(&self, tx_id: u64) -> TxStatus {
        let status = self.tx_status.read().unwrap();
        status.get(&tx_id).copied().unwrap_or(TxStatus::Aborted)
    }

    /// Membuat Snapshot Isolation untuk sesi pembacaan
    pub fn create_snapshot(&self) -> Snapshot {
        let status = self.tx_status.read().unwrap();
        let next = *self.next_tx_id.lock().unwrap();

        let mut active_tx = HashSet::new();
        let mut xmin = next;

        for (&tx, &st) in status.iter() {
            if st == TxStatus::Active {
                active_tx.insert(tx);
                if tx < xmin {
                    xmin = tx;
                }
            }
        }

        Snapshot {
            xmin,
            xmax: next,
            active_tx,
        }
    }

    /// Menentukan apakah sebuah baris terlihat oleh transaksi pembaca
    pub fn is_visible(&self, header: &MvccHeader, snapshot: &Snapshot, current_tx_id: u64) -> bool {
        // Jika baris dibuat oleh transaksi saat ini sendiri, selalu terlihat
        if header.xmin == current_tx_id {
            return header.xmax == 0 || header.xmax != current_tx_id;
        }

        // 1. Cek xmin (Apakah pembuat sudah commit?)
        if header.xmin >= snapshot.xmax {
            return false; // Dibuat setelah snapshot diambil
        }
        if snapshot.active_tx.contains(&header.xmin) {
            return false; // Masih aktif saat snapshot diambil
        }
        if self.get_status(header.xmin) != TxStatus::Committed {
            return false; // Transaksi pembuat digagalkan/aborted
        }

        // 2. Cek xmax (Apakah baris sudah dihapus/diubah?)
        if header.xmax == 0 {
            return true; // Belum pernah dihapus
        }
        if header.xmax == current_tx_id {
            return false; // Dihapus oleh transaksi saat ini
        }
        if header.xmax >= snapshot.xmax {
            return true; // Dihapus setelah snapshot diambil -> tetap terlihat
        }
        if snapshot.active_tx.contains(&header.xmax) {
            return true; // Sedang dihapus tapi belum commit saat snapshot -> tetap terlihat
        }
        if self.get_status(header.xmax) == TxStatus::Committed {
            return false; // Sudah resmi dihapus oleh transaksi yang commit sebelum snapshot
        }

        true
    }
}
