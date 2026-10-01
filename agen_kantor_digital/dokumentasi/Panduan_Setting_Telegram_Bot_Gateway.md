# Panduan Pengaturan Telegram Bot Gateway Kantor Digital 9 AI Agent

Dokumen ini menjelaskan langkah-langkah mudah untuk menghubungkan **Kantor Digital 9 AI Agent Berdaulat** ke akun atau grup **Telegram** resmi Anda.

---

## Mengapa Telegram Bot Gateway?
- **100% Gratis & Resmi**: Menggunakan official Telegram Bot API tanpa biaya langganan bulanan.
- **Tanpa Kedaluwarsa Sesi**: Berbeda dengan WhatsApp Web yang sering putus QR / logout, token bot Telegram aktif permanen.
- **Kecepatan Tinggi**: Notifikasi briefing pagi (08:00 WIB), rekap malam (20:00 WIB), dan instruksi darurat langsung terkirim dalam < 1 detik.
- **Dukungan Format Kaya**: Teks tebal, miring, monospace kode, blok kutipan (*blockquote*), dan tautan dokumen.

---

## Langkah 1: Buat Bot & Dapatkan Token API (via @BotFather)
1. Buka aplikasi Telegram di ponsel atau laptop Anda.
2. Pada kolom pencarian, cari **`@BotFather`** (pastikan akun bercentang biru verifikasi resmi dari Telegram).
3. Klik tombol **Start** atau ketik `/start`.
4. Kirim perintah:
   ```text
   /newbot
   ```
5. BotFather akan meminta nama tampilan bot Anda, ketik misalnya:
   ```text
   Kantor AI Widya
   ```
6. BotFather akan meminta username bot (wajib diakhiri kata `bot`, tanpa spasi), ketik misalnya:
   ```text
   WidyaDigitalBot
   ```
   *(Jika username sudah digunakan orang lain, coba variasi lain seperti `WidyaEnterpriseBot` atau `WidyaOfficeBot`)*.
7. BotFather akan memberikan **HTTP API Token**, contohnya:
   ```text
   7123456789:AAFlKj89_2-XxYyZzAabbCcDdEeFfGg
   ```
   *Salin token ini.*

---

## Langkah 2: Dapatkan Chat ID Anda
Bot membutuhkan **Chat ID** (tujuan ke mana laporan harus dikirim).

### Opsi A: Dikirim ke Akun Pribadi Anda (Paling Sering Digunakan)
1. Cari bot **`@userinfobot`** di Telegram.
2. Klik tombol **Start**.
3. Bot akan langsung menampilkan informasi akun Anda:
   ```text
   Id: 123456789
   First: Satriyo
   ...
   ```
   *Catat angka `Id` tersebut (contoh: `123456789`).*
4. **PENTING**: Buka bot yang baru saja Anda buat di Langkah 1 (misal `@WidyaDigitalBot`) lalu klik tombol **Start** agar bot memiliki izin mengirim pesan ke Anda.

### Opsi B: Dikirim ke Grup / Channel Tim Kerja
1. Buat Grup atau Channel Telegram baru.
2. Masukkan bot Anda (`@WidyaDigitalBot`) ke dalam Grup atau Channel tersebut sebagai **Admin**.
3. Dapatkan Chat ID grup/channel (biasanya berupa angka bertanda minus, contoh: `-1001234567890`).

---

## Langkah 3: Masukkan Konfigurasi ke Sistem Widya

Buka berkas konfigurasi di proyek kantor digital:

### 1. Di berkas `telegram_config.json`:
```json
{
  "aktif": true,
  "bot_token": "7123456789:AAFlKj89_2-XxYyZzAabbCcDdEeFfGg",
  "chat_id": "123456789",
  "username_bot": "@WidyaDigitalBot",
  "parse_mode": "HTML"
}
```

### 2. Di berkas `config.wya`:
```wya
        // 2. PENGATURAN TELEGRAM BOT GATEWAY
        "telegram": {
            "aktif": benar,
            "bot_token": "7123456789:AAFlKj89_2-XxYyZzAabbCcDdEeFfGg",
            "chat_id": "123456789",
            "username_bot": "@WidyaDigitalBot",
            "parse_mode": "HTML"
        },
```

---

## Langkah 4: Uji Coba Pengiriman Pesan (Live Ping)

Anda dapat menguji apakah bot berhasil terhubung dengan salah satu cara berikut:

### Cara 1: Menggunakan Perintah Terminal / PowerShell
```powershell
Invoke-RestMethod -Uri "http://127.0.0.1:8085/api/telegram/test" -Method Post
```
atau menggunakan `curl`:
```bash
curl -X POST http://127.0.0.1:8085/api/telegram/test
```

### Cara 2: Dari Antarmuka Web 3D
1. Buka browser: **`http://127.0.0.1:8085`**
2. Klik tombol **💬 Komunikasi Langsung** di bar atas.
3. Pilih agen **✈️ Nadia Safitri (Humas & Telegram Gateway)**.
4. Klik tombol perintah cepat **✈️ Draf Telegram**, lalu tekan **Kirim Instruksi**.
5. Nadia Safitri akan langsung memproses dan mendispatch pesan ke Telegram Bot Gateway Anda!
