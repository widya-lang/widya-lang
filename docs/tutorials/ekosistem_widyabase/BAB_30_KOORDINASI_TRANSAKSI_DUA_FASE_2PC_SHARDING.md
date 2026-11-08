## Bab 30: Koordinasi Transaksi Dua Fase (2PC) & Sharding

### 30.1 Transaksi ACID Lintas Server: Dilema Atomisitas Terdistribusi

Ketika volume data dan beban transaksi suatu bangsa telah melampaui batas kapasitas vertikal satu mesin peladen fisik tunggal (*Scale-Up Limit*), data harus dipecah secara horizontal ke berbagai peladen independen (**Sharding**). Misalnya: rekening nasabah wilayah Barat disimpan di Kluster Shard Sumatera-Jawa, sedangkan rekening nasabah wilayah Timur disimpan di Kluster Shard Sulawesi-Papua.

Tantangan terbesar muncul ketika terjadi **Transaksi Lintas Shard (*Cross-Shard Transaction*)**—seperti transfer dana dari rekening di Shard Barat ke rekening di Shard Timur. Bagaimana kita menjamin sifat *Atomicity* (keutuhan transaksi): saldo pengirim tidak terlanjur terdebit jika server penerima di wilayah Timur tiba-tiba mengalami mati listrik?

WidyaBase mengatasi dilema ini dengan mengimplementasikan protokol **Koordinasi Transaksi Dua Fase (*Distributed Two-Phase Commit / 2PC*)** terpadu (`consensus.rs`):
- **Koordinator Transaksi (*Transaction Coordinator*)**: Simpul yang memimpin dan mengorkestrasi jalannya transaksi global.
- **Partisipan (*Participants / Cohorts*)**: Simpul-simpul shard yang memegang data lokal yang terlibat dalam transaksi.

```mermaid
flowchart TD
    Klien["Aplikasi Perbankan Terpusat"] -->|1. Transfer Dana Lintas Shard: T_global| Koordinator["Koordinator Transaksi WidyaBase"]
    subgraph FaseSatuPrepare["FASE 1: PREPARE (Persiapan & Penguncian)"]
        Koordinator -->|2. PREPARE T_global| ShardA["Shard 1 (Debet Rek A: Jawa)"]
        Koordinator -->|2. PREPARE T_global| ShardB["Shard 2 (Kredit Rek B: Papua)"]
        ShardA -->|3. VOTE_COMMIT (Siap & Terkunci di WAL)| Koordinator
        ShardB -->|3. VOTE_COMMIT (Siap & Terkunci di WAL)| Koordinator
    end
    subgraph FaseDuaCommit["FASE 2: COMMIT (Kemit Global Paripurna)"]
        Koordinator -->|4. GLOBAL_COMMIT Terverifikasi| ShardA
        Koordinator -->|4. GLOBAL_COMMIT Terverifikasi| ShardB
        ShardA & ShardB -->|5. Selesai & Lepaskan Kunci| Koordinator
    end
    Koordinator -->|6. Kemit Sukses ACID Penuh| Klien
```

Prinsip kerja Two-Phase Commit (2PC) di WidyaBase:
1. **Fase 1: Persiapan (*Prepare Phase*)**:
   - Koordinator mengirim pesan `PREPARE` ke seluruh partisipan shard.
   - Setiap partisipan memeriksa batasan integritas, mengunci baris data, menuliskan seluruh mutasi ke WAL lokal, lalu merespons dengan `VOTE_COMMIT` (siap komit) atau `VOTE_ABORT` (gagal/ada konflik).
2. **Fase 2: Keputusan Global (*Commit / Abort Phase*)**:
   - **Jika seluruh partisipan merespons `VOTE_COMMIT`**: Koordinator mencatat keputusan `GLOBAL_COMMIT` di WAL-nya, lalu memerintahkan seluruh partisipan untuk melakukan komit permanen.
   - **Jika ada satu saja partisipan merespons `VOTE_ABORT` atau tidak merespons (timeout)**: Koordinator membatalkan transaksi secara serempak (`GLOBAL_ABORT`) di seluruh shard, mengembalikan kondisi semula 100%.

### 30.2 Implementasi Kode Program Utuh

Berikut implementasi logika Koordinator Transaksi 2PC dan Partisipan Shard pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Koordinasi Transaksi Dua Fase (2PC) Terdistribusi WidyaBase
modul koordinasi_transaksi_2pc {
    impor ekosistem::widyabase::{
        WidyaBase, Koordinator2PC, PartisipanShard,
        StatusVote2PC, KeputusanGlobal2PC, TransaksiID
    };

    fungsi eksekusi_transfer_lintas_shard(
        id_tx_global: TransaksiID,
        shard_pengirim: &mut PartisipanShard,
        shard_penerima: &mut PartisipanShard
    ) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [2PC-COORDINATOR] Memulai Transaksi Global T" + teks(id_tx_global) + " Lintas Shard...");

        biar mut koordinator = Koordinator2PC::buat(id_tx_global);

        // ==========================================
        // FASE 1: PREPARE
        // ==========================================
        cetak(">>> [2PC-FASE-1] Mengirim pesan PREPARE ke seluruh Shard...");

        // Shard Pengirim Menyiapkan Debit Rekening
        biar vote_pengirim = shard_pengirim.siapkan_transaksi_lokal(
            id_tx_global, 
            "PERBARUI rekening SET saldo = saldo - 50000000 WHERE id = 101;"
        )?;
        cetak(">>> [SHARD-1] Vote Pengirim: " + vote_pengirim.nama_status());

        // Shard Penerima Menyiapkan Kredit Rekening
        biar vote_penerima = shard_penerima.siapkan_transaksi_lokal(
            id_tx_global, 
            "PERBARUI rekening SET saldo = saldo + 50000000 WHERE id = 909;"
        )?;
        cetak(">>> [SHARD-2] Vote Penerima: " + vote_penerima.nama_status());

        // ==========================================
        // EVALUASI KEPUTUSAN KOORDINATOR
        // ==========================================
        jika vote_pengirim == StatusVote2PC::SiapKemit && vote_penerima == StatusVote2PC::SiapKemit {
            cetak(">>> [2PC-VOTE-UNANIMOUS] Seluruh Shard SIAP! Koordinator memutuskan GLOBAL_COMMIT.");
            koordinator.tulis_keputusan_wal(KeputusanGlobal2PC::KemitGlobal)?;

            // ==========================================
            // FASE 2: COMMIT GLOBAL
            // ==========================================
            cetak(">>> [2PC-FASE-2] Mengalirkan perintah GLOBAL_COMMIT...");
            shard_pengirim.komit_permanen_lokal(id_tx_global)?;
            shard_penerima.komit_permanen_lokal(id_tx_global)?;
            
            cetak(">>> [SUKSES-2PC] Transaksi Lintas Shard Komit Paripurna secara ACID!");
            cetak("==================================================================");
            kembalikan Ok(benar);
        } selain_itu {
            cetak(">>> [2PC-ABORT] Ada kegagalan/penolakan! Membatalkan di seluruh Shard...");
            koordinator.tulis_keputusan_wal(KeputusanGlobal2PC::BatalGlobal)?;

            // Pembatalan Serentak di Seluruh Shard
            shard_pengirim.batalkan_lokal(id_tx_global)?;
            shard_penerima.batalkan_lokal(id_tx_global)?;

            cetak(">>> [ABORTED] Transaksi dibatalkan bersih tanpa inkonsistensi saldo.");
            cetak("==================================================================");
            kembalikan Salah("Galat2PC: Transaksi terdistribusi digugurkan.");
        }
    }
}
```

### 30.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan protokol koordinasi terdistribusi pada WidyaBase:

| Karakteristik Protokol | Two-Phase Commit (2PC Standar) | 2PC Diperkuat Raft (WidyaBase) | Three-Phase Commit (3PC) |
| :--- | :--- | :--- | :--- |
| **Ketahanan Koordinator Mati**| Rawan Menggantung (*Blocking Issue*)| **Toleran Sempurna (Koordinator Failover)**| Menghindari Blocking via Status PreCommit|
| **Jumlah Putaran Jaringan (RTT)**| 2 Putaran RTT (Prepare + Commit) | 2 Putaran RTT + Kuorum Raft | 3 Putaran RTT (Lebih Lambat) |
| **Kepatuhan ACID Atomisitas**| 100% Terjamin Penuh | **100% Terjamin Penuh** | 100% Terjamin Penuh |
| **Durasi Penguncian Sumber Daya**| Selama Fase 1 hingga Fase 2 Selesai| Sangat Singkat (Dioptimalkan WAL)| Lebih Lama (Ada fase perantara) |
| **Kasus Penggunaan Optimal** | Transaksi ACID Finansial Lintas Shard| **Standar Inti Sharding WidyaBase**| Jaringan latensi tinggi tidak stabil |

### 30.4 Latihan Soal Esai

1. Terangkan mengapa kegagalan daya pada simpul koordinator (*coordinator crash*) tepat setelah mengirimkan pesan `PREPARE` dapat menyebabkan partisipan berada dalam kondisi menggantung (*in-doubt / blocking state*)!
2. Bagaimana integrasi protokol konsensus Raft pada lapisan koordinator di WidyaBase mampu mengeliminasi kelemahan klasik *Single Point of Failure* dari protokol 2PC standar?
3. Jelaskan trade-off performa antara latensi transaksi lokal (dalam satu shard) dan latensi transaksi terdistribusi (*cross-shard*) yang memerlukan orkestrasi 2PC!

### 30.5 Soal Pilihan Ganda

1. Protokol terdistribusi klasik yang digunakan untuk menjamin atomisitas transaksi di berbagai simpul basis data yang terpisah adalah...
   A. Two-Phase Commit (2PC)
   B. Two-Phase Locking (2PL)
   C. Carrier Sense Multiple Access
   D. Round Robin Scheduling

2. Dua fase kerja utama yang berurutan pada protokol Two-Phase Commit adalah...
   A. Fase Persiapan (*Prepare Phase*) dan Fase Komit/Batal (*Commit/Abort Phase*)
   B. Fase Baca dan Fase Tulis
   C. Fase Buka dan Fase Tutup
   D. Fase Unduh dan Fase Unggah

3. Apa yang dilakukan oleh simpul partisipan shard pada Fase Prepare?
   A. Mengevaluasi kueri, mengunci data, menulis log ke WAL, dan mengirimkan respon siap (*Vote Commit*) atau gagal (*Vote Abort*)
   B. Mematikan peladen basis data
   C. Menghapus data tabel
   D. Menyetel ulang kata sandi admin

4. Jika satu dari sepuluh simpul partisipan merespons dengan `VOTE_ABORT`, maka tindakan mutlak yang diambil oleh Koordinator adalah...
   A. Memerintahkan `GLOBAL_ABORT` ke seluruh sembilan simpul lainnya untuk membatalkan seluruh perubahan
   B. Melanjutkan komit pada sembilan simpul lainnya
   C. Mengabaikan simpul yang gagal
   D. Mengunci database selamanya

5. Kelemahan teoritis klasik dari protokol 2PC standar tanpa redundansi pemimpin adalah...
   A. Sifat pemblokiran (*Blocking Problem*) jika koordinator mati mendadak saat partisipan sedang menunggu keputusan
   B. Basis data tidak bisa menyimpan angka
   C. Menghabiskan baterai laptop
   D. Memerlukan kabel serat optik khusus

6. Kondisi di mana simpul partisipan telah menyatakan siap (`Vote Commit`) namun belum menerima instruksi akhir dari koordinator dinamakan status...
   A. Status Ragu-Ragu (*In-Doubt State*)
   B. Status Selesai
   C. Status Hang
   D. Status Format

7. Titik penentu tanpa kembali (*Point of No Return*) di mana transaksi terdistribusi secara resmi dianggap berhasil dikomit terjadi saat...
   A. Koordinator berhasil menuliskan catatan log keputusan `GLOBAL_COMMIT` ke dalam WAL-nya
   B. Klien menutup browser
   C. Partisipan pertama mengirim pesan
   D. Jam server berdentang

8. Teknik memecah tabel raksasa secara horizontal ke berbagai server fisik independen dinamakan...
   A. Database Sharding
   B. Page Split
   C. Vectorization
   D. Index Scanning

9. Mengapa arsitektur aplikasi berskala besar sedapat mungkin meminimalkan kueri transaksi lintas-shard (*cross-shard transactions*)?
   A. Karena transaksi lintas-shard memerlukan koordinasi jaringan ganda (2PC) yang menambah latensi respons dibandingkan transaksi lokal satu shard
   B. Karena transaksi lintas-shard dilarang oleh undang-undang
   C. Karena komputer server tidak mendukung kabel LAN
   D. Supaya warna monitor tetap stabil

10. Keuntungan solusi 2PC yang diperkuat oleh replikasi Raft pada WidyaBase adalah...
    A. Jika koordinator fisik mengalami crash, simpul pengganti Raft baru dapat langsung melanjutkan orkestrasi 2PC dari catatan WAL terdistribusi tanpa ada transaksi yang menggantung
    B. Menghapus kebutuhan media penyimpanan SSD
    C. Menggandakan kapasitas listrik secara otomatis
    D. Menghilangkan kebutuhan autentikasi pengguna
