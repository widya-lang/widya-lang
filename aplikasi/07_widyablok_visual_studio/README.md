# WidyaBlok: Studio Pemrograman Visual Berbasis Blok (Scratch untuk Widya-Lang)

**WidyaBlok** adalah platform lingkungan pemrograman visual (*visual block-based programming*) berbasis drag-and-drop yang dirancang khusus untuk ekosistem **Widya-Lang**. Terinspirasi dari kesuksesan MIT Scratch, WidyaBlok menyajikan antarmuka yang ramah pemula, siswa, dan pengembang untuk mempelajari logika algoritma dan langsung melihat transpilasi kode sumber **Widya-Lang (.wya)** secara real-time.

---

## 🌟 Fitur Utama

1. **Palet Blok Berwarna Standar Scratch & Sentuhan Kedaulatan Widya**:
   - 🔵 **Gerakan (Motion)**: `gerak langkah`, `putar kanan/kiri`, `pergi ke koordinat`, `jika di pinggir pantulkan`, `ubah x/y`.
   - 🟣 **Tampilan (Looks)**: `katakan teks`, `pikirkan`, `ubah ukuran`, `tampilkan`, `sembunyikan`.
   - 🎵 **Suara & Nada (Web Audio Synthesizer)**: `mainkan nada MIDI (C/D/E/G/A)`, `bunyikan gong & kendang`, `efek laser/koin/meow/pop`.
   - 🟡 **Kejadian (Events)**: `ketika bendera hijau diklik 🚩`, `ketika tombol keyboard ditekan`, `ketika sprite diklik`, `siarkan pesan`.
   - 🟠 **Kontrol (Control)**: `tunggu detik`, `ulang N kali`, `selamanya`, `jika kondisi maka`.
   - 🩵 **Sensor (Sensing)**: Deteksi tabrakan tepi/mouse, tanya pengguna & simpan jawaban.
   - 🟢 **Operator & Matematika**: Angka acak rentang, penggabungan teks string.
   - 🔴 **Variabel & Data**: Pembuatan dan manipulasi variabel dinamis dengan *watcher overlay* di layar.
   - 🪄 **Pena Seni Grafika (Pen Graphics)**: Turunkan/angkat pena, atur warna, atur ketebalan, seni fraktal turtle.
   - 🇮🇩 **Blok Khusus Widya**: `cetak ke konsol`, `transliterasi aksara Nusantara`, `kripto SHA256`.

2. **Transpilasi Real-Time ke Kode Asli Widya-Lang (.wya)**:
   - Setiap kali blok disusun atau parameternya diubah, tab *Transpilasi Kode Widya-Lang* secara otomatis memperbarui representasi kode murni yang siap diekspor.

3. **Panggung Interaktif (Canvas Stage 480x360)**:
   - Multi-Sprite (Garuda Widya, Kucing Oranye, Cenderawasih, Apel, Bintang, Roket, Robot).
   - Pengaturan latar belakang (Grid X-Y, Galaksi, Panggung Teater, Pantai/Alam Tropis, Polos).
   - Gelembung percakapan (*speech & think bubbles*).
   - Mode Turbo & Grid Koordinat.

4. **Koleksi Contoh Proyek Interaktif Bawaan**:
   1. 🐱 **Tari & Animasi Kucing Widya**
   2. 🍎 **Game Tangkap Apel Merah Putih & Skor**
   3. 🌀 **Seni Geometri Bintang & Pena Fraktal**
   4. 🎹 **Sintesis Melodi Musik Gamelan & Piano**
   5. 🤖 **Logika Navigasi Robot & Sensor Cerdas**

5. **Simpan, Muat, & Ekspor**:
   - Simpan proyek dalam format `.json`.
   - Ekspor kode Widya murni ke berkas `.wya`.

---

## 🚀 Cara Menjalankan

Buka peramban web dan akses:
```
http://localhost:8080/07_widyablok_visual_studio/index.html
```
atau buka langsung berkas `index.html` di browser modern mana pun (Google Chrome, Firefox, Safari, Edge).
