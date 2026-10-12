// ============================================================================
// INTEGRATION TEST: WIDYACODER LOCAL RAG ENGINE (tests/widyacoder_rag_test.rs)
// ============================================================================

use std::fs;
use std::path::Path;
use widya::ai::widyacoder::{KnowledgeBase, WidyaCoderEngine};

#[test]
fn test_widyacoder_build_search_and_prompt() {
    let mut engine = WidyaCoderEngine::new();
    let root = Path::new(".");
    let res = engine.build_from_workspace(root);
    assert!(res.is_ok(), "Build from workspace harus sukses");

    // Pastikan terindeks ratusan atau ribuan potongan pengetahuan
    assert!(
        engine.kb.total_chunks > 100,
        "Total chunks harus lebih dari 100, ditemukan: {}",
        engine.kb.total_chunks
    );

    // 1. Kueri WidyaBase
    let matches_db = engine.search("widyabase slotted page buffer pool mvcc", 3);
    assert!(!matches_db.is_empty(), "Pencarian widyabase harus mengembalikan hasil");
    let sources_db: Vec<String> = matches_db.iter().map(|m| m.chunk.source_path.clone()).collect();
    assert!(
        sources_db.iter().any(|s| s.contains("WIDYABASE") || s.contains("wya")),
        "Hasil harus merujuk ke dokumentasi atau kode WidyaBase"
    );

    // 2. Kueri Web Server / WidyaWeb
    let matches_web = engine.search("widyaweb http server route handler", 3);
    assert!(!matches_web.is_empty(), "Pencarian widyaweb harus mengembalikan hasil");

    // 3. Kueri Kripto
    let matches_kripto = engine.search("enkripsi AES ed25519 hash argon2", 3);
    assert!(!matches_kripto.is_empty(), "Pencarian kripto harus mengembalikan hasil");

    // 4. Periksa Penyusunan Prompt
    let (system_prompt, user_prompt) = engine.format_prompt("Buatkan server HTTP sederhana di Widya", &matches_web);
    assert!(system_prompt.contains("WidyaCoder"));
    assert!(system_prompt.contains("RAG CONTEXT"));
    assert_eq!(user_prompt, "Buatkan server HTTP sederhana di Widya");
}

#[test]
fn test_widyacoder_kb_persistence() {
    let tmp_path = Path::new("target/test_kb_tmp.json");
    let mut kb = KnowledgeBase::new();

    let embedder = widya::widyabase::hybrid_search::LocalEmbeddingEngine::new();
    let mut id_counter = 0;
    let chunks = WidyaCoderEngine::chunk_markdown(
        "# Uji Coba Widya\nIni adalah teks uji coba untuk persistensi.",
        "DOK_TEST.md",
        &mut id_counter,
        &embedder,
    );

    kb.total_chunks = chunks.len();
    kb.chunks = chunks;

    // Simpan ke disk
    assert!(kb.save_to_file(tmp_path).is_ok());
    assert!(tmp_path.exists());

    // Muat kembali
    let loaded = KnowledgeBase::load_from_file(tmp_path);
    assert!(loaded.is_ok());
    let loaded_kb = loaded.unwrap();
    assert_eq!(loaded_kb.total_chunks, kb.total_chunks);
    assert_eq!(loaded_kb.chunks[0].title, kb.chunks[0].title);

    let _ = fs::remove_file(tmp_path);
}
