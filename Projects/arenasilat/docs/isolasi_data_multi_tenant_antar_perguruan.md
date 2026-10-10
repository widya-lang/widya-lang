# Arsitektur Isolasi Data Multi-Tenant Antar-Perguruan Silat (Guild Data Isolation & Zero-Leakage Architecture)

Dokumen ini memuat analisis dan rancangan arsitektur keamanan tingkat tinggi untuk memastikan bahwa **setiap perguruan pencak silat (Tapak Suci, PSHT, Perisai Diri, Merpati Putih, IKSPI, Pagar Nusa, dll) TERISOLASI 100% (Strict Multi-Tenant Isolation)**, sehingga satu perguruan **MUTLAK TIDAK BISA MELIHAT ATAU MENGAKSES** data anggota, materi kurikulum rahasia, catatan taktis pelatih, data presensi ranting, maupun keuangan perguruan lain.

---

## 1. Prinsip Keamanan Isolasi Multi-Tenant Antar-Perguruan

```mermaid
graph TD
    subgraph Multi-Tenant Database Layer (Shared-Database, Isolated-Tenant)
        PSHT_DATA[(Tenant PSHT: 100% Terisolasi)]
        TS_DATA[(Tenant Tapak Suci: 100% Terisolasi)]
        PD_DATA[(Tenant Perisai Diri: 100% Terisolasi)]
        MP_DATA[(Tenant Merpati Putih: 100% Terisolasi)]
    end

    subgraph Security Enforcement Engines
        RLS[1. PostgreSQL Row-Level Security / RLS Policy]
        JWT[2. Cryptographic JWT Org Token Verification]
        GUARD[3. API Gateway Tenant Context Middleware]
    end

    USER_TS[Admin / Pelatih Tapak Suci] --> JWT
    USER_PSHT[Admin / Pelatih PSHT] --> JWT

    JWT --> GUARD
    GUARD --> RLS

    RLS -->|HANYA BISA AKSES| TS_DATA
    RLS -->|HANYA BISA AKSES| PSHT_DATA

    USER_TS -.->|DIBLOKIR TOTAL (Access Denied 403)| PSHT_DATA
    USER_PSHT -.->|DIBLOKIR TOTAL (Access Denied 403)| TS_DATA
```

---

## 2. Tiga Lapis Perlindungan Isolasi Data (3-Tier Security Defenses)

### A. Lapis 1: Token Autentikasi Kriptografi (JWT Tenant Claims)
- Setiap kali pengurus atau pelatih login, server menerbitkan token JWT bertanda tangan digital yang memuat klaim identitas perguruan:
  ```json
  {
    "userId": "usr_99812",
    "guildId": "guild_tapak_suci_001",
    "guildBranchId": "branch_sleman_04",
    "role": "guild_branch_admin"
  }
  ```
- Setiap *request* API yang dikirim wajib menyertakan token ini. Jika ID Perguruan tidak cocok dengan data yang diminta, server langsung melempar error **`403 FORBIDDEN: UNAUTHORIZED GUILD ACCESS`**.

---

### B. Lapis 2: Middleware Penegakan Konteks API (API Gateway Context Guard)
- Di level backend Node.js (Fastify Middleware), sistem secara otomatis menyuntikkan `guild_id` ke dalam setiap operasi database:
  ```typescript
  // Middleware Otomatis: Memastikan query selalu terkunci pada guild_id milik user
  fastify.addHook('preHandler', async (request, reply) => {
    const user = request.user;
    if (user.role.startsWith('guild_')) {
      request.scopedGuildId = user.guildId; // Mengunci query ke perguruan user
    }
  });
  ```

---

### C. Lapis 3: Proteksi Basis Data Paling Rendah (PostgreSQL Row-Level Security / RLS)
Bahkan jika ada celah bug di level aplikasi atau serangan SQL Injection, **database PostgreSQL itu sendiri yang menolak akses** jika kueri mencoba mengambil baris data perguruan lain:

```sql
-- 1. Aktifkan Row-Level Security pada Tabel Master Perguruan & Ranting
ALTER TABLE athletes ENABLE ROW LEVEL SECURITY;
ALTER TABLE guild_rantings ENABLE ROW LEVEL SECURITY;
ALTER TABLE ranting_training_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE student_attendance_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE athlete_tactical_evaluations ENABLE ROW LEVEL SECURITY;
ALTER TABLE belt_grading_sessions ENABLE ROW LEVEL SECURITY;

-- 2. Kebijakan RLS untuk Atlet: Admin/Pelatih Perguruan HANYA bisa membaca data atlet perguruannya sendiri
CREATE POLICY guild_athlete_isolation_policy ON athletes
    FOR ALL
    USING (
        guild_id = NULLIF(current_setting('app.current_guild_id', true), '')::UUID
        OR 
        -- Super Admin Platform tetap bisa audit global
        current_setting('app.current_user_role', true) = 'super_admin'
    );

-- 3. Kebijakan RLS untuk Catatan Taktis & Rahasia Pelatih
CREATE POLICY coach_tactical_isolation_policy ON athlete_tactical_evaluations
    FOR ALL
    USING (
        athlete_id IN (
            SELECT id FROM athletes 
            WHERE guild_id = NULLIF(current_setting('app.current_guild_id', true), '')::UUID
        )
    );

-- 4. Kebijakan RLS untuk Presensi Latihan Ranting
CREATE POLICY ranting_training_isolation_policy ON ranting_training_sessions
    FOR ALL
    USING (
        ranting_id IN (
            SELECT id FROM guild_rantings 
            WHERE guild_id = NULLIF(current_setting('app.current_guild_id', true), '')::UUID
        )
    );
```

---

## 3. Matriks Batasan Akses Data Antar-Perguruan (Strict Privacy Matrix)

| Kategori Data | Apakah Bisa Dilihat Perguruan Lain? | Penjelasan Teknis & Privasi |
|---|---|---|
| **Data Latihan Ranting & Presensi Siswa** | 🔴 **TIDAK BISA MUTLAK** | Terkunci 100% hanya untuk DPP/Pengda/Cabang perguruan bersangkutan. |
| **Catatan Taktik & Radar Kelemahan Atlet** | 🔴 **TIDAK BISA MUTLAK (RAHASIA)** | Catatan strategi pelatih terenkripsi dan terisolasi per perguruan. |
| **Data Keuangan Kas Ranting / Iuran UKT** | 🔴 **TIDAK BISA MUTLAK** | Seluruh mutasi kas hanya bisa dilihat bendahara perguruan terkait. |
| **Kurikulum & Materi Ujian Sabuk Internal** | 🔴 **TIDAK BISA MUTLAK** | Setiap perguruan memiliki kurikulum jurus privat masing-masing. |
| **Rekor Pertandingan Publik di Turnamen** | 🟢 **BISA DILIHAT (PUBLIK)** | Skor resmi kejuaraan, medali, dan Elo rating di turnamen terbuka (karena turnamen adalah domain publik). |
| **Nama Pesilat di Bagan Pertandingan** | 🟢 **BISA DILIHAT (PUBLIK)** | Nama dan sudut matras yang sedang tanding di kejuaraan. |

---

## 4. Log Audit Upaya Akses Ilegal (Cross-Tenant Intrusion Detection)

Jika ada pengguna yang mencoba memodifikasi ID di browser untuk melihat data perguruan lain (misal: mengganti URL `/guild/tapak-suci/students` menjadi `/guild/psht/students`):

```sql
-- Tabel Deteksi Pelanggaran Batas Antar-Perguruan (Cross-Tenant Security Alerts)
CREATE TABLE cross_tenant_access_violations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    attempted_by_user_id UUID NOT NULL REFERENCES users(id),
    user_guild_id UUID NOT NULL REFERENCES martial_arts_guilds(id),
    target_guild_id UUID NOT NULL REFERENCES martial_arts_guilds(id),
    endpoint_attempted VARCHAR(255) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    user_agent TEXT,
    attempted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tindakan Otomatis Sistem:
-- 1. Blokir request seketika (Return 403 Forbidden).
-- 2. Catat log ke tabel cross_tenant_access_violations.
-- 3. Jika terjadi > 3 kali dalam 1 jam, akun pengguna otomatis di-SUSPEND sementara demi keamanan data.
```
