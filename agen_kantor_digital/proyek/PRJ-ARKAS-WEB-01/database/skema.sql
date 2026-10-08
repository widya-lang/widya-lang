-- ==============================================================================
-- skema.sql - Skema Basis Data Mandiri ARKAS Web Terintegrasi
-- Proyek: PRJ-ARKAS-WEB-01
-- Engine: WidyaBase v1.0.0 (Slotted-Page 4KB, B+Tree, WAL, ACID)
-- ==============================================================================

BUAT_TABEL JIKA_TIDAK_ADA tabel_sekolah (
    npsn TEKS KUNCI_UTAMA,
    nama_sekolah TEKS,
    alamat TEKS,
    nama_kepsek TEKS,
    nip_kepsek TEKS,
    nama_bendahara TEKS,
    rekening_bank TEKS,
    nama_bank TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA tabel_sumber_dana (
    id_alokasi TEKS KUNCI_UTAMA,
    npsn TEKS,
    jenis_dana TEKS,
    tahap INT,
    pagu_tahunan INT,
    dana_cair INT,
    sisa_saldo INT
);

BUAT_TABEL JIKA_TIDAK_ADA tabel_rkas_kegiatan (
    id_kegiatan TEKS KUNCI_UTAMA,
    kode_snp TEKS,
    uraian_kegiatan TEKS,
    kode_rekening TEKS,
    kelompok_belanja TEKS,
    pagu_anggaran INT,
    realisasi_belanja INT,
    sisa_anggaran INT
);

BUAT_TABEL JIKA_TIDAK_ADA tabel_transaksi_bku (
    nomor_urut INT KUNCI_UTAMA,
    tanggal TEKS,
    nomor_bukti TEKS,
    uraian TEKS,
    jenis_transaksi TEKS,
    metode_bayar TEKS,
    penerimaan INT,
    pengeluaran INT,
    saldo_kumulatif INT,
    hash_transaksi TEKS,
    hash_sebelumnya TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA tabel_buku_pajak (
    id_pajak TEKS KUNCI_UTAMA,
    nomor_bukti_bku TEKS,
    pph_21 INT,
    pph_22 INT,
    pph_23 INT,
    ppn_11 INT,
    total_pajak INT,
    kode_billing TEKS,
    ntpn_setor TEKS,
    status_setor TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA tabel_pesanan_siplah (
    id_sp TEKS KUNCI_UTAMA,
    nama_mitra_siplah TEKS,
    nama_penyedia TEKS,
    nilai_pesanan INT,
    nomor_bast TEKS,
    tgl_bast TEKS,
    status_pembayaran TEKS
);

-- Seed Data Profil Sekolah
SISIP_KE tabel_sekolah NILAI ('20301234', 'SD Negeri 1 Somagede', 'Jl. Raya Somagede No. 12, Kab. Banyumas', 'Drs. Slamet Riyadi, M.Pd', '196908151994121001', 'Anisa Rahmawati, S.Pd', '140-00-9876543-2', 'Bank BPD Jateng KCP Somagede');

-- Seed Data Alokasi Multi-Sumber Dana
SISIP_KE tabel_sumber_dana NILAI ('DANA-BOSP-2026', '20301234', 'BOSP Reguler (Tahap 1 & 2)', 1, 225000000, 112500000, 75500000);
SISIP_KE tabel_sumber_dana NILAI ('DANA-BOSKIN-2026', '20301234', 'BOS Kinerja Rapor Mutu', 1, 45000000, 45000000, 45000000);
SISIP_KE tabel_sumber_dana NILAI ('DANA-BOSDA-2026', '20301234', 'BOSDA APBD Banyumas', 1, 25000000, 25000000, 20000000);
SISIP_KE tabel_sumber_dana NILAI ('DANA-BANPER-2026', '20301234', 'Banper APBN Smart Classroom', 1, 150000000, 105000000, 5000000);

-- Seed Data Rencana Kegiatan Anggaran Sekolah (RKAS)
SISIP_KE tabel_rkas_kegiatan NILAI ('KEG-01-SNP6', '06.01.01', 'Pengadaan Buku Teks Utama Kurikulum Nasional', '5.2.05.01.01.0001', 'BelanjaModalBuku', 35000000, 35000000, 0);
SISIP_KE tabel_rkas_kegiatan NILAI ('KEG-02-SNP5', '05.02.01', 'Honorarium Guru Tidak Tetap & Tenaga Kependidikan', '5.1.01.02.01.0005', 'BelanjaPegawaiHonor', 48000000, 24000000, 24000000);
SISIP_KE tabel_rkas_kegiatan NILAI ('KEG-03-SNP8', '08.03.01', 'Langganan Daya Listrik PLN & Internet Sekolah', '5.1.02.02.01.0061', 'BelanjaBarangJasa', 18000000, 9000000, 9000000);
SISIP_KE tabel_rkas_kegiatan NILAI ('KEG-04-SNP6', '06.02.03', 'Pengadaan Smart Display Interaktif (Banper APBN)', '5.2.02.05.01.0002', 'BelanjaModalPeralatan', 100000000, 100000000, 0);
SISIP_KE tabel_rkas_kegiatan NILAI ('KEG-05-SNP7', '07.01.02', 'Pelaksanaan Asesmen Nasional Berbasis Komputer (ANBK)', '5.1.02.01.01.0024', 'BelanjaBarangJasa', 12000000, 4000000, 8000000);

-- Seed Data Mutasi Buku Kas Umum (BKU) Berantai Hash Kriptografis
SISIP_KE tabel_transaksi_bku NILAI (1, '2026-01-15', 'TRF-BOSP-T1', 'Penyaluran BOSP Reguler Tahap 1 dari Kasda', 'PenerimaanDanaTransfer', 'TransferBankGiro', 112500000, 0, 112500000, 'a718c2d8e4b5f90123456789abcdef0123456789abcdef0123456789abcdef01', '0000000000000000000000000000000000000000000000000000000000000000');
SISIP_KE tabel_transaksi_bku NILAI (2, '2026-01-20', 'SP-SIP-202601-01', 'Belanja SIPLah Buku Teks Utama PT Intan Pariwara', 'BelanjaOperasionalSIPLah', 'VirtualAccountSIPLah', 0, 35000000, 77500000, 'b829d3e9f5c6a01234567890bcdef1234567890bcdef1234567890bcdef1234', 'a718c2d8e4b5f90123456789abcdef0123456789abcdef0123456789abcdef01');
SISIP_KE tabel_transaksi_bku NILAI (3, '2026-02-01', 'TARIK-TUNAI-01', 'Penarikan Kas Tunai Bank untuk Operasional Sekolah', 'TarikTunaiDariBank', 'TransferBankGiro', 0, 10000000, 77500000, 'c93ae4fa06d7b12345678901cdef2345678901cdef2345678901cdef2345678', 'b829d3e9f5c6a01234567890bcdef1234567890bcdef1234567890bcdef1234');
SISIP_KE tabel_transaksi_bku NILAI (4, '2026-02-05', 'KWT-PLN-0201', 'Pembayaran Rekening Listrik PLN & Internet Tunai', 'BelanjaNonSIPLah', 'KasTunaiBendahara', 0, 1500000, 76000000, 'da4bf50b17e8c23456789012def3456789012def3456789012def3456789012', 'c93ae4fa06d7b12345678901cdef2345678901cdef2345678901cdef2345678');
SISIP_KE tabel_transaksi_bku NILAI (5, '2026-02-25', 'SP-SIP-202602-02', 'Pengadaan 2 Unit Smart Display Kelas (Banper APBN)', 'BelanjaOperasionalSIPLah', 'VirtualAccountSIPLah', 0, 100000000, 76000000, 'eb5cf61c28f9d34567890123ef4567890123ef4567890123ef4567890123ef45', 'da4bf50b17e8c23456789012def3456789012def3456789012def3456789012');

-- Seed Data Buku Pembantu Pajak
SISIP_KE tabel_buku_pajak NILAI ('PJK-202601-01', 'SP-SIP-202601-01', 0, 525000, 0, 3850000, 4375000, 'EBILL-88334411', 'NTPN-998877665544', 'DISETOR');
SISIP_KE tabel_buku_pajak NILAI ('PJK-202602-01', 'SP-SIP-202602-02', 0, 1500000, 0, 11000000, 12500000, 'EBILL-88334422', 'NTPN-776655443322', 'DISETOR');

-- Seed Data Pengadaan SIPLah Permen 15/2026
SISIP_KE tabel_pesanan_siplah NILAI ('SP-SIP-202601-01', 'SIPLah Telkom', 'PT Intan Pariwara', 35000000, 'BAST-20260120-001', '2026-01-20', 'LUNAS');
SISIP_KE tabel_pesanan_siplah NILAI ('SP-SIP-202602-02', 'SIPLah Blibli', 'PT Teknologi Edukasi Nusantara', 100000000, 'BAST-20260226-0081', '2026-02-26', 'LUNAS');
