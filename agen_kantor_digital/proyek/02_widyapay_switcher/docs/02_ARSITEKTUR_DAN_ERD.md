# ARSITEKTUR SWITCHING & ENTITY RELATIONSHIP DIAGRAM (ERD)
## PROYEK: WIDYAPAY ULTRA-FAST PAYMENT SWITCHER
**Disusun Oleh:** `agen_arsitek` (SOP-03)  
**Status:** DIVERIFIKASI • ISO 20022 & EMVCo COMPLIANT  

---

### 1. Topologi Switching Engine

```mermaid
graph LR
    subgraph Inbound["Inbound Gateways"]
        QR[QRIS Dynamic Client]
        BF[BI-FAST Partner Bank]
        POS[Card Terminal ISO 8583]
    end

    subgraph Core["WidyaPay Switching Core (Rust Native)"]
        Parser[Zero-Copy TLV/XML Parser]
        FDS[AI Fraud Detection System]
        HSM[HSM PIN Translation Engine]
        Router[Sub-ms Routing Matrix]
    end

    subgraph Outbound["Settlement & Clearing"]
        BI[Bank Indonesia BI-FAST Core]
        ASPI[ASPI QRIS Switcher Network]
        BPD[Core Banking BPD / BPR]
    end

    Inbound --> Parser
    Parser --> FDS
    FDS --> HSM
    HSM --> Router
    Router --> Outbound
```

---

### 2. Skema ERD Relasional Basis Data

```mermaid
erDiagram
    INSTITUSI_BANK ||--o{ REKENING_PARTNER : memiliki
    MERCHANT ||--o{ QRIS_INVOICE : menerbitkan
    QRIS_INVOICE ||--|| TRANSAKSI_SWITCHING : diproses
    TRANSAKSI_SWITCHING ||--o{ LOG_ROUTING_HOP : mencatat
    TRANSAKSI_SWITCHING ||--o{ RISK_ALERT_FDS : dievaluasi

    MERCHANT {
        string nmid PK
        string nama_merchant
        string mcc_code
        string kota
        string postal_code
    }

    TRANSAKSI_SWITCHING {
        string id_transaksi PK
        string jenis_layanan
        string bank_asal FK
        string bank_tujuan FK
        bigint nominal
        decimal latensi_ms
        string status_transaksi
        datetime waktu_settlement
    }

    RISK_ALERT_FDS {
        string id_alert PK
        string id_transaksi FK
        int risk_score
        string aturan_terpicu
        string tindakan_pencegahan
    }
```
