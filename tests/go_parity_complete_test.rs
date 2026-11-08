use widya::interpreter::Interpreter;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::value::Value;

fn eval_widya(src: &str) -> Interpreter {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");
    let mut interp = Interpreter::new();
    interp.interpret(&program).expect("Interpreter gagal");
    interp
}

fn get_num(interp: &Interpreter, name: &str) -> f64 {
    let globals = interp.get_global_variables();
    let val = globals
        .get(name)
        .unwrap_or_else(|| panic!("Variabel '{}' tidak ditemukan", name));
    match val {
        Value::Number(n) => *n,
        other => panic!("Variabel '{}' bukan angka: {:?}", name, other),
    }
}

fn get_str(interp: &Interpreter, name: &str) -> String {
    let globals = interp.get_global_variables();
    let val = globals
        .get(name)
        .unwrap_or_else(|| panic!("Variabel '{}' tidak ditemukan", name));
    val.to_string_repr()
}

fn get_bool(interp: &Interpreter, name: &str) -> bool {
    let globals = interp.get_global_variables();
    let val = globals
        .get(name)
        .unwrap_or_else(|| panic!("Variabel '{}' tidak ditemukan", name));
    match val {
        Value::Bool(b) => *b,
        other => panic!("Variabel '{}' bukan bool: {:?}", name, other),
    }
}

#[test]
fn test_pilar_g1_defer_lifo_panic_recover_and_multi_return() {
    let code = r#"
        // 1. Urutan eksekusi `tunda` (defer) harus LIFO (Last-In, First-Out)
        buat jejak_defer = "";

        fungsi uji_urutan_defer() -> teks {
            tunda { jejak_defer = jejak_defer + "1"; }
            tunda { jejak_defer = jejak_defer + "2"; }
            tunda { jejak_defer = jejak_defer + "3"; }
            jejak_defer = jejak_defer + "A-";
            kembalikan "SELESAI";
        }

         status_uji := uji_urutan_defer();

        // 2. Pasangan `panik` (panic) & `pulihkan` (recover) di dalam blok `tunda` (defer)
        buat pesan_pulih = "";

        fungsi operasi_berbahaya(x: angka) -> angka {
            tunda {
                buat r = pulihkan();
                jika r != nihil {
                    pesan_pulih = f"PULIH:{r}";
                    kembalikan -99;
                }
            }
            jika x == 0 {
                panik("pembagian_dengan_nol_fatal");
            }
            kembalikan 100 / x;
        }

        hasil_aman := operasi_berbahaya(4);
        hasil_panik := operasi_berbahaya(0);

        // 3. Multi-Return Values (`kembalikan hasil, galat`) & Deklarasi Singkat `:=`
        fungsi bagi_go(a: angka, b: angka) {
            jika b == 0 {
                kembalikan 0, "pembagi tidak boleh nol";
            }
            kembalikan a / b, nihil;
        }

        buat hasil_ok, galat_ok = bagi_go(42, 6);
        hasil_err, galat_err := bagi_go(10, 0);

        // 4. Idiom Comma-Ok pada Kamus (Map Lookup: `nilai, ada := kamus[kunci]`)
        peta_kota := {"Jakarta": 10, "Bandung": 2};
        pop_jkt, ada_jkt := peta_kota["Jakarta"];
        pop_sby, ada_sby := peta_kota["Surabaya"];
    "#;

    let interp = eval_widya(code);
    assert_eq!(get_str(&interp, "status_uji"), "SELESAI");
    // "A-" diikuti tunda ke-3, ke-2, ke-1 -> "A-321"
    assert_eq!(get_str(&interp, "jejak_defer"), "A-321");

    assert_eq!(get_num(&interp, "hasil_aman"), 25.0);
    assert_eq!(get_num(&interp, "hasil_panik"), -99.0);
    assert_eq!(
        get_str(&interp, "pesan_pulih"),
        "PULIH:pembagian_dengan_nol_fatal"
    );

    assert_eq!(get_num(&interp, "hasil_ok"), 7.0);
    assert_eq!(get_str(&interp, "galat_ok"), "nihil");
    assert_eq!(get_num(&interp, "hasil_err"), 0.0);
    assert_eq!(get_str(&interp, "galat_err"), "pembagi tidak boleh nol");

    assert_eq!(get_num(&interp, "pop_jkt"), 10.0);
    assert!(get_bool(&interp, "ada_jkt"));
    assert_eq!(get_str(&interp, "pop_sby"), "nihil");
    assert!(!get_bool(&interp, "ada_sby"));
}

#[test]
fn test_pilar_g2_goroutines_channels_buffered_range_select_waitgroup_mutex() {
    let code = r#"
        // 1. Goroutine (`jalan`), GrupTunggu (`WaitGroup`), KunciMutex (`Mutex`), & `tunda wg.selesai()`
        wg := GrupTunggu();
        mu := KunciMutex();
        buat akumulator = 0;

        untuk i dalam 1..5 {
            wg.tambah(1);
            jalan {
                tunda wg.selesai();
                mu.kunci();
                akumulator = akumulator + (i * 10);
                mu.buka_kunci();
            }
        }
        wg.tunggu();

        // 2. Buffered Channel (`saluran(kapasitas)`), Operator `<-`, `tutup_saluran`, & Range-over-Channel
        ch_buf := saluran(4);
        kap_ch := ch_buf.kapasitas();
        ch_buf <- 11;
        ch_buf <- 22;
        ch_buf <- 33;
        pjg_sebelum := ch_buf.panjang();
        tutup_saluran(ch_buf);

        buat total_range = 0;
        untuk item dalam ch_buf {
            total_range = total_range + item;
        }

        // Idiom Comma-Ok pada Saluran yang sudah ditutup: `nilai, masih_buka := <-ch_buf`
        nilai_kosong, masih_buka := <-ch_buf;

        // 3. Multiplexing `pilih_saluran` (select) dengan cabang terima, bawaan (default), dan setelah_waktu (timeout)
        ch1 := saluran(2);
        ch2 := saluran(2);
        ch2 <- "PESAN_DARI_CH2";

        buat hasil_select_1 = "";
        pilih_saluran {
            kasus m1 = <-ch1 => {
                hasil_select_1 = f"CH1:{m1}";
            }
            kasus m2 = <-ch2 => {
                hasil_select_1 = f"CH2:{m2}";
            }
            bawaan => {
                hasil_select_1 = "BAWAAN";
            }
        }

        // Ketika kedua saluran kosong, cabang `bawaan` langsung dieksekusi tanpa memblokir
        buat hasil_select_default = "";
        pilih_saluran {
            kasus m1 = <-ch1 => {
                hasil_select_default = m1;
            }
            bawaan => {
                hasil_select_default = "NON_BLOCKING_DEFAULT";
            }
        }

        // Ketika saluran kosong dan tidak ada `bawaan`, cabang `setelah_waktu(15)` terpicu
        buat hasil_select_timeout = "";
        pilih_saluran {
            kasus m1 = <-ch1 => {
                hasil_select_timeout = m1;
            }
            setelah_waktu(15) => {
                hasil_select_timeout = "TIMEOUT_TERPICU";
            }
        }
    "#;

    let interp = eval_widya(code);
    // 10 + 20 + 30 + 40 + 50 = 150
    assert_eq!(get_num(&interp, "akumulator"), 150.0);
    assert_eq!(get_num(&interp, "kap_ch"), 4.0);
    assert_eq!(get_num(&interp, "pjg_sebelum"), 3.0);
    // 11 + 22 + 33 = 66
    assert_eq!(get_num(&interp, "total_range"), 66.0);
    assert_eq!(get_str(&interp, "nilai_kosong"), "nihil");
    assert!(!get_bool(&interp, "masih_buka"));

    assert_eq!(get_str(&interp, "hasil_select_1"), "CH2:PESAN_DARI_CH2");
    assert_eq!(
        get_str(&interp, "hasil_select_default"),
        "NON_BLOCKING_DEFAULT"
    );
    assert_eq!(get_str(&interp, "hasil_select_timeout"), "TIMEOUT_TERPICU");
}

#[test]
fn test_pilar_g3_implicit_structural_interfaces_struct_embedding_and_receiver_methods() {
    let code = r#"
        // 1. Definisi struct dasar & metode Receiver bergaya Go (`fungsi (m: Mesin) ...`)
        struktur Mesin {
            tenaga_kuda: angka
        }

        fungsi (m: Mesin) nyalakan() -> teks {
            kembalikan f"Mesin {m.tenaga_kuda} HP menyala";
        }

        // 2. Struct Embedding (Komposisi Anonim Go): `Kendaraan` menyematkan `Mesin`
        struktur Kendaraan {
            Mesin,
            merek: teks
        }

        fungsi (k: Kendaraan) deskripsi() -> teks {
            // Mengakses field `tenaga_kuda` dan metode `nyalakan()` yang dipromosikan dari `Mesin`!
            kembalikan f"{k.merek}: {k.nyalakan()}";
        }

        // 3. Antarmuka Struktural Implisit (Go Interface tanpa `terapkan Sifat untuk Tipe`)
        antarmuka Penggerak {
            nyalakan() -> teks;
            deskripsi() -> teks;
        }

        antarmuka Terbang {
            lepas_landas() -> teks;
        }

        mobil := Kendaraan {
            Mesin: Mesin { tenaga_kuda: 350 },
            merek: "Maung Garuda"
        };

        // Promosi field langsung (baca & tulis)
        hp_awal := mobil.tenaga_kuda;
        mobil.tenaga_kuda = 420;
        hp_baru := mobil.tenaga_kuda;

        // Promosi metode langsung & metode receiver
        suara_mesin := mobil.nyalakan();
        info_mobil  := mobil.deskripsi();

        // Verifikasi kecocokan antarmuka struktural implisit (Duck Typing terverifikasi)
        cocok_penggerak := memenuhi_sifat(mobil, Penggerak);
        cocok_terbang   := memenuhi_sifat(mobil, Terbang);
        cocok_any       := memenuhi_sifat(mobil, "interface{}");

        // Type assertion bergaya Go: `obj, ok := pastikan_sifat(mobil, Penggerak)`
        obj_valid, ok_assert := pastikan_sifat(mobil, Penggerak);
    "#;

    let interp = eval_widya(code);
    assert_eq!(get_num(&interp, "hp_awal"), 350.0);
    assert_eq!(get_num(&interp, "hp_baru"), 420.0);
    assert_eq!(get_str(&interp, "suara_mesin"), "Mesin 420 HP menyala");
    assert_eq!(
        get_str(&interp, "info_mobil"),
        "Maung Garuda: Mesin 420 HP menyala"
    );

    assert!(get_bool(&interp, "cocok_penggerak"));
    assert!(!get_bool(&interp, "cocok_terbang"));
    assert!(get_bool(&interp, "cocok_any"));
    assert!(get_bool(&interp, "ok_assert"));
}

#[test]
fn test_pilar_g4_context_cancellation_timeout_and_binary_protobuf_grpc() {
    let code = r#"
        // 1. Go `context.Context` (`Konteks`): WithValue, WithCancel, Cascading Child Cancel, &WithTimeout
        induk := konteks_latar();
        ctx_nilai := konteks_dengan_nilai(induk, "trace_id", "TRC-2026-NUSANTARA");

        ctx_batal, batalkan := konteks_dengan_batal(ctx_nilai);
        ctx_anak, _ := konteks_dengan_batal(ctx_batal);

        trace_di_anak := ctx_anak.nilai("trace_id");
        batal_sebelum := ctx_anak.apakah_batal();

        // Batalkan konteks induk -> konteks anak ikut dibatalkan secara berantai & saluran selesai() tertutup
        batalkan();
        batal_sesudah := ctx_anak.apakah_batal();
        alasan_batal  := ctx_anak.galat();

        buat sinyal_selesai = "";
        pilih_saluran {
            kasus msg = <-ctx_anak.selesai() => {
                sinyal_selesai = f"SELESAI:{msg}";
            }
            bawaan => {
                sinyal_selesai = "BELUM_SELESAI";
            }
        }

        // Uji WithTimeout (`konteks_dengan_batas_waktu`)
        ctx_waktu, _ := konteks_dengan_batas_waktu(induk, 15);
        buat status_deadline = "";
        pilih_saluran {
            kasus msg = <-ctx_waktu.selesai() => {
                status_deadline = msg;
            }
            setelah_waktu(200) => {
                status_deadline = "TIDAK_TERBATALKAN";
            }
        }

        // 2. Binary Protocol Buffers (Protobuf Wire Format) & Layanan gRPC Asli
        pesan_asli := {
            "id_transaksi": 1024,
            "nominal": 250.75,
            "pengirim": "WidyaNode"
        };

        wire_bytes := protobuf_encode(pesan_asli);
        ukuran_wire := wire_bytes.panjang();
        hasil_dekode := protobuf_decode(wire_bytes, ["id_transaksi", "nominal", "pengirim"]);

        id_dekode      := hasil_dekode["id_transaksi"];
        nominal_dekode := hasil_dekode["nominal"];
        pengirim_dek   := hasil_dekode["pengirim"];

        // Server & Klien gRPC dengan registrasi handler
        srv := ServerGRPC("LayananPembayaran", 50051);
        grpc_daftarkan_metode(srv, "ProsesBayar", fungsi(req) {
            kembalikan f"LUNAS:{req['pengirim']}:{req['id_transaksi']}";
        });

        klien := KlienGRPC("127.0.0.1:50051", "LayananPembayaran");
        resp_rpc := grpc_panggil(klien, "ProsesBayar", pesan_asli);
        status_rpc := resp_rpc["status"];
        isi_rpc    := resp_rpc["respon"];
        bytes_rpc  := resp_rpc["ukuran_protobuf"];
    "#;

    let interp = eval_widya(code);
    assert_eq!(get_str(&interp, "trace_di_anak"), "TRC-2026-NUSANTARA");
    assert!(!get_bool(&interp, "batal_sebelum"));
    assert!(get_bool(&interp, "batal_sesudah"));
    assert_eq!(get_str(&interp, "alasan_batal"), "context canceled");
    assert_eq!(
        get_str(&interp, "sinyal_selesai"),
        "SELESAI:context canceled"
    );
    assert_eq!(
        get_str(&interp, "status_deadline"),
        "context deadline exceeded"
    );

    assert!(get_num(&interp, "ukuran_wire") > 10.0);
    assert_eq!(get_num(&interp, "id_dekode"), 1024.0);
    assert_eq!(get_num(&interp, "nominal_dekode"), 250.75);
    assert_eq!(get_str(&interp, "pengirim_dek"), "WidyaNode");

    assert_eq!(get_str(&interp, "status_rpc"), "OK_200");
    assert_eq!(get_str(&interp, "isi_rpc"), "LUNAS:WidyaNode:1024");
    assert_eq!(
        get_num(&interp, "bytes_rpc"),
        get_num(&interp, "ukuran_wire")
    );
}
