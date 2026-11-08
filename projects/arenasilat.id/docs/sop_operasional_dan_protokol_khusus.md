# Audit Operasional Spesifik Lapangan & SOP Eksepsional Turnamen Pencak Silat (Arena Silat)

Dokumen ini memuat analisis dan rancangan penanganan untuk 5 kondisi lapangan kritis yang sering terjadi di turnamen Pencak Silat: sistem panggilan atlet (*Marshalling*), penanganan pesilat cedera & protokol gegar otak (*Concussion Protocol*), sidang banding sengketa wasit (*Jury Dispute Protocol*), formula penentuan Pesilat Terbaik & Juara Umum, serta mekanisme pencetakan massal ID Card & Sertifikat offline.

---

## 1. Sistem Pemanggilan Atlet & Ruang Tunggu (Marshalling & Call Room)

### Permasalahan Lapangan:
Partai pertandingan sering tertunda karena pesilat belum memakai perlengkapan pelindung (*body protector*, pelindung kemaluan/cup, pelindung tulang kering) atau masih berada di tribun/luar gedung saat nomor partainya dipanggil.

### Solusi Sistem di Arena Silat:
```mermaid
graph LR
    A[Papan Status Marshalling / Call Room Screen] --> B[Panggilan I: H-3 Partai / 15 Menit Sebelum Main]
    B --> C[Panggilan II: H-1 Partai / Masuk Ruang Pemakaian Body Protector & Cek Sabuk/Kuku]
    C --> D[Panggilan III: Siap di Pintu Masuk Gelanggang]
    D -->|Atlet Tidak Hadir setelah 3x Panggilan 1 Menit| E[Auto-Status: Diskualifikasi / Walkover - WO]
```

- **Fitur Notifikasi WhatsApp / Telegram Otomatis ke Manager Kontingen**: Mengirim pesan bot saat partai atletnya kurang 3 partai lagi (*"Pesilat Ahmad Sudut Merah harap segera menuju Ruang Marshalling Gelanggang 2"*).
- **Tabel `match_marshalling`**:
  ```sql
  CREATE TABLE match_marshalling (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      match_participant_id UUID NOT NULL REFERENCES match_participants(id) ON DELETE CASCADE,
      call_1_at TIMESTAMP,
      call_2_at TIMESTAMP,
      call_3_at TIMESTAMP,
      is_equipment_verified BOOLEAN DEFAULT FALSE, -- Cek body protector, deker, pelindung gigi
      status VARCHAR(50) DEFAULT 'waiting' -- 'waiting', 'in_call_room', 'ready_at_court', 'walkover_no_show'
  );
  ```

---

## 2. Protokol Medis Khusus, Timer Pemulihan & Penanganan Gegar Otak (Concussion Protocol)

### Permasalahan Lapangan:
Saat atlet terkena pukulan/tendangan telak atau benturan kepala, wasit menghentikan laga dan memanggil tim medis. Ada batasan waktu penanganan medis yang ketat sebelum wasit memutuskan apakah laga dilanjutkan atau atlet dinyatakan tidak layak tanding (*Medical Disqualification/WMP*).

```mermaid
graph TD
    A[Wasit Panggil Medis: Dokter Masuk Matras] --> B[Operator Mengaktifkan Medical Recovery Timer: Maks 3 Menit]
    B --> C{Evaluasi Dokter Pertandingan}
    C -->|Atlet Pulih & Sadar Penuh| D[Laga Dilanjutkan]
    C -->|Cedera Serius / Diduga Gegar Otak / Waktu Habis| E[Dokter Rekomendasikan Stop Laga]
    E --> F{Penyebab Cedera}
    F -->|Akibat Pelanggaran Ilegal Lawan| G[Lawan Dikenakan Sanksi / Didiskualifikasi]
    F -->|Akibat Teknik Sah / Jatuh Sendiri| H[Lawan Dinyatakan Menang Teknik / WMP]
```

- **Timer Khusus Medis di Scoreboard**: Layar arena menampilkan hitung mundur waktu perawatan medis (standar 3 menit kumulatif per partai).
- **Red-Flag Medical Suspension**: Jika pesilat kalah K.O benturan kepala/gegar otak, sistem otomatis menandai profil atlet di tabel database: `athletes.medical_suspension_until = NOW() + INTERVAL '30 days'` (atlet dilarang didaftarkan ke turnamen lain selama 30 hari demi keselamatan otak).

---

## 3. Tata Cara Sidang Banding Sengketa Resmi (Official Dispute & Protest Protocol)

Jika sebuah kontingen merasa dirugikan dan mengajukan protes resmi tertulis ke Dewan Hakim:

```mermaid
sequenceDiagram
    autonumber
    participant Ofisial as Manager Kontingen
    participant Sekretariat as Sekretariat Pertandingan / Dewan Hakim
    participant DB as Sistem Arena Silat

    Ofisial->>Sekretariat: 1. Serahkan Formulir Protes + Uang Jaminan (Rp 3.000.000) dlm waktu maks 15 menit pasca laga
    Sekretariat->>DB: 2. Input Kasus Banding & Freeze Status Pemenang Partai Tersebut
    Sekretariat->>Sekretariat: 3. Dewan Hakim Sidang Tertutup (Review Log Skor, Putar Video Replay)
    Sekretariat->>DB: 4. Input Keputusan Sidang (Protes Diterima / Ditolak)
    alt Protes Diterima
        DB->>DB: Update Pemenang Partai & Kembalikan Uang Jaminan
    else Protes Ditolak
        DB->>DB: Status Pemenang Tetap & Uang Jaminan Masuk Kas Panitia
    end
    DB->>Ofisial: 5. Terbitkan Surat Keputusan Dewan Hakim Ber-Barcode Resmi
```

---

## 4. Formula Perhitungan Juara Umum & Pesilat Terbaik (Piala Bergilir Engine)

Aplikasi SaaS secara otomatis menghitung klasemen kontingen dan peraih gelar individu tanpa perdebatan manual:

### A. Klasemen Perolehan Medali Kontingen (Medal Tally Standings)
Urutan peringkat dihitung berdasarkan:
1. Jumlah **Medali Emas** terbanyak.
2. Jika emas sama, dihitung jumlah **Medali Perak** terbanyak.
3. Jika perak sama, dihitung jumlah **Medali Perunggu** terbanyak.
4. Jika seluruh medali identik, dihitung kontingen dengan **jumlah pesilat terbanyak yang dikirim**.

### B. Formula Pesilat Terbaik Putra & Putri (Best Athlete Algorithm)
Dihitung otomatis berdasarkan skor agregat performa:
$$\text{Score Pesilat Terbaik} = (\text{Total Poin Masuk Laga}) - (2 \times \text{Poin Pelanggaran}) + (\text{Bonus K.O/WMP} \times 10)$$
*(Pesilat dengan rasio kemenangan telak dan disiplin pelanggaran terminimal mendapat predikat Pesilat Terbaik).*

---

## 5. Mode Cetak Massal Offline (High-Volume Batch Generator)

Untuk turnamen besar dengan 1.500+ atlet:
- **Batch Generator ID Card**: Menghasilkan 1 file PDF gabungan berisi seluruh ID Card peserta per kontingen (format siap potong A4/A6 dengan foto, barcode presensi, dan QR Code gelanggang).
- **Dynamic Certificate Auto-Fill**: Begitu partai final selesai dan wasit mengangkat tangan pemenang, e-piagam juara 1, 2, dan 3 langsung ter-*generate* detik itu juga dan dapat langsung dicetak oleh tim sekretariat di venue atau diunduh langsung oleh pelatih dari dashboard HP mereka.
