## Lampiran A: Tabel Lengkap Pemetaan Syscall Linux x86_64 & AArch64

Lampiran ini menyajikan tabel referensi resmi sistem panggilan (*System Calls*) kernel Linux untuk arsitektur **x86_64 (AMD64)** dan **AArch64 (ARM64)** yang digunakan dalam pemanggilan *Direct Syscall* bebas libc pada bahasa pemrograman Widya-Lang.

### A.1 Konvensi Register Antarmuka Biner Aplikasi (ABI)

| Parameter Konvensi | Linux x86_64 (System V ABI) | Linux AArch64 (ARM64 ABI) |
| :--- | :--- | :--- |
| **Instruksi CPU Pemanggil** | `syscall` | `svc #0` |
| **Register Nomor Panggilan** | `RAX` | `X8` |
| **Register Argumen 1** | `RDI` | `X0` |
| **Register Argumen 2** | `RSI` | `X1` |
| **Register Argumen 3** | `RDX` | `X2` |
| **Register Argumen 4** | `R10` | `X3` |
| **Register Argumen 5** | `R8` | `X4` |
| **Register Argumen 6** | `R9` | `X5` |
| **Register Nilai Kembalian** | `RAX` | `X0` |
| **Status Galat Kernel** | Nilai negatif antara `-1` s/d `-4095` (`-errno`) | Nilai negatif antara `-1` s/d `-4095` (`-errno`) |

---

### A.2 Tabel Komparasi Nomor Syscall Utama

| Nama Sistem Panggilan | Nomor x86_64 | Nomor AArch64 | Tanda Tangan Parameter Linux Kernel | Deskripsi Fungsional Tingkat Sistem |
| :--- | :---: | :---: | :--- | :--- |
| **`sys_read`** | **0** | **63** | `(unsigned int fd, char *buf, size_t count)` | Membaca byte aliran dari deskriptor berkas/soket |
| **`sys_write`** | **1** | **64** | `(unsigned int fd, const char *buf, size_t count)` | Mengalirkan byte langsung ke kernel/STDOUT |
| **`sys_open` / `openat`** | **2** | **56** | `(int dfd, const char *filename, int flags, umode_t mode)` | Membuka atau membuat berkas pada sistem berkas |
| **`sys_close`** | **3** | **57** | `(unsigned int fd)` | Menutup dan membebaskan deskriptor berkas |
| **`sys_stat` / `newfstatat`** | **4** | **79** | `(int dfd, const char *filename, struct stat *statbuf, int flag)` | Mengambil metadata Inode berkas |
| **`sys_poll` / `ppoll`** | **7** | **73** | `(struct pollfd *ufds, unsigned int nfds, struct timespec *tmo_p, ...)` | Menunggu kesiapan I/O pada kumpulan file descriptor |
| **`sys_lseek`** | **8** | **62** | `(unsigned int fd, off_t offset, unsigned int whence)` | Mengubah posisi penunjuk baca/tulis berkas |
| **`sys_mmap`** | **9** | **222** | `(unsigned long addr, unsigned long len, unsigned long prot, ...)` | Mengalokasikan halaman memori virtual langsung |
| **`sys_mprotect`** | **10** | **226** | `(unsigned long start, size_t len, unsigned long prot)` | Mengatur hak akses proteksi halaman memori (R/W/X) |
| **`sys_munmap`** | **11** | **215** | `(unsigned long addr, size_t len)` | Membebaskan pemetaan halaman memori virtual |
| **`sys_brk`** | **12** | **214** | `(unsigned long brk)` | Mengubah batas alokasi segmen heap proses |
| **`sys_ioctl`** | **16** | **29** | `(unsigned int fd, unsigned int cmd, unsigned long arg)` | Perintah kontrol perangkat keras periferal (MMIO) |
| **`sys_writev`** | **20** | **66** | `(unsigned long fd, const struct iovec *vec, unsigned long vlen)` | Penulisan I/O vektor beberapa buffer sekaligus |
| **`sys_sched_yield`** | **24** | **124** | `(void)` | Menyerahkan giliran kuota waktu CPU ke tugas lain |
| **`sys_nanosleep` / `clock_nanosleep`** | **35** | **115** | `(struct timespec *rqtp, struct timespec *rmtp)` | Menidurkan proses dalam durasi nanodetik presisi |
| **`sys_socket`** | **41** | **198** | `(int family, int type, int protocol)` | Membuat titik akhir komunikasi soket jaringan |
| **`sys_connect`** | **42** | **203** | `(int fd, struct sockaddr *uservaddr, int addrlen)` | Memulai inisiasi sambungan soket ke simpul server |
| **`sys_accept` / `accept4`** | **43** | **242** | `(int fd, struct sockaddr *upeer_sockaddr, int *upeer_addrlen, int flags)` | Mengambil sambungan klien yang telah jabat tangan |
| **`sys_sendto`** | **44** | **206** | `(int fd, void *buff, size_t len, unsigned int flags, ...)` | Mengirimkan paket datagram pada soket (UDP/TCP) |
| **`sys_recvfrom`** | **45** | **207** | `(int fd, void *ubuf, size_t size, unsigned int flags, ...)` | Menerima paket datagram dari soket jaringan |
| **`sys_bind`** | **49** | **200** | `(int fd, struct sockaddr *umyaddr, int addrlen)` | Mengaitkan soket ke alamat IP dan port lokal |
| **`sys_listen`** | **50** | **201** | `(int fd, int backlog)` | Memulai mode pendengaran koneksi pasif |
| **`sys_clone` / `clone3`** | **56** | **220** | `(unsigned long clone_flags, unsigned long newsp, ...)` | Melahirkan proses/utas kernel baru (Multi-Threading) |
| **`sys_exit`** | **60** | **93** | `(int error_code)` | Terminasi proses bersih dengan kode status |
| **`sys_wait4`** | **61** | **260** | `(pid_t upid, int *stat_addr, int options, struct rusage *ru)` | Menunggu perubahan status proses anak |
| **`sys_kill`** | **62** | **129** | `(pid_t pid, int sig)` | Mengirimkan sinyal interupsi ke proses tertentu |
| **`sys_fsync`** | **74** | **82** | `(unsigned int fd)` | Memaksa flushing data kotor page cache ke media disk |
| **`sys_getpid`** | **39** | **172** | `(void)` | Mengambil Process ID (PID) unik proses saat ini |
| **`sys_epoll_create1`** | **291** | **20** | `(int flags)` | Membuat instance I/O multiplexing epoll baru |
| **`sys_epoll_ctl`** | **233** | **21** | `(int epfd, int op, int fd, struct epoll_event *event)` | Menambah/menghapus soket ke tabel pantau epoll |
| **`sys_epoll_wait` / `pwait`** | **232** | **22** | `(int epfd, struct epoll_event *events, int maxevents, int timeout)` | Menunggu kesiapan I/O soket secara non-blocking |
