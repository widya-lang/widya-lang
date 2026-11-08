//! Bare-Metal & Freestanding Core Subsystem for Widya-Lang (widya::inti & widya::alokasi)
//! Menyediakan abstraksi zero-allocation, pointer mentah hardware, MMIO register volatil,
//! antarmuka alokator memori kustom, dan panic handler mandiri bebas libc/OS (setara #![no_std] di Rust).

use std::fmt;

/// Tata letak memori (Memory Layout: Ukuran dan Penyelarasan/Alignment)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMemori {
    pub ukuran: usize,
    pub penyelarasan: usize,
}

impl LayoutMemori {
    pub const fn baru(ukuran: usize, penyelarasan: usize) -> Result<Self, &'static str> {
        if penyelarasan == 0 || (penyelarasan & (penyelarasan - 1)) != 0 {
            return Err("Penyelarasan memori harus berupa bilangan pangkat dua dan bukan nol");
        }
        Ok(Self { ukuran, penyelarasan })
    }

    pub const fn dari_tipe<T>() -> Self {
        Self {
            ukuran: std::mem::size_of::<T>(),
            penyelarasan: std::mem::align_of::<T>(),
        }
    }
}

/// Pointer Mentah Aman untuk Akses Hardware / Register MMIO (*konst T dan *mut T)
#[derive(Debug, PartialEq, Eq)]
pub struct PenunjukMentah<T> {
    pub ptr: *mut T,
}

impl<T> Copy for PenunjukMentah<T> {}
impl<T> Clone for PenunjukMentah<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PenunjukMentah<T> {
    pub const fn dari_alamat(alamat: usize) -> Self {
        Self {
            ptr: alamat as *mut T,
        }
    }

    pub const fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn alamat(&self) -> usize {
        self.ptr as usize
    }

    /// Geser pointer sebesar N elemen
    pub unsafe fn geser(&self, count: isize) -> Self {
        unsafe {
            Self {
                ptr: self.ptr.offset(count),
            }
        }
    }

    /// Baca nilai dari register hardware secara volatil (mencegah compiler optimization drop)
    pub unsafe fn baca_volatil(&self) -> T
    where
        T: Copy,
    {
        unsafe { std::ptr::read_volatile(self.ptr) }
    }

    /// Tulis nilai ke register hardware secara volatil
    pub unsafe fn tulis_volatil(&self, val: T) {
        unsafe { std::ptr::write_volatile(self.ptr, val) }
    }
}

/// Irisan Memori Tanpa Alokasi Heap (Freestanding Slice &[T])
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Irisan<'a, T> {
    pub data: PenunjukMentah<T>,
    pub panjang: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Irisan<'a, T> {
    pub fn dari_pointer_mentah(data: PenunjukMentah<T>, panjang: usize) -> Self {
        Self {
            data,
            panjang,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.panjang
    }

    pub fn is_empty(&self) -> bool {
        self.panjang == 0
    }

    pub fn ambil(&self, index: usize) -> Option<&'a T> {
        if index < self.panjang {
            unsafe {
                let elem_ptr = self.data.ptr.add(index);
                Some(&*elem_ptr)
            }
        } else {
            None
        }
    }
}

/// Antarmuka Alokator Memori Mandiri (Setara Trait GlobalAlloc / Allocator di Rust)
pub trait AntarmukaAlokator {
    /// Alokasi blok memori dengan tata letak tertentu
    unsafe fn alokasi(&mut self, layout: LayoutMemori) -> Option<PenunjukMentah<u8>>;

    /// Bebaskan blok memori yang telah dialokasikan
    unsafe fn dealokasi(&mut self, ptr: PenunjukMentah<u8>, layout: LayoutMemori);
}

/// Alokator Bump Statis (Zero-Libc / Zero-OS Static Arena Allocator)
/// Sangat ideal untuk kernel, mikrokontroler (Cortex-M/ESP32), dan bootloader UEFI
pub struct AlokatorTumpukanStatik<const N: usize> {
    buffer: [u8; N],
    offset: usize,
    allocated_blocks: usize,
}

impl<const N: usize> AlokatorTumpukanStatik<N> {
    pub const fn baru() -> Self {
        Self {
            buffer: [0u8; N],
            offset: 0,
            allocated_blocks: 0,
        }
    }

    pub fn kapasitas(&self) -> usize {
        N
    }

    pub fn terpakai(&self) -> usize {
        self.offset
    }

    pub fn sisa(&self) -> usize {
        N.saturating_sub(self.offset)
    }

    pub fn reset(&mut self) {
        self.offset = 0;
        self.allocated_blocks = 0;
    }
}

impl<const N: usize> AntarmukaAlokator for AlokatorTumpukanStatik<N> {
    unsafe fn alokasi(&mut self, layout: LayoutMemori) -> Option<PenunjukMentah<u8>> {
        // Penyelarasan alamat (alignment)
        let current_ptr = unsafe { self.buffer.as_mut_ptr().add(self.offset) as usize };
        let align_mask = layout.penyelarasan - 1;
        let padding = if (current_ptr & align_mask) == 0 {
            0
        } else {
            layout.penyelarasan - (current_ptr & align_mask)
        };

        let new_offset = self.offset + padding + layout.ukuran;
        if new_offset <= N {
            let result_ptr = unsafe { self.buffer.as_mut_ptr().add(self.offset + padding) };
            self.offset = new_offset;
            self.allocated_blocks += 1;
            Some(PenunjukMentah { ptr: result_ptr })
        } else {
            None // Kehabisan memori statis
        }
    }

    unsafe fn dealokasi(&mut self, _ptr: PenunjukMentah<u8>, _layout: LayoutMemori) {
        // Alokator bump membebaskan memori secara massal via reset(), bukan per-pointer
    }
}

/// Kotak Pintar Alokasi Heap Bebas-OS (Freestanding Box<T>)
pub struct Kotak<T, A: AntarmukaAlokator> {
    ptr: PenunjukMentah<T>,
    layout: LayoutMemori,
    alokator: *mut A,
}

impl<T, A: AntarmukaAlokator> Kotak<T, A> {
    pub fn baru_in(val: T, alokator: &mut A) -> Option<Self> {
        let layout = LayoutMemori::dari_tipe::<T>();
        unsafe {
            let raw_mem = alokator.alokasi(layout)?;
            let typed_ptr = PenunjukMentah {
                ptr: raw_mem.ptr as *mut T,
            };
            std::ptr::write(typed_ptr.ptr, val);

            Some(Self {
                ptr: typed_ptr,
                layout,
                alokator: alokator as *mut A,
            })
        }
    }

    pub fn nilai(&self) -> &T {
        unsafe { &*self.ptr.ptr }
    }

    pub fn nilai_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.ptr }
    }
}

impl<T, A: AntarmukaAlokator> Drop for Kotak<T, A> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr.ptr);
            let raw_u8 = PenunjukMentah {
                ptr: self.ptr.ptr as *mut u8,
            };
            (*self.alokator).dealokasi(raw_u8, self.layout);
        }
    }
}

/// Larik Dinamis Bebas-OS (Freestanding Vec<T>)
pub struct LarikDinamis<T, A: AntarmukaAlokator> {
    ptr: PenunjukMentah<T>,
    panjang: usize,
    kapasitas: usize,
    alokator: *mut A,
}

impl<T, A: AntarmukaAlokator> LarikDinamis<T, A> {
    pub fn baru_in(alokator: &mut A, kapasitas_awal: usize) -> Option<Self> {
        let layout = LayoutMemori::baru(
            std::mem::size_of::<T>() * kapasitas_awal.max(1),
            std::mem::align_of::<T>(),
        ).ok()?;

        unsafe {
            let raw_mem = alokator.alokasi(layout)?;
            Some(Self {
                ptr: PenunjukMentah { ptr: raw_mem.ptr as *mut T },
                panjang: 0,
                kapasitas: kapasitas_awal.max(1),
                alokator: alokator as *mut A,
            })
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.panjang >= self.kapasitas {
            return Err("Kapasitas LarikDinamis freestanding terlampaui");
        }
        unsafe {
            let slot = self.ptr.ptr.add(self.panjang);
            std::ptr::write(slot, val);
            self.panjang += 1;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.panjang
    }

    pub fn is_empty(&self) -> bool {
        self.panjang == 0
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.panjang {
            unsafe { Some(&*self.ptr.ptr.add(idx)) }
        } else {
            None
        }
    }
}

impl<T, A: AntarmukaAlokator> Drop for LarikDinamis<T, A> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.panjang {
                std::ptr::drop_in_place(self.ptr.ptr.add(i));
            }
            let layout = LayoutMemori::baru(
                std::mem::size_of::<T>() * self.kapasitas,
                std::mem::align_of::<T>(),
            ).unwrap();
            let raw_u8 = PenunjukMentah { ptr: self.ptr.ptr as *mut u8 };
            (*self.alokator).dealokasi(raw_u8, layout);
        }
    }
}

/// Pengendali Kepanikan Mandiri Bare-Metal (Bare-Metal Panic Hook)
pub trait TanganKepanikan {
    fn tangani_kepanikan(pesan: &str, berkas: &str, baris: usize) -> !;
}

/// Handler Kepanikan Default: Hentikan CPU dalam Loop Tak Hingga (hlt)
pub struct HentiSistemKepanikan;

impl TanganKepanikan for HentiSistemKepanikan {
    fn tangani_kepanikan(pesan: &str, berkas: &str, baris: usize) -> ! {
        // Pada bare-metal x86_64/ARM, loop hlt untuk mencegah kerusakan memori lebih lanjut
        loop {
            std::hint::spin_loop();
        }
    }
}
