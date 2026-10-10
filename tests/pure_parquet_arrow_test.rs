// ============================================================================
// Integration Test: Pure Sovereign Apache Parquet & Arrow Columnar Engine
// ============================================================================
// Menguji serialisasi dan deserialisasi biner Parquet (PAR1) dan Arrow IPC
// (ARROW1) dengan 64-byte SIMD alignment tanpa ketergantungan PyArrow / C++.
// ============================================================================

use std::fs;
use widya::widyabase::{
    ArrowRecordBatchIpc, ColumnData, ColumnType, ParquetArrowConnector,
    ParquetCompression, PureArrowEngine, PureParquetEngine, PureRecordBatch,
};

#[test]
fn test_record_batch_creation_and_aggregations() {
    let mut batch = PureRecordBatch::baru();

    batch
        .tambah_kolom(
            "id",
            ColumnData::Int64(vec![101, 102, 103, 104, 105]),
        )
        .unwrap();

    batch
        .tambah_kolom(
            "nilai",
            ColumnData::Float64(vec![75.5, 82.0, 91.5, 68.0, 95.0]),
        )
        .unwrap();

    batch
        .tambah_kolom(
            "nama",
            ColumnData::Utf8(vec![
                "Budi".to_string(),
                "Siti".to_string(),
                "Joko".to_string(),
                "Dewi".to_string(),
                "Agus".to_string(),
            ]),
        )
        .unwrap();

    batch
        .tambah_kolom(
            "lulus",
            ColumnData::Boolean(vec![true, true, true, false, true]),
        )
        .unwrap();

    assert_eq!(batch.num_rows, 5);
    assert_eq!(batch.column_names.len(), 4);

    // Agregasi numerik
    let sum_id = batch.sum("id").unwrap();
    assert_eq!(sum_id, 515.0);

    let avg_nilai = batch.avg("nilai").unwrap();
    assert!((avg_nilai - 82.4).abs() < 1e-4);

    let min_nilai = batch.min("nilai").unwrap();
    assert_eq!(min_nilai, 68.0);

    let max_nilai = batch.max("nilai").unwrap();
    assert_eq!(max_nilai, 95.0);
}

#[test]
fn test_parquet_binary_serialization_and_roundtrip() {
    let mut batch = PureRecordBatch::baru();
    batch
        .tambah_kolom("transaksi_id", ColumnData::Int64(vec![1001, 1002, 1003]))
        .unwrap();
    batch
        .tambah_kolom("nominal", ColumnData::Float64(vec![150_000.50, 275_500.00, 89_900.25]))
        .unwrap();
    batch
        .tambah_kolom(
            "merchant",
            ColumnData::Utf8(vec![
                "Warung Berdaulat".to_string(),
                "Toko Nusantara".to_string(),
                "Koperasi Maju".to_string(),
            ]),
        )
        .unwrap();
    batch
        .tambah_kolom("sukses", ColumnData::Boolean(vec![true, true, false]))
        .unwrap();

    // 1. Tulis ke format Parquet biner
    let parquet_bytes = PureParquetEngine::tulis_ke_bytes(&batch)
        .expect("Serialisasi Parquet harus berhasil");

    // 2. Verifikasi magic PAR1 di awal dan akhir
    assert!(parquet_bytes.len() >= 12);
    assert_eq!(&parquet_bytes[0..4], b"PAR1");
    let len = parquet_bytes.len();
    assert_eq!(&parquet_bytes[len - 4..len], b"PAR1");

    // 3. Baca kembali dari format Parquet biner
    let parsed_batch = PureParquetEngine::baca_dari_bytes(&parquet_bytes)
        .expect("Deserialisasi Parquet harus berhasil");

    assert_eq!(parsed_batch.num_rows, 3);
    assert_eq!(parsed_batch.column_names, batch.column_names);
    assert_eq!(parsed_batch, batch);
}

#[test]
fn test_arrow_ipc_serialization_and_roundtrip() {
    let mut batch = PureRecordBatch::baru();
    batch
        .tambah_kolom("sensor_id", ColumnData::Int64(vec![1, 2, 3, 4]))
        .unwrap();
    batch
        .tambah_kolom("suhu", ColumnData::Float64(vec![28.4, 29.1, 31.0, 27.8]))
        .unwrap();
    batch
        .tambah_kolom(
            "lokasi",
            ColumnData::Utf8(vec![
                "Jakarta".to_string(),
                "Bandung".to_string(),
                "Surabaya".to_string(),
                "Medan".to_string(),
            ]),
        )
        .unwrap();
    batch
        .tambah_kolom("aktif", ColumnData::Boolean(vec![true, true, true, false]))
        .unwrap();

    // 1. Tulis ke format Arrow IPC biner
    let arrow_bytes = PureArrowEngine::tulis_ke_bytes(&batch)
        .expect("Serialisasi Arrow IPC harus berhasil");

    // 2. Verifikasi magic ARROW1 di awal dan akhir
    assert_eq!(&arrow_bytes[0..6], b"ARROW1");
    let len = arrow_bytes.len();
    assert_eq!(&arrow_bytes[len - 8..len - 2], b"ARROW1");

    // 3. Baca kembali dari format Arrow IPC biner
    let parsed_batch = PureArrowEngine::baca_dari_bytes(&arrow_bytes)
        .expect("Deserialisasi Arrow IPC harus berhasil");

    assert_eq!(parsed_batch.num_rows, 4);
    assert_eq!(parsed_batch.column_names, batch.column_names);
    assert_eq!(parsed_batch, batch);
}

#[test]
fn test_parquet_and_arrow_file_export_import() {
    let mut batch = PureRecordBatch::baru();
    batch
        .tambah_kolom("kode_akun", ColumnData::Int64(vec![1101, 1102, 1103]))
        .unwrap();
    batch
        .tambah_kolom("saldo", ColumnData::Float64(vec![1_250_000.0, 3_400_000.0, 850_000.0]))
        .unwrap();

    let temp_dir = std::env::temp_dir().join("widya_parquet_arrow_test");
    let parquet_path = temp_dir.join("buku_besar.parquet");
    let arrow_path = temp_dir.join("buku_besar.arrow");

    // Ekspor & Impor Parquet
    let exp_parquet = PureParquetEngine::ekspor_berkas(&batch, &parquet_path).unwrap();
    assert_eq!(exp_parquet, parquet_path);
    assert!(parquet_path.exists());
    let imported_parquet = PureParquetEngine::impor_berkas(&parquet_path).unwrap();
    assert_eq!(imported_parquet, batch);

    // Ekspor & Impor Arrow
    let exp_arrow = PureArrowEngine::ekspor_berkas(&batch, &arrow_path).unwrap();
    assert_eq!(exp_arrow, arrow_path);
    assert!(arrow_path.exists());
    let imported_arrow = PureArrowEngine::impor_berkas(&arrow_path).unwrap();
    assert_eq!(imported_arrow, batch);

    // Pembersihan
    let _ = fs::remove_file(&parquet_path);
    let _ = fs::remove_file(&arrow_path);
    let _ = fs::remove_dir(&temp_dir);
}

#[test]
fn test_backwards_compatibility_connector() {
    let meta = ParquetArrowConnector::create_parquet_metadata(
        &["col_a", "col_b"],
        5000,
        ParquetCompression::Snappy,
    );
    assert_eq!(meta.num_rows, 5000);
    assert_eq!(meta.columns.len(), 2);

    let batch_ipc: ArrowRecordBatchIpc = ParquetArrowConnector::create_arrow_record_batch(
        vec![("id", "Int64"), ("val", "Float64")],
        1000,
    );
    assert_eq!(batch_ipc.length, 1000);
    assert_eq!(batch_ipc.memory_alignment, 64);
    assert_eq!(batch_ipc.schema.len(), 2);
}
