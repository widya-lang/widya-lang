# 🤖 Dokumentasi Resmi WidyaCoder: Asisten AI Pemrograman Berdaulat 100% Lokal
### Arsitektur In-Database Local RAG (WidyaBase HNSW) + Model Open-Source Lokal (Bebas Cloud / Google Colab)

---

## 🌟 1. Pendahuluan & Filosofi (Opsi 3)

**WidyaCoder** adalah sistem asisten kecerdasan buatan (*AI Pair Programmer*) berdaulat yang dirancang khusus untuk memahami seluruh ekosistem bahasa pemrograman **Widya-Lang**. 

Berbeda dari pendekatan berbasis Cloud API (seperti OpenAI, Claude, atau Gemini) yang membutuhkan biaya langganan, internet stabil, dan mengekspos kode ke server asing, atau fine-tuning berat yang membutuhkan Google Colab GPU berbayar:
**WidyaCoder mengimplementasikan Opsi 3: 100% Local In-Database RAG (Retrieval-Augmented Generation)**.

### Keunggulan Utama Opsi 3:
1. **100% Berdaulat & Mandiri (*Air-Gapped Ready*)**: Berjalan sepenuhnya di mesin komputer lokal Anda tanpa koneksi internet ke cloud.
2. **Optimal untuk CPU & RAM Skala Server/Workstation**: Sangat cepat dieksekusi pada CPU multi-core (seperti Intel Xeon 16-Core dengan RAM 60 GB) menggunakan model GGUF 4-bit / 8-bit quantized.
3. **Nol Halusinasi (*Zero Hallucination*)**: Menggunakan dokumen dan contoh kode asli Widya-Lang dari basis repositori sebagai sumber kebenaran utama (*ground truth*).
4. **Pembaruan Instan Tanpa Retraining**: Setiap kali ada rilis sintaks baru atau modul baru di Widya-Lang, Anda cukup menjalankan perintah indeks ulang (`widyacoder indeks`) yang selesai dalam hitungan **3 detik**!

---

## 🏛️ 2. Arsitektur Teknis

```
┌────────────────────────────────────────────────────────────────────────┐
│                          PENGGUNA / EDITOR IDE                         │
│     (CLI: widyacoder tanya / chat / API HTTP: http://localhost:8088)   │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        WIDYACODER ENGINE CORE                          │
│                                                                        │
│  [1] Query Analysis & Stopword Filtering                               │
│  [2] Dense Text Embedding via WidyaBase LocalEmbeddingEngine (384-dim) │
│  [3] In-Database HNSW Vector Search (Kemiripan Kosinus)                │
│  [4] Lexical BM25 Keyword Search + Title & Source Boosting             │
│  [5] Reciprocal Rank Fusion (RRF) Ranking                              │
│  [6] Precision Prompt Synthesizer (Sintaks + Rujukan Konteks Ringkas) │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                      LOCAL LLM BACKEND (ON-DEVICE)                     │
│  • Ollama (http://localhost:11434): ornith-coder / llama3.2 / qwen2.5  │
│  • llama.cpp / llama-server (http://localhost:8080): GGUF Q4_K_M       │
│  • SovereignInferenceEngine (Pure Rust KV-Cache CPU Inference)         │
└────────────────────────────────────────────────────────────────────────┘
```

### 1. In-Database Local Embedding Engine
Menggunakan algoritma proyeksi teks berkecepatan tinggi murni di Rust (`src/widyabase/hybrid_search.rs`) yang menghasilkan vektor padat (*dense vector*) 384-dimensi bernormalisasi L2 tanpa dependensi PyTorch atau eksternal.

### 2. Algoritma Pencarian Hibrida RRF (Reciprocal Rank Fusion)
Menggabungkan keunggulan pencarian leksikal (kecocokan kata kunci sintaks seperti `fungsi`, `misal`, `widyabase`) dengan pencarian semantik vektor (pemahaman konsep bahasa), dipadukan dengan formula:
$$\text{Skor RRF}(d) = \sum_{m \in \{\text{lex}, \text{vec}\}} \frac{w_m}{k + \text{rank}_m(d)}$$

---

## 🚀 3. Panduan Penggunaan CLI `widyacoder`

### A. Membangun / Memperbarui Indeks Pengetahuan
Pindai seluruh berkas dokumentasi (`.md`) dan seluruh berkas contoh kode (`contoh/*.wya`):
```bash
# Membangun indeks dari ruang kerja saat ini
widyacoder indeks

# Atau dengan menentukan direktori target dan nama berkas cache
widyacoder indeks --dir /home/devops/widya-lang --kb widyacoder_kb.json
```
*Hasil:* Menghasilkan berkas cache `widyacoder_kb.json` (~8 MB) yang memuat 800+ potongan pengetahuan siap query secara instan.

### B. Mengajukan Pertanyaan Teknis (`tanya`)
```bash
# Tanya langsung (otomatis mendeteksi model lokal yang aktif)
widyacoder tanya "bagaimana cara membuat server web di widya-lang?"

# Tanya tentang basis data WidyaBase
widyacoder tanya "bagaimana cara query tabel dengan SQL dwibahasa di widyabase?"

# Menentukan model tertentu (misal: llama3.2 atau ornith-coder)
widyacoder tanya "buatkan contoh enkripsi AES dan Ed25519" --model llama3.2:latest

# Mode verifikasi konteks (hanya cetak rujukan & prompt tanpa memanggil LLM)
widyacoder tanya "cara membuat komponen reaktif di widyaweb" --prompt-only
```

### C. Konsol Percakapan Interaktif (`chat` / REPL)
Masuk ke terminal dialog interaktif:
```bash
widyacoder chat
```
Perintah khusus dalam sesi chat:
- `:sumber` — Menampilkan/menyembunyikan rujukan dokumen yang diambil.
- `:info` / `:status` — Memeriksa statistik basis pengetahuan & model LLM aktif.
- `:keluar` / `:exit` — Mengakhiri sesi percakapan.

### D. Memeriksa Status Kesehatan Sistem (`status`)
```bash
widyacoder status
```
Output menampilkan:
- Status berkas basis pengetahuan (`widyacoder_kb.json`), ukuran berkas, jumlah potongan (*chunks*), dan stempel waktu.
- Status koneksi server LLM lokal (Ollama port 11434 atau llama.cpp port 8080).
- Model yang direkomendasikan.

### E. Menjalankan HTTP REST API Server (`serve`)
Buka port 8088 untuk menghubungkan WidyaCoder dengan ekstensi VS Code, Cursor, antarmuka web, atau skrip otomatisasi:
```bash
widyacoder serve --port 8088
```
Endpoint yang tersedia:
- `POST /api/chat` : `{ "query": "pertanyaan", "top_k": 4 }`
- `POST /api/search` : `{ "query": "kata kunci", "top_k": 5 }`
- `GET /health` & `GET /api/status` : Informasi status dan model.

---

## 🧠 4. Panduan Model Open-Source Rekomendasi (CPU-Optimized)

Untuk mesin berspesifikasi **16 CPU Cores dan RAM 60 GB** (tanpa Nvidia GPU):

| Model | Ukuran Parameter | Format | Kebutuhan RAM | Catatan |
|---|---|---|---|---|
| **`ornith-coder:latest`** | 9.0B | Q4_K_M | ~6 GB | **Sangat Direkomendasikan** (Sudah terpasang di Ollama lokal) |
| **`llama3.2:latest`** | 3.2B | Q4_K_M | ~2.5 GB | **Sangat Cepat di CPU** (Inferensi ringan) |
| **`qwen2.5-coder:7b`** | 7.0B | Q4_K_M | ~5 GB | **Sangat Presisi untuk Kode** (`ollama run qwen2.5-coder:7b`) |
| **`deepseek-coder:6.7b`** | 6.7B | Q4_K_M | ~4.8 GB | Sangat ahli dalam logika pemrograman |

### Cara Menjalankan Model Baru via Ollama di Lokal:
```bash
# Unduh dan jalankan model koding open-source secara lokal
ollama pull qwen2.5-coder:7b
ollama run qwen2.5-coder:7b
```
WidyaCoder akan otomatis mendeteksi model tersebut saat Anda menjalankan `widyacoder tanya` atau `widyacoder chat`!

---

## 🔒 5. Jaminan Mutu & Pengujian Regresi

WidyaCoder dilengkapi dengan rangkaian pengujian integrasi otomatis:
```bash
cargo test --test widyacoder_rag_test
```
Pengujian ini memastikan:
1. Pemindaian seluruh dokumen dan contoh kode berlangsung sukses tanpa error.
2. Vektor embedding dan persistensi berkas JSON bekerja presisi.
3. Pencarian hibrida BM25 + HNSW mengembalikan rujukan yang relevan untuk setiap domain (Database, Web, Kripto, Sintaks Dasar).
