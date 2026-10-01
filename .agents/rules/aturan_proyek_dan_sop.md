# Aturan Baku Agen Kantor Digital: Standar Dokumen Persiapan, Format Laporan & Proyek Mandiri

## 1. Aturan Proyek Berdiri Sendiri (*Standalone Isolated Projects*)
- Setiap proyek yang dikerjakan oleh Agen Kantor Digital **HARUS BERDIRI SENDIRI** di direktori `/home/devops/widya-lang/proyek/<nama_proyek>/`.
- Dilarang menggabungkan proyek-proyek menjadi satu dashboard monolitik.
- Setiap proyek memiliki antarmuka web tersendiri di `web/`, server runtime Widya-Lang mandiri di `server.wya` pada port unik (8001, 8002, dst.), dan basis data terisolasi di `database/data.wdb`.

## 2. Gerbang Dokumen Persiapan (*Document-First Execution*)
Sebelum `agen_coder` melakukan penulisan kode, tim multi-agent wajib melengkapi 5 dokumen persiapan di `proyek/<nama_proyek>/docs/`:
1. `01_PRD.md` (Product Requirements Document - disiapkan oleh `agen_analis`)
2. `02_ARSITEKTUR_DAN_ERD.md` (Arsitektur & Skema ERD - disiapkan oleh `agen_arsitek`)
3. `03_SECURITY_DAN_KEPATUHAN_PDP.md` (Kepatuhan UU PDP & Enkripsi - disiapkan oleh `agen_keamanan`)
4. `04_RENCANA_PENGUJIAN_QA.md` (Rencana Pengujian & Test Matrix - disiapkan oleh `agen_qa`)
5. `05_SOP_DEVOPS_DAN_DEPLOYMENT.md` (Prosedur Deployment & Monitoring - disiapkan oleh `agen_devops`)

## 3. Standar Wajib Format Laporan Progres (Dual-Format: PDF & PPTX)
Setiap laporan progres proyek (laporan harian/berkala, ringkasan per-fase, status evaluasi, dan laporan penutupan proyek) **WAJIB** diterbitkan dalam dua format dokumen resmi:
1. **Format PDF (`.pdf`)**: Berstandar ISO 32000-1, dokumen formal cetak/arsip yang memuat cover, ringkasan eksekutif, tabel aktivitas 10 agen, matriks status pengujian QA, dan kepatuhan UU PDP/keamanan.
2. **Format Presentasi PPTX (`.pptx`)**: Deck slide presentasi eksekutif berstandar OpenXML / WidyaOffice, menggunakan tema premium (seperti `ExecutiveNavy` atau `DarkModern`), mencakup slide judul, kartu metrik KPI, arsitektur visual, status pengujian, dan ringkasan capaian.
- Seluruh berkas laporan wajib disimpan pada direktori `proyek/<nama_proyek>/laporan/` atau `proyek/<nama_proyek>/docs/laporan/` dan disinkronisasikan ke Obsidian Vault (`ObsidianVault/Reports/` & `ObsidianVault/DailyNotes/`).
- Notifikasi humas via WhatsApp & Telegram (`agen_humas`) wajib menyertakan tautan atau lampiran berkas PDF dan PPTX tersebut.

## 4. Runtime & Basis Data Resmi
- Server web wajib menggunakan biner resmi Widya: `widya web dev web -p <port>`. Dilarang menggunakan Python atau runtime eksternal.
- Penyimpanan data wajib menggunakan WidyaBase (`.wdb` + `.wal`).
- Pemrosesan dan ekspor dokumen wajib menggunakan modul resmi WidyaOffice (`.docx`, `.xlsx`, `.pptx`, `.pdf`).
