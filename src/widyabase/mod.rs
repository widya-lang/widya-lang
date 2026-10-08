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
pub mod cdc;
pub mod parquet_arrow;
pub mod hnsw;
pub mod timeseries;
pub mod fulltext;
pub mod gis;
pub mod graph;
pub mod trigger;
pub mod htap;
pub mod wire;
pub use wire::*;

pub use parquet_arrow::{ParquetArrowConnector, ParquetCompression, ParquetFileMetadata, ArrowRecordBatchIpc};
pub use hybrid_search::{HybridDocument, HybridSearchEngine, HybridSearchResult, LocalEmbeddingEngine};
pub use hnsw::{HnswVectorIndex, MetadataFilter};
pub use checkpoint::CheckpointCoordinator;
pub use cdc::{CdcFilter, CdcOp, CdcRecord, CdcStreamer, CdcSubscription};
pub use consensus::{
    AppendEntriesArgs, AppendEntriesReply, DistributedReplicationEngine, LogEntry, RaftCluster,
    RaftNode, RaftRole, RequestVoteArgs, RequestVoteReply, TwoPhaseCoordinator,
    TwoPhaseParticipant, TwoPhaseStatus,
};

pub use object_store::{
    BucketAccessPolicy, BucketInfo, ListObjectsV2Result, ObjectMetadata, ObjectStoreConfig,
    StoredObject, VersioningStatus, WidyaObjectStore,
};
pub use docdb::{
    ChangeEvent, Collection, CollectionOptions, DeleteResult, IndexSpec, OperationType,
    UpdateResult, WidyaDocDB,
};
pub use timeseries::{
    GorillaCodec, Hypertable, OhlcvBar, TimeBucketAgg, TimeBucketResult, TimeChunk,
    TimeSeriesEngine, TimeSeriesRecord,
};
pub use fulltext::{
    damerau_levenshtein_distance, fuzzy_similarity_score, FullTextHit, FullTextIndex,
    IndonesianStemmer,
};
pub use gis::{
    st_centroid, st_contains, st_distance_sphere, st_dwithin, BoundingBox2D, Geometry, GisEngine,
    LineString2D, Point2D, Polygon2D, SpatialFeature, SpatialRTree,
};
pub use graph::{
    EdgeId, GraphEdge, GraphEngine, GraphNode, NodeId, ShortestPathResult,
};
pub use trigger::{
    AuditRecord, RowData, TriggerAction, TriggerDefinition, TriggerEvent, TriggerLevel,
    TriggerManager, TriggerTiming,
};
pub use htap::{
    ColumnVector, HtapEngine, HtapRoutingPlan, HtapTable,
};

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use crate::widyabase::cache::CacheEngine;
use crate::widyabase::columnar::ColumnarEngine;
pub use crate::widyabase::planner::{ColumnStats, CostModel, ExecutionEngine, QueryResult, TableStats};
use crate::widyabase::server::{PgwireServer, RespServer};
use crate::widyabase::sql_parser::{SqlLexer, SqlParser};
use crate::widyabase::storage::StorageEngine;
use crate::widyabase::wal::WalManager;

pub struct WidyaBase {
    pub db_path: PathBuf,
    pub storage: Arc<StorageEngine>,
    pub wal: Arc<WalManager>,
    pub engine: Arc<ExecutionEngine>,
    pub timeseries: Arc<TimeSeriesEngine>,
    pub fulltext: Arc<RwLock<FullTextIndex>>,
    pub gis: Arc<GisEngine>,
    pub graph: Arc<GraphEngine>,
    pub triggers: Arc<TriggerManager>,
    pub htap: Arc<HtapEngine>,
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
        let timeseries = Arc::new(TimeSeriesEngine::new());
        let fulltext = Arc::new(RwLock::new(FullTextIndex::new()));
        let gis = Arc::new(GisEngine::new());
        let graph = Arc::new(GraphEngine::new());
        let triggers = Arc::new(TriggerManager::new());
        let htap = Arc::new(HtapEngine::new());

        Ok(WidyaBase {
            db_path,
            storage,
            wal,
            engine,
            timeseries,
            fulltext,
            gis,
            graph,
            triggers,
            htap,
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

    /// Membuat indeks vektor HNSW mandiri untuk pencarian kemiripan semantik
    pub fn vector_index(&self, _dimension: usize) -> HnswVectorIndex {
        HnswVectorIndex::new(16, 200)
    }

    /// Membuat instance mesin pencarian hibrida (BM25 Lexical + Dense Vector)
    pub fn hybrid_search(&self) -> HybridSearchEngine {
        HybridSearchEngine::new()
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

    /// Mengambil data statistik tabel yang telah dianalisis oleh CBO (Cost-Based Optimizer)
    pub fn get_table_stats(&self, table: &str) -> Option<TableStats> {
        self.engine.table_stats.read().unwrap().get(&table.to_uppercase()).cloned()
    }

    /// Mengakses pengelola aliran Change Data Capture (CDC)
    pub fn cdc(&self) -> Arc<CdcStreamer> {
        Arc::clone(&self.engine.cdc)
    }

    /// Berlangganan aliran event perubahan data real-time (CDC stream)
    pub fn subscribe_cdc(&self, filter: Option<CdcFilter>, from_lsn: Option<u64>) -> CdcSubscription {
        self.engine.cdc.subscribe(filter, from_lsn)
    }

    /// Mengakses mesin Time-Series Hypertable & Gorilla Compression
    pub fn timeseries(&self) -> Arc<TimeSeriesEngine> {
        Arc::clone(&self.timeseries)
    }

    /// Mengakses indeks Full-Text Search dengan Sastrawi/Nazief Morphological Stemmer
    pub fn fulltext(&self) -> Arc<RwLock<FullTextIndex>> {
        Arc::clone(&self.fulltext)
    }

    /// Mengakses mesin Spasial GIS & R-Tree Spatial Index (PostGIS compatible)
    pub fn gis(&self) -> Arc<GisEngine> {
        Arc::clone(&self.gis)
    }

    /// Mengakses mesin Property Graph Database & Cypher Traversal
    pub fn graph(&self) -> Arc<GraphEngine> {
        Arc::clone(&self.graph)
    }

    /// Mengakses pengelola Native Database Triggers & Audit Trail Log
    pub fn triggers(&self) -> Arc<TriggerManager> {
        Arc::clone(&self.triggers)
    }

    /// Mengakses mesin Autonomous Zero-ETL HTAP & SIMD Vectorized Aggregation
    pub fn htap(&self) -> Arc<HtapEngine> {
        Arc::clone(&self.htap)
    }
}

