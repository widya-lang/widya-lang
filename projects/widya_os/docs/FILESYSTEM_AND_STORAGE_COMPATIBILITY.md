# Spesifikasi Sistem Berkas & Matriks Kompatibilitas WidyaOS 💽

> **Arsitektur Sistem Berkas Murni, Lapisan VFS & Interoperabilitas Universal**  
> *WidyaFS Copy-on-Write · Dukungan Universal Partisi Windows, Linux, & macOS*

---

## 1. Sistem Berkas Bawaan Utama: WidyaFS (Sovereign CoW)

WidyaOS menggunakan sistem berkas bawaan buatan mandiri yang dinamakan **WidyaFS** (*Widya Sovereign File System*).

### A. Pilar Karakteristik WidyaFS:
1. **Copy-on-Write (CoW) Modern:**
   - Data yang diperbarui tidak pernah menimpa blok lama secara langsung, melainkan ditulis ke lokasi blok baru sebelum pointer inode dialihkan.
   - **Tahan Mati Lampu Seketika (*Power-loss Resilient*):** Komputer yang mati mendadak saat proses penulisan tidak akan pernah mengalami korupsi sistem berkas (*zero filesystem corruption*).
2. **Integritas Kriptografis Blake3 Per-Blok:**
   - Setiap blok data 4 KB memiliki hash checksum Blake3.
   - Mampu mendeteksi secara otomatis kerusakan perangkat keras (*bit-rot*) atau manipulasi malware tanpa perlu menjalankan `chkdsk` atau `fsck` manual berjam-jam.
3. **Snapshot Instan Tanpa Menghabiskan Ruang (< 1 ms):**
   - Pengguna dapat membuat titik pemulihan sistem (*System Snapshot*) secara instan sebelum memasang aplikasi atau melakukan pembaruan besar.
4. **Partisi Sistem Kebal Modifikasi (*Immutable RootFS dm-verity*):**
   - Partisi sistem inti (`/usr`, `/bin`, `/lib`) di-mount secara *Read-Only* terlindungi pohon hash Merkle kriptografis. Rootkit tidak dapat menyusup ke biner sistem.
5. **Kompresi & Enkripsi Transparan Otomatis:**
   - Partisi pengguna (`/home` dan `/data`) secara otomatis dikompresi menggunakan algoritma Zstandard (Zstd) tingkat 3 (menghemat 30–45% ruang disk NVMe/SSD).
   - Enkripsi volume tingkat perangkat keras (AES-256-XTS atau ChaCha20-Poly1305) terikat langsung ke chip TPM 2.0 / HSM enclave komputer pengguna.
6. **WidyaFS CRDT Terdistribusi:**
   - Folder workspace pengguna (`~/WidyaWorkspace`) mendukung sinkronisasi mesh P2P multi-perangkat tanpa server pusat menggunakan algoritma CRDT (*Conflict-free Replicated Data Types*).

---

## 2. Matriks Kompatibilitas Sistem Berkas Eksternal

WidyaOS dirancang untuk **interoperabilitas 100% tanpa batas**. Ketika pengguna menghubungkan flashdisk USB, harddisk eksternal, kartu memori SD, atau melakukan instalasi berdampingan (*Dual-Boot*) dengan Windows atau macOS, **WidyaOS dapat mengenali, membaca, dan menulis** format berkas lintas-ekosistem melalui lapisan **VFS (Virtual File System)**:

| Sistem Berkas | Asal Ekosistem | Status Baca (*Read*) | Status Tulis (*Write*) | Driver & Mekanisme di WidyaOS |
| :--- | :--- | :---: | :---: | :--- |
| **FAT32** | Universal / USB Lawas | **100% YA** | **100% YA** | Driver FAT native kernel, automount instan. |
| **exFAT** | SD Card / USB Modern | **100% YA** | **100% YA** | Driver exFAT berkinerja tinggi (dukungan berkas > 4 GB). |
| **NTFS** | Microsoft Windows | **100% YA** | **100% YA** | Driver NTFS-3G / kernel NTFS terintegrasi penuh. Baca/tulis partisi Windows C: & D: saat dual-boot. |
| **ext4 / ext3 / ext2** | Linux Standar (Ubuntu/Debian) | **100% YA** | **100% YA** | Driver ext4 native tingkat kernel; sangat cepat dan stabil. |
| **Btrfs** | Linux CoW (Fedora/SUSE) | **100% YA** | **100% YA** | Driver Btrfs native dengan dukungan subvolume & snapshot. |
| **XFS** | Enterprise Linux (RHEL/CentOS) | **100% YA** | **100% YA** | Driver XFS native, cocok untuk storage workstation besar. |
| **ZFS** | OpenZFS / FreeBSD / Solaris | **100% YA** | **100% YA** | Modul OpenZFS / FUSE user-space, akses zpool penuh. |
| **APFS** | macOS Modern (Mojave - Sequoia) | **100% YA** | **Aman (Read-Only)** | Driver APFS murni. Membaca SSD Mac & USB APFS (termasuk partisi terenkripsi dengan kata sandi). *Write dinonaktifkan demi keselamatan metadata Apple*. |
| **HFS+ (Mac OS Extended)** | macOS Warisan (Legacy Mac) | **100% YA** | **YA (Non-Journaled)**| Baca penuh 100%; tulis didukung jika journaling dinonaktifkan. |
| **ISO 9660 / UDF** | CD, DVD, Citra Disk ISO | **100% YA** | Read-Only | Membaca dan me-mount file ISO secara virtual tanpa aplikasi pihak ketiga. |

---

## 3. Fitur Keamanan Khusus Saat Membaca Media Eksternal (USB / Harddisk Luar)

Saat pengguna mencolokkan flashdisk yang sebelumnya digunakan di komputer Windows atau komputer umum:

1. **Mounting Otomatis dengan Flag `noexec`:**
   - Semua partisi eksternal (FAT32, exFAT, NTFS) di-mount dengan proteksi `noexec,nosuid,nodev`.
   - Kode biner berbahaya dari Windows (`.exe`, `.scr`, `.bat`, `.vbs`) tidak akan pernah bisa dieksekusi secara tidak sengaja di WidyaOS.
2. **Kekebalan Virus Shortcut & AutoRun:**
   - Tidak ada fitur autorun/autoplay di WidyaOS.
   - File shortcut jahat (`.lnk`) yang sering menyembunyikan dokumen asli di Windows akan dinetralkan, dan berkas asli pengguna langsung ditampilkan secara transparan di **WidyaExplorer**.
3. **Pemberitahuan Aman Partisi Windows Fast-Startup:**
   - Jika partisi NTFS Windows berada dalam status hibernasi (*Windows Fast-Startup / Dirty Bit*), WidyaOS otomatis mendeteksi dan me-mount partisi dalam mode *Read-Only* untuk mencegah rusaknya data Windows pengguna.
