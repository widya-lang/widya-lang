## Bab 16: Pemicu Transaksi & Audit Jejak Kepatuhan

### 16.1 Integritas Otomatis: Trigger, Constraint & Jejak Audit Finansial

Dalam sistem komputasi berdaulat skala nasional—seperti perbendaharaan negara, sistem perbankan terpusat, dan rekam medis kependudukan—integritas data dan akuntabilitas hukum tidak boleh hanya diserahkan pada disiplin logika aplikasi klien. Aplikasi klien dapat mengandung celah keamanan, salah konfigurasi, atau dieksploitasi oleh pihak tak berwenang.

Oleh karena itu, WidyaBase menanamkan **Mesin Pemicu Transaksi (*Trigger Engine*)** dan **Sistem Audit Kepatuhan Otomatis (*Compliance Audit Subsystem*)** langsung pada lapisan inti mesin basis data:
- **Pemicu (*Triggers*)**: Prosedur logika yang secara otomatis terpicu sebelum (*BEFORE*) atau sesudah (*AFTER*) operasi mutasi data (`SISIP_KE`, `PERBARUI`, `HAPUS_DARI`) pada tingkat pernyataan (*statement-level*) maupun per baris (*row-level / FOR EACH ROW*).
- **Variabel Semu Spesial**: Menyediakan akses instan ke tupel `LAMA` (*OLD*) dan tupel `BARU` (*NEW*).
- **Jejak Audit Kekal (*Immutable Append-Only Audit Trail*)**: Setiap perubahan data finansial dicatat ke tabel audit terenkripsi yang dilengkapi hash kriptografi rantai blok (*blockchain-like hash chain*), menjamin data tidak dapat dimanipulasi bahkan oleh administrator basis data (*non-repudiation*).

```mermaid
flowchart TD
    A["Kueri Klien: PERBARUI rekening SET saldo = ..."] --> B["Tahap 1: Evaluasi Pemicu SEBELUM (BEFORE Trigger)"]
    subgraph MesinTriggerWidyaBase["Mesin Pemicu & Integritas Transaksi WidyaBase (trigger.rs)"]
        B -->|Validasi Nilai Tupel BARU & LAMA| C{"Apakah Saldo Valid & >= 0?"}
        C -->|Tidak (Melanggar Batasan)| D["Batalkan Transaksi & Bangkitkan Galat Integritas"]
        C -->|Ya (Valid)| E["Tahap 2: Tulis Pembaruan ke Buffer Pool & WAL Engine"]
        E --> F["Tahap 3: Evaluasi Batasan Integritas Kunci Asing & Unik"]
        F --> G["Tahap 4: Eksekusi Pemicu SESUDAH (AFTER Trigger)"]
        G --> H["Tahap 5: Sisipkan Catatan Immutable Audit Log Berantai Hash"]
    end
    H --> I["Kemit Transaksi Sukses Paripurna"]
```

Pilar arsitektur audit WidyaBase:
1. **Pemicu Pencegahan (*BEFORE Trigger*)**: Mampu mengubah nilai tupel `BARU` sebelum ditulis ke disk (misalnya pembersihan format nomor HP, enkripsi kolom NIK), atau menggugurkan transaksi jika terdeteksi manipulasi saldo negatif.
2. **Pemicu Propagasi (*AFTER Trigger*)**: Digunakan untuk menyinkronkan data turunan, mengirim notifikasi peristiwa perubahan data, atau memperbarui agregasi material.
3. **Pemberlakuan Kunci Asing Berjenjang (*Cascading Constraints*)**: Mendukung aksi penanganan otomatis saat data induk dihapus: `HAPUS_BERUNTUN` (*CASCADE*), `ATUR_KOSONG` (*SET NULL*), atau `TOLAK` (*RESTRICT*).

### 16.2 Implementasi Kode Program Utuh

Berikut implementasi definisi Trigger audit saldo keuangan dan verifikasi integritas rantai audit pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Pemicu Transaksi dan Jejak Audit Kepatuhan WidyaBase
modul mesin_audit_trigger {
    impor ekosistem::widyabase::{
        WidyaBase, KonteksPemicu, WaktuPemicu, PeristiwaPemicu,
        BarisData, HasilPemicu, KriptoHash
    };

    fungsi inisialisasi_pemicu_audit(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [TRIGGER] Mengonfigurasi Pemicu Audit Rekening Perbankan...");

        // 1. Buat Tabel Audit Append-Only dengan Rantai Hash SHA-256
        biar ddl_audit = "
            BUAT TABEL log_audit_rekening (
                id BULAT KUNCI_UTAMA_SERI,
                no_rek TEKS TIDAK_KOSONG,
                saldo_lama DESIMAL,
                saldo_baru DESIMAL,
                waktu_kejadian CAP_WAKTU,
                diubah_oleh TEKS,
                hash_sebelumnya TEKS,
                hash_sekarang TEKS
            );
        ";
        mesin.eksekusi_sql(ddl_audit)?;

        // 2. Registrasikan Pemicu 'BEFORE UPDATE' untuk Menolak Saldo Negatif
        biar sql_trigger_before = "
            BUAT PEMICU validasi_saldo_minimum
            SEBELUM PERBARUI PADA rekening
            UNTUK SETIAP BARIS
            JIKA (BARU.saldo < 0.0)
            MAKA
                BANGKITKAN_GALAT('GalatIntegritas: Saldo rekening tidak boleh bernilai negatif!');
        ";
        mesin.eksekusi_sql(sql_trigger_before)?;
        cetak(">>> [TRIGGER] Pemicu validasi integritas SEBELUM berhasil didaftarkan.");

        // 3. Registrasikan Pemicu 'AFTER UPDATE' untuk Mencatat Jejak Audit Otomatis
        biar sql_trigger_after = "
            BUAT PEMICU catat_audit_keuangan
            SESUDAH PERBARUI PADA rekening
            UNTUK SETIAP BARIS
            JALANKAN
                SISIP_KE log_audit_rekening (
                    no_rek, saldo_lama, saldo_baru, waktu_kejadian, diubah_oleh
                )
                NILAI (
                    LAMA.no_rek, LAMA.saldo, BARU.saldo, SEKARANG(), PENGGUNA_AKTIF()
                );
        ";
        mesin.eksekusi_sql(sql_trigger_after)?;
        cetak(">>> [TRIGGER] Pemicu audit SESUDAH berhasil didaftarkan.");

        // 4. Uji Mutasi Transaksional: Transaksi Berhasil
        cetak(">>> [TX] Menjalankan pembaruan saldo nasabah sah...");
        mesin.eksekusi_sql("PERBARUI rekening SET saldo = saldo - 500000 WHERE no_rek = 'REK-101';")?;
        
        // 5. Uji Mutasi Transaksional: Penolakan Saldo Minus
        cetak(">>> [TX] Menguji proteksi transaksi penarikan melebihi batas saldo...");
        biar hasil_ilegal = mesin.eksekusi_sql("PERBARUI rekening SET saldo = saldo - 99999999 WHERE no_rek = 'REK-101';");
        jika hasil_ilegal.apakah_galat() {
            cetak(">>> [PROTEKSI SUKSES] Transaksi ilegal digugurkan oleh Trigger: " + hasil_ilegal.ambil_pesan_galat());
        }

        cetak(">>> [AUDIT] Memeriksa isi catatan log audit kepatuhan perbankan...");
        biar log = mesin.kueri_sql("PILIH * DARI log_audit_rekening URUTKAN_BERDASARKAN id;")?;
        untuk baris di log.baris_data() {
            cetak("  [AUDIT] Rek: " + baris.ambil_teks("no_rek") + 
                  " | Saldo Lama: Rp " + teks(baris.ambil_desimal("saldo_lama")) +
                  " -> Saldo Baru: Rp " + teks(baris.ambil_desimal("saldo_baru")));
        }

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 16.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan jenis pemicu transaksional pada mesin WidyaBase:

| Tipe Pemicu (*Trigger*) | Titik Evaluasi Eksekusi | Akses Modifikasi Tupel `BARU` | Dampak Kegagalan | Kasus Penggunaan Optimal |
| :--- | :--- | :--- | :--- | :--- |
| **BEFORE ROW** | Sebelum data ditulis ke halaman RAM | **Bisa Mengubah `BARU`** | Menggugurkan Transaksi | Validasi batasan bisnis, enkripsi data |
| **AFTER ROW** | Setelah data tercatat di mesin transaksi| Hanya-Baca (`BARU` & `LAMA`) | Menggugurkan Transaksi | Jejak audit kepatuhan, replikasi internal |
| **INSTEAD OF** | Menggantikan perintah asli kueri | Mengontrol eksekusi penuh | Menggugurkan Transaksi | Pembaruan pada View yang kompleks |
| **AFTER STATEMENT** | Sekali di akhir kueri kumpulan baris | Tidak ada variabel per-baris | Menggugurkan Transaksi | Ringkasan metrik statistik harian |

### 16.4 Latihan Soal Esai

1. Terangkan perbedaan mendasar antara pemicu berjenis `SEBELUM` (*BEFORE*) dan `SESUDAH` (*AFTER*)! Mengapa hanya pemicu *BEFORE* yang diizinkan memodifikasi nilai bidang tupel `BARU`?
2. Mengapa pencatatan audit log transaksi finansial harus dirancang berkonsep *Append-Only* dan tidak boleh memiliki hak akses perintah `UPDATE` atau `DELETE` bahkan untuk pengguna berkasta superuser (*DBA*)?
3. Uraikan mekanisme kerja integrasi rantai hash kriptografi (*Hash Chain*) pada tabel log audit untuk menjamin deteksi seketika bila ada manipulasi data piringan disk secara offline!

### 16.5 Soal Pilihan Ganda

1. Pemicu transaksi (*Trigger*) yang dieksekusi sebelum data fisik dimodifikasi sehingga dapat membatalkan atau mengubah nilai masukan adalah...
   A. BEFORE Trigger
   B. AFTER Trigger
   C. INSTEAD OF Trigger
   D. POST Trigger

2. Variabel semu yang digunakan untuk membaca nilai kolom sebelum terjadinya modifikasi pada operasi UPDATE atau DELETE dinamakan...
   A. `LAMA` (atau `OLD` pada standar SQL)
   B. `BARU` (atau `NEW`)
   C. `SEMENTARA`
   D. `MEMORI`

3. Variabel semu yang menyimpan nilai kolom baru yang akan disisipkan atau diperbarui adalah...
   A. `BARU` (atau `NEW`)
   B. `LAMA` (atau `OLD`)
   C. `AWAL`
   D. `BATAL`

4. Klausa pada pendefinisian pemicu yang menentukan bahwa pemicu dieksekusi satu kali untuk setiap baris data yang terpengaruh adalah...
   A. `FOR EACH ROW` (atau `UNTUK SETIAP BARIS`)
   B. `FOR EACH STATEMENT`
   C. `FOR EACH DATABASE`
   D. `FOR EACH USER`

5. Pemicu khusus yang digunakan untuk mengintersep operasi INSERT/UPDATE pada View yang rumit dan mengalihkannya ke tabel dasar dinamakan...
   A. INSTEAD OF Trigger
   B. BEFORE STATEMENT
   C. CASCADE Trigger
   D. MERGE Trigger

6. Batasan integritas referensial yang secara otomatis menghapus baris-baris anak ketika baris induk dihapus dinamakan...
   A. `ON DELETE CASCADE` (atau `PADA HAPUS BERUNTUN`)
   B. `ON DELETE RESTRICT`
   C. `ON DELETE NO ACTION`
   D. `ON DELETE FREEZE`

7. Sifat dari tabel log audit kepatuhan yang hanya mengizinkan penambahan catatan baru tanpa boleh diubah atau dihapus disebut...
   A. Append-Only
   B. Overwrite-Only
   C. Ephemeral
   D. Volatile

8. Teknik matematis yang digunakan untuk menghubungkan setiap baris catatan audit dengan hash catatan sebelumnya seperti buku besar blockchain dinamakan...
   A. Cryptographic Hash Chain
   B. Linear Regression
   C. Binary Search
   D. Round Robin

9. Apa yang terjadi pada transaksi utama pengguna jika sebuah AFTER Trigger di dalamnya melemparkan eksepsi galat (*error/exception*)?
   A. Seluruh transaksi utama otomatis digugurkan (*Rollback*) secara ACID
   B. Transaksi tetap sukses dan galat diabaikan
   C. Data ditulis separuh ke tabel
   D. Server basis data otomatis mati

10. Mengapa peletakan batasan integritas bisnis kritis di dalam mesin basis data (via Trigger/Constraint) jauh lebih aman daripada hanya meletakkannya di aplikasi web?
    A. Mencegah pelanggaran integritas data dari akses langsung, skrip migrasi, alat BI pihak ketiga, maupun eksploitasi API
    B. Menjadikan tampilan website lebih menarik
    C. Mengurangi kebutuhan memori pada HP pengguna
    D. Menghapus kebutuhan login pengguna
