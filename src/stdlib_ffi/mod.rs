//! Pustaka Standar FFI untuk Widya-Lang
//! Menyediakan binding level tinggi untuk SQLite/MySQL/PostgreSQL (SQL Relasional)
//! + DuckDB (Embedded OLAP) + SurrealDB (Embedded Multi-Model) + Redb (Embedded KV)
//! serta Kriptografi industri.
//! — Mobile-ready: semua embedded DB single-file, tanpa FFI native (kecuali DuckDB/bundled).

pub mod sqlite;
pub mod mysql;
pub mod postgres;
pub mod duckdb;
pub mod surrealdb_w;
pub mod redb_w;
pub mod crypto;
