# Dashboard Keuangan SuperAdmin & Pencegahan Sengketa (Anti-Dispute Escrow Engine)

Dokumen ini menjelaskan arsitektur konsol keuangan terpusat bagi **SuperAdmin (Platform Owner)** untuk memantau arus kas (*Gross Merchandise Value / GMV*), biaya pendaftaran (*Entry Fee*), skema escrow penampungan dana event, pencatatan otomatis komisi platform, audit trail rekonsiliasi gateway, serta mekanisme resolusi sengketa (*Dispute & Chargeback Resolution*) antar pihak (Kontingen vs Event Organizer vs Platform).

---

## 1. Urgensi & Titik Rawan Sengketa Keuangan Turnamen

Dalam kejuaraan pencak silat, sengketa finansial umumnya timbul akibat beberapa skenario:
1. **Event Batal / Diundur Mendadak**: Kontingen menuntut pengembalian dana (*Full Refund*), sedangkan EO sudah menggunakan sebagian dana untuk sewa GOR/medali.
2. **Atlet Diskualifikasi Medis / Kelebihan Berat Badan**: Perdebatan apakah biaya pendaftaran atlet yang gagal timbang badan dapat dikembalikan sebagian (*Partial Refund*).
3. **Klaim Gelap / Transfer Siluman**: Kontingen mengklaim sudah transfer manual ke rekening panitia tanpa melalui sistem, atau panitia mengklaim belum menerima dana dari platform.
4. **Ketidaksesuaian Bagi Hasil Komisi**: Perhitungan potongan gateway fee (MDR Xendit), SaaS platform fee, biaya asuransi BPJS-TK, dan komisi afiliator.

---

## 2. Arsitektur Escrow & Alur Pencairan Bertahap (Milestone-Based Payout)

Untuk melindungi seluruh pihak dan mencegah EO membawa lari dana pendaftaran (*fraud / event rug-pull*), SuperAdmin menerapkan sistem **Smart Escrow Release**:

```mermaid
graph TD
    A[Kontingen Bayar Entry Fee via Xendit] --> B[Rekening Penampungan Escrow Master]
    B --> C[Audit Log & Ledger Imutabel]
    
    C -->|Fase 1: Registrasi Dibuka| D[Hold di Escrow: 100%]
    C -->|Fase 2: Drawing / Bagan Terbit| E[Rilis Termin 1: 30% ke Rekening EO (DP Venue & Logistik)]
    C -->|Fase 3: Hari-H Timbang Badan Selesai| F[Rilis Termin 2: 40% ke Rekening EO (Honor Wasit & Medis)]
    C -->|Fase 4: Final Selesai & No Dispute 1x24 Jam| G[Rilis Termin 3: Sisa 30% Bersih ke EO]
    
    C -->|Potongan Otomatis Langsung| H[SaaS Platform Fee -> Kas SuperAdmin]
    C -->|Potongan Otomatis Langsung| I[Premi BPJS Ketenagakerjaan -> Akun SIPP]
    C -->|Potongan Otomatis Langsung| J[Komisi Afiliator -> Saldo Afiliasi]
```

---

## 3. Fitur Utama Dashboard Keuangan SuperAdmin

### A. Rekonsiliasi Realtime & Three-Way Matching
Sistem mencocokkan 3 data secara otomatis setiap 5 menit:
1. **Data Invoice Sistem**: Faktur yang diterbitkan (`INV-2026-XXXX`).
2. **Data Callback Payment Gateway**: Settlement status dari Xendit / Bank Virtual Account.
3. **Status Pendaftaran Atlet**: Atlet yang terverifikasi di bagan pertandingan.
*(Jika ditemukan selisih Rp 1 pun, sistem menandai sebagai `RECONCILIATION_FLAG` untuk diinvestigasi SuperAdmin).*

### B. Dispute Resolution Center (Pusat Mediasi Sengketa)
- Menangani tiket komplain refund dari ofisial kontingen.
- Menyediakan tombol **Freeze Event Funds** (SuperAdmin dapat membekukan pencairan dana ke EO jika event dilaporkan bermasalah atau melanggar regulasi IPSI).
- **1-Click Direct Refund**: SuperAdmin dapat mengeksekusi pengembalian dana instan ke rekening kontingen dari saldo escrow yang ditahan.

### C. Ledger Imutabel & Bukti Potong Finansial
- Menghasilkan **Laporan Rekapitulasi Finansial Bersama** berformat PDF dengan *Digital Signature & QR Verifikasi Hash* yang mengikat secara hukum bagi EO dan SuperAdmin.

---

## 4. Skema Database Rekonsiliasi & Anti-Sengketa

```sql
-- 1. Rekening Escrow Penampungan Event
CREATE TABLE event_escrow_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    total_inflow NUMERIC(14,2) DEFAULT 0.00,        -- Total dana masuk dari peserta
    platform_fee_collected NUMERIC(12,2) DEFAULT 0.00,
    insurance_fee_collected NUMERIC(12,2) DEFAULT 0.00,
    affiliate_fee_collected NUMERIC(12,2) DEFAULT 0.00,
    released_to_eo NUMERIC(14,2) DEFAULT 0.00,       -- Dana yang sudah dicairkan ke EO
    held_in_escrow NUMERIC(14,2) DEFAULT 0.00,       -- Dana yang masih ditahan di escrow
    refunded_to_participants NUMERIC(12,2) DEFAULT 0.00,
    escrow_status VARCHAR(30) DEFAULT 'ACTIVE',     -- 'ACTIVE', 'FROZEN_DISPUTE', 'SETTLED'
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 2. Log Sengketa & Tiket Pengaduan (Dispute Tickets)
CREATE TABLE payment_disputes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_number VARCHAR(50) UNIQUE NOT NULL,      -- 'DISP-2026-0042'
    event_id UUID NOT NULL REFERENCES events(id),
    contingent_id UUID REFERENCES contingents(id),
    invoice_id UUID REFERENCES invoices(id),
    dispute_reason VARCHAR(100) NOT NULL,           -- 'EVENT_CANCELLED', 'OVERWEIGHT_REFUND', 'DOUBLE_PAYMENT'
    disputed_amount NUMERIC(12,2) NOT NULL,
    complainant_evidence_url TEXT,
    eo_response_notes TEXT,
    superadmin_verdict VARCHAR(50) DEFAULT 'UNDER_REVIEW', -- 'UNDER_REVIEW', 'REFUND_APPROVED', 'CLAIM_REJECTED', 'PARTIAL_REFUND'
    refund_transaction_id VARCHAR(100),
    resolved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
