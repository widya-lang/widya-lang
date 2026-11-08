// tests/wpm_packages_parity_test.rs
// Pengujian Integrasi Paket Resmi WPM: Paritas dengan Python & Node.js
// 1. @widya/tensor   -> Paritas NumPy (Array N-D, Matmul, Transpose, Reduksi)
// 2. @widya/dataframe -> Paritas Pandas (Tabular, Filter, GroupBy, CSV I/O, Describe)
// 3. @widya/web       -> Paritas Express/FastAPI (Radix Router, Middleware, JSON Response)

use std::fs;
use std::path::Path;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::borrow_checker::BorrowChecker;
use widya::interpreter::Interpreter;
use widya::value::Value;

fn execute_widya_code(code: &str) -> (Interpreter, Value) {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");
    let mut borrow_checker = BorrowChecker::new();
    borrow_checker.check_program(&program).expect("BorrowChecker gagal");
    let mut interpreter = Interpreter::new();
    let val = interpreter.interpret(&program).expect("Interpreter gagal");
    (interpreter, val)
}

#[test]
fn test_package_widya_tensor_numpy_parity() {
    let tensor_code = fs::read_to_string("packages/widya-tensor/src/lib.wya")
        .expect("Gagal membaca packages/widya-tensor/src/lib.wya");

    let test_script = format!(
        r#"
        {}

        // Uji 1: Matriks Identitas 3x3
        misal I3 = identitas(3);
        misal jml_identitas = jumlah(I3);

        // Uji 2: Perkalian Matriks 2x2
        // A = [[1, 2], [3, 4]], B = [[2, 0], [1, 2]]
        // C = A x B = [[4, 4], [10, 8]]
        misal A = tensor_buat([1.0, 2.0, 3.0, 4.0], [2, 2]);
        misal B = tensor_buat([2.0, 0.0, 1.0, 2.0], [2, 2]);
        misal C = matmul(A, B);

        // Uji 3: Transpose
        misal C_T = transpose(C);

        // Uji 4: Reduksi Statistik
        misal total_c = jumlah(C);
        misal rata_c = rata_rata(C);
        misal max_c = maksimum(C);
        misal min_c = minimum(C);
        "#,
        tensor_code
    );

    let (interp, _) = execute_widya_code(&test_script);
    let globals = interp.get_global_variables();

    // Verifikasi Matriks Identitas (diagonal 1+1+1 = 3)
    let jml_identitas = globals.get("jml_identitas").unwrap();
    assert_eq!(jml_identitas, &Value::Number(3.0));

    // Verifikasi Perkalian Matriks C = [[4, 4], [10, 8]] -> sum = 26
    let total_c = globals.get("total_c").unwrap();
    assert_eq!(total_c, &Value::Number(26.0));

    // Verifikasi Nilai Ekstrem (min = 4, max = 10)
    let min_c = globals.get("min_c").unwrap();
    let max_c = globals.get("max_c").unwrap();
    assert_eq!(min_c, &Value::Number(4.0));
    assert_eq!(max_c, &Value::Number(10.0));

    // Verifikasi Rata-rata (26 / 4 = 6.5)
    let rata_c = globals.get("rata_c").unwrap();
    assert_eq!(rata_c, &Value::Number(6.5));
}

#[test]
fn test_package_widya_dataframe_pandas_parity() {
    let df_code = fs::read_to_string("packages/widya-dataframe/src/lib.wya")
        .expect("Gagal membaca packages/widya-dataframe/src/lib.wya");

    let test_script = format!(
        r#"
        {}

        // 1. Buat DataFrame dari CSV
        misal csv_mentah = "nama,departemen,gaji\nBudi,Teknik,12000000\nSiti,Keuangan,15000000\nAndi,Teknik,18000000\nDewi,SDM,10000000";
        misal df = dari_csv(csv_mentah);

        // 2. Filter / Saring: gaji >= 15000000
        misal df_tinggi = saring(df, "gaji", ">=", 15000000);
        misal jumlah_tinggi = df_tinggi.jumlah_baris;

        // 3. Proyeksi Kolom
        misal df_nama_dept = pilih_kolom(df, ["nama", "departemen"]);

        // 4. Ringkasan Statistik Describe
        misal stats = ringkasan(df);

        // 5. GroupBy & Count per Departemen
        misal grup_dept = kelompokkan_dan_hitung(df, "departemen");

        // 6. Konversi Kembali ke CSV
        misal csv_ulang = ke_csv(df_tinggi);
        "#,
        df_code
    );

    let (interp, _) = execute_widya_code(&test_script);
    let globals = interp.get_global_variables();

    // Verifikasi jumlah baris yang tersaring (Siti & Andi: 2 baris)
    let jml_tinggi = globals.get("jumlah_tinggi").unwrap();
    assert_eq!(jml_tinggi, &Value::Number(2.0));

    // Verifikasi konversi ke CSV ulang memuat header dan data
    let csv_ulang = globals.get("csv_ulang").unwrap();
    if let Value::String(s) = csv_ulang {
        assert!(s.contains("nama,departemen,gaji"));
        assert!(s.contains("Siti"));
        assert!(s.contains("Andi"));
        assert!(!s.contains("Dewi")); // Terfilter keluar
    } else {
        panic!("csv_ulang harus bertipe String");
    }
}

#[test]
fn test_package_widya_web_express_fastapi_parity() {
    let web_code = fs::read_to_string("packages/widya-web/src/lib.wya")
        .expect("Gagal membaca packages/widya-web/src/lib.wya");

    let test_script = format!(
        r#"
        {}

        misal app = buat_router();

        // 1. Daftarkan Rute GET
        app = get(app, "/api/v1/status", fungsi(req) {{
            kembalikan respons_json({{"status": "OK", "layanan": "Widya-Server"}}, 200);
        }});

        // 2. Daftarkan Rute POST
        app = post(app, "/api/v1/pengguna", fungsi(req) {{
            kembalikan respons_json({{"sukses": true, "pesan": "Pengguna dibuat"}}, 201);
        }});

        // 3. Uji Permintaan Sukses
        misal req_status = Permintaan("GET", "/api/v1/status", {{}}, {{}}, nil);
        misal res_status = tangani_permintaan(app, req_status);
        misal status_code = res_status.status;

        // 4. Uji Rute 404
        misal req_hilang = Permintaan("GET", "/api/v1/tidak_ada", {{}}, {{}}, nil);
        misal res_hilang = tangani_permintaan(app, req_hilang);
        misal status_hilang = res_hilang.status;
        "#,
        web_code
    );

    let (interp, _) = execute_widya_code(&test_script);
    let globals = interp.get_global_variables();

    // Verifikasi Status HTTP 200 untuk route yang cocok
    let status_code = globals.get("status_code").unwrap();
    assert_eq!(status_code, &Value::Number(200.0));

    // Verifikasi Status HTTP 404 untuk route yang tidak ditemukan
    let status_hilang = globals.get("status_hilang").unwrap();
    assert_eq!(status_hilang, &Value::Number(404.0));
}

#[test]
fn test_package_widya_ai_pytorch_parity() {
    let script = r#"
        impor "@widya/ai" sebagai torch;

        // 1. Inisialisasi Lapisan Linear: y = w * x + b
        // w = 2.0, b = 1.0, x = 3.0 -> y = 2*3 + 1 = 7.0
        misal layer = torch.lapisan_linear(2.0, 1.0);
        misal x = torch.variabel(3.0, false);
        misal y_pred = torch.linear_maju(layer, x);
        misal hasil_pred = y_pred.data;

        // 2. Hitung MSE Loss dengan target 10.0
        // diff = 7 - 10 = -3, loss = 0.5 * (-3)^2 = 4.5
        misal rugi = torch.mse_loss(y_pred, 10.0);
        misal nilai_rugi = rugi.data;

        // 3. Backward Pass (Autograd perambatan mundur)
        // dLoss/dy = (y - 10) = -3
        // dy/dw = x = 3 -> dLoss/dw = -3 * 3 = -9.0
        // dy/db = 1 -> dLoss/db = -3 * 1 = -3.0
        torch.mundur(rugi);
        misal grad_w = layer.bobot.grad;
        misal grad_b = layer.bias.grad;

        // 4. Langkah Optimasi SGD dengan lr = 0.1
        // w_baru = 2.0 - (0.1 * -9.0) = 2.9
        torch.sgd_langkah([layer.bobot, layer.bias], 0.1);
        misal w_baru = layer.bobot.data;
    "#;

    let (interp, _) = execute_widya_code(script);
    let globals = interp.get_global_variables();

    assert_eq!(globals.get("hasil_pred").unwrap(), &Value::Number(7.0));
    assert_eq!(globals.get("nilai_rugi").unwrap(), &Value::Number(4.5));
    assert_eq!(globals.get("grad_w").unwrap(), &Value::Number(-9.0));
    assert_eq!(globals.get("grad_b").unwrap(), &Value::Number(-3.0));
    assert_eq!(globals.get("w_baru").unwrap(), &Value::Number(2.9));
}

#[test]
fn test_package_widya_sql_sqlite_parity() {
    let script = r#"
        impor "@widya/sql" sebagai sql;

        // 1. Inisialisasi basis data relasional in-memory
        misal db = sql.db_buka("sistem_transaksi");
        sql.db_buat_tabel(db, "pelanggan");

        // 2. Operasi INSERT (db_sisip)
        sql.db_sisip(db, "pelanggan", {"id": 1, "nama": "Ahmad", "status": "aktif", "poin": 120});
        sql.db_sisip(db, "pelanggan", {"id": 2, "nama": "Budi", "status": "nonaktif", "poin": 40});
        sql.db_sisip(db, "pelanggan", {"id": 3, "nama": "Citra", "status": "aktif", "poin": 250});

        // 3. Operasi COUNT
        misal total_awal = sql.db_hitung(db, "pelanggan");

        // 4. Operasi SELECT dengan filter WHERE
        fungsi hanya_aktif(row) {
            kembalikan row["status"] == "aktif";
        }
        misal data_aktif = sql.db_pilih(db, "pelanggan", hanya_aktif, ["id", "nama", "poin"]);
        misal jml_aktif = panjang(data_aktif);

        // 5. Operasi UPDATE
        fungsi cari_budi(row) {
            kembalikan row["id"] == 2;
        }
        misal terupdate = sql.db_perbarui(db, "pelanggan", cari_budi, {"status": "aktif", "poin": 100});

        // 6. Hitung ulang setelah update
        misal jml_aktif_akhir = sql.db_hitung(db, "pelanggan", hanya_aktif);

        // 7. Tutup koneksi
        sql.db_tutup(db);
        misal status_tutup = db.terbuka;
    "#;

    let (interp, _) = execute_widya_code(script);
    let globals = interp.get_global_variables();

    assert_eq!(globals.get("total_awal").unwrap(), &Value::Number(3.0));
    assert_eq!(globals.get("jml_aktif").unwrap(), &Value::Number(2.0));
    assert_eq!(globals.get("terupdate").unwrap(), &Value::Number(1.0));
    assert_eq!(globals.get("jml_aktif_akhir").unwrap(), &Value::Number(3.0));
    assert_eq!(globals.get("status_tutup").unwrap(), &Value::Bool(false));
}

#[test]
fn test_package_widya_test_framework_parity() {
    let script = r#"
        impor "@widya/test" sebagai test;

        misal pelari = test.uji_buat_pelari();

        fungsi kasus_sukses(t) {
            test.tegaskan_sama(t, 10 + 5, 15, "10+5 harus 15");
            test.tegaskan_tidak_sama(t, 10, 20, "10 != 20");
            test.tegaskan_benar(t, 100 > 50, "100 > 50");
            test.tegaskan_salah(t, 10 == 99, "10 tidak sama dengan 99");
        }

        fungsi kasus_gagal(t) {
            test.tegaskan_sama(t, 2 + 2, 5, "Sengaja gagal untuk verifikasi pelapor");
        }

        test.uji_kasus(pelari, "Kasus Sukses", kasus_sukses);
        test.uji_kasus(pelari, "Kasus Kegagalan", kasus_gagal);

        misal ringkasan = test.uji_ringkasan(pelari);
        misal total_uji = ringkasan["total"];
        misal lulus_uji = ringkasan["lulus"];
        misal gagal_uji = ringkasan["gagal"];
        misal status_lulus_semua = ringkasan["sukses"];
    "#;

    let (interp, _) = execute_widya_code(script);
    let globals = interp.get_global_variables();

    assert_eq!(globals.get("total_uji").unwrap(), &Value::Number(2.0));
    assert_eq!(globals.get("lulus_uji").unwrap(), &Value::Number(1.0));
    assert_eq!(globals.get("gagal_uji").unwrap(), &Value::Number(1.0));
    assert_eq!(globals.get("status_lulus_semua").unwrap(), &Value::Bool(false));
}

#[test]
fn test_wpm_manifest_validity_for_all_packages() {
    let packages = [
        "widya-tensor",
        "widya-dataframe",
        "widya-web",
        "widya-ai",
        "widya-sql",
        "widya-test"
    ];

    for pkg_name in &packages {
        let manifest_path = format!("packages/{}/Widya.toml", pkg_name);
        assert!(Path::new(&manifest_path).exists(), "Manifest {} harus ada", manifest_path);

        let content = fs::read_to_string(&manifest_path).unwrap();
        assert!(content.contains("[package]"), "{} harus memuat blok [package]", pkg_name);
        assert!(content.contains("name = \"@widya/"), "{} harus menggunakan namespace @widya/", pkg_name);
        assert!(content.contains("version = \"0.1.0\""), "{} harus memiliki versi valid", pkg_name);
        assert!(content.contains("license = \"MIT\""), "{} harus memiliki lisensi open source", pkg_name);
    }
}

#[test]
fn test_wpm_package_import_resolution_syntax() {
    let script = r#"
        impor "@widya/tensor" sebagai np;
        impor "@widya/dataframe" sebagai pd;
        impor "@widya/web" sebagai web;
        impor "@widya/ai" sebagai torch;
        impor "@widya/sql" sebagai sql;
        impor "@widya/test" sebagai test;

        misal t = np.identitas(2);
        misal total = np.jumlah(t);

        misal df = pd.df_buat(["a", "b"], [{"a": 10, "b": 20}]);
        misal jml_baris = df.jumlah_baris;

        misal app = web.buat_router();
        misal jml_rute = panjang(app.rute_list);

        misal v = torch.variabel(42.0);
        misal nilai_v = v.data;

        misal db = sql.db_buka();
        misal status_db = db.terbuka;

        misal pelari = test.uji_buat_pelari();
        misal total_pelari = pelari.total;
    "#;

    let (interp, _) = execute_widya_code(script);
    let globals = interp.get_global_variables();

    assert_eq!(globals.get("total").unwrap(), &Value::Number(2.0));
    assert_eq!(globals.get("jml_baris").unwrap(), &Value::Number(1.0));
    assert_eq!(globals.get("jml_rute").unwrap(), &Value::Number(0.0));
    assert_eq!(globals.get("nilai_v").unwrap(), &Value::Number(42.0));
    assert_eq!(globals.get("status_db").unwrap(), &Value::Bool(true));
    assert_eq!(globals.get("total_pelari").unwrap(), &Value::Number(0.0));
}
