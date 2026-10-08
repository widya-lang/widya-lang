//! Sovereign Slab Memory Allocator (Pilar III: Alokator Memori Mandiri Bebas Malloc)
//! Mengelola memori tumpukan (*heap*) mandiri menggunakan halaman virtual direct-kernel mmap.
//! Mendukung alokasi berukuran tetap (Slab Classes) berkinerja tinggi O(1) tanpa overhead jemalloc/libc.

use super::syscall::DirectSyscall;
use std::sync::Mutex;

const SLAB_CLASSES: [usize; 7] = [32, 64, 128, 256, 512, 1024, 2048];
const PAGE_SIZE: usize = 4096;

pub struct SlabBlock {
    pub next: *mut SlabBlock,
}

pub struct SlabPool {
    pub block_size: usize,
    pub free_list: *mut SlabBlock,
    pub allocated_blocks: usize,
    pub total_blocks: usize,
    pub pages_held: Vec<*mut u8>,
}

unsafe impl Send for SlabPool {}
unsafe impl Sync for SlabPool {}

impl SlabPool {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size,
            free_list: std::ptr::null_mut(),
            allocated_blocks: 0,
            total_blocks: 0,
            pages_held: Vec::new(),
        }
    }

    pub fn grow(&mut self) -> Result<(), String> {
        let page = DirectSyscall::allocate_pages(PAGE_SIZE)?;
        self.pages_held.push(page);

        let blocks_in_page = PAGE_SIZE / self.block_size;
        for i in 0..blocks_in_page {
            let block_ptr = unsafe { page.add(i * self.block_size) as *mut SlabBlock };
            unsafe {
                (*block_ptr).next = self.free_list;
                self.free_list = block_ptr;
            }
        }
        self.total_blocks += blocks_in_page;
        Ok(())
    }

    pub fn allocate(&mut self) -> Result<*mut u8, String> {
        if self.free_list.is_null() {
            self.grow()?;
        }

        if self.free_list.is_null() {
            return Err("Memori habis pada slab pool".to_string());
        }

        let block = self.free_list;
        unsafe {
            self.free_list = (*block).next;
        }
        self.allocated_blocks += 1;
        Ok(block as *mut u8)
    }

    pub fn deallocate(&mut self, ptr: *mut u8) {
        let block = ptr as *mut SlabBlock;
        unsafe {
            (*block).next = self.free_list;
            self.free_list = block;
        }
        if self.allocated_blocks > 0 {
            self.allocated_blocks -= 1;
        }
    }
}

pub struct SovereignAllocator {
    pools: Vec<Mutex<SlabPool>>,
}

impl SovereignAllocator {
    pub fn new() -> Self {
        let mut pools = Vec::with_capacity(SLAB_CLASSES.len());
        for &size in &SLAB_CLASSES {
            pools.push(Mutex::new(SlabPool::new(size)));
        }
        Self { pools }
    }

    pub fn alloc(&self, size: usize) -> Result<*mut u8, String> {
        for (i, &class_size) in SLAB_CLASSES.iter().enumerate() {
            if size <= class_size {
                let mut pool = self.pools[i].lock().unwrap();
                return pool.allocate();
            }
        }

        // Alokasi berukuran besar (> 2048 byte): gunakan halaman mmap langsung
        let pages_needed = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        DirectSyscall::allocate_pages(pages_needed * PAGE_SIZE)
    }

    pub fn dealloc(&self, ptr: *mut u8, size: usize) {
        if ptr.is_null() {
            return;
        }

        for (i, &class_size) in SLAB_CLASSES.iter().enumerate() {
            if size <= class_size {
                let mut pool = self.pools[i].lock().unwrap();
                pool.deallocate(ptr);
                return;
            }
        }

        // Alokasi besar langsung dibebaskan via munmap
        let pages_needed = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        let _ = DirectSyscall::free_pages(ptr, pages_needed * PAGE_SIZE);
    }

    pub fn get_stats(&self) -> String {
        let mut report = String::from("Statistik Alokator Berdaulat (Zero Libc):\n");
        for (i, &class_size) in SLAB_CLASSES.iter().enumerate() {
            let pool = self.pools[i].lock().unwrap();
            report.push_str(&format!(
                "  Slab {:>4}B : Digunakan {:>4} / {:>4} blok (Halaman: {})\n",
                class_size,
                pool.allocated_blocks,
                pool.total_blocks,
                pool.pages_held.len()
            ));
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_slab_alloc_and_free() {
        let alloc = SovereignAllocator::new();

        // Uji alokasi blok kecil (32-byte)
        let ptr1 = alloc.alloc(24).expect("Gagal alokasi 24B");
        let ptr2 = alloc.alloc(32).expect("Gagal alokasi 32B");
        assert_ne!(ptr1, ptr2);

        unsafe {
            *ptr1 = 0xAA;
            *ptr2 = 0xBB;
            assert_eq!(*ptr1, 0xAA);
            assert_eq!(*ptr2, 0xBB);
        }

        alloc.dealloc(ptr1, 24);
        alloc.dealloc(ptr2, 32);

        // Alokasi ulang harus mendaur ulang blok sebelumnya
        let ptr3 = alloc.alloc(32).expect("Gagal alokasi ulang");
        assert!(!ptr3.is_null());
        alloc.dealloc(ptr3, 32);

        let stats = alloc.get_stats();
        assert!(stats.contains("Statistik Alokator Berdaulat"));
    }
}
