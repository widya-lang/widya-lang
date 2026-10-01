use widya::compiler::decimal::Desimal;
use widya::widyabase::btree::{BTreeIndex, IndexKey, RecordPointer};
use widya::widyabase::hybrid_search::{HybridDocument, HybridSearchEngine};
use std::collections::HashMap;

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
