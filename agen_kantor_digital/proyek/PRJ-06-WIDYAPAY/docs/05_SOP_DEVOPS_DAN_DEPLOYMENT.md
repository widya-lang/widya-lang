# SOP DEVOPS, DEPLOYMENT, & HIGH-AVAILABILITY CLUSTER
## PROYEK: WIDYAPAY SWITCHER CORE
**Disusun Oleh:** `agen_devops` (SOP-06)  

---

### 1. Port Layanan & Menjalankan Server
- **Port Khusus**: `http://localhost:8002`

```bash
cd /home/devops/widya-lang/proyek/02_widyapay_switcher
../../target/release/widya server.wya
```

### 2. High-Availability (Active-Active 99.999% SLA)
Sistem beroperasi dengan kluster multi-node sinkronisasi memori zero-copy untuk menjamin pergantian node (*failover*) dalam waktu < 100ms tanpa membatalkan transaksi yang sedang berjalan di *in-flight pipeline*.
