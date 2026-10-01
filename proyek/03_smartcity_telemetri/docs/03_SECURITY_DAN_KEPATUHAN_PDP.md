# SPESIFIKASI KEAMANAN & INTEGRITAS IOT SENSOR
## PROYEK: SMARTCITY ISPU TELEMETRI & SPATIAL GIS
**Disusun Oleh:** `agen_keamanan` (SOP-07)  

---

### 1. Keamanan Komunikasi IoT Telemetri
1. **Enkripsi Saluran Sensor**: Data telemetri dikirim menggunakan protokol aman MQTT over TLS (port 8883) dengan otentikasi kunci pre-shared key (PSK).
2. **Anti-Tampering Sensor**: Verifikasi signature digital pada setiap paket telemetri untuk mencegah injeksi data palsu dari luar jaringan.
3. **Privasi Geospasial Warga (UU PDP)**: Data pergerakan warga tidak disimpan dalam bentuk identitas individu, hanya dalam bentuk kepadatan agregat (*anonymized heatmaps*).
