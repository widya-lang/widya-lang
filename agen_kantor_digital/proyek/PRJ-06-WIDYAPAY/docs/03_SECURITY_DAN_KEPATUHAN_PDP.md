# SPESIFIKASI KEAMANAN & SERTIFIKASI PCI-DSS / UU PDP
## PROYEK: WIDYAPAY SWITCHER CORE
**Disusun Oleh:** `agen_keamanan` (SOP-07)  
**Tingkat Sertifikasi:** PCI-DSS v4.0 Level 1 Service Provider & FIPS 140-2 Level 3  

---

### 1. Proteksi Data Pemegang Kartu & Nasabah (PCI-DSS & UU PDP)
1. **Zero Raw PAN/PIN Storage**: Nomor kartu (PAN) selalu dimasking menjadi `4111-XXXX-XXXX-1111` dan PIN nasabah tidak pernah disimpan di disk dalam kondisi tidak terenkripsi.
2. **Hardware Security Module (HSM)**: Translasi blok PIN antar-bank dilakukan di dalam chip HSM yang terisolasi secara fisik.
3. **Audit Enklave Kripto**: Seluruh transmisi API dilindungi dengan enkripsi **TLS 1.3** dan Mutual Authentication (mTLS) dengan sertifikat X.509 Kemenkominfo.
