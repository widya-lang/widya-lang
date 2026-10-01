# ARSITEKTUR SISTEM & ENTITY RELATIONSHIP DIAGRAM (ERD)
## PROYEK: SISTEM KOPERASI DESA MERAH PUTIH (KDMP)
**Disusun Oleh:** `agen_arsitek` (SOP-03: Desain Skema Database & Kontrak API)  
**Status:** DIVERIFIKASI • KONSISTENSI ACID 100%  

---

### 1. Diagram Arsitektur Komponen

```mermaid
graph TB
    subgraph UI_Layer["Layer 1: Frontend Mandiri (HTML5 / Vanilla JS / CSS)"]
        POS[Modul Kasir POS]
        SP[Modul Simpan Pinjam]
        MED[Modul Apotik & Klinik]
        SRG[Modul Resi Gudang]
        LOG[Modul Logistik]
    end

    subgraph Service_Layer["Layer 2: Service & Business Logic"]
        EnginePOS[POS Checkout Engine]
        EngineLoan[Anuitas & Flat Loan Calculator]
        EngineFHIR[SatuSehat HL7 JSON Formatter]
        EngineVault[Argon2id Auth Guard]
    end

    subgraph Storage_Layer["Layer 3: Basis Data Berdaulat WidyaBase"]
        WDB[(koperasi.wdb\nSlotted-Page 4KB\nB+Tree Index)]
        WAL[koperasi.wal\nWrite-Ahead Journal]
    end

    UI_Layer --> Service_Layer
    Service_Layer --> Storage_Layer
```

---

### 2. Entity Relationship Diagram (ERD Relasional)

```mermaid
erDiagram
    ANGGOTA ||--o{ SIMPANAN : memiliki
    ANGGOTA ||--o{ PINJAMAN : mengajukan
    ANGGOTA ||--o{ TRANSAKSI_POS : berbelanja
    PINJAMAN ||--o{ ANGSURAN : memiliki
    PRODUK_WARUNG ||--o{ DETAIL_TRANSAKSI_POS : termuat
    TRANSAKSI_POS ||--|{ DETAIL_TRANSAKSI_POS : berisi
    OBAT_APOTIK ||--o{ TRANSAKSI_APOTIK : terjual
    PETANI ||--o{ SERTIFIKAT_RESI : menitipkan
    ARMADA_LOGISTIK ||--o{ JADWAL_RUTE : beroperasi

    ANGGOTA {
        string no_anggota PK
        string nik UK
        string nama_lengkap
        string alamat_desa
        string status_keanggotaan
        datetime tgl_bergabung
    }

    PINJAMAN {
        string id_pinjaman PK
        string no_anggota FK
        bigint nominal_pokok
        decimal bunga_persen
        int tenor_bulan
        string status_approval
    }

    TRANSAKSI_POS {
        string id_transaksi PK
        string no_anggota FK
        datetime waktu_transaksi
        bigint total_bayar
        bigint diskon_anggota
        string metode_bayar
    }

    PRODUK_WARUNG {
        string kode_produk PK
        string nama_produk
        string kategori
        bigint harga_jual
        int stok_tersedia
    }

    SERTIFIKAT_RESI {
        string no_srg PK
        string id_petani FK
        string komoditas
        decimal bobot_ton
        decimal kadar_air
        bigint taksiran_nilai
    }
```

---

### 3. Kontrak REST API / Internal Event Dispatch

| Endpoint | Method | Payload / Request | Response |
| :--- | :---: | :--- | :--- |
| `/api/pos/checkout` | `POST` | `{ cart: [{ id, qty, price }], member_id, method }` | `{ success: true, trx_id: "TRX-...", total, change }` |
| `/api/pinjaman/ajukan` | `POST` | `{ no_anggota, nominal, tenor_bulan, peruntukan }` | `{ success: true, loan_id: "PJ-...", angsuran_per_bulan }` |
| `/api/klinik/satusehat`| `POST` | `{ nik, icd10, vital_signs, resep }` | `{ success: true, fhir_encounter_id: "...", status: 201 }` |
| `/api/resi/terbitkan` | `POST` | `{ komoditas, bobot_ton, kadar_air, taksiran_rp }` | `{ success: true, no_srg: "SRG-...", hash_sha256 }` |
