# 🥋 ARENA SILAT SAAS — SOVEREIGN PENCAK SILAT TOURNAMENT PLATFORM
### 100% Berdaulat Penuh di atas Ekosistem Widya-Lang (Zero External Dependencies)

Platform manajemen turnamen pencak silat terpadu kelas dunia berbasis **Regulasi Resmi IPSI / PERSILAT 2026**, mencakup pendaftaran kontingen online, pembagian kategori & drawing adil, manajemen multi-gelanggang ($N$-Gelanggang), digital scoring konsensus juri, live scoreboard TV, video review (VAR), penimbangan bluetooth, bagan pohon $2^n$, penilaian Seni TGR 5-Juri Trimmed Mean, konsol dokter medis 3 menit, hingga penerbitan e-sertifikat anti-pemalsuan dan format SIMT Puspresnas.

---

## 🏛️ 1. Peta 8 Pilar & 9 Peran Pengguna (*Role-by-Role Architecture*)

```
+---------------------------------------------------------------------------------------------------------------+
|                             ARENA SILAT — 100% SOVEREIGN WIDYA-LANG MATRIX                                    |
+-------------------+--------------------+--------------------+--------------------+----------------------------+
|  Jury Touchpad    |   Scoreboard TV    |   KP Master Desk   |     VAR Replay     |  E-Sertifikat & Puspresnas |
|  & Portal Atlet   |  & OBS Stream Bug  |   & Timbangan BLE  |   & Multi-Camera   |  (SHA-256 Anti-Fraud)      |
+-------------------+--------------------+--------------------+--------------------+----------------------------+
|    WidyaMobile    |      WidyaWeb      |    WidyaDesktop    |    WidyaStream     |        WidyaOffice         |
| (Pure Canvas UI)  | (Realtime & SSR)   |  (Hardware & IPC)  | (WebRTC Broadcast) |   (BSrE QR, PDF & SIMT)    |
+-------------------+--------------------+--------------------+--------------------+----------------------------+
| • 60 FPS Canvas   | • Latensi < 10ms   | • Timbangan ASCII  | • Cloud DVR Buffer | • QR Dinamis SHA-256       |
| • Touch Haptic    | • Transparent Bug  | • Struk POS Cetak  | • Slow-Mo Review   | • UU PDP Masking NIK       |
| • Blind Scoring   | • Multi-Gelanggang | • Kiosk Mode Full  | • Multi-Angle Cam  | • Cetak Vektor & SIMT      |
+-------------------+--------------------+--------------------+--------------------+----------------------------+
|                       Basis Data Terdistribusi & Real-time Consensus: WIDYABASE                               |
|         (Slotted 4KB Page, NoSQL DocDB, Vector HNSW AI Atlet, MVCC ACID, CDC Realtime Streamer)               |
+---------------------------------------------------------------------------------------------------------------+
```

---

## 📦 2. Seluruh Modul Sovereign yang Terpasang di `Projects/arenasilat/src/`

1. **Mesin Penilaian & Konsensus Juri ([`src/scoring_engine.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/scoring_engine.rs))**:
   - Aturan IPSI 2026: Pukulan (+1), Tendangan (+2), Jatuhan (+3), Kuncian (+4).
   - Window konsensus dinamis 1.5 detik (minimal 2 dari 3 juri sepakat untuk pengesahan poin).
   - Panel hukuman KP: Teguran 1 (-1), Teguran 2 (-2), Peringatan 1 (-5), Peringatan 2 (-10), Peringatan 3 (Diskualifikasi).
   - Tiebreaker otomatis jika skor seri: Keunggulan teknik Jatuhan $\rightarrow$ Tendangan $\rightarrow$ Pukulan $\rightarrow$ Timbang ulang.

2. **Bagan Eliminasi & Fair Court Scheduler ([`src/bracket_generator.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/bracket_generator.rs))**:
   - Generator pohon eliminasi $2^n$ dengan perhitungan BYE otomatis.
   - *Contingent Separation*: Pemisahan atlet dari perguruan/kontingen yang sama ke Pool Atas dan Pool Bawah.
   - *Fair Court Scheduler*: Penyeimbang beban multi-gelanggang dengan estimasi waktu partai dinamis.

3. **Video Assistant Referee & Hardware Penimbangan ([`src/var_and_hardware.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/var_and_hardware.rs))**:
   - Siklus klaim kartu review VAR (kuota 2 kartu per partai), status tinjauan dewan hakim, dan pengembalian kartu jika protes diterima.
   - Verifikasi rentang berat badan kelas tanding (toleransi 0.00 kg pada penimbangan resmi).
   - Generator URL e-sertifikat anti-fraud dengan hash SHA-256 dan verifikasi keaslian instan.

4. **Pendaftaran & Billing Kontingen ([`src/pendaftaran.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/pendaftaran.rs))**:
   - Manajemen Kontingen, entri atlet (KTP/KIA, Akte, Surat Sehat), tracking status pembayaran & tagihan otomatis.

5. **Kategori, Kelas & Drawing Pool ([`src/kategori_and_drawing.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/kategori_and_drawing.rs))**:
   - Tabel kelas tanding resmi (Usia Dini, Pra-Remaja, Remaja, Dewasa, Master) dan pembagian Pool A / Pool B adil.

6. **Laporan, Klasemen Medali & Neraca Keuangan ([`src/laporan_and_rekap.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/laporan_and_rekap.rs))**:
   - Klasemen Juara Umum Kontingen (Emas, Perak, Perunggu, Poin), Award Pesilat Terbaik, dan Neraca Keuangan LPJ Event.

7. **Dokter Medis & Marshalling Call Room ([`src/medis_and_marshalling.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/medis_and_marshalling.rs))**:
   - *Doctor Stoppage Authority*, countdown 3 menit, skorsing 30 hari cedera gegar otak (K.O), dan rujukan jalur hijau IGD RSUD + BPJS.
   - Uji timbang badan digital BLE dan antrean Call Room H-3 s/d H-1.

8. **Dewan Hakim & Black Box Audit Trail ([`src/dewan_hakim_td.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/dewan_hakim_td.rs))**:
   - Otorisasi resmi Technical Delegate `[ APPROVE BRACKET DRAWING ]`, sidang banding protes (uang jaminan Rp 3.000.000,-), dan rekaman microsecond black box audit trail juri.

9. **Penilaian Seni TGR & Solo Kreatif ([`src/seni_tgr_engine.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/seni_tgr_engine.rs))**:
   - Tunggal, Ganda, Regu, Solo Kreatif: Nilai dasar 9.90, pinalti deviasi waktu (2:55 - 3:05), pinalti senjata lepas (-0.50), dan 5-Juri Trimmed Mean.

10. **Sistem Ranking Elo & Leaderboard Nasional ([`src/ranking_and_elo.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/ranking_and_elo.rs))**:
    - Formula Dynamic Elo Rating ($K \times (S - E)$), bobot turnamen Tier 1 s/d Tier 4, dan klasemen pesilat nasional.

11. **Puspresnas SIMT Exporter & Social Trading Cards ([`src/puspresnas_and_social.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/puspresnas_and_social.rs))**:
    - Ekspor format JSON SIMT Kemendikdasmen dan generator kartu digital atlet 9:16 untuk media sosial.

12. **Tata Kelola Perguruan & Otomasi Notifikasi WhatsApp ([`src/perguruan_and_notifikasi.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/perguruan_and_notifikasi.rs))**:
    - Hirarki perguruan silat (Ranting $\rightarrow$ Cabang $\rightarrow$ DPP), UKT Sabuk, dan pengingat otomatis H-3 partai ke WhatsApp pelatih.

13. **Manajemen Multi-Gelanggang & Personel ([`src/multi_gelanggang_personel.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/multi_gelanggang_personel.rs))**:
    - Mendukung $N$-Gelanggang paralel (Gelanggang A, B, C, D, E, F, dst.) lengkap dengan roster penugasan KP, Wasit Matras, Tim Juri 1..5, Medis, dan Dynamic Court Load Balancing.

14. **Multi-Camera & Bandwidth Optimization ([`src/multi_camera_and_bandwidth.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/multi_camera_and_bandwidth.rs))**:
    - Manajemen 4 kamera per gelanggang, token QR WebRTC smartphone, dan 4 mode bandwidth (Mosaic Quad-View hemat 85%, Local Edge Buffer hemat 100%, Dynamic ABR, Direct Multi-Channel).

15. **Tata Kelola Federasi IPSI & Perizinan Kejuaraan ([`src/federasi_and_perizinan.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/federasi_and_perizinan.rs))**:
    - Hierarki PB IPSI, Pengprov, dan Pengcab, validasi lisensi wasit juri Kelas 1-3, audit sarana gelanggang (skor 100%), dan penerbitan rekomendasi resmi & izin keramaian kepolisian.

16. **Payment Gateway & Automated Split Settlement ([`src/payment_and_split_settlement.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/payment_and_split_settlement.rs))**:
    - Pemisahan dana otomatis seketika (EO Panitia, Kas PB IPSI Rp 10.000/atlet, Platform SaaS Rp 15.000/atlet), deposit jaminan banding Dewan Hakim Rp 3.000.000,- (otomatis refund jika protes diterima), dan QRIS Dinamis ISO 20022.

17. **Computer Vision Pose Estimation & Biomekanik ([`src/ai_biomekanik_and_cv.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/ai_biomekanik_and_cv.rs))**:
    - Deteksi titik benturan zona sasaran sah (Body Protector dada/perut) vs zona pelanggaran leher/kepala, pelacak batas tangkapan 2.0 detik, dan radar biomotorik (kecepatan tumbukan m/s, waktu reaksi ms, daya ledak Watt).

18. **Asuransi BPJS Ketenagakerjaan Atlet ([`src/asuransi_and_klaim_bpjs.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/asuransi_and_klaim_bpjs.rs))**:
    - Perlindungan BPU atlet turnamen (plafon Rp 50.000.000,- / unlimited), penerbitan berkas rujukan jalur hijau IGD RSUD terverifikasi QR Code, dan koordinasi dokter Sp.KO.

19. **Portal Publik Live & Head-to-Head Analytics ([`src/portal_publik_and_h2h.rs`](file:///home/devops/widya-lang/Projects/arenasilat/src/portal_publik_and_h2h.rs))**:
    - Format live ticker multi-gelanggang untuk OBS / publik dan kalkulasi probabilitas kemenangan Head-to-Head berbasis formula logistik Bradley-Terry & Elo.

---

## 🌐 3. Direktori Portal & Antarmuka Terpadu (Port 8088)

- **📺 Gelanggang TV Scoreboard**: `http://127.0.0.1:8088/scoreboard`
- **📷 Multi-Camera & VAR Replay**: `http://127.0.0.1:8088/camera`
- **🏟️ Manajemen Multi-Gelanggang**: `http://127.0.0.1:8088/gelanggang`
- **📝 Pendaftaran & EO**: `http://127.0.0.1:8088/pendaftaran`
- **🎯 Kategori & Drawing**: `http://127.0.0.1:8088/kategori`
- **🌳 Bagan Pohon 2^n**: `http://127.0.0.1:8088/bracket`
- **📱 Touchpad Juri (1, 2, 3)**: `http://127.0.0.1:8088/juri?id=1`
- **⚖️ Panel KP & Meja Gelanggang**: `http://127.0.0.1:8088/kp`
- **🏛️ Dewan Hakim & TD Audit**: `http://127.0.0.1:8088/td`
- **🩺 Medis & Dokter**: `http://127.0.0.1:8088/medis`
- **⚖️ Marshalling & Timbangan BLE**: `http://127.0.0.1:8088/marshalling`
- **🥋 Portal Kontingen & Coach**: `http://127.0.0.1:8088/kontingen`
- **👑 Super Admin Platform**: `http://127.0.0.1:8088/superadmin`
- **📊 Laporan Medali & Neraca**: `http://127.0.0.1:8088/laporan`
- **🏛️ Federasi & Audit IPSI**: `http://127.0.0.1:8088/federasi`
- **💳 Pembayaran & Split Finansial**: `http://127.0.0.1:8088/pembayaran`
- **⚡ AI Biomekanik & Vision**: `http://127.0.0.1:8088/ai-vision`
- **🩺 Asuransi BPJS Atlet**: `http://127.0.0.1:8088/asuransi`
- **🌐 Portal Publik & H2H**: `http://127.0.0.1:8088/publik`

---

## 🧪 4. Hasil Pengujian Integrasi (21 / 21 Tests Passed)

```bash
cargo test --test arenasilat_sovereign_suite_test \
           --test arenasilat_management_suite_test \
           --test arenasilat_extended_parity_test \
           --test multi_gelanggang_personel_test \
           --test multi_camera_bandwidth_test \
           --test arenasilat_federasi_and_financial_test
```
```text
test test_national_ranking_elo_and_tier_multipliers ... ok
test test_puspresnas_simt_and_whatsapp_dispatcher ... ok
test test_seni_tgr_trimmed_mean_and_penalties ... ok
test test_ai_pose_estimation_and_biomechanical_analytics ... ok
test test_federasi_licensing_and_tournament_audit ... ok
test test_insurance_coverage_and_bpjs_claim_issuance ... ok
test test_public_portal_h2h_and_live_ticker ... ok
test test_payment_split_settlement_and_protest_deposit ... ok
test test_medal_standings_and_best_athlete_award ... ok
test test_category_classification_and_drawing ... ok
test test_tournament_registration_and_financial_flow ... ok
test test_arena_silat_anti_fraud_e_certificate ... ok
test test_arena_silat_var_video_assistant_referee ... ok
test test_arena_silat_bracket_generator_and_contingent_separation ... ok
test test_arena_silat_ipsi_2026_consensus_scoring ... ok
test test_arena_silat_weigh_in_scale_validation ... ok
test test_var_replay_clip_extraction_and_mosaic_config ... ok
test test_bandwidth_optimization_strategies ... ok
test test_multi_camera_registration_and_qr_tokens ... ok
test test_dynamic_court_load_balancing ... ok
test test_multi_gelanggang_scaling_and_personnel_roster ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; finished in 0.00s
```

