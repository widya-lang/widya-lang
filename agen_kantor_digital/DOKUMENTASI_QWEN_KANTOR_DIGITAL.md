# 🧠 Panduan Arsitektur & Operasional: Kantor Digital Widya Berbasis Qwen 2.5 Coder

Dokumen ini menjelaskan implementasi resmi kecerdasan buatan lokal berdaulat (*Sovereign Local AI*) yang mentenagai **10 AI Agent Kantor Digital Widya** menggunakan model **Qwen 2.5 Coder 7B Instruct**.

---

## 🏛️ 1. Latar Belakang & Filosofi Kedaulatan Digital

Ekosistem Kantor Digital Widya dirancang untuk beroperasi secara mandiri (*self-hosted* dan *air-gapped*) guna mematuhi:
* **UU No. 27/2022 tentang Pelindungan Data Pribadi (UU PDP)**: Data rahasia instansi, kode sumber, risalah rapat, dan identitas karyawan dilarang bocor ke peladen awan (*cloud*) asing.
* **ISO/IEC 27001:2022 & NIST SP 800-53**: Keamanan informasi terisolasi (*air-gapped sovereign deployment*).
* **Efisiensi Infrastruktur**: Menjalankan inferensi cerdas langsung di perangkat keras lokal (CPU x86_64 dengan AVX-512, GPU AMD Radeon/RDNA3, atau GPU NVIDIA RTX).

Pemilihan **Qwen 2.5 Coder** sebagai model fondasi utama didasarkan pada:
1. **Akurasi Pemrograman Kelas Dunia**: Qwen 2.5-Coder menguasai *multi-file reasoning*, sintaks DDL/DML, dan sintaks bahasa baru seperti **Widya-Lang**.
2. **Kefasihan Bahasa Indonesia Formal**: Mampu menyusun risalah rapat, notula Obsidian, dan briefing Telegram dengan bahasa formal dan terstruktur.
3. **Ukuran Kompak & Kuantisasi Efisien**: Model 7B pada kuantisasi GGUF `Q4_K_M` (~4.4 GB) dapat dijalankan di RAM/VRAM laptop kerja standar.

---

## ⚙️ 2. Arsitektur Multi-Tier Inference Pipeline

Untuk menjaga kecepatan rapat (latensi sub-detik) tanpa membebani komputasi secara berlebih, sistem menggunakan pipa inferensi bertingkat (*multi-tier cascade*):

```
                        Permintaan Penalaran Agen
                                    │
                                    ▼
                     ┌─────────────────────────────┐
                     │  Tier 1: Semantic Cache     │ ── Hit (0ms) ──► Hasil Instan
                     └──────────────┬──────────────┘
                                    │ Miss
                                    ▼
                     ┌─────────────────────────────┐
                     │  Tier 2: Qwen 2.5 Coder     │ ── Sukses ─────► Simpan ke Cache
                     │  (Ollama / localhost:11434) │                  & Kembalikan Hasil
                     └──────────────┬──────────────┘
                                    │ Timeout / Offline
                                    ▼
                     ┌─────────────────────────────┐
                     │  Tier 3: Heuristik Native   │ ── 100% Siap ──► Respons Andal
                     │  (Air-Gapped Sovereign Fall)│                  Tanpa Crash
                     └─────────────────────────────┘
```

---

## 👥 3. Matriks Persona & SOP 10 AI Agent

Setiap agen diinjeksi dengan *System Persona Prompt* dan SOP khusus:

| ID Agen | Nama Peran | SOP Acuan | Persona & Fokus Qwen |
| :--- | :--- | :---: | :--- |
| `agen_orkestrator` | Mandor Utama / PM | SOP-01 | Kepemimpinan Agile sprint, perumusan prioritas Eisenhower Matrix (P1-P3), resolusi deadlock, dan disiplin timeline. |
| `agen_analis` | Analis Bisnis & Produk | SOP-02 | Dekomposisi PRD/BRD, perumusan User Stories, Acceptance Criteria, dan kepatuhan BABOK. |
| `agen_arsitek` | Arsitek Sistem & Database | SOP-03 | Skema basis data ACID WidyaBase, normalisasi tabel B+Tree, kontrak antarmuka API, dan arsitektur TOGAF. |
| `agen_coder` | Lead Developer Widya | SOP-04 | Sintesis logika program dialek Nusantara Widya-Lang (`misal`, `fungsi`, `jika-lain`, `kembalikan`) bebas celah memori. |
| `agen_qa` | QA & Automation Tester | SOP-05 | Pengujian batas (*edge cases*), stress test streaming SSE, fuzzing fungsional, dan sertifikasi lolos uji. |
| `agen_devops` | SRE & Infrastruktur | SOP-06 | Telemetri kesehatan server, backup snapshot WAL harian, otomatisasi systemd/CI-CD, dan disaster recovery. |
| `agen_keamanan` | Auditor Siber & UU PDP | SOP-07 | Audit OWASP Top 10, sanitasi *log masking*, enkripsi Argon2id/AES-256-GCM, dan kepatuhan UU PDP No. 27/2022. |
| `agen_notulis` | Notulis Obsidian | SOP-08 | Penyusunan risalah Markdown GFM dengan YAML Frontmatter lengkap, pemutakhiran papan Kanban, dan version control. |
| `agen_humas` | Kurir Telegram / WA | SOP-09 | Formatting pesan visual eksekutif, bot gateway dispatching, dan laporan pimpinan ringkas dan formal. |
| `agen_analis_data`| Kepala KPI & Intelijen | SOP-10 | Kalkulasi produktivitas tim, deteksi anomali beban kerja, pemantauan proyek at-risk, dan Balanced Scorecard. |

---

## 🛠️ 4. Konfigurasi Sistem (`config.wya`)

Pengaturan terpusat untuk Qwen diatur pada bagian `ai_llm` dalam berkas [`config.wya`](file:///home/devops/widya-lang/agen_kantor_digital/config.wya):

```widya
"ai_llm": {
    "aktif": benar,
    "penyedia": "ollama",
    "endpoint_url": "http://127.0.0.1:11434/api/generate",
    "model": "qwen2.5-coder:7b",
    "model_cadangan": "widya-coder",
    "timeout_detik": 15,
    "max_tokens": 90,
    "temperature": 0.25,
    "cache_semantik_aktif": benar,
    "profil_persona_aktif": benar
}
```

---

## 🧪 5. Verifikasi & Pengujian

### 1. Menjalankan Suite Uji Integrasi Qwen:
```bash
widya agen_kantor_digital/uji_qwen_kantor_digital.wya
```
*Hasil Verifikasi:*
* `[PASS]` Konfigurasi Qwen 2.5 Coder terdeteksi aktif.
* `[PASS]` Persona Mandor Utama (`agen_orkestrator`) merespons dengan otoritas kepemimpinan sprint.
* `[PASS]` Penjaga Sintaks Widya-Lang (`agen_coder`) menghasilkan penalaran sintaks valid.
* `[PASS]` Akselerasi Semantic Cache terkonfirmasi (0ms latency, cache hit).
* `[PASS]` Auditor Siber (`agen_keamanan`) menerapkan standar UU PDP No. 27/2022.

### 2. Menjalankan Rapat Harian:
```bash
# Rapat Pagi (08:00 WIB)
SESI=pagi widya agen_kantor_digital/jalankan.wya

# Rapat Malam (20:00 WIB)
SESI=malam widya agen_kantor_digital/jalankan.wya
```

Dokumentasi otomatis tersimpan ke Obsidian Vault pada:
* `obsidian_vault/DailyNotes/YYYY-MM-DD-Pagi.md`
* `obsidian_vault/DailyNotes/YYYY-MM-DD-Malam.md`
* `obsidian_vault/Tasks/Kanban_Prioritas.md`
