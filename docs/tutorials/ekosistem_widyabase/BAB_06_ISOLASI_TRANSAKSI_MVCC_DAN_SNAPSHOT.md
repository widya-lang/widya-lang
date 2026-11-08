## Bab 6: Isolasi Transaksi MVCC & Snapshot Isolation

### 6.1 Teori Konkurensi Tanpa Kunci: Pembaca Tidak Menghalangi Penulis

Dalam sistem pemrosesan transaksi daring (*Online Transaction Processing* / OLTP) dengan throughput tinggi, strategi penguncian tradisional berbasis *Strict Two-Phase Locking* (S2PL) menimbulkan kemacetan parah (*bottleneck*). Pada sistem S2PL murni, pembaca memblokir penulis, dan penulis memblokir pembaca.

WidyaBase memecahkan dilema konkurensi ini dengan mengadopsi **Multi-Version Concurrency Control** (MVCC) berbasis **Snapshot Isolation**:
- **Pembaca Tidak Memblokir Penulis (*Readers Never Block Writers*)**: Kueri analitis pembacaan panjang dapat berjalan berjam-jam tanpa pernah menghambat transaksi mutasi data keuangan.
- **Penulis Tidak Memblokir Pembaca (*Writers Never Block Readers*)**: Transaksi pembaruan data tidak mengunci kueri pelaporan.

```mermaid
flowchart TD
    A["Tabel Data: Baris Pegawai ID=101"] --> B["Versi 1: Gaji = Rp 10.000.000<br>(xmin=100, xmax=105)"]
    B -->|Pointer Rantai Versi (Prev)| C["Versi 2: Gaji = Rp 12.500.000<br>(xmin=105, xmax=120)"]
    C -->|Pointer Rantai Versi (Prev)| D["Versi 3: Gaji = Rp 15.000.000<br>(xmin=120, xmax=0 / Aktif)"]
    subgraph PenglihatanSnapshot["Snapshot Transaksi Berdasarkan Snapshot ID"]
        E["Transaksi T102 (Snapshot ID=102)"] -.->|Melihat Versi 1 (xmin <= 102 < xmax)| B
        F["Transaksi T110 (Snapshot ID=110)"] -.->|Melihat Versi 2 (xmin <= 110 < xmax)| C
        G["Transaksi T125 (Snapshot ID=125)"] -.->|Melihat Versi 3 (xmin <= 125 & xmax=0)| D
    end
```

Prinsip kerja MVCC pada WidyaBase:
1. **Metadata Tuple**: Setiap baris data (*tuple*) di dalam slotted-page dilengkapi dengan metadata transaksional:
   - `xmin`: ID transaksi pembuat (*creating transaction ID*).
   - `xmax`: ID transaksi penghapus atau pembaru (*deleting/updating transaction ID*). Nilai `0` menandakan tuple masih aktif dan belum dihapus.
   - `t_ctid`: Pointer yang menunjuk ke lokasi fisik versi terbaru dari baris tersebut.
2. **Snapshot Transaksional**: Ketika transaksi dimulai dengan tingkat isolasi `PISAH_TERBACA` (*READ COMMITTED*) atau `SNAPSHOT`, sistem mengambil potret transaksi aktif (*Snapshot* = `{xmin, xmax, xip_list}`). Tuple dinyatakan tampak (*visible*) jika transaksi pembuatnya telah berhasil dikomit sebelum snapshot diambil.
3. **Pembersihan Vakum (*Vacuuming*)**: Versi lama yang sudah kedaluwarsa (*dead tuples*) dan tidak dapat dilihat lagi oleh transaksi aktif manapun akan dibersihkan oleh daemon *Auto-Vacuum* untuk mengklaim ulang ruang penyimpanan fisik.

### 6.2 Implementasi Kode Program Utuh

Berikut implementasi manajemen versi tupel MVCC dan evaluasi visibilitas snapshot pada WidyaBase:

```widya
// Modul Multi-Version Concurrency Control (MVCC) WidyaBase
modul mesin_mvcc {
    impor ekosistem::widyabase::{
        WidyaBase, TransaksiID, SnapshotTransaksi, HeaderTupel,
        HasilVisibilitas, StatusTransaksi
    };

    struktur SnapshotData {
        snapshot_id: TransaksiID,
        daftar_transaksi_aktif: Daftar<TransaksiID>,
        transaksi_terendah_aktif: TransaksiID,
        transaksi_tertinggi_aktif: TransaksiID,
    }

    fungsi evaluasi_visibilitas_tupel(
        header: &HeaderTupel,
        snapshot: &SnapshotData,
        status_manajer: &WidyaBase
    ) -> HasilVisibilitas {
        // Aturan 1: Jika pembuat belum selesai atau dibatalkan (Aborted), abaikan
        biar status_pembuat = status_manajer.dapatkan_status_transaksi(header.xmin);
        jika status_pembuat != StatusTransaksi::Kemit {
            kembalikan HasilVisibilitas::TidakTampak;
        }

        // Aturan 2: Jika pembuat berada di dalam daftar transaksi aktif saat snapshot dibuat
        jika snapshot.daftar_transaksi_aktif.berisi(header.xmin) {
            kembalikan HasilVisibilitas::TidakTampak;
        }

        // Aturan 3: Periksa status penghapus (xmax)
        jika header.xmax == 0 {
            // Belum pernah dihapus atau diperbarui oleh siapapun
            kembalikan HasilVisibilitas::Tampak;
        }

        biar status_penghapus = status_manajer.dapatkan_status_transaksi(header.xmax);
        jika status_penghapus == StatusTransaksi::Kemit {
            // Jika penghapus sudah komit SEBELUM snapshot diambil, data sudah mati bagi kita
            jika header.xmax < snapshot.snapshot_id && !snapshot.daftar_transaksi_aktif.berisi(header.xmax) {
                kembalikan HasilVisibilitas::TidakTampak;
            }
        }

        // Jika penghapus belum komit atau berjalan setelah snapshot, data tetap tampak
        kembalikan HasilVisibilitas::Tampak;
    }

    fungsi lakukan_pembaruan_mvcc(
        kunci: Teks,
        data_baru: Daftar<U8>,
        id_transaksi_kini: TransaksiID
    ) -> Hasil<Boolean, Teks> {
        cetak(">>> [MVCC] Memperbarui baris data secara non-destruktif...");

        // 1. Dapatkan versi lama
        biar mut header_lama = WidyaBase::baca_header_tupel(&kunci)?;

        // 2. Tandai xmax pada versi lama dengan ID transaksi yang sedang memperbarui
        header_lama.xmax = id_transaksi_kini;
        WidyaBase::tulis_header_tupel(&kunci, &header_lama)?;

        // 3. Masukkan versi baru dengan xmin = id_transaksi_kini, xmax = 0
        biar header_baru = HeaderTupel {
            xmin: id_transaksi_kini,
            xmax: 0,
            ctid_lanjutan: 0,
        };
        WidyaBase::tulis_versi_baru(&kunci, header_baru, data_baru)?;

        cetak(">>> [MVCC] Versi baru berhasil disisipkan dengan pointer rantai ctid.");
        kembalikan Ok(benar);
    }
}
```

### 6.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel anomali konkurensi SQL standar dan perilakunya pada tingkatan isolasi WidyaBase:

| Tingkatan Isolasi (*Isolation Level*) | Bacaan Kotor (*Dirty Read*) | Bacaan Tak Berulang (*Non-Repeatable*) | Bacaan Fantom (*Phantom Read*) | Kehilangan Pembaruan (*Lost Update*) |
| :--- | :--- | :--- | :--- | :--- |
| **BACA_BELUM_KOMIT (Read Uncommitted)** | Dicegah (WidyaBase tidak mengizinkan kotor) | Mungkin Muncul | Mungkin Muncul | Rawan Muncul |
| **BACA_SUDAH_KOMIT (Read Committed)** | Dicegah Mutlak | Dicegah per Pernyataan | Mungkin Muncul | Dicegah via Kunci Baris |
| **SNAPSHOT / DAPAT_DIULANG (Repeatable Read)** | Dicegah Mutlak | Dicegah Mutlak | Dicegah Mutlak | Dicegah via Pemeriksaan Konflik |
| **SERIALISASI (Serializable SSI)** | Dicegah Mutlak | Dicegah Mutlak | Dicegah Mutlak | Dicegah Mutlak (Deteksi Siklus Graf) |

### 6.4 Latihan Soal Esai

1. Mengapa pendekatan *In-Place Update* (menimpa langsung data lama pada disk) sangat berbahaya bagi konkurensi transaksi dan pemulihan ACID dibandingkan pendekatan MVCC multi-versi?
2. Bagaimana algoritma evaluasi visibilitas tupel MVCC di WidyaBase memanfaatkan variabel `xmin`, `xmax`, dan daftar transaksi aktif pada snapshot?
3. Uraikan mekanisme kerja daemon *Auto-Vacuum* dalam mendeteksi dan membersihkan *dead tuples* tanpa menimbulkan penurunan performa (*I/O thrashing*) pada beban kerja produksi!

### 6.5 Soal Pilihan Ganda

1. Keuntungan utama dari arsitektur konkurensi Multi-Version Concurrency Control (MVCC) adalah...
   A. Basis data tidak memerlukan ruang penyimpanan disk
   B. Pembaca (*readers*) tidak pernah memblokir penulis (*writers*), dan penulis tidak memblokir pembaca
   C. Kecepatan prosesor bertambah secara otomatis
   D. Mematikan seluruh transaksi komit

2. Bidang (*field*) pada header baris data MVCC yang mencatat ID transaksi yang menyisipkan baris tersebut adalah...
   A. `xmin`
   B. `xmax`
   C. `lsn`
   D. `flags`

3. Bidang `xmax` bernilai `0` pada header tupel menandakan bahwa...
   A. Baris data tersebut telah dihapus secara permanen
   B. Baris data masih aktif dan belum pernah dihapus atau diperbarui oleh transaksi komit
   C. Baris data mengalami korupsi fisik
   D. Baris data dibuat oleh administrator

4. Pada saat kueri dijalankan pada tingkat isolasi Snapshot, pandangan data konsisten yang dilihat oleh transaksi didasarkan pada...
   A. Waktu lokal pengguna
   B. Status transaksi yang telah komit pada saat snapshot dibuat
   C. Warna antarmuka pengguna
   D. Nama tabel yang diakses

5. Fenomena anomali di mana dua transaksi membaca saldo yang sama lalu menimpa perubahannya sehingga salah satu saldo hilang dinamakan...
   A. Phantom Read
   B. Lost Update
   C. Dirty Read
   D. Cache Hit

6. Apa peran dari pointer rantai versi (`ctid`) pada arsitektur MVCC WidyaBase?
   A. Menghubungkan versi baris data yang lama ke lokasi fisik versi baris data yang baru
   B. Mengarahkan kueri ke server lain
   C. Menyimpan nama pengguna pengubah data
   D. Mengunci memori monitor

7. Proses latar belakang yang bertugas mengklaim kembali ruang penyimpanan fisik dari baris data mati (*dead tuples*) dinamakan...
   A. Formatting
   B. Vacuuming (Auto-Vacuum)
   C. Overclocking
   D. Defragmenting Windows

8. Anomali konkurensi pembacaan data yang belum berhasil dikomit oleh transaksi lain dikenal dengan istilah...
   A. Dirty Read
   B. Non-repeatable Read
   C. Phantom Read
   D. Serialization Failure

9. Tingkat isolasi tertinggi pada standar ANSI SQL yang menjamin eksekusi transaksi berjalan seolah-olah berurutan satu per satu adalah...
   A. Read Uncommitted
   B. Read Committed
   C. Serializable
   D. Repeatable Read

10. Mengapa pembersihan tupel kedaluwarsa tidak boleh langsung dilakukan saat perintah DELETE dikomit?
    A. Karena masih ada transaksi aktif lain yang snapshot-nya memerlukan pandangan versi lama tersebut
    B. Menunggu persetujuan manual administrator sistem
    C. Supaya kapasitas harddisk cepat habis
    D. Karena sistem operasi melarang penghapusan file
