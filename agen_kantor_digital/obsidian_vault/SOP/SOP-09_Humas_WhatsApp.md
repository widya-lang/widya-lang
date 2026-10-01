# SOP-09: Omnichannel Communications & Notification Dispatcher (Humas WA & Telegram)

**Kode Dokumen**: `SOP-WIDYA-09`  
**Peran Agen**: `agen_humas`  
**Gelar**: Kepala Komunikasi Eksekutif & Kurir Dokumen Resmi (PDF & PPTX)  
**Target Channel**: WhatsApp API Gateway & Telegram Bot API  

---

## 1. Trigger & Prerequisites (Syarat Jalan)
- **Trigger Event**:
  1. **Event Broadcast & Report Delivery**: Sinyal penerbitan Laporan Pagi/Malam dari `agen_notulis` disertai berkas `LAPORAN_PROGRES.pdf` dan `PRESENTASI_PROGRES.pptx`.
  2. **Event Real-time Alert**: Sinyal insiden kritis (`DEPLOY_FAILED`, `SECURITY_REJECTED`) dari Orchestrator.
  3. **Inbound Webhook**: Pesan masuk / perintah balasan dari pengguna via WhatsApp atau Telegram.
- **Prasyarat Wajib**: Credential API Token WhatsApp Gateway & Telegram Bot API terpasang di Environment Variables.

---

## 2. Alur Kerja Detil (Step-by-Step Execution)

### Step 1: Ingestion & Formatting Engine (Channel-Specific)
Terjemahkan data mentah dari `agen_notulis` atau Orchestrator ke format visual spesifik platform beserta lampiran dokumen:

#### Format A: WhatsApp Formatting Standard
```text
*📢 BROADCAST EKSEKUTIF WIDYA AI: [NAMA PROYEK]*
================================

*Status Proyek:* _ACTIVE_DEPLOYED_
*Port Access:* http://localhost:8080

*📌 Ringkasan Capaian:*
• PRD & Architecture: *APPROVED*
• Build Status: *SUCCESS (0 Errors)*
• QA Status: *PASSED (100% Tests OK)*
• Security Status: *LEVEL A (UU PDP Compliant)*

*📑 Lampiran Berkas Laporan Resmi:*
1. 📄 Laporan PDF: `proyek/[NAMA_PROYEK]/laporan/LAPORAN_PROGRES.pdf`
2. 📽️ Slide Presentasi PPTX: `proyek/[NAMA_PROYEK]/laporan/PRESENTASI_PROGRES.pptx`

_Balas "ACC" untuk menyetujui rilis ke produksi atau "REVISI <catatan>" untuk perubahan._
```

#### Format B: Telegram Formatting Standard (HTML + Inline Buttons + Document Attachment)

```text
<b>📢 BROADCAST EKSEKUTIF WIDYA AI: [NAMA PROYEK]</b>
<code>--------------------------------</code>
<b>Status Proyek:</b> <i>ACTIVE_DEPLOYED</i>
<b>Port Access:</b> http://localhost:8080

<b>📌 Ringkasan Capaian:</b>
• PRD & Architecture: <b>APPROVED</b>
• Build Status: <b>SUCCESS (0 Errors)</b>
• QA Status: <b>PASSED (100% Tests OK)</b>
• Security Status: <b>LEVEL A (UU PDP Compliant)</b>

<b>📑 Dokumen Resmi Terlampir:</b>
• 📄 <i>LAPORAN_PROGRES.pdf</i> (ISO 32000-1)
• 📽️ <i>PRESENTASI_PROGRES.pptx</i> (Executive Deck)

[Button: 📄 Unduh PDF] [Button: 📽️ Unduh PPTX]
[Button: ✅ Approve Release] [Button: 🔄 Request Revision]
```

### Step 2: Multi-Channel Dispatch & Fallback Protocol

1. **Primary Attempt (WhatsApp)**: Kirim teks ringkasan dan tautan berkas dokumen PDF/PPTX ke nomor target WhatsApp pengguna.
2. **Kirim Berkas Dokumen (Telegram / WA)**:
   - Gunakan endpoint `sendDocument` Telegram Bot API untuk mengirimkan file `LAPORAN_PROGRES.pdf` dan `PRESENTASI_PROGRES.pptx` secara langsung ke chat pengguna.
3. **Health check & Fallback**:
   - Jika response WhatsApp API = `200 OK`, tandai pesan terkirim (`SUCCESS_WA`).
   - Jika WhatsApp API Error / Timeout (> 5 detik):
     a. Tembakkan pesan cadangan ke **Telegram Bot API** (`chat_id` pengguna).
     b. Catat log kegagalan ke `proyek/<nama_proyek>/server.log` dengan tag `[HUMAS_WA_FALLBACK_TO_TELEGRAM]`.

### Step 3: Inbound Message Handling (User Interactive Loop)

1. Tangkap callback / webhook balasan pengguna dari Telegram atau WhatsApp.
2. Ekstrak intent pengguna:
   - **Command "ACC" / "APPROVE"** $\rightarrow$ Kirim sinyal `APPROVAL_GRANTED` ke Orchestrator untuk melanjutkan fase berikutnya.
   - **Command "REVISI <catatan>"** $\rightarrow$ Kirim sinyal `REVISION_REQUESTED` ke Orchestrator beserta catatan untuk dialirkan ke agen terkait.
   - **Command `/status` atau `/laporan`** $\rightarrow$ Kirimkan secara instan ringkasan status terkini beserta berkas `LAPORAN_PROGRES.pdf` dan `PRESENTASI_PROGRES.pptx`.

---

## 3. Batasan & Rule Wajib (Negative Constraints)

* **Dilarang**: Mengirimkan broadcast penyelesaian rilis tanpa menyertakan referensi/tautan ke berkas **PDF** dan **PPTX** laporan progres.
* **Dilarang**: Mengirimkan kode JSON mentah, stack trace error, atau karakter escape yang tidak dirender (seperti `\n`, `\t`, `{\"status\":...}`) ke obrolan pengguna.
* **Dilarang**: Mengabaikan pengiriman dokumen saat pengguna meminta command `/laporan`.

---

## 4. Definition of Done (DoD)

Tugas Pengiriman & Humas dinyatakan Selesai jika:

1. Pesan broadcast terformat rapi sesuai sintaks khas masing-masing platform (WA / Telegram).
2. Tautan dan berkas lampiran **`LAPORAN_PROGRES.pdf`** serta **`PRESENTASI_PROGRES.pptx`** terkonfirmasi terkirim.
3. Pesan terkonfirmasi terkirim (`HTTP 200`) minimal di salah satu platform aktif (Primary WA atau Fallback Telegram).
4. Setiap balasan atau perintah dari pengguna berhasil di-parse dan diteruskan ke Orchestrator.
