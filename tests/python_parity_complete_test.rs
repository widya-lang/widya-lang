use widya::ai::python_bridge::{DlPackCapsule, PyValue, PythonBridge};
use widya::ai::tensor::Tensor;
use widya::compiler::bytecode_compiler::BytecodeCompiler;
use widya::compiler::vm::VmValue;
use widya::compiler::NativeCompiler;
use widya::ffi::FfiManager;
use widya::gpu::GpuHardwareRuntime;
use widya::interpreter::Interpreter;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::value::Value;

fn eval_widya(source: &str) -> Value {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program).expect("Interpreter gagal")
}

fn eval_widya_var(source: &str, var_name: &str) -> Value {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program).expect("Interpreter gagal");
    let bindings = interpreter.get_global_variables();
    bindings.get(var_name).cloned().unwrap_or(Value::Nil)
}

#[test]
fn test_pilar_1_fstring_interpolation() {
    let kode = r#"
        misalkan nama = "Widya";
        misalkan versi = 2;
        misalkan pesan = f"Halo {nama} v{versi + 1}, status={{aktif}}!";
    "#;
    let hasil = eval_widya_var(kode, "pesan");
    assert_eq!(hasil, Value::String("Halo Widya v3, status={aktif}!".to_string()));
}

#[test]
fn test_pilar_1_list_and_dict_comprehensions() {
    let kode = r#"
        misalkan kuadrat_ganjil = [x * x untuk x dalam 1..6 jika x % 2 == 1];
        misalkan peta_kuadrat = {ke_teks(x): x * 10 untuk x dalam 1..3};
        misalkan nilai_2 = peta_kuadrat["2"];
    "#;
    let kg = eval_widya_var(kode, "kuadrat_ganjil");
    assert_eq!(kg.to_string_repr(), "[1, 9, 25]");

    let n2 = eval_widya_var(kode, "nilai_2");
    assert_eq!(n2, Value::Number(20.0));
}

#[test]
fn test_pilar_1_slice_and_membership_operators() {
    let kode = r#"
        misalkan angka = [10, 20, 30, 40, 50];
        misalkan potong_tengah = angka[1:4];
        misalkan potong_awal = angka[:3];
        misalkan potong_akhir = angka[3:];
        misalkan balik_langkah = angka[::-1];
        misalkan teks_balik = "widya"[::-1];

        misalkan ada_30 = 30 dalam angka;
        misalkan tidak_ada_99 = 99 bukan dalam angka;
        misalkan sub_dalam_teks = "dya" dalam "widya";
    "#;
    assert_eq!(eval_widya_var(kode, "potong_tengah").to_string_repr(), "[20, 30, 40]");
    assert_eq!(eval_widya_var(kode, "potong_awal").to_string_repr(), "[10, 20, 30]");
    assert_eq!(eval_widya_var(kode, "potong_akhir").to_string_repr(), "[40, 50]");
    assert_eq!(eval_widya_var(kode, "balik_langkah").to_string_repr(), "[50, 40, 30, 20, 10]");
    assert_eq!(eval_widya_var(kode, "teks_balik"), Value::String("aydiw".to_string()));
    assert_eq!(eval_widya_var(kode, "ada_30"), Value::Bool(true));
    assert_eq!(eval_widya_var(kode, "tidak_ada_99"), Value::Bool(true));
    assert_eq!(eval_widya_var(kode, "sub_dalam_teks"), Value::Bool(true));
}

#[test]
fn test_pilar_1_default_params_variadics_kwargs_decorators_lambda_with() {
    let kode = r#"
        fungsi sapa(nama = "Nusantara", sapaan = "Halo") {
            kembalikan f"{sapaan}, {nama}!";
        }
        misalkan s1 = sapa();
        misalkan s2 = sapa(sapaan="Selamat Pagi", nama="Garuda");

        fungsi jumlahkan_semua(awal, *angka_lain) {
            kembalikan awal + angka_lain.jumlah();
        }
        misalkan total_var = jumlahkan_semua(10, 20, 30, 40);

        fungsi gandakan_hasil(fn_asli) {
            kembalikan fungsi(x) {
                kembalikan fn_asli(x) * 2;
            };
        }

        @gandakan_hasil
        fungsi tambah_lima(n) {
            kembalikan n + 5;
        }
        misalkan hasil_dekorator = tambah_lima(10);

        misalkan kali = lambda a, b: a * b;
        misalkan hasil_lambda = kali(6, 7);

        misalkan sumber_daya = {"nama": "berkas.txt"};
        misalkan jejak = "";
        dengan sumber_daya sebagai sd {
            jejak = sd["nama"];
        }
        misalkan sudah_ditutup = sumber_daya["_ditutup"];
    "#;
    assert_eq!(eval_widya_var(kode, "s1"), Value::String("Halo, Nusantara!".to_string()));
    assert_eq!(eval_widya_var(kode, "s2"), Value::String("Selamat Pagi, Garuda!".to_string()));
    assert_eq!(eval_widya_var(kode, "total_var"), Value::Number(100.0));
    assert_eq!(eval_widya_var(kode, "hasil_dekorator"), Value::Number(30.0));
    assert_eq!(eval_widya_var(kode, "hasil_lambda"), Value::Number(42.0));
    assert_eq!(eval_widya_var(kode, "jejak"), Value::String("berkas.txt".to_string()));
    assert_eq!(eval_widya_var(kode, "sudah_ditutup"), Value::Bool(true));
}

#[test]
fn test_pilar_1_first_class_object_methods() {
    let kode = r#"
        misalkan teks = "  halo dunia widya  ";
        misalkan bersih = teks.hapus_spasi().besar();
        misalkan kata = bersih.pisah(" ");
        misalkan digabung = "-".gabung(kata);

        misalkan daftar = [3, 1, 4, 2];
        daftar.tambah(5);
        daftar.urutkan();
        misalkan hasil_peta = daftar.petakan(lambda x: x * 10);
        misalkan hasil_saring = hasil_peta.saring(lambda x: x >= 30);

        misalkan kamus = {"b": 2, "a": 1};
        misalkan kunci_urut = kamus.kunci();
        misalkan ambil_default = kamus.ambil("z", 99);
    "#;
    assert_eq!(eval_widya_var(kode, "bersih"), Value::String("HALO DUNIA WIDYA".to_string()));
    assert_eq!(eval_widya_var(kode, "digabung"), Value::String("HALO-DUNIA-WIDYA".to_string()));
    assert_eq!(eval_widya_var(kode, "hasil_saring").to_string_repr(), "[30, 40, 50]");
    assert_eq!(eval_widya_var(kode, "kunci_urut").to_string_repr(), "[\"a\", \"b\"]");
    assert_eq!(eval_widya_var(kode, "ambil_default"), Value::Number(99.0));
}

#[test]
fn test_pilar_2_unified_bytecode_vm_and_native_compiler() {
    let kode = r#"
        misalkan daftar = [x * 2 untuk x dalam 1..4];
        misalkan irisan = daftar[1:3];
        misalkan total = 0;
        untuk item dalam irisan {
            total += item;
        }
        misalkan nilai_akar = akar(16);
        kembalikan total + nilai_akar;
    "#;
    let mut lexer = Lexer::new(kode);
    let tokens = lexer.scan_tokens().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    // 1. Jalankan di Interpreter
    let mut interp = Interpreter::new();
    let interp_res = interp.interpret(&program).unwrap();
    assert_eq!(interp_res, Value::Number(14.0)); // irisan = [4, 6] -> sum = 10 + sqrt(16)=4 = 14

    // 2. Jalankan di Bytecode VM (dengan jembatan CallNative ke stdlib)
    let bc_compiler = BytecodeCompiler::new();
    let unit = bc_compiler.compile_program(&program).unwrap();
    let vm_res = unit.run().unwrap();
    assert_eq!(vm_res, VmValue::Int(14));

    // 3. Kompilasi di NativeCompiler (Rust codegen)
    let mut native = NativeCompiler::new();
    let rust_code = native.compile_to_rust(&program).unwrap();
    assert!(rust_code.contains("fn _iris_koleksi"));
    assert!(rust_code.contains("fn _apakah_dalam"));
}

#[test]
fn test_pilar_3_zero_copy_dlpack_real_python_and_ffi() {
    // 1. Zero-Copy DLPack Capsule
    let tensor = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let capsule = DlPackCapsule::from_tensor(&tensor);
    assert_eq!(capsule.shape, vec![2, 3]);
    assert_eq!(capsule.strides, vec![3, 1]);
    assert_ne!(capsule.data_ptr, 0);
    let reconstructed = unsafe { capsule.to_tensor_zero_copy().unwrap() };
    assert_eq!(reconstructed, tensor);

    // 2. Real Python 3 Execution via PythonBridge & Widya stdlib
    let mut bridge = PythonBridge::new();
    let py_eval = bridge.eval_python_real("[x * x for x in range(1, 5)]").unwrap();
    assert_eq!(
        py_eval,
        PyValue::List(vec![
            PyValue::Int(1),
            PyValue::Int(4),
            PyValue::Int(9),
            PyValue::Int(16)
        ])
    );

    let kode_py = r#"
        misalkan hasil_py = panggil_python("math", "sqrt", 81);
        misalkan eval_py = eval_python("sum([10, 20, 30])");
    "#;
    assert_eq!(eval_widya_var(kode_py, "hasil_py"), Value::Number(9.0));
    assert_eq!(eval_widya_var(kode_py, "eval_py"), Value::Number(60.0));

    // 3. Real C-ABI FFI via libloading
    let mut ffi = FfiManager::new();
    let sqrt_res = ffi.call_foreign_function("sqrt", &[Value::Number(49.0)]).unwrap();
    assert_eq!(sqrt_res, Value::Number(7.0));
}

#[test]
fn test_pilar_4_hierarchical_std_module_imports() {
    let kode = r#"
        impor std::matematika sebagai mtk;
        dari std::json impor dumps, loads;
        dari std::koleksi impor rentang, enumerasi, pasangkan;

        misalkan nilai_akar = mtk.sqrt(25);
        misalkan json_str = dumps({"bahasa": "Widya"});
        misalkan obj = loads(json_str);
        misalkan r = rentang(0, 6, 2);
        misalkan z = pasangkan(["a", "b"], [1, 2]);
    "#;
    assert_eq!(eval_widya_var(kode, "nilai_akar"), Value::Number(5.0));
    let obj = eval_widya_var(kode, "obj");
    assert!(obj.to_string_repr().contains("Widya"));
    assert_eq!(eval_widya_var(kode, "r").to_string_repr(), "[0, 2, 4]");
    assert_eq!(eval_widya_var(kode, "z").to_string_repr(), "[[\"a\", 1], [\"b\", 2]]");
}

#[test]
fn test_pilar_5_tiled_simd_tensor_and_gpu_runtime() {
    let hw = GpuHardwareRuntime::probe();
    assert!(hw.compute_units >= 1);

    let kode = r#"
        dari std::ai impor tensor_baru, tensor_matmul, tensor_transpose, tensor_perhatian, tensor_jumlah, info_perangkat_gpu;

        misalkan a = tensor_baru([2, 2], [1, 2, 3, 4]);
        misalkan b = tensor_baru([2, 2], [5, 6, 7, 8]);
        misalkan c = tensor_matmul(a, b);
        misalkan total_c = tensor_jumlah(c);

        misalkan q = tensor_baru([2, 2], [1.0, 0.0, 0.0, 1.0]);
        misalkan k = tensor_baru([2, 2], [1.0, 0.0, 0.0, 1.0]);
        misalkan v = tensor_baru([2, 2], [10.0, 20.0, 30.0, 40.0]);
        misalkan attn = tensor_perhatian(q, k, v);
        misalkan info = info_perangkat_gpu();
        misalkan punya_backend = info["backend"] != "";
    "#;
    // [1,2; 3,4] x [5,6; 7,8] = [19, 22; 43, 50], sum = 19+22+43+50 = 134
    assert_eq!(eval_widya_var(kode, "total_c"), Value::Number(134.0));
    assert_eq!(eval_widya_var(kode, "punya_backend"), Value::Bool(true));
    let attn_val = eval_widya_var(kode, "attn");
    let attn_tensor = Tensor::from_widya_value(&attn_val).unwrap();
    assert_eq!(attn_tensor.shape, vec![2, 2]);
}
