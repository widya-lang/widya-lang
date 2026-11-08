## Bab 6: Manajemen State & Reaktivitas Berbutir Halus (Signals)

### 6.1 Runtuhnya Virtual DOM: Evolusi Menuju Fine-Grained Reactivity

Selama satu dekade terakhir, dunia pengembangan antarmuka web didominasi oleh paradigma **Virtual DOM (VDOM)** yang dipopulerkan oleh framework generasi lama seperti React. Pada pendekatan VDOM:
- Setiap kali suatu variabel state berubah, framework merender ulang seluruh pohon komponen secara virtual.
- Menjalankan algoritma pembandingan (*Diffing Algorithm*) yang membandingkan pohon virtual lama dengan pohon virtual baru ($O(N)$ node) untuk menemukan perbedaan.
- Mengakibatkan pemborosan memori heap, latensi CPU yang tinggi pada ponsel berdaya rendah, dan komputasi render yang sia-sia (*Over-Rendering*).

WidyaWeb meninggalkan Virtual DOM secara total dan mengadopsi paradigma mutakhir **Reaktivitas Berbutir Halus (*Fine-Grained Reactivity*)** berbasis struktur data **Signals & Reactive Dependency Graph** (`reactive.rs`):
- **Sinyal Reaktif (*Reactive Signals*)**: Pembungkus nilai yang secara otomatis melacak fungsi komputasi mana saja yang membacanya (*Auto-Tracking Subscribers*).
- **Pembaruan DOM Bertarget Instan ($O(1)$)**: Ketika nilai sebuah sinyal berubah (misal: jumlah `likes` artikel berita bertambah 1), **hanya satu simpul teks DOM spesifik itu saja yang diperbarui secara langsung di peramban** tanpa merender ulang komponen induk maupun mengeksekusi algoritma diffing VDOM.
- **Kompilasi Graf Ketergantungan Statis**: Graf relasi antar sinyal dianalisis pada fase kompilasi Widya-Lang, menghilangkan beban pelacakan dinamis saat runtime.

```mermaid
flowchart TD
    S1["Sinyal: jumlah_pembaca = 1.420"] --> D1["Efek Reaktif: Update <span> di Navbar"]
    S2["Sinyal: status_breaking_news = 'Gempa Bumi 6.2 SR'"] --> D2["Efek Reaktif: Render Ticker Banner"]
    
    Aksi["Pengguna Mengklik Tombol 'Suka' (Event w-click)"] -->|Mutasi Nilai| S3["Sinyal: total_suka = 850 (+1)"]
    S3 -->|Notifikasi Langsung O(1) Tanpa VDOM Diffing!| D3["Perbarui Langsung DOM: <span id='btn-like-count'>851</span>"]
```

Tiga pilar primitif reaktivitas di WidyaWeb:
1. `Sinyal<T>` (*Signal*): State reaktif primitif yang memegang nilai dan menyediakan metode baca (`baca()`) dan tulis (`atur()`).
2. `Komputasi<T>` (*Derived / Computed Value*): Nilai turunan yang secara otomatis dihitung ulang hanya jika salah satu sinyal yang menjadi sumber dependensinya mengalami perubahan (*Memoized Evaluation*).
3. `Efek` (*Reactive Effect / Watcher*): Prosedur yang dieksekusi secara otomatis sebagai efek samping ketika sinyal di dalamnya berubah (misalnya: memperbarui teks DOM, memicu panggilan jaringan, atau menyimpan riwayat ke penyimpanan lokal).

### 6.2 Implementasi Kode Program Utuh

Berikut implementasi manajemen state reaktif pada komponen pembaca berita interaktif menggunakan WidyaWeb pada Widya-Lang:

```widya
// Modul Manajemen State Reaktif Berbasis Signals WidyaWeb
modul komponen_berita_reaktif {
    impor ekosistem::widyaweb::{
        WidyaWeb, Sinyal, Komputasi, Efek,
        KomponenReaktif, InstruksiPatchDOM
    };

    struktur StateArtikelBerita {
        id_artikel: Bulat,
        judul: Teks,
        total_suka: Sinyal<Bulat>,
        ukuran_huruf_pt: Sinyal<Bulat>,
        status_langganan: Sinyal<Boolean>,
    }

    implementasi StateArtikelBerita {
        fungsi buat(id: Bulat, judul: Teks) -> Diri {
            kembalikan StateArtikelBerita {
                id_artikel: id,
                judul: judul,
                total_suka: Sinyal::buat(124),
                ukuran_huruf_pt: Sinyal::buat(16), // Ukuran font default 16pt
                status_langganan: Sinyal::buat(salah),
            };
        }

        fungsi perbesar_huruf(&mut diri) {
            biar ukuran_kini = diri.ukuran_huruf_pt.baca();
            jika ukuran_kini < 24 {
                diri.ukuran_huruf_pt.atur(ukuran_kini + 2);
                cetak(">>> [REAKTIF] Ukuran huruf diperbesar menjadi: " + teks(ukuran_kini + 2) + "pt");
            }
        }

        fungsi tambah_suka(&mut diri) {
            biar suka_kini = diri.total_suka.baca();
            diri.total_suka.atur(suka_kini + 1);
            cetak(">>> [REAKTIF] Total suka artikel diperbarui langsung: " + teks(suka_kini + 1));
        }
    }

    fungsi demonstrasi_graf_reaktif() -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [SIGNALS] Menguji Reaktivitas Berbutir Halus Komponen Berita...");

        biar mut artikel = StateArtikelBerita::buat(101, "Kedaulatan Digital Indonesia");

        // Definisikan Nilai Komputasi Turunan (Computed Value)
        biar label_popularitas = Komputasi::buat(|| -> Teks {
            biar jumlah = artikel.total_suka.baca();
            jika jumlah > 500 {
                kembalikan "SANGAT POPULER (Trending)";
            } selain_itu jika jumlah > 100 {
                kembalikan "POPULER";
            } selain_itu {
                kembalikan "BIASA";
            }
        });

        // Daftarkan Efek Reaktif Penambal DOM (DOM Patch Effect)
        Efek::buat(|| {
            biar likes = artikel.total_suka.baca();
            biar label = label_popularitas.baca();
            cetak("  ★ [DOM-PATCH-DISPATCH] Mengirim instruksi penambalan targeted:");
            cetak("    - Target Elemen ID: 'counter-likes' -> Nilai Baru: " + teks(likes));
            cetak("    - Target Elemen ID: 'badge-status'  -> Label Baru: " + label);
        });

        // Simulasikan Interaksi Pembaca Mengklik Tombol 'Suka'
        cetak(">>> [USER-ACTION] Pembaca mengklik tombol jempol suka...");
        artikel.tambah_suka(); // Memicu efek reaktif seketika tanpa VDOM Diffing

        // Simulasikan Pembaca Mengatur Aksesibilitas Ukuran Font
        cetak(">>> [USER-ACTION] Pembaca mengklik tombol perbesar teks 'A+'...");
        artikel.perbesar_huruf();

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 6.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan paradigma manajemen state pada framework front end:

| Karakteristik Reaktivitas | Virtual DOM (React / Vue lama) | Dirty Checking (Angular awal) | Fine-Grained Signals (WidyaWeb) |
| :--- | :--- | :--- | :--- |
| **Mekanisme Pelacakan State** | Re-render Pohon Komponen Penuh | Siklus Digest Loop Berkala | **Graf Ketergantungan Sinyal Otomatis** |
| **Kompleksitas Pembaruan DOM**| $O(N)$ Diffing Pohon Komponen | $O(N)$ Pengecekan Seluruh Scope| **$O(1)$ Pembaruan Node Spesifik** |
| **Konsumsi Memori Klien** | Tinggi (Duplikasi Pohon Virtual)| Sedang | **Ultra Hemat (Pointer Langsung)** |
| **Over-Rendering Issue** | Masalah Klasik (Perlu `useMemo`) | Ya | **Nol Mutlak (Hanya node yang berubah)** |
| **Kinerja di Perangkat Seluler** | Sering Lag saat DOM Besar | Lambat | **Super Mulus (Zero Overhead)** |

### 6.4 Latihan Soal Esai

1. Terangkan kelemahan performa dan komputasi dari pendekatan *Virtual DOM Diffing* pada halaman portal berita yang memuat ribuan elemen artikel, widget komentar, dan live ticker!
2. Bagaimana cara kerja konsep *Fine-Grained Reactivity* pada WidyaWeb dalam melacak relasi antara *Signal*, *Computed Value*, dan *Effect* secara otomatis saat kode program dijalankan?
3. Uraikan mengapa pembaruan DOM berorde $O(1)$ berbasis sinyal mampu memberikan pengalaman pengguna yang jauh lebih mulus dan menghemat konsumsi baterai ponsel pintar masyarakat Indonesia!

### 6.5 Soal Pilihan Ganda

1. Paradigma reaktivitas modern yang digunakan oleh WidyaWeb untuk menggantikan Virtual DOM adalah...
   A. Fine-Grained Reactivity berbasis Signals
   B. Manual DOM Refresh
   C. Polling Timer
   D. Full Page Reload

2. Kompleksitas komputasi untuk memperbarui satu elemen teks DOM yang terikat pada sebuah sinyal reaktif di WidyaWeb adalah...
   A. $O(1)$ instan langsung ke simpul yang bersangkutan
   B. $O(N)$ pemindaian seluruh halaman
   C. $O(N^2)$
   D. $O(\log N)$

3. Komponen primitif reaktivitas yang menyimpan nilai state dan secara otomatis memberitahukan perubahannya kepada para pelanggan dinamakan...
   A. Sinyal (Signal)
   B. Cookie
   C. LocalStorage
   D. Socket

4. Nilai yang secara otomatis dihitung ulang hanya jika sinyal sumber dependensinya mengalami perubahan dinamakan...
   A. Nilai Komputasi (Computed / Derived Value)
   B. Nilai Statis
   C. Nilai Acak
   D. Nilai Sampah

5. Masalah klasik pada framework berbasis Virtual DOM di mana komponen anak ikut dirender ulang secara sia-sia meskipun datanya tidak berubah disebut...
   A. Over-Rendering
   B. Buffer Overflow
   C. Deadlock
   D. Packet Drop

6. Prosedur yang dieksekusi secara otomatis sebagai efek samping ketika nilai sinyal di dalamnya mengalami mutasi adalah...
   A. Efek Reaktif (Reactive Effect / Watcher)
   B. Kompilator JIT
   C. Garbage Collector
   D. Task Scheduler

7. Keuntungan graf ketergantungan sinyal dianalisis pada fase kompilasi bahasa Widya-Lang adalah...
   A. Menghilangkan beban pelacakan dinamis saat runtime sehingga kinerja aplikasi semakin cepat
   B. Mematikan fungsi klik tombol
   C. Menghapus kode CSS
   D. Mengurangi resolusi foto berita

8. Atribut pengikat peristiwa interaktif pada sintaksis komponen WidyaWeb untuk menangani peristiwa klik mouse adalah...
   A. `w-click`
   B. `onclick_native`
   C. `v-bind`
   D. `ng-click`

9. Mengapa pemakaian Signals sangat menghemat konsumsi baterai pada perangkat ponsel cerdas?
   A. Karena meniadakan komputasi rekursif pembandingan pohon VDOM di CPU setiap kali terjadi perubahan state kecil
   B. Karena meredupkan layar ponsel otomatis
   C. Karena mematikan koneksi 4G
   D. Karena mengubah warna aplikasi menjadi hitam

10. Penerapan Fine-Grained Signals pada portal berita NusantaraNews memastikan bahwa...
    A. Pembaruan angka live-counter pembaca atau skor pertandingan bola dapat diperbarui seketika tanpa mengganggu kenyamanan membaca teks artikel
    B. Berita lama otomatis terhapus
    C. Seluruh gambar berita diubah menjadi teks
    D. Pengguna harus menginstal aplikasi tambahan
