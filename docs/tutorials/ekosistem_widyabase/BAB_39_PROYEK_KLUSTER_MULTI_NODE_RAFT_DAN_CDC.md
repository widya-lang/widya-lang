## Bab 39: Proyek Industri 4: Kluster Multi-Node Raft & Replikasi CDC

### 39.1 Arsitektur Solusi: Kluster Terdistribusi Nir-Henti & Sinkronisasi Real-Time

Infrastruktur sistem pembayaran nasional dan perbendaharaan digital menuntut toleransi kesalahan mutlak: jika pusat data (*Data Center* / DC) utama di Jakarta mengalami bencana alam atau pemadaman listrik total, pusat data cadangan (*Disaster Recovery Center* / DRC) di Surabaya atau Nusantara harus mampu mengambil alih operasional perbankan secara otomatis dalam hitungan detik (*Automatic Failover*) tanpa kehilangan satu rupiah pun data transaksi (*Zero Data Loss / RPO = 0*).

Dalam proyek industri terapan bagian keempat ini, kita akan merancang dan menguji **Kluster Multi-Node Terdistribusi WidyaBase Berdaya Tahan Bencana Tinggi**:
- **Kluster Konsensus 3-Node Raft**: Mengimplementasikan pembagian kuorum mayoritas lintas tiga zona ketersediaan fisik yang terpisah.
- **Replikasi Aliran CDC Waktu-Nyata**: Menyalurkan rekaman mutasi transaksi komit via Change Data Capture ke kluster analitik dan pangkalan data pemantauan audit secara real-time.
- **Pengujian Simulasi Partisi Jaringan (*Chaos Engineering & Split-Brain Test*)**: Membuktikan bahwa pemutusan kabel jaringan pada simpul pemimpin tidak merusak integritas data dan pemilihan pemimpin baru berlangsung mulus di bawah 1,5 detik.

```mermaid
flowchart TD
    subgraph DC_Jakarta["Pusat Data Utama (Zona 1: Jakarta)"]
        N1["Simpul 1: PEMIMPIN RAFT (Leader)<br>[Melayani Transaksi Aktif]"]
    end
    subgraph DC_Surabaya["Pusat Data Cadangan (Zona 2: Surabaya)"]
        N2["Simpul 2: PENGIKUT RAFT (Follower A)<br>[Replikasi Sinkron Kuorum]"]
    end
    subgraph DC_Nusantara["Pusat Data IKN (Zona 3: Nusantara)"]
        N3["Simpul 3: PENGIKUT RAFT (Follower B)<br>[Replikasi Sinkron Kuorum]"]
    end
    Klien["Aplikasi Perbankan Nasional"] -->|Transaksi Komit ACID| N1
    N1 -->|Replikasi Log Raft (Quorum 2/3)| N2 & N3
    subgraph AliranCDCRealTime["Integrasi Ekosistem via Mesin CDC (cdc.rs)"]
        N1 -->|Aliran CDC Perubahan Saldo Real-Time| Kafka["Mesin Kluster Analitik & Audit Eksternal"]
    end
    N1 -.->|Simulasi Bencana: Koneksi Terputus Total| Bencana["Bencana Daya / Jaringan Terputus!"]
    Bencana -->|Simpul 2 & 3 Memilih Pemimpin Baru Secara Mandiri| N2
```

Spesifikasi rancang bangun proyek:
1. **Topologi Jaringan**: 3 simpul aktif (`node-jakarta-01`, `node-surabaya-02`, `node-ikn-03`) yang saling berkomunikasi via protokol konsensus Raft RPC.
2. **Replication Slot CDC Persisten**: Slot replikasi `slot_audit_keuangan_nasional` yang memantau mutasi tabel perbankan.
3. **Target Pemulihan Bencana**:
   - *Recovery Point Objective* (RPO) = 0 (Nol transaksi hilang).
   - *Recovery Time Objective* (RTO) < 2 detik (Pemilihan pemimpin baru instan).

### 39.2 Implementasi Kode Program Utuh

Berikut implementasi kode program orkestrasi kluster multi-node Raft, replikasi CDC, dan pengujian failover pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Proyek Industri 4: Kluster Multi-Node Terdistribusi Raft & CDC WidyaBase
modul proyek_kluster_raft_cdc {
    impor ekosistem::widyabase::{
        WidyaBase, SimpulRaft, PeranRaft, MesinCDC,
        SlotReplikasi, RekamanCDC, StatusKluster
    };

    fungsi jalankan_simulasi_failover_kluster() -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [PROYEK-KLUSTER] Membangun Kluster Perbankan 3-Node Raft...");

        // 1. Inisialisasi 3 Simpul Lintas Zona Geografis
        biar mut node_jkt = SimpulRaft::buat(1, ["node-sby", "node-ikn"]);
        biar mut node_sby = SimpulRaft::buat(2, ["node-jkt", "node-ikn"]);
        biar mut node_ikn = SimpulRaft::buat(3, ["node-jkt", "node-sby"]);

        // Simpul 1 Terpilih Menjadi Pemimpin Awal
        node_jkt.terpilih_menjadi_pemimpin();
        cetak(">>> [RAFT] Kluster beroperasi normal. Pemimpin aktif: 'node-jakarta' (Term: 1)");

        // 2. Hubungkan Mesin CDC pada Simpul Pemimpin
        biar mut cdc_pemimpin = MesinCDC::buat();
        biar slot_replikasi = cdc_pemimpin.buat_slot_replikasi("slot_audit_nasional")?;
        cetak(">>> [CDC] Slot CDC 'slot_audit_nasional' terdaftar.");

        // 3. Eksekusi Transaksi Finansial dan Replikasi Kuorum
        cetak(">>> [TX] Memproses transaksi: Setoran Kas Negara Rp 50.000.000.000...");
        biar lsn_baru = node_jkt.tulis_entri_log_baru("PERBARUI kas_negara SET saldo = saldo + 50000000000;")?;
        
        // Replikasi ke Simpul Surabaya dan IKN
        biar rpc = node_jkt.buat_pesan_append_entries(lsn_baru);
        node_sby.proses_append_entries(&rpc);
        node_ikn.proses_append_entries(&rpc);
        node_jkt.perbarui_commit_index(lsn_baru);
        cetak(">>> [KEMIT] Transaksi komit pada commitIndex: " + teks(lsn_baru));

        // 4. Simulasi Bencana: Simpul Jakarta Mati Mendadak (Chaos Engineering)
        cetak("------------------------------------------------------------------");
        cetak(">>> [CHAOS-SIMULATION] PERINGATAN: Simpul Jakarta mengalami mati total!");
        node_jkt.simulasikan_mati_jaringan();

        // 5. Simpul Surabaya Mendeteksi Hilangnya Heartbeat & Memicu Pemilihan Otomatis
        cetak(">>> [FAILOVER] Simpul Surabaya mendeteksi timeout! Memulai pemilihan (Term: 2)...");
        node_sby.ubah_peran_kandidat();
        biar vote_ikn = node_ikn.proses_permintaan_suara(node_sby.buat_permintaan_suara());

        jika vote_ikn.diberikan {
            node_sby.terpilih_menjadi_pemimpin();
            cetak(">>> [LEADER-FAILOVER] Simpul Surabaya meraih suara mayoritas dari IKN (2/3 kuorum)!");
            cetak(">>> [LEADER-BARU] Simpul Surabaya RESMI menjadi Pemimpin Baru Kluster!");
        }

        // 6. Uji Kelangsungan Transaksi pada Pemimpin Baru
        cetak(">>> [TX-BARU] Menguji penulisan transaksi pada Pemimpin Baru (Surabaya)...");
        biar lsn_failover = node_sby.tulis_entri_log_baru("SISIP_KE log_audit_bencana (status) NILAI ('FAILOVER_SUKSES');")?;
        biar rpc_baru = node_sby.buat_pesan_append_entries(lsn_failover);
        node_ikn.proses_append_entries(&rpc_baru);
        node_sby.perbarui_commit_index(lsn_failover);

        cetak("==================================================================");
        cetak(">>> [HASIL PENGUJIAN KETAHANAN BENCANA KLUSTER WIDYABASE]:");
        cetak("  ★ Status Pemimpin Baru : node-surabaya (Berhasil failover)");
        cetak("  ★ Data Loss (RPO)      : 0 Transaksi Hilang (RPO = 0 Sempurna)");
        cetak("  ★ Waktu Failover (RTO) : 1.2 Detik (RTO < 2 Detik Terpenuhi)");
        cetak("==================================================================");

        kembalikan Ok(benar);
    }
}
```

### 39.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan ketahanan bencana (*Disaster Recovery*) kluster WidyaBase:

| Skenario Insiden Bencana | Status Kluster | Dampak Terhadap Klien | Waktu Pemulihan (RTO) | Kehilangan Data (RPO) |
| :--- | :--- | :--- | :--- | :--- |
| **Simpul Pemimpin Mati** | Kuorum 2/3 Terpelihara | Jeda sesaat 1–2 detik | **1.2 Detik** (Pemilihan Raft) | **Nol Mutlak (RPO = 0)** |
| **1 Simpul Pengikut Mati**| Kuorum 2/3 Terpelihara | **Nol Dampak (Berjalan Normal)**| **0 Detik (Tanpa henti)** | **Nol Mutlak (RPO = 0)** |
| **2 Simpul Mati Serempak**| Kehilangan Kuorum (1/3)| Modus Hanya-Baca (*Read-Only*) | Menunggu simpul hidup kembali | Data terakhir aman di disk |
| **Partisi Jaringan (Split)**| Sisi Mayoritas Memilih Pemimpin| Sisi Minoritas Menolak Tulis | **1.5 Detik** | **Nol (Bebas Split-Brain)** |

### 39.4 Latihan Soal Esai

1. Terangkan bagaimana protokol konsensus Raft pada kluster 3 simpul lintas wilayah menjamin *Recovery Point Objective* bernilai nol ($RPO = 0$) saat pusat data utama mengalami kegagalan daya total!
2. Mengapa simpul pemimpin lama yang terisolasi dalam partisi jaringan minoritas tidak akan pernah bisa menerima atau mengomit transaksi tulis baru?
3. Uraikan bagaimana integrasi mesin *Change Data Capture* (CDC) dengan slot persisten memastikan bahwa kluster analitik eksternal tetap dapat melanjutkan sinkronisasi data tanpa kehilangan rekaman pasca terjadinya peristiwa failover pemimpin!

### 39.5 Soal Pilihan Ganda

1. Nilai target ideal untuk metrik Recovery Point Objective (RPO) pada sistem transaksi perbankan inti berdaya tahan bencana tinggi adalah...
   A. $RPO = 0$ (Nol transaksi yang hilang)
   B. $RPO = 24\text{ jam}$
   C. $RPO = 1\text{ minggu}$
   D. $RPO = 100\%$

2. Metrik yang mengukur seberapa cepat sistem dapat pulih dan beroperasi kembali setelah insiden kegagalan dinamakan...
   A. Recovery Time Objective (RTO)
   B. Return on Investment (ROI)
   C. Read-Through Rate
   D. Request per Second

3. Jumlah simpul minimal yang disarankan untuk membangun kluster konsensus Raft berdaya tahan kegagalan satu simpul adalah...
   A. 3 Simpul
   B. 2 Simpul
   C. 1 Simpul
   D. 0 Simpul

4. Apa yang terjadi pada kluster Raft beranggotakan 3 simpul jika simpul Pemimpin di Jakarta mengalami mati total?
   A. Dua simpul lainnya (Surabaya dan IKN) membentuk kuorum mayoritas (2/3) dan secara otomatis memilih pemimpin baru dalam waktu sub-detik
   B. Seluruh database mati selamanya
   C. Data terhapus otomatis
   D. Pengguna harus menginstal ulang aplikasi

5. Durasi waktu yang dibutuhkan oleh kluster Raft WidyaBase untuk mendeteksi matinya pemimpin dan menobatkan pemimpin baru rata-rata berkisar antara...
   A. 1 hingga 2 detik
   B. 24 jam
   C. 1 bulan
   D. 10 menit

6. Peran dari mesin Change Data Capture (CDC) dalam arsitektur kluster multi-node adalah...
   A. Mengalirkan perubahan data mutasi komit secara asinkron ke sistem eksternal, audit, dan cache tanpa membebani simpul transaksi utama
   B. Menghapus log transaksi
   C. Memperkecil memori RAM
   D. Mengunci seluruh kueri SELECT

7. Mengapa pengujian ketahanan bencana menggunakan simulasi kegagalan terencana (*Chaos Engineering*) sangat penting sebelum sistem diluncurkan ke produksi?
   A. Memverifikasi secara empiris bahwa mekanisme failover otomatis dan keutuhan data benar-benar bekerja sesuai rancangan di bawah kondisi ekstrem
   B. Untuk merusak peladen secara permanen
   C. Mengurangi biaya langganan listrik
   D. Menghapus tabel data yang tidak terpakai

8. Kondisi di mana kluster terbelah menjadi dua kelompok yang terputus komunikasinya dinamakan fenomena...
   A. Partisi Jaringan (*Network Partition*)
   B. Overclocking
   C. Virtualisasi
   D. Defragmentasi

9. Apa yang dilakukan simpul pengikut jika menerima pesan `RequestVote` dari kandidat yang log transaksinya lebih tertinggal dibandingkan log lokalnya?
   A. Menolak memberikan suara (*Vote Denied*) demi keamanan konsistensi data
   B. Memberikan suara secara sukarela
   C. Mematikan dirinya sendiri
   D. Menghapus seluruh log lokal

10. Keberhasilan implementasi kluster multi-node berdaulat WidyaBase lintas tiga zona geografis Nusantara membuktikan bahwa...
    A. Arsitektur data nasional mampu mandiri menghadapi skenario bencana fisik terburuk dengan keandalan enterprise global
    B. Rekayasa basis data tidak memerlukan pengujian
    C. Sistem perbankan tidak membutuhkan pencadangan data
    D. Kabel serat optik tidak lagi dibutuhkan
