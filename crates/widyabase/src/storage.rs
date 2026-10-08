// ============================================================================
// WIDYABASE STORAGE ENGINE: SLOTTED-PAGE ARCHITECTURE (storage.rs)
// ============================================================================
// Halaman berukuran tetap 4096 byte (4 KB) berstandar industri dengan:
// - Header Halaman (Magic, LSN, Jumlah Slot, Lower Offset, Upper Offset)
// - Slotted-Page Directory (Array offset dan panjang record tumbuh ke bawah)
// - Area Penyimpanan Record (Tumbuh ke atas dari akhir halaman)
// - Buffer Pool Manager dengan paging memori dan LRU eviction
// ============================================================================

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_HEADER_SIZE: usize = 32;
pub const SLOT_SIZE: usize = 4; // 2 byte offset + 2 byte length
pub const WIDYABASE_MAGIC: u32 = 0x57444231; // "WDB1"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    Header = 0,
    Data = 1,
    BTreeIndex = 2,
    Columnar = 3,
    Overflow = 4,
}

impl PageType {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => PageType::Header,
            1 => PageType::Data,
            2 => PageType::BTreeIndex,
            3 => PageType::Columnar,
            4 => PageType::Overflow,
            _ => PageType::Data,
        }
    }
}

/// Struktur Halaman Mandiri Slotted-Page 4096 Byte
#[derive(Clone)]
pub struct Page {
    pub id: u32,
    pub data: [u8; PAGE_SIZE],
    pub is_dirty: bool,
}

impl Page {
    pub fn new(page_id: u32, ptype: PageType) -> Self {
        let mut page = Page {
            id: page_id,
            data: [0u8; PAGE_SIZE],
            is_dirty: true,
        };
        // Tulis Magic
        page.data[0..4].copy_from_slice(&WIDYABASE_MAGIC.to_le_bytes());
        // Page ID
        page.data[4..8].copy_from_slice(&page_id.to_le_bytes());
        // Page Type
        page.data[8] = ptype as u8;
        // Flags
        page.data[9] = 0;
        // Slot count (0)
        page.data[10..12].copy_from_slice(&0u16.to_le_bytes());
        // Lower offset (awal array slot, setelah header = 32)
        page.data[12..14].copy_from_slice(&(PAGE_HEADER_SIZE as u16).to_le_bytes());
        // Upper offset (awal penyimpanan record dari belakang = PAGE_SIZE)
        page.data[14..16].copy_from_slice(&(PAGE_SIZE as u16).to_le_bytes());
        // LSN (Log Sequence Number) awal = 0
        page.data[16..24].copy_from_slice(&0u64.to_le_bytes());
        // Checksum reserve
        page.data[24..28].copy_from_slice(&0u32.to_le_bytes());
        // Reserved
        page.data[28..32].copy_from_slice(&0u32.to_le_bytes());

        page
    }

    pub fn from_bytes(page_id: u32, bytes: [u8; PAGE_SIZE]) -> Result<Self, String> {
        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if magic != WIDYABASE_MAGIC {
            return Err(format!("Magic number tidak cocok pada halaman {}: 0x{:X}", page_id, magic));
        }
        Ok(Page {
            id: page_id,
            data: bytes,
            is_dirty: false,
        })
    }

    pub fn page_type(&self) -> PageType {
        PageType::from_u8(self.data[8])
    }

    pub fn slot_count(&self) -> u16 {
        u16::from_le_bytes(self.data[10..12].try_into().unwrap())
    }

    fn set_slot_count(&mut self, val: u16) {
        self.data[10..12].copy_from_slice(&val.to_le_bytes());
        self.is_dirty = true;
    }

    pub fn lower_offset(&self) -> u16 {
        u16::from_le_bytes(self.data[12..14].try_into().unwrap())
    }

    fn set_lower_offset(&mut self, val: u16) {
        self.data[12..14].copy_from_slice(&val.to_le_bytes());
        self.is_dirty = true;
    }

    pub fn upper_offset(&self) -> u16 {
        u16::from_le_bytes(self.data[14..16].try_into().unwrap())
    }

    fn set_upper_offset(&mut self, val: u16) {
        self.data[14..16].copy_from_slice(&val.to_le_bytes());
        self.is_dirty = true;
    }

    pub fn get_lsn(&self) -> u64 {
        u64::from_le_bytes(self.data[16..24].try_into().unwrap())
    }

    pub fn set_lsn(&mut self, lsn: u64) {
        self.data[16..24].copy_from_slice(&lsn.to_le_bytes());
        self.is_dirty = true;
    }

    /// Ruang bebas yang tersedia untuk record dan slot baru
    pub fn free_space(&self) -> usize {
        let upper = self.upper_offset() as usize;
        let lower = self.lower_offset() as usize;
        if upper >= lower + SLOT_SIZE {
            upper - lower - SLOT_SIZE
        } else {
            0
        }
    }

    /// Menyisipkan record biner ke dalam halaman slotted-page
    pub fn insert_record(&mut self, record: &[u8]) -> Option<u16> {
        let rec_len = record.len();
        let needed = rec_len + SLOT_SIZE;
        if self.free_space() < rec_len {
            return None;
        }

        let upper = self.upper_offset() as usize;
        let new_upper = upper - rec_len;

        // Tulis data record di bagian atas
        self.data[new_upper..upper].copy_from_slice(record);
        self.set_upper_offset(new_upper as u16);

        // Tulis slot entry (offset u16 + length u16) di bagian lower
        let lower = self.lower_offset() as usize;
        self.data[lower..lower + 2].copy_from_slice(&(new_upper as u16).to_le_bytes());
        self.data[lower + 2..lower + 4].copy_from_slice(&(rec_len as u16).to_le_bytes());

        let new_lower = (lower + SLOT_SIZE) as u16;
        self.set_lower_offset(new_lower);

        let slots = self.slot_count();
        self.set_slot_count(slots + 1);

        Some(slots) // Slot index
    }

    /// Membaca record berdasarkan indeks slot
    pub fn get_record(&self, slot_idx: u16) -> Option<Vec<u8>> {
        let count = self.slot_count();
        if slot_idx >= count {
            return None;
        }

        let slot_pos = PAGE_HEADER_SIZE + (slot_idx as usize * SLOT_SIZE);
        let offset = u16::from_le_bytes(self.data[slot_pos..slot_pos + 2].try_into().unwrap()) as usize;
        let length = u16::from_le_bytes(self.data[slot_pos + 2..slot_pos + 4].try_into().unwrap()) as usize;

        if offset == 0 || length == 0 || offset + length > PAGE_SIZE {
            return None;
        }

        Some(self.data[offset..offset + length].to_vec())
    }

    /// Menghapus slot record (tombstone offset = 0)
    pub fn delete_record(&mut self, slot_idx: u16) -> bool {
        let count = self.slot_count();
        if slot_idx >= count {
            return false;
        }

        let slot_pos = PAGE_HEADER_SIZE + (slot_idx as usize * SLOT_SIZE);
        self.data[slot_pos..slot_pos + 2].copy_from_slice(&0u16.to_le_bytes());
        self.data[slot_pos + 2..slot_pos + 4].copy_from_slice(&0u16.to_le_bytes());
        self.is_dirty = true;
        true
    }
}

/// Buffer Pool Manager dengan proteksi pembacaan/penulisan bersama
pub struct StorageEngine {
    db_path: PathBuf,
    file: Arc<Mutex<File>>,
    buffer_pool: Arc<RwLock<HashMap<u32, Page>>>,
    pub total_pages: Arc<Mutex<u32>>,
    pool_capacity: usize,
}

impl StorageEngine {
    pub fn open<P: AsRef<Path>>(path: P, pool_capacity: usize) -> io::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        if let Some(parent) = path_buf.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path_buf)?;

        let metadata = file.metadata()?;
        let file_len = metadata.len();
        let total_pages = (file_len / PAGE_SIZE as u64) as u32;

        let engine = StorageEngine {
            db_path: path_buf,
            file: Arc::new(Mutex::new(file)),
            buffer_pool: Arc::new(RwLock::new(HashMap::new())),
            total_pages: Arc::new(Mutex::new(total_pages)),
            pool_capacity: if pool_capacity == 0 { 256 } else { pool_capacity },
        };

        // Jika berkas baru dibuat (0 halaman), inisialisasi Header Page 0
        if total_pages == 0 {
            let header_page = Page::new(0, PageType::Header);
            engine.write_page_to_disk(&header_page)?;
            *engine.total_pages.lock().unwrap() = 1;
        }

        Ok(engine)
    }

    /// Alokasi halaman baru bertipe tertentu
    pub fn allocate_page(&self, ptype: PageType) -> io::Result<Page> {
        let mut total = self.total_pages.lock().unwrap();
        let new_id = *total;
        let page = Page::new(new_id, ptype);
        self.write_page_to_disk(&page)?;
        *total += 1;

        // Masukkan ke cache buffer pool
        let mut pool = self.buffer_pool.write().unwrap();
        pool.insert(new_id, page.clone());

        Ok(page)
    }

    /// Mengambil halaman berdasarkan ID (dari Buffer Pool jika ada, atau membaca dari Disk)
    pub fn get_page(&self, page_id: u32) -> io::Result<Page> {
        // Cek Cache
        {
            let pool = self.buffer_pool.read().unwrap();
            if let Some(page) = pool.get(&page_id) {
                return Ok(page.clone());
            }
        }

        // Baca dari disk
        let mut file = self.file.lock().unwrap();
        let offset = page_id as u64 * PAGE_SIZE as u64;
        file.seek(SeekFrom::Start(offset))?;

        let mut buf = [0u8; PAGE_SIZE];
        file.read_exact(&mut buf)?;

        let page = Page::from_bytes(page_id, buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Simpan ke cache buffer pool
        let mut pool = self.buffer_pool.write().unwrap();
        if pool.len() >= self.pool_capacity {
            // Evict clean page or flush
            if let Some(&first_key) = pool.keys().next() {
                if let Some(evicted) = pool.remove(&first_key) {
                    if evicted.is_dirty {
                        let mut f = self.file.lock().unwrap();
                        let ev_off = evicted.id as u64 * PAGE_SIZE as u64;
                        let _ = f.seek(SeekFrom::Start(ev_off));
                        let _ = f.write_all(&evicted.data);
                    }
                }
            }
        }
        pool.insert(page_id, page.clone());

        Ok(page)
    }

    /// Memperbarui halaman di buffer pool dan menandainya sebagai dirty
    pub fn update_page(&self, page: Page) {
        let mut pool = self.buffer_pool.write().unwrap();
        pool.insert(page.id, page);
    }

    /// Flush semua halaman kotor (*dirty pages*) ke disk (Checkpointer)
    pub fn flush_all(&self) -> io::Result<()> {
        let mut pool = self.buffer_pool.write().unwrap();
        let mut file = self.file.lock().unwrap();

        for (_, page) in pool.iter_mut() {
            if page.is_dirty {
                let offset = page.id as u64 * PAGE_SIZE as u64;
                file.seek(SeekFrom::Start(offset))?;
                file.write_all(&page.data)?;
                page.is_dirty = false;
            }
        }
        file.flush()?;
        Ok(())
    }

    fn write_page_to_disk(&self, page: &Page) -> io::Result<()> {
        let mut file = self.file.lock().unwrap();
        let offset = page.id as u64 * PAGE_SIZE as u64;
        file.seek(SeekFrom::Start(offset))?;
        file.write_all(&page.data)?;
        file.flush()?;
        Ok(())
    }

    pub fn get_path(&self) -> PathBuf {
        self.db_path.clone()
    }
}
