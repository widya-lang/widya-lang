//! Widya-Lang Interactive Tutor & Guided Learning Engine (learn.rs)
//! Menyediakan kurikulum interaktif di terminal dengan modul pembelajaran bertingkat,
//! kuis pemahaman, dan tantangan coding terpandu.

use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonChapter {
    pub id: usize,
    pub title: String,
    pub level: String,
    pub concept: String,
    pub sample_code: String,
    pub explanation: String,
    pub challenge: String,
    pub expected_output: String,
}

pub struct TutorialEngine;

impl TutorialEngine {
    pub fn get_curriculum() -> Vec<LessonChapter> {
        vec![
            LessonChapter {
                id: 1,
                title: "Sintaks Natural & Variabel Imutabel/Mutabel".to_string(),
                level: "Dasar (Beginner)".to_string(),
                concept: "Penggunaan kata kunci 'misal', tipe data primitif, dan immutability secara default.".to_string(),
                sample_code: "misal nama = \"Indonesia\";\nmisal mutabel saldo = 100000;\nsaldo = saldo + 50000;\ncetak(\"Saldo: \" + teks(saldo));".to_string(),
                explanation: "Di Widya-Lang, semua variabel secara default bersifat imutabel untuk menjamin konkurensi aman bebas data race.".to_string(),
                challenge: "Deklarasikan variabel 'nama_koperasi' dan 'modal_awal', lalu cetak keduanya.".to_string(),
                expected_output: "Koperasi Merah Putih: Rp 50000000".to_string(),
            },
            LessonChapter {
                id: 2,
                title: "Fungsi, Parameter Tipe, & Kontrol Alur (jika/lain)".to_string(),
                level: "Dasar (Beginner)".to_string(),
                concept: "Struktur fungsi dengan tipe data eksplisit, ekspresi 'jika' dan 'kembalikan'.".to_string(),
                sample_code: "fungsi cek_kelayakan(skor: Int) -> Bool {\n    jika skor >= 75 {\n        kembalikan true;\n    }\n    kembalikan false;\n}".to_string(),
                explanation: "Fungsi didefinisikan dengan 'fungsi' dan tipe kembalian '-> Tipe'.".to_string(),
                challenge: "Buat fungsi kalkulasi diskon belanja koperasi berdasarkan persentase.".to_string(),
                expected_output: "Total Diskon: Rp 15000".to_string(),
            },
            LessonChapter {
                id: 3,
                title: "Struktur Data (Struct) & Metode Entitas".to_string(),
                level: "Menengah (Intermediate)".to_string(),
                concept: "Pengelompokan data terstruktur dengan 'struktur' dan validasi skema.".to_string(),
                sample_code: "struktur Anggota {\n    id: Int,\n    nama: String,\n    saldo: Int\n}".to_string(),
                explanation: "Struktur memungkinkan pemodelan domain bisnis dengan jaminan tipe data statis.".to_string(),
                challenge: "Buat struktur TransaksiQRIS dengan atribut merchant, nominal, dan status.".to_string(),
                expected_output: "QRIS Berhasil: Rp 25000 ke Warung Desa".to_string(),
            },
            LessonChapter {
                id: 4,
                title: "Pattern Matching Exhaustif & Algebraic Data Types (ADT)".to_string(),
                level: "Menengah (Intermediate)".to_string(),
                concept: "Pencocokan pola menggunakan 'cocokkan' dengan guard condition exhaustiveness.".to_string(),
                sample_code: "cocokkan status {\n    Status::Sukses => cetak(\"Transaksi Berhasil\"),\n    Status::Gagal(alasan) => cetak(\"Galat: \" + alasan),\n}".to_string(),
                explanation: "Pattern matching Widya-Lang menjamin seluruh varian tertangani tanpa risiko unhandled case.".to_string(),
                challenge: "Tangani varian Result<Value, Error> dari pembacaan basis data.".to_string(),
                expected_output: "Data ditemukan: 3 records".to_string(),
            },
            LessonChapter {
                id: 5,
                title: "Borrow Checker Statis, Zero-Copy & Aliasing Safety".to_string(),
                level: "Mahir (Advanced)".to_string(),
                concept: "Manajemen memori otomatis tanpa Garbage Collector menggunakan kepemilikan (ownership) dan peminjaman (borrowing).".to_string(),
                sample_code: "fungsi proses_data(ref buffer: Vector<Byte>) {\n    // Peminjaman imutabel zero-copy\n}".to_string(),
                explanation: "Borrow checker Widya memvalidasi masa hidup referensi (lifetimes) pada saat kompilasi dengan 0 byte runtime overhead.".to_string(),
                challenge: "Implementasikan pemrosesan payload data tanpa alokasi heap baru.".to_string(),
                expected_output: "Zero-Copy Check: PASSED (0 GC Pauses)".to_string(),
            },
            LessonChapter {
                id: 6,
                title: "Actor Model, Sovereign Blockchain & Post-Quantum Cryptography".to_string(),
                level: "Mahir (Advanced)".to_string(),
                concept: "Konkurensi pesan asinkron, smart contract berdaulat, dan enkripsi pasca-kuantum (ML-KEM/Kyber).".to_string(),
                sample_code: "aktor Kasir {\n    terima Pesan::Bayar(jumlah) => {\n        // Enkripsi PQC & Ledger append\n    }\n}".to_string(),
                explanation: "Arsitektur Actor Model memungkinkan komunikasi aman antar-agen AI dan sistem terdistribusi.".to_string(),
                challenge: "Buat aktor validator blok transaksi yang menandatangani payload dengan kunci PQC.".to_string(),
                expected_output: "Block #1002 Mined: Signature PQC-ML-DSA Verified".to_string(),
            },
        ]
    }

    pub fn display_curriculum() {
        let list = Self::get_curriculum();
        println!("\n{}", "================================================================================".bright_blue());
        println!("{}", "       🎓  AKADEMI & KURIKULUM INTERAKTIF WIDYA-LANG (WIDYA-TUTOR)           ".bright_cyan().bold());
        println!("{}", "================================================================================".bright_blue());
        println!("  Jalur Belajar Terstruktur (Interactive Chapters):\n");

        for ch in list {
            println!("  [{}] {} — {}", ch.id.to_string().bright_yellow().bold(), ch.title.bright_white().bold(), ch.level.bright_green());
            println!("      Konsep    : {}", ch.concept.bright_black());
            println!("      Tantangan : {}", ch.challenge.bright_cyan());
            println!();
        }

        println!("{}", "--------------------------------------------------------------------------------".bright_blue());
        println!("  Mulai Pembelajaran:");
        println!("    widya learn <NOMOR_BAB>     (Contoh: widya learn 1)");
        println!("    widya tutor <NOMOR_BAB>     (Contoh: widya tutor 5)");
        println!("{}\n", "================================================================================".bright_blue());
    }

    pub fn run_chapter(chapter_id: usize) {
        let list = Self::get_curriculum();
        let chapter = match list.into_iter().find(|c| c.id == chapter_id) {
            Some(c) => c,
            None => {
                eprintln!("❌ Bab #{} tidak ditemukan. Ketik 'widya learn' untuk melihat daftar kurikulum.", chapter_id);
                return;
            }
        };

        println!("\n{}", "================================================================================".bright_blue());
        println!("{}", format!("  📘 BAB {}: {} [{}]", chapter.id, chapter.title, chapter.level).bright_cyan().bold());
        println!("{}", "================================================================================".bright_blue());
        println!("💡 KONSEP DASAR:");
        println!("   {}\n", chapter.concept.bright_white());
        println!("💻 CONTOH KODE SUMBER WIDYA:");
        println!("--------------------------------------------------------------------------------");
        for line in chapter.sample_code.lines() {
            println!("   {}", line.bright_yellow());
        }
        println!("--------------------------------------------------------------------------------");
        println!("📖 PENJELASAN:");
        println!("   {}\n", chapter.explanation);
        println!("🎯 TANTANGAN LATIHAN:");
        println!("   {}", chapter.challenge.bright_green().bold());
        println!("   Hasil yang diharapkan: {}", chapter.expected_output.bright_yellow());
        println!("{}", "================================================================================".bright_blue());
        println!("{}", "✅ Ketik kode Anda pada berkas .wya dan uji menggunakan 'widya jalankan <berkas.wya>'".bright_cyan());
        println!();
    }
}
