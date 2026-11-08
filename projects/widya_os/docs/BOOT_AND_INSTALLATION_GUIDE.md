# Panduan Bootloader, Pembuatan ISO & Instalasi WidyaOS 🚀

> **Spesifikasi Rekayasa Booting Cepat & Installer Grafis 1-Klik**  
> *Hybrid Live USB · Dual-Boot Windows/macOS · Kompatibilitas x86_64 & ARM64*

---

## 1. Pendahuluan

Sistem instalasi WidyaOS dirancang untuk menghilangkan kerumitan teknis instalasi sistem operasi tradisional. Pengguna awam dapat menguji dan memasang WidyaOS dalam hitungan menit, baik sebagai sistem operasi utama maupun berdampingan dengan sistem operasi lama (*Dual-Boot*).

---

## 2. Matriks Kompatibilitas Perangkat Keras

WidyaOS mendukung arsitektur perangkat keras modern:

| Arsitektur | Kategori Perangkat | Spesifikasi Minimum |
| :--- | :--- | :--- |
| **x86_64** | Laptop & PC Desktop (Intel Core i3/i5/i7/i9, AMD Ryzen, Celeron/Athlon) | RAM 2 GB (Disarankan 4 GB+), Penyimpanan 16 GB |
| **AArch64**| Komputer ARM64 (Sovereign Laptop, Raspberry Pi 4/5, Mini PC ARM) | RAM 2 GB, Penyimpanan MicroSD/NVMe 16 GB |
| **UEFI Class 3**| Motherboard Modern Tanpa CSM / Legacy BIOS | Secure Boot Ready (Dukungan Kunci PQC) |

---

## 3. Mode Live USB ("Coba WidyaOS Tanpa Mengubah Harddisk")

Pengguna dapat mencolokkan flashdisk USB yang berisi citra biner `widya-os-enterprise-1.0.0.iso`:
1. Sistem melakukan booting langsung ke memori RAM dalam waktu **< 15 detik** dari USB 3.0.
2. Pengguna langsung masuk ke desktop grafis aktif.
3. Pengguna dapat langsung mencoba mengetik di **WidyaWriter**, mencoba rumus matematika OMML, membuat grafik di **WidyaCalc**, dan **menguji ketiga persona tampilan (Windows, macOS, Ubuntu)** secara nyata tanpa mengubah isi harddisk komputer satu bita pun!

---

## 4. Alur Kerja Installer Grafis 1-Klik (WidyaInstaller)

Jika pengguna memutuskan untuk memasang WidyaOS di komputernya:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        PEMASANG GRAFIS WIDYAOS                         │
├────────────────────────────────────────────────────────────────────────┤
│  [1. Bahasa] ➔ [2. Persona] ➔ [3. Partisi] ➔ [4. Akun] ➔ [5. Selesai]  │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  Langkah 2: Pilih Tampilan Awal Favorit Anda:                          │
│                                                                        │
│   ┌────────────────┐   ┌────────────────┐   ┌────────────────┐         │
│   │   [ Gambar ]   │   │   [ Gambar ]   │   │   [ Gambar ]   │         │
│   │  GAYA WINDOWS  │   │   GAYA MACOS   │   │  GAYA UBUNTU   │         │
│   │ Taskbar Bawah  │   │ Top Menu+Dock  │   │ Dash Vertikal  │         │
│   │ [  PILIH INI ] │   │ [    PILIH   ] │   │ [    PILIH   ] │         │
│   └────────────────┘   └────────────────┘   └────────────────┘         │
│                                                                        │
│  *Catatan: Anda dapat mengubah pilihan ini kapan saja setelah terpasang!│
│                                                                        │
├────────────────────────────────────────────────────────────────────────┤
│ [ < Kembali ]                                         [ Lanjut: Partisi > ]
└────────────────────────────────────────────────────────────────────────┘
```

### Opsi Partisi Cerdas:
1. **Pasang Berdampingan (*Dual-Boot*):** Otomatis mendeteksi partisi Windows atau macOS yang sudah ada, mengecilkan ruang kosong secara aman, dan menambahkan menu pilihan OS saat komputer dinyalakan (*WidyaBoot Menu*).
2. **Gunakan Seluruh Disk (*Clean Install*):** Memformat SSD secara otomatis dengan partisi terenkripsi berstandar industri dan mengaktifkan fitur *A/B Atomic Updates*.

---

## 5. Perintah Pembuatan Berkas Citra ISO

Untuk membangun berkas ISO rilis siap bakar dari repositori Widya:

```bash
# 1. Kompilasi kernel, compositor, shell, dan aplikasi bawaan
cargo build --release --bin widya

# 2. Bangun citra ISO hybrid yang dapat di-flash ke flashdisk
widya os buat-iso \
  --nama "widya-os-enterprise" \
  --arsitektur x86_64 \
  --edisi "Sovereign Desktop" \
  --output ./dist/widya-os-enterprise-1.0.0.iso

# 3. Bakar citra ISO ke USB Drive (Contoh: /dev/sdb)
sudo dd if=./dist/widya-os-enterprise-1.0.0.iso of=/dev/sdb bs=4M status=progress conv=fsync
```

Setelah flashdisk selesai dibuat, komputer siap di-boot dan dijalankan!
