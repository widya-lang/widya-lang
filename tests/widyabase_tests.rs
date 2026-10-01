// ============================================================================
// WIDYABASE TEST SUITE (widyabase_tests.rs)
// ============================================================================
// Pengujian Komprehensif Arsitektur Mesin Basis Data Mandiri:
// 1. Slotted-Page Storage 4KB & Buffer Pool
// 2. Write-Ahead Log (WAL) & LSN Tracking
// 3. B+Tree Indexing & Range Scan
// 4. Kueri SQL Dwibahasa Simetris (Nusantara vs ANSI Global)
// 5. Fungsi Agregasi (HITUNG, JUMLAH, RATA_RATA, MIN, MAKS)
// 6. MVCC Snapshot Isolation & Concurrency
// 7. Multi-Modal (Vektor HNSW Cosine & JSONB)
// 8. Pemeriksaan Integritas & Status Kesehatan
// ============================================================================

use std::fs;
use std::path::PathBuf;
use widya::widyabase::btree::{BTreeIndex, IndexKey, RecordPointer};
use widya::widyabase::multimodal::{JsonbEngine, VectorEngine, VectorRecord};
use widya::widyabase::mvcc::{MvccHeader, TransactionManager};
use widya::widyabase::planner::QueryResult;
use widya::widyabase::sql_parser::{SqlLexer, SqlParser, SqlValue};
use widya::widyabase::storage::{Page, PageType, StorageEngine, PAGE_SIZE};
use widya::widyabase::wal::{WalManager, WalOpType};
use widya::widyabase::WidyaBase;

fn clean_test_file(path: &str) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(format!("{}.wal", path));
    let mut p = PathBuf::from(path);
    p.set_extension("wal");
    let _ = fs::remove_file(p);
}

#[test]
fn test_widyabase_storage_slotted_page() {
    let mut page = Page::new(1, PageType::Data);
    assert_eq!(page.slot_count(), 0);
    assert!(page.free_space() > 4000);

    // Sisip 3 record biner
    let slot0 = page.insert_record(b"Baris Pertama Basis Data").unwrap();
    let slot1 = page.insert_record(b"Baris Kedua Transaksi POS").unwrap();
    let slot2 = page.insert_record(b"Baris Ketiga Log Kurir").unwrap();

    assert_eq!(slot0, 0);
    assert_eq!(slot1, 1);
    assert_eq!(slot2, 2);
    assert_eq!(page.slot_count(), 3);

    // Verifikasi pembacaan record
    assert_eq!(page.get_record(0).unwrap(), b"Baris Pertama Basis Data");
    assert_eq!(page.get_record(1).unwrap(), b"Baris Kedua Transaksi POS");
    assert_eq!(page.get_record(2).unwrap(), b"Baris Ketiga Log Kurir");

    // Hapus record (tombstone)
    assert!(page.delete_record(1));
    assert!(page.get_record(1).is_none());
    assert_eq!(page.get_record(0).unwrap(), b"Baris Pertama Basis Data");
    assert_eq!(page.get_record(2).unwrap(), b"Baris Ketiga Log Kurir");
}

#[test]
fn test_widyabase_wal_and_recovery() {
    let test_db = "target/test_widyabase_wal.wdb";
    clean_test_file(test_db);

    let wal = WalManager::open(test_db).expect("Gagal membuka WAL");
    let lsn1 = wal.append(101, WalOpType::Begin, 0, 0, Vec::new()).unwrap();
    let lsn2 = wal.append(101, WalOpType::Insert, 1, 0, b"DATA_TX_101".to_vec()).unwrap();
    let lsn3 = wal.append(101, WalOpType::Commit, 0, 0, Vec::new()).unwrap();
    let lsn_cp = wal.checkpoint().unwrap();

    assert!(lsn2 > lsn1);
    assert!(lsn3 > lsn2);
    assert!(lsn_cp > lsn3);

    let records = wal.read_all_records().unwrap();
    assert_eq!(records.len(), 4);
    assert_eq!(records[0].op_type, WalOpType::Begin);
    assert_eq!(records[1].op_type, WalOpType::Insert);
    assert_eq!(records[2].op_type, WalOpType::Commit);
    assert_eq!(records[3].op_type, WalOpType::Checkpoint);

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_btree_index() {
    let idx = BTreeIndex::new("idx_produk_id");
    idx.insert(IndexKey::Integer(10), RecordPointer { page_id: 1, slot_id: 0 });
    idx.insert(IndexKey::Integer(50), RecordPointer { page_id: 1, slot_id: 1 });
    idx.insert(IndexKey::Integer(30), RecordPointer { page_id: 2, slot_id: 0 });
    idx.insert(IndexKey::Integer(20), RecordPointer { page_id: 2, slot_id: 1 });

    assert_eq!(idx.len(), 4);

    // Exact search
    let found = idx.search(&IndexKey::Integer(30));
    assert_eq!(found, Some(RecordPointer { page_id: 2, slot_id: 0 }));

    // Range scan: 20 s.d 40
    let range = idx.range_scan(Some(&IndexKey::Integer(20)), Some(&IndexKey::Integer(40)));
    assert_eq!(range.len(), 2);
    assert_eq!(range[0].0, IndexKey::Integer(20));
    assert_eq!(range[1].0, IndexKey::Integer(30));

    // Delete
    assert!(idx.delete(&IndexKey::Integer(30)));
    assert_eq!(idx.search(&IndexKey::Integer(30)), None);
}

#[test]
fn test_widyabase_bilingual_sql_ddl_and_dml() {
    let test_db = "target/test_widyabase_bilingual.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka database");

    // 1. DDL dengan Bahasa Indonesia (BUAT_TABEL)
    let res_create = db.execute("BUAT_TABEL produk (id INT KUNCI_UTAMA, nama TEKS, harga ANGKA, stok INT);").unwrap();
    match res_create {
        QueryResult::Message(msg) => assert!(msg.contains("berhasil dibuat")),
        _ => panic!("Expected message"),
    }

    // 2. DML dengan Bahasa Indonesia (SISIP_KE & NILAI)
    db.execute("SISIP_KE produk (id, nama, harga, stok) NILAI (1, 'Kopi Gayo', 35000, 15);").unwrap();
    db.execute("SISIP_KE produk (id, nama, harga, stok) NILAI (2, 'Kopi Toraja', 42000, 8);").unwrap();
    db.execute("SISIP_KE produk (id, nama, harga, stok) NILAI (3, 'Kopi Mandailing', 28000, 20);").unwrap();

    // 3. Kueri dengan Bahasa Indonesia (PILIH ... DARI ... DIMANA ... URUTKAN BERDASARKAN ... TURUN)
    let res_select_id = db.execute("
        PILIH nama, harga, stok 
        DARI produk 
        DIMANA harga > 30000 
        URUTKAN BERDASARKAN harga TURUN;
    ").unwrap();

    match res_select_id {
        QueryResult::Rows { columns, rows, affected } => {
            assert_eq!(columns, vec!["nama", "harga", "stok"]);
            assert_eq!(affected, 2);
            // Kopi Toraja (42000) harus pertama
            assert_eq!(rows[0][0], SqlValue::Text("Kopi Toraja".to_string()));
            assert_eq!(rows[0][1], SqlValue::Integer(42000));
            // Kopi Gayo (35000) kedua
            assert_eq!(rows[1][0], SqlValue::Text("Kopi Gayo".to_string()));
        }
        _ => panic!("Expected rows"),
    }

    // 4. Kueri yang sama menggunakan Standar Global (SELECT ... FROM ... WHERE ... ORDER BY ... DESC)
    let res_select_en = db.execute("
        SELECT nama, harga, stok 
        FROM produk 
        WHERE harga > 30000 
        ORDER BY harga DESC;
    ").unwrap();

    match res_select_en {
        QueryResult::Rows { rows, affected, .. } => {
            assert_eq!(affected, 2);
            assert_eq!(rows[0][0], SqlValue::Text("Kopi Toraja".to_string()));
            assert_eq!(rows[1][0], SqlValue::Text("Kopi Gayo".to_string()));
        }
        _ => panic!("Expected rows"),
    }

    // 5. UPDATE / PERBARUI dengan Bahasa Indonesia
    let res_update = db.execute("PERBARUI produk ATUR stok = 25 DIMANA id = 1;").unwrap();
    match res_update {
        QueryResult::Rows { affected, .. } => assert_eq!(affected, 1),
        _ => panic!("Expected update rows"),
    }

    // 6. DELETE / HAPUS_DARI dengan Bahasa Indonesia
    let res_del = db.execute("HAPUS_DARI produk DIMANA id = 3;").unwrap();
    match res_del {
        QueryResult::Rows { affected, .. } => assert_eq!(affected, 1),
        _ => panic!("Expected delete rows"),
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_aggregates() {
    let test_db = "target/test_widyabase_agg.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka database");
    db.execute("BUAT_TABEL kasir (id INT, total ANGKA);").unwrap();
    db.execute("SISIP_KE kasir (id, total) NILAI (1, 100000);").unwrap();
    db.execute("SISIP_KE kasir (id, total) NILAI (2, 200000);").unwrap();
    db.execute("SISIP_KE kasir (id, total) NILAI (3, 300000);").unwrap();

    // Uji fungsi agregat Indonesia: HITUNG(*), JUMLAH(total), RATA_RATA(total), MIN(total), MAKS(total)
    let res = db.execute("PILIH HITUNG(*), JUMLAH(total), RATA_RATA(total), MIN(total), MAKS(total) DARI kasir;").unwrap();
    match res {
        QueryResult::Rows { rows, .. } => {
            assert_eq!(rows[0][0], SqlValue::Integer(3)); // HITUNG(*)
            assert_eq!(rows[0][1], SqlValue::Float(600000.0)); // JUMLAH(total)
            assert_eq!(rows[0][2], SqlValue::Float(200000.0)); // RATA_RATA(total)
            assert_eq!(rows[0][3], SqlValue::Float(100000.0)); // MIN(total)
            assert_eq!(rows[0][4], SqlValue::Float(300000.0)); // MAKS(total)
        }
        _ => panic!("Expected aggregate row"),
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_mvcc_snapshot_isolation() {
    let tx_mgr = TransactionManager::new();
    let tx1 = tx_mgr.begin_transaction();
    let tx2 = tx_mgr.begin_transaction();

    let snap_tx2 = tx_mgr.create_snapshot();

    // Baris dibuat oleh tx1 tapi belum commit
    let row_header = MvccHeader { xmin: tx1, xmax: 0 };

    // tx2 tidak boleh melihat baris yang dibuat tx1 yang masih aktif
    assert!(!tx_mgr.is_visible(&row_header, &snap_tx2, tx2));

    // tx1 commit
    tx_mgr.commit(tx1);

    // Ambil snapshot baru setelah tx1 commit
    let snap_tx2_baru = tx_mgr.create_snapshot();
    // Sekarang baris terlihat
    assert!(tx_mgr.is_visible(&row_header, &snap_tx2_baru, tx2));
}

#[test]
fn test_widyabase_vector_and_multimodal() {
    // 1. Uji Kemiripan Kosinus Vektor Embedding AI
    let v_kopi_a = vec![0.9f32, 0.1f32, 0.0f32];
    let v_kopi_b = vec![0.85f32, 0.15f32, 0.05f32];
    let v_komputer = vec![0.0f32, 0.1f32, 0.95f32];

    let sim_a_b = VectorEngine::cosine_similarity(&v_kopi_a, &v_kopi_b);
    let sim_a_komp = VectorEngine::cosine_similarity(&v_kopi_a, &v_komputer);

    assert!(sim_a_b > 0.95, "Embedding kopi A dan B harus sangat mirip");
    assert!(sim_a_komp < 0.2, "Embedding kopi dan komputer harus tidak mirip");

    // 2. KNN Search
    let dataset = vec![
        VectorRecord { id: "p1".to_string(), embedding: v_kopi_b, metadata: "Kopi Arabika".to_string() },
        VectorRecord { id: "p2".to_string(), embedding: v_komputer, metadata: "Laptop Gaming".to_string() },
    ];
    let top = VectorEngine::search_knn(&v_kopi_a, &dataset, 1);
    assert_eq!(top.len(), 1);
    assert_eq!(top[0].0.id, "p1");

    // 3. Uji JSONB Field Extraction
    let payload = r#"{"nama": "Warung Kopi Widya", "nominal": 75000, "kota": "Jakarta"}"#;
    assert_eq!(JsonbEngine::extract_field(payload, "nama"), Some("Warung Kopi Widya".to_string()));
    assert_eq!(JsonbEngine::extract_field(payload, "nominal"), Some("75000".to_string()));
    assert_eq!(JsonbEngine::extract_field(payload, "kota"), Some("Jakarta".to_string()));
}

#[test]
fn test_widyabase_health_check() {
    let test_db = "target/test_widyabase_health.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).unwrap();
    db.execute("BUAT_TABEL akun (id INT, saldo ANGKA);").unwrap();
    db.execute("SISIP_KE akun (id, saldo) NILAI (1, 5000000);").unwrap();

    let health = db.check_health();
    assert!(health.is_healthy);
    assert!(health.total_pages >= 1);
    assert_eq!(health.total_tables, 1);
    assert!(health.wal_records >= 2);

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_relational_join() {
    let test_db = "target/test_widyabase_join.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka database untuk uji JOIN");

    // 1. Buat Tabel Induk (pelanggan) dan Tabel Anak (pesanan)
    db.execute("BUAT_TABEL pelanggan (id INT KUNCI_UTAMA, nama TEKS, kota TEKS);").unwrap();
    db.execute("BUAT_TABEL pesanan (id INT KUNCI_UTAMA, pelanggan_id INT, total ANGKA);").unwrap();

    // 2. Isi data pelanggan
    db.execute("SISIP_KE pelanggan (id, nama, kota) NILAI (1, 'Budi Santoso', 'Jakarta');").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama, kota) NILAI (2, 'Siti Rahma', 'Surabaya');").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama, kota) NILAI (3, 'Ahmad Dahlan', 'Yogyakarta');").unwrap();

    // 3. Isi data pesanan
    db.execute("SISIP_KE pesanan (id, pelanggan_id, total) NILAI (101, 1, 150000);").unwrap();
    db.execute("SISIP_KE pesanan (id, pelanggan_id, total) NILAI (102, 2, 75000);").unwrap();
    db.execute("SISIP_KE pesanan (id, pelanggan_id, total) NILAI (103, 1, 200000);").unwrap();
    db.execute("SISIP_KE pesanan (id, pelanggan_id, total) NILAI (104, 3, 30000);").unwrap();

    // 4. Kueri Relasional JOIN dengan Dialek Nusantara
    let res_nusantara = db.execute("
        PILIH pesanan.id, pelanggan.nama, pesanan.total 
        DARI pesanan 
        GABUNG pelanggan PADA pesanan.pelanggan_id = pelanggan.id 
        DIMANA pesanan.total >= 75000 
        URUTKAN BERDASARKAN pesanan.total TURUN;
    ").unwrap();

    match res_nusantara {
        QueryResult::Rows { columns, rows, affected } => {
            assert_eq!(columns, vec!["pesanan.id", "pelanggan.nama", "pesanan.total"]);
            assert_eq!(affected, 3);
            // Baris 1: pesanan 103 (total 200000) oleh Budi Santoso
            assert_eq!(rows[0][0], SqlValue::Integer(103));
            assert_eq!(rows[0][1], SqlValue::Text("Budi Santoso".to_string()));
            assert_eq!(rows[0][2], SqlValue::Integer(200000));

            // Baris 2: pesanan 101 (total 150000) oleh Budi Santoso
            assert_eq!(rows[1][0], SqlValue::Integer(101));
            assert_eq!(rows[1][1], SqlValue::Text("Budi Santoso".to_string()));
            assert_eq!(rows[1][2], SqlValue::Integer(150000));

            // Baris 3: pesanan 102 (total 75000) oleh Siti Rahma
            assert_eq!(rows[2][0], SqlValue::Integer(102));
            assert_eq!(rows[2][1], SqlValue::Text("Siti Rahma".to_string()));
            assert_eq!(rows[2][2], SqlValue::Integer(75000));
        }
        _ => panic!("Expected rows from Nusantara JOIN"),
    }

    // 5. Kueri yang sama menggunakan Standar Global ANSI SQL (SELECT ... FROM ... JOIN ... ON ...)
    let res_ansi = db.execute("
        SELECT pesanan.id, pelanggan.nama, pesanan.total 
        FROM pesanan 
        JOIN pelanggan ON pesanan.pelanggan_id = pelanggan.id 
        WHERE pesanan.total >= 75000 
        ORDER BY pesanan.total DESC;
    ").unwrap();

    match res_ansi {
        QueryResult::Rows { rows, affected, .. } => {
            assert_eq!(affected, 3);
            assert_eq!(rows[0][0], SqlValue::Integer(103));
            assert_eq!(rows[1][0], SqlValue::Integer(101));
            assert_eq!(rows[2][0], SqlValue::Integer(102));
        }
        _ => panic!("Expected rows from ANSI JOIN"),
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_index_scan_optimization() {
    let test_db = "target/test_widyabase_index_scan.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka database untuk uji Index Scan");
    db.execute("BUAT_TABEL anggota (id INT KUNCI_UTAMA, nama TEKS, poin INT);").unwrap();

    for i in 1..=50 {
        db.execute(&format!("SISIP_KE anggota (id, nama, poin) NILAI ({}, 'Anggota_{}', {});", i, i, i * 10)).unwrap();
    }

    // Point lookup pada Primary Key yang memicu jalur Index Scan O(log N)
    let res = db.execute("PILIH nama, poin DARI anggota DIMANA id = 25;").unwrap();
    match res {
        QueryResult::Rows { rows, affected, .. } => {
            assert_eq!(affected, 1);
            assert_eq!(rows[0][0], SqlValue::Text("Anggota_25".to_string()));
            assert_eq!(rows[0][1], SqlValue::Integer(250));
        }
        _ => panic!("Expected row from index scan"),
    }

    // Point lookup pada item yang tidak ada
    let res_none = db.execute("PILIH nama, poin DARI anggota DIMANA id = 999;").unwrap();
    match res_none {
        QueryResult::Rows { affected, .. } => assert_eq!(affected, 0),
        _ => panic!("Expected 0 rows"),
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_tps_benchmark() {
    let test_db = "target/test_widyabase_tps.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka database untuk benchmark TPS");
    db.execute("BUAT_TABEL transaksi_kasir (id INT KUNCI_UTAMA, nilai ANGKA);").unwrap();

    let start = std::time::Instant::now();
    let total_tx = 200;

    for i in 1..=total_tx {
        db.execute(&format!("SISIP_KE transaksi_kasir (id, nilai) NILAI ({}, {});", i, i * 1000)).unwrap();
    }

    let elapsed = start.elapsed();
    let tps = (total_tx as f64) / elapsed.as_secs_f64();
    println!("⏱️ WidyaBase Insertion Benchmark: {} transaksi dalam {:?} (Throughput: {:.2} TPS)", total_tx, elapsed, tps);

    assert!(tps > 100.0, "Throughput TPS harus lebih tinggi dari 100 TPS");

    let count_res = db.execute("PILIH HITUNG(*) DARI transaksi_kasir;").unwrap();
    match count_res {
        QueryResult::Rows { rows, .. } => assert_eq!(rows[0][0], SqlValue::Integer(total_tx as i64)),
        _ => panic!("Expected aggregate row"),
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_like_pattern_matching() {
    let test_db = "target/test_widyabase_like.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL katalog (id INT, nama TEKS, kode TEKS);").unwrap();
    db.execute("SISIP_KE katalog (id, nama, kode) NILAI (1, 'Kopi Arabika Gayo', 'KOP-01');").unwrap();
    db.execute("SISIP_KE katalog (id, nama, kode) NILAI (2, 'Kopi Robusta Lampung', 'KOP-02');").unwrap();
    db.execute("SISIP_KE katalog (id, nama, kode) NILAI (3, 'Teh Hijau Celup', 'TEH-01');").unwrap();
    db.execute("SISIP_KE katalog (id, nama, kode) NILAI (4, 'Susu Segar Murni', 'SSU-01');").unwrap();

    // 1. Prefix pattern matching: 'Kopi%'
    let res1 = db.execute("PILIH nama DARI katalog DIMANA nama SEPERTI 'Kopi%' URUTKAN BERDASARKAN id NAIK;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res1 {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Kopi Arabika Gayo".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Kopi Robusta Lampung".to_string()));
    } else {
        panic!("Expected rows from LIKE prefix");
    }

    // 2. Suffix pattern matching: '%Murni'
    let res2 = db.execute("PILIH nama DARI katalog DIMANA nama SEPERTI '%Murni';").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res2 {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("Susu Segar Murni".to_string()));
    } else {
        panic!("Expected rows from LIKE suffix");
    }

    // 3. Substring pattern matching: '%Hijau%'
    let res3 = db.execute("SELECT nama FROM katalog WHERE nama LIKE '%Hijau%';").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res3 {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("Teh Hijau Celup".to_string()));
    } else {
        panic!("Expected rows from ANSI LIKE substring");
    }

    // 4. Single-character wildcard: 'KOP-_1'
    let res4 = db.execute("PILIH kode DARI katalog DIMANA kode SEPERTI 'KOP-_1';").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res4 {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("KOP-01".to_string()));
    } else {
        panic!("Expected rows from single character wildcard");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_group_by_aggregation() {
    let test_db = "target/test_widyabase_groupby.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL inventaris (id INT, item TEKS, kategori TEKS, harga ANGKA, stok INT);").unwrap();
    db.execute("SISIP_KE inventaris (id, item, kategori, harga, stok) NILAI (1, 'Kopi Hitam', 'Minuman', 15000, 20);").unwrap();
    db.execute("SISIP_KE inventaris (id, item, kategori, harga, stok) NILAI (2, 'Teh Manis', 'Minuman', 10000, 30);").unwrap();
    db.execute("SISIP_KE inventaris (id, item, kategori, harga, stok) NILAI (3, 'Nasi Goreng', 'Makanan', 25000, 10);").unwrap();
    db.execute("SISIP_KE inventaris (id, item, kategori, harga, stok) NILAI (4, 'Mie Rebus', 'Makanan', 20000, 15);").unwrap();

    // Group By dialek Nusantara
    let res = db.execute("
        PILIH kategori, HITUNG(*), JUMLAH(harga), RATA_RATA(harga), MIN(harga), MAKS(harga)
        DARI inventaris
        KELOMPOKKAN_BERDASARKAN kategori
        URUTKAN BERDASARKAN kategori;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res {
        assert_eq!(affected, 2);
        // Baris 1: Makanan
        assert_eq!(rows[0][0], SqlValue::Text("Makanan".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(2));
        assert_eq!(rows[0][2], SqlValue::Float(45000.0));
        assert_eq!(rows[0][3], SqlValue::Float(22500.0));
        assert_eq!(rows[0][4], SqlValue::Float(20000.0));
        assert_eq!(rows[0][5], SqlValue::Float(25000.0));

        // Baris 2: Minuman
        assert_eq!(rows[1][0], SqlValue::Text("Minuman".to_string()));
        assert_eq!(rows[1][1], SqlValue::Integer(2));
        assert_eq!(rows[1][2], SqlValue::Float(25000.0));
        assert_eq!(rows[1][3], SqlValue::Float(12500.0));
        assert_eq!(rows[1][4], SqlValue::Float(10000.0));
        assert_eq!(rows[1][5], SqlValue::Float(15000.0));
    } else {
        panic!("Expected grouped rows");
    }

    // Group By dialek ANSI SQL
    let res_ansi = db.execute("
        SELECT kategori, COUNT(*), SUM(harga)
        FROM inventaris
        GROUP BY kategori
        ORDER BY kategori DESC;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_ansi {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Minuman".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Makanan".to_string()));
    } else {
        panic!("Expected ANSI grouped rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_between_and_in_filtering() {
    let test_db = "target/test_widyabase_between_in.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL transaksi (id INT, kode TEKS, total ANGKA);").unwrap();
    db.execute("SISIP_KE transaksi (id, kode, total) NILAI (1, 'TRX-01', 50000);").unwrap();
    db.execute("SISIP_KE transaksi (id, kode, total) NILAI (2, 'TRX-02', 120000);").unwrap();
    db.execute("SISIP_KE transaksi (id, kode, total) NILAI (3, 'TRX-03', 250000);").unwrap();
    db.execute("SISIP_KE transaksi (id, kode, total) NILAI (4, 'TRX-04', 400000);").unwrap();

    // 1. ANTARA ... DAN ...
    let res_between = db.execute("PILIH id, total DARI transaksi DIMANA total ANTARA 100000 DAN 300000 URUTKAN BERDASARKAN id NAIK;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_between {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Integer(2));
        assert_eq!(rows[1][0], SqlValue::Integer(3));
    } else {
        panic!("Expected rows from ANTARA");
    }

    // 2. ANSI BETWEEN
    let res_between_ansi = db.execute("SELECT id FROM transaksi WHERE total BETWEEN 200000 AND 500000;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_between_ansi {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Integer(3));
        assert_eq!(rows[1][0], SqlValue::Integer(4));
    } else {
        panic!("Expected rows from ANSI BETWEEN");
    }

    // 3. DI_DALAM (...)
    let res_in = db.execute("PILIH id, kode DARI transaksi DIMANA kode DI_DALAM ('TRX-01', 'TRX-04') URUTKAN BERDASARKAN id NAIK;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_in {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Integer(1));
        assert_eq!(rows[1][0], SqlValue::Integer(4));
    } else {
        panic!("Expected rows from DI_DALAM");
    }

    // 4. ANSI IN (...)
    let res_in_ansi = db.execute("SELECT id FROM transaksi WHERE id IN (2, 3) ORDER BY id ASC;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_in_ansi {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Integer(2));
        assert_eq!(rows[1][0], SqlValue::Integer(3));
    } else {
        panic!("Expected rows from ANSI IN");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_computed_columns() {
    let test_db = "target/test_widyabase_computed.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL rincian_produk (id INT, nama TEKS, harga ANGKA, stok INT);").unwrap();
    db.execute("SISIP_KE rincian_produk (id, nama, harga, stok) NILAI (1, 'Mouse Gaming', 150000, 10);").unwrap();
    db.execute("SISIP_KE rincian_produk (id, nama, harga, stok) NILAI (2, 'Keyboard Mekanikal', 450000, 5);").unwrap();
    db.execute("SISIP_KE rincian_produk (id, nama, harga, stok) NILAI (3, 'Headset Wireless', 300000, 8);").unwrap();

    // Komputasi kolom: harga * stok AS total_nilai
    let res = db.execute("
        PILIH nama, harga * stok AS total_nilai 
        DARI rincian_produk 
        URUTKAN BERDASARKAN total_nilai TURUN;
    ").unwrap();

    if let QueryResult::Rows { columns, rows, affected } = res {
        assert_eq!(columns, vec!["nama", "total_nilai"]);
        assert_eq!(affected, 3);
        // Baris 1: Headset Wireless: 300000 * 8 = 2400000
        assert_eq!(rows[0][0], SqlValue::Text("Headset Wireless".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(2400000));
        // Baris 2: Keyboard Mekanikal: 450000 * 5 = 2250000
        assert_eq!(rows[1][0], SqlValue::Text("Keyboard Mekanikal".to_string()));
        assert_eq!(rows[1][1], SqlValue::Integer(2250000));
        // Baris 3: Mouse Gaming: 150000 * 10 = 1500000
        assert_eq!(rows[2][0], SqlValue::Text("Mouse Gaming".to_string()));
        assert_eq!(rows[2][1], SqlValue::Integer(1500000));
    } else {
        panic!("Expected computed column rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_sql_vector_similarity() {
    let test_db = "target/test_widyabase_vector_sql.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL artikel (id INT, judul TEKS, embedding TEKS);").unwrap();
    db.execute("SISIP_KE artikel (id, judul, embedding) NILAI (1, 'Kecerdasan Buatan dan Robotika', '[0.9, 0.1, 0.0]');").unwrap();
    db.execute("SISIP_KE artikel (id, judul, embedding) NILAI (2, 'Resep Kue Donat Empuk', '[0.0, 0.1, 0.9]');").unwrap();
    db.execute("SISIP_KE artikel (id, judul, embedding) NILAI (3, 'Algoritma Jaringan Syaraf Tiruan', '[0.85, 0.15, 0.05]');").unwrap();

    // 1. Kueri Kemiripan Vektor dengan ORDER BY skor DESC (Semantic Search)
    let res = db.execute("
        PILIH id, judul, KEMIRIPAN_VEKTOR(embedding, '[0.9, 0.1, 0.0]') AS skor
        DARI artikel
        URUTKAN BERDASARKAN skor TURUN;
    ").unwrap();

    if let QueryResult::Rows { columns, rows, affected } = res {
        assert_eq!(columns, vec!["id", "judul", "skor"]);
        assert_eq!(affected, 3);

        // Peringkat 1: id 1 (skor ~1.0)
        assert_eq!(rows[0][0], SqlValue::Integer(1));
        if let SqlValue::Float(score) = rows[0][2] {
            assert!(score > 0.99);
        } else {
            panic!("Expected float score");
        }

        // Peringkat 2: id 3 (skor ~0.99)
        assert_eq!(rows[1][0], SqlValue::Integer(3));

        // Peringkat 3: id 2 (skor rendah < 0.2)
        assert_eq!(rows[2][0], SqlValue::Integer(2));
        if let SqlValue::Float(score) = rows[2][2] {
            assert!(score < 0.2);
        } else {
            panic!("Expected float score");
        }
    } else {
        panic!("Expected vector search rows");
    }

    // 2. Kueri Kemiripan Vektor di dalam klausa WHERE
    let res_filter = db.execute("
        PILIH judul
        DARI artikel
        DIMANA KEMIRIPAN_VEKTOR(embedding, '[0.9, 0.1, 0.0]') > 0.8
        URUTKAN BERDASARKAN id NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_filter {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Kecerdasan Buatan dan Robotika".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Algoritma Jaringan Syaraf Tiruan".to_string()));
    } else {
        panic!("Expected filtered vector search rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_1_having_clause() {
    let test_db = "target/test_widyabase_having.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL produk (id INT, kategori TEKS, harga INT);").unwrap();
    db.execute("SISIP_KE produk (id, kategori, harga) NILAI (1, 'Elektronik', 5000000);").unwrap();
    db.execute("SISIP_KE produk (id, kategori, harga) NILAI (2, 'Elektronik', 3000000);").unwrap();
    db.execute("SISIP_KE produk (id, kategori, harga) NILAI (3, 'Buku', 50000);").unwrap();
    db.execute("SISIP_KE produk (id, kategori, harga) NILAI (4, 'Buku', 70000);").unwrap();
    db.execute("SISIP_KE produk (id, kategori, harga) NILAI (5, 'Pakaian', 150000);").unwrap();

    // 1. Nusantara: KELOMPOKKAN BERDASARKAN + MEMILIKI
    let res_nusantara = db.execute("
        PILIH kategori, RATA_RATA(harga) AS rata, HITUNG(id) AS total
        DARI produk
        KELOMPOKKAN BERDASARKAN kategori
        MEMILIKI RATA_RATA(harga) > 1000000
        URUTKAN BERDASARKAN rata TURUN;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_nusantara {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("Elektronik".to_string()));
        assert_eq!(rows[0][1], SqlValue::Float(4000000.0));
        assert_eq!(rows[0][2], SqlValue::Integer(2));
    } else {
        panic!("Expected HAVING rows in Nusantara");
    }

    // 2. ANSI: GROUP BY + HAVING
    let res_ansi = db.execute("
        SELECT kategori, COUNT(id) AS total
        FROM produk
        GROUP BY kategori
        HAVING COUNT(id) >= 2
        ORDER BY total DESC, kategori ASC;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_ansi {
        assert_eq!(affected, 2);
        // Buku dan Elektronik keduanya memiliki count = 2
        let cats: Vec<String> = rows.iter().map(|r| r[0].as_text()).collect();
        assert!(cats.contains(&"Buku".to_string()));
        assert!(cats.contains(&"Elektronik".to_string()));
    } else {
        panic!("Expected HAVING rows in ANSI");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_2_distinct() {
    let test_db = "target/test_widyabase_distinct.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL inventaris (id INT, kategori TEKS, kota TEKS);").unwrap();
    db.execute("SISIP_KE inventaris (id, kategori, kota) NILAI (1, 'Makanan', 'Jakarta');").unwrap();
    db.execute("SISIP_KE inventaris (id, kategori, kota) NILAI (2, 'Makanan', 'Bandung');").unwrap();
    db.execute("SISIP_KE inventaris (id, kategori, kota) NILAI (3, 'Minuman', 'Jakarta');").unwrap();
    db.execute("SISIP_KE inventaris (id, kategori, kota) NILAI (4, 'Makanan', 'Jakarta');").unwrap();

    // 1. Nusantara: BERBEDA
    let res_berbeda = db.execute("
        PILIH BERBEDA kategori DARI inventaris URUTKAN BERDASARKAN kategori NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_berbeda {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Makanan".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Minuman".to_string()));
    } else {
        panic!("Expected DISTINCT rows with BERBEDA");
    }

    // 2. Nusantara alias: UNIK
    let res_unik = db.execute("
        PILIH UNIK kategori, kota DARI inventaris URUTKAN BERDASARKAN kategori NAIK, kota NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_unik {
        assert_eq!(affected, 3); // (Makanan, Bandung), (Makanan, Jakarta), (Minuman, Jakarta)
        assert_eq!(rows[0][0], SqlValue::Text("Makanan".to_string()));
        assert_eq!(rows[0][1], SqlValue::Text("Bandung".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Makanan".to_string()));
        assert_eq!(rows[1][1], SqlValue::Text("Jakarta".to_string()));
    } else {
        panic!("Expected DISTINCT rows with UNIK");
    }

    // 3. ANSI: SELECT DISTINCT
    let res_distinct = db.execute("
        SELECT DISTINCT kategori FROM inventaris ORDER BY kategori DESC;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_distinct {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Minuman".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Makanan".to_string()));
    } else {
        panic!("Expected DISTINCT rows with SELECT DISTINCT");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_3_outer_joins() {
    let test_db = "target/test_widyabase_outer_join.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL pelanggan (id INT, nama TEKS);").unwrap();
    db.execute("BUAT_TABEL pesanan (id INT, pelanggan_id INT, total INT);").unwrap();

    db.execute("SISIP_KE pelanggan (id, nama) NILAI (1, 'Budi');").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama) NILAI (2, 'Siti');").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama) NILAI (3, 'Joko');").unwrap(); // Joko tidak punya pesanan

    db.execute("SISIP_KE pesanan (id, pelanggan_id, total) NILAI (101, 1, 150000);").unwrap();
    db.execute("SISIP_KE pesanan (id, pelanggan_id, total) NILAI (102, 2, 250000);").unwrap();
    db.execute("SISIP_KE pesanan (id, pelanggan_id, total) NILAI (103, 99, 500000);").unwrap(); // pelanggan 99 tidak ada di tabel pelanggan

    // 1. Nusantara: GABUNG_KIRI (LEFT JOIN)
    let res_left_nusantara = db.execute("
        PILIH p.id, p.nama, pes.total
        DARI pelanggan AS p
        GABUNG_KIRI pesanan AS pes PADA p.id = pes.pelanggan_id
        URUTKAN BERDASARKAN p.id NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_left_nusantara {
        assert_eq!(affected, 3);
        // Budi -> 150000
        assert_eq!(rows[0][1], SqlValue::Text("Budi".to_string()));
        assert_eq!(rows[0][2], SqlValue::Integer(150000));
        // Siti -> 250000
        assert_eq!(rows[1][1], SqlValue::Text("Siti".to_string()));
        assert_eq!(rows[1][2], SqlValue::Integer(250000));
        // Joko -> Null
        assert_eq!(rows[2][1], SqlValue::Text("Joko".to_string()));
        assert_eq!(rows[2][2], SqlValue::Null);
    } else {
        panic!("Expected LEFT JOIN rows");
    }

    // 2. ANSI: RIGHT JOIN / GABUNG_KANAN
    let res_right = db.execute("
        SELECT p.nama, pes.id, pes.total
        FROM pelanggan AS p
        RIGHT JOIN pesanan AS pes ON p.id = pes.pelanggan_id
        ORDER BY pes.id ASC;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_right {
        assert_eq!(affected, 3);
        // pesanan 101 -> Budi
        assert_eq!(rows[0][0], SqlValue::Text("Budi".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(101));
        // pesanan 102 -> Siti
        assert_eq!(rows[1][0], SqlValue::Text("Siti".to_string()));
        assert_eq!(rows[1][1], SqlValue::Integer(102));
        // pesanan 103 -> pelanggan Null
        assert_eq!(rows[2][0], SqlValue::Null);
        assert_eq!(rows[2][1], SqlValue::Integer(103));
    } else {
        panic!("Expected RIGHT JOIN rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_4_null_semantics() {
    let test_db = "target/test_widyabase_nulls.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL anggota (id INT, nama TEKS, email TEKS);").unwrap();
    db.execute("SISIP_KE anggota (id, nama, email) NILAI (1, 'Andi', 'andi@widya.id');").unwrap();
    db.execute("SISIP_KE anggota (id, nama, email) NILAI (2, 'Bambang', NIHIL);").unwrap();
    db.execute("SISIP_KE anggota (id, nama, email) NILAI (3, 'Citra', NULL);").unwrap();

    // 1. Nusantara: ADALAH NIHIL
    let res_null = db.execute("
        PILIH nama DARI anggota DIMANA email ADALAH NIHIL URUTKAN BERDASARKAN id NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_null {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Bambang".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Citra".to_string()));
    } else {
        panic!("Expected IS NULL rows");
    }

    // 2. Nusantara: BUKAN NIHIL
    let res_not_null = db.execute("
        PILIH nama DARI anggota DIMANA email BUKAN NIHIL;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_not_null {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("Andi".to_string()));
    } else {
        panic!("Expected IS NOT NULL rows with BUKAN NIHIL");
    }

    // 3. ANSI: IS NOT NULL
    let res_ansi_not_null = db.execute("
        SELECT id, nama FROM anggota WHERE email IS NOT NULL;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_ansi_not_null {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Integer(1));
    } else {
        panic!("Expected IS NOT NULL rows in ANSI");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_5_case_when_and_coalesce() {
    let test_db = "target/test_widyabase_case.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL siswa (id INT, nama TEKS, nilai INT, hp TEKS);").unwrap();
    db.execute("SISIP_KE siswa (id, nama, nilai, hp) NILAI (1, 'Dewi', 95, '081234');").unwrap();
    db.execute("SISIP_KE siswa (id, nama, nilai, hp) NILAI (2, 'Eko', 75, NIHIL);").unwrap();
    db.execute("SISIP_KE siswa (id, nama, nilai, hp) NILAI (3, 'Fajar', 50, NULL);").unwrap();

    // 1. Nusantara: KASUS ... KETIKA ... MAKA ... LAINNYA ... SELESAI
    let res_kasus = db.execute("
        PILIH nama,
              KASUS
                  KETIKA nilai >= 85 MAKA 'Istimewa'
                  KETIKA nilai >= 70 MAKA 'Cukup'
                  LAINNYA 'Kurang'
              SELESAI AS predikat
        DARI siswa
        URUTKAN BERDASARKAN id NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_kasus {
        assert_eq!(affected, 3);
        assert_eq!(rows[0][1], SqlValue::Text("Istimewa".to_string()));
        assert_eq!(rows[1][1], SqlValue::Text("Cukup".to_string()));
        assert_eq!(rows[2][1], SqlValue::Text("Kurang".to_string()));
    } else {
        panic!("Expected CASE WHEN rows in Nusantara");
    }

    // 2. ANSI: CASE WHEN ... THEN ... ELSE ... END
    let res_case_ansi = db.execute("
        SELECT nama,
               CASE
                   WHEN nilai >= 90 THEN 'A'
                   WHEN nilai >= 75 THEN 'B'
                   ELSE 'C'
               END AS grade
        FROM siswa
        ORDER BY id ASC;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_case_ansi {
        assert_eq!(affected, 3);
        assert_eq!(rows[0][1], SqlValue::Text("A".to_string()));
        assert_eq!(rows[1][1], SqlValue::Text("B".to_string()));
        assert_eq!(rows[2][1], SqlValue::Text("C".to_string()));
    } else {
        panic!("Expected CASE WHEN rows in ANSI");
    }

    // 3. JIKA_NIHIL / COALESCE / IFNULL
    let res_coalesce = db.execute("
        PILIH nama, JIKA_NIHIL(hp, 'Tidak Ada Kontak') AS kontak
        DARI siswa
        URUTKAN BERDASARKAN id NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_coalesce {
        assert_eq!(affected, 3);
        assert_eq!(rows[0][1], SqlValue::Text("081234".to_string()));
        assert_eq!(rows[1][1], SqlValue::Text("Tidak Ada Kontak".to_string()));
        assert_eq!(rows[2][1], SqlValue::Text("Tidak Ada Kontak".to_string()));
    } else {
        panic!("Expected COALESCE rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_6_scalar_functions() {
    let test_db = "target/test_widyabase_scalars.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL data_uji (id INT, teks TEKS, angka REAL);").unwrap();
    db.execute("SISIP_KE data_uji (id, teks, angka) NILAI (1, '  WidyaBase Nusantara  ', -12.75);").unwrap();

    // 1. String functions (Nusantara & ANSI)
    let res_str = db.execute("
        PILIH
            HURUF_BESAR(teks) AS besar,
            HURUF_KECIL(teks) AS kecil,
            PANJANG(teks) AS panjang,
            PANGKAS(teks) AS rapi,
            GABUNG_TEKS('Prefix: ', PANGKAS(teks)) AS gabungan
        DARI data_uji;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_str {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("  WIDYABASE NUSANTARA  ".to_string()));
        assert_eq!(rows[0][1], SqlValue::Text("  widyabase nusantara  ".to_string()));
        assert_eq!(rows[0][2], SqlValue::Integer(23));
        assert_eq!(rows[0][3], SqlValue::Text("WidyaBase Nusantara".to_string()));
        assert_eq!(rows[0][4], SqlValue::Text("Prefix: WidyaBase Nusantara".to_string()));
    } else {
        panic!("Expected string scalar functions");
    }

    // 2. Math functions (ROUND, ABS, FLOOR, CEIL)
    let res_math = db.execute("
        SELECT
            ABS(angka) AS mutlak,
            ROUND(angka) AS bulat,
            FLOOR(angka) AS lantai,
            CEIL(angka) AS atap
        FROM data_uji;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_math {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Float(12.75));
        assert_eq!(rows[0][1], SqlValue::Integer(-13));
        assert_eq!(rows[0][2], SqlValue::Integer(-13));
        assert_eq!(rows[0][3], SqlValue::Integer(-12));
    } else {
        panic!("Expected math scalar functions");
    }

    // 3. Temporal functions (NOW / SEKARANG, CURRENT_DATE / HARI_INI)
    let res_time = db.execute("PILIH SEKARANG() AS saat_ini, HARI_INI() AS hari;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_time {
        assert_eq!(affected, 1);
        let now_str = rows[0][0].as_text();
        let date_str = rows[0][1].as_text();
        assert!(!now_str.is_empty());
        assert!(!date_str.is_empty());
        assert_eq!(date_str.len(), 10); // YYYY-MM-DD
    } else {
        panic!("Expected temporal functions");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_7_union_operations() {
    let test_db = "target/test_widyabase_union.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL cabang_a (id INT, item TEKS);").unwrap();
    db.execute("BUAT_TABEL cabang_b (id INT, item TEKS);").unwrap();

    db.execute("SISIP_KE cabang_a (id, item) NILAI (1, 'Kopi');").unwrap();
    db.execute("SISIP_KE cabang_a (id, item) NILAI (2, 'Teh');").unwrap();

    db.execute("SISIP_KE cabang_b (id, item) NILAI (3, 'Teh');").unwrap(); // 'Teh' duplikat
    db.execute("SISIP_KE cabang_b (id, item) NILAI (4, 'Susu');").unwrap();

    // 1. Nusantara: GABUNGAN (UNION - deduplicated)
    let res_union = db.execute("
        PILIH item DARI cabang_a
        GABUNGAN
        PILIH item DARI cabang_b;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_union {
        assert_eq!(affected, 3); // 'Kopi', 'Teh', 'Susu'
        let items: Vec<String> = rows.iter().map(|r| r[0].as_text()).collect();
        assert!(items.contains(&"Kopi".to_string()));
        assert!(items.contains(&"Teh".to_string()));
        assert!(items.contains(&"Susu".to_string()));
    } else {
        panic!("Expected UNION rows");
    }

    // 2. ANSI: UNION ALL / GABUNGAN_SEMUA (preserve duplicates)
    let res_union_all = db.execute("
        SELECT item FROM cabang_a
        UNION ALL
        SELECT item FROM cabang_b;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_union_all {
        assert_eq!(affected, 4); // 'Kopi', 'Teh', 'Teh', 'Susu'
        let teh_count = rows.iter().filter(|r| r[0].as_text() == "Teh").count();
        assert_eq!(teh_count, 2);
    } else {
        panic!("Expected UNION ALL rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_8_subqueries() {
    let test_db = "target/test_widyabase_subqueries.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL produk (id INT, nama TEKS, harga INT, status TEKS);").unwrap();
    db.execute("SISIP_KE produk (id, nama, harga, status) NILAI (1, 'Laptop', 12000000, 'Tersedia');").unwrap();
    db.execute("SISIP_KE produk (id, nama, harga, status) NILAI (2, 'Mouse', 150000, 'Tersedia');").unwrap();
    db.execute("SISIP_KE produk (id, nama, harga, status) NILAI (3, 'Keyboard', 450000, 'Habis');").unwrap();
    db.execute("SISIP_KE produk (id, nama, harga, status) NILAI (4, 'Monitor', 2500000, 'Tersedia');").unwrap();

    db.execute("BUAT_TABEL pesanan_item (produk_id INT, kuantitas INT);").unwrap();
    db.execute("SISIP_KE pesanan_item (produk_id, kuantitas) NILAI (1, 2);").unwrap();
    db.execute("SISIP_KE pesanan_item (produk_id, kuantitas) NILAI (4, 1);").unwrap();

    // 1. Subquery IN (WHERE id IN (SELECT produk_id FROM pesanan_item))
    let res_in = db.execute("
        PILIH nama, harga
        DARI produk
        DIMANA id DIDALAM (PILIH produk_id DARI pesanan_item)
        URUTKAN BERDASARKAN id NAIK;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_in {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Laptop".to_string()));
        assert_eq!(rows[1][0], SqlValue::Text("Monitor".to_string()));
    } else {
        panic!("Expected IN subquery rows");
    }

    // 2. Scalar Subquery with comparison (WHERE harga > (SELECT AVG(harga) FROM ...))
    // Harga rata-rata: (12000000 + 150000 + 450000 + 2500000) / 4 = 15100000 / 4 = 3,775,000
    // Produk di atas rata-rata: hanya Laptop (12,000,000)
    let res_scalar = db.execute("
        SELECT nama, harga
        FROM produk
        WHERE harga > (SELECT RATA_RATA(harga) FROM produk);
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_scalar {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("Laptop".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(12000000));
    } else {
        panic!("Expected scalar subquery rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_9_common_table_expressions_cte() {
    let test_db = "target/test_widyabase_cte.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL karyawan (id INT, nama TEKS, divisi TEKS, gaji INT);").unwrap();
    db.execute("SISIP_KE karyawan (id, nama, divisi, gaji) NILAI (1, 'Budi', 'Teknologi', 15000000);").unwrap();
    db.execute("SISIP_KE karyawan (id, nama, divisi, gaji) NILAI (2, 'Siti', 'Teknologi', 18000000);").unwrap();
    db.execute("SISIP_KE karyawan (id, nama, divisi, gaji) NILAI (3, 'Agus', 'Pemasaran', 10000000);").unwrap();
    db.execute("SISIP_KE karyawan (id, nama, divisi, gaji) NILAI (4, 'Dewi', 'Pemasaran', 12000000);").unwrap();

    // 1. Dialek Nusantara: DENGAN ... SEBAGAI (...)
    let res_nusantara = db.execute("
        DENGAN gaji_tinggi SEBAGAI (
            PILIH nama, divisi, gaji
            DARI karyawan
            DIMANA gaji >= 15000000
        )
        PILIH nama, gaji DARI gaji_tinggi URUTKAN_BERDASARKAN gaji TURUN;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_nusantara {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Siti".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(18000000));
        assert_eq!(rows[1][0], SqlValue::Text("Budi".to_string()));
        assert_eq!(rows[1][1], SqlValue::Integer(15000000));
    } else {
        panic!("Expected CTE rows for Nusantara");
    }

    // 2. Standar Global ANSI: WITH ... AS (...)
    let res_ansi = db.execute("
        WITH divisi_pemasaran AS (
            SELECT nama, gaji
            FROM karyawan
            WHERE divisi = 'Pemasaran'
        )
        SELECT nama, gaji FROM divisi_pemasaran WHERE gaji > 10000000;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_ansi {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Text("Dewi".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(12000000));
    } else {
        panic!("Expected CTE rows for ANSI");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_10_window_functions() {
    let test_db = "target/test_widyabase_window.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL atlet (id INT, nama TEKS, cabang TEKS, skor INT);").unwrap();
    db.execute("SISIP_KE atlet (id, nama, cabang, skor) NILAI (1, 'Rian', 'Renang', 90);").unwrap();
    db.execute("SISIP_KE atlet (id, nama, cabang, skor) NILAI (2, 'Fajar', 'Renang', 95);").unwrap();
    db.execute("SISIP_KE atlet (id, nama, cabang, skor) NILAI (3, 'Kevin', 'Bulutangkis', 88);").unwrap();
    db.execute("SISIP_KE atlet (id, nama, cabang, skor) NILAI (4, 'Marcus', 'Bulutangkis', 98);").unwrap();
    db.execute("SISIP_KE atlet (id, nama, cabang, skor) NILAI (5, 'Hendra', 'Bulutangkis', 88);").unwrap();

    // 1. Dialek Nusantara: NOMOR_BARIS() LEWAT (PARTISI_BERDASARKAN cabang URUTKAN_BERDASARKAN skor TURUN)
    let res_nusantara = db.execute("
        PILIH nama, cabang, skor, NOMOR_BARIS() LEWAT (PARTISI_BERDASARKAN cabang URUTKAN_BERDASARKAN skor TURUN) SEBAGAI urutan
        DARI atlet
        URUTKAN_BERDASARKAN cabang NAIK, skor TURUN;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_nusantara {
        assert_eq!(affected, 5);
        // Marcus di Bulutangkis urutan 1
        assert_eq!(rows[0][0], SqlValue::Text("Marcus".to_string()));
        assert_eq!(rows[0][3], SqlValue::Integer(1));
        // Kevin di Bulutangkis urutan 2
        assert_eq!(rows[1][3], SqlValue::Integer(2));
        // Fajar di Renang urutan 1
        assert_eq!(rows[3][0], SqlValue::Text("Fajar".to_string()));
        assert_eq!(rows[3][3], SqlValue::Integer(1));
    } else {
        panic!("Expected Window Function rows for Nusantara");
    }

    // 2. Standar Global ANSI: RANK() OVER (ORDER BY skor DESC) & SUM(skor) OVER (PARTITION BY cabang)
    let res_ansi = db.execute("
        SELECT nama, cabang, skor, RANK() OVER (ORDER BY skor DESC) AS ranking, JUMLAH(skor) OVER (PARTITION BY cabang) AS total_cabang
        FROM atlet
        ORDER BY skor DESC;
    ").unwrap();

    if let QueryResult::Rows { rows, affected, .. } = res_ansi {
        assert_eq!(affected, 5);
        // Peringkat 1: Marcus (skor 98)
        assert_eq!(rows[0][0], SqlValue::Text("Marcus".to_string()));
        assert_eq!(rows[0][3], SqlValue::Integer(1));
        // Peringkat 2: Fajar (skor 95)
        assert_eq!(rows[1][0], SqlValue::Text("Fajar".to_string()));
        assert_eq!(rows[1][3], SqlValue::Integer(2));
        // Peringkat 3: Rian (skor 90)
        assert_eq!(rows[2][0], SqlValue::Text("Rian".to_string()));
        assert_eq!(rows[2][3], SqlValue::Integer(3));
        // Total skor renang = 90 + 95 = 185
        assert_eq!(rows[1][4], SqlValue::Float(185.0));
    } else {
        panic!("Expected Window Function rows for ANSI");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_11_secondary_indexes() {
    let test_db = "target/test_widyabase_index.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL pelanggan (id INT PRIMARY_KEY, email TEKS, kota TEKS);").unwrap();
    db.execute("SISIP_KE pelanggan (id, email, kota) NILAI (1, 'andi@mail.id', 'Jakarta');").unwrap();
    db.execute("SISIP_KE pelanggan (id, email, kota) NILAI (2, 'budi@mail.id', 'Bandung');").unwrap();
    db.execute("SISIP_KE pelanggan (id, email, kota) NILAI (3, 'citra@mail.id', 'Surabaya');").unwrap();

    // 1. Dialek Nusantara: BUAT_INDEKS idx_email PADA pelanggan (email)
    let res_idx = db.execute("BUAT_INDEKS idx_email PADA pelanggan (email);").unwrap();
    if let QueryResult::Message(msg) = res_idx {
        assert!(msg.contains("idx_email"));
    } else {
        panic!("Expected message result on CREATE INDEX");
    }

    // Kueri memanfaatkan Index Scan pada kolom sekunder
    let res_query = db.execute("PILIH id, kota DARI pelanggan DIMANA email = 'citra@mail.id';").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_query {
        assert_eq!(affected, 1);
        assert_eq!(rows[0][0], SqlValue::Integer(3));
        assert_eq!(rows[0][1], SqlValue::Text("Surabaya".to_string()));
    } else {
        panic!("Expected row result from secondary index scan");
    }

    // 2. Standar ANSI: DROP INDEX idx_email / HAPUS_INDEKS
    let res_drop = db.execute("DROP INDEX idx_email;").unwrap();
    if let QueryResult::Message(msg) = res_drop {
        assert!(msg.contains("idx_email"));
    } else {
        panic!("Expected message result on DROP INDEX");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_12_virtual_views() {
    let test_db = "target/test_widyabase_views.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL pesanan (id INT, pembeli TEKS, total INT, status TEKS);").unwrap();
    db.execute("SISIP_KE pesanan (id, pembeli, total, status) NILAI (1, 'Joko', 500000, 'LUNAS');").unwrap();
    db.execute("SISIP_KE pesanan (id, pembeli, total, status) NILAI (2, 'Sari', 300000, 'MENUNGGU');").unwrap();
    db.execute("SISIP_KE pesanan (id, pembeli, total, status) NILAI (3, 'Bowo', 1200000, 'LUNAS');").unwrap();

    // 1. Dialek Nusantara: BUAT_TAMPILAN tampilan_lunas SEBAGAI ...
    db.execute("
        BUAT_TAMPILAN tampilan_lunas SEBAGAI
            PILIH id, pembeli, total DARI pesanan DIMANA status = 'LUNAS';
    ").unwrap();

    // Kueri langsung dari View virtual
    let res_view = db.execute("PILIH pembeli, total DARI tampilan_lunas URUTKAN_BERDASARKAN total TURUN;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_view {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][0], SqlValue::Text("Bowo".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(1200000));
        assert_eq!(rows[1][0], SqlValue::Text("Joko".to_string()));
        assert_eq!(rows[1][1], SqlValue::Integer(500000));
    } else {
        panic!("Expected View rows");
    }

    // 2. Standar ANSI: DROP VIEW / HAPUS_TAMPILAN
    let res_drop = db.execute("DROP VIEW tampilan_lunas;").unwrap();
    if let QueryResult::Message(msg) = res_drop {
        assert!(msg.contains("tampilan_lunas"));
    } else {
        panic!("Expected DROP VIEW message");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_13_schema_evolution() {
    let test_db = "target/test_widyabase_alter.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL anggota (id INT, nama TEKS);").unwrap();
    db.execute("SISIP_KE anggota (id, nama) NILAI (1, 'Rudi');").unwrap();
    db.execute("SISIP_KE anggota (id, nama) NILAI (2, 'Nina');").unwrap();

    // 1. Tambah Kolom tanpa kehilangan data lama: UBAH_TABEL ... TAMBAH_KOLOM ...
    db.execute("UBAH_TABEL anggota TAMBAH_KOLOM nomor_hp TEKS;").unwrap();

    // Data lama tetap utuh dengan nomor_hp NULL
    let res_select = db.execute("PILIH id, nama, nomor_hp DARI anggota URUTKAN_BERDASARKAN id NAIK;").unwrap();
    if let QueryResult::Rows { rows, affected, .. } = res_select {
        assert_eq!(affected, 2);
        assert_eq!(rows[0][1], SqlValue::Text("Rudi".to_string()));
        assert_eq!(rows[0][2], SqlValue::Null);
    } else {
        panic!("Expected rows after ADD COLUMN");
    }

    // Perbarui data pada kolom baru
    db.execute("PERBARUI anggota ATUR nomor_hp = '08123456789' DIMANA id = 1;").unwrap();
    let res_updated = db.execute("PILIH nomor_hp DARI anggota DIMANA id = 1;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_updated {
        assert_eq!(rows[0][0], SqlValue::Text("08123456789".to_string()));
    } else {
        panic!("Expected updated column value");
    }

    // 2. Hapus Kolom: ALTER TABLE ... DROP COLUMN ...
    db.execute("ALTER TABLE anggota DROP COLUMN nomor_hp;").unwrap();
    let res_after_drop = db.execute("PILIH * DARI anggota;").unwrap();
    if let QueryResult::Rows { columns, affected, .. } = res_after_drop {
        assert_eq!(affected, 2);
        assert_eq!(columns, vec!["id".to_string(), "nama".to_string()]);
    } else {
        panic!("Expected columns after DROP COLUMN");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_14_foreign_key_integrity() {
    let test_db = "target/test_widyabase_fk.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    // Tabel Induk
    db.execute("BUAT_TABEL departemen (dept_id INT PRIMARY_KEY, nama_dept TEKS);").unwrap();
    db.execute("SISIP_KE departemen (dept_id, nama_dept) NILAI (10, 'Keuangan');").unwrap();
    db.execute("SISIP_KE departemen (dept_id, nama_dept) NILAI (20, 'SDM');").unwrap();

    // Tabel Anak dengan KUNCI_ASING
    db.execute("
        BUAT_TABEL staf (
            id INT PRIMARY_KEY,
            nama TEKS,
            dept_id INT,
            KUNCI_ASING (dept_id) REFERENSI departemen(dept_id)
        );
    ").unwrap();

    // 1. Sisip nilai foreign key valid -> SUKSES
    let res_valid = db.execute("SISIP_KE staf (id, nama, dept_id) NILAI (1, 'Ali', 10);");
    assert!(res_valid.is_ok());

    // 2. Sisip nilai foreign key yang TIDAK ADA di tabel induk -> DITOLAK
    let res_invalid = db.execute("SISIP_KE staf (id, nama, dept_id) NILAI (2, 'Badu', 99);");
    assert!(res_invalid.is_err());
    let err_msg = res_invalid.err().unwrap();
    assert!(err_msg.contains("Pelanggaran Kunci Asing"));

    // 3. Perbarui foreign key ke nilai tidak valid -> DITOLAK
    let res_update_invalid = db.execute("PERBARUI staf ATUR dept_id = 99 DIMANA id = 1;");
    assert!(res_update_invalid.is_err());

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pillar_15_savepoints_and_nested_transactions() {
    let test_db = "target/test_widyabase_savepoints.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("BUAT_TABEL dompet (id INT PRIMARY_KEY, pemilik TEKS, saldo INT);").unwrap();
    db.execute("SISIP_KE dompet (id, pemilik, saldo) NILAI (1, 'Hadi', 1000000);").unwrap();

    // Mulai Transaksi
    db.execute("MULAI;").unwrap();

    // Operasi 1: Tambah saldo
    db.execute("PERBARUI dompet ATUR saldo = 1500000 DIMANA id = 1;").unwrap();

    // Buat Titik Simpan (Savepoint)
    db.execute("TITIK_SIMPAN setelah_topup;").unwrap();

    // Operasi 2: Pengurangan yang salah
    db.execute("PERBARUI dompet ATUR saldo = 200000 DIMANA id = 1;").unwrap();

    // Verifikasi saldo sementara adalah 200,000
    let res_temp = db.execute("PILIH saldo DARI dompet DIMANA id = 1;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_temp {
        assert_eq!(rows[0][0], SqlValue::Integer(200000));
    }

    // Gugurkan kembali ke Titik Simpan: GUGURKAN_KE setelah_topup
    db.execute("GUGURKAN_KE setelah_topup;").unwrap();

    // Verifikasi saldo pulih ke 1,500,000!
    let res_rolled = db.execute("PILIH saldo DARI dompet DIMANA id = 1;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_rolled {
        assert_eq!(rows[0][0], SqlValue::Integer(1500000));
    } else {
        panic!("Expected balance restored to savepoint");
    }

    // Lepas Titik Simpan & Konfirmasi Transaksi
    db.execute("LEPAS_TITIK_SIMPAN setelah_topup;").unwrap();
    db.execute("KOMIT;").unwrap();

    // Pastikan setelah komit, saldo akhir tetap 1,500,000
    let res_final = db.execute("PILIH saldo DARI dompet DIMANA id = 1;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_final {
        assert_eq!(rows[0][0], SqlValue::Integer(1500000));
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_returning_clause() {
    let test_db = "target/test_widyabase_returning.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE users (id INT PRIMARY KEY, name TEXT, role TEXT);").unwrap();

    // 1. INSERT ... RETURNING
    let res_ins = db.execute("INSERT INTO users (id, name, role) VALUES (1, 'Budi', 'Admin') RETURNING id, name;").unwrap();
    if let QueryResult::Rows { columns, rows, affected } = res_ins {
        assert_eq!(columns, vec!["id", "name"]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], SqlValue::Integer(1));
        assert_eq!(rows[0][1], SqlValue::Text("Budi".to_string()));
        assert_eq!(affected, 1);
    } else {
        panic!("Expected rows from INSERT RETURNING");
    }

    // 2. UPDATE ... RETURNING (Dwibahasa: HASILKAN)
    let res_upd = db.execute("PERBARUI users ATUR role = 'SuperAdmin' DIMANA id = 1 HASILKAN id, role;").unwrap();
    if let QueryResult::Rows { columns, rows, affected } = res_upd {
        assert_eq!(columns, vec!["id", "role"]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], SqlValue::Integer(1));
        assert_eq!(rows[0][1], SqlValue::Text("SuperAdmin".to_string()));
        assert_eq!(affected, 1);
    } else {
        panic!("Expected rows from UPDATE RETURNING");
    }

    // 3. DELETE ... RETURNING
    let res_del = db.execute("DELETE FROM users WHERE id = 1 RETURNING name, role;").unwrap();
    if let QueryResult::Rows { columns, rows, affected } = res_del {
        assert_eq!(columns, vec!["name", "role"]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], SqlValue::Text("Budi".to_string()));
        assert_eq!(rows[0][1], SqlValue::Text("SuperAdmin".to_string()));
        assert_eq!(affected, 1);
    } else {
        panic!("Expected rows from DELETE RETURNING");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_on_conflict() {
    let test_db = "target/test_widyabase_conflict.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE products (id INT PRIMARY KEY, name TEXT, stock INT);").unwrap();
    db.execute("INSERT INTO products (id, name, stock) VALUES (101, 'Laptop', 5);").unwrap();

    // 1. ON CONFLICT DO NOTHING (Dwibahasa: JIKA_KONFLIK ABAIKAN)
    let res_nothing = db.execute("SISIP_KE products (id, name, stock) NILAI (101, 'Laptop Baru', 10) JIKA_KONFLIK ABAIKAN;").unwrap();
    if let QueryResult::Rows { affected, .. } = res_nothing {
        assert_eq!(affected, 0);
    }
    let res_check = db.execute("SELECT stock FROM products WHERE id = 101;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_check {
        assert_eq!(rows[0][0], SqlValue::Integer(5)); // Tetap 5
    }

    // 2. ON CONFLICT DO UPDATE
    let res_update = db.execute("INSERT INTO products (id, name, stock) VALUES (101, 'Laptop Super', 20) ON CONFLICT (id) DO UPDATE SET stock = 20, name = 'Laptop Super';").unwrap();
    if let QueryResult::Rows { affected, .. } = res_update {
        assert_eq!(affected, 1);
    }
    let res_check2 = db.execute("SELECT name, stock FROM products WHERE id = 101;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_check2 {
        assert_eq!(rows[0][0], SqlValue::Text("Laptop Super".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(20));
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_full_outer_and_cross_join() {
    let test_db = "target/test_widyabase_joins.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE t1 (id INT PRIMARY KEY, val1 TEXT);").unwrap();
    db.execute("CREATE TABLE t2 (id INT PRIMARY KEY, val2 TEXT);").unwrap();

    db.execute("INSERT INTO t1 (id, val1) VALUES (1, 'A');").unwrap();
    db.execute("INSERT INTO t1 (id, val1) VALUES (2, 'B');").unwrap();

    db.execute("INSERT INTO t2 (id, val2) VALUES (2, 'X');").unwrap();
    db.execute("INSERT INTO t2 (id, val2) VALUES (3, 'Y');").unwrap();

    // 1. FULL OUTER JOIN
    let res_full = db.execute("SELECT t1.id, t1.val1, t2.id, t2.val2 FROM t1 FULL OUTER JOIN t2 ON t1.id = t2.id;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_full {
        assert_eq!(rows.len(), 3); // (1, A, null, null), (2, B, 2, X), (null, null, 3, Y)
    } else {
        panic!("Expected rows from FULL OUTER JOIN");
    }

    // 2. CROSS JOIN (Cartesian product 2 x 2 = 4 baris)
    let res_cross = db.execute("SELECT t1.val1, t2.val2 FROM t1 CROSS JOIN t2;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_cross {
        assert_eq!(rows.len(), 4);
    } else {
        panic!("Expected 4 rows from CROSS JOIN");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_extended_window_functions() {
    let test_db = "target/test_widyabase_win.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE sales (emp TEXT, amount INT);").unwrap();
    db.execute("INSERT INTO sales (emp, amount) VALUES ('A', 100);").unwrap();
    db.execute("INSERT INTO sales (emp, amount) VALUES ('B', 200);").unwrap();
    db.execute("INSERT INTO sales (emp, amount) VALUES ('C', 300);").unwrap();
    db.execute("INSERT INTO sales (emp, amount) VALUES ('D', 400);").unwrap();

    // Test LEAD, LAG, FIRST_VALUE, LAST_VALUE, NTILE
    let res = db.execute("SELECT emp, amount, LEAD(amount) OVER (ORDER BY amount ASC) AS next_amt, LAG(amount) OVER (ORDER BY amount ASC) AS prev_amt, FIRST_VALUE(amount) OVER (ORDER BY amount ASC) AS first_amt, LAST_VALUE(amount) OVER (ORDER BY amount ASC) AS last_amt, NTILE(2) OVER (ORDER BY amount ASC) AS tile FROM sales;").unwrap();
    if let QueryResult::Rows { rows, .. } = res {
        assert_eq!(rows.len(), 4);
        // Row 0: emp=A, amt=100, next=200, prev=null, first=100, last=400, tile=1
        assert_eq!(rows[0][2], SqlValue::Integer(200));
        assert_eq!(rows[0][3], SqlValue::Null);
        assert_eq!(rows[0][4], SqlValue::Integer(100));
        assert_eq!(rows[0][5], SqlValue::Integer(400));
        assert_eq!(rows[0][6], SqlValue::Integer(1));

        // Row 3: emp=D, amt=400, next=null, prev=300, first=100, last=400, tile=2
        assert_eq!(rows[3][2], SqlValue::Null);
        assert_eq!(rows[3][3], SqlValue::Integer(300));
        assert_eq!(rows[3][6], SqlValue::Integer(2));
    } else {
        panic!("Expected rows from extended window functions");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_extended_aggregates() {
    let test_db = "target/test_widyabase_aggs.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE tags (item TEXT, tag TEXT);").unwrap();
    db.execute("INSERT INTO tags (item, tag) VALUES ('Book', 'SciFi');").unwrap();
    db.execute("INSERT INTO tags (item, tag) VALUES ('Book', 'Space');").unwrap();
    db.execute("INSERT INTO tags (item, tag) VALUES ('Book', 'Novel');").unwrap();

    // STRING_AGG / GROUP_CONCAT & ARRAY_AGG
    let res = db.execute("SELECT item, STRING_AGG(tag, ', ') AS all_tags, ARRAY_AGG(tag) AS arr_tags FROM tags GROUP BY item;").unwrap();
    if let QueryResult::Rows { rows, .. } = res {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][1], SqlValue::Text("SciFi, Space, Novel".to_string()));
        assert_eq!(rows[0][2], SqlValue::Text("[SciFi, Space, Novel]".to_string()));
    } else {
        panic!("Expected rows from STRING_AGG and ARRAY_AGG");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_extended_scalar_functions() {
    let test_db = "target/test_widyabase_scalars.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // NULLIF, GREATEST, LEAST, SUBSTRING, REPLACE, LPAD, RPAD, SPLIT_PART, CAST, MAKE_DATE, MOD
    let q = "SELECT NULLIF(10, 10) AS n1, NULLIF(10, 20) AS n2, GREATEST(5, 12, 8) AS g, LEAST(5, 12, 8) AS l, SUBSTRING('WidyaBase', 1, 5) AS sub, REPLACE('Postgres', 'Post', 'Widya') AS rep, LPAD('42', 5, '0') AS lpad, RPAD('Hi', 4, '!') AS rpad, SPLIT_PART('jakarta-bandung-surabaya', '-', 2) AS split, CAST('1234' AS INTEGER) AS c_int, MAKE_DATE(2026, 9, 30) AS mdate, MOD(17, 5) AS m;";
    let res = db.execute(q).unwrap();
    if let QueryResult::Rows { rows, .. } = res {
        assert_eq!(rows[0][0], SqlValue::Null);
        assert_eq!(rows[0][1], SqlValue::Integer(10));
        assert_eq!(rows[0][2], SqlValue::Integer(12));
        assert_eq!(rows[0][3], SqlValue::Integer(5));
        assert_eq!(rows[0][4], SqlValue::Text("Widya".to_string()));
        assert_eq!(rows[0][5], SqlValue::Text("Widyagres".to_string()));
        assert_eq!(rows[0][6], SqlValue::Text("00042".to_string()));
        assert_eq!(rows[0][7], SqlValue::Text("Hi!!".to_string()));
        assert_eq!(rows[0][8], SqlValue::Text("bandung".to_string()));
        assert_eq!(rows[0][9], SqlValue::Integer(1234));
        assert_eq!(rows[0][10], SqlValue::Text("2026-09-30".to_string()));
        assert_eq!(rows[0][11], SqlValue::Integer(2));
    } else {
        panic!("Expected scalar function evaluation results");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_intersect_and_except() {
    let test_db = "target/test_widyabase_set_ops.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE s1 (num INT);").unwrap();
    db.execute("CREATE TABLE s2 (num INT);").unwrap();

    db.execute("INSERT INTO s1 (num) VALUES (1);").unwrap();
    db.execute("INSERT INTO s1 (num) VALUES (2);").unwrap();
    db.execute("INSERT INTO s1 (num) VALUES (3);").unwrap();

    db.execute("INSERT INTO s2 (num) VALUES (2);").unwrap();
    db.execute("INSERT INTO s2 (num) VALUES (3);").unwrap();
    db.execute("INSERT INTO s2 (num) VALUES (4);").unwrap();

    // 1. INTERSECT (Dwibahasa: IRISAN) -> {2, 3}
    let res_inter = db.execute("PILIH num DARI s1 IRISAN PILIH num DARI s2;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_inter {
        assert_eq!(rows.len(), 2);
        let vals: Vec<i64> = rows.into_iter().map(|r| match r[0] { SqlValue::Integer(i) => i, _ => 0 }).collect();
        assert!(vals.contains(&2));
        assert!(vals.contains(&3));
    } else {
        panic!("Expected rows from INTERSECT");
    }

    // 2. EXCEPT (Dwibahasa: KECUALI) -> {1}
    let res_except = db.execute("SELECT num FROM s1 EXCEPT SELECT num FROM s2;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_except {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], SqlValue::Integer(1));
    } else {
        panic!("Expected rows from EXCEPT");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_column_constraints_serial_default_notnull_unique() {
    let test_db = "target/test_widyabase_constraints.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // SERIAL, DEFAULT, NOT NULL, UNIQUE (Dwibahasa & ANSI)
    db.execute("
        BUAT_TABEL akun (
            id SERIAL PRIMARY_KEY,
            username TEKS UNIQUE TIDAK_NIHIL,
            status TEKS DEFAULT 'Aktif',
            poin INT DEFAULT 100
        );
    ").unwrap();

    // 1. Auto-increment SERIAL & DEFAULT values
    db.execute("SISIP_KE akun (username) NILAI ('ahmad');").unwrap();
    db.execute("SISIP_KE akun (username) NILAI ('budi');").unwrap();

    let res = db.execute("PILIH id, username, status, poin DARI akun URUTKAN_BERDASARKAN id ASC;").unwrap();
    if let QueryResult::Rows { rows, .. } = res {
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], SqlValue::Integer(1));
        assert_eq!(rows[0][1], SqlValue::Text("ahmad".to_string()));
        assert_eq!(rows[0][2], SqlValue::Text("Aktif".to_string()));
        assert_eq!(rows[0][3], SqlValue::Integer(100));

        assert_eq!(rows[1][0], SqlValue::Integer(2));
        assert_eq!(rows[1][1], SqlValue::Text("budi".to_string()));
    } else {
        panic!("Expected serial and default values");
    }

    // 2. UNIQUE constraint rejection
    let res_dup = db.execute("SISIP_KE akun (username) NILAI ('ahmad');");
    assert!(res_dup.is_err());
    assert!(res_dup.err().unwrap().contains("Pelanggaran batasan UNIK"));

    // 3. NOT NULL constraint rejection
    let res_null = db.execute("SISIP_KE akun (id, status) NILAI (3, 'Pending');");
    assert!(res_null.is_err());
    assert!(res_null.err().unwrap().contains("tidak boleh NIHIL"));

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_truncate_vacuum_show_explain() {
    let test_db = "target/test_widyabase_mgmt.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE items (id INT PRIMARY KEY, name TEXT);").unwrap();
    db.execute("INSERT INTO items (id, name) VALUES (1, 'Box');").unwrap();

    // 1. SHOW TABLES
    let res_tables = db.execute("SHOW TABLES;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_tables {
        assert!(rows.iter().any(|r| r[0] == SqlValue::Text("ITEMS".to_string())));
    }

    // 2. SHOW COLUMNS
    let res_cols = db.execute("SHOW COLUMNS FROM items;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_cols {
        assert_eq!(rows.len(), 2);
    }

    // 3. EXPLAIN & EXPLAIN ANALYZE
    let res_explain = db.execute("EXPLAIN SELECT * FROM items WHERE id = 1;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_explain {
        assert!(!rows.is_empty());
    }

    let res_analyze = db.execute("EXPLAIN ANALYZE SELECT * FROM items WHERE id = 1;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_analyze {
        let plan_text: String = rows.into_iter().map(|r| r[0].as_text()).collect::<Vec<_>>().join("\n");
        assert!(plan_text.contains("Execution Time"));
    }

    // 4. VACUUM
    let res_vacuum = db.execute("VACUUM;").unwrap();
    assert!(matches!(res_vacuum, QueryResult::Message(_)));

    // 5. TRUNCATE TABLE
    let res_trunc = db.execute("TRUNCATE TABLE items;").unwrap();
    assert!(matches!(res_trunc, QueryResult::Message(_)));

    let res_after = db.execute("SELECT COUNT(*) FROM items;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_after {
        assert_eq!(rows[0][0], SqlValue::Integer(0));
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_redis_compatible_cache_engine() {
    let test_db = "target/test_widyabase_redis.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // 1. SET & GET & TTL
    db.execute("SET user:100 'Widya User' EX 60").unwrap();
    let res = db.execute("GET user:100").unwrap();
    if let QueryResult::Rows { rows, .. } = res {
        assert_eq!(rows[0][0], SqlValue::Text("Widya User".to_string()));
    } else {
        panic!("Expected get result");
    }

    let ttl_res = db.execute("TTL user:100").unwrap();
    if let QueryResult::Message(msg) = ttl_res {
        assert!(msg.starts_with("(integer) "));
    }

    // 2. INCR / INCRBY
    db.execute("SET counter 10").unwrap();
    let incr_res = db.execute("INCR counter").unwrap();
    assert_eq!(incr_res_str(&incr_res), "(integer) 11");

    let incrby_res = db.execute("INCRBY counter 5").unwrap();
    assert_eq!(incr_res_str(&incrby_res), "(integer) 16");

    // 3. HASHES (HSET, HGET, HGETALL)
    db.execute("HSET user:profile name 'Budi'").unwrap();
    db.execute("HSET user:profile age 25").unwrap();
    let hget_res = db.execute("HGET user:profile name").unwrap();
    if let QueryResult::Rows { rows, .. } = hget_res {
        assert_eq!(rows[0][0], SqlValue::Text("Budi".to_string()));
    }

    let hgetall_res = db.execute("HGETALL user:profile").unwrap();
    if let QueryResult::Rows { rows, .. } = hgetall_res {
        assert_eq!(rows.len(), 2);
    }

    // 4. LISTS (LPUSH, RPUSH, LPOP, RPOP, LRANGE)
    db.execute("LPUSH mylist item1").unwrap();
    db.execute("RPUSH mylist item2").unwrap();
    let lrange_res = db.execute("LRANGE mylist 0 -1").unwrap();
    if let QueryResult::Rows { rows, .. } = lrange_res {
        assert_eq!(rows.len(), 2);
    }

    // 5. SETS (SADD, SMEMBERS, SREM)
    db.execute("SADD myset apple banana cherry").unwrap();
    let smembers_res = db.execute("SMEMBERS myset").unwrap();
    if let QueryResult::Rows { rows, .. } = smembers_res {
        assert_eq!(rows.len(), 3);
    }

    // 6. SORTED SETS (ZADD, ZRANGE)
    db.execute("ZADD leaderboard 100 'player1'").unwrap();
    db.execute("ZADD leaderboard 200 'player2'").unwrap();
    let zrange_res = db.execute("ZRANGE leaderboard 0 -1 WITHSCORES").unwrap();
    if let QueryResult::Rows { rows, columns, .. } = zrange_res {
        assert_eq!(columns, vec!["member", "score"]);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], SqlValue::Text("player1".to_string()));
        assert_eq!(rows[0][1], SqlValue::Integer(100));
        assert_eq!(rows[1][0], SqlValue::Text("player2".to_string()));
        assert_eq!(rows[1][1], SqlValue::Integer(200));
    }

    // 7. RESP Wire Protocol Execution Test
    let redis_res = db.cache().execute_redis_command("PING").unwrap();
    assert_eq!(redis_res, "+PONG\r\n");

    let redis_set = db.cache().execute_redis_command("SET test_wire hello").unwrap();
    assert_eq!(redis_set, "+OK\r\n");

    let redis_get = db.cache().execute_redis_command("GET test_wire").unwrap();
    assert_eq!(redis_get, "$5\r\nhello\r\n");

    clean_test_file(test_db);
}

fn incr_res_str(res: &QueryResult) -> String {
    match res {
        QueryResult::Message(s) => s.clone(),
        _ => String::new(),
    }
}

#[test]
fn test_widyabase_clickhouse_compatible_columnar_olap() {
    let test_db = "target/test_widyabase_columnar.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // 1. CREATE TABLE dengan ENGINE = MergeTree & ORDER BY
    db.execute("CREATE TABLE web_events (user_id INT, duration FLOAT, url TEXT) ENGINE = MergeTree ORDER BY (user_id);").unwrap();

    // 2. Direct Columnar Engine & Vectorized Aggregation
    let columnar = db.columnar();
    assert!(columnar.has_table("web_events"));

    // Sisipkan jutaan baris analitik simulasi secara chunk-vectorized
    let mut rows = Vec::new();
    for i in 1..=1000 {
        rows.push(vec![
            SqlValue::Integer(i as i64),
            SqlValue::Float(i as f64 * 1.5),
            SqlValue::Text(format!("/page/{}", i % 10)),
        ]);
    }
    let inserted = columnar.insert("web_events", &["user_id".to_string(), "duration".to_string(), "url".to_string()], rows).unwrap();
    assert_eq!(inserted, 1000);

    // Vectorized Aggregations (SUM, AVG, MIN, MAX, COUNT)
    let sum_duration = columnar.execute_olap_aggregation("web_events", "SUM", "duration").unwrap();
    if let SqlValue::Float(sum) = sum_duration {
        // sum = 1.5 * (1000 * 1001 / 2) = 1.5 * 500500 = 750750.0
        assert_eq!(sum, 750750.0);
    } else {
        panic!("Expected float sum");
    }

    let avg_duration = columnar.execute_olap_aggregation("web_events", "AVG", "duration").unwrap();
    if let SqlValue::Float(avg) = avg_duration {
        // avg = 750750.0 / 1000 = 750.75
        assert_eq!(avg, 750.75);
    }

    let count_events = columnar.execute_olap_aggregation("web_events", "COUNT", "user_id").unwrap();
    assert_eq!(count_events, SqlValue::Integer(1000));

    let min_user = columnar.execute_olap_aggregation("web_events", "MIN", "user_id").unwrap();
    assert_eq!(min_user, SqlValue::Integer(1));

    let max_user = columnar.execute_olap_aggregation("web_events", "MAX", "user_id").unwrap();
    assert_eq!(max_user, SqlValue::Integer(1000));

    clean_test_file(test_db);
}

