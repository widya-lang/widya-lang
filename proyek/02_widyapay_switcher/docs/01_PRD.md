# PRODUCT REQUIREMENT DOCUMENT (PRD)
## PROYEK: WIDYAPAY SWITCHER & NATIONAL PAYMENT GATEWAY (50.000 TPS)
**Disusun Oleh:** `agen_analis` (SOP-02) & `agen_arsitek` (SOP-03)  
**Target Klien:** Bank Pembangunan Daerah (BPD), BPR, FinTech Payment Gateway, E-Wallet  

---

### 1. Problem Statement & Target Kinerja
Biaya lisensi engine switching asing (seperti Base24, FIS, dsb.) mencapai miliaran rupiah per tahun per bank, dengan latensi transaksi rata-rata 100-300ms. 

**Tujuan Produk**: Menyediakan mesin switching nasional native berkecepatan tinggi (**50.000+ Transaksi per Detik / TPS**) dengan latensi sub-milidetik (**< 0.5 ms**), kompatibilitas penuh standar **BI SNAP**, **EMVCo QRIS**, **BI-FAST (ISO 20022)**, dan kartu debit/kredit **ISO 8583**.

---

### 2. Spesifikasi Kebutuhan Fungsional (Functional Requirements)
1. **EMVCo QRIS Generator & Parser**: Pembangkitan QRIS Dinamis (Point of Initiation 12) dan Statis (11), parsing data TLV (Tag 00-63), serta perhitungan checksum CRC-16 otomatis.
2. **BI-FAST ISO 20022 Switcher Engine**: Parser dan builder pesan keuangan internasional `pacs.008` (Credit Transfer), `pacs.002` (Status), dan `camt.053` (Statement).
3. **AI Fraud Detection System (FDS)**: Velocity rule limit, deteksi impossible travel geografis, dan penyaringan blacklist rekening judi online OJK/PPATK.
4. **HSM Cryptographic Key Vault**: Manajemen kunci enkripsi LMK, ZPK, dan proteksi PIN Block berbasis standar PCI-DSS.

---

### 3. Batasan Non-Fungsional (Non-Functional Requirements)
- **Ketersediaan**: SLA 99.999% (maksimal downtime 5 menit per tahun).
- **Kapasitas**: 50.000 TPS dengan konsumsi memori < 100 MB.
- **Kepatuhan**: Regulasi Bank Indonesia (PBI SNAP) dan UU P2SK.
