# 💳 DOKUMENTASI RESMI WIDYAPAY & PAYMENT GATEWAY ENGINE (v1.7.0)

Modul **WidyaPay** (`src/fintech/`) adalah suite infrastruktur finansial dan gerbang pembayaran (*turnkey payment gateway*) skala industri yang terintegrasi penuh di dalam **Widya-Lang**. Modul ini dirancang khusus untuk mematuhi regulasi **Bank Indonesia (SNAP BI, QRIS EMVCo)** serta standar keamanan perbankan internasional **PCI-DSS**.

---

## 🌟 Pilar Fitur Utama

```
                      ┌────────────────────────────────────────┐
                      │        WIDYAPAY FINTECH ENGINE         │
                      └───────────────────┬────────────────────┘
                                          │
    ┌──────────────────────┬──────────────┴───────┬──────────────────────┐
    │                      │                      │                      │
┌───▼─────────────┐ ┌──────▼────────────┐ ┌───────▼───────────┐ ┌────────▼──────────┐
│  ASPI QRIS      │ │  SNAP BI OPEN API │ │  ISO 8583 ENGINE  │ │  PCI-DSS VAULT    │
│  - EMVCo MPM    │ │  - HMAC-SHA512    │ │  - MTI (0200/0100)│ │  - Luhn (Mod 10)  │
│  - CRC16-CCITT  │ │  - String-to-Sign │ │  - Dual Bitmap    │ │  - PAN Masking    │
│  - Static->Dyn  │ │  - VA & DirectPay │ │  - Packing/Unpack │ │  - AES-256 Vault  │
└───┬─────────────┘ └──────┬────────────┘ └───────┬───────────┘ └────────┬──────────┘
    │                      │                      │                      │
    └──────────────────────┴──────────────┬───────┴──────────────────────┘
                                          │
                    ┌─────────────────────▼────────────────────┐
                    │       TRANSACTION CORE & SETTLEMENT      │
                    │   - Atomic Double-Entry Ledger (ACID)    │
                    │   - MDR & Fee Split Engine               │
                    │   - Webhook Dispatcher (Backoff Retry)   │
                    │   - Fraud Detection System (FDS Engine)  │
                    └──────────────────────────────────────────┘
```

---

## 🛠️ 1. Perintah CLI `widya bayar`

Widya-Lang menyediakan CLI terpadu untuk pengujian, validasi, simulasi biaya, dan administrasi payment gateway:

```bash
# 1. Menghasilkan String QRIS Dinamis standar ASPI/EMVCo
widya bayar qris --merchant "Toko Widya" --kota "Bandung" --nominal 150000 --invoice "INV-2026-001"

# 2. Validasi Integritas Checksum CRC-16 pada QRIS
widya bayar validasi-qris "00020101021226530014ID.CO.QRIS.WWW...6304F051"

# 3. Mengurai informasi terstruktur payload QRIS
widya bayar urai-qris "00020101021226530014ID.CO.QRIS.WWW...6304F051"

# 4. Validasi Algoritma Luhn (Mod 10) & Deteksi Jaringan Kartu (Visa, Mastercard, GPN, JCB, Amex)
widya bayar validasi-kartu "4111111111111111"

# 5. Masking Nomor Kartu Standar PCI-DSS (4111-11XX-XXXX-1111)
widya bayar mask-kartu "4111111111111111"

# 6. Simulasi Pembagian Biaya MDR, PPN 11%, dan Payout Merchant
widya bayar fee-split --gross 250000 --mdr 0.007 --fixed 0 --bank 300

# 7. Memeriksa Status Kesiapan Engine Finansial
widya bayar status
```

---

## 💻 2. Contoh Penggunaan dalam Kode Sumber Widya-Lang

### A. Generator & Konversi QRIS Dinamis
```widya
// Mengubah QRIS Statis kasir menjadi QRIS Dinamis dengan menyuntikkan nominal
biar qris_statis = "00020101021126530014ID.CO.QRIS.WWW01189360000000000000000202010303UMI51360013ID.OR.GPN.WWW0215ID10200000000005204541153033605802ID5910TOKO WIDYA6007BANDUNG6105129506304A1B2"
biar qris_dinamis = qris_buat_dinamis(qris_statis, 75000, "INV-8821")

// Validasi checksum
biar sah = qris_validasi(qris_dinamis)
pastikan_benar(sah)
cetak("QRIS Siap Ditampilkan:", qris_dinamis)
```

### B. Otentikasi Header SNAP Bank Indonesia (Standar Nasional Open API)
```widya
biar metode = "POST"
biar endpoint = "/v1.0/transfer-va/payment"
biar token = "Bearer secret_access_token_123"
biar body = "{\"partnerServiceId\":\"08888\",\"customerNo\":\"00123\",\"virtualAccountNo\":\"0888800123\",\"trxId\":\"TX-101\"}"
biar client_secret = "RAHASIA_MERCHANT_KEY"

// Hasilkan tanda tangan simetris SNAP BI (HMAC-SHA512)
biar hasil_sig = snap_simetris_signature(metode, endpoint, token, body, client_secret)
cetak("X-SIGNATURE :", hasil_sig["signature"])
cetak("X-TIMESTAMP :", hasil_sig["timestamp"])

// Verifikasi integritas notifikasi pembayaran dari Bank
biar valid = snap_verifikasi_signature(metode, endpoint, token, body, hasil_sig["timestamp"], hasil_sig["signature"], client_secret)
pastikan_benar(valid)
```

### C. Keamanan Kartu PCI-DSS (Luhn & Masking)
```widya
biar pan = "4111111111111111"

// Validasi Luhn Mod 10
biar luhn_ok = kartu_validasi_luhn(pan)
biar jaringan = kartu_deteksi_jaringan(pan)
biar masked_pan = kartu_masking(pan)

cetak("Jaringan Kartu :", jaringan)    // "VISA"
cetak("Masked PAN     :", masked_pan)  // "4111-11XX-XXXX-1111"
```

### D. Double-Entry Accounting Ledger & Fee Split
```widya
// Simulasi transaksi QRIS Rp 100.000 dengan MDR 0.7%
biar fee = pembayaran_hitung_fee_split(100000, 0.007, 0, 300)

cetak("Nilai Gross   :", fee["gross_amount"])        // 100000
cetak("Total Fee MDR :", fee["total_fee"])           // 700
cetak("PPN 11%       :", fee["ppn_tax"])             // 77
cetak("Net Merchant  :", fee["merchant_net_payout"]) // 99300
```

### E. Evaluasi Fraud Detection System (FDS)
```widya
biar konteks_transaksi = {
    "nominal": 150000,
    "ip": "182.1.2.3",
    "frekuensi_10m": 2,
    "gagal_1jam": 0,
    "apakah_vpn": false,
    "negara_kartu": "ID",
    "negara_merchant": "ID"
}

biar hasil_fds = pembayaran_evaluasi_fraud(konteks_transaksi)
cetak("Skor Risiko :", hasil_fds["skor_risiko"]) // 0
cetak("Keputusan   :", hasil_fds["keputusan"])   // "TERIMA"
```

---

## 🔒 3. Arsitektur Kepatuhan Keamanan PCI-DSS

| Aspek Keamanan | Implementasi di Widya-Lang |
| :--- | :--- |
| **Data Encryption at Rest** | Modul `CardTokenizer` mengenkripsi CVV & PAN menggunakan **AES-256-GCM** dengan *zero-knowledge tokenization*. |
| **Data in Transit** | Otentikasi asimetris Ed25519 & tanda tangan simetris HMAC-SHA512 pada setiap permintaan SNAP BI. |
| **PIN Encryption** | Standar ISO 9564 PIN Block Format 0 (XOR Plaintext PIN Block dengan 12-digit PAN). |
| **Audit Trail** | Buku besar *Double-Entry* di WidyaBase dengan ACID WAL logging permanen. |

---

## 📦 4. Status Pengujian
Seluruh modul telah diverifikasi dengan **11 unit test terpadu (`tests/fintech_tests.rs`)** dengan tingkat kelulusan **100% (Passed)**.
