# 🤖 Sistem Multi-Agent Kantor Digital Berdaulat (10 AI Agent)

Sistem otonom 10 AI Agent berbasis **Widya-Lang** yang terintegrasi langsung dengan **Obsidian Vault** (pencatatan pekerjaan Markdown & papan Kanban), **Telegram Bot Gateway** (briefing rapat pagi & evaluasi malam), dan **Mesin Analitika KPI** (dashboard otomatis & deteksi anomali produktivitas).

---

## 🏛️ 10 AI Agent & SOP Masing-Masing

1. **`agen_orkestrator`** — Mandor Utama / PM (*SOP-01: Moderasi Rapat, Penentuan Prioritas & Resolusi Deadlock*)
2. **`agen_analis`** — Analis Bisnis & Produk (*SOP-02: Dekomposisi Kebutuhan & Acceptance Criteria*)
3. **`agen_arsitek`** — Arsitek Sistem & Database (*SOP-03: Skema SQLite/WidyaBase & Kontrak API*)
4. **`agen_coder`** — Lead Developer (*SOP-04: Eksekusi Kode Widya-Lang & Unit Testing*)
5. **`agen_qa`** — QA & Automation Tester (*SOP-05: Verifikasi Fungsional, Edge Cases & Sertifikasi Rilis*)
6. **`agen_devops`** — Insinyur Keandalan Sistem / SRE (*SOP-06: Monitoring Server, Backup Harian & Systemd*)
7. **`agen_keamanan`** — Auditor Siber & Kepatuhan (*SOP-07: Audit OWASP Top 10, Enkripsi Argon2id & UU PDP*)
8. **`agen_notulis`** — Notulis Obsidian (*SOP-08: Pencatatan DailyNotes Markdown & Sinkronisasi Kanban*)
9. **`agen_humas`** — Kurir Telegram (*SOP-09: Formatting Pesan Visual Telegram & Pengiriman via Bot Gateway*)
10. **`agen_analis_data`** — Kepala Intelijen Data & KPI (*SOP-10: Analitika WidyaBase, Deteksi Anomali & Dashboard KPI Otomatis*)

Seluruh berkas SOP lengkap tersimpan di folder [`sop/`](sop/) dan otomatis disalin ke dalam Obsidian Vault Anda pada folder `SOP/`.

---

## ⚙️ Cara Mengatur Pengaturan (`config.wya`)

Buka berkas `config.wya` untuk menyesuaikan:

### 1. Menghubungkan ke Obsidian Vault Anda
Ubah baris `jalur_vault`:
```widya
"obsidian": {
    "jalur_vault": "C:/Satriyo/Vault",
    "folder_daily_notes": "DailyNotes",
    "folder_tasks": "Tasks",
    "folder_sop": "SOP",
    "folder_arsip": "Arsip"
}
```
*Jika dibiarkan `./obsidian_vault`, sistem otomatis menggunakan folder bawaan di dalam proyek ini.*

### 2. Menghubungkan ke WhatsApp Gateway
Sistem mendukung Fonnte, Wablas, atau Gateway Baileys lokal:
```widya
"whatsapp": {
    "aktif": benar,
    "penyedia": "fonnte",
    "endpoint_url": "https://api.fonnte.com/send",
    "api_token": "MASUKKAN_TOKEN_FONNTE_ANDA",
    "nomor_tujuan": "081234567890"
}
```
*Catatan: Selama token masih berupa contoh, pesan otomatis disimpan rapi ke berkas `./whatsapp_terakhir.txt` dan dicetak di konsol tanpa galat.*

---

## 🚀 Cara Menjalankan

Buka PowerShell di folder `c:\widya-lang\agen_kantor_digital\`:

### 1. Menjalankan Simulasi Siklus Penuh (Pagi & Malam):
```powershell
& "..\dist\releases\v1.0.0\widya.exe" jalankan.wya
```

### 2. Menjalankan Hanya Rapat Pagi (08:00 WIB):
```powershell
$env:SESI = "pagi"
& "..\dist\releases\v1.0.0\widya.exe" jalankan.wya
```

### 3. Menjalankan Hanya Rapat Malam (20:00 WIB):
```powershell
$env:SESI = "malam"
& "..\dist\releases\v1.0.0\widya.exe" jalankan.wya
```

---

## ⏰ Otomasi 2x Sehari di Windows Task Scheduler

Untuk menjadwalkan eksekusi otomatis tanpa perlu membuka terminal manual:

1. **Jadwal Rapat Pagi (08:00 WIB)**:
   * Action: Start a program
   * Program/script: `powershell.exe`
   * Arguments: `-NoProfile -Command "$env:SESI='pagi'; cd 'c:\widya-lang\agen_kantor_digital'; & 'c:\widya-lang\dist\releases\v1.0.0\widya.exe' jalankan.wya"`
   * Trigger: Daily, 08:00 AM

2. **Jadwal Rapat Malam (20:00 WIB)**:
   * Action: Start a program
   * Program/script: `powershell.exe`
   * Arguments: `-NoProfile -Command "$env:SESI='malam'; cd 'c:\widya-lang\agen_kantor_digital'; & 'c:\widya-lang\dist\releases\v1.0.0\widya.exe' jalankan.wya"`
   * Trigger: Daily, 08:00 PM

---

## 📊 Hasil Sinkronisasi ke Obsidian

* **DailyNotes**: Format Markdown dengan YAML Frontmatter berisi 3 Prioritas Utama, Hard Deadline, dan Evaluasi Capaian.
* **Kanban Board** (`Tasks/Kanban_Prioritas.md`): Otomatis memindahkan tugas dari status `TODO` menjadi `DONE` setelah diverifikasi oleh Agen QA.
