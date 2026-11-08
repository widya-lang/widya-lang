## Bab 29: Protokol Konsensus Terdistribusi Raft & Replikasi Multi-Node

### 29.1 Ketersediaan Tinggi & Ketahanan Bencana Berbasis Quorum

Dalam perancangan infrastruktur data misi-kritis (*mission-critical systems*) nasional—seperti gerbang pembayaran antar-bank, sistem kendali lalu lintas udara, atau pertukaran data intelijen pertahanan—kegagalan satu perangkat keras peladen fisik (*single node failure*) tidak boleh merusak atau menghentikan layanan basis data. Sistem dituntut memiliki ketersediaan tinggi (*High Availability*) dan toleransi kesalahan (*Fault Tolerance*) mutlak.

Untuk mewujudkan keandalan tersebut tanpa ketergantungan pada perangkat lunak orkestrasi asing, WidyaBase mengintegrasikan implementasi **Protokol Konsensus Terdistribusi Raft** mandiri (`consensus.rs`):
- **Struktur Tiga Peran State Machine**: Setiap simpul (*node*) di dalam kluster Raft beroperasi dalam salah satu dari tiga peran berdaulat:
  - `Pengikut` (*Follower*): Merespons permintaan RPC dari kandidat dan pemimpin.
  - `Kandidat` (*Candidate*): Menginisiasi pemilihan pemimpin baru ketika detak jantung (*heartbeat*) terputus.
  - `Pemimpin` (*Leader*): Menerima seluruh transaksi tulis dari klien, mereplikasi entri log ke seluruh anggota kluster, dan memutuskan indeks komit global.
- **Konsensus Mayoritas Kuorum (*Quorum Majority*)**: Sebuah mutasi transaksi dinyatakan sah dan dikomit jika dan hanya jika entri log telah berhasil direplikasi ke mayoritas simpul ($Q = \lfloor N/2 \rfloor + 1$). Kluster beranggotakan 3 simpul dapat menoleransi kegagalan 1 simpul, sedangkan kluster 5 simpul dapat menoleransi 2 simpul mati secara serentak.
- **Detak Jantung & Batas Waktu Acak (*Randomized Election Timeout*)**: Mencegah pemungutan suara terbelah (*split vote*) saat pemilihan pemimpin berlangsung.

```mermaid
flowchart TD
    subgraph KlusterRaftWidyaBase["Kluster Konsensus Raft WidyaBase (3 Simpul)"]
        Leader["Simpul 1: PEMIMPIN (Leader)<br>[Menerima Transaksi Klien]"]
        FollowerA["Simpul 2: PENGIKUT (Follower A)<br>[Replikasi Log]"]
        FollowerB["Simpul 3: PENGIKUT (Follower B)<br>[Replikasi Log]"]
    end
    Klien["Aplikasi Klien Finansial"] -->|1. Kueri Tulis: SISIP_KE transaksi ...| Leader
    Leader -->|2. AppendEntries RPC (Log Entry & Term)| FollowerA & FollowerB
    FollowerA & FollowerB -->|3. Kirim Pengakuan Sukses (Ack)| Leader
    Leader -->|4. Kuorum Tercapai (2 dari 3)! Perbarui commitIndex| Leader
    Leader -->|5. Beri Respons Sukses ACID ke Klien| Klien
    Leader -.->|Detak Jantung Berkala (Heartbeat RPC)| FollowerA & FollowerB
```

Fitur utama implementasi Raft di WidyaBase:
1. **Keamanan Pemilihan (*Election Safety*)**: Menjamin bahwa sebuah simpul hanya dapat terpilih menjadi pemimpin jika log transaksinya setidaknya sama mutakhirnya dengan mayoritas pemilih (*Up-to-Date Log Check*).
2. **Konsistensi Log Monoton**: Jika sebuah entri log telah dikomit oleh seorang pemimpin pada term tertentu, entri tersebut dijamin hadir dalam log seluruh pemimpin di masa depan.
3. **Penyusutan Log & Snapshot Otomatis**: Secara berkala membuat potret kondisi (*compact snapshot*) untuk mencegah log bertumbuh tanpa batas dan mempercepat sinkronisasi simpul baru (*Catch-Up Join*).

### 29.2 Implementasi Kode Program Utuh

Berikut implementasi simulasi pemilihan pemimpin Raft dan replikasi entri log konsensus pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Konsensus Terdistribusi Raft WidyaBase
modul konsensus_raft {
    impor ekosistem::widyabase::{
        WidyaBase, SimpulRaft, PeranRaft, EntriLogRaft,
        PesanAppendEntries, PesanRequestVote, StatusKonsensus
    };

    fungsi jalankan_simulasi_kluster_raft() -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [RAFT-CLUSTER] Menginisialisasi Kluster Konsensus 3 Simpul...");

        // 1. Inisialisasi 3 Simpul Raft Terdistribusi
        biar mut simpul_1 = SimpulRaft::buat(1, ["node-2", "node-3"]);
        biar mut simpul_2 = SimpulRaft::buat(2, ["node-1", "node-3"]);
        biar mut simpul_3 = SimpulRaft::buat(3, ["node-1", "node-2"]);

        cetak(">>> [RAFT] Seluruh simpul awal berstatus: PENGIKUT (Term: 0)");

        // 2. Simulasikan Batas Waktu Pemilihan Habis pada Simpul 1 (Menjadi Kandidat)
        cetak(">>> [ELECTION] Simpul 1 mengalami timeout! Memulai pemilihan (Term: 1)...");
        simpul_1.ubah_peran_kandidat();
        
        // Simpul 1 Meminta Suara (RequestVote) ke Simpul 2 dan 3
        biar suara_2 = simpul_2.proses_permintaan_suara(simpul_1.buat_permintaan_suara());
        biar suara_3 = simpul_3.proses_permintaan_suara(simpul_1.buat_permintaan_suara());

        jika suara_2.diberikan && suara_3.diberikan {
            simpul_1.terpilih_menjadi_pemimpin();
            cetak(">>> [LEADER] Simpul 1 berhasil mengumpulkan mayoritas suara (3/3)!");
            cetak(">>> [LEADER] Simpul 1 resmi menjadi PEMIMPIN AKTIF Kluster.");
        }

        // 3. Eksekusi Replikasi Mutasi Transaksi Klien via Pemimpin
        cetak(">>> [RAFT-LOG] Menerima transaksi: Transfer Antar Bank Rp 1.000.000.000");
        biar perintah_mutasi = "PERBARUI kas_negara SET saldo = saldo + 1000000000;";
        
        // Pemimpin Menulis ke Log Lokal
        biar indeks_log = simpul_1.tulis_entri_log_baru(perintah_mutasi)?;
        cetak(">>> [LEADER] Entri dicatat di Log Pemimpin pada Indeks: " + teks(indeks_log));

        // Pemimpin Mengirimkan AppendEntries RPC ke Para Pengikut
        biar pesan_rpc = simpul_1.buat_pesan_append_entries(indeks_log);
        biar respon_2 = simpul_2.proses_append_entries(&pesan_rpc);
        biar respon_3 = simpul_3.proses_append_entries(&pesan_rpc);

        jika respon_2.sukses && respon_3.sukses {
            cetak(">>> [QUORUM] Pengakuan replikasi sukses diterima dari Simpul 2 & 3.");
            simpul_1.perbarui_commit_index(indeks_log);
            cetak(">>> [COMMIT] Transaksi berstatus KEMIT GLOBAL pada commitIndex: " + teks(indeks_log));
        }

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 29.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel toleransi kegagalan simpul (*Fault Tolerance*) pada kuorum Raft WidyaBase:

| Jumlah Simpul Kluster ($N$) | Kuorum Mayoritas ($Q$) | Toleransi Jumlah Simpul Mati | Rekomendasi Kasus Penggunaan |
| :--- | :--- | :--- | :--- |
| **1 Simpul** (Standalone) | 1 Simpul | 0 Simpul (Tanpa toleransi) | Lingkungan pengujian / laptop developer |
| **3 Simpul** (Minimal HA) | 2 Simpul | 1 Simpul Mati | Kluster produksi standar cabang perbankan |
| **5 Simpul** (Standar Enterprise)| 3 Simpul | **2 Simpul Mati Serempak** | Inti perbankan nasional & pusat data utama |
| **7 Simpul** (Sangat Kritis) | 4 Simpul | 3 Simpul Mati Serempak | Infrastruktur pertahanan & perbendaharaan negara |
| **Genap (misal 4 Simpul)** | 3 Simpul | 1 Simpul Mati (Tidak efisien)| Tidak disarankan (Sama toleransinya dengan 3) |

### 29.4 Latihan Soal Esai

1. Mengapa jumlah anggota simpul di dalam sebuah kluster konsensus Raft selalu disarankan berjumlah ganjil (3, 5, atau 7) dan bukan genap?
2. Terangkan mekanisme kerja *Randomized Election Timeout* dalam mencegah terjadinya kebuntuan pemungutan suara terbelah (*split vote*) saat pemilihan pemimpin!
3. Uraikan apa yang terjadi apabila sebuah simpul pemimpin mengalami partisi jaringan (*network partition*) sehingga terpisah sendirian dari mayoritas simpul lainnya! Bagaimana Raft menjamin tidak ada fenomena *Split-Brain*?

### 29.5 Soal Pilihan Ganda

1. Tiga status peran simpul yang ada pada protokol konsensus Raft adalah...
   A. Follower (Pengikut), Candidate (Kandidat), dan Leader (Pemimpin)
   B. Master, Slave, dan Worker
   C. Client, Server, dan Gateway
   D. Root, Branch, dan Leaf

2. Formula matematis untuk menghitung batas minimum suara kuorum mayoritas pada kluster beranggotakan $N$ simpul adalah...
   A. $\lfloor N/2 \rfloor + 1$
   B. $N - 1$
   C. $N \times 2$
   D. 100% dari seluruh simpul

3. Berapa jumlah simpul yang boleh mengalami kegagalan/mati serempak pada kluster Raft beranggotakan 5 simpul tanpa menyebabkan basis data berhenti beroperasi?
   A. 2 simpul
   B. 4 simpul
   C. 1 simpul saja
   D. 0 simpul

4. Pesan RPC yang dikirimkan oleh simpul Pemimpin kepada seluruh Pengikut untuk mereplikasi entri transaksi sekaligus bertindak sebagai detak jantung (*heartbeat*) adalah...
   A. `AppendEntries RPC`
   B. `RequestVote RPC`
   C. `PingPong RPC`
   D. `InstallOS RPC`

5. Kondisi berbahaya di mana dua simpul sama-sama merasa menjadi pemimpin yang sah dan menerima transaksi tulis yang saling bertentangan dinamakan anomali...
   A. Split-Brain
   B. Dirty Read
   C. Buffer Overflow
   D. Latch Deadlock

6. Bagaimana cara protokol Raft mencegah kemunculan fenomena Split-Brain pada kluster terpartisi?
   A. Pemimpin lama yang terisolasi di sisi minoritas tidak akan pernah berhasil mencapai kuorum suara mayoritas untuk mengomit transaksi tulis baru
   B. Mematikan seluruh harddisk di semua simpul
   C. Menghapus data transaksi lama
   D. Mengabaikan aturan matematika

7. Variabel monoton 64-bit yang bertindak sebagai jam logis masa jabatan dan selalu meningkat di setiap pemilihan pemimpin baru dinamakan...
   A. Term (Masa Jabatan)
   B. LSN
   C. SlotID
   D. ThreadID

8. Simpul Pengikut akan bertransformasi menjadi Kandidat dan memicu pemilihan baru apabila...
   A. Tidak menerima detak jantung (*heartbeat*) dari Pemimpin dalam rentang waktu *Election Timeout*
   B. Kapasitas RAM peladen habis
   C. Menerima kueri SELECT dari klien
   D. Jam dinding lokal menunjukkan pukul 12 malam

9. Apa yang dilakukan simpul Kandidat jika dalam batas waktu pemilihan tidak ada satu pun kandidat yang berhasil meraih suara mayoritas?
   A. Menyetel ulang timer acak dan memulai babak pemilihan baru dengan Term berikutnya
   B. Memformat ulang database
   C. Keluar dari jaringan internet
   D. Mengembalikan hak akses ke administrator

10. Mengapa pemangkasan log (*Log Compaction*) berbasis Snapshot sangat esensial pada kluster Raft yang telah beroperasi selama bertahun-tahun?
    A. Mencegah berkas catatan log Raft memenuhi kapasitas disk dan mempercepat proses bergabungnya simpul baru (*catch-up*)
    B. Menghapus akun nasabah yang tidak aktif
    C. Mengganti sistem operasi server
    D. Menghilangkan kebutuhan kabel LAN
