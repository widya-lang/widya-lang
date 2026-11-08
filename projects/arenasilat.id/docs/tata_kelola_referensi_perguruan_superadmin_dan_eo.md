# Arsitektur Tata Kelola & Hirarki Referensi Perguruan Pencak Silat (Global SuperAdmin vs Event Local)

Dokumen ini mendefinisikan arsitektur pengelolaan referensi master perguruan silat dengan skema hibrida (**Global Master vs Custom Event Overrides**) pada platform **Arena Silat Enterprise**.

---

## 1. Konsep & Filosofi Bisnis

Dalam ekosistem pencak silat Indonesia dan internasional:
1. **Master Referensi Global (SuperAdmin / PB IPSI)**:
   - Terdiri dari 10 Perguruan Historis Pendiri IPSI, perguruan anggota resmi IPSI, serta perguruan internasional terdaftar di PERSILAT.
   - Bersifat *Read-Only* bagi penyelenggara turnamen biasa untuk menjaga standarisasi e-sertifikat, integrasi Puspresnas SIMT, dan data ranking nasional.
2. **Kebutuhan Fleksibilitas Penyelenggara (Event Organizer / EO)**:
   - Di turnamen tingkat daerah/kampus/lokal, sering muncul sasana baru, paguyuban lokal, perguruan tradisional kampung, atau ranting independen yang belum terdaftar di master nasional.
   - Penyelenggara event **diberikan hak untuk menambahkan perguruan lokal (Custom Event School)** khusus untuk event mereka tanpa merusak master database global.
3. **Pemberdayaan SuperAdmin (Promote to Global)**:
   - SuperAdmin memiliki dashboard untuk meninjau usulan perguruan lokal dari para EO.
   - SuperAdmin dapat meng-approve dan mempromosikan perguruan lokal tersebut menjadi **Global Master Reference** yang dapat digunakan oleh seluruh turnamen di masa mendatang.

---

## 2. Skema Relasional Database (PostgreSQL DDL)

```sql
-- 1. Tabel Master Referensi Perguruan Silat
CREATE TABLE master_schools (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(30) UNIQUE, -- e.g. 'TS', 'PSHT', 'PD'
    name VARCHAR(255) NOT NULL,
    short_name VARCHAR(100) NOT NULL,
    founding_year INT,
    historical_status VARCHAR(50) DEFAULT 'STANDARD', -- 'HISTORIS_IPSI', 'NASIONAL', 'TRADISIONAL_LOKAL'
    logo_url TEXT,
    badge_emoji VARCHAR(10) DEFAULT '⚔️',
    color_theme VARCHAR(20) DEFAULT '#3b82f6',
    headquarter_city VARCHAR(100),
    is_global_verified BOOLEAN DEFAULT TRUE, -- TRUE: Dikelola SuperAdmin, FALSE: Usulan EO
    created_by_tenant_id UUID, -- NULL jika dibuat oleh SuperAdmin
    status VARCHAR(30) DEFAULT 'APPROVED', -- 'APPROVED', 'PENDING_REVIEW', 'REJECTED'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 2. Index Pencarian Cepat
CREATE INDEX idx_master_schools_search ON master_schools(name, short_name, code);
CREATE INDEX idx_master_schools_tenant ON master_schools(created_by_tenant_id, is_global_verified);
```

---

## 3. Matriks Hak Akses (RBAC)

| Aksi Pengelolaan | SuperAdmin (PB IPSI / Developer) | Event Organizer (Penyelenggara) |
| :--- | :---: | :---: |
| Tambah Master Global Resmi | ✅ YA (Langsung Approved & Muncul di Semua Event) | ❌ Tidak Bisa |
| Edit / Hapus Master Global | ✅ YA | ❌ Tidak Bisa |
| Tambah Perguruan Lokal Baru | ✅ YA | ✅ YA (Khusus Event Berjalan) |
| Ajukan ke Master Global | N/A (SuperAdmin Approval) | ✅ YA (Status: PENDING_REVIEW) |
| Approve Usulan EO ke Global | ✅ YA (Promote to Master 1-Klik) | ❌ Tidak Bisa |
