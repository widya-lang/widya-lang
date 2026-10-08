use widya::compiler::decimal::Desimal;
use widya::widyabase::btree::{BTreeIndex, IndexKey, RecordPointer};
use widya::widyabase::hybrid_search::{HybridDocument, HybridSearchEngine};
use widya::widyabase::hnsw::HnswVectorIndex;
use widya::widyabase::{
    CdcFilter, CdcOp, CdcRecord, QueryResult, RaftCluster, RaftRole, TwoPhaseCoordinator,
    TwoPhaseParticipant, TwoPhaseStatus, WidyaBase,
};
use widya::widyabase::sql_parser::SqlValue;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

fn clean_test_file(path: &str) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(format!("{}.wal", path));
    let mut p = PathBuf::from(path);
    p.set_extension("wal");
    let _ = fs::remove_file(p);
}

#[test]
fn test_widyabase_decimal_btree_index() {
    let index = BTreeIndex::new("idx_nominal_transaksi");

    let k1 = IndexKey::Decimal(Desimal::dari_int(100000));
    let k2 = IndexKey::Decimal(Desimal::dari_int(500000));
    let k3 = IndexKey::Decimal(Desimal::dari_int(1500000));

    index.insert(k1.clone(), RecordPointer { page_id: 1, slot_id: 0 });
    index.insert(k2.clone(), RecordPointer { page_id: 1, slot_id: 1 });
    index.insert(k3.clone(), RecordPointer { page_id: 2, slot_id: 0 });

    assert_eq!(index.len(), 3);
    assert_eq!(index.search(&k2), Some(RecordPointer { page_id: 1, slot_id: 1 }));

    // Range Scan: >= 200.000 AND <= 1.000.000
    let min_k = IndexKey::Decimal(Desimal::dari_int(200000));
    let max_k = IndexKey::Decimal(Desimal::dari_int(1000000));
    let range = index.range_scan(Some(&min_k), Some(&max_k));

    assert_eq!(range.len(), 1);
    assert_eq!(range[0].0, k2);
}

#[test]
fn test_widyabase_hybrid_vector_search() {
    let mut engine = HybridSearchEngine::new();

    let doc1 = HybridDocument {
        id: "DOC-01".into(),
        text_content: "Sistem Pembayaran QRIS BI-FAST Perbankan Nasional".into(),
        embedding: vec![0.9, 0.1, 0.2, 0.0],
        metadata: HashMap::new(),
    };
    let doc2 = HybridDocument {
        id: "DOC-02".into(),
        text_content: "Klinik SatuSehat Rekam Medis Elektronik Pasien BPJS".into(),
        embedding: vec![0.1, 0.9, 0.1, 0.3],
        metadata: HashMap::new(),
    };

    engine.index_document(doc1);
    engine.index_document(doc2);

    let query_tokens = vec!["QRIS", "Perbankan"];
    let query_vector = vec![0.85, 0.15, 0.10, 0.0];

    let results = engine.search_hybrid(&query_tokens, &query_vector, 5, 60.0);
    assert!(!results.is_empty());
    assert_eq!(results[0].document.id, "DOC-01");
    assert!(results[0].rrf_score > 0.0);
}

#[test]
fn test_widyabase_hnsw_vector_index() {
    let mut hnsw = HnswVectorIndex::new(16, 32);

    let v_finance = vec![0.95, 0.05, 0.10, 0.02];
    let v_health = vec![0.05, 0.92, 0.08, 0.15];
    let v_education = vec![0.12, 0.10, 0.88, 0.30];

    hnsw.insert("DOC-FINANCE", &v_finance);
    hnsw.insert("DOC-HEALTH", &v_health);
    hnsw.insert("DOC-EDU", &v_education);

    assert_eq!(hnsw.len(), 3);

    // Query mirip finance
    let q_finance = vec![0.90, 0.08, 0.05, 0.01];
    let knn = hnsw.search_knn(&q_finance, 2, 16);

    assert!(!knn.is_empty());
    assert_eq!(knn[0].0, "DOC-FINANCE");
    assert!(knn[0].1 > 0.95, "Similaritas kosinus mendekati 1.0");

    // Query mirip kesehatan
    let q_health = vec![0.02, 0.89, 0.10, 0.20];
    let knn_health = hnsw.search_knn(&q_health, 1, 16);
    assert_eq!(knn_health[0].0, "DOC-HEALTH");
}

#[test]
fn test_widyabase_cbo_table_statistics_collection() {
    let test_db = "target/test_widyabase_cbo_stats.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // 1. Buat tabel dan sisipkan data uji
    db.execute("BUAT_TABEL pelanggan (id INT PRIMARY KEY, nama TEXT, kota TEXT, saldo INT);").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama, kota, saldo) NILAI (1, 'Budi', 'Jakarta', 5000);").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama, kota, saldo) NILAI (2, 'Siti', 'Bandung', 7500);").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama, kota, saldo) NILAI (3, 'Agus', 'Jakarta', 3000);").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama, kota, saldo) NILAI (4, 'Dewi', 'Surabaya', 12000);").unwrap();
    db.execute("SISIP_KE pelanggan (id, nama, kota, saldo) NILAI (5, 'Eko', 'Jakarta', 5000);").unwrap();

    // 2. Jalankan perintah dwibahasa ANALISIS_TABEL
    let analyze_res = db.execute("ANALISIS_TABEL pelanggan;").unwrap();
    if let QueryResult::Message(msg) = analyze_res {
        assert!(msg.contains("ANALISIS_TABEL selesai"));
        assert!(msg.contains("PELANGGAN"));
    } else {
        panic!("Diharapkan QueryResult::Message");
    }

    // 3. Verifikasi statistik yang terkumpul pada catalog
    let stats = db.get_table_stats("pelanggan").expect("Statistik tabel pelanggan harus ada");
    assert_eq!(stats.table_name, "PELANGGAN");
    assert_eq!(stats.row_count, 5);
    assert!(stats.page_count >= 1);

    // Cek kolom 'kota'
    let col_kota = stats.columns.get("KOTA").expect("Statistik kolom KOTA harus ada");
    assert_eq!(col_kota.column_name, "kota");
    assert_eq!(col_kota.null_count, 0);
    assert_eq!(col_kota.distinct_count, 3); // Jakarta, Bandung, Surabaya
    assert!(!col_kota.most_common_vals.is_empty());
    // 'Jakarta' muncul 3 kali
    assert_eq!(col_kota.most_common_vals[0].0, SqlValue::Text("Jakarta".to_string()));
    assert_eq!(col_kota.most_common_vals[0].1, 3);

    // Cek kolom 'saldo'
    let col_saldo = stats.columns.get("SALDO").expect("Statistik kolom SALDO harus ada");
    assert_eq!(col_saldo.min_val, Some(SqlValue::Integer(3000)));
    assert_eq!(col_saldo.max_val, Some(SqlValue::Integer(12000)));

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_cbo_index_vs_seq_scan_planning() {
    let test_db = "target/test_widyabase_cbo_scan.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    db.execute("CREATE TABLE products (id INT PRIMARY KEY, name TEXT, category TEXT, price INT);").unwrap();

    for i in 1..=100 {
        let cat = if i % 2 == 0 { "Elektronik" } else { "Pakaian" };
        let sql = format!("INSERT INTO products (id, name, category, price) VALUES ({}, 'Produk_{}', '{}', {});", i, i, cat, i * 1000);
        db.execute(&sql).unwrap();
    }

    // Analisis tabel untuk memicu CBO cost estimates yang akurat
    db.execute("ANALYZE TABLE products;").unwrap();

    // Skenario A: Kueri WHERE pada Primary Key -> CBO harus memilih Index Scan
    let res_pk = db.execute("EXPLAIN SELECT * FROM products WHERE id = 15;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_pk {
        let plan_text: String = rows.into_iter().map(|r| r[0].as_text()).collect::<Vec<_>>().join("\n");
        assert!(plan_text.contains("Index Scan using pk_products on products"), "Rencana kueri harus menggunakan Index Scan untuk PK: {}", plan_text);
        assert!(plan_text.contains("cost=0.15.."), "Biaya Index Scan startup harus 0.15: {}", plan_text);
    } else {
        panic!("Diharapkan QueryResult::Rows");
    }

    // Skenario B: Kueri WHERE pada kolom yang belum diindeks ('price') -> CBO harus memilih Seq Scan
    let res_seq = db.execute("EXPLAIN SELECT * FROM products WHERE price = 10000;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_seq {
        let plan_text: String = rows.into_iter().map(|r| r[0].as_text()).collect::<Vec<_>>().join("\n");
        assert!(plan_text.contains("Seq Scan on products"), "Rencana kueri harus menggunakan Seq Scan: {}", plan_text);
        assert!(plan_text.contains("cost=0.00.."), "Biaya Seq Scan startup harus 0.00: {}", plan_text);
    } else {
        panic!("Diharapkan QueryResult::Rows");
    }

    // Skenario C: Buat Secondary Index pada 'price'
    db.execute("CREATE INDEX idx_products_price ON products(price);").unwrap();

    // Sekarang kueri WHERE price = 10000 harus beralih ke Index Scan!
    let res_idx = db.execute("EXPLAIN SELECT * FROM products WHERE price = 10000;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_idx {
        let plan_text: String = rows.into_iter().map(|r| r[0].as_text()).collect::<Vec<_>>().join("\n");
        assert!(plan_text.to_lowercase().contains("index scan using idx_products_price on products"), "Rencana kueri harus beralih ke Index Scan setelah index dibuat: {}", plan_text);
    } else {
        panic!("Diharapkan QueryResult::Rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_enhanced_explain_analyze_bilingual() {
    let test_db = "target/test_widyabase_cbo_explain_analyze.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    db.execute("BUAT_TABEL pesanan (id INT PRIMARY KEY, pembeli TEXT, jumlah INT);").unwrap();
    for i in 1..=100 {
        db.execute(&format!("SISIP_KE pesanan (id, pembeli, jumlah) NILAI ({}, 'Pembeli_{}', {});", i, i, i % 10 + 1)).unwrap();
    }

    db.execute("ANALISIS_TABEL pesanan;").unwrap();

    // Uji perintah dwibahasa: JELASKAN ANALISIS PILIH ...
    let res = db.execute("JELASKAN ANALISIS PILIH * DARI pesanan DIMANA id = 42;").unwrap();
    if let QueryResult::Rows { rows, .. } = res {
        let plan_text: String = rows.into_iter().map(|r| r[0].as_text()).collect::<Vec<_>>().join("\n");
        assert!(plan_text.contains("Index Scan using pk_pesanan on pesanan"), "Harus menggunakan Index Scan: {}", plan_text);
        assert!(plan_text.contains("actual time="), "Harus memiliki metrik waktu aktual: {}", plan_text);
        assert!(plan_text.contains("rows=1 loops=1"), "Harus menampilkan actual rows dan loops: {}", plan_text);
        assert!(plan_text.contains("Planning Time:"), "Harus menampilkan planning time: {}", plan_text);
        assert!(plan_text.contains("Execution Time:"), "Harus menampilkan execution time: {}", plan_text);
    } else {
        panic!("Diharapkan QueryResult::Rows");
    }

    // Uji perintah standar ANSI: EXPLAIN ANALYZE SELECT ...
    let res_ansi = db.execute("EXPLAIN ANALYZE SELECT * FROM pesanan WHERE id = 42;").unwrap();
    if let QueryResult::Rows { rows, .. } = res_ansi {
        let plan_text: String = rows.into_iter().map(|r| r[0].as_text()).collect::<Vec<_>>().join("\n");
        assert!(plan_text.contains("Index Scan using pk_pesanan on pesanan"));
        assert!(plan_text.contains("actual time="));
        assert!(plan_text.contains("Execution Time:"));
    } else {
        panic!("Diharapkan QueryResult::Rows");
    }

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_raft_cluster_election_and_quorum_replication() {
    // 1. Buat cluster Raft 3-node
    let cluster = RaftCluster::new(vec![1, 2, 3]);

    let node1 = cluster.get_node(1).expect("Node 1 harus ada");
    let node2 = cluster.get_node(2).expect("Node 2 harus ada");
    let node3 = cluster.get_node(3).expect("Node 3 harus ada");

    assert_eq!(node1.get_role(), RaftRole::Follower);
    assert_eq!(node2.get_role(), RaftRole::Follower);
    assert_eq!(node3.get_role(), RaftRole::Follower);

    // 2. Pemilihan Leader: Picu pemilihan untuk Node 1
    let election_ok = cluster.start_election_for(1);
    assert!(election_ok, "Node 1 harus berhasil memenangkan pemilu Raft dengan kuorum");
    assert_eq!(node1.get_role(), RaftRole::Leader);
    assert_eq!(cluster.get_leader(), Some(1));
    assert_eq!(node1.current_term.load(Ordering::SeqCst), 1);

    // 3. Replikasi Perintah SQL oleh Leader (Node 1) ke Quorum
    let cmd1 = b"INSERT INTO akun (id, saldo) VALUES (1, 50000000);".to_vec();
    let rep_res = cluster.replicate(1, cmd1.clone());
    assert!(rep_res.is_ok(), "Replikasi ke seluruh node cluster harus berhasil");
    let (idx, quorums) = rep_res.unwrap();
    assert_eq!(idx, 1);
    assert_eq!(quorums, 3); // 3/3 node konfirmasi

    // Pastikan commit index tersinkronisasi di semua node
    assert_eq!(node1.commit_index.load(Ordering::SeqCst), 1);
    assert_eq!(node2.commit_index.load(Ordering::SeqCst), 1);
    assert_eq!(node3.commit_index.load(Ordering::SeqCst), 1);

    // 4. Simulasi Kegagalan Jaringan / Partisi: Isolasi Node 1
    cluster.isolate_node(1);
    assert!(cluster.is_isolated(1));
    assert_eq!(cluster.get_leader(), None, "Leader yang terisolasi tidak boleh diakui cluster");

    // Replikasi dari Node 1 yang terisolasi harus gagal
    let failed_rep = cluster.replicate(1, b"SISIP...".to_vec());
    assert!(failed_rep.is_err());

    // 5. Failover Pemilihan: Node 2 memicu pemilihan dan menjadi Leader baru (Quorum 2/3: Node 2 & Node 3)
    let election2_ok = cluster.start_election_for(2);
    assert!(election2_ok, "Node 2 harus menang pemilihan dengan kuorum 2 dari 3 node");
    assert_eq!(node2.get_role(), RaftRole::Leader);
    assert_eq!(cluster.get_leader(), Some(2));
    assert_eq!(node2.current_term.load(Ordering::SeqCst), 2);

    // Replikasi perintah kedua melalui Leader baru (Node 2)
    let cmd2 = b"UPDATE akun SET saldo = 45000000 WHERE id = 1;".to_vec();
    let rep2_res = cluster.replicate(2, cmd2);
    assert!(rep2_res.is_ok());
    let (idx2, quorums2) = rep2_res.unwrap();
    assert_eq!(idx2, 2);
    assert_eq!(quorums2, 2); // Node 2 & Node 3

    assert_eq!(node2.commit_index.load(Ordering::SeqCst), 2);
    assert_eq!(node3.commit_index.load(Ordering::SeqCst), 2);

    // 6. Pemulihan Jaringan: Sambungkan kembali Node 1 dan kirim Heartbeat
    cluster.reconnect_node(1);
    assert!(!cluster.is_isolated(1));

    // Kirim heartbeat dari Node 2 ke Node 1
    cluster.broadcast_heartbeat(2);
    // Node 1 harus memperbarui term-nya dan mengakui role Follower
    assert_eq!(node1.current_term.load(Ordering::SeqCst), 2);
    assert_eq!(node1.get_role(), RaftRole::Follower);
}

#[test]
fn test_widyabase_distributed_2pc_coordinator_success_and_abort() {
    let coordinator = TwoPhaseCoordinator::new(100);

    let p1 = Arc::new(TwoPhaseParticipant::new(10));
    let p2 = Arc::new(TwoPhaseParticipant::new(20));
    let p3 = Arc::new(TwoPhaseParticipant::new(30));

    coordinator.register_participant(Arc::clone(&p1));
    coordinator.register_participant(Arc::clone(&p2));
    coordinator.register_participant(Arc::clone(&p3));

    // Skenario A: Transaksi Terdistribusi Berhasil (Global Commit)
    let tx1 = 5001;
    let payload1 = b"TRANSFER 1000000 DARI 10 KE 20".to_vec();
    let res1 = coordinator.execute_2pc(tx1, &[10, 20, 30], payload1);

    assert_eq!(res1, Ok(TwoPhaseStatus::Committed));
    assert_eq!(coordinator.get_transaction_status(tx1), Some(TwoPhaseStatus::Committed));
    assert_eq!(p1.get_status(tx1), Some(TwoPhaseStatus::Committed));
    assert_eq!(p2.get_status(tx1), Some(TwoPhaseStatus::Committed));
    assert_eq!(p3.get_status(tx1), Some(TwoPhaseStatus::Committed));

    // Skenario B: Transaksi Terdistribusi Gagal / Dibatalkan (Global Abort)
    // Simulasikan p2 menolak (misal saldo tidak cukup atau constraint violation)
    let tx2 = 5002;
    p2.set_forced_abort(tx2, true);

    let payload2 = b"TRANSFER 999999999 DARI 10 KE 20".to_vec();
    let res2 = coordinator.execute_2pc(tx2, &[10, 20, 30], payload2);

    assert_eq!(res2, Ok(TwoPhaseStatus::Aborted));
    assert_eq!(coordinator.get_transaction_status(tx2), Some(TwoPhaseStatus::Aborted));
    // Seluruh partisipan harus beralih ke status Aborted demi konsistensi atomik terdistribusi
    assert_eq!(p1.get_status(tx2), Some(TwoPhaseStatus::Aborted));
    assert_eq!(p2.get_status(tx2), Some(TwoPhaseStatus::Aborted));
    assert_eq!(p3.get_status(tx2), Some(TwoPhaseStatus::Aborted));
}

#[test]
fn test_widyabase_cdc_realtime_streaming_dml_and_debezium_format() {
    let test_db = "target/test_widyabase_cdc_dml.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // 1. Berlangganan aliran CDC secara langsung sebelum transaksi
    let mut sub = db.subscribe_cdc(None, None);

    // 2. Buat tabel
    db.execute("BUAT_TABEL rekening (id INT PRIMARY KEY, pemilik TEXT, saldo INT);").unwrap();

    // 3. Eksekusi INSERT
    db.execute("SISIP_KE rekening (id, pemilik, saldo) NILAI (1, 'Budi Santoso', 5000000);").unwrap();

    // Terima event CDC Insert
    let event1 = sub.recv_timeout(Duration::from_millis(500))
        .expect("Kanal tidak boleh error")
        .expect("Event INSERT harus diterima");

    assert_eq!(event1.op, CdcOp::Insert);
    assert_eq!(event1.table, "REKENING");
    assert!(event1.before.is_none());
    assert!(event1.after.is_some());
    let after1 = event1.after.as_ref().unwrap();
    assert_eq!(after1.get("ID"), Some(&SqlValue::Integer(1)));
    assert_eq!(after1.get("PEMILIK"), Some(&SqlValue::Text("Budi Santoso".to_string())));
    assert_eq!(after1.get("SALDO"), Some(&SqlValue::Integer(5000000)));

    // Verifikasi format Debezium
    let deb1 = event1.to_debezium_format();
    assert_eq!(deb1["op"], "c");
    assert_eq!(deb1["source"]["table"], "rekening");
    assert_eq!(deb1["after"]["PEMILIK"], "Budi Santoso");

    // 4. Eksekusi UPDATE
    db.execute("PERBARUI rekening ATUR saldo = 7500000 DIMANA id = 1;").unwrap();

    // Terima event CDC Update
    let event2 = sub.recv_timeout(Duration::from_millis(500))
        .expect("Kanal tidak boleh error")
        .expect("Event UPDATE harus diterima");

    assert_eq!(event2.op, CdcOp::Update);
    assert_eq!(event2.table, "REKENING");
    assert!(event2.before.is_some());
    assert!(event2.after.is_some());
    let before2 = event2.before.as_ref().unwrap();
    let after2 = event2.after.as_ref().unwrap();
    assert_eq!(before2.get("SALDO"), Some(&SqlValue::Integer(5000000)));
    assert_eq!(after2.get("SALDO"), Some(&SqlValue::Integer(7500000)));

    let deb2 = event2.to_debezium_format();
    assert_eq!(deb2["op"], "u");
    assert_eq!(deb2["before"]["SALDO"], 5000000);
    assert_eq!(deb2["after"]["SALDO"], 7500000);

    // 5. Eksekusi DELETE
    db.execute("HAPUS_DARI rekening DIMANA id = 1;").unwrap();

    // Terima event CDC Delete
    let event3 = sub.recv_timeout(Duration::from_millis(500))
        .expect("Kanal tidak boleh error")
        .expect("Event DELETE harus diterima");

    assert_eq!(event3.op, CdcOp::Delete);
    assert_eq!(event3.table, "REKENING");
    assert!(event3.before.is_some());
    assert!(event3.after.is_none());
    let before3 = event3.before.as_ref().unwrap();
    assert_eq!(before3.get("PEMILIK"), Some(&SqlValue::Text("Budi Santoso".to_string())));

    let deb3 = event3.to_debezium_format();
    assert_eq!(deb3["op"], "d");
    assert_eq!(deb3["before"]["PEMILIK"], "Budi Santoso");

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_cdc_filtering_and_historical_replay() {
    let test_db = "target/test_widyabase_cdc_replay.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    db.execute("CREATE TABLE users (id INT PRIMARY KEY, name TEXT);").unwrap();
    db.execute("CREATE TABLE orders (id INT PRIMARY KEY, user_id INT, amount INT);").unwrap();

    // Lakukan beberapa operasi sebelum ada subscriber (rekam ke history)
    db.execute("INSERT INTO users (id, name) VALUES (1, 'Alice');").unwrap();
    db.execute("INSERT INTO orders (id, user_id, amount) VALUES (101, 1, 50000);").unwrap();
    db.execute("INSERT INTO users (id, name) VALUES (2, 'Bob');").unwrap();
    db.execute("INSERT INTO orders (id, user_id, amount) VALUES (102, 2, 75000);").unwrap();

    // Sekarang buat subscriber yang difilter HANYA untuk tabel 'ORDERS' dan operasi INSERT
    // dengan historical replay dari awal (min_lsn = 1)
    let filter = CdcFilter::new()
        .with_table("ORDERS")
        .with_operation(CdcOp::Insert);

    let mut sub_orders = db.subscribe_cdc(Some(filter), Some(1));

    // Ambil event pertama dari replay: harus pesanan 101
    let ev1 = sub_orders.recv_timeout(Duration::from_millis(200))
        .expect("Kanal ok")
        .expect("Harus menerima pesanan 101 dari riwayat");
    assert_eq!(ev1.table, "ORDERS");
    assert_eq!(ev1.op, CdcOp::Insert);
    assert_eq!(ev1.after.unwrap().get("ID"), Some(&SqlValue::Integer(101)));

    // Ambil event kedua dari replay: harus pesanan 102
    let ev2 = sub_orders.recv_timeout(Duration::from_millis(200))
        .expect("Kanal ok")
        .expect("Harus menerima pesanan 102 dari riwayat");
    assert_eq!(ev2.table, "ORDERS");
    assert_eq!(ev2.op, CdcOp::Insert);
    assert_eq!(ev2.after.unwrap().get("ID"), Some(&SqlValue::Integer(102)));

    // Tidak ada lagi event di replay buffer (event users dilewati filter)
    let ev_none = sub_orders.try_recv().unwrap();
    assert!(ev_none.is_none());

    // Sekarang lakukan live INSERT ke ORDERS
    db.execute("INSERT INTO orders (id, user_id, amount) VALUES (103, 1, 120000);").unwrap();

    // Harus langsung diterima secara real-time
    let ev_live = sub_orders.recv_timeout(Duration::from_millis(500))
        .expect("Kanal ok")
        .expect("Harus menerima event live pesanan 103");
    assert_eq!(ev_live.table, "ORDERS");
    assert_eq!(ev_live.after.unwrap().get("ID"), Some(&SqlValue::Integer(103)));

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_materialized_views_create_refresh_drop() {
    let test_db = "target/test_widyabase_matview.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // 1. Buat tabel transaksi dan isi baris awal
    db.execute("BUAT_TABEL transaksi (id INT PRIMARY KEY, kategori TEXT, nominal INT);").unwrap();
    db.execute("SISIP_KE transaksi (id, kategori, nominal) NILAI (1, 'Elektronik', 2500000);").unwrap();
    db.execute("SISIP_KE transaksi (id, kategori, nominal) NILAI (2, 'Buku', 150000);").unwrap();
    db.execute("SISIP_KE transaksi (id, kategori, nominal) NILAI (3, 'Elektronik', 1500000);").unwrap();

    // 2. Buat View Materialisasi dalam bahasa Indonesia
    db.execute("BUAT_VIEW_MATERIALISASI v_elektronik SEBAGAI PILIH id, nominal DARI transaksi DIMANA nominal >= 1000000;").unwrap();

    // 3. Kueri View Materialisasi secara langsung (performa cepat tanpa mengevaluasi ulang query induk)
    let res1 = db.execute("PILIH id, nominal DARI v_elektronik URUTKAN_BERDASARKAN nominal;").unwrap();
    match res1 {
        QueryResult::Rows { rows, .. } => {
            assert_eq!(rows.len(), 2, "Harus berisi 2 baris transaksi elektronik awal");
            assert_eq!(rows[0][1], SqlValue::Integer(1500000));
            assert_eq!(rows[1][1], SqlValue::Integer(2500000));
        }
        _ => panic!("Expected rows from materialized view"),
    }

    // 4. Sisipkan transaksi baru ke tabel asal yang memenuhi kriteria
    db.execute("SISIP_KE transaksi (id, kategori, nominal) NILAI (4, 'Elektronik', 5000000);").unwrap();

    // Pastikan View Materialisasi MASIH mengembalikan data cached (belum di-refresh)
    let res_stale = db.execute("PILIH id DARI v_elektronik;").unwrap();
    match res_stale {
        QueryResult::Rows { rows, .. } => {
            assert_eq!(rows.len(), 2, "View materialisasi harus tetap mempertahankan snapshot hingga di-refresh");
        }
        _ => panic!("Expected rows"),
    }

    // 5. Segarkan View Materialisasi (SEGAR_VIEW / REFRESH MATERIALIZED VIEW)
    db.execute("SEGAR_VIEW v_elektronik;").unwrap();

    // Sekarang View Materialisasi harus menyajikan data terbaru yang telah disegarkan
    let res_refreshed = db.execute("PILIH id, nominal DARI v_elektronik URUTKAN_BERDASARKAN nominal;").unwrap();
    match res_refreshed {
        QueryResult::Rows { rows, .. } => {
            assert_eq!(rows.len(), 3, "Harus berisi 3 baris setelah di-refresh");
            assert_eq!(rows[2][1], SqlValue::Integer(5000000));
        }
        _ => panic!("Expected rows"),
    }

    // 6. Buang View Materialisasi (BUANG_VIEW_MATERIALISASI / DROP MATERIALIZED VIEW)
    db.execute("BUANG_VIEW_MATERIALISASI v_elektronik;").unwrap();

    // Verifikasi view telah dihapus
    let res_err = db.execute("PILIH * DARI v_elektronik;");
    assert!(res_err.is_err(), "Kueri ke view yang telah dibuang harus menghasilkan galat");

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_cascade_foreign_keys_delete_and_update() {
    let test_db = "target/test_widyabase_cascade_fk.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");

    // 1. Buat tabel induk dan anak dengan aksi cascade
    db.execute("BUAT_TABEL departemen (id INT PRIMARY KEY, nama TEXT);").unwrap();
    db.execute("BUAT_TABEL pegawai (id INT PRIMARY KEY, nama TEXT, dept_id INT MERUJUK departemen(id) KETIKA HAPUS AIR_TERJUN KETIKA PERBARUI AIR_TERJUN);").unwrap();

    db.execute("SISIP_KE departemen (id, nama) NILAI (10, 'Teknologi Informasi');").unwrap();
    db.execute("SISIP_KE departemen (id, nama) NILAI (20, 'Keuangan');").unwrap();

    db.execute("SISIP_KE pegawai (id, nama, dept_id) NILAI (101, 'Aditya', 10);").unwrap();
    db.execute("SISIP_KE pegawai (id, nama, dept_id) NILAI (102, 'Bambang', 10);").unwrap();
    db.execute("SISIP_KE pegawai (id, nama, dept_id) NILAI (103, 'Citra', 20);").unwrap();

    // 2. Uji CASCADE UPDATE: ubah dept_id 10 -> 99 pada departemen
    db.execute("PERBARUI departemen ATUR id = 99 DIMANA id = 10;").unwrap();

    // Verifikasi pada pegawai 101 dan 102 otomatis berubah dept_id menjadi 99
    let res_peg = db.execute("PILIH id, dept_id DARI pegawai DIMANA dept_id = 99 URUTKAN_BERDASARKAN id;").unwrap();
    match res_peg {
        QueryResult::Rows { rows, .. } => {
            assert_eq!(rows.len(), 2, "Dua pegawai harus ter-cascade update ke dept_id 99");
            assert_eq!(rows[0][0], SqlValue::Integer(101));
            assert_eq!(rows[1][0], SqlValue::Integer(102));
        }
        _ => panic!("Expected rows"),
    }

    // 3. Uji CASCADE DELETE: hapus departemen id = 99
    db.execute("HAPUS_DARI departemen DIMANA id = 99;").unwrap();

    // Verifikasi pegawai dengan dept_id 99 otomatis terhapus (air terjun / cascade)
    let res_del_check = db.execute("PILIH id DARI pegawai DIMANA dept_id = 99;").unwrap();
    match res_del_check {
        QueryResult::Rows { rows, .. } => {
            assert_eq!(rows.len(), 0, "Pegawai di bawah departemen 99 harus terhapus otomatis");
        }
        _ => panic!("Expected rows"),
    }

    // Pegawai di departemen 20 (Citra) harus tetap ada
    let res_citra = db.execute("PILIH id, nama DARI pegawai DIMANA dept_id = 20;").unwrap();
    match res_citra {
        QueryResult::Rows { rows, .. } => {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0][1], SqlValue::Text("Citra".to_string()));
        }
        _ => panic!("Expected rows"),
    }

    // 4. Uji RESTRICT / NO ACTION (Default):
    // Buat tabel proyek yang merujuk departemen secara RESTRICT
    db.execute("BUAT_TABEL proyek (id INT PRIMARY KEY, nama TEXT, dept_id INT MERUJUK departemen(id) KETIKA HAPUS BATASI);").unwrap();
    db.execute("SISIP_KE proyek (id, nama, dept_id) NILAI (1, 'Sistem ERP', 20);").unwrap();

    // Hapus departemen 20 harus GAGAL karena ada baris anak aktif di 'proyek'
    let err_restrict = db.execute("HAPUS_DARI departemen DIMANA id = 20;");
    assert!(err_restrict.is_err(), "Penghapusan dengan relasi RESTRICT aktif harus ditolak");
    let err_msg = err_restrict.unwrap_err();
    assert!(err_msg.contains("Pelanggaran Kunci Asing"), "Pesan harus mengindikasikan pelanggaran foreign key: {}", err_msg);

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_pgwire_extended_protocol_and_binary_formats() {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use widya::widyabase::server::PgwireServer;

    let test_db = "target/test_widyabase_pgwire.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Gagal membuka db");
    db.execute("CREATE TABLE users (id INT PRIMARY KEY, name TEXT);").unwrap();
    db.execute("INSERT INTO users (id, name) VALUES (1, 'Widya');").unwrap();

    // Dapatkan port acak yang bebas
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let engine = Arc::clone(&db.engine);
    thread::spawn(move || {
        let server = PgwireServer::new(engine, port);
        let _ = server.start();
    });

    thread::sleep(Duration::from_millis(100));

    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("Koneksi pgwire harus berhasil");

    // 1. SSL Request (panjang 8, kode 80877103) -> Harus dibalas 'N'
    stream.write_all(&8i32.to_be_bytes()).unwrap();
    stream.write_all(&80877103i32.to_be_bytes()).unwrap();
    stream.flush().unwrap();

    let mut ssl_resp = [0u8; 1];
    stream.read_exact(&mut ssl_resp).unwrap();
    assert_eq!(ssl_resp[0], b'N');

    // 2. Startup Message
    let mut startup = Vec::new();
    startup.extend_from_slice(&196608i32.to_be_bytes()); // Protocol v3.0
    startup.extend_from_slice(b"user\0widya\0database\0testdb\0application_name\0DBeaver\0\0");
    let startup_len = (startup.len() + 4) as i32;
    stream.write_all(&startup_len.to_be_bytes()).unwrap();
    stream.write_all(&startup).unwrap();
    stream.flush().unwrap();

    // Baca AuthenticationOk ('R') + BackendKeyData ('K') + ParameterStatus ('S') + ReadyForQuery ('Z')
    let mut ready = false;
    let mut found_auth = false;
    let mut found_backend_key = false;

    while !ready {
        let mut msg_type = [0u8; 1];
        stream.read_exact(&mut msg_type).unwrap();
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).unwrap();
        let len = i32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; len - 4];
        stream.read_exact(&mut body).unwrap();

        match msg_type[0] {
            b'R' => found_auth = true,
            b'K' => found_backend_key = true,
            b'Z' => ready = true,
            _ => {}
        }
    }

    assert!(found_auth, "Harus menerima pesan AuthenticationOk ('R')");
    assert!(found_backend_key, "Harus menerima pesan BackendKeyData ('K')");

    // 3. Simple Query Protocol: SET dan SHOW (DBeaver / TablePlus connection handshake)
    let set_query = b"SET search_path = 'public'\0";
    let q_len = (set_query.len() + 4) as i32;
    stream.write_all(b"Q").unwrap();
    stream.write_all(&q_len.to_be_bytes()).unwrap();
    stream.write_all(set_query).unwrap();
    stream.flush().unwrap();

    // Baca respons sampai ReadyForQuery
    ready = false;
    let mut command_complete = false;
    while !ready {
        let mut msg_type = [0u8; 1];
        stream.read_exact(&mut msg_type).unwrap();
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).unwrap();
        let len = i32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; len - 4];
        stream.read_exact(&mut body).unwrap();

        if msg_type[0] == b'C' {
            command_complete = true;
            assert!(String::from_utf8_lossy(&body).contains("SET"));
        } else if msg_type[0] == b'Z' {
            ready = true;
        }
    }
    assert!(command_complete);

    // 4. Extended Query Protocol: Parse ('P') -> Bind ('B') -> Describe ('D') -> Execute ('E') -> Sync ('S')
    // Parse
    let mut parse_payload = Vec::new();
    parse_payload.extend_from_slice(b"s1\0"); // stmt name
    parse_payload.extend_from_slice(b"SELECT 42 AS angka;\0"); // query
    parse_payload.extend_from_slice(&0i16.to_be_bytes()); // 0 params
    let p_len = (parse_payload.len() + 4) as i32;
    stream.write_all(b"P").unwrap();
    stream.write_all(&p_len.to_be_bytes()).unwrap();
    stream.write_all(&parse_payload).unwrap();

    // Bind dengan binary result format (format code 1)
    let mut bind_payload = Vec::new();
    bind_payload.extend_from_slice(b"p1\0"); // portal name
    bind_payload.extend_from_slice(b"s1\0"); // stmt name
    bind_payload.extend_from_slice(&0i16.to_be_bytes()); // 0 param formats
    bind_payload.extend_from_slice(&0i16.to_be_bytes()); // 0 params
    bind_payload.extend_from_slice(&1i16.to_be_bytes()); // 1 result format
    bind_payload.extend_from_slice(&1i16.to_be_bytes()); // format code 1 (BINARY)
    let b_len = (bind_payload.len() + 4) as i32;
    stream.write_all(b"B").unwrap();
    stream.write_all(&b_len.to_be_bytes()).unwrap();
    stream.write_all(&bind_payload).unwrap();

    // Describe Portal
    let mut desc_payload = Vec::new();
    desc_payload.push(b'P');
    desc_payload.extend_from_slice(b"p1\0");
    let d_len = (desc_payload.len() + 4) as i32;
    stream.write_all(b"D").unwrap();
    stream.write_all(&d_len.to_be_bytes()).unwrap();
    stream.write_all(&desc_payload).unwrap();

    // Execute Portal
    let mut exec_payload = Vec::new();
    exec_payload.extend_from_slice(b"p1\0");
    exec_payload.extend_from_slice(&0i32.to_be_bytes()); // max_rows 0 (all)
    let e_len = (exec_payload.len() + 4) as i32;
    stream.write_all(b"E").unwrap();
    stream.write_all(&e_len.to_be_bytes()).unwrap();
    stream.write_all(&exec_payload).unwrap();

    // Sync
    stream.write_all(&[b'S', 0, 0, 0, 4]).unwrap();
    stream.flush().unwrap();

    // Baca respons: ParseComplete ('1'), BindComplete ('2'), RowDescription ('T'), DataRow ('D'), CommandComplete ('C'), ReadyForQuery ('Z')
    let mut got_parse_complete = false;
    let mut got_bind_complete = false;
    let mut got_row_desc = false;
    let mut got_data_row = false;
    ready = false;

    while !ready {
        let mut msg_type = [0u8; 1];
        stream.read_exact(&mut msg_type).unwrap();
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).unwrap();
        let len = i32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; len - 4];
        stream.read_exact(&mut body).unwrap();

        match msg_type[0] {
            b'1' => got_parse_complete = true,
            b'2' => got_bind_complete = true,
            b'T' => got_row_desc = true,
            b'D' => {
                got_data_row = true;
                // Verifikasi format DataRow binary
                assert!(body.len() >= 2);
            }
            b'Z' => ready = true,
            _ => {}
        }
    }

    assert!(got_parse_complete, "Harus menerima ParseComplete ('1')");
    assert!(got_bind_complete, "Harus menerima BindComplete ('2')");
    assert!(got_row_desc, "Harus menerima RowDescription ('T')");
    assert!(got_data_row, "Harus menerima DataRow ('D')");

    // Kirim Terminate ('X')
    let _ = stream.write_all(&[b'X', 0, 0, 0, 4]);

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_multinode_raft_automated_failover_and_wal_streaming() {
    use widya::widyabase::consensus::DistributedReplicationEngine;
    use widya::widyabase::wal::{WalOpType, WalRecord};

    // 1. Inisialisasi cluster 3-node
    let cluster = Arc::new(RaftCluster::new(vec![1, 2, 3]));

    // Node 1 terpilih sebagai Leader
    assert!(cluster.start_election_for(1), "Node 1 harus terpilih sebagai leader");
    assert_eq!(cluster.get_leader(), Some(1));

    let engine_node1 = DistributedReplicationEngine::new(1, Arc::clone(&cluster));
    assert!(engine_node1.is_leader());

    // 2. Replikasi WAL Record tunggal melalui Raft
    let wal_rec1 = WalRecord {
        lsn: 100,
        tx_id: 1,
        op_type: WalOpType::Insert,
        page_id: 1,
        slot_id: 0,
        payload: b"DATA_TX_01".to_vec(),
    };

    let (entry_idx, quorum_nodes) = cluster.replicate_wal_record(1, &wal_rec1).unwrap();
    assert_eq!(entry_idx, 1);
    assert!(quorum_nodes >= 2, "Replikasi WAL harus mencapai kuorum mayoritas");

    // 3. Streaming sekumpulan record WAL
    let mut wal_stream = Vec::new();
    for i in 1..=5 {
        wal_stream.push(WalRecord {
            lsn: 100 + i,
            tx_id: i,
            op_type: WalOpType::Insert,
            page_id: i as u32,
            slot_id: 0,
            payload: format!("STREAM_PAYLOAD_{}", i).into_bytes(),
        });
    }

    let last_replicated_idx = cluster.replicate_wal_stream(1, &wal_stream).unwrap();
    assert_eq!(last_replicated_idx, 6);

    // 4. Simulasi Automated Failover:
    // Node 1 (Leader) mengalami crash / partisi jaringan
    cluster.isolate_node(1);
    assert_eq!(cluster.get_leader(), None, "Leader 1 harus tidak terdeteksi setelah terisolasi");

    // Pemicu failover otomatis
    let new_leader_id = cluster.trigger_automated_failover().expect("Failover otomatis harus berhasil");
    assert!(new_leader_id == 2 || new_leader_id == 3, "Leader baru harus Node 2 atau Node 3");
    assert_eq!(cluster.get_leader(), Some(new_leader_id));

    // Leader baru mampu melanjutkan transaksi tulis terdistribusi
    let wal_rec_failover = WalRecord {
        lsn: 200,
        tx_id: 99,
        op_type: WalOpType::Insert,
        page_id: 10,
        slot_id: 0,
        payload: b"POST_FAILOVER_WRITE".to_vec(),
    };
    let (fo_idx, fo_quorum) = cluster.replicate_wal_record(new_leader_id, &wal_rec_failover).unwrap();
    assert!(fo_idx > 6);
    assert!(fo_quorum >= 2);

    // 5. Catch-up Replication: Node 1 pulih kembali dan mengejar ketertinggalan log
    cluster.reconnect_node(1);
    let caught_up = cluster.catch_up_follower_wal(new_leader_id, 1, 1).unwrap();
    assert!(caught_up > 0, "Node 1 harus menyinkronkan seluruh entri WAL yang tertinggal");
}

#[test]
fn test_widyabase_ai_vector_hnsw_metadata_filtering_and_local_onnx_rag() {
    use widya::widyabase::hnsw::{HnswVectorIndex, MetadataFilter};
    use widya::widyabase::hybrid_search::{HybridSearchEngine, LocalEmbeddingEngine};

    // 1. Uji LocalEmbeddingEngine In-Database
    let embedder = LocalEmbeddingEngine::new();
    assert_eq!(embedder.dimension, 384);

    let v1 = embedder.embed("Sistem pembayaran QRIS transfer dana perbankan");
    let v2 = embedder.embed("Transaksi pembayaran elektronik QR code instan");
    let v3 = embedder.embed("Resep masakan rendang daging sapi khas Padang");

    assert_eq!(v1.len(), 384);
    assert_eq!(v2.len(), 384);
    assert_eq!(v3.len(), 384);

    // Hitung similaritas kosinus semantik: v1 & v2 harus berkorelasi tinggi, v1 & v3 harus rendah
    let sim_fintech = embedder.semantic_similarity(
        "Sistem pembayaran QRIS transfer dana perbankan",
        "Transaksi pembayaran elektronik QR code instan",
    );
    let sim_food = embedder.semantic_similarity(
        "Sistem pembayaran QRIS transfer dana perbankan",
        "Resep masakan rendang daging sapi khas Padang",
    );

    assert!(sim_fintech > sim_food, "Similaritas semantik topik sejenis ({:.3}) harus lebih tinggi dari topik berbeda ({:.3})", sim_fintech, sim_food);

    // 2. Uji Indeks HNSW dengan Metadata Filtering (Pre-filtering & Post-filtering)
    let mut hnsw = HnswVectorIndex::new(16, 32);

    let mut meta_a = HashMap::new();
    meta_a.insert("tenant_id".to_string(), "CORP_ALPHA".to_string());
    meta_a.insert("category".to_string(), "fintech".to_string());
    meta_a.insert("level".to_string(), "5".to_string());

    let mut meta_b = HashMap::new();
    meta_b.insert("tenant_id".to_string(), "CORP_BETA".to_string());
    meta_b.insert("category".to_string(), "fintech".to_string());
    meta_b.insert("level".to_string(), "1".to_string());

    let mut meta_c = HashMap::new();
    meta_c.insert("tenant_id".to_string(), "CORP_ALPHA".to_string());
    meta_c.insert("category".to_string(), "medical".to_string());
    meta_c.insert("level".to_string(), "9".to_string());

    hnsw.insert_with_metadata("DOC-FIN-A", &v1, meta_a);
    hnsw.insert_with_metadata("DOC-FIN-B", &v2, meta_b);
    hnsw.insert_with_metadata("DOC-MED-A", &v3, meta_c);

    // Pre-filtering: Hanya ambil dokumen dari CORP_ALPHA
    let filter_alpha = MetadataFilter::Exact("tenant_id".to_string(), "CORP_ALPHA".to_string());
    let res_alpha = hnsw.search_knn_prefiltered(&v1, 5, 16, &filter_alpha);
    assert_eq!(res_alpha.len(), 2);
    for (id, _) in &res_alpha {
        assert!(id == "DOC-FIN-A" || id == "DOC-MED-A");
    }

    // Filter gabungan AND: category = 'fintech' DAN level > 2
    let filter_complex = MetadataFilter::And(vec![
        MetadataFilter::Exact("category".to_string(), "fintech".to_string()),
        MetadataFilter::GreaterThan("level".to_string(), 2.0),
    ]);
    let res_complex = hnsw.search_knn_filtered(&v1, 5, 16, &filter_complex);
    assert_eq!(res_complex.len(), 1);
    assert_eq!(res_complex[0].0, "DOC-FIN-A");

    // 3. Uji HybridSearchEngine RAG Terpadu
    let mut rag_engine = HybridSearchEngine::new();

    let mut meta1 = HashMap::new();
    meta1.insert("department".to_string(), "finance".to_string());
    rag_engine.index_text("DOC-01", "Infrastruktur Gateway Pembayaran QRIS BI-FAST Nasional", meta1);

    let mut meta2 = HashMap::new();
    meta2.insert("department".to_string(), "healthcare".to_string());
    rag_engine.index_text("DOC-02", "Sistem Telemedicine Terintegrasi SatuSehat Kemenkes", meta2);

    let filter_dept = MetadataFilter::Exact("department".to_string(), "finance".to_string());
    let rag_results = rag_engine.search_rag("pembayaran qris nasional", 2, Some(&filter_dept));

    assert_eq!(rag_results.len(), 1);
    assert_eq!(rag_results[0].document.id, "DOC-01");
    assert!(rag_results[0].rrf_score > 0.0);
    assert!(rag_results[0].vector_score > 0.0);
}

#[test]
fn test_widyabase_enterprise_six_pillars_integration() {
    use widya::widyabase::{
        TimeSeriesRecord, TimeBucketAgg,
        Point2D, Polygon2D, Geometry,
        TriggerTiming, TriggerEvent, TriggerAction, TriggerDefinition, TriggerLevel,
    };
    use std::collections::HashMap;

    let test_db = "test_enterprise_pillars.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Harus berhasil membuka instance WidyaBase");

    // ========================================================================
    // PILAR 1: TIME-SERIES HYPERTABLE & GORILLA COMPRESSION
    // ========================================================================
    let ts = db.timeseries();
    let ht = ts.create_hypertable("iot_telemetry", 60_000, Some(86400_000));
    let base_t = 1200_000u64;

    for i in 0..120 {
        let t = base_t + (i * 1000);
        ht.insert(TimeSeriesRecord {
            timestamp_ms: t,
            series_key: "cpu_usage".to_string(),
            value_numeric: 20.0 + (i as f64 * 0.5),
            tags: HashMap::new(),
        }).unwrap();
    }

    let records = ht.query_range(Some("cpu_usage"), base_t, base_t + 120_000);
    assert_eq!(records.len(), 120);

    let buckets = ht.time_bucket(30_000, "cpu_usage", base_t, base_t + 120_000, TimeBucketAgg::Avg);
    assert_eq!(buckets.len(), 4);
    assert!(buckets[0].aggregated_value < buckets[1].aggregated_value);

    let ohlcv = ht.calculate_ohlcv(60_000, "cpu_usage", base_t, base_t + 120_000);
    assert_eq!(ohlcv.len(), 2);
    assert_eq!(ohlcv[0].open, 20.0);

    let compressed = ht.compress_historical_chunks(0);
    assert_eq!(compressed, 2);

    // ========================================================================
    // PILAR 2: INDONESIAN STEMMER & FUZZY FULL-TEXT SEARCH
    // ========================================================================
    {
        let ft_engine = db.fulltext();
        let mut ft = ft_engine.write().unwrap();
        ft.add_document("doc-1", "Kementerian Keuangan melakukan pembayaran subsidi energi nasional.");
        ft.add_document("doc-2", "Guru pengajar mendidik anak-anak dengan modul pelajaran informatika modern.");
        ft.add_document("doc-3", "Perusahaan teknologi menyediakan layanan transaksi perbankan digital.");

        // Pencarian dengan stemming imbuhan (Nazief-Adriani)
        let res_bayar = ft.search("membayar", 1, 5);
        assert!(!res_bayar.is_empty());
        assert_eq!(res_bayar[0].document_id, "doc-1");

        let res_ajar = ft.search("mengajar", 1, 5);
        assert!(!res_ajar.is_empty());
        assert_eq!(res_ajar[0].document_id, "doc-2");

        // Autocomplete berbasis edge n-grams
        let autocomp = ft.autocomplete("pembay", 5);
        assert!(autocomp.iter().any(|w| w.contains("pembayaran")));

        // Typo tolerance (Damerau-Levenshtein distance)
        let res_typo = ft.search("pembayran", 1, 5);
        assert!(!res_typo.is_empty());
        assert_eq!(res_typo[0].document_id, "doc-1");
    }

    // ========================================================================
    // PILAR 3: GEOSPATIAL GIS & R-TREE SPATIAL INDEX
    // ========================================================================
    let gis = db.gis();
    // Koordinat Jakarta: Monas (-6.1754, 106.8272), Bundaran HI (-6.1950, 106.8230)
    let monas = Point2D::new(106.8272, -6.1754);
    let bundaran_hi = Point2D::new(106.8230, -6.1950);
    let dist_m = widya::widyabase::st_distance_sphere(&monas, &bundaran_hi);
    assert!(dist_m > 2000.0 && dist_m < 2400.0, "Jarak Monas ke HI sekitar 2.2 km, didapat: {}", dist_m);

    gis.add_feature("poi_monas", Geometry::Point(monas.clone()), HashMap::new());
    gis.add_feature("poi_hi", Geometry::Point(bundaran_hi.clone()), HashMap::new());

    let jakarta_pusat = Polygon2D::new(vec![
        Point2D::new(106.80, -6.22),
        Point2D::new(106.86, -6.22),
        Point2D::new(106.86, -6.16),
        Point2D::new(106.80, -6.16),
    ]);
    assert!(widya::widyabase::st_contains(&jakarta_pusat, &monas));
    assert!(widya::widyabase::st_contains(&jakarta_pusat, &bundaran_hi));

    let nearby = gis.query_within_radius(&monas, 3000.0);
    assert_eq!(nearby.len(), 2, "Monas dan HI harus berada dalam radius 3km dari Monas");

    // ========================================================================
    // PILAR 4: PROPERTY GRAPH & CYPHER TRAVERSAL
    // ========================================================================
    let graph = db.graph();
    let mut props_alice = HashMap::new();
    props_alice.insert("name".to_string(), serde_json::json!("Alice"));
    let alice = graph.create_node(&["User", "Admin"], props_alice);

    let mut props_bob = HashMap::new();
    props_bob.insert("name".to_string(), serde_json::json!("Bob"));
    let bob = graph.create_node(&["User"], props_bob);

    let mut props_server = HashMap::new();
    props_server.insert("name".to_string(), serde_json::json!("Server-1"));
    let srv = graph.create_node(&["Resource"], props_server);

    let _ = graph.create_edge(alice, bob, "FRIENDS_WITH", 1.0, HashMap::new());
    let _ = graph.create_edge(bob, srv, "CAN_ACCESS", 2.0, HashMap::new());

    let bfs = graph.bfs_traverse(alice, 2);
    assert_eq!(bfs.len(), 3);

    let sp = graph.shortest_path(alice, srv).expect("Harus ada jalur Alice -> Bob -> Server");
    assert_eq!(sp.total_cost, 3.0);
    assert_eq!(sp.node_path, vec![alice, bob, srv]);

    let pr = graph.calculate_pagerank(20, 0.85);
    assert_eq!(pr.len(), 3);
    assert!(pr[&srv] > 0.0);

    // ========================================================================
    // PILAR 5: NATIVE TRIGGERS & AUDIT TRAIL LOG
    // ========================================================================
    let triggers = db.triggers();
    triggers.create_trigger(TriggerDefinition {
        name: "trg_check_gaji_positif".to_string(),
        table_name: "KARYAWAN".to_string(),
        timing: TriggerTiming::Before,
        event: TriggerEvent::Insert,
        level: TriggerLevel::Row,
        action: TriggerAction::ValidateCondition {
            column_name: "gaji".to_string(),
            rule_description: "Gaji minimal 3000000".to_string(),
            min_numeric: Some(3000000.0),
            max_numeric: None,
            not_empty: true,
        },
        is_enabled: true,
    });

    triggers.create_trigger(TriggerDefinition {
        name: "trg_audit_karyawan".to_string(),
        table_name: "KARYAWAN".to_string(),
        timing: TriggerTiming::After,
        event: TriggerEvent::Insert,
        level: TriggerLevel::Row,
        action: TriggerAction::LogAuditTrail {
            actor: "hr_admin".to_string(),
        },
        is_enabled: true,
    });

    // Validasi penolakan
    let mut invalid_row = HashMap::new();
    invalid_row.insert("nama".to_string(), "Budi".to_string());
    invalid_row.insert("gaji".to_string(), "1500000".to_string()); // di bawah batas
    let res_invalid = triggers.fire_row_triggers("KARYAWAN", TriggerTiming::Before, TriggerEvent::Insert, None, Some(&mut invalid_row));
    assert!(res_invalid.is_err(), "Trigger harus menolak gaji di bawah 3 juta");

    // Validasi penerimaan & audit trail
    let mut valid_row = HashMap::new();
    valid_row.insert("nama".to_string(), "Siti".to_string());
    valid_row.insert("gaji".to_string(), "7500000".to_string());
    let res_valid = triggers.fire_row_triggers("KARYAWAN", TriggerTiming::Before, TriggerEvent::Insert, None, Some(&mut valid_row));
    assert!(res_valid.is_ok());

    let res_after = triggers.fire_row_triggers("KARYAWAN", TriggerTiming::After, TriggerEvent::Insert, None, Some(&mut valid_row));
    assert!(res_after.is_ok());

    let audits = triggers.get_audit_trail(Some("KARYAWAN"));
    assert_eq!(audits.len(), 1);
    assert_eq!(audits[0].actor, "hr_admin");
    assert_eq!(audits[0].operation, "INSERT");

    // ========================================================================
    // PILAR 6: ZERO-ETL HTAP ROUTER & SIMD VECTORIZED COLUMNAR
    // ========================================================================
    let htap = db.htap();
    htap.register_table("transaksi_qris");

    for i in 1..=100 {
        let mut row = HashMap::new();
        row.insert("id".to_string(), i.to_string());
        row.insert("nominal".to_string(), (i * 1000).to_string());
        htap.stream_zero_etl_mutation("transaksi_qris", &row);
    }
    assert_eq!(htap.total_ingested_rows(), 100);

    let plan_olap = htap.analyze_and_route_query("SELECT SUM(nominal), AVG(nominal) FROM transaksi_qris GROUP BY merchant_id");
    assert_eq!(plan_olap.target_engine, "COLUMNAR_SIMD_OLAP");

    let plan_oltp = htap.analyze_and_route_query("SELECT * FROM transaksi_qris WHERE id = 42 LIMIT 1");
    assert_eq!(plan_oltp.target_engine, "ROW_STORE_OLTP");

    let sum_res = htap.execute_columnar_aggregate("transaksi_qris", "nominal", "SUM");
    assert_eq!(sum_res, Ok(5050000.0)); // 1000 * (100 * 101 / 2) = 5,050,000

    clean_test_file(test_db);
}

#[test]
fn test_widyabase_arabic_rtl_text_storage_and_queries() {
    let test_db = "test_arabic_rtl.wdb";
    clean_test_file(test_db);

    let db = WidyaBase::open(test_db).expect("Harus berhasil membuka instance WidyaBase");

    // 1. DDL: Buat tabel untuk koleksi hadis dan teks Arab
    db.execute("CREATE TABLE hadis (id INT, perawi TEXT, matan TEXT, makna TEXT);")
        .expect("Gagal membuat tabel hadis");

    // 2. DML: Sisipkan data teks bahasa Arab lengkap dengan harakat dan tanda baca
    db.execute(
        "INSERT INTO hadis VALUES (1, 'البخاري', 'إِنَّمَا الأَعْمَالُ بِالنِّيَّاتِ', 'Sesungguhnya amal itu tergantung niatnya');"
    ).expect("Gagal insert hadis 1");

    db.execute(
        "INSERT INTO hadis VALUES (2, 'مسلم', 'الدِّينُ النَّصِيحَةُ', 'Agama itu adalah nasihat');"
    ).expect("Gagal insert hadis 2");

    db.execute(
        "INSERT INTO hadis VALUES (3, 'الترمذي', 'لاَ يُؤْمِنُ أَحَدُكُمْ حَتَّى يُحِبَّ لأَخِيهِ مَا يُحِبُّ لِنَفْسِهِ', 'Tidak beriman salah seorang di antara kalian hingga mencintai saudaranya seperti mencintai dirinya sendiri');"
    ).expect("Gagal insert hadis 3");

    // 3. Kueri SELECT dengan filter pencocokan tepat (Exact Match)
    let res_exact = db.execute("SELECT * FROM hadis WHERE perawi = 'البخاري';")
        .expect("Kueri exact match perawi gagal");
    if let QueryResult::Rows { rows, .. } = res_exact {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][1], SqlValue::Text("البخاري".to_string()));
        assert_eq!(rows[0][2], SqlValue::Text("إِنَّمَا الأَعْمَالُ بِالنِّيَّاتِ".to_string()));
    } else {
        panic!("Hasil kueri bukan Rows");
    }

    // 4. Kueri SELECT dengan pola LIKE wildcard pada teks Arab
    let res_like = db.execute("SELECT * FROM hadis WHERE matan LIKE '%الأَعْمَالُ%';")
        .expect("Kueri LIKE teks Arab gagal");
    if let QueryResult::Rows { rows, .. } = res_like {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], SqlValue::Integer(1));
    } else {
        panic!("Hasil kueri bukan Rows");
    }

    // 5. Uji fungsi skalar SQL khusus RTL & Bahasa Arab
    let res_fn = db.execute("SELECT ARAH_TEKS(perawi), HAPUS_HARAKAT(matan), BALIK_TEKS_VISUAL(perawi), TEKS_KE_RTL(perawi) FROM hadis WHERE id = 1;")
        .expect("Kueri fungsi skalar RTL gagal");
    if let QueryResult::Rows { rows, .. } = res_fn {
        assert_eq!(rows.len(), 1);
        // ARAH_TEKS('البخاري') -> 'rtl'
        assert_eq!(rows[0][0], SqlValue::Text("rtl".to_string()));
        // HAPUS_HARAKAT('إِنَّمَا الأَعْمَالُ بِالنِّيَّاتِ') -> harakat dihilangkan
        let matan_gundul = rows[0][1].as_text();
        assert!(!matan_gundul.contains('\u{064E}')); // Fathah
        assert!(!matan_gundul.contains('\u{0650}')); // Kasrah
        assert!(!matan_gundul.contains('\u{0651}')); // Tasydid
        // BALIK_TEKS_VISUAL: karakter dibalik urutannya untuk terminal non-BiDi
        let rev_perawi = rows[0][2].as_text();
        let expected_rev: String = "البخاري".chars().rev().collect();
        assert_eq!(rev_perawi, expected_rev);
        // TEKS_KE_RTL: dibungkus embedding U+202B .. U+202C
        let rtl_wrapped = rows[0][3].as_text();
        assert!(rtl_wrapped.starts_with('\u{202B}'));
        assert!(rtl_wrapped.ends_with('\u{202C}'));
    } else {
        panic!("Hasil kueri bukan Rows");
    }

    // 6. Uji penyimpanan pada FullTextIndex
    {
        let ft_engine = db.fulltext();
        let mut ft = ft_engine.write().unwrap();
        ft.add_document("doc-ar-1", "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ");
        ft.add_document("doc-ar-2", "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ");

        let search_res = ft.search("الرَّحْمَٰنِ", 0, 5);
        assert!(!search_res.is_empty());
        assert_eq!(search_res[0].document_id, "doc-ar-1");
    }

    clean_test_file(test_db);
}



