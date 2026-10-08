-- ==============================================================================
-- skema.sql - Skema Basis Data Manajemen Inventaris UMKM Nusantara
-- Proyek: PRJ-WIDYA-INV-02
-- Engine: WidyaBase v1.0.0 (Slotted-Page 4KB, B+Tree, WAL, ACID)
-- ==============================================================================

BUAT_TABEL JIKA_TIDAK_ADA tabel_barang (
    kode_sku TEKS KUNCI_UTAMA,
    barcode_ean13 TEKS,
    nama_barang TEKS,
    kategori TEKS,
    satuan TEKS,
    stok_sistem INT,
    harga_beli INT,
    harga_jual INT
);

BUAT_TABEL JIKA_TIDAK_ADA lokasi_rak (
    kode_bin TEKS KUNCI_UTAMA,
    zona TEKS,
    lorong INT,
    rak INT,
    tingkat INT,
    kapasitas_unit INT
);

BUAT_TABEL JIKA_TIDAK_ADA rfid_tag_epc (
    epc_hex TEKS KUNCI_UTAMA,
    kode_sku TEKS,
    nomor_seri TEKS,
    status_tag TEKS,
    rssi_dbm REAL,
    tgl_terakhir_scan TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA pergerakan_stok (
    id_mutasi TEKS KUNCI_UTAMA,
    kode_sku TEKS,
    jenis_mutasi TEKS,
    jumlah INT,
    dari_lokasi TEKS,
    ke_lokasi TEKS,
    waktu_mutasi TEKS,
    operator TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA kartu_stok_opname (
    id_opname TEKS KUNCI_UTAMA,
    kode_sku TEKS,
    stok_tercatat INT,
    stok_fisik INT,
    selisih INT,
    status_rekonsiliasi TEKS,
    waktu_audit TEKS
);

-- Seed Data Master Barang
SISIP_KE tabel_barang NILAI ('BRG-KOP-001', '8992753123456', 'Beras Rojo Lele Super 5kg', 'Sembako', 'Sak', 120, 68000, 75000);
SISIP_KE tabel_barang NILAI ('BRG-KOP-002', '8993175402014', 'Minyak Goreng Sawit 2L', 'Sembako', 'Pouch', 85, 29000, 33500);
SISIP_KE tabel_barang NILAI ('BRG-KOP-003', '8999999195028', 'Gula Pasir Kristal Putih 1kg', 'Sembako', 'Bks', 240, 14500, 17000);
SISIP_KE tabel_barang NILAI ('BRG-KOP-004', '8996001301019', 'Kopi Robusta Temanggung 250g', 'Komoditas', 'Bks', 60, 22000, 28000);
SISIP_KE tabel_barang NILAI ('BRG-KOP-005', '8991001402026', 'Pupuk Organik Granul 10kg', 'Pertanian', 'Sak', 45, 45000, 52000);

-- Seed Data Lokasi Bin/Rak
SISIP_KE lokasi_rak NILAI ('BIN-A-01-01-01', 'Zona A', 1, 1, 1, 150);
SISIP_KE lokasi_rak NILAI ('BIN-A-01-01-02', 'Zona A', 1, 1, 2, 150);
SISIP_KE lokasi_rak NILAI ('BIN-B-02-01-01', 'Zona B', 2, 1, 1, 200);
SISIP_KE lokasi_rak NILAI ('BIN-B-02-02-01', 'Zona B', 2, 2, 1, 200);
SISIP_KE lokasi_rak NILAI ('BIN-C-03-01-01', 'Zona C', 3, 1, 1, 100);

-- Seed Data Tag RFID UHF (EPC SGTIN-96)
SISIP_KE rfid_tag_epc NILAI ('3014257BF7194E4000001A85', 'BRG-KOP-001', 'SER-0001', 'AKTIF', -42.5, '2026-10-02 10:15:00');
SISIP_KE rfid_tag_epc NILAI ('3014257BF7194E4000001A86', 'BRG-KOP-001', 'SER-0002', 'AKTIF', -43.1, '2026-10-02 10:15:02');
SISIP_KE rfid_tag_epc NILAI ('3014257BF7194E4000002B10', 'BRG-KOP-002', 'SER-0010', 'AKTIF', -50.2, '2026-10-02 10:16:30');

-- Seed Data Mutasi Stok Awal
SISIP_KE pergerakan_stok NILAI ('MUT-20261002-0001', 'BRG-KOP-001', 'MASUK', 50, 'Supplier PT Tani Makmur', 'BIN-A-01-01-01', '2026-10-02 08:30:00', 'Rizky Firmansyah');
SISIP_KE pergerakan_stok NILAI ('MUT-20261002-0002', 'BRG-KOP-002', 'MASUK', 40, 'Supplier CV Sawit Jaya', 'BIN-B-02-01-01', '2026-10-02 09:15:00', 'Rizky Firmansyah');
SISIP_KE pergerakan_stok NILAI ('MUT-20261002-0003', 'BRG-KOP-001', 'KELUAR', 10, 'BIN-A-01-01-01', 'Warung KDMP Unit 1', '2026-10-02 11:00:00', 'Budi Santoso');

-- Seed Data Audit Stok Opname
SISIP_KE kartu_stok_opname NILAI ('OPN-20261002-001', 'BRG-KOP-001', 120, 120, 0, 'SESUAI', '2026-10-02 14:00:00');
SISIP_KE kartu_stok_opname NILAI ('OPN-20261002-002', 'BRG-KOP-002', 85, 83, -2, 'TERTUNDA_APPROVAL', '2026-10-02 14:15:00');
