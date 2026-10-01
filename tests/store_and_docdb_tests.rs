use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use widya::widyabase::docdb::*;
use widya::widyabase::object_store::*;

#[test]
fn test_widya_store_bucket_and_object_crud() {
    let test_dir = PathBuf::from("./target/test_widya_store_crud");
    let _ = fs::remove_dir_all(&test_dir);

    let config = ObjectStoreConfig {
        base_path: test_dir.clone(),
        enable_encryption: false,
        ..Default::default()
    };
    let store = WidyaObjectStore::new(config).expect("Inisialisasi WidyaObjectStore gagal");

    // 1. Create Bucket
    let bucket_info = store.create_bucket("dokumen-negara").expect("Gagal membuat bucket");
    assert_eq!(bucket_info.name, "dokumen-negara");
    assert_eq!(bucket_info.versioning, VersioningStatus::Suspended);

    // 2. List Buckets
    let buckets = store.list_buckets();
    assert_eq!(buckets.len(), 1);
    assert_eq!(buckets[0].name, "dokumen-negara");

    // 3. Put Object
    let content = b"Dokumen Resmi Widya-Lang Berdaulat 2026";
    let mut meta_map = HashMap::new();
    meta_map.insert("author".to_string(), "Dr. Satriyo".to_string());
    meta_map.insert("classification".to_string(), "Rahasia".to_string());

    let put_meta = store.put_object(
        "dokumen-negara",
        "laporan/q3_2026.pdf",
        content,
        "application/pdf",
        meta_map,
    ).expect("Gagal put object");

    assert_eq!(put_meta.size_bytes, content.len() as u64);
    assert_eq!(put_meta.content_type, "application/pdf");
    assert!(put_meta.is_latest);

    // 4. Get Object
    let retrieved = store.get_object("dokumen-negara", "laporan/q3_2026.pdf").expect("Gagal get object");
    assert_eq!(retrieved.data, content);
    assert_eq!(retrieved.metadata.user_metadata.get("author").unwrap(), "Dr. Satriyo");

    // 5. Range Get
    let (range_meta, slice) = store.get_object_range("dokumen-negara", "laporan/q3_2026.pdf", 0, 6).expect("Gagal range get");
    assert_eq!(slice, b"Dokumen");
    assert_eq!(range_meta.key, "laporan/q3_2026.pdf");

    // 6. Copy Object
    let copy_meta = store.copy_object(
        "dokumen-negara",
        "laporan/q3_2026.pdf",
        "dokumen-negara",
        "arsip/q3_2026_backup.pdf",
    ).expect("Gagal copy object");
    assert_eq!(copy_meta.key, "arsip/q3_2026_backup.pdf");

    // 7. List Objects V2 (S3 Compatible)
    let list_res = store.list_objects_v2("dokumen-negara", Some("laporan/"), None, 100);
    assert_eq!(list_res.key_count, 1);
    assert_eq!(list_res.objects[0].key, "laporan/q3_2026.pdf");

    let s3_xml = list_res.to_s3_xml();
    assert!(s3_xml.contains("<ListBucketResult"));
    assert!(s3_xml.contains("laporan/q3_2026.pdf"));

    // 8. Delete Object
    store.delete_object("dokumen-negara", "laporan/q3_2026.pdf").expect("Gagal delete object");
    assert!(store.get_object("dokumen-negara", "laporan/q3_2026.pdf").is_err());

    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_widya_store_multipart_upload_and_presigned_url() {
    let test_dir = PathBuf::from("./target/test_widya_store_multipart");
    let _ = fs::remove_dir_all(&test_dir);

    let config = ObjectStoreConfig {
        base_path: test_dir.clone(),
        enable_encryption: true, // Test AES-256-GCM encryption at rest
        ..Default::default()
    };
    let store = WidyaObjectStore::new(config).expect("Inisialisasi WidyaObjectStore gagal");
    store.create_bucket("bigdata-bucket").unwrap();

    // 1. Initiate Multipart
    let upload_id = store.initiate_multipart_upload(
        "bigdata-bucket",
        "dataset_sensor.bin",
        "application/octet-stream",
        HashMap::new(),
    ).expect("Gagal initiate multipart");

    // 2. Upload Parts
    let part1 = b"BAGIAN_PERTAMA_1234567890_";
    let part2 = b"BAGIAN_KEDUA_ABCDEFGHIJ_";
    let part3 = b"BAGIAN_KETIGA_SELESAI!";

    store.upload_part(&upload_id, 1, part1).expect("Gagal upload part 1");
    store.upload_part(&upload_id, 2, part2).expect("Gagal upload part 2");
    store.upload_part(&upload_id, 3, part3).expect("Gagal upload part 3");

    // 3. Complete Multipart
    let completed_meta = store.complete_multipart_upload(&upload_id).expect("Gagal complete multipart");
    let expected_len = part1.len() + part2.len() + part3.len();
    assert_eq!(completed_meta.size_bytes, expected_len as u64);

    // 4. Retrieve & Decrypt from storage
    let fetched = store.get_object("bigdata-bucket", "dataset_sensor.bin").expect("Gagal get encrypted object");
    let mut expected_bytes = Vec::new();
    expected_bytes.extend_from_slice(part1);
    expected_bytes.extend_from_slice(part2);
    expected_bytes.extend_from_slice(part3);
    assert_eq!(fetched.data, expected_bytes);

    // 5. Presigned URL (HMAC-SHA256 Signature V4)
    let presigned_url = store.generate_presigned_url(
        "bigdata-bucket",
        "dataset_sensor.bin",
        "WIDYA_ACCESS_KEY_2026",
        "WIDYA_SECRET_KEY_SUPER_SECURE",
        3600,
    );
    assert!(presigned_url.contains("AWSAccessKeyId=WIDYA_ACCESS_KEY_2026"));
    assert!(presigned_url.contains("Signature="));

    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_widya_docdb_crud_and_rich_queries() {
    let test_dir = PathBuf::from("./target/test_widya_docdb_crud");
    let _ = fs::remove_dir_all(&test_dir);

    let db = WidyaDocDB::open(&test_dir).expect("Gagal membuka WidyaDocDB");
    let users = db.collection("pengguna");

    // 1. Insert Many
    let docs = vec![
        json!({
            "username": "budi_santoso",
            "nama": "Budi Santoso",
            "umur": 32,
            "peran": "Direktur",
            "gaji": 25000000.0,
            "aktif": true,
            "kontak": { "email": "budi@perusahaan.co.id", "kota": "Jakarta" },
            "keahlian": ["Manajemen", "Keuangan", "Strategi"]
        }),
        json!({
            "username": "siti_aminah",
            "nama": "Siti Aminah",
            "umur": 28,
            "peran": "Lead Engineer",
            "gaji": 22000000.0,
            "aktif": true,
            "kontak": { "email": "siti@perusahaan.co.id", "kota": "Bandung" },
            "keahlian": ["Rust", "Widya-Lang", "Distributed Systems"]
        }),
        json!({
            "username": "ahmad_fauzi",
            "nama": "Ahmad Fauzi",
            "umur": 24,
            "peran": "Junior Dev",
            "gaji": 9000000.0,
            "aktif": false,
            "kontak": { "email": "ahmad@perusahaan.co.id", "kota": "Yogyakarta" },
            "keahlian": ["HTML", "CSS", "JavaScript"]
        }),
    ];

    let inserted_ids = users.insert_many(docs).expect("Gagal insert_many");
    assert_eq!(inserted_ids.len(), 3);

    // 2. Query $eq & $gt
    let res_gt = users.find(&json!({ "umur": { "$gt": 25 } }), None, None, None);
    assert_eq!(res_gt.len(), 2);

    // 3. Query Nested Dot Notation (`kontak.kota`)
    let res_bandung = users.find(&json!({ "kontak.kota": "Bandung" }), None, None, None);
    assert_eq!(res_bandung.len(), 1);
    assert_eq!(res_bandung[0]["username"], "siti_aminah");

    // 4. Query $in & Array field
    let res_in = users.find(&json!({ "peran": { "$in": ["Lead Engineer", "Junior Dev"] } }), None, None, None);
    assert_eq!(res_in.len(), 2);

    // 5. Query $regex & $and
    let res_regex = users.find(&json!({
        "$and": [
            { "kontak.email": { "$regex": ".*@perusahaan\\.co\\.id" } },
            { "aktif": true }
        ]
    }), None, None, None);
    assert_eq!(res_regex.len(), 2);

    // 6. Update Modifiers ($set & $inc)
    let update_res = users.update_one(
        &json!({ "username": "siti_aminah" }),
        &json!({
            "$inc": { "gaji": 3000000.0 },
            "$set": { "peran": "Principal Architect" }
        }),
        false,
    ).expect("Gagal update_one");

    assert_eq!(update_res.modified_count, 1);

    let updated_siti = users.find_one(&json!({ "username": "siti_aminah" })).unwrap();
    assert_eq!(updated_siti["gaji"], 25000000.0);
    assert_eq!(updated_siti["peran"], "Principal Architect");

    // 7. Delete One
    let del_res = users.delete_one(&json!({ "username": "ahmad_fauzi" })).expect("Gagal delete_one");
    assert_eq!(del_res.deleted_count, 1);
    assert_eq!(users.count_documents(&json!({})), 2);

    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_widya_docdb_aggregation_pipeline() {
    let test_dir = PathBuf::from("./target/test_widya_docdb_agg");
    let _ = fs::remove_dir_all(&test_dir);

    let db = WidyaDocDB::open(&test_dir).expect("Gagal membuka WidyaDocDB");
    let sales = db.collection("penjualan");

    let records = vec![
        json!({ "cabang": "Jakarta", "kategori": "Elektronik", "nominal": 15000000.0, "qty": 5 }),
        json!({ "cabang": "Jakarta", "kategori": "Furniture", "nominal": 8000000.0, "qty": 2 }),
        json!({ "cabang": "Bandung", "kategori": "Elektronik", "nominal": 12000000.0, "qty": 4 }),
        json!({ "cabang": "Bandung", "kategori": "Furniture", "nominal": 6000000.0, "qty": 3 }),
        json!({ "cabang": "Surabaya", "kategori": "Elektronik", "nominal": 20000000.0, "qty": 8 }),
    ];
    sales.insert_many(records).unwrap();

    // Aggregation Pipeline: $match (Elektronik) -> $group by cabang ($sum nominal, $sum qty, $avg nominal) -> $sort by total_nominal desc
    let pipeline = vec![
        json!({
            "$match": { "kategori": "Elektronik" }
        }),
        json!({
            "$group": {
                "_id": "$cabang",
                "total_omset": { "$sum": "$nominal" },
                "total_qty": { "$sum": "$qty" },
                "rata_rata": { "$avg": "$nominal" }
            }
        }),
        json!({
            "$sort": { "total_omset": -1 }
        })
    ];

    let results = db.aggregate("penjualan", &pipeline).expect("Gagal eksekusi agregasi");
    assert_eq!(results.len(), 3);

    // Peringkat 1: Surabaya (20M)
    assert_eq!(results[0]["_id"], "Surabaya");
    assert_eq!(results[0]["total_omset"], 20000000.0);

    // Peringkat 2: Jakarta (15M)
    assert_eq!(results[1]["_id"], "Jakarta");
    assert_eq!(results[1]["total_omset"], 15000000.0);

    // Peringkat 3: Bandung (12M)
    assert_eq!(results[2]["_id"], "Bandung");
    assert_eq!(results[2]["total_omset"], 12000000.0);

    let _ = fs::remove_dir_all(test_dir);
}
