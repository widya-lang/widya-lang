## Bab 39: Proyek Sistem 4: Driver Antarmuka Bare-Metal & Kontrol Perangkat Keras

### 39.1 Pemetaan Memori I/O MMIO

Tingkat penguasaan rekayasa sistem yang paling mendasar adalah kemampuan perangkat lunak untuk mengendalikan perangkat keras secara langsung pada tingkat silikon tanpa bantuan sistem operasi (*Bare-Metal Programming*). Komunikasi antara CPU dan perangkat periferal (seperti mikrokontroler pertahanan, chip sensor kedirgantaraan, atau port komunikasi serial) umumnya dilakukan melalui mekanisme **Memory-Mapped I/O (MMIO)**.

Proyek Sistem keempat adalah membangun **Driver Komunikasi Serial UART 16550 Bare-Metal Berdaulat (Widya-UART Driver)** yang mampu membaca dan menulis data langsung ke register perangkat keras fisik.

```mermaid
flowchart TD
    A["Aplikasi Kendali Berdaulat Widya-Lang"] --> B["Lapisan Driver Perangkat Keras Bare-Metal"]
    subgraph MMIOArchitecture["Arsitektur Memory-Mapped I/O (Alamat Fisik 0x10000000)"]
        B -->|Tulis Byte (THR / DLL)| C["Register Pengirim Data: UART_THR (Offset 0)"]
        B -->|Baca Byte (RBR)| D["Register Penerima Data: UART_RBR (Offset 0)"]
        B -->|Periksa Kesiapan (LSR)| E["Register Status Jalur: UART_LSR (Offset 5: THRE / DR)"]
        B -->|Konfigurasi Baud Rate (LCR)| F["Register Kontrol Jalur: UART_LCR (Offset 3: DLAB / 8N1)"]
    end
    C --> G["Pin Fisik Serial TX (Transmisi Gelombang Listrik)"]
    D <-- H["Pin Fisik Serial RX (Penerimaan Gelombang Listrik)"]
```

Peta Register Pengontrol Serial Standar UART 16550:
- **Offset `0x00` (RBR/THR)**: *Receiver Buffer Register* (saat membaca) / *Transmitter Holding Register* (saat menulis).
- **Offset `0x01` (IER)**: *Interrupt Enable Register*.
- **Offset `0x03` (LCR)**: *Line Control Register* (konfigurasi panjang kata 8-bit, parity None, 1 stop bit / 8N1, serta pengaktifan DLAB).
- **Offset `0x05` (LSR)**: *Line Status Register* (Bit 0: Data Ready `DR`; Bit 5: Transmitter Holding Register Empty `THRE`).

### 39.2 Implementasi Kode Program Utuh

Berikut implementasi kode program utuh Driver UART 16550 bare-metal pada Widya-Lang menggunakan blok `tidak_aman` dan manipulasi register fisik:

```widya
// ============================================================================
// PROYEK SISTEM 4: DRIVER BARE-METAL UART 16550 SERIAL HARDWARE
// Arsitektur: Direct Memory-Mapped I/O (MMIO) Registers + Polling Loop
// ============================================================================

modul driver_uart {
    // Alamat Fisik Basis MMIO UART 16550 (Standar QEMU virt / Hardware ARM64)
    konstanta ALAMAT_BASIS_UART: Angka = 0x10000000;

    // Offset Register UART
    konstanta REG_DATA_RBR_THR: Angka = 0; // Transmit / Receive
    konstanta REG_INTERRUPT_IER: Angka = 1;
    konstanta REG_FIFO_FCR: Angka = 2;
    konstanta REG_KONTROL_LCR: Angka = 3;   // Line Control (8N1 & DLAB)
    konstanta REG_STATUS_LSR: Angka = 5;    // Line Status Register

    // Bit Mask Status Jalur (LSR)
    konstanta LSR_DATA_SIAP: Angka = 0x01;  // Bit 0: Ada data yang diterima
    konstanta LSR_PENGIRIM_KOSONG: Angka = 0x20; // Bit 5: Buffer siap kirim

    // Struktur Pengendali Perangkat Keras UART
    struktur PengendaliUART {
        alamat_basis: Angka,

        fungsi inisialisasi() -> PengendaliUART {
            cetak("==================================================================");
            cetak("  MEMULAI DRIVER BARE-METAL UART 16550 HARDWARE CONTROLLER       ");
            cetak("==================================================================");
            cetak("[UART INIT] Mengonfigurasi register fisik pada alamat MMIO: 0x10000000");

            tidak_aman {
                // 1. Nonaktifkan seluruh interupsi hardware
                sys_tulis_memori_langsung(ALAMAT_BASIS_UART + REG_INTERRUPT_IER, 0x00);

                // 2. Aktifkan DLAB (Divisor Latch Access Bit) untuk mengatur Baud Rate
                sys_tulis_memori_langsung(ALAMAT_BASIS_UART + REG_KONTROL_LCR, 0x80);

                // 3. Atur Baud Rate ke 115200 (Divisor = 1 untuk clock 1.8432 MHz)
                sys_tulis_memori_langsung(ALAMAT_BASIS_UART + 0, 0x01); // Divisor Low Byte
                sys_tulis_memori_langsung(ALAMAT_BASIS_UART + 1, 0x00); // Divisor High Byte

                // 4. Nonaktifkan DLAB dan atur mode 8N1 (8 data bit, No parity, 1 stop bit)
                sys_tulis_memori_langsung(ALAMAT_BASIS_UART + REG_KONTROL_LCR, 0x03);

                // 5. Aktifkan dan bersihkan buffer FIFO
                sys_tulis_memori_langsung(ALAMAT_BASIS_UART + REG_FIFO_FCR, 0xC7);
            }

            cetak("[UART INIT] Konfigurasi UART 16550 Sukses: 115200 Baud, 8N1 Aktif.");
            kembalikan PengendaliUART { alamat_basis: ALAMAT_BASIS_UART };
        }

        // Mengirimkan satu byte karakter ke pin TX
        fungsi kirim_byte(diri, data_byte: Angka) {
            tidak_aman {
                // Tunggu hingga bit THRE (Transmitter Empty) bernilai 1
                misal mut status = 0;
                selama (status & LSR_PENGIRIM_KOSONG) == 0 {
                    status = sys_baca_memori_langsung(diri.alamat_basis + REG_STATUS_LSR);
                }
                // Tulis byte ke register pengirim (THR)
                sys_tulis_memori_langsung(diri.alamat_basis + REG_DATA_RBR_THR, data_byte & 0xFF);
            }
        }

        // Mengirimkan untaian string ke konsol serial
        fungsi cetak_serial(diri, teks_pesan: Teks) {
            misal bytes = teks_pesan.ke_larik_byte();
            untuk b in bytes {
                diri.kirim_byte(b);
            }
        }
    }
}

modul utama {
    fungsi utama() {
        // Inisialisasi perangkat keras UART
        misal mut uart = driver_uart::PengendaliUART::inisialisasi();

        // Mengirimkan pesan bootloader langsung ke register perangkat keras
        cetak("\n[TRANSMISI SERIAL] Mengalirkan data ke pin TX hardware...");
        uart.cetak_serial("KEDAULATAN SISTEM BARE-METAL WIDYA AKTIF 100%\r\n");
        uart.cetak_serial("Node Sensor Militer Nusantara Siap Beroperasi.\r\n");

        cetak("[TRANSMISI SERIAL] Selesai: Seluruh byte berhasil dialirkan ke pin serial.");
    }
}
```

### 39.3 Rangkuman Desain & Analisis Rekayasa Sistem

Perbandingan antara Pemrograman Berbasis OS vs Pemrograman Bare-Metal:

| Parameter Rekayasa | Pemrograman Aplikasi Berbasis OS | Pemrograman Bare-Metal (Widya-UART) |
| :--- | :--- | :--- |
| **Ketergantungan Sistem Operasi** | Bergantung penuh pada kernel Linux/Windows | Berjalan Langsung di Silikon (*No OS Required*) |
| **Akses Alamat Memori** | Memori Virtual Terproteksi (*Page Table*) | Alamat Fisik Perangkat Keras Nyata (*Direct MMIO*) |
| **Determinisasi Waktu (Real-Time)** | Non-deterministik (Tergantung scheduler OS) | Deterministik Mutlak Siklus per Siklus Clock |
| **Waktu Booting (Startup)** | Hitungan detik s/d menit | Instan dalam hitungan mikrodetik |
| **Penggunaan Ideal** | Web server, aplikasi desktop, basis data | Bootloader, firmware satelit, pengendali drone |

### 39.4 Latihan Mandiri & Evaluasi Pemahaman Sistem

1. Jelaskan bagaimana mekanisme *Memory-Mapped I/O* (MMIO) memetakan register perangkat keras periferal ke dalam ruang alamat fisik CPU yang sama dengan memori RAM!
2. Mengapa manipulasi register perangkat keras bare-metal wajib menggunakan operasi *volatile memory access* agar kompilator tidak menghilangkan instruksi baca/tulis melalui optimasi penghapusan kode mati (*dead-code elimination*)?
3. Rancanglah fungsi pembacaan serial non-blocking `baca_karakter_serial() -> Mungkin<Angka>` yang memeriksa bit `LSR_DATA_SIAP` sebelum membaca register `UART_RBR`!

### 39.5 Latihan Soal Pilihan Ganda (10 Soal)

1. Mekanisme komunikasi perangkat keras di mana register kontrol dan data periferal dipetakan ke dalam alamat memori fisik yang sama dengan RAM dinamakan...
   A. Memory-Mapped I/O (MMIO)
   B. Port-Mapped I/O Terisolasi
   C. Virtual Memory Paging
   D. Direct Memory Access (DMA)

2. Jenis pengontrol komunikasi serial standar industri yang sangat populer dan disimulasikan pada arsitektur sistem komputer adalah...
   A. UART 16550
   B. Sound Blaster 16
   C. VGA Controller
   D. Floppy Controller

3. Konfigurasi format komunikasi serial standar "8N1" memiliki arti...
   A. 8 data bit, No parity (tanpa paritas), dan 1 stop bit
   B. 8 baud rate, Nilai nol, dan 1 kabel
   C. 8 bit komputer, Jaringan nirkabel, dan 1 antena
   D. 8 core prosesor dan 1 monitor

4. Register pada UART 16550 yang digunakan untuk memantau status apakah buffer pengirim kosong dan siap menerima data baru adalah...
   A. Line Status Register (UART_LSR)
   B. Receiver Buffer Register (UART_RBR)
   C. Interrupt Enable Register (UART_IER)
   D. Scratch Register

5. Mengapa penulisan driver bare-metal pada Widya-Lang wajib dibungkus di dalam blok `tidak_aman`?
   A. Karena melibatkan manipulasi pointer alamat memori fisik mentah yang berada di luar jangkauan pembuktian keselamatan otomatis kompilator
   B. Karena memperlambat koneksi satelit
   C. Karena dilarang oleh lisensi perangkat keras
   D. Agar huruf kode menjadi miring

6. Keuntungan utama dari eksekusi perangkat lunak secara *Bare-Metal* tanpa sistem operasi pada perangkat pertahanan adalah...
   A. Respon waktu nyata yang deterministik mutlak (*Hard Real-Time*) dan ketiadaan overhead sistem operasi
   B. Menghapus kebutuhan baterai listrik
   C. Memperbesar ukuran memori ROM hingga 100 kali
   D. Memformat hard disk secara otomatis

7. Kata kunci atau anotasi compiler yang mencegah optimasi kompilator menghilangkan pembacaan register perangkat keras berulang dinamakan...
   A. `volatile` (Akses Memori Volatil)
   B. `const`
   C. `inline`
   D. `static`

8. Bit khusus pada register Line Control UART (UART_LCR) yang wajib diaktifkan untuk mengatur pembagi frekuensi Baud Rate dinamakan...
   A. DLAB (Divisor Latch Access Bit)
   B. Parity Bit
   C. Break Control Bit
   D. Stop Bit Select

9. Lingkungan pengujian emulator arsitektur perangkat keras sumber terbuka yang sangat populer untuk menjalankan kode bare-metal ARM64 dan RISC-V adalah...
   A. QEMU
   B. VirtualBox
   C. DOSBox
   D. Wine

10. Mengapa penguasaan rekayasa perangkat lunak bare-metal sangat esensial bagi kedaulatan teknologi kedirgantaraan dan militer nasional?
    A. Memungkinkan penciptaan sistem kendali wahana udara nirawak (drone), sistem pemandu rudal, dan mikrokontroler satelit yang mandiri, tahan gangguan, dan bebas dari backdoor sistem operasi asing
    B. Menghapus kebutuhan anggaran riset nasional
    C. Mengurangi bobot fisik pesawat terbang
    D. Membatasi pengembang agar tidak menulis kode baru
