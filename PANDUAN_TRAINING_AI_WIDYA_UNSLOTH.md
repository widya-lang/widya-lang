# 🧠 Panduan Lengkap Fine-Tuning AI Lokal Widya-Lang Menggunakan Unsloth

Panduan ini menjelaskan cara melatih model AI lokal (*Large Language Model*) agar menguasai sintaks, ekosistem pustaka, dan mesin basis data **Widya-Lang & WidyaBase** secara mendalam menggunakan **Unsloth**.

---

## 🌟 1. Mengapa Menggunakan Unsloth?

[Unsloth](https://github.com/unslothai/unsloth) adalah kerangka kerja fine-tuning LLM sumber terbuka (*open-source*) terpopuler di dunia karena:
1. **Hemat VRAM Hingga 70%-80%**: Memungkinkan fine-tune model 7B/8B (seperti Qwen 2.5 Coder atau Llama 3) hanya dengan GPU 6GB - 16GB VRAM (misal: RTX 3060, RTX 4060, RTX 3090, atau Google Colab T4 gratis).
2. **Kecepatan 2x - 5x Lebih Cepat**: Memanfaatkan kernel Triton/CUDA kustom tanpa penurunan akurasi (*0% loss in accuracy*).
3. **Ekspor Langsung ke GGUF**: Otomatis mengonversi model terlatih ke format GGUF 4-bit (`q4_k_m`) sehingga langsung bisa dijalankan secara offline di komputer lokal menggunakan **Ollama**, **LM Studio**, atau **llama.cpp**.

---

## 🎯 2. Model Dasar Rekomendasi (*Base Model*)

Pilihan model dasar terbaik untuk dilatih dengan data Widya-Lang:

| Model Dasar | Ukuran | VRAM Minimum | Keunggulan Utama |
| :--- | :---: | :---: | :--- |
| **`unsloth/Qwen2.5-Coder-7B-Instruct-bnb-4bit`** *(Sangat Direkomendasikan)* | 7 Miliar | 8 GB | Model coding open-weights terbaik di dunia saat ini, sangat responsif terhadap dialek bahasa baru dan Bahasa Indonesia. |
| **`unsloth/Llama-3.2-3B-Instruct-bnb-4bit`** | 3 Miliar | 5 GB | Sangat ringan, inferensi kilat di laptop/PC tanpa kartu grafis kelas atas. |
| **`unsloth/DeepSeek-R1-Distill-Qwen-7B-bnb-4bit`** | 7 Miliar | 8 GB | Memiliki kemampuan penalaran bertahap (*chain-of-thought*) untuk pemecahan algoritma rumit. |

---

## 🛠️ 3. Tahap 1: Ekstraksi Data Pelatihan (*Dataset Preparation*)

Repositori Widya-Lang telah dilengkapi dengan skrip otomatis [`scripts/generate_dataset.py`](file:///c:/widya-lang/scripts/generate_dataset.py) yang memanen seluruh pengetahuan proyek:
* **176+ berkas contoh nyata** di direktori [`contoh/`](file:///c:/widya-lang/contoh/)
* **60+ modul pustaka standar** di direktori [`modul/`](file:///c:/widya-lang/modul/)
* **Dokumentasi resmi** [`DOKUMENTASI_WIDYABASE.md`](file:///c:/widya-lang/DOKUMENTASI_WIDYABASE.md), [`README.md`](file:///c:/widya-lang/README.md), dan sintaks DDL/DML dwibahasa.

### Menjalankan Generator Dataset:
```bash
# Di terminal Linux, WSL2, atau terminal Windows dengan Python 3:
python scripts/generate_dataset.py
```
*Hasil:* Berkas [`widya_ai_dataset.jsonl`](file:///c:/widya-lang/widya_ai_dataset.jsonl) berformat ChatML / ShareGPT siap santap untuk Unsloth.

---

## 🚀 4. Tahap 2: Menjalankan Training dengan Unsloth

Karena Unsloth membutuhkan kernel CUDA/Triton (Linux/WSL2), Anda memiliki **2 opsi termudah**:

### Opsi A: Google Colab (Gratis GPU T4 16GB — Rekomendasi Tercepat)

1. Buka [Google Colab](https://colab.research.google.com/) dan ubah Runtime ke **T4 GPU** (*Runtime > Change runtime type > T4 GPU*).
2. Buat sel baru dan pasang Unsloth:
   ```bash
   %%capture
   !pip install "unsloth[colab-new] @ git+https://github.com/unslothai/unsloth.git"
   !pip install --no-deps trl peft accelerate bitsandbytes
   ```
3. Unggah berkas `widya_ai_dataset.jsonl` dan skrip [`scripts/train_unsloth.py`](file:///c:/widya-lang/scripts/train_unsloth.py).
4. Jalankan training:
   ```bash
   !python train_unsloth.py
   ```
5. Setelah selesai (sekitar 15-25 menit), unduh berkas model GGUF:
   `widya-coder-gguf/widya-coder-7b-q4_k_m.gguf`

---

### Opsi B: Mesin Lokal Windows Menggunakan WSL2 (Ubuntu + NVIDIA GPU)

Jika PC Windows Anda memiliki GPU NVIDIA (RTX 3060/4060/4070/4080/4090):

1. Buka terminal **WSL2** (Ubuntu):
   ```bash
   wsl
   ```
2. Buat virtual environment Python:
   ```bash
   conda create --name unsloth_env python=3.10 pytorch-cuda=12.1 pytorch cudatoolkit xformers -c pytorch -c nvidia -c xformers -y
   conda activate unsloth_env
   pip install "unsloth[cu121-torch240] @ git+https://github.com/unslothai/unsloth.git"
   pip install --no-deps trl peft accelerate bitsandbytes
   ```
3. Pindah ke direktori proyek Windows dan jalankan training:
   ```bash
   cd /mnt/c/widya-lang
   python3 scripts/generate_dataset.py
   python3 scripts/train_unsloth.py
   ```

---

## 💻 5. Tahap 3: Menjalankan AI Hasil Training di Komputer Lokal (Ollama)

Setelah berkas `widya-coder-7b-q4_k_m.gguf` selesai diekspor, Anda dapat menjalankannya 100% offline di Windows dengan **Ollama**:

1. Pastikan [Ollama untuk Windows](https://ollama.com/download) sudah terpasang.
2. Gunakan template Modelfile yang telah disediakan di [`scripts/Modelfile.widya`](file:///c:/widya-lang/scripts/Modelfile.widya):
   ```dockerfile
   FROM ./widya-coder-gguf/widya-coder-7b-q4_k_m.gguf

   TEMPLATE """<|im_start|>system
   {{ .System }}<|im_end|>
   <|im_start|>user
   {{ .Prompt }}<|im_end|>
   <|im_start|>assistant
   """

   PARAMETER temperature 0.2
   PARAMETER top_p 0.95

   SYSTEM """Anda adalah WidyaCoder, asisten AI lokal ahli pemrograman bahasa Widya-Lang dan mesin basis data WidyaBase."""
   ```
3. Daftarkan model ke Ollama via PowerShell:
   ```powershell
   ollama create widya-coder -f ./scripts/Modelfile.widya
   ```
4. Jalankan AI lokal Anda!
   ```powershell
   ollama run widya-coder
   ```

---

## 💬 6. Contoh Pertanyaan untuk Menguji AI WidyaCoder Anda

Setelah dijalankan di Ollama, Anda bisa langsung menguji kemampuannya:

### 1. Meminta Kode Program Widya-Lang
```text
>>> Buatkan program Widya-Lang untuk aplikasi kasir minimarket yang menghitung diskon dan mencatat ke basis data WidyaBase.
```
*Model AI akan menghasilkan kode sintaks asli Widya-Lang lengkap dengan `fungsi utama()`, `misal`, dan kueri `BUAT_TABEL` / `SISIP_KE`.*

### 2. Kueri Analitik SQL Dwibahasa WidyaBase
```text
>>> Bagaimana cara membuat kueri pemeringkatan gaji divisi menggunakan Window Functions dan CTE di WidyaBase?
```
*Model AI akan langsung menyusun kueri berstandar `DENGAN ... SEBAGAI` dan `NOMOR_BARIS() LEWAT (PARTISI_BERDASARKAN ...)`.*

### 3. Konversi dari Bahasa Pemrograman Lain
```text
>>> Ubah fungsi Python berikut menjadi Widya-Lang:
def hitung_pajak(gaji, pph=0.05):
    return gaji * pph
```
*Model AI akan mengonversinya menjadi:*
```widya
fungsi hitung_pajak(gaji, pph) {
    kembalikan gaji * pph;
}
```

---

## 📂 Berkas yang Tersedia di Repositori

* Skrip Pembuat Dataset: [`scripts/generate_dataset.py`](file:///c:/widya-lang/scripts/generate_dataset.py)
* Skrip Fine-Tuning Unsloth: [`scripts/train_unsloth.py`](file:///c:/widya-lang/scripts/train_unsloth.py)
* Konfigurasi Runtime Ollama: [`scripts/Modelfile.widya`](file:///c:/widya-lang/scripts/Modelfile.widya)
* Dokumentasi Panduan Ini: [`PANDUAN_TRAINING_AI_WIDYA_UNSLOTH.md`](file:///c:/widya-lang/PANDUAN_TRAINING_AI_WIDYA_UNSLOTH.md)
