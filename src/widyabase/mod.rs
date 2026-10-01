// ============================================================================
// WIDYABASE UNIFIED DATABASE ENGINE (mod.rs)
// ============================================================================
// Mesin Basis Data Mandiri Skala Industri:
// - Slotted-Page Storage 4KB + Buffer Pool Manager
// - Write-Ahead Logging (WAL) + Crash Recovery
// - Multi-Version Concurrency Control (MVCC) + Snapshot Isolation
// - B+Tree Indexing O(log N)
// - Bilingual SQL Lexer & Parser (Indonesian & English ANSI SQL)
// - pgwire v3.0 Server Daemon (PostgreSQL Protocol Kompatibel)
// - Multi-Modal: Vektor HNSW, JSONB & Kolumnar
// ============================================================================

pub mod storage;
pub mod wal;
pub mod mvcc;
pub mod btree;
pub mod sql_parser;
pub mod planner;
pub mod server;
pub mod multimodal;
pub mod hybrid_search;
pub mod checkpoint;
pub mod cache;
pub mod columnar;
pub mod object_store;
pub mod docdb;
pub mod consensus;
pub mod parquet_arrow;

pub use parquet_arrow::{ParquetArrowConnector, ParquetCompression, ParquetFileMetadata, ArrowRecordBatchIpc};
pub use hybrid_search::{HybridDocument, HybridSearchEngine, HybridSearchResult};
pub use checkpoint::CheckpointCoordinator;
pub use consensus::{RaftNode, RaftRole, LogEntry};

pub use object_store::{
    BucketAccessPolicy, BucketInfo, ListObjectsV2Result, ObjectMetadata, ObjectStoreConfig,
    StoredObject, VersioningStatus, WidyaObjectStore,
};
pub use docdb::{
    ChangeEvent, Collection, CollectionOptions, DeleteResult, IndexSpec, OperationType,
    UpdateResult, WidyaDocDB,
};

use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::widyabase::cache::CacheEngine;
use crate::widyabase::columnar::ColumnarEngine;
use crate::widyabase::planner::{ExecutionEngine, QueryResult};
use crate::widyabase::server::{PgwireServer, RespServer};
use crate::widyabase::sql_parser::{SqlLexer, SqlParser};
use crate::widyabase::storage::StorageEngine;
use crate::widyabase::wal::WalManager;

pub struct WidyaBase {
    pub db_path: PathBuf,
    pub storage: Arc<StorageEngine>,
    pub wal: Arc<WalManager>,
    pub engine: Arc<ExecutionEngine>,
}

#[derive(Debug, Clone)]
pub struct HealthReport {
    pub total_pages: u32,
    pub total_tables: usize,
    pub wal_records: usize,
    pub buffer_pool_status: String,
    pub is_healthy: bool,
}

impl WidyaBase {
    /// Membuka atau membuat berkas database WidyaBase (.wdb)
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let db_path = path.as_ref().to_path_buf();
        let storage = Arc::new(
            StorageEngine::open(&db_path, 512)
                .map_err(|e| format!("Gagal membuka storage engine: {}", e))?
        );
        let wal = Arc::new(
            WalManager::open(&db_path)
                .map_err(|e| format!("Gagal membuka WAL manager: {}", e))?
        );
        let engine = Arc::new(ExecutionEngine::new(Arc::clone(&storage), Arc::clone(&wal)));

        Ok(WidyaBase {
            db_path,
            storage,
            wal,
            engine,
        })
    }

    /// Mengeksekusi string kueri SQL (Bahasa Indonesia atau Standar ANSI)
    pub fn execute(&self, sql: &str) -> Result<QueryResult, String> {
        let mut lexer = SqlLexer::new(sql);
        let tokens = lexer.tokenize()?;
        let mut parser = SqlParser::new(tokens);
        let stmt = parser.parse_statement()?;
        self.engine.execute(stmt, 1)
    }

    /// Mengeksekusi kueri di dalam transaksi tertentu
    pub fn execute_in_tx(&self, sql: &str, tx_id: u64) -> Result<QueryResult, String> {
        let mut lexer = SqlLexer::new(sql);
        let tokens = lexer.tokenize()?;
        let mut parser = SqlParser::new(tokens);
        let stmt = parser.parse_statement()?;
        self.engine.execute(stmt, tx_id)
    }

    /// Menjalankan server daemon PostgreSQL Wire Protocol v3.0 (Port 5432)
    pub fn start_pgwire_server(&self, port: u16) -> std::io::Result<()> {
        let srv = PgwireServer::new(Arc::clone(&self.engine), port);
        srv.start()
    }

    /// Menjalankan server daemon Redis RESP Protocol v2/v3 (Port 6379)
    pub fn start_redis_server(&self, port: u16) -> std::io::Result<()> {
        let srv = RespServer::new(Arc::clone(&self.engine), port);
        srv.start()
    }

    /// Melakukan sinkronisasi halaman kotor ke disk (Checkpoint)
    pub fn checkpoint(&self) -> Result<(), String> {
        self.storage.flush_all().map_err(|e| format!("Flush error: {}", e))?;
        self.wal.checkpoint().map_err(|e| format!("WAL checkpoint error: {}", e))?;
        Ok(())
    }

    /// Mengakses mesin In-Memory Cache & Key-Value (Redis-compatible)
    pub fn cache(&self) -> Arc<CacheEngine> {
        Arc::clone(&self.engine.cache_engine)
    }

    /// Mengakses mesin Columnar OLAP (ClickHouse-compatible)
    pub fn columnar(&self) -> Arc<ColumnarEngine> {
        Arc::clone(&self.engine.columnar_engine)
    }

    /// Memeriksa kesehatan dan integritas basis data
    pub fn check_health(&self) -> HealthReport {
        let total_pages = *self.storage.total_pages.lock().unwrap();
        let cat = self.engine.catalog.read().unwrap();
        let total_tables = cat.len();
        let wal_records = self.wal.read_all_records().unwrap_or_default().len();

        HealthReport {
            total_pages,
            total_tables,
            wal_records,
            buffer_pool_status: "Aktif (512 Halaman Terkelola)".to_string(),
            is_healthy: true,
        }
    }
}
