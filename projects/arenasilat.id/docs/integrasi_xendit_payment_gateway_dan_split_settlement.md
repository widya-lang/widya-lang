# Integrasi Payment Gateway Xendit & Multi-Tenancy Split Disbursement

Dokumen ini menjelaskan arsitektur pembayaran end-to-end pada platform **ArenaSilat** menggunakan **Xendit Payment Gateway**, mencakup model kepemilikan platform (SuperAdmin / Platform Owner), model pendaftaran event oleh Panitia/EO, otomatisasi verifikasi pendaftaran atlet, hingga pembagian komisi platform (*Platform Fee*) dan pencairan dana langsung ke rekening panitia (*Split Payment / xenPlatform*).

---

## 1. Arsitektur Model Bisnis & Alur Finansial

Dalam ekosistem ArenaSilat, terdapat 3 tingkatan entitas finansial:
1. **Platform Owner (SuperAdmin ArenaSilat)**: Pemilik akun master induk Xendit (*Master Account / Platform Account*). Menetapkan biaya layanan platform per atlet/kontingen (misal: Rp 5.000 - Rp 10.000 / atlet terdaftar) atau biaya langganan SaaS event.
2. **Penyelenggara Event (Event Organizer / Panitia Pelaksana)**: Akun Sub-Merchant (*xenPlatform Sub-Account*) yang menerima pendapatan bersih biaya pendaftaran (*Registration Entry Fee*) setelah dipotong fee gateway & komisi platform.
3. **Pendaftar (Kontingen / Perguruan / Atlet Mandiri)**: Membayar tagihan pendaftaran via Single Checkout Invoice (QRIS Dinamis, Virtual Account BCA/BRI/Mandiri/BNI/BSI, E-Wallet DANA/OVO/ShopeePay, atau Kartu Kredit/Debit).

```
 +--------------------+       Bayar Invoice         +-------------------------------------+
 |  Kontingen/Atlet   | --------------------------> |        Xendit Checkout Page         |
 +--------------------+                             |  (QRIS, VA Multi-Bank, E-Wallet)    |
                                                    +-------------------------------------+
                                                                       |
                                                               Webhook Notification
                                                                       v
                                                    +-------------------------------------+
                                                    |  ArenaSilat Payment Engine API      |
                                                    |  - Validasi Signature Token         |
                                                    |  - Update Status Pendaftaran (PAID) |
                                                    |  - Unlock Barcode ID & Tiket Timbang|
                                                    +-------------------------------------+
                                                                       |
                             +-----------------------------------------+-----------------------------------------+
                             | Split Settlement (xenPlatform Managed Sub-Account)                                |
                             v                                                                                   v
       +---------------------------------------------+                                     +-------------------------------------------+
       |   Sub-Account EO / Panitia Kejuaraan        |                                     |    Platform Master Account (ArenaSilat)   |
       |  - 95% Entry Fee (Pencairan Otomatis ke     |                                     |   - 5% Platform SaaS Fee                  |
       |    Rekening Bank Panitia Pelaksana)         |                                     |   - Xendit Processing MDR Fee             |
       +---------------------------------------------+                                     +-------------------------------------------+
```

---

## 2. Fitur Utama Integrasi Xendit

### A. Dynamic QRIS (Standar Bank Indonesia)
- Menghasilkan QRIS unik untuk setiap nomor faktur pendaftaran (`INV-2026-XXXX`).
- Mendukung pembayaran instan dari seluruh aplikasi m-Banking (BCA Mobile, Livin Mandiri, BRImo, Wondr BNI) dan E-Wallet (GoPay, OVO, ShopeePay, DANA, LinkAja).
- Konfirmasi pembayaran *real-time* (< 2 detik) melalui callback webhook.

### B. Virtual Account (VA) Multi-Bank Aggregator
- Rekening Virtual Account tertutup (*Closed VA*) dengan nominal persis sesuai tagihan kontingen.
- Bank pendukung: BCA, Mandiri, BRI, BNI, BSI (Syariah), Permata, CIMB Niaga, Danamon, dan BJB.
- Masa kedaluwarsa dinamis (default: 24 jam sebelum slot kelas tanding di-*lock*).

### C. xenPlatform (Automated Split Payment & Sub-Accounts)
- SuperAdmin tidak perlu mentransfer dana hasil pendaftaran secara manual ke panitia satu per satu.
- Setiap Event yang dibuat oleh EO memiliki `user_id` Sub-Account Xendit tersendiri.
- Ketika invoice lunas, dana otomatis dipecah:
  ```json
  {
    "external_id": "INV-KEJURNAS-2026-0891",
    "amount": 350000,
    "payer_email": "official@tapaksuci.org",
    "description": "Biaya Pendaftaran 1 Atlet Kelas C Tanding Dewasa",
    "for-user-id": "xnd_sub_account_eo_jateng_01",
    "with-fee-rule": "fee_rule_arenasilat_5_percent"
  }
  ```

---

## 3. Skema Database Konfigurasi Payment Gateway

```sql
-- 1. Konfigurasi Global SuperAdmin (Master Platform)
CREATE TABLE sys_payment_gateway_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider VARCHAR(50) NOT NULL DEFAULT 'XENDIT', -- 'XENDIT' | 'MIDTRANS'
    environment VARCHAR(20) NOT NULL DEFAULT 'PRODUCTION', -- 'SANDBOX' | 'PRODUCTION'
    public_key VARCHAR(255) NOT NULL,
    secret_key VARCHAR(255) NOT NULL,
    webhook_verification_token VARCHAR(255) NOT NULL,
    default_platform_fee_percent NUMERIC(5,2) DEFAULT 3.50, -- 3.5%
    default_platform_fee_fixed INTEGER DEFAULT 5000,        -- Rp 5.000 / transaksi
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 2. Konfigurasi Rekening & Sub-Account Event Panitia (EO)
CREATE TABLE event_payment_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    eo_tenant_id UUID NOT NULL REFERENCES tenants(id),
    xendit_sub_account_id VARCHAR(100), -- ID xenPlatform Sub-Account
    bank_code VARCHAR(30) NOT NULL,     -- 'MANDIRI', 'BCA', 'BRI', 'BNI'
    bank_account_number VARCHAR(50) NOT NULL,
    bank_account_holder_name VARCHAR(150) NOT NULL,
    auto_withdrawal_enabled BOOLEAN DEFAULT TRUE,
    daily_payout_schedule VARCHAR(20) DEFAULT 'DAILY_SETTLEMENT',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 3. Transaksi Invoicing Pembayaran
CREATE TABLE invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_number VARCHAR(100) UNIQUE NOT NULL, -- INV/2026/03/001
    event_id UUID NOT NULL REFERENCES events(id),
    contingent_id UUID REFERENCES contingents(id),
    total_amount NUMERIC(12,2) NOT NULL,
    platform_fee NUMERIC(12,2) NOT NULL,
    gateway_fee NUMERIC(12,2) NOT NULL,
    net_eo_amount NUMERIC(12,2) NOT NULL,
    status VARCHAR(30) DEFAULT 'PENDING', -- 'PENDING', 'PAID', 'EXPIRED', 'REFUNDED'
    xendit_invoice_url TEXT,
    payment_method VARCHAR(50), -- 'QRIS', 'BCA_VA', 'MANDIRI_VA', 'EWALLET'
    paid_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

---

## 4. Keamanan Webhook & Idempotency Handler

Endpoint webhook backend (`/api/v1/payments/xendit/webhook`) diamankan dengan 3 lapis perlindungan:
1. **Verification Token Header**: Memverifikasi header `x-callback-token` harus identik dengan token rahasia di dashboard Xendit.
2. **Idempotency Key**: Menggunakan ID pembayaran Xendit unik agar jika webhook terkirim ganda (*retry network*), status atlet tidak diproses berulang.
3. **Audit Trail**: Seluruh notifikasi callback disimpan dalam tabel `payment_webhook_logs` untuk rekonsiliasi laporan keuangan.
