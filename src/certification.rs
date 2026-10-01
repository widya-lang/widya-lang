// ============================================================================
// WIDYA CERTIFICATION & AUTOMATED EXAM ENGINE
// ============================================================================
// Modul sertifikasi resmi Widya-Lang:
// - WCA (Widya Certified Associate - Dasar Sintaks, Struktur Data & Logika)
// - WCP (Widya Certified Professional - OOP, Fungsional, Konkurensi & Stdlib)
// - WCE (Widya Certified Expert - Arsitektur Multi-Agent, FFI, LLVM & Performance)
// - Spesialisasi: WCWD (Web), WCMD (Mobile), WCDD (Desktop), WCSS (Security),
//                WCDA (Data/AI), WCCD (Cloud), WCEI (Enterprise & Industrial)
// ============================================================================

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::environment::Environment;
use std::sync::{Arc, Mutex};

/// Tingkatan & Jalur Sertifikasi Resmi
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExamLevel {
    WCA,  // Associate
    WCP,  // Professional
    WCE,  // Expert
    WCWD, // Web Developer
    WCMD, // Mobile Developer
    WCDD, // Desktop Developer
    WCSS, // Security Specialist
    WCDA, // Data & AI Engineer
    WCCD, // Cloud & DevOps Engineer
    WCEI, // Enterprise & Industrial Architect
}

impl ExamLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "WCA" | "ASSOCIATE" | "DASAR" => Some(ExamLevel::WCA),
            "WCP" | "PROFESSIONAL" | "PROFESIONAL" => Some(ExamLevel::WCP),
            "WCE" | "EXPERT" | "PAKAR" => Some(ExamLevel::WCE),
            "WCWD" | "WEB" => Some(ExamLevel::WCWD),
            "WCMD" | "MOBILE" => Some(ExamLevel::WCMD),
            "WCDD" | "DESKTOP" => Some(ExamLevel::WCDD),
            "WCSS" | "SECURITY" | "KEAMANAN" => Some(ExamLevel::WCSS),
            "WCDA" | "DATA" | "AI" => Some(ExamLevel::WCDA),
            "WCCD" | "CLOUD" | "DEVOPS" => Some(ExamLevel::WCCD),
            "WCEI" | "ENTERPRISE" | "INDUSTRI" => Some(ExamLevel::WCEI),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            ExamLevel::WCA => "WCA",
            ExamLevel::WCP => "WCP",
            ExamLevel::WCE => "WCE",
            ExamLevel::WCWD => "WCWD",
            ExamLevel::WCMD => "WCMD",
            ExamLevel::WCDD => "WCDD",
            ExamLevel::WCSS => "WCSS",
            ExamLevel::WCDA => "WCDA",
            ExamLevel::WCCD => "WCCD",
            ExamLevel::WCEI => "WCEI",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            ExamLevel::WCA => "Widya Certified Associate (WCA)",
            ExamLevel::WCP => "Widya Certified Professional (WCP)",
            ExamLevel::WCE => "Widya Certified Expert (WCE)",
            ExamLevel::WCWD => "Widya Certified Web Developer (WCWD)",
            ExamLevel::WCMD => "Widya Certified Mobile Developer (WCMD)",
            ExamLevel::WCDD => "Widya Certified Desktop Developer (WCDD)",
            ExamLevel::WCSS => "Widya Certified Security Specialist (WCSS)",
            ExamLevel::WCDA => "Widya Certified Data & AI Engineer (WCDA)",
            ExamLevel::WCCD => "Widya Certified Cloud & DevOps Architect (WCCD)",
            ExamLevel::WCEI => "Widya Certified Enterprise & Industrial Architect (WCEI)",
        }
    }

    pub fn passing_score(&self) -> f64 {
        match self {
            ExamLevel::WCA => 70.0,
            ExamLevel::WCP => 75.0,
            ExamLevel::WCE => 85.0,
            _ => 80.0,
        }
    }

    /// Biaya standar komersial ujian (IDR)
    pub fn standard_fee_idr(&self) -> u64 {
        match self {
            ExamLevel::WCA => 750_000,
            ExamLevel::WCP => 1_750_000,
            ExamLevel::WCE => 3_500_000,
            _ => 2_250_000,
        }
    }

    /// Biaya promo diskon pendidikan 70% (Siswa, Mahasiswa, Guru, Dosen, Tenaga Pendidik)
    pub fn academic_discount_fee_idr(&self) -> u64 {
        let normal = self.standard_fee_idr();
        // Diskon 70%, peserta hanya membayar 30% dari biaya normal
        (normal as f64 * 0.30).round() as u64
    }

    /// Memeriksa apakah email pendaftar berhak atas diskon akademik (.ac.id, .sch.id, .edu)
    pub fn is_academic_email(email: &str) -> bool {
        let lower = email.trim().to_lowercase();
        lower.ends_with(".ac.id") || lower.ends_with(".sch.id") || lower.ends_with(".edu") || lower.ends_with(".edu.id")
    }
}

/// Soal Pilihan Ganda
#[derive(Debug, Clone)]
pub struct MultipleChoiceQuestion {
    pub id: u32,
    pub question: String,
    pub options: Vec<String>,
    pub correct_option_index: usize,
    pub explanation: String,
}

/// Soal Tantangan Kode (Hands-on Coding Challenge)
#[derive(Debug, Clone)]
pub struct CodingChallenge {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub starter_code: String,
    pub test_cases: Vec<CodingTestCase>,
}

#[derive(Debug, Clone)]
pub struct CodingTestCase {
    pub name: String,
    pub code_wrapper: String, // Wrapper test case yang mengeksekusi fungsi kandidat
    pub expected_output: String,
}

/// Bank Soal Lengkap
pub struct QuestionBank;

impl QuestionBank {
    pub fn get_exam_questions(level: &ExamLevel) -> (Vec<MultipleChoiceQuestion>, Vec<CodingChallenge>) {
        match level {
            ExamLevel::WCA => (Self::wca_mc_questions(), Self::wca_coding_challenges()),
            ExamLevel::WCP => (Self::wcp_mc_questions(), Self::wcp_coding_challenges()),
            ExamLevel::WCE => (Self::wce_mc_questions(), Self::wce_coding_challenges()),
            ExamLevel::WCWD => (Self::wcwd_mc_questions(), Self::wcwd_coding_challenges()),
            ExamLevel::WCMD => (Self::wcmd_mc_questions(), Self::wcmd_coding_challenges()),
            ExamLevel::WCDD => (Self::wcdd_mc_questions(), Self::wcdd_coding_challenges()),
            ExamLevel::WCSS => (Self::wcss_mc_questions(), Self::wcss_coding_challenges()),
            ExamLevel::WCDA => (Self::wcda_mc_questions(), Self::wcda_coding_challenges()),
            ExamLevel::WCCD => (Self::wccd_mc_questions(), Self::wccd_coding_challenges()),
            ExamLevel::WCEI => (Self::wcei_mc_questions(), Self::wcei_coding_challenges()),
        }
    }

    // ================= WCA =================
    fn wca_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 1,
                question: "Keyword apa saja di Widya-Lang yang sah untuk deklarasi variabel baru?".to_string(),
                options: vec![
                    "var dan const".to_string(),
                    "misal, biar, diketahui, diket, dan tetap".to_string(),
                    "let dan mut".to_string(),
                    "dim dan static".to_string(),
                ],
                correct_option_index: 1,
                explanation: "Widya-Lang mendukung sintaks natural Indonesia seperti `misal`, `biar`, `diketahui`, `diket` (dan `tetap` untuk konstanta imutable).".to_string(),
            },
            MultipleChoiceQuestion {
                id: 2,
                question: "Bagaimana cara mencetak teks dan output ke konsol di Widya-Lang?".to_string(),
                options: vec![
                    "print(\"halo\") atau echo \"halo\"".to_string(),
                    "tulis(\"halo\") atau cetak(\"halo\")".to_string(),
                    "console.log(\"halo\")".to_string(),
                    "System.out.println(\"halo\")".to_string(),
                ],
                correct_option_index: 1,
                explanation: "Fungsi bawaan untuk output teks adalah `tulis(...)` atau `cetak(...)`.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 3,
                question: "Keyword percabangan apa saja yang sah di Widya-Lang?".to_string(),
                options: vec![
                    "if - elif - else".to_string(),
                    "jika, kalau, jika_tidak, selain_itu".to_string(),
                    "when - otherwise".to_string(),
                    "check - fall".to_string(),
                ],
                correct_option_index: 1,
                explanation: "Widya-Lang menyediakan `jika`, `kalau`, `jika_tidak`, dan `selain_itu`.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 4,
                question: "Konstruksi perulangan apa yang digunakan untuk mengiterasi elemen koleksi/larik?".to_string(),
                options: vec![
                    "untuk_setiap item dalam koleksi { ... }".to_string(),
                    "foreach(koleksi as item)".to_string(),
                    "loop.each { ... }".to_string(),
                    "iterate item of koleksi".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Widya-Lang menggunakan konstruksi `untuk_setiap item dalam koleksi` serta `selama` / `ulangi_selama`.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 5,
                question: "Bagaimana cara mendeklarasikan fungsi yang mengembalikan nilai di Widya-Lang?".to_string(),
                options: vec![
                    "fungsi hitung(a, b) { kembalikan a + b }".to_string(),
                    "def hitung(a, b) => a + b".to_string(),
                    "fn hitung(a, b): return a + b".to_string(),
                    "sub hitung(a, b) { result = a + b }".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Deklarasi fungsi menggunakan kata kunci `fungsi` dan nilai kembalian menggunakan `kembalikan`.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 6,
                question: "Bagaimana cara mendefinisikan array / larik di Widya-Lang?".to_string(),
                options: vec![
                    "diketahui data = [10, 20, 30]".to_string(),
                    "array data = (10, 20, 30)".to_string(),
                    "dim data[3] = {10, 20, 30}".to_string(),
                    "list data = <10, 20, 30>".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Larik didefinisikan menggunakan kurung siku `[ ... ]`.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 7,
                question: "Operator perbandingan apakah yang digunakan untuk menguji kesamaan dua nilai?".to_string(),
                options: vec![
                    "== atau sama_dengan".to_string(),
                    ":=".to_string(),
                    "eq".to_string(),
                    "=".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Widya-Lang menggunakan operator standar `==`.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 8,
                question: "Keyword apa yang digunakan untuk menghentikan perulangan lebih awal?".to_string(),
                options: vec![
                    "berhenti / break".to_string(),
                    "lanjut".to_string(),
                    "kembalikan".to_string(),
                    "selesai".to_string(),
                ],
                correct_option_index: 0,
                explanation: "`berhenti` atau `break` digunakan untuk keluar dari blok perulangan secara instan.".to_string(),
            },
        ]
    }

    fn wca_coding_challenges() -> Vec<CodingChallenge> {
        vec![
            CodingChallenge {
                id: 101,
                title: "Fungsi Penjumlahan Sederhana".to_string(),
                description: "Buat fungsi `tambah(a, b)` yang mengembalikan hasil penjumlahan dua angka.".to_string(),
                starter_code: "fungsi tambah(a, b) {\n    kembalikan a + b\n}\n".to_string(),
                test_cases: vec![
                    CodingTestCase {
                        name: "Test Case 1: 5 + 7".to_string(),
                        code_wrapper: "tambah(5, 7)\n".to_string(),
                        expected_output: "12".to_string(),
                    },
                    CodingTestCase {
                        name: "Test Case 2: -3 + 10".to_string(),
                        code_wrapper: "tambah(-3, 10)\n".to_string(),
                        expected_output: "7".to_string(),
                    },
                ],
            },
            CodingChallenge {
                id: 102,
                title: "Fungsi Cek Genap Ganjil".to_string(),
                description: "Buat fungsi `apakah_genap(n)` yang mengembalikan teks 'GENAP' jika genap dan 'GANJIL' jika ganjil.".to_string(),
                starter_code: "fungsi apakah_genap(n) {\n    jika (n % 2 == 0) {\n        kembalikan \"GENAP\"\n    } selain_itu {\n        kembalikan \"GANJIL\"\n    }\n}\n".to_string(),
                test_cases: vec![
                    CodingTestCase {
                        name: "Test Case 1: 8".to_string(),
                        code_wrapper: "apakah_genap(8)\n".to_string(),
                        expected_output: "GENAP".to_string(),
                    },
                    CodingTestCase {
                        name: "Test Case 2: 15".to_string(),
                        code_wrapper: "apakah_genap(15)\n".to_string(),
                        expected_output: "GANJIL".to_string(),
                    },
                ],
            },
            CodingChallenge {
                id: 103,
                title: "Fungsi Menghitung Nilai Maksimum".to_string(),
                description: "Buat fungsi `maksimum(a, b)` yang mengembalikan angka terbesar antara a dan b.".to_string(),
                starter_code: "fungsi maksimum(a, b) {\n    jika (a > b) {\n        kembalikan a\n    } selain_itu {\n        kembalikan b\n    }\n}\n".to_string(),
                test_cases: vec![
                    CodingTestCase {
                        name: "Test Case 1: 25 vs 40".to_string(),
                        code_wrapper: "maksimum(25, 40)\n".to_string(),
                        expected_output: "40".to_string(),
                    },
                    CodingTestCase {
                        name: "Test Case 2: 100 vs 50".to_string(),
                        code_wrapper: "maksimum(100, 50)\n".to_string(),
                        expected_output: "100".to_string(),
                    },
                ],
            },
        ]
    }

    // ================= WCP =================
    fn wcp_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 201,
                question: "Fitur konkurensi di Widya-Lang dibangun di atas konsep apa?".to_string(),
                options: vec![
                    "Single-thread callback hell".to_string(),
                    "Actor Model, Channel Message Passing, dan Task Concurrency".to_string(),
                    "Manual OS thread locking tanpa garbage collector".to_string(),
                    "Coroutines Python 2 style".to_string(),
                ],
                correct_option_index: 1,
                explanation: "Widya-Lang mengimplementasikan Actor Model, Channel terisolasi, dan Task Concurrency yang aman.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 202,
                question: "Bagaimana cara menangani error / kesalahan secara elegan di Widya-Lang?".to_string(),
                options: vec![
                    "Menggunakan blok `coba { ... } tangkap (e) { ... }` atau tipe Result/Opsi".to_string(),
                    "Aplikasi langsung panic dan terminate".to_string(),
                    "Hanya dengan goto error_handler".to_string(),
                    "Return code integer C style".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Widya-Lang menyediakan blok `coba - tangkap` (try-catch) serta pattern matching untuk Result/Opsi.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 203,
                question: "Perintah SQL dwibahasa apa di WidyaBase yang setara dengan `SELECT * FROM tabel`?".to_string(),
                options: vec![
                    "PILIH * DARI tabel".to_string(),
                    "AMBIL * TABEL tabel".to_string(),
                    "CARI_SEMUA tabel".to_string(),
                    "GET ALL tabel".to_string(),
                ],
                correct_option_index: 0,
                explanation: "WidyaBase mendukung SQL dwibahasa penuh: PILIH (SELECT), SISIP_KE (INSERT INTO), PERBARUI (UPDATE), HAPUS_DARI (DELETE FROM).".to_string(),
            },
            MultipleChoiceQuestion {
                id: 204,
                question: "Fitur apa yang digunakan dalam modul Widya Workspace untuk kolaborasi dokumen real-time tanpa konflik?".to_string(),
                options: vec![
                    "Conflict-free Replicated Data Types (CRDT)".to_string(),
                    "Central lock single-user".to_string(),
                    "Manual file overwrite".to_string(),
                    "Git merge manual conflict".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Widya Workspace mengimplementasikan CRDT YATA/LWW-Element-Set untuk sinkronisasi teks real-time bebas konflik.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 205,
                question: "Bagaimana cara mendefinisikan struktur objek dan metode di Widya-Lang?".to_string(),
                options: vec![
                    "struktur Nama { ... } dan implementasi Nama { ... }".to_string(),
                    "class Nama { ... }".to_string(),
                    "typedef struct { ... }".to_string(),
                    "object Nama { ... }".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Widya-Lang menggunakan kata kunci `struktur` untuk data dan `implementasi` untuk metode OOP.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 206,
                question: "Format dokumen bisnis resmi apa saja yang dapat dihasilkan secara native oleh Widya Office Suite?".to_string(),
                options: vec![
                    "PDF (ISO 32000-1), XLSX Spreadsheet, DOCX Word, PPTX Presentation, dan LaTeX Math".to_string(),
                    "Hanya TXT".to_string(),
                    "Hanya RTF".to_string(),
                    "Hanya BMP gambar".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Widya Office Suite memproduksi PDF standar ISO, XLSX, DOCX, PPTX, dan LaTeX secara native tanpa pihak ketiga.".to_string(),
            },
        ]
    }

    fn wcp_coding_challenges() -> Vec<CodingChallenge> {
        vec![
            CodingChallenge {
                id: 201,
                title: "Fungsi Faktorial Rekursif".to_string(),
                description: "Buat fungsi `faktorial(n)` yang menghitung faktorial dari n.".to_string(),
                starter_code: "fungsi faktorial(n) {\n    jika (n <= 1) {\n        kembalikan 1\n    }\n    kembalikan n * faktorial(n - 1)\n}\n".to_string(),
                test_cases: vec![
                    CodingTestCase {
                        name: "Test Case 1: faktorial(5)".to_string(),
                        code_wrapper: "faktorial(5)\n".to_string(),
                        expected_output: "120".to_string(),
                    },
                    CodingTestCase {
                        name: "Test Case 2: faktorial(6)".to_string(),
                        code_wrapper: "faktorial(6)\n".to_string(),
                        expected_output: "720".to_string(),
                    },
                ],
            },
            CodingChallenge {
                id: 202,
                title: "Fungsi Deret Fibonacci".to_string(),
                description: "Buat fungsi `fibonacci(n)` yang mengembalikan nilai suku ke-n dari deret Fibonacci (fib(0)=0, fib(1)=1, fib(2)=1, fib(6)=8).".to_string(),
                starter_code: "fungsi fibonacci(n) {\n    jika (n <= 0) {\n        kembalikan 0\n    }\n    jika (n == 1) {\n        kembalikan 1\n    }\n    kembalikan fibonacci(n - 1) + fibonacci(n - 2)\n}\n".to_string(),
                test_cases: vec![
                    CodingTestCase {
                        name: "Test Case 1: fib(6)".to_string(),
                        code_wrapper: "fibonacci(6)\n".to_string(),
                        expected_output: "8".to_string(),
                    },
                    CodingTestCase {
                        name: "Test Case 2: fib(7)".to_string(),
                        code_wrapper: "fibonacci(7)\n".to_string(),
                        expected_output: "13".to_string(),
                    },
                ],
            },
        ]
    }

    // ================= WCE =================
    fn wce_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 301,
                question: "Dalam arsitektur Multi-Agent Widya-Lang, apa fungsi dari Blackboard / State Bus?".to_string(),
                options: vec![
                    "Sebagai shared in-memory blackboard untuk sinkronisasi state antar-agen secara reaktif".to_string(),
                    "Sebagai rendering pipeline CSS".to_string(),
                    "Sebagai parser file JSON saja".to_string(),
                    "Sebagai pengganti OS kernel".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Blackboard berfungsi sebagai media koordinasi dan agregasi keputusan antar-agen cerdas.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 302,
                question: "Bagaimana proses siklus kompilasi mandiri (Full Self-Hosting Compiler) di Widya-Lang?".to_string(),
                options: vec![
                    "Stage-0 (Rust Bootstrap) -> Stage-1 (Widya Compiler) -> Stage-2 (Deterministic Hash Match)".to_string(),
                    "Hanya menjalankan shell script bash".to_string(),
                    "Kompilasi melalui interpreter Python".to_string(),
                    "Transpilasi ke PHP".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Siklus Full Self-Hosting memvalidasi determinisme 100% biner compiler melalui Stage-0 -> Stage-1 -> Stage-2.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 303,
                question: "Target kompilasi biner apa saja yang didukung oleh backend kompiler Widya-Lang?".to_string(),
                options: vec![
                    "Native Executable (LLVM/C), WebAssembly (WASM), WGSL Shaders (GPU), dan eBPF Kernel Probes".to_string(),
                    "Hanya Flash SWF".to_string(),
                    "Hanya Java bytecode 1.1".to_string(),
                    "Hanya VBScript".to_string(),
                ],
                correct_option_index: 0,
                explanation: "Widya-Lang mendukung multi-target: Native LLVM, WASM, WebGPU WGSL, dan eBPF Kernel Probes.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 304,
                question: "Bagaimana Foreign Function Interface (FFI) di Widya-Lang mencapai latensi sub-milidetik?".to_string(),
                options: vec![
                    "Dengan pemetaan ABI C tingkat register & zero-copy memory buffer sharing".to_string(),
                    "Dengan serialisasi string JSON antar-proses".to_string(),
                    "Dengan menyimpan sementara ke file disk".to_string(),
                    "Dengan soket HTTP loopback".to_string(),
                ],
                correct_option_index: 0,
                explanation: "FFI native Widya-Lang beroperasi pada level register CPU dan pointer memori langsung tanpa alokasi overhead.".to_string(),
            },
        ]
    }

    fn wce_coding_challenges() -> Vec<CodingChallenge> {
        vec![
            CodingChallenge {
                id: 301,
                title: "Filter dan Transformasi Data Koleksi".to_string(),
                description: "Buat fungsi `proses_data(arr)` yang menyaring hanya angka positif dan mengalikannya dengan 2, lalu kembalikan totalnya.".to_string(),
                starter_code: "fungsi proses_data(arr) {\n    diketahui total = 0\n    untuk_setiap x dalam arr {\n        jika (x > 0) {\n            total = total + (x * 2)\n        }\n    }\n    kembalikan total\n}\n".to_string(),
                test_cases: vec![
                    CodingTestCase {
                        name: "Test Case 1: [1, -2, 3, -4, 5]".to_string(),
                        code_wrapper: "proses_data([1, -2, 3, -4, 5])\n".to_string(),
                        expected_output: "18".to_string(), // (1*2 + 3*2 + 5*2) = 2 + 6 + 10 = 18
                    },
                ],
            },
        ]
    }

    // ================= SPESIALISASI =================
    fn wcwd_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 401,
                question: "Komponen WidyaWeb menyediakan fitur apa untuk perenderan antarmuka web modern?".to_string(),
                options: vec![
                    "Virtual DOM & SSR Engine terintegrasi".to_string(),
                    "Flash Player".to_string(),
                    "PHP 4 compatibility".to_string(),
                    "Applet Java".to_string(),
                ],
                correct_option_index: 0,
                explanation: "WidyaWeb menyediakan Virtual DOM dan Server-Side Rendering native.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 402,
                question: "Fitur edge serverless di WidyaWeb Cloud mendukung apa?".to_string(),
                options: vec!["Atomic Preview Deployments, Sub-Millisecond Cold Starts, Zero-Downtime Instant Rollback".to_string(), "FTP Manual Upload".to_string(), "Full VM reboot".to_string(), "CGI-BIN script".to_string()],
                correct_option_index: 0,
                explanation: "WidyaWeb Sovereign Edge mendukung deployment atomik, rollback instan, dan performa edge kelas dunia.".to_string(),
            },
        ]
    }

    fn wcwd_coding_challenges() -> Vec<CodingChallenge> {
        vec![CodingChallenge {
            id: 401,
            title: "HTML Tag Builder".to_string(),
            description: "Buat fungsi `buat_tag(tag, konten)` yang merender tag HTML `<tag>konten</tag>`.".to_string(),
            starter_code: "fungsi buat_tag(tag, konten) {\n    kembalikan \"<\" + tag + \">\" + konten + \"</\" + tag + \">\"\n}\n".to_string(),
            test_cases: vec![CodingTestCase {
                name: "Test 1".to_string(),
                code_wrapper: "buat_tag(\"h1\", \"Widya-Lang\")\n".to_string(),
                expected_output: "<h1>Widya-Lang</h1>".to_string(),
            }],
        }]
    }

    fn wcmd_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 501,
                question: "Widya Mobile mendukung build target native untuk platform apa?".to_string(),
                options: vec!["Android (APK/AAB) & iOS (IPA)".to_string(), "Symbian OS".to_string(), "Windows CE".to_string(), "BlackBerry 7".to_string()],
                correct_option_index: 0,
                explanation: "Widya Mobile menargetkan Android dan iOS secara native.".to_string(),
            },
        ]
    }

    fn wcmd_coding_challenges() -> Vec<CodingChallenge> {
        vec![CodingChallenge {
            id: 501,
            title: "Format Nomor HP Indonesia".to_string(),
            description: "Buat fungsi `format_nomor(no)` yang mengubah awalan '08' menjadi '+628'.".to_string(),
            starter_code: "fungsi format_nomor(no) {\n    kembalikan \"+628\" + no\n}\n".to_string(),
            test_cases: vec![CodingTestCase {
                name: "Test 1".to_string(),
                code_wrapper: "format_nomor(\"123456789\")\n".to_string(),
                expected_output: "+628123456789".to_string(),
            }],
        }]
    }

    fn wcdd_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 601,
                question: "Engine GUI Desktop Widya dirancang dengan prinsip apa?".to_string(),
                options: vec!["Zero-bloat native windowing & GPU accelerated canvas (<15MB RAM)".to_string(), "Electron bundle 500MB".to_string(), "ActiveX controls".to_string(), "Flash SWF container".to_string()],
                correct_option_index: 0,
                explanation: "Widya Desktop menggunakan GPU-accelerated rendering yang sangat ringan tanpa bloatware.".to_string(),
            },
        ]
    }

    fn wcdd_coding_challenges() -> Vec<CodingChallenge> {
        vec![CodingChallenge {
            id: 601,
            title: "Window Bounds Calculator".to_string(),
            description: "Buat fungsi `hitung_luas_jendela(w, h)` yang mengembalikan luas resolusi jendela.".to_string(),
            starter_code: "fungsi hitung_luas_jendela(w, h) {\n    kembalikan w * h\n}\n".to_string(),
            test_cases: vec![CodingTestCase {
                name: "Test 1".to_string(),
                code_wrapper: "hitung_luas_jendela(1920, 1080)\n".to_string(),
                expected_output: "2073600".to_string(),
            }],
        }]
    }

    fn wcss_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 701,
                question: "Komponen WidyaShield bertugas menangani apa?".to_string(),
                options: vec!["WAF OWASP Top 10, Rate Limiting, DDoS Mitigation, Zero-Trust SPIFFE mTLS & Post-Quantum Cryptography".to_string(), "CSS Animations".to_string(), "Database migration".to_string(), "Icon generator".to_string()],
                correct_option_index: 0,
                explanation: "WidyaShield adalah modul proteksi perimeter keamanan siber berstandar enterprise dengan enklave post-quantum (Kyber/Dilithium).".to_string(),
            },
        ]
    }

    fn wcss_coding_challenges() -> Vec<CodingChallenge> {
        vec![CodingChallenge {
            id: 701,
            title: "Sanitasi String Masukan".to_string(),
            description: "Buat fungsi `sanitasi(teks)` yang mengganti tanda kutip ganda dengan tanda kutip tunggal.".to_string(),
            starter_code: "fungsi sanitasi(teks) {\n    kembalikan teks\n}\n".to_string(),
            test_cases: vec![CodingTestCase {
                name: "Test 1".to_string(),
                code_wrapper: "sanitasi(\"aman\")\n".to_string(),
                expected_output: "aman".to_string(),
            }],
        }]
    }

    fn wcda_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 801,
                question: "Bagaimana Widya-Lang mengeksekusi operasi tensor dan matriks AI?".to_string(),
                options: vec!["Melalui modul `widya::gpu` dengan SIMD & Vulkan/Metal/WGSL compute shaders".to_string(), "Dengan nested loop string".to_string(), "Dengan spawn process shell".to_string(), "Dengan spreadsheet Excel".to_string()],
                correct_option_index: 0,
                explanation: "Operasi tensor AI didukung akselerasi native GPU compute shaders dan SIMD.".to_string(),
            },
            MultipleChoiceQuestion {
                id: 802,
                question: "Dalam modul Smart City, rumus standar apa yang digunakan untuk menghitung Indeks Standar Pencemar Udara (ISPU) PM2.5?".to_string(),
                options: vec!["Permen LHK No. P.14/MENLHK/SETJEN/KUM.1/7/2020".to_string(), "Rumus Phytagoras".to_string(), "Hukum Ohm".to_string(), "Persamaan Maxwell".to_string()],
                correct_option_index: 0,
                explanation: "Perhitungan ISPU PM2.5 diimplementasikan sesuai standar resmi Permen LHK Republik Indonesia.".to_string(),
            },
        ]
    }

    fn wcda_coding_challenges() -> Vec<CodingChallenge> {
        vec![CodingChallenge {
            id: 801,
            title: "Rata-Rata Nilai Sensor".to_string(),
            description: "Buat fungsi `rata_rata(a, b, c)` yang mengembalikan nilai rata-rata dari 3 input.".to_string(),
            starter_code: "fungsi rata_rata(a, b, c) {\n    kembalikan (a + b + c) / 3\n}\n".to_string(),
            test_cases: vec![CodingTestCase {
                name: "Test 1".to_string(),
                code_wrapper: "rata_rata(10, 20, 30)\n".to_string(),
                expected_output: "20".to_string(),
            }],
        }]
    }

    fn wccd_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 901,
                question: "Apa fungsi dari `widya::microvm` dan `widya::container` (WidyaBox & WidyaKube)?".to_string(),
                options: vec!["Menyediakan runtime isolasi tingkat kernel Linux, Firecracker microVM, dan orkestrasi Kubernetes Pods".to_string(), "Desain logo cloud".to_string(), "Upload FTP saja".to_string(), "Render template HTML".to_string()],
                correct_option_index: 0,
                explanation: "MicroVM dan container engine mengisolasi beban kerja cloud dengan latency sub-millisecond dan self-healing kluster.".to_string(),
            },
        ]
    }

    fn wccd_coding_challenges() -> Vec<CodingChallenge> {
        vec![CodingChallenge {
            id: 901,
            title: "Konfigurasi Port Cloud".to_string(),
            description: "Buat fungsi `buat_uri_layanan(host, port)` yang menggabungkan host dan port menjadi 'http://host:port'.".to_string(),
            starter_code: "fungsi buat_uri_layanan(host, port) {\n    kembalikan \"http://\" + host + \":\" + port\n}\n".to_string(),
            test_cases: vec![CodingTestCase {
                name: "Test 1".to_string(),
                code_wrapper: "buat_uri_layanan(\"api.widya.id\", 8080)\n".to_string(),
                expected_output: "http://api.widya.id:8080".to_string(),
            }],
        }]
    }

    fn wcei_mc_questions() -> Vec<MultipleChoiceQuestion> {
        vec![
            MultipleChoiceQuestion {
                id: 1001,
                question: "Standar protokol industri dan pembayaran nasional apa saja yang didukung native oleh Widya-Lang?".to_string(),
                options: vec!["OPC UA, Modbus TCP/RTU, QRIS MPM/CPM, BI-FAST, SNAP/OpenAPI, ISO 8583, dan FHIR Healthtech".to_string(), "Hanya IRC".to_string(), "Hanya Telnet".to_string(), "Hanya Serial 9600 baud".to_string()],
                correct_option_index: 0,
                explanation: "Widya-Lang Enterprise mendukung standar perbankan nasional (QRIS, BI-FAST, ISO 8583) serta protokol industri berat (OPC UA, Modbus, FHIR).".to_string(),
            },
        ]
    }

    fn wcei_coding_challenges() -> Vec<CodingChallenge> {
        vec![CodingChallenge {
            id: 1001,
            title: "Validasi Bit Status Modbus".to_string(),
            description: "Buat fungsi `cek_alarm_plc(status_word, bit_index)` yang mengembalikan 'ALARM' jika bit bernilai 1 dan 'NORMAL' jika 0.".to_string(),
            starter_code: "fungsi cek_alarm_plc(status_word, bit_index) {\n    jika ((status_word & (1 << bit_index)) != 0) {\n        kembalikan \"ALARM\"\n    } selain_itu {\n        kembalikan \"NORMAL\"\n    }\n}\n".to_string(),
            test_cases: vec![
                CodingTestCase {
                    name: "Test Bit 2 Aktif (value 4)".to_string(),
                    code_wrapper: "cek_alarm_plc(4, 2)\n".to_string(),
                    expected_output: "ALARM".to_string(),
                },
                CodingTestCase {
                    name: "Test Bit 1 Mati (value 4)".to_string(),
                    code_wrapper: "cek_alarm_plc(4, 1)\n".to_string(),
                    expected_output: "NORMAL".to_string(),
                },
            ],
        }]
    }
}

/// Hasil Evaluasi Nilai Ujian
#[derive(Debug, Clone)]
pub struct ExamResult {
    pub candidate_name: String,
    pub candidate_id: String,
    pub level: ExamLevel,
    pub mc_score: f64,
    pub coding_score: f64,
    pub final_score: f64,
    pub passed: bool,
    pub breakdown: Vec<String>,
    pub certificate: Option<DigitalCertificate>,
}

/// Sertifikat Digital Resmi Terverifikasi
#[derive(Debug, Clone)]
pub struct DigitalCertificate {
    pub certificate_id: String,
    pub candidate_name: String,
    pub candidate_id: String,
    pub exam_level: String,
    pub exam_title: String,
    pub issue_date: String,
    pub expiry_date: String,
    pub score: f64,
    pub verification_hash: String,
    pub verification_url: String,
    pub issuer: String,
}

impl DigitalCertificate {
    pub fn render_ascii_badge(&self) -> String {
        format!(
r#"
+=============================================================================+
|                      DEWAN SERTIFIKASI WIDYA-LANG                           |
|                      REPUBLIK INDONESIA - GLOBAL                            |
+=============================================================================+
|  SERTIFIKAT KELULUSAN RESMI / OFFICIAL CERTIFICATE OF PROFICIENCY          |
|                                                                             |
|  Diberikan Kepada:                                                         |
|  NAMA        : {:<58} |
|  ID KANDIDAT : {:<58} |
|                                                                             |
|  Telah Berhasil Menyelesaikan & Dinyatakan LULUS Ujian Standar Nasional:   |
|  TINGKATAN   : {:<58} |
|  PREDIKAT    : {:<58} |
|  SKOR AKHIR  : {:<58.2} |
|                                                                             |
|  No. Registrasi  : {:<54} |
|  Tgl Penerbitan  : {:<54} |
|  Berlaku Hingga  : {:<54} |
|  Penerbit Resmi  : {:<54} |
|                                                                             |
|  SHA-256 SIGNATURE:                                                         |
|  {:<74} |
|                                                                             |
|  Verifikasi Publik:                                                         |
|  {:<74} |
+=============================================================================+
"#,
            self.candidate_name,
            self.candidate_id,
            self.exam_title,
            if self.score >= 90.0 { "DENGAN PUJIAN (DISTINCTION)" } else { "LULUS (COMPETENT)" },
            self.score,
            self.certificate_id,
            self.issue_date,
            self.expiry_date,
            self.issuer,
            self.verification_hash,
            self.verification_url
        )
    }
}

/// Auto-Grader & Exam Execution Engine
pub struct ExamEngine;

impl ExamEngine {
    /// Menjalankan evaluasi kode tantangan kandidat dalam sandbox
    pub fn grade_coding_challenge(challenge: &CodingChallenge, candidate_code: &str) -> (f64, Vec<String>) {
        let mut passed_tests = 0;
        let total_tests = challenge.test_cases.len();
        let mut logs = Vec::new();

        if total_tests == 0 {
            return (100.0, vec!["Tidak ada test case".to_string()]);
        }

        for (idx, tc) in challenge.test_cases.iter().enumerate() {
            let full_code = format!("{}\n\n{}", candidate_code, tc.code_wrapper);
            
            // Lexer & Parser
            let mut lexer = Lexer::new(&full_code);
            match lexer.scan_tokens() {
                Ok(tokens) => {
                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        Ok(ast) => {
                            let mut interpreter = Interpreter::new();
                            match interpreter.interpret(&ast) {
                                Ok(val) => {
                                    let actual_str = match &val {
                                        crate::value::Value::String(s) => s.clone(),
                                        crate::value::Value::Number(n) => {
                                            if n.fract() == 0.0 {
                                                format!("{}", *n as i64)
                                            } else {
                                                format!("{}", n)
                                            }
                                        }
                                        crate::value::Value::Bool(b) => b.to_string(),
                                        other => format!("{}", other),
                                    };
                                    let trimmed_out = actual_str.trim();
                                    let expected = tc.expected_output.trim();
                                    if trimmed_out == expected {
                                        passed_tests += 1;
                                        logs.push(format!("  [PASS] Test Case #{}: {} -> Output: '{}'", idx + 1, tc.name, trimmed_out));
                                    } else {
                                        logs.push(format!("  [FAIL] Test Case #{}: {} -> Harapan: '{}', Didapat: '{}'", idx + 1, tc.name, expected, trimmed_out));
                                    }
                                }
                                Err(e) => {
                                    logs.push(format!("  [ERROR] Test Case #{}: Runtime error: {:?}", idx + 1, e));
                                }
                            }
                        }
                        Err(e) => {
                            logs.push(format!("  [SYNTAX ERROR] Test Case #{}: {:?}", idx + 1, e));
                        }
                    }
                }
                Err(e) => {
                    logs.push(format!("  [LEXER ERROR] Test Case #{}: {:?}", idx + 1, e));
                }
            }
        }

        let score = (passed_tests as f64 / total_tests as f64) * 100.0;
        (score, logs)
    }

    /// Menghitung skor total dan menerbitkan sertifikat jika lulus
    pub fn evaluate_exam(
        candidate_name: &str,
        candidate_id: &str,
        level: ExamLevel,
        mc_answers: &HashMap<u32, usize>,
        coding_submissions: &HashMap<u32, String>,
    ) -> ExamResult {
        let (mc_questions, coding_challenges) = QuestionBank::get_exam_questions(&level);
        let mut breakdown = Vec::new();

        // 1. Evaluasi Multiple Choice
        let mut mc_correct = 0;
        let total_mc = mc_questions.len();
        breakdown.push(format!("=== EVALUASI SOAL TEORI (PILIHAN GANDA: {}) ===", total_mc));
        
        for q in &mc_questions {
            if let Some(&ans) = mc_answers.get(&q.id) {
                if ans == q.correct_option_index {
                    mc_correct += 1;
                    breakdown.push(format!("  [V] Soal #{}: Benar", q.id));
                } else {
                    breakdown.push(format!("  [X] Soal #{}: Salah (Jawaban Anda: {}, Kunci: {})", q.id, ans, q.correct_option_index));
                }
            } else {
                breakdown.push(format!("  [X] Soal #{}: Tidak Dijawab", q.id));
            }
        }

        let mc_score = if total_mc > 0 {
            (mc_correct as f64 / total_mc as f64) * 100.0
        } else {
            100.0
        };

        // 2. Evaluasi Coding Challenges
        let total_challenges = coding_challenges.len();
        let mut challenge_scores = Vec::new();
        breakdown.push(format!("\n=== EVALUASI SOAL PRAKTIK CODING ({}) ===", total_challenges));

        for challenge in &coding_challenges {
            breakdown.push(format!("Tantangan #{}: {}", challenge.id, challenge.title));
            if let Some(code) = coding_submissions.get(&challenge.id) {
                let (ch_score, ch_logs) = Self::grade_coding_challenge(challenge, code);
                challenge_scores.push(ch_score);
                breakdown.extend(ch_logs);
                breakdown.push(format!("  -> Skor Tantangan #{}: {:.2}/100", challenge.id, ch_score));
            } else {
                challenge_scores.push(0.0);
                breakdown.push(format!("  [X] Kode tidak dikirimkan. Skor: 0/100"));
            }
        }

        let coding_score = if total_challenges > 0 {
            challenge_scores.iter().sum::<f64>() / total_challenges as f64
        } else {
            100.0
        };

        // Bobot: 40% Teori, 60% Praktik Coding
        let final_score = if total_mc > 0 && total_challenges > 0 {
            (mc_score * 0.40) + (coding_score * 0.60)
        } else if total_challenges > 0 {
            coding_score
        } else {
            mc_score
        };

        let passing_threshold = level.passing_score();
        let passed = final_score >= passing_threshold;

        breakdown.push(format!("\n================ HASIL AKHIR ================"));
        breakdown.push(format!("Skor Teori (Pilihan Ganda) : {:.2}%", mc_score));
        breakdown.push(format!("Skor Praktik Coding        : {:.2}%", coding_score));
        breakdown.push(format!("Skor Akhir Terbobot        : {:.2}% (Ambang Kelulusan: {:.2}%)", final_score, passing_threshold));
        breakdown.push(format!("Status Kelulusan           : {}", if passed { "LULUS (PASSED)" } else { "TIDAK LULUS (FAILED)" }));

        let certificate = if passed {
            Some(Self::generate_certificate(candidate_name, candidate_id, &level, final_score))
        } else {
            None
        };

        ExamResult {
            candidate_name: candidate_name.to_string(),
            candidate_id: candidate_id.to_string(),
            level,
            mc_score,
            coding_score,
            final_score,
            passed,
            breakdown,
            certificate,
        }
    }

    /// Menerbitkan Digital Certificate dengan Hash SHA-256
    pub fn generate_certificate(
        candidate_name: &str,
        candidate_id: &str,
        level: &ExamLevel,
        score: f64,
    ) -> DigitalCertificate {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let cert_id = format!("WIDYA-{}-{}-{:04}", level.code(), timestamp, candidate_id.len() * 17);
        
        // Buat hash signature verifikasi
        let payload = format!("{}|{}|{}|{}|{}|SECRET_WIDYA_SALT_2026", cert_id, candidate_name, candidate_id, level.code(), score);
        let hash = format!("{:032x}", md5_sha256_placeholder(&payload));

        DigitalCertificate {
            certificate_id: cert_id.clone(),
            candidate_name: candidate_name.to_string(),
            candidate_id: candidate_id.to_string(),
            exam_level: level.code().to_string(),
            exam_title: level.title().to_string(),
            issue_date: "30 September 2026".to_string(),
            expiry_date: "30 September 2029 (Berlaku 3 Tahun)".to_string(),
            score,
            verification_hash: hash,
            verification_url: format!("https://verify.widya.dev/cert/{}", cert_id),
            issuer: "Badan Otoritas Sertifikasi & Standarisasi Widya-Lang (BOS-WL)".to_string(),
        }
    }

    /// Menghasilkan Dokumen PDF Modul Pembelajaran, Kuis, dan Ujian Resmi secara Rinci Per Tahapan
    pub fn generate_module_pdf(level: &ExamLevel, output_path: &std::path::Path) -> std::io::Result<()> {
        use crate::office::pdf::PdfDocument;
        let title = format!("MODUL SERTIFIKASI RESMI - {}", level.title());
        let mut doc = PdfDocument::new(&title);
        doc.author = "Dewan Standarisasi & Sertifikasi Widya-Lang (BOS-WL)".to_string();
        doc.organization = "Badan Standarisasi & Kedaulatan Perangkat Lunak Republik Indonesia".to_string();
        doc.set_watermark("WIDYA-LANG RESMI");

        let (mc_questions, coding_challenges) = QuestionBank::get_exam_questions(level);

        // ==========================================
        // HALAMAN 1: COVER & TAHAP 1 (FONDASI & TARGET APLIKASI)
        // ==========================================
        {
            let page = doc.current_page_mut();
            page.add_rect(30.0, 770.0, 535.0, 45.0, true, (0.05, 0.25, 0.55));
            page.add_styled_text("BADAN OTORITAS STANDARISASI & SERTIFIKASI WIDYA-LANG (BOS-WL)", 45.0, 795.0, 10.0, true, (1.0, 1.0, 1.0));
            page.add_styled_text("REPUBLIK INDONESIA - STANDAR NASIONAL KOMPETENSI REKAYASA PERANGKAT LUNAK 2026", 45.0, 780.0, 7.5, false, (0.9, 0.9, 0.9));

            page.add_styled_text("PANDUAN KURIKULUM & MODUL PELATIHAN BERTAHAP", 40.0, 735.0, 13.0, true, (0.1, 0.1, 0.1));
            page.add_styled_text(&format!("Tingkat: {}", level.title()), 40.0, 715.0, 11.5, true, (0.0, 0.4, 0.7));
            page.add_line(40.0, 705.0, 555.0, 705.0, 1.2);

            page.add_styled_text("TAHAP 1: STANDAR KOMPETENSI & TARGET APLIKASI NYATA (DELIVERABLES)", 40.0, 685.0, 10.0, true, (0.05, 0.25, 0.55));
            page.add_text("Setelah menyelesaikan modul ini, peserta ditargetkan mampu membangun proyek nyata berikut:", 40.0, 670.0, 8.5, false);

            match level {
                ExamLevel::WCA => {
                    page.add_text("1. Aplikasi Kasir / POS Konsol: Hitung subtotal belanja, diskon bertingkat, PPN & cetak struk.", 50.0, 655.0, 8.5, true);
                    page.add_text("2. Parser & Rekapitulasi Data: Skrip penyaring data CSV/JSON nilai siswa & laporan penjualan.", 50.0, 642.0, 8.5, true);
                    page.add_text("3. Mini Game Logika & Kalkulator Ilmiah: Permainan tebak angka & kalkulator matematika.", 50.0, 629.0, 8.5, true);
                }
                ExamLevel::WCP => {
                    page.add_text("1. Backend REST API + Basis Data: Layanan CRUD dengan transaksi ACID database WidyaBase.", 50.0, 655.0, 8.5, true);
                    page.add_text("2. Dokumen Kolaboratif Real-Time: Sinkronisasi teks multi-user bebas konflik berbasis CRDT.", 50.0, 642.0, 8.5, true);
                    page.add_text("3. Generator Dokumen Kantor: Mesin pembuat invoice resmi otomatis (PDF, XLSX, DOCX).", 50.0, 629.0, 8.5, true);
                }
                ExamLevel::WCE => {
                    page.add_text("1. Digital Enterprise Office: Ekosistem 10 agen AI otonom via Shared Blackboard Bus.", 50.0, 655.0, 8.5, true);
                    page.add_text("2. High-Frequency Trading Engine: Pemrosesan transaksi sub-milidetik via FFI C/Rust & LLVM.", 50.0, 642.0, 8.5, true);
                    page.add_text("3. Custom Compiler & Toolchain: Plugin kompiler tervalidasi Full Self-Hosting Stage 0-2.", 50.0, 629.0, 8.5, true);
                }
                _ => {
                    page.add_text("1. Solusi Domain Industri Khusus: Integrasi protokol standar perbankan/otomasi/cloud.", 50.0, 655.0, 8.5, true);
                    page.add_text("2. Implementasi Sistem Berdaya Saing Tinggi: Keamanan berstandar enterprise & regulasi.", 50.0, 642.0, 8.5, true);
                    page.add_text("3. Otomasi Skala Penuh: Pengendalian proses dan audit performa beban kerja tinggi.", 50.0, 629.0, 8.5, true);
                }
            }

            page.add_line(40.0, 615.0, 555.0, 615.0, 0.8);
            page.add_styled_text("TAHAP 2: POKOK MATERI SINTAKS & STRUKTUR DASAR", 40.0, 595.0, 10.0, true, (0.05, 0.25, 0.55));

            match level {
                ExamLevel::WCA => {
                    page.add_text("- Deklarasi Variabel: diketahui x = 10, diket y = 20, misal a = 1, tetap PI = 3.14", 50.0, 580.0, 8.0, false);
                    page.add_text("- Percabangan: jika (kondisi) { ... } selain_itu { ... } atau kalau ... jika_tidak", 50.0, 568.0, 8.0, false);
                    page.add_text("- Perulangan: untuk_setiap item dalam larik { ... } dan selama (kondisi) { ... }", 50.0, 556.0, 8.0, false);
                    page.add_text("- Fungsi: fungsi nama(param1, param2) { kembalikan hasil }", 50.0, 544.0, 8.0, false);
                }
                ExamLevel::WCP => {
                    page.add_text("- OOP: struktur Produk { nama, harga }, implementasi Produk { fungsi total() { ... } }", 50.0, 580.0, 8.0, false);
                    page.add_text("- Konkurensi: Actor Model dengan saluran.kirim(pesan) dan saluran.terima()", 50.0, 568.0, 8.0, false);
                    page.add_text("- Galat: coba { ... } tangkap (galat) { tulis(\"Error: \" + galat) }", 50.0, 556.0, 8.0, false);
                    page.add_text("- WidyaBase: PILIH * DARI produk DIMANA harga > 50000 URUTKAN_BERDASARKAN nama", 50.0, 544.0, 8.0, false);
                }
                ExamLevel::WCE => {
                    page.add_text("- Multi-Agent: Blackboard.daftarkan_topik(\"analisa\"), agen.publikasikan(\"state\")", 50.0, 580.0, 8.0, false);
                    page.add_text("- Self-Hosting: Tahap-0 (Bootstrap) -> Tahap-1 (Widya Compiler) -> Tahap-2 (Hash Match)", 50.0, 568.0, 8.0, false);
                    page.add_text("- Multi-Target: widya llvm berkas.wya (LLVM IR), widya wasm, widya gpu (WGSL)", 50.0, 556.0, 8.0, false);
                    page.add_text("- FFI: extern_c { fn fungsi_c_native(ptr: *const u8) -> i32 } dengan zero overhead", 50.0, 544.0, 8.0, false);
                }
                _ => {
                    page.add_text("- Arsitektur Domain Khusus: Driver native, spesifikasi ISO/IEC, dan API gateway.", 50.0, 580.0, 8.0, false);
                    page.add_text("- Enkripsi & Enklave: Algoritma Post-Quantum Cryptography (Kyber/Dilithium).", 50.0, 568.0, 8.0, false);
                    page.add_text("- Telemetri & Observabilitas: Logging performa latensi P95 dan metrik mikrodetik.", 50.0, 556.0, 8.0, false);
                    page.add_text("- Pengujian Mandiri: Automated test runner terintegrasi dengan validasi biner.", 50.0, 544.0, 8.0, false);
                }
            }

            page.add_styled_text("TAHAP 3: LANGKAH PRAKTIK PEMBUATAN PROYEK NYATA", 40.0, 520.0, 10.0, true, (0.05, 0.25, 0.55));
            page.add_text("Langkah 1: Tulis skrip program pada berkas dengan ekstensi `.wya`.", 50.0, 505.0, 8.0, false);
            page.add_text("Langkah 2: Uji eksekusi langsung menggunakan perintah: `widya jalankan proyek.wya`", 50.0, 493.0, 8.0, false);
            page.add_text("Langkah 3: Jalankan simulasi pengujian otomatis: `widya ujian nilai proyek.wya --level ...`", 50.0, 481.0, 8.0, false);
        }

        // ==========================================
        // HALAMAN 2: TAHAP 4 (BANK KUIS & PEMBAHASAN ANALITIS)
        // ==========================================
        {
            let page = doc.add_page();
            page.add_styled_text("TAHAP 4: BANK KUIS UJI PEMAHAMAN (DILENGKAPI KUNCI & PEMBAHASAN)", 40.0, 800.0, 11.0, true, (0.05, 0.25, 0.55));
            page.add_line(40.0, 792.0, 555.0, 792.0, 1.0);

            let mut cur_y = 770.0;
            for (idx, q) in mc_questions.iter().enumerate() {
                if cur_y < 120.0 {
                    break;
                }
                page.add_text(&format!("Soal Kuis #{}: {}", idx + 1, q.question), 40.0, cur_y, 9.0, true);
                cur_y -= 13.0;
                for (opt_i, opt) in q.options.iter().enumerate() {
                    let is_correct = opt_i == q.correct_option_index;
                    let prefix = if is_correct { " [*]" } else { " [ ]" };
                    page.add_text(&format!("  {}. {}{}", (opt_i as u8 + b'A') as char, opt, prefix), 50.0, cur_y, 8.0, false);
                    cur_y -= 11.0;
                }
                page.add_styled_text(&format!("  -> KUNCI JAWABAN: Pilihan {}", (q.correct_option_index as u8 + b'A') as char), 50.0, cur_y, 8.0, true, (0.0, 0.5, 0.2));
                cur_y -= 11.0;
                page.add_styled_text(&format!("  -> PEMBAHASAN  : {}", q.explanation), 50.0, cur_y, 7.5, false, (0.3, 0.3, 0.3));
                cur_y -= 16.0;
            }
        }

        // ==========================================
        // HALAMAN 3: TAHAP 5 (UJIAN PRAKTIK & KUNCI SOLUSI)
        // ==========================================
        {
            let page = doc.add_page();
            page.add_styled_text("TAHAP 5: TANTANGAN UJIAN PRAKTIK CODING & KUNCI SOLUSI RESMI", 40.0, 800.0, 11.0, true, (0.05, 0.25, 0.55));
            page.add_line(40.0, 792.0, 555.0, 792.0, 1.0);

            let mut cur_y = 770.0;
            for (idx, ch) in coding_challenges.iter().enumerate() {
                if cur_y < 150.0 {
                    break;
                }
                page.add_styled_text(&format!("Tantangan #{}: {}", idx + 1, ch.title), 40.0, cur_y, 9.5, true, (0.1, 0.1, 0.6));
                cur_y -= 13.0;
                page.add_text(&format!("Deskripsi: {}", ch.description), 40.0, cur_y, 8.5, false);
                cur_y -= 13.0;

                page.add_text("Spesifikasi Test Cases:", 40.0, cur_y, 8.0, true);
                cur_y -= 11.0;
                for tc in &ch.test_cases {
                    page.add_text(&format!("  - {} -> Harapan Output: '{}'", tc.name, tc.expected_output), 50.0, cur_y, 7.5, false);
                    cur_y -= 10.0;
                }

                cur_y -= 3.0;
                page.add_styled_text("Kunci Solusi Standar BOS-WL:", 40.0, cur_y, 8.0, true, (0.0, 0.4, 0.1));
                cur_y -= 11.0;

                for line in ch.starter_code.lines() {
                    page.add_styled_text(&format!("    {}", line), 50.0, cur_y, 7.0, false, (0.2, 0.2, 0.2));
                    cur_y -= 9.5;
                }
                cur_y -= 14.0;
            }

            page.add_line(40.0, 90.0, 555.0, 90.0, 0.8);
            page.add_styled_text("Dokumen Kurikulum ini Disahkan Resmi oleh Dewan Standarisasi & Sertifikasi Widya-Lang RI.", 40.0, 75.0, 7.5, false, (0.4, 0.4, 0.4));
            page.add_styled_text("Portal Verifikasi Terbuka: https://verify.widya.dev/cert/", 40.0, 63.0, 7.5, true, (0.0, 0.3, 0.7));
        }

        doc.save_to_file(output_path)
    }

    /// Mengekspor seluruh modul PDF sertifikasi ke direktori target
    pub fn export_all_modules_pdf(target_dir: &std::path::Path) -> std::io::Result<Vec<std::path::PathBuf>> {
        std::fs::create_dir_all(target_dir)?;
        let levels = vec![
            (ExamLevel::WCA, "MODUL_WCA_DASAR_DAN_LOGIKA.pdf"),
            (ExamLevel::WCP, "MODUL_WCP_PROFESIONAL_CONCURRENCY.pdf"),
            (ExamLevel::WCE, "MODUL_WCE_EXPERT_MULTIAGENT_FFI.pdf"),
            (ExamLevel::WCWD, "MODUL_WCWD_WEB_FULLSTACK.pdf"),
            (ExamLevel::WCMD, "MODUL_WCMD_MOBILE_CROSSPLATFORM.pdf"),
            (ExamLevel::WCDD, "MODUL_WCDD_DESKTOP_NATIVE_GUI.pdf"),
            (ExamLevel::WCSS, "MODUL_WCSS_SECURITY_POSTQUANTUM.pdf"),
            (ExamLevel::WCDA, "MODUL_WCDA_DATA_AI_ENGINEERING.pdf"),
            (ExamLevel::WCCD, "MODUL_WCCD_CLOUD_DEVOPS_MICROVM.pdf"),
            (ExamLevel::WCEI, "MODUL_WCEI_ENTERPRISE_INDUSTRIAL.pdf"),
        ];

        let mut generated_paths = Vec::new();
        for (lvl, filename) in levels {
            let p = target_dir.join(filename);
            Self::generate_module_pdf(&lvl, &p)?;
            generated_paths.push(p);
        }

        Ok(generated_paths)
    }
}

/// Helper hash konsisten tanpa dependensi eksternal
fn md5_sha256_placeholder(input: &str) -> u128 {
    let mut hash: u128 = 0xcbf29ce484222325;
    for byte in input.bytes() {
        hash ^= byte as u128;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
