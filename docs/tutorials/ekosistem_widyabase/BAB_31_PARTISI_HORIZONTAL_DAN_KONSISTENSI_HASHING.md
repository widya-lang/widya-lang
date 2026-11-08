## Bab 31: Partisi Horizontal & Cincin Konsistensi Hashing

### 31.1 Distribusi Beban Merata: Sharding Berbasis Hash

Ketika sebuah basis data nasional harus melayani puluhan juta pengguna secara serempak, menaruh seluruh data pada satu peladen akan menimbulkan kemacetan sumber daya. Salah satu strategi pembagian data horizontal yang paling fundamental adalah **Partisi Berbasis Kunci Sharding (*Sharding Key*)**.

Namun, pendekatan pembagian naif menggunakan fungsi modulo sederhana ($\text{ServerID} = \text{Hash}(Kunci) \pmod N$):
- Memiliki cacat fatal: ketika jumlah server bertambah dari $N$ menjadi $N+1$ (atau berkurang saat satu server mati), hampir **100% data harus dipindahkan ke server baru**. Fenomena ini memicu badai migrasi data (*resharding storm*) yang melumpuhkan sistem operasional.

WidyaBase memecahkan dilema penskalaan elastis ini dengan menerapkan **Cincin Konsistensi Hashing (*Consistent Hash Ring*)** yang diperkuat oleh konsep **Node Virtual (*Virtual Nodes / V-Nodes*)** (`consensus.rs`):
- **Ruang Cincin Melingkar 64-Bit ($0 \text{ s/d } 2^{64}-1$)**: Kunci sharding dan identitas server dipetakan ke dalam satu ruang cincin melingkar yang sama.
- **Dampak Penambahan Simpul Minimal ($1/N$)**: Saat sebuah server baru ditambahkan ke dalam kluster, sistem hanya perlu memindahkan sebagian kecil data ($K/N$ bagian) dari simpul tetangga terdekatnya, tanpa mengganggu simpul lainnya di dalam cincin.
- **Penyebaran Beban Homogen via V-Nodes**: Setiap server fisik dipetakan ke ratusan titik simpul virtual acak di sepanjang cincin (misal 256 V-Nodes per mesin fisik), mencegah terbentuknya titik panas beban data (*hotspots*).

```mermaid
flowchart TD
    subgraph CincinKonsistensi["Cincin Konsistensi Hashing WidyaBase (Consistent Hash Ring)"]
        R0["Posisi 0 / 2^64-1"]
        V1["Node Fisik A (V-Node A1)"]
        V2["Node Fisik B (V-Node B1)"]
        V3["Node Fisik C (V-Node C1)"]
        V4["Node Fisik A (V-Node A2)"]
        V5["Node Fisik B (V-Node B2)"]
        V6["Node Fisik C (V-Node C2)"]
    end
    K1["Kunci Sharding: NIK Nasabah '3201...' (Hash: 0x4F12A...)"] -->|Searah Jarum Jam (Clockwise Walk)| V2
    V2 -->|Ditampung di Server Fisik B| S_B["Server Fisik B (Data Center Surabaya)"]
```

Aturan partisi horizontal di WidyaBase:
1. **Penentuan Sharding Key Cerdas**: Menentukan kunci distribusi utama tabel (misalnya: `nik`, `nomor_rekening`, atau `uuid_pelanggan`) yang meminimalkan kueri lintas shard.
2. **Penelusuran Searah Jarum Jam (*Clockwise Traversal*)**: Nilai hash kunci dicocokkan dengan posisi simpul pertama yang bernilai lebih besar atau sama dengannya pada cincin ($O(\log V)$ via pencarian biner pada tabel cincin).
3. **Replikasi Cincin $N$-Rangkap**: Untuk menjamin redundansi data, data tidak hanya disimpan pada simpul utama pertama, melainkan direplikasi secara otomatis ke $R-1$ simpul fisik berikutnya di sepanjang putaran cincin.

### 31.2 Implementasi Kode Program Utuh

Berikut implementasi struktur data Cincin Konsistensi Hashing dengan Virtual Nodes pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Partisi Horizontal dan Cincin Konsistensi Hashing WidyaBase
modul cincin_hashing_konsisten {
    impor ekosistem::widyabase::{
        WidyaBase, CincinHash, SimpulFisik, TitikCincin,
        FungsiHashMurmur3, HasilRuteSimpul
    };

    konstanta JUMLAH_VIRTUAL_NODES_PER_MESIN: Bulat = 256;

    struktur CincinKonsistenWidya {
        titik_cincin_terurut: Daftar<TitikCincin>,
        peta_simpul_fisik: Kamus<Teks, SimpulFisik>,
    }

    implementasi CincinKonsistenWidya {
        fungsi buat() -> Diri {
            kembalikan CincinKonsistenWidya {
                titik_cincin_terurut: [],
                peta_simpul_fisik: Kamus::baru(),
            };
        }

        fungsi tambah_simpul_fisik(&mut diri, nama_server: Teks, alamat_ip: Teks) {
            biar server = SimpulFisik::buat(nama_server.klon(), alamat_ip);
            diri.peta_simpul_fisik.sisipkan(nama_server.klon(), server);

            // Bangun 256 V-Nodes acak di sepanjang cincin
            untuk i di 0..JUMLAH_VIRTUAL_NODES_PER_MESIN {
                biar token_nama = nama_server + "#vnode#" + teks(i);
                biar nilai_hash = FungsiHashMurmur3::hitung_hash64(&token_nama);
                diri.titik_cincin_terurut.tambah(TitikCincin {
                    nilai_hash: nilai_hash,
                    nama_server_fisik: nama_server.klon(),
                });
            }

            // Urutkan titik cincin secara menaik untuk pencarian biner O(log V)
            diri.titik_cincin_terurut.urutkan_berdasarkan_hash();
            cetak(">>> [HASH-RING] Simpul '" + nama_server + "' terdaftar dengan 256 Virtual Nodes.");
        }

        fungsi temukan_simpul_target(&diri, kunci_data: &Teks) -> Teks {
            biar hash_kunci = FungsiHashMurmur3::hitung_hash64(kunci_data);

            // Lakukan penelusuran biner searah jarum jam (Clockwise Search)
            biar indeks = diri.titik_cincin_terurut.cari_biner_pertama_lebih_besar(hash_kunci);
            cocokkan indeks {
                Ada(idx) => kembalikan diri.titik_cincin_terurut[idx].nama_server_fisik.klon(),
                TidakAda => kembalikan diri.titik_cincin_terurut[0].nama_server_fisik.klon(), // Melingkar ke awal cincin
            }
        }
    }

    fungsi demonstrasi_routing_sharding() -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [SHARDING] Membangun Kluster Sharding Berdaulat Nasional...");

        biar mut cincin = CincinKonsistenWidya::buat();
        cincin.tambah_simpul_fisik("shard-jakarta-01", "10.0.1.10");
        cincin.tambah_simpul_fisik("shard-surabaya-02", "10.0.2.20");
        cincin.tambah_simpul_fisik("shard-makassar-03", "10.0.3.30");

        // Uji Perutean Kunci Sharding NIK Kependudukan
        biar daftar_nik = ["3201012345670001", "3578021122330002", "7371034455660003", "3171059988770004"];

        cetak(">>> [ROUTING] Memetakan NIK ke Shard Fisik:");
        untuk nik di daftar_nik {
            biar shard_tujuan = cincin.temukan_simpul_target(&nik);
            cetak("  - NIK: " + nik + " -> Dirutekan ke Peladen: " + shard_tujuan);
        }

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 31.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan metode sharding data terdistribusi pada WidyaBase:

| Metode Pembagian Shard | Kompleksitas Rute Kueri | Dampak Penambahan Server Baru | Distribusi Beban Data |
| :--- | :--- | :--- | :--- |
| **Modulo Hash Naif** ($H \pmod N$) | $O(1)$ Sederhana | **Bencana (100% Data Bergeser/Reshard)**| Rawan ketimpangan |
| **Partisi Rentang (Range Sharding)**| $O(\log S)$ Metadata Lookup| Rendah (Hanya belah partisi penuh)| **Sangat Rawan Titik Panas (Hotspot)**|
| **Cincin Konsisten + V-Nodes (WidyaBase)**| $O(\log V)$ Pencarian Biner | **Minimalis (Hanya $1/N$ data berpindah)**| **Sangat Merata & Seimbang (Homogen)**|
| **Partisi Daftar (List Sharding)** | $O(1)$ Pemetaan Wilayah | Membutuhkan modifikasi aturan manual| Tergantung volume per wilayah |

### 31.4 Latihan Soal Esai

1. Mengapa teknik sharding berbasis modulo matematika sederhana ($\text{Hash}(K) \pmod N$) sangat dihindari dalam sistem basis data cloud elastis modern?
2. Terangkan mekanisme kerja struktur *Consistent Hash Ring* dan bagaimana algoritma penelusuran searah jarum jam (*clockwise lookup*) memetakan kunci data ke server fisik!
3. Apa peran krusial dari konsep *Virtual Nodes* (V-Nodes) dalam mencegah timbulnya fenomena ketimpangan beban data (*data skew / hotspotting*) pada kluster sharding?

### 31.5 Soal Pilihan Ganda

1. Keuntungan utama dari algoritma Consistent Hashing dibandingkan pembagian modulo biasa saat terjadi penambahan server baru ke dalam kluster adalah...
   A. Hanya sebagian kecil data ($1/N$) yang perlu dipindahkan dari server lama ke server baru
   B. Seluruh database harus diformat ulang
   C. Kecepatan kipas server berkurang
   D. Memori RAM bertambah dua kali lipat

2. Representasi ruang nilai kunci yang digunakan pada Cincin Konsistensi Hashing umumnya memetakan nilai dari...
   A. Posisi 0 hingga $2^{64}-1$ secara melingkar tertutup
   B. Angka minus seribu sampai plus seribu
   C. Huruf A sampai Z
   D. Jumlah hari dalam kalender

3. Arah penelusuran pada cincin hash untuk menemukan simpul server yang bertanggung jawab menampung suatu kunci data adalah...
   A. Penelusuran searah jarum jam (*Clockwise Search*)
   B. Penelusuran acak bolak-balik
   C. Penelusuran berlawanan arah jarum jam
   D. Penelusuran diagonal

4. Peran dari Virtual Nodes (V-Nodes) pada Cincin Konsistensi Hashing adalah...
   A. Memetakan setiap satu mesin fisik ke ratusan titik token di sepanjang cincin agar sebaran data merata homogen
   B. Menggantikan prosesor fisik dengan prosesor virtual
   C. Menghubungkan peladen ke jaringan satelit
   D. Mematikan kueri SQL yang berat

5. Algoritma fungsi hash non-kriptografi berkecepatan tinggi yang umum digunakan untuk menghitung posisi cincin token sharding adalah...
   A. MurmurHash3 / xxHash
   B. RSA 4096
   C. MD5 Kuno
   D. Caesar Cipher

6. Kondisi di mana salah satu server menerima beban transaksi 90% lebih berat dibandingkan server lainnya karena pemilihan sharding key yang buruk dinamakan...
   A. Hotspot / Data Skew
   B. Deadlock Loop
   C. Cache Hit
   D. Zero-Copy

7. Kompleksitas waktu untuk menemukan server tujuan pada cincin yang memiliki $V$ buah Virtual Nodes menggunakan pencarian biner adalah...
   A. $O(\log V)$
   B. $O(V^2)$
   C. $O(V!)$
   D. $O(N \times M)$

8. Bagaimana Cincin Konsistensi Hashing menangani ketersediaan tinggi (*High Availability*) jika simpul utama data mati?
   A. Salinan data direplikasi ke $R-1$ simpul tetangga fisik berikutnya di sepanjang putaran cincin
   B. Data otomatis dihapus dari memori
   C. Pengguna diminta mengetik ulang data
   D. Server mengirim sinyal alarm kebakaran

9. Sharding Key yang paling buruk untuk tabel transaksi perbankan dengan partisi rentang waktu adalah...
   A. Kolom tanggal hari ini (karena seluruh transaksi baru akan menumpuk di satu server hari ini saja)
   B. UUID acak
   C. Nomor rekening nasabah
   D. NIK terdistribusi

10. Mengapa pemahaman arsitektur partisi horizontal sangat penting bagi insinyur basis data nasional berdaulat?
    A. Memungkinkan penskalaan sistem komputasi kependudukan hingga ratusan juta warga secara elastis tanpa batas perangkat keras tunggal
    B. Supaya tidak perlu membeli lisensi sistem operasi
    C. Menghilangkan kebutuhan kabel listrik di data center
    D. Mengubah komputer menjadi radio
