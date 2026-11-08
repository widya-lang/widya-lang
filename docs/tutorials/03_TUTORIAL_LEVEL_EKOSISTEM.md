# Panduan & Tutorial Resmi Widya-Lang: Level Ekosistem & Solusi Berdaulat

Dokumen ini menyajikan panduan arsitektur dan implementasi aplikasi terintegrasi tingkat industri menggunakan seluruh komponen ekosistem **Widya-Lang**: **WidyaWeb**, **WidyaDesktop**, **WidyaMobile**, dan mesin basis data mandiri **WidyaBase**.

---

## Bab 1: Lanskap Arsitektur Ekosistem Widya-Lang

Ekosistem Widya-Lang dirancang dengan visi kemandirian teknologi nasional untuk menyediakan tumpukan teknologi lengkap (*full-stack*) tanpa ketergantungan pada vendor asing:

| Modul Ekosistem | Fokus Platform | Fitur Utama | Standar Kedaulatan |
| :--- | :--- | :--- | :--- |
| **WidyaWeb** | Web / Cloud Backend | SSR, Islands Hydration, WebSockets, HTTP/3 | Zero-JS Static, 6-Tier Security |
| **WidyaDesktop** | Windows / Linux Desktop | Native Windowing, Struk Thermal ESC/POS | <15MB RAM, IPC Bridge <1ms |
| **WidyaMobile** | Android / iOS / Tablet | Canvas 2D Software Rasterizer, QRIS ASPI | Paket Mandiri, Standar BI |
| **WidyaBase** | Penyimpanan Data | Relasional ACID B+Tree, PGWire v3.0, DocDB | SQL Dwibahasa Nusantara |
| **WidyaOffice** | Dokumen Digital | DOCX, XLSX, PPTX, PDF ISO 32000-1, EPUB | TTE Tersertifikasi BSrE / X.509 |

---

## Bab 2: WidyaWeb Framework (SSR, Islands, & WebSockets)

WidyaWeb adalah kerangka kerja web non-blocking berkinerja tinggi yang mendukung perenderan sisi server (*Server-Side Rendering*) dengan arsitektur *Islands*. Halaman statis dirender dengan ukuran JavaScript 0 KB di browser.

### 2.1 Menjalankan Server Web & Routing RESTful
```widya
impor ekosistem::web;

misal aplikasi = web::AplikasiWeb::baru();

// Menangani permintaan GET pada rute utama
aplikasi.rute_get("/", fungsi(permintaan, respon) {
    respon.teks_html("<h1>Selamat Datang di Portal WidyaWeb Berdaulat</h1>");
});

// Menangani permintaan API JSON
aplikasi.rute_get("/api/v1/komoditas", fungsi(permintaan, respon) {
    misal daftar = [
        {"id": 1, "nama": "Beras Rojolele", "harga": 14500, "stok": 1200},
        {"id": 2, "nama": "Minyak Goreng Rakyat", "harga": 15000, "stok": 850}
    ];
    respon.json(daftar);
});

// Memulai server pada port 8080 dengan TLS 1.3
aplikasi.mulai(8080);
```

### 2.2 Komunikasi Real-Time WebSockets
```widya
aplikasi.rute_ws("/ws/telemetri", fungsi(koneksi) {
    koneksi.saat_pesan(fungsi(data) {
        cetak("[WS RECV]:", data);
        koneksi.kirim("ACK: Pesan telah dicatat pada sistem berdaulat.");
    });
});
```

---

## Bab 3: WidyaDesktop & Workstation POS Koperasi

WidyaDesktop dirancang untuk mesin kasir, kios informasi publik, dan aplikasi perkantoran dengan konsumsi memori sangat hemat (<15MB RAM footprint) dan latensi antarmuka kurang dari 1 milidetik.

### 3.1 Antarmuka Desktop & Pencetakan Thermal ESC/POS
WidyaDesktop menyediakan driver langsung untuk mencetak nota kasir thermal tanpa perantara spooler eksternal:

```widya
impor ekosistem::desktop;
impor ekosistem::desktop::printer;

// Membuka antarmuka jendela desktop
misal jendela = desktop::Jendela::baru("Sistem Kasir Koperasi Mandiri", 1024, 768);

// Menghasilkan perintah cetak thermal ESC/POS
fungsi cetak_struk_kasir(nomor_transaksi, total_belanja) {
    misal pos = printer::EscPosBuilder::baru();
    pos.inisialisasi();
    pos.atur_perataan(printer::RATA_TENGAH);
    pos.tulis_tebal("KOPERASI UNIT DESA MAKMUR\n");
    pos.tulis_garis("--------------------------------\n");
    pos.atur_perataan(printer::RATA_KIRI);
    pos.tulis("No. Transaksi : " + nomor_transaksi + "\n");
    pos.tulis("Total Belanja : Rp " + ke_teks(total_belanja) + "\n");
    pos.tulis_garis("--------------------------------\n");
    pos.atur_perataan(printer::RATA_TENGAH);
    pos.tulis("Terima Kasih Atas Partisipasi Anda\n\n\n");
    pos.potong_kertas();

    pos.kirim_ke_perangkat("/dev/usb/lp0");
}
```

---

## Bab 4: WidyaMobile: Canvas 2D & Standar QRIS ASPI

WidyaMobile dibangun di atas *pure software canvas rasterizer* mandiri yang mampu merender komponen UI di layar sentuh secara deterministik serta memproses pembayaran QRIS sesuai standar Asosiasi Sistem Pembayaran Indonesia (ASPI).

### 4.1 Parser & Verifikator Payload QRIS
```widya
impor ekosistem::mobile::qris;

misal payload_qris = "00020101021226610014ID.LINKAJA.WWW011893600911000000000151440014ID.GO.KEMENKEU021500000000000000152045812530336054061500005802ID5915KOPERASI WARUNG6007BANDUNG6304A1B2";

misal data_qris = qris::urai(payload_qris);

jika data_qris.apakah_valid() {
    cetak("Nama Merchant :", data_qris.nama_merchant);
    cetak("Kota Merchant :", data_qris.kota);
    cetak("Nominal Bayar : Rp", data_qris.nominal);
    cetak("Kode Pos      :", data_qris.kode_pos);
} selain {
    lempar "Payload QRIS tidak valid atau checksum CRC16 tidak cocok!";
}
```

---

## Bab 5: WidyaBase: Mesin Basis Data Unified Multi-Model

WidyaBase adalah mesin basis data mandiri berkinerja tinggi yang mendukung model Relasional (SQL), Dokumen JSON (NoSQL), dan Object Store (S3-compatible) dalam satu biner terpadu.

### 5.1 Arsitektur B+Tree & Write-Ahead Log (WAL)
WidyaBase menjamin integritas transaksi ACID (*Atomicity, Consistency, Isolation, Durability*) dengan teknik *slotted-page storage* dan pemulihan *crash-recovery* instan.

### 5.2 Sintaksis SQL Dwibahasa (Nusantara & Standar ANSI)
WidyaBase mendukung penulisan kueri dalam Bahasa Indonesia murni maupun sintaksis SQL konvensional:

```sql
-- Sintaks SQL Nusantara (Bahasa Indonesia)
BUAT_TABEL inventaris_koperasi (
    id INT KUNCI_UTAMA,
    kode_barang TEKS,
    nama_barang TEKS,
    harga_satuan ANGKA,
    jumlah_stok INT
);

SISIP_KE inventaris_koperasi (id, kode_barang, nama_barang, harga_satuan, jumlah_stok)
NILAI (1, 'BRG-001', 'Pupuk Organik Granul 50kg', 120000, 250);

PILIH nama_barang, harga_satuan, jumlah_stok 
DARI inventaris_koperasi 
DIMANA harga_satuan <= 150000 DAN jumlah_stok > 50;
```

### 5.3 Kompatibilitas Wire Protocol PGWire v3.0
WidyaBase dapat diakses oleh driver PostgreSQL standar (seperti `pgcli`, `libpq`, `psql`, `npgsql`, `pgx`) pada port `5432`, memudahkan integrasi ke sistem yang telah ada.

---

## Bab 6: WidyaOffice & Otomasi Dokumen Legal

WidyaOffice memungkinkan pembuatan dan konversi dokumen perkantoran (DOCX, XLSX, PPTX, PDF, EPUB) murni dengan Rust tanpa lisensi pihak ketiga.

```bash
# Konversi naskah Markdown menjadi PDF dengan Daftar Isi otomatis
widya office convert naskah.md -o laporan_resmi.pdf --toc -t "Laporan Tahunan"

# Menyematkan Tanda Tangan Elektronik (TTE) & Segel Digital
widya office tte-docx surat_keputusan.docx --penandatangan "Ketua Dewan Pembina"
```

---

## Bab 7: Kepatuhan Keamanan & UU PDP No. 27/2022

Kompiler Widya-Lang menyertakan penganalisis statis Abstract Syntax Tree (AST Linter) untuk mendeteksi paparan Data Pribadi dan secara otomatis melakukan sensor (*redaction*) sebelum masuk ke rekaman log publik:

```widya
impor ekosistem::keamanan::pdp;

misal data_nasabah = {
    "nik": "3404011205900001",
    "nama": "Rahman Santoso",
    "rekening": "1370009988776"
};

// Fungsi sensor otomatis menyamarkan data sensitif sesuai amanat UU PDP
misal data_tersensor = pdp::sensor_data(data_nasabah);
cetak("Log Aman:", data_tersensor);
// Output NIK tersensor: "340401******0001"
// Output Rekening tersensor: "137******8776"
```

---

## Bab 8: Studi Kasus Integrasi Lengkap: Solusi Kasir Koperasi Terpadu

Melalui perpaduan seluruh modul ekosistem:
1. **Frontend Kasir**: Berjalan di mesin touchscreen POS menggunakan **WidyaDesktop** (<15MB RAM).
2. **Penyimpanan Transaksi**: Tersimpan secara ACID di **WidyaBase** dengan replikasi berkas WAL.
3. **Pembayaran Digital**: Menampilkan kode dinamis dan memvalidasi pembayaran melalui **WidyaMobile QRIS**.
4. **Pencetakan Bukti**: Mengirim perintah grafis langsung ke printer kasir thermal USB via ESC/POS.
5. **Pelaporan Finansial**: Menghasilkan berkas PDF audit formal secara otomatis dengan mesin **WidyaOffice**.

Arsitektur ini membuktikan ketangguhan Widya-Lang dalam menghadirkan kedaulatan perangkat lunak terintegrasi yang hemat sumber daya, berkinerja tinggi, dan patuh regulasi nasional.
