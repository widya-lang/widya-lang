//! Sovereign Raw Kernel Syscall Interface (Pilar III: Direct Syscalls Bebas Libc)
//! Mengakses langsung kernel Linux x86_64 tanpa melalui perantara glibc/musl C runtime.

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub const PROT_READ: i32 = 0x1;
pub const PROT_WRITE: i32 = 0x2;
pub const MAP_PRIVATE: i32 = 0x02;
pub const MAP_ANONYMOUS: i32 = 0x20;

pub struct DirectSyscall;

impl DirectSyscall {
    /// Syscall 1: sys_write(fd, buf, count)
    #[inline(always)]
    pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
        let ret: isize;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") 1isize,
                in("rdi") fd as isize,
                in("rsi") buf,
                in("rdx") count,
                lateout("rax") ret,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            ret = count as isize;
        }
        ret
    }

    /// Syscall 0: sys_read(fd, buf, count)
    #[inline(always)]
    pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
        let ret: isize;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") 0isize,
                in("rdi") fd as isize,
                in("rsi") buf,
                in("rdx") count,
                lateout("rax") ret,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            ret = 0;
        }
        ret
    }

    /// Syscall 9: sys_mmap(addr, len, prot, flags, fd, offset)
    #[inline(always)]
    pub unsafe fn mmap(
        addr: *mut u8,
        length: usize,
        prot: i32,
        flags: i32,
        fd: i32,
        offset: i64,
    ) -> *mut u8 {
        let ret: *mut u8;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") 9isize,
                in("rdi") addr,
                in("rsi") length,
                in("rdx") prot as isize,
                in("r10") flags as isize,
                in("r8") fd as isize,
                in("r9") offset,
                lateout("rax") ret,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            ret = std::ptr::null_mut();
        }
        ret
    }

    /// Syscall 11: sys_munmap(addr, len)
    #[inline(always)]
    pub unsafe fn munmap(addr: *mut u8, length: usize) -> isize {
        let ret: isize;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") 11isize,
                in("rdi") addr,
                in("rsi") length,
                lateout("rax") ret,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            ret = 0;
        }
        ret
    }

    /// Syscall 39: sys_getpid()
    #[inline(always)]
    pub unsafe fn getpid() -> i32 {
        let ret: isize;
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") 39isize,
                lateout("rax") ret,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            ret = 1;
        }
        ret as i32
    }

    /// Cetak string langsung ke stdout (FD 1) tanpa perantara libc
    pub fn print_stdout(text: &str) -> isize {
        unsafe { Self::write(1, text.as_ptr(), text.len()) }
    }

    /// Cetak string langsung ke stderr (FD 2) tanpa perantara libc
    pub fn print_stderr(text: &str) -> isize {
        unsafe { Self::write(2, text.as_ptr(), text.len()) }
    }

    /// Alokasi memori halaman virtual anonim langsung dari kernel via mmap (Zero libc malloc)
    pub fn allocate_pages(bytes: usize) -> Result<*mut u8, String> {
        let ptr = unsafe {
            Self::mmap(
                std::ptr::null_mut(),
                bytes,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        // Kernel Linux mengembalikan -4095..-1 jika terjadi galat mmap
        let val = ptr as usize;
        if val > usize::MAX - 4096 {
            return Err("Gagal alokasi halaman kernel via sys_mmap".to_string());
        }
        Ok(ptr)
    }

    /// Bebaskan halaman virtual ke kernel via munmap
    pub fn free_pages(ptr: *mut u8, bytes: usize) -> Result<(), String> {
        let ret = unsafe { Self::munmap(ptr, bytes) };
        if ret < 0 {
            return Err(format!("Gagal membebaskan halaman mmap: kode {}", ret));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_syscall_getpid() {
        let pid = unsafe { DirectSyscall::getpid() };
        assert!(pid > 0, "PID harus bilangan bulat positif");
    }

    #[test]
    fn test_direct_mmap_and_munmap() {
        let page_size = 4096;
        let ptr = DirectSyscall::allocate_pages(page_size).expect("Gagal mmap halaman kernel");
        assert!(!ptr.is_null());

        // Uji tulis dan baca langsung ke halaman mmap
        unsafe {
            let slice = std::slice::from_raw_parts_mut(ptr, page_size);
            slice[0] = 0x42;
            slice[4095] = 0x99;
            assert_eq!(slice[0], 0x42);
            assert_eq!(slice[4095], 0x99);
        }

        DirectSyscall::free_pages(ptr, page_size).expect("Gagal membebaskan halaman kernel");
    }
}
