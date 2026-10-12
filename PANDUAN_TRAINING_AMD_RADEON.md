# 🔴 Panduan Fine-Tuning AI Lokal Widya-Lang untuk AMD Radeon (16GB VRAM)

Jika kartu grafis Anda adalah **AMD Radeon 16GB VRAM** (seperti RX 6800, RX 6800 XT, RX 6900 XT, RX 7800 XT, RX 7900 GRE, RX 7900 XT, atau Radeon Pro W6800/W7800), Anda memiliki kartu grafis yang **sangat bertenaga**.

Kapasitas **16 GB VRAM** sangat leluasa untuk melatih model coding kelas atas berukuran **7 Miliar hingga 8 Miliar Parameter** (seperti `Qwen2.5-Coder-7B` atau `Llama-3-8B`) dengan adapter LoRA berpresisi tinggi (FP16/BF16).

---

## ❓ Mengapa Unsloth Kurang Cocok untuk AMD?

Unsloth secara arsitektural dibangun menggunakan kernel komputasi Triton/CUDA yang dikhususkan (*hardcoded*) untuk arsitektur NVIDIA Tensor Core. Meskipun Unsloth sedang bereksperimen dengan ROCm, dukungan untuk AMD Radeon di lingkungan Windows/WSL masih belum stabil.

---

## 🏆 3 Pengganti Unsloth Terbaik untuk AMD Radeon

| Solusi Alternatif | Platform / Backend | Antarmuka | Keunggulan Utama |
| :--- | :--- | :---: | :--- |
| **1. Hugging Face `peft` + `trl` (SFTTrainer)** *(Rekomendasi Utama)* | Native Windows (DirectML) atau WSL2 (ROCm) | Skrip Python | Standar resmi industri, 100% stabil di AMD, didukung langsung oleh Microsoft & AMD. |
| **2. LLaMA-Factory** | WSL2 (ROCm) atau Windows | WebUI Browser | Memiliki tampilan visual di web browser, bisa atur parameter LoRA tanpa menulis kode. |
| **3. Axolotl** | Linux / Docker ROCm | CLI YAML | Pilihan favorit praktisi AI untuk model berskala besar. |

---

## 🚀 Cara 1: Menggunakan Hugging Face PEFT + TRL (Skrip Siap Pakai)

Kami telah menyiapkan skrip pelatihan khusus untuk AMD Radeon di repositori ini:
* Skrip Pelatihan: [`scripts/train_amd_radeon.py`](file:///home/devops/widya-lang/scripts/train_amd_radeon.py)
* Dataset Siap Pakai: [`widya_ai_dataset.jsonl`](file:///home/devops/widya-lang/widya_ai_dataset.jsonl)

Anda bisa menjalankannya dengan dua opsi:

### Opsi A: 100% Native Windows (Tanpa WSL2 / Linux) Menggunakan Microsoft DirectML

Microsoft menyediakan backend **DirectML** yang memungkinkan PyTorch berkomunikasi langsung dengan GPU AMD Radeon di Windows melalui DirectX 12:

1. Buka PowerShell dan pasang pustaka:
   ```powershell
   pip install torch-directml torchvision
   pip install transformers datasets peft trl accelerate
   ```
2. Jalankan skrip pelatihan:
   ```powershell
   python scripts/train_amd_radeon.py
   ```
   *Skrip akan otomatis mendeteksi perangkat AMD Radeon via DirectML dan mengalokasikan 16GB VRAM untuk melatih model `Qwen2.5-Coder-7B`.*

---

### Opsi B: Menggunakan WSL2 dengan AMD ROCm (Performa Maksimal)

Jika Anda ingin kecepatan komputasi komparabel dengan NVIDIA CUDA:

1. Buka terminal **WSL2** (Ubuntu):
   ```bash
   wsl
   ```
2. Pasang PyTorch versi resmi AMD ROCm:
   ```bash
   pip install --pre torch torchvision torchaudio --index-url https://download.pytorch.org/whl/nightly/rocm6.2
   pip install transformers datasets peft trl accelerate
   ```
3. Pindah ke direktori proyek dan jalankan:
   ```bash
   cd /mnt/c/widya-lang
   python3 scripts/train_amd_radeon.py
   ```

---

## 🎨 Cara 2: Menggunakan LLaMA-Factory (Dengan Tampilan WebUI)

Jika Anda menyukai antarmuka grafis di browser (*No-Code Web UI*):

1. Pasang LLaMA-Factory:
   ```bash
   git clone --depth 1 https://github.com/hiyouga/LLaMA-Factory.git
   cd LLaMA-Factory
   pip install -e ".[torch,metrics]"
   ```
2. Buka antarmuka WebUI:
   ```bash
   llamafactory-cli webui
   ```
3. Di browser (`http://localhost:7860`):
   * **Model name**: Pilih `Qwen2.5-Coder-7B-Instruct`
   * **Stage**: Supervised Fine-Tuning (SFT)
   * **Dataset**: Arahkan ke berkas `widya_ai_dataset.jsonl`
   * **LoRA Rank**: 16
   * Klik tombol **Start** untuk memulai pelatihan!
   * Di tab **Export**, Anda bisa langsung mengekspor model ke format GGUF!

---

## 💾 Menjalankan Model Hasil Training di AMD Radeon (Ollama di Windows)

Setelah pelatihan selesai:
1. Jalankan penggabungan LoRA:
   ```bash
   python scripts/export_to_gguf.py
   ```
2. Konversi ke GGUF (4-bit):
   ```bash
   python llama.cpp/convert_hf_to_gguf.py widya-coder-merged --outfile widya-coder-7b-q4_k_m.gguf --outtype q4_k_m
   ```
3. Daftarkan dan jalankan di **Ollama** Windows:
   ```powershell
   ollama create widya-coder -f .\scripts\Modelfile.widya
   ollama run widya-coder
   ```

> [!TIP]
> **Kabar Baik**: Ollama untuk Windows versi terbaru telah mendukung **AMD Radeon ROCm secara bawaan**. Saat Anda menjalankan model hasil training ini di Windows, Ollama akan secara otomatis memanfaatkan seluruh 16GB VRAM AMD Radeon Anda dengan kecepatan inferensi puluhan token per detik!
