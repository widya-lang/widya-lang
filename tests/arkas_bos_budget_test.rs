// ============================================================================
// UJI INTEGRASI ARKAS & MANAJEMEN ANGGARAN BOS (tests/arkas_bos_budget_test.rs)
// Pengujian Rantai Audit Kriptografis, Buku Pembantu, Kalkulasi Pajak, dan Plafon RKAS
// ============================================================================

use widya::arkas::{
    BukuKasUmumSekolah, DokumenRkasTahunan, ItemKegiatanRkas, JenisSumberDana,
    KelompokBelanjaAkuntansi, MetodePembayaranBku, StandarNasionalPendidikan,
};

#[test]
fn test_tamper_evident_bku_audit_chain() {
    let mut bku = BukuKasUmumSekolah::new("20309999", 2026, 1, 100_000_000.0, 0.0);

    // 1. Catat transaksi penerimaan BOS Tahap 1
    bku.catat_penerimaan_dana(
        "2026-01-10",
        "TRF-BOS-2026-01",
        "Penyaluran Dana BOSP Reguler Tahap 1",
        JenisSumberDana::BospReguler,
        50_000_000.0,
    );

    // 2. Belanja SIPLah Pengadaan Buku Teks Utama
    bku.catat_belanja_siplah(
        "2026-01-15",
        "SP-SIPLah-20260115-001",
        "BAST-SIPLah-20260120-001",
        "Pembelian Buku Teks Utama Kurikulum Nasional",
        JenisSumberDana::BospReguler,
        "KEG-SNP6-BUKU-01",
        15_000_000.0,
        225_000.0,   // PPh 22
        1_650_000.0, // PPN 11%
    ).expect("Belanja SIPLah gagal");

    // 3. Tarik tunai untuk operasional
    bku.tarik_tunai_bank("2026-01-22", "TARIK-001", 5_000_000.0)
        .expect("Tarik tunai gagal");

    // 4. Verifikasi keutuhan rantai hash audit
    assert!(bku.verifikasi_integritas_rantai(), "Rantai audit harus valid");
    assert_eq!(bku.daftar_transaksi.len(), 3);

    // Verifikasi masing-masing hash tidak kosong dan terkait
    let t1 = &bku.daftar_transaksi[0];
    let t2 = &bku.daftar_transaksi[1];
    let t3 = &bku.daftar_transaksi[2];

    assert_eq!(t2.hash_sebelumnya.as_ref(), t1.hash_transaksi.as_ref());
    assert_eq!(t3.hash_sebelumnya.as_ref(), t2.hash_transaksi.as_ref());

    // 5. Uji deteksi manipulasi (Tampering Test)
    let mut bku_tampered = bku.clone();
    bku_tampered.daftar_transaksi[1].pengeluaran_rp = 10_000_000.0; // Diubah secara ilegal!
    assert!(!bku_tampered.verifikasi_integritas_rantai(), "Deteksi kecurangan manipulasi BKU harus berhasil memicu invalid hash");
}

#[test]
fn test_belanja_non_siplah_dan_tarik_tunai() {
    let mut bku = BukuKasUmumSekolah::new("20309999", 2026, 2, 80_000_000.0, 0.0);

    // Tarik tunai Rp 10 Juta dari bank ke kas bendahara
    bku.tarik_tunai_bank("2026-02-01", "KWT-TARIK-01", 10_000_000.0).unwrap();
    assert_eq!(bku.saldo_bank_saat_ini(), 70_000_000.0);
    assert_eq!(bku.saldo_tunai_saat_ini(), 10_000_000.0);
    assert_eq!(bku.saldo_total_saat_ini(), 80_000_000.0);

    // Belanja Non-SIPLah (Pembayaran daya listrik & internet via tunai)
    let pajak = BukuKasUmumSekolah::hitung_pajak_belanja(1_500_000.0, "Jasa", true);
    bku.catat_belanja_non_siplah(
        "2026-02-05",
        "PLN-202602-01",
        "Pembayaran Tagihan Listrik PLN Bulan Januari",
        JenisSumberDana::BospReguler,
        "KEG-SNP8-LISTRIK-01",
        MetodePembayaranBku::KasTunaiBendahara,
        1_500_000.0,
        Some(pajak),
    ).expect("Belanja non-SIPLah tunai gagal");

    assert_eq!(bku.saldo_tunai_saat_ini(), 8_500_000.0);
    assert_eq!(bku.saldo_bank_saat_ini(), 70_000_000.0);
    assert_eq!(bku.saldo_total_saat_ini(), 78_500_000.0);

    // Setor sisa kas tunai kembali ke bank Rp 3 Juta
    bku.setor_tunai_ke_bank("2026-02-10", "SETOR-01", 3_000_000.0).unwrap();
    assert_eq!(bku.saldo_tunai_saat_ini(), 5_500_000.0);
    assert_eq!(bku.saldo_bank_saat_ini(), 73_000_000.0);
    assert_eq!(bku.saldo_total_saat_ini(), 78_500_000.0);
}

#[test]
fn test_kalkulasi_pajak_otomatis() {
    // 1. Belanja Barang >= 2 Juta ber-NPWP (PPN 11%, PPh 22 1.5%)
    let p1 = BukuKasUmumSekolah::hitung_pajak_belanja(10_000_000.0, "Barang", true);
    assert_eq!(p1.ppn_11_rp, 1_100_000.0);
    assert_eq!(p1.pph_22_rp, 150_000.0);
    assert_eq!(p1.total_pajak(), 1_250_000.0);

    // 2. Belanja Barang >= 2 Juta NON-NPWP (PPN 11%, PPh 22 3.0% tarif 100% lebih tinggi)
    let p2 = BukuKasUmumSekolah::hitung_pajak_belanja(10_000_000.0, "Barang", false);
    assert_eq!(p2.ppn_11_rp, 1_100_000.0);
    assert_eq!(p2.pph_22_rp, 300_000.0);

    // 3. Honor Guru Non-PNS ber-NPWP (PPh 21 5%)
    let p3 = BukuKasUmumSekolah::hitung_pajak_belanja(2_000_000.0, "HonorNonPNS", true);
    assert_eq!(p3.pph_21_rp, 100_000.0);
    assert_eq!(p3.ppn_11_rp, 0.0);

    // 4. Honor PNS Golongan IV (PPh 21 Final 15%)
    let p4 = BukuKasUmumSekolah::hitung_pajak_belanja(1_000_000.0, "HonorPNS_Gol4", true);
    assert_eq!(p4.pph_21_rp, 150_000.0);
}

#[test]
fn test_validasi_pagu_rkas_dan_realisasi() {
    let mut rkas = DokumenRkasTahunan::new("RKAS-2026-TEST", "20309999", "SDN Berdaulat", 2026);

    let item_laptop = ItemKegiatanRkas {
        id_kegiatan: "KEG-LAPTOP-01".to_string(),
        kode_kegiatan_snp: "06.01.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarSaranaDanPrasarana,
        uraian_kegiatan: "Pengadaan Laptop Pembelajaran Digital".to_string(),
        kode_rekening: "5.2.02.01.01.0001".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaModalPeralatanMesin,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 2.0,
        satuan: "Unit".to_string(),
        tarif_harga_satuan_rp: 10_000_000.0,
        total_anggaran_tahunan_rp: 20_000_000.0,
        alokasi_triwulan_1_rp: 20_000_000.0,
        alokasi_triwulan_2_rp: 0.0,
        alokasi_triwulan_3_rp: 0.0,
        alokasi_triwulan_4_rp: 0.0,
        total_realisasi_rp: 0.0,
    };
    rkas.tambah_kegiatan(item_laptop);

    // Realisasi Pertama: Rp 12.000.000 (Berhasil)
    let res1 = rkas.catat_realisasi_kegiatan("KEG-LAPTOP-01", 12_000_000.0);
    assert!(res1.is_ok());
    assert_eq!(rkas.sisa_pagu_kegiatan("KEG-LAPTOP-01"), Some(8_000_000.0));

    // Realisasi Kedua: Rp 9.000.000 (Harus GAGAL karena sisa pagu hanya 8 Juta)
    let res2 = rkas.catat_realisasi_kegiatan("KEG-LAPTOP-01", 9_000_000.0);
    assert!(res2.is_err(), "Harus ditolak jika melebihi sisa pagu");

    // Realisasi Ketiga: Rp 8.000.000 (Pas habis)
    let res3 = rkas.catat_realisasi_kegiatan("KEG-LAPTOP-01", 8_000_000.0);
    assert!(res3.is_ok());
    assert_eq!(rkas.sisa_pagu_kegiatan("KEG-LAPTOP-01"), Some(0.0));
    assert_eq!(rkas.persentase_serapan_anggaran(), 100.0);
}

#[test]
fn test_buku_pembantu_dan_penutupan_kas() {
    let mut bku = BukuKasUmumSekolah::new("20309999", 2026, 3, 50_000_000.0, 0.0);

    // Penarikan tunai Rp 10 Juta
    bku.tarik_tunai_bank("2026-03-02", "TRK-01", 10_000_000.0).unwrap();

    // Belanja tunai ATK Rp 2 Juta
    bku.catat_belanja_non_siplah(
        "2026-03-05",
        "KWT-ATK-01",
        "Pembelian Kertas HVS & ATK Kantor",
        JenisSumberDana::BospReguler,
        "KEG-SNP7-ATK",
        MetodePembayaranBku::KasTunaiBendahara,
        2_000_000.0,
        None,
    ).unwrap();

    // Belanja SIPLah Bank Rp 15 Juta + Pajak Rp 1.65 Juta PPN
    bku.catat_belanja_siplah(
        "2026-03-12",
        "SP-SIP-99",
        "BAST-SIP-99",
        "Pembelian Alat Peraga IPA",
        JenisSumberDana::BospReguler,
        "KEG-SNP6-IPA",
        15_000_000.0,
        0.0,
        1_650_000.0,
    ).unwrap();

    // Setor Pajak
    bku.catat_setor_pajak(
        "2026-03-20",
        "BILL-PPN-99",
        "NTPN-887766554433",
        "Setor PPN 11% Alat Peraga",
        1_650_000.0,
        MetodePembayaranBku::TransferBankGiro,
    ).unwrap();

    // Tutup buku akhir bulan
    bku.tutup_buku("2026-03-31");
    assert!(bku.status_tutup_buku);

    // Uji Buku Pembantu
    let bp_kas = bku.buat_buku_pembantu_kas();
    assert_eq!(bp_kas.saldo_akhir_rp, 8_000_000.0);

    let bp_bank = bku.buat_buku_pembantu_bank();
    assert_eq!(bp_bank.saldo_akhir_rp, 25_000_000.0);

    let bp_pajak = bku.buat_buku_pembantu_pajak();
    assert_eq!(bp_pajak.total_ppn_11_rp, 1_650_000.0);
    assert_eq!(bp_pajak.total_pajak_disetor_rp, 1_650_000.0);
    assert_eq!(bp_pajak.sisa_saldo_titipan_pajak_rp, 0.0);
}
