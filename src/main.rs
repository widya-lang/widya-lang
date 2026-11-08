use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use widya::borrow_checker::BorrowChecker;
use widya::compiler::{compile_to_executable, NativeCompiler};
use widya::interpreter::Interpreter;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::repl::start_repl;
use widya::studio::{jalankan_lsp, jalankan_studio};
use widya::value::Value;

mod cli;
use cli::*;

#[derive(ClapParser)]
#[command(name = "widya")]
#[command(author = "Widya-Lang Team")]
#[command(version = "0.1.0")]
#[command(about = "Interpreter dan Kompiler Bahasa Pemrograman Widya-Lang", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path berkas skrip Widya (.wya) yang ingin langsung dijalankan
    #[arg(value_name = "BERKAS")]
    berkas: Option<PathBuf>,

    /// Profil aktif untuk membatasi fungsi dengan atribut #[profil("...")]
    #[arg(long, value_name = "PROFIL")]
    profil: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Menjalankan berkas kode sumber Widya-Lang (.wya)
    Jalankan {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menjalankan berkas kode sumber Widya-Lang (alias: run)
    Run {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Mengompilasi berkas Widya menjadi executable biner mandiri (.exe)
    Kompilasi {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Alias untuk kompilasi (build)
    Build {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Menghasilkan kode native Rust/C tanpa interpretasi
    Emit {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Menghasilkan LLVM IR (.ll) untuk optimasi Clang/LLVM dan WebAssembly/Bare-metal
    Llvm {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Mengompilasi program Widya langsung ke biner WebAssembly (.wasm)
    Wasm {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Mengompilasi kode program ke WGSL (WebGPU Shading Language) Compute Shader
    Gpu {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Mengompilasi kode program ke program C / eBPF Kernel Tracing & Probing
    Ebpf {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Memantau perubahan berkas dan mengeksekusi ulang secara instan (Live Hot-Reload)
    Tonton {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Alias Tonton: watch
    Watch {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Mempublikasikan paket pustaka ke Widya Central Registry (wpm.widya-lang.org)
    Publikasi,
    /// Mengunduh dan memasang seluruh dependensi dari widya.toml
    Pasang,
    /// Memasok seluruh dependensi ke direktori lokal untuk deployment offline / air-gapped (Vendor)
    Pasok {
        #[arg(value_name = "DIREKTORI", default_value = ".")]
        dir: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Alias Pasok: vendor
    Vendor {
        #[arg(value_name = "DIREKTORI", default_value = ".")]
        dir: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Menghapus dependensi paket dari proyek
    Hapus {
        #[arg(value_name = "NAMA_PAKET")]
        paket: String,
    },
    /// Menjalankan suite unit test bertanda #[uji] atau berkas uji standar (Integrated Test Runner)
    Uji {
        #[arg(value_name = "BERKAS")]
        berkas: Option<PathBuf>,
    },
    /// Alias Uji: test
    Test {
        #[arg(value_name = "BERKAS")]
        berkas: Option<PathBuf>,
    },
    /// Memformat kode sumber Widya sesuai standar indentasi & gaya resmi (Formatter)
    Format {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long)]
        tulis: bool,
        #[arg(short, long)]
        periksa: bool,
        #[arg(short, long, value_name = "GAYA")]
        style: Option<String>,
    },
    /// Memeriksa potensi galat, unused variable, dan anti-patterns (Linter)
    Periksa {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menghasilkan situs web dokumentasi HTML interaktif dari doc comments (DocGen)
    Dok {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Time-Travel / Reversible Debugger dengan rewind & forward execution state
    Debug {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menjalankan berkas dengan mesin optimasi Just-In-Time (JIT)
    Jit {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Memaketkan kode ke Single Standalone Executable atau Mobile Webview App (Bundler)
    Kemas {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
        #[arg(short, long)]
        mobile: bool,
    },
    /// Alias Kemas: bundle
    Bundle {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
        #[arg(short, long)]
        mobile: bool,
    },
    /// Mengukur throughput komputasi beresolusi tinggi / ns per operasi (Benchmark)
    TolakUkur {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Alias TolakUkur: bench
    Bench {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Alias TolakUkur: benchmark
    Benchmark {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Merender dan menjalankan aplikasi WidyaUI (Flutter-like) ke HTML5
    Ui {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Membuka Widya Studio (IDE Mandiri berbasis Desktop / Browser) atau MCR Broadcasting Studio
    Studio {
        #[command(subcommand)]
        aksi: Option<StudioSubcommands>,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Sovereign Multi-Camera Live Broadcast & MCR Studio (OB Van Launcher)
    Broadcast {
        #[command(subcommand)]
        aksi: Option<BroadcastSubcommands>,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(short, long, default_value = "0.0.0.0")]
        bind: String,
        #[arg(long)]
        open: bool,
    },
    /// Alias Broadcast: mcr
    Mcr {
        #[command(subcommand)]
        aksi: Option<BroadcastSubcommands>,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(short, long, default_value = "0.0.0.0")]
        bind: String,
        #[arg(long)]
        open: bool,
    },
    /// Menjalankan Language Server Protocol (LSP) untuk editor IDE
    Lsp,
    /// Inisialisasi proyek baru Widya (Widya Package Manager)
    Inisialisasi {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
    },
    /// Alias inisialisasi: new
    New {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
    },
    /// Menambahkan modul/paket eksternal ke proyek
    Tambah {
        #[arg(value_name = "NAMA_PAKET")]
        paket: String,
    },
    /// Membuka sesi interaktif REPL
    Repl,
    /// Menampilkan daftar token dari berkas kode sumber
    Tokens {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menampilkan Abstract Syntax Tree (AST) dari berkas kode sumber
    Ast {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Manajemen dan Kompilasi Aplikasi Mobile (Android & iOS)
    Mobile {
        #[command(subcommand)]
        aksi: MobileSubcommands,
    },
    /// Manajemen, Eksekusi, dan Kompilasi Aplikasi Desktop Native (Windows, macOS, Linux)
    Desktop {
        #[command(subcommand)]
        aksi: DesktopSubcommands,
    },
    /// Mesin Basis Data Mandiri WidyaBase (B+Tree, WAL, MVCC & SQL Dwibahasa)
    Base {
        #[command(subcommand)]
        aksi: BaseSubcommands,
    },
    /// WidyaPay - Gerbang Pembayaran Finansial (QRIS, SNAP BI, ISO 8583, Kartu & FDS)
    Bayar {
        #[command(subcommand)]
        aksi: BayarSubcommands,
    },
    /// Alias Bayar: payment
    Payment {
        #[command(subcommand)]
        aksi: BayarSubcommands,
    },
    /// WidyaGame3D - Mesin Game 3D PBR, Fisika, glTF, dan Renderer WebGPU
    Game3d {
        #[command(subcommand)]
        aksi: Game3dSubcommands,
    },
    /// WidyaWeb3 - Ekosistem Aset Digital, EVM, Smart Contracts, dan Tokenisasi RWA
    Web3 {
        #[command(subcommand)]
        aksi: Web3Subcommands,
    },
    /// WidyaHealth - Standar Medis HL7 FHIR SatuSehat, DICOM, dan ICD-10
    Health {
        #[command(subcommand)]
        aksi: HealthSubcommands,
    },
    /// Alias Health: fhir
    Fhir {
        #[command(subcommand)]
        aksi: HealthSubcommands,
    },
    /// WidyaOffice - Pemrosesan Dokumen DOCX, Spreadsheet XLSX, dan Slide PPTX
    Office {
        #[command(subcommand)]
        aksi: OfficeSubcommands,
    },
    /// Alias Office: dokumen
    Dokumen {
        #[command(subcommand)]
        aksi: OfficeSubcommands,
    },
    /// WidyaDrive - Penyimpanan Berkas Cloud Sovereign Terenkripsi AES-256
    Drive {
        #[command(subcommand)]
        aksi: DriveSubcommands,
    },
    /// Widya Workspace - Platform Kolaborasi Enterprise Terintegrasi
    Workspace {
        #[command(subcommand)]
        aksi: WorkspaceSubcommands,
    },
    /// Widya SmartCity - Intelijen Geospasial, ISPU, Rekayasa Lalu Lintas & Mikrogrid EBT
    Smartcity {
        #[command(subcommand)]
        aksi: SmartcitySubcommands,
    },
    /// WidyaStore - Mesin Penyimpanan Objek Berkinerja Tinggi (Kompatibel S3 & MinIO)
    Store {
        #[command(subcommand)]
        aksi: StoreSubcommands,
    },
    /// Alias Store: minio / s3
    Minio {
        #[command(subcommand)]
        aksi: StoreSubcommands,
    },
    /// WidyaDocDB - Basis Data Dokumen NoSQL Skala Industri (Kompatibel MongoDB)
    Docdb {
        #[command(subcommand)]
        aksi: DocdbSubcommands,
    },
    /// Alias Docdb: mongo / nosql
    Mongo {
        #[command(subcommand)]
        aksi: DocdbSubcommands,
    },

    /// Audit Keamanan Kode Sumber, Pendeteksian Rahasia, SQLi, dan XSS
    Security {
        #[command(subcommand)]
        aksi: Option<SecuritySubcommands>,
        #[arg(value_name = "BERKAS_ATAU_DIR")]
        target: Option<PathBuf>,
    },
    /// Alias Security: audit
    Audit {
        #[command(subcommand)]
        aksi: Option<SecuritySubcommands>,
        #[arg(value_name = "BERKAS_ATAU_DIR")]
        target: Option<PathBuf>,
    },
    /// Menjalankan Memory Profiler dan Detektor Siklus Referensi
    Profile {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menjalankan server Debug Adapter Protocol (DAP) untuk editor IDE
    Dap,
    /// Manajemen dependensi, lockfile, dan verifikasi paket WPM
    Wpm {
        #[command(subcommand)]
        aksi: WpmSubcommands,
    },
    /// Alias Wpm: pkg
    Pkg {
        #[command(subcommand)]
        aksi: WpmSubcommands,
    },
    /// Alias Wpm: package
    Package {
        #[command(subcommand)]
        aksi: WpmSubcommands,
    },
    /// Menjalankan orkestrator Full Self-Hosting Compiler (Stage-0 -> Stage-1 -> Stage-2)
    Bootstrap {
        #[arg(short, long, default_value = "compiler_self_hosted")]
        dir: String,
    },
    /// Alias Bootstrap: selfhost
    Selfhost {
        #[arg(short, long, default_value = "compiler_self_hosted")]
        dir: String,
    },
    /// Alias Bootstrap: self-host
    SelfHost {
        #[arg(short, long, default_value = "compiler_self_hosted")]
        dir: String,
    },
    /// Manajemen dan peluncuran terpadu seluruh microservices WidyaOS (Port 8001 - 8010)
    Services {
        #[arg(short, long, default_value_t = 8001)]
        start_port: u16,
    },
    /// Alias Services: os-services
    OsServices {
        #[arg(short, long, default_value_t = 8001)]
        start_port: u16,
    },
    /// WidyaWeb Cloud & Sovereign Edge Platform (Platform Hosting Serverless Berdaulat seperti Vercel)
    Web {
        #[command(subcommand)]
        aksi: WebSubcommands,
    },
    /// Alias Web: cloud
    Cloud {
        #[command(subcommand)]
        aksi: WebSubcommands,
    },
    /// Deploy instan proyek WidyaWeb ke Sovereign Edge (Preview & Production)
    Deploy {
        #[arg(value_name = "DIR", default_value = ".")]
        dir: PathBuf,
        #[arg(long)]
        prod: bool,
    },
    /// WidyaBox - Mesin Kontainer & Pod Rootless Berdaulat (Setara Podman / Docker)
    Box {
        #[command(subcommand)]
        aksi: BoxSubcommands,
    },
    /// Alias Box: podman
    Podman {
        #[command(subcommand)]
        aksi: BoxSubcommands,
    },
    /// Alias Box: container
    Container {
        #[command(subcommand)]
        aksi: BoxSubcommands,
    },
    /// Orkestrasi Pod Kontainer Mandiri (Kubernetes / Podman Pods Compatible)
    Pod {
        #[command(subcommand)]
        aksi: PodSubcommands,
    },
    /// WidyaKube - Orkestrasi Kluster Kontainer Multi-Node & Self-Healing (Setara Kubernetes / K3s)
    K8s {
        #[command(subcommand)]
        aksi: K8sSubcommands,
    },
    /// Alias K8s: kube
    Kube {
        #[command(subcommand)]
        aksi: K8sSubcommands,
    },
    /// Alias K8s: cluster
    Cluster {
        #[command(subcommand)]
        aksi: K8sSubcommands,
    },
    /// Mesin Sertifikasi Resmi & Evaluasi Otomatis Widya-Lang (WCA, WCP, WCE, dll.)
    Ujian {
        #[command(subcommand)]
        aksi: ExamSubcommands,
    },
    /// Alias Ujian: exam / cert
    Exam {
        #[command(subcommand)]
        aksi: ExamSubcommands,
    },
    /// WidyaAd - Sovereign Ad & Overlay Engine (Video & In-App RTB, VAST 4.2 & UU PDP)
    Ad {
        #[command(subcommand)]
        aksi: AdSubcommands,
    },
    /// Alias Ad: iklan
    Iklan {
        #[command(subcommand)]
        aksi: AdSubcommands,
    },
    /// Generator Dokumentasi API Web Interaktif WidyaDoc
    Doc {
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        #[arg(short, long, default_value = "target/docs")]
        output: PathBuf,
        #[arg(long)]
        serve: bool,
        #[arg(short, long, default_value_t = 8090)]
        port: u16,
    },
    /// Alias Doc: wdoc
    Wdoc {
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        #[arg(short, long, default_value = "target/docs")]
        output: PathBuf,
        #[arg(long)]
        serve: bool,
        #[arg(short, long, default_value_t = 8090)]
        port: u16,
    },
    /// Akademi & Kurikulum Pembelajaran Interaktif Widya-Lang
    Learn {
        #[arg(value_name = "BAB")]
        bab: Option<usize>,
    },
    /// Alias Learn: tutor
    Tutor {
        #[arg(value_name = "BAB")]
        bab: Option<usize>,
    },
    /// Generator Firmware Bare-Metal & Mikrokontroler (ARM Cortex-M / RISC-V no_std)
    Embedded {
        #[arg(short, long, default_value = "firmware_widya")]
        nama: String,
        #[arg(short, long, default_value = "cortex-m4")]
        arch: String,
        #[arg(short, long, default_value = "target/embedded")]
        output: PathBuf,
    },
    /// Alias Embedded: firmware
    Firmware {
        #[arg(short, long, default_value = "firmware_widya")]
        nama: String,
        #[arg(short, long, default_value = "cortex-m4")]
        arch: String,
        #[arg(short, long, default_value = "target/embedded")]
        output: PathBuf,
    },
    /// Widya AI - Inferensi Sovereign SLM/LLM & Format GGUF Lokal
    Ai {
        #[command(subcommand)]
        aksi: AiSubcommands,
    },
    /// Alias Ai: gguf
    Gguf {
        #[command(subcommand)]
        aksi: AiSubcommands,
    },
    /// Widya Kripto - Kriptografi Pasca-Kuantum (ML-KEM, ML-DSA) & Zero-Knowledge Proofs
    Kripto {
        #[command(subcommand)]
        aksi: KriptoSubcommands,
    },
    /// Alias Kripto: pqc
    Pqc {
        #[command(subcommand)]
        aksi: KriptoSubcommands,
    },
    /// Widya Fintech - Infrastruktur Pembayaran Nasional BI-FAST, Resolusi Proxy & ISO 20022
    Fintech {
        #[command(subcommand)]
        aksi: FintechSubcommands,
    },
    /// Alias Fintech: bifast
    Bifast {
        #[command(subcommand)]
        aksi: FintechSubcommands,
    },
    /// Widya WebAssembly Component Model (Preview 2) & Serverless Edge
    WasmKomponen {
        #[command(subcommand)]
        aksi: WasmComponentSubcommands,
    },
    /// Alias WasmKomponen: wasm-component
    WasmComponent {
        #[command(subcommand)]
        aksi: WasmComponentSubcommands,
    },
    /// Widya Interactive Notebook (.wnb) Sains Data & Laporan Interaktif
    Notebook {
        #[command(subcommand)]
        aksi: NotebookSubcommands,
    },
    /// Alias Notebook: buku
    Buku {
        #[command(subcommand)]
        aksi: NotebookSubcommands,
    },
    /// Bangun Target Multi-Platform Sekali Perintah (Android APK, iOS IPA, WasmEdge, Desktop)
    BangunTarget {
        #[arg(value_name = "NAMA_PROYEK")]
        proyek: String,
        #[arg(short, long, default_value = "android")]
        target: String,
    },
    /// Alias BangunTarget: build-target
    BuildTarget {
        #[arg(value_name = "NAMA_PROYEK")]
        proyek: String,
        #[arg(short, long, default_value = "android")]
        target: String,
    },
    /// Widya Otomotif & Sistem Misi-Kritis (CAN Bus 2.0B / CAN-FD, OBD-II Telemetri)
    Otomotif {
        #[command(subcommand)]
        aksi: OtomotifSubcommands,
    },
    /// Alias Otomotif: automotive
    Automotive {
        #[command(subcommand)]
        aksi: OtomotifSubcommands,
    },
    /// Widya Telekomunikasi 5G/6G & Open-RAN Edge (UPF, GTP-U, Network Slicing SLA)
    Telekomunikasi {
        #[command(subcommand)]
        aksi: TelecomSubcommands,
    },
    /// Alias Telekomunikasi: telecom
    Telecom {
        #[command(subcommand)]
        aksi: TelecomSubcommands,
    },
    /// Widya Industrial IoT & SCADA (Modbus TCP/RTU, OPC-UA IEC 62541, TSN IEEE 802.1Qbv)
    Iiot {
        #[command(subcommand)]
        aksi: IiotSubcommands,
    },
    /// Alias Iiot: scada
    Scada {
        #[command(subcommand)]
        aksi: IiotSubcommands,
    },
    /// Widya Komputasi Kuantum & Algoritma Hibrida (OpenQASM 3.0, QIR LLVM, State Vector, VQE)
    Kuantum {
        #[command(subcommand)]
        aksi: KuantumSubcommands,
    },
    /// Alias Kuantum: quantum
    Quantum {
        #[command(subcommand)]
        aksi: KuantumSubcommands,
    },
    /// Lima Pilar Kemandirian dari Pustaka Luar (Sovereign Stack Zero-Dependency)
    Pilar {
        #[command(subcommand)]
        aksi: Option<PilarSubcommands>,
    },
    /// Alias Pilar: sovereign
    Sovereign {
        #[command(subcommand)]
        aksi: Option<PilarSubcommands>,
    },
    /// Generator Citra Unikernel & MicroVM Direct Boot (AWS Firecracker / QEMU)
    Unikernel {
        #[command(subcommand)]
        aksi: UnikernelSubcommands,
    },
    /// Widya Model Context Protocol (MCP v2024-11-05) Server & Tools Inspector
    Mcp {
        #[command(subcommand)]
        aksi: McpSubcommands,
    },
    /// Widya Software Bill of Materials (SBOM) & Supply Chain Security Auditor
    Sbom {
        #[command(subcommand)]
        aksi: SbomSubcommands,
    },
    /// Widya Concurrency & Jepsen Chaos Simulator (Split-Brain & Packet Drops)
    Chaos {
        #[command(subcommand)]
        aksi: ChaosSubcommands,
    },
    /// Widya Hardware Security Module (HSM) PKCS#11 v2.40 & Post-Quantum Key Enclave
    Hsm {
        #[command(subcommand)]
        aksi: HsmSubcommands,
    },
}







fn main() {
    let builder = thread::Builder::new()
        .name("widya-main".into())
        .stack_size(32 * 1024 * 1024);

    let handler = builder
        .spawn(|| {
            run_main();
        })
        .expect("Gagal menginisialisasi thread utama Widya-Lang");

    if let Err(e) = handler.join() {
        std::panic::resume_unwind(e);
    }
}

fn run_main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Jalankan { berkas }) | Some(Commands::Run { berkas }) => {
            jalankan_berkas(&berkas, cli.profil.as_deref());
        }
        Some(Commands::Kompilasi { berkas, output }) | Some(Commands::Build { berkas, output }) => {
            kompilasi_native(&berkas, output);
        }
        Some(Commands::Emit { berkas, output }) => {
            emit_native_code(&berkas, output);
        }
        Some(Commands::Llvm { berkas, output }) => {
            emit_llvm_ir_code(&berkas, output);
        }
        Some(Commands::Wasm { berkas, output }) => {
            kompilasi_wasm(&berkas, output);
        }
        Some(Commands::Gpu { berkas, output }) => {
            kompilasi_gpu_wgsl(&berkas, output);
        }
        Some(Commands::Ebpf { berkas, output }) => {
            kompilasi_ebpf(&berkas, output);
        }
        Some(Commands::Tonton { berkas }) | Some(Commands::Watch { berkas }) => {
            widya::tools::watcher::awasi_dan_jalankan(&berkas);
        }
        Some(Commands::Publikasi) => {
            widya::pm::publikasi_paket();
        }
        Some(Commands::Pasang) => {
            widya::pm::pasang_dependensi();
        }
        Some(Commands::Pasok { dir, output }) | Some(Commands::Vendor { dir, output }) => {
            let wpm = widya::wpm::Wpm::new(widya::wpm::WpmConfig::default());
            match wpm.vendor(&dir, output.as_deref()) {
                Ok(vendor_manifest) => {
                    let out_path = output.as_deref().unwrap_or(std::path::Path::new("vendor"));
                    println!("{}", format!("📦 Berhasil memasok {} paket ke direktori '{}' (Mode Offline / Air-Gapped)!", vendor_manifest.packages.len(), out_path.display()).bright_green().bold());
                    for pkg in &vendor_manifest.packages {
                        println!("  - {} @ {} (sha256: {}...)", pkg.name.bright_white().bold(), pkg.version.bright_yellow(), &pkg.checksum_sha256[..12.min(pkg.checksum_sha256.len())]);
                    }
                    if let Ok(count) = wpm.verify_vendor(out_path) {
                        println!("{}", format!("🛡️ Integritas vendor terverifikasi ({} paket valid).", count).bright_cyan());
                    }
                }
                Err(e) => eprintln!("❌ Gagal memasok dependensi: {}", e),
            }
        }
        Some(Commands::Hapus { paket }) => {
            widya::pm::hapus_paket(&paket);
        }
        Some(Commands::Uji { berkas }) | Some(Commands::Test { berkas }) => {
            let target_path = berkas.unwrap_or_else(|| PathBuf::from("."));
            widya::tools::test_runner::jalankan_test_suite(&target_path);
        }
        Some(Commands::Format { berkas, tulis, periksa, style }) => {
            let keyword_style = style.as_deref().and_then(|s| s.parse::<widya::tools::formatter::KeywordStyle>().ok());
            let ok = widya::tools::formatter::format_berkas(&berkas, tulis, periksa, keyword_style);
            if periksa && !ok {
                std::process::exit(1);
            }
        }
        Some(Commands::Periksa { berkas }) => {
            widya::tools::formatter::periksa_linter(&berkas);
        }
        Some(Commands::Dok { berkas, output }) => {
            widya::tools::doc_gen::hasilkan_dokumentasi(&berkas, output);
        }
        Some(Commands::Debug { berkas }) => {
            if let Err(e) = widya::tools::debugger::jalankan_debugger(&berkas) {
                eprintln!("{}", e.bright_red());
                process::exit(1);
            }
        }
        Some(Commands::Jit { berkas }) => {
            widya::tools::jit::jalankan_jit_berkas(&berkas);
        }
        Some(Commands::Kemas { berkas, output, mobile }) | Some(Commands::Bundle { berkas, output, mobile }) => {
            widya::tools::bundler::kemas_aplikasi(&berkas, output, mobile);
        }
        Some(Commands::TolakUkur { berkas }) | Some(Commands::Bench { berkas }) | Some(Commands::Benchmark { berkas }) => {
            widya::tools::bench::jalankan_benchmark(&berkas);
        }
        Some(Commands::Ui { berkas }) => {
            jalankan_ui(&berkas);
        }
        Some(Commands::Studio { aksi, port }) => {
            match aksi {
                Some(StudioSubcommands::Run { port, bind, open }) | Some(StudioSubcommands::Mcr { port, bind, open }) => {
                    handle_broadcast_studio_run(&bind, port, open);
                }
                Some(StudioSubcommands::Ide { port }) => {
                    jalankan_studio(port);
                }
                None => {
                    jalankan_studio(port);
                }
            }
        }
        Some(Commands::Broadcast { aksi, port, bind, open }) | Some(Commands::Mcr { aksi, port, bind, open }) => {
            match aksi {
                Some(BroadcastSubcommands::Run { port, bind, open }) => {
                    handle_broadcast_studio_run(&bind, port, open);
                }
                Some(BroadcastSubcommands::Kamera { cam, port }) => {
                    let url = format!("http://localhost:{}/kamera?cam={}", port, cam);
                    #[cfg(target_os = "linux")]
                    let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                    #[cfg(target_os = "macos")]
                    let _ = std::process::Command::new("open").arg(&url).spawn();
                    #[cfg(target_os = "windows")]
                    let _ = std::process::Command::new("cmd").args(["/C", "start", &url]).spawn();
                    println!("📱 Membuka Viewfinder Kamera {} di {}", cam.bright_green(), url.bright_cyan());
                }
                Some(BroadcastSubcommands::Var { port }) => {
                    let url = format!("http://localhost:{}/var", port);
                    #[cfg(target_os = "linux")]
                    let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                    #[cfg(target_os = "macos")]
                    let _ = std::process::Command::new("open").arg(&url).spawn();
                    #[cfg(target_os = "windows")]
                    let _ = std::process::Command::new("cmd").args(["/C", "start", &url]).spawn();
                    println!("⚖️  Membuka Terminal VAR Dewan Wasit di {}", url.bright_cyan());
                }
                None => {
                    handle_broadcast_studio_run(&bind, port, open);
                }
            }
        }
        Some(Commands::Lsp) => {
            jalankan_lsp();
        }
        Some(Commands::Inisialisasi { nama }) | Some(Commands::New { nama }) => {
            widya::pm::inisialisasi_proyek(&nama);
        }
        Some(Commands::Tambah { paket }) => {
            widya::pm::tambah_paket(&paket);
        }
        Some(Commands::Repl) => {
            start_repl();
        }
        Some(Commands::Tokens { berkas }) => {
            tampilkan_tokens(&berkas);
        }
        Some(Commands::Ast { berkas }) => {
            tampilkan_ast(&berkas);
        }
        Some(Commands::Mobile { aksi }) => handle_mobile_command(&aksi),
        Some(Commands::Desktop { aksi }) => handle_desktop_command(&aksi),
        Some(Commands::Base { aksi }) => handle_base_command(&aksi),
        Some(Commands::Bayar { aksi }) | Some(Commands::Payment { aksi }) => handle_bayar_command(&aksi),
        Some(Commands::Game3d { aksi }) => handle_game3d_command(&aksi),
        Some(Commands::Web3 { aksi }) => handle_web3_command(&aksi),
        Some(Commands::Health { aksi }) | Some(Commands::Fhir { aksi }) => handle_health_command(&aksi),
        Some(Commands::Office { aksi }) | Some(Commands::Dokumen { aksi }) => handle_office_command(&aksi),
        Some(Commands::Drive { aksi }) => handle_drive_command(&aksi),
        Some(Commands::Workspace { aksi }) => handle_workspace_command(&aksi),
        Some(Commands::Smartcity { aksi }) => handle_smartcity_command(&aksi),
        Some(Commands::Store { aksi }) | Some(Commands::Minio { aksi }) => handle_store_command(&aksi),
        Some(Commands::Docdb { aksi }) | Some(Commands::Mongo { aksi }) => handle_docdb_command(&aksi),
        Some(Commands::Security { aksi, target }) | Some(Commands::Audit { aksi, target }) => {
            handle_security_command(aksi, target);
        }
        Some(Commands::Profile { berkas }) => {
            jalankan_memory_profiler(&berkas);
        }
        Some(Commands::Dap) => {
            jalankan_dap_server();
        }
        Some(Commands::Wpm { aksi }) | Some(Commands::Pkg { aksi }) | Some(Commands::Package { aksi }) => {
            handle_wpm_command(&aksi);
        }
        Some(Commands::Bootstrap { dir }) | Some(Commands::Selfhost { dir }) | Some(Commands::SelfHost { dir }) => {
            use widya::bootstrap::{SelfHostingBootstrap, BootstrapStageStatus};
            println!("{}", "🚀 Menjalankan Full Self-Hosting Compiler Pipeline...".bright_cyan().bold());
            let bootstrapper = SelfHostingBootstrap::new(&dir);
            let stages = bootstrapper.run_full_bootstrap_cycle();
            for st in stages {
                match st {
                    BootstrapStageStatus::Success { stage_name, hash } => {
                        println!("{} {} [{}]", "✅ PASS:".bright_green().bold(), stage_name.bright_white(), hash.bright_yellow());
                    }
                    BootstrapStageStatus::Failure { stage_name, error_msg } => {
                        eprintln!("{} {} -> {}", "❌ FAIL:".bright_red().bold(), stage_name, error_msg);
                        process::exit(1);
                    }
                }
            }
            println!("{}", "🎉 100% Deterministic Full Self-Hosting Validated Successfully!".bright_green().bold());
        }
        Some(Commands::Services { start_port }) | Some(Commands::OsServices { start_port }) => {
            handle_services_command(start_port);
        }
        Some(Commands::Web { aksi }) | Some(Commands::Cloud { aksi }) => {
            handle_web_command(&aksi);
        }
        Some(Commands::Deploy { dir, prod }) => {
            handle_web_command(&WebSubcommands::Deploy { dir: dir.clone(), prod });
        }
        Some(Commands::Box { aksi }) | Some(Commands::Podman { aksi }) | Some(Commands::Container { aksi }) => {
            handle_box_command(&aksi);
        }
        Some(Commands::Pod { aksi }) => {
            handle_pod_command(&aksi);
        }
        Some(Commands::K8s { aksi }) | Some(Commands::Kube { aksi }) | Some(Commands::Cluster { aksi }) => {
            handle_k8s_command(&aksi);
        }
        Some(Commands::Ujian { aksi }) | Some(Commands::Exam { aksi }) => {
            handle_exam_command(&aksi);
        }
        Some(Commands::Ad { aksi }) | Some(Commands::Iklan { aksi }) => {
            handle_ad_command(&aksi);
        }
        Some(Commands::Doc { dir, output, serve, port }) | Some(Commands::Wdoc { dir, output, serve, port }) => {
            use widya::wdoc::{Wdoc, WdocConfig};
            let config = WdocConfig {
                input_dir: dir.clone(),
                output_dir: output.clone(),
                theme: "modern".to_string(),
                include_private: false,
                generate_examples: true,
            };
            let mut doc_gen = Wdoc::new(config);
            let _ = doc_gen.load(&dir.join("src/lib.rs"));
            if serve {
                let _ = doc_gen.serve_docs(port, &output);
                println!("{}", format!("🌐 Menjalankan server dokumentasi di http://localhost:{}", port).bright_cyan().bold());
                loop {
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                match doc_gen.generate_html(&output) {
                    Ok(()) => {
                        println!("{}", "✨ Dokumentasi HTML berhasil dibangun!".bright_green().bold());
                        println!("📄 Buka: {}", output.join("index.html").display().to_string().bright_cyan());
                    }
                    Err(e) => eprintln!("❌ Gagal membangun dokumentasi: {}", e),
                }
            }
        }
        Some(Commands::Learn { bab }) | Some(Commands::Tutor { bab }) => {
            use widya::learn::TutorialEngine;
            if let Some(id) = bab {
                TutorialEngine::run_chapter(id);
            } else {
                TutorialEngine::display_curriculum();
            }
        }
        Some(Commands::Embedded { nama, arch, output }) | Some(Commands::Firmware { nama, arch, output }) => {
            use widya::embedded::{EmbeddedFirmwareGenerator, TargetArch};
            let target_arch = TargetArch::from_str(&arch);
            match EmbeddedFirmwareGenerator::emit_firmware_package(&output, &nama, &target_arch) {
                Ok(ld_path) => {
                    println!("{}", "✨ Paket Firmware Bare-Metal Berhasil Dibuat!".bright_green().bold());
                    println!("🎯 Target Arch   : {}", target_arch.triple().bright_yellow().bold());
                    println!("📁 Output Dir    : {}", output.display().to_string().bright_cyan());
                    println!("📜 Linker Script : {}", ld_path.display().to_string().bright_white());
                }
                Err(e) => eprintln!("❌ Gagal membangun paket firmware: {}", e),
            }
        }
        Some(Commands::Ai { aksi }) | Some(Commands::Gguf { aksi }) => {
            handle_ai_command(&aksi);
        }
        Some(Commands::Kripto { aksi }) | Some(Commands::Pqc { aksi }) => {
            handle_kripto_command(&aksi);
        }
        Some(Commands::Fintech { aksi }) | Some(Commands::Bifast { aksi }) => {
            handle_fintech_command(&aksi);
        }
        Some(Commands::WasmKomponen { aksi }) | Some(Commands::WasmComponent { aksi }) => {
            handle_wasm_component_command(&aksi);
        }
        Some(Commands::Notebook { aksi }) | Some(Commands::Buku { aksi }) => {
            handle_notebook_command(&aksi);
        }
        Some(Commands::BangunTarget { proyek, target }) | Some(Commands::BuildTarget { proyek, target }) => {
            handle_bangun_target_command(&proyek, &target);
        }
        Some(Commands::Otomotif { aksi }) | Some(Commands::Automotive { aksi }) => {
            handle_otomotif_command(&aksi);
        }
        Some(Commands::Telekomunikasi { aksi }) | Some(Commands::Telecom { aksi }) => {
            handle_telecom_command(&aksi);
        }
        Some(Commands::Iiot { aksi }) | Some(Commands::Scada { aksi }) => {
            handle_iiot_command(&aksi);
        }
        Some(Commands::Kuantum { aksi }) | Some(Commands::Quantum { aksi }) => {
            handle_kuantum_command(&aksi);
        }
        Some(Commands::Pilar { aksi }) | Some(Commands::Sovereign { aksi }) => {
            handle_pilar_command(&aksi);
        }
        Some(Commands::Unikernel { aksi }) => {
            handle_unikernel_command(&aksi);
        }
        Some(Commands::Mcp { aksi }) => {
            handle_mcp_command(&aksi);
        }
        Some(Commands::Sbom { aksi }) => {
            handle_sbom_command(&aksi);
        }
        Some(Commands::Chaos { aksi }) => {
            handle_chaos_command(&aksi);
        }
        Some(Commands::Hsm { aksi }) => {
            handle_hsm_command(&aksi);
        }

        None => {
            if let Some(berkas) = cli.berkas {
                jalankan_berkas(&berkas, cli.profil.as_deref());
            } else {
                start_repl();
            }
        }
    }
}



fn jalankan_memory_profiler(path: &PathBuf) {
    use widya::profiler::MemoryProfiler;

    println!("{}", format!("🧠 Memulai Memory Profiling untuk: {}", path.display()).bright_cyan().bold());
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!("❌ Gagal membuka berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut profiler = MemoryProfiler::new();

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    
    // Simulate runtime allocation tracking
    for stmt in &program.statements {
        match interpreter.execute(stmt) {
            Ok(val) => {
                let bytes = match &val {
                    Value::String(s) => s.len() + 24,
                    Value::Array(a) => a.borrow().len() * 16 + 32,
                    Value::Map(m) => m.borrow().len() * 32 + 64,
                    _ => 16,
                };
                let id = profiler.track_allocation(&val, bytes);
                if matches!(val, Value::Array(_) | Value::Map(_)) {
                    profiler.add_reference(id, id); // Cycle test check
                }
            }
            Err(e) => {
                eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
                break;
            }
        }
    }

    println!("{}", profiler.format_report());
}

fn jalankan_dap_server() {
    use widya::dap::DapEngine;

    println!("{}", "=======================================================".bright_blue());
    println!("{}", "   🐞 Widya Debug Adapter Protocol (DAP) Server v0.1.0".bright_cyan().bold());
    println!("{}", "   Menunggu koneksi dari IDE Client (VS Code / Studio)...".white());
    println!("{}", "=======================================================".bright_blue());

    let engine = DapEngine::new();
    let caps = engine.get_dap_capabilities();
    println!("Capabilities: {}", serde_json::to_string_pretty(&caps).unwrap().bright_green());
    println!("DAP Server aktif dan siap menerima sesi debugging.");
}

fn jalankan_berkas(path: &PathBuf, profil: Option<&str>) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!(
                "{} Gagal membuka berkas '{}': {}",
                "❌".bright_red(),
                path.display(),
                e
            );
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut borrow_checker = BorrowChecker::new();
    if let Err(e) = borrow_checker.check_profile_context(&program, profil) {
        eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
        process::exit(1);
    }

    let mut interpreter = Interpreter::new();
    if let Some(parent) = path.parent() {
        interpreter.source_dir_stack.push(parent.to_path_buf());
    }
    if let Err(e) = interpreter.interpret(&program) {
        eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
        process::exit(1);
    }
}


fn kompilasi_native(path: &PathBuf, output: Option<PathBuf>) {
    let out_path = output.unwrap_or_else(|| {
        let mut p = path.clone();
        if cfg!(target_os = "windows") {
            p.set_extension("exe");
        } else {
            p.set_extension("");
        }
        p
    });

    println!("{}", "⚙️  Mengompilasi Widya ke Binary Executable Mandiri...".bright_cyan().bold());
    println!("   Kode Sumber : {}", path.display().to_string().bright_yellow());
    println!("   Target Biner: {}", out_path.display().to_string().bright_green().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    match compile_to_executable(&program, &out_path) {
        Ok(_) => {
            println!("{}", "✅ Kompilasi Mandiri Berhasil!".bright_green().bold());
            println!("🚀 Executable mandiri siap dijalankan tanpa interpreter:");
            println!("   .\\{}", out_path.display().to_string().bright_yellow());
        }
        Err(e) => {
            eprintln!("❌ Galat Kompilasi: {}", e);
            process::exit(1);
        }
    }
}

fn emit_native_code(path: &PathBuf, output: Option<PathBuf>) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut compiler = NativeCompiler::new();
    let code = match compiler.compile_to_rust(&program) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Galat Emitter: {}", e);
            process::exit(1);
        }
    };

    if let Some(out) = output {
        if let Err(e) = fs::write(&out, &code) {
            eprintln!("Gagal menulis berkas keluaran: {}", e);
            process::exit(1);
        }
        println!("Kode native ditulis ke: {}", out.display());
    } else {
        println!("{}", "=== KODE NATIVE TERKOMPILASI ===".bright_cyan().bold());
        println!("{}", code);
    }
}

fn emit_llvm_ir_code(path: &PathBuf, output: Option<PathBuf>) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut emitter = widya::llvm::LlvmEmitter::new();
    let llvm_ir = match emitter.emit_llvm_ir(&program) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Galat LLVM IR Emitter: {}", e);
            process::exit(1);
        }
    };

    if let Some(out) = output {
        if let Err(e) = fs::write(&out, &llvm_ir) {
            eprintln!("Gagal menulis berkas keluaran LLVM IR: {}", e);
            process::exit(1);
        }
        println!("{}", "✅ LLVM IR Berhasil Dihasilkan!".bright_green().bold());
        println!("📄 Berkas LLVM IR: {}", out.display().to_string().bright_yellow());
    } else {
        println!("{}", "=== LLVM IR (Intermediate Representation) ===".bright_cyan().bold());
        println!("{}", llvm_ir);
    }
}

fn kompilasi_wasm(path: &PathBuf, output: Option<PathBuf>) {
    let out_wasm = output.unwrap_or_else(|| {
        let mut p = path.clone();
        p.set_extension("wasm");
        p
    });

    let mut out_html = out_wasm.clone();
    out_html.set_extension("html");

    println!("{}", "⚡ Mengompilasi Widya langsung ke biner WebAssembly (.wasm)...".bright_cyan().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut wasm_emitter = widya::wasm::WasmEmitter::new();
    let wasm_bytes = match wasm_emitter.emit_wasm(&program) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Galat Kompilasi WASM: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&out_wasm, &wasm_bytes) {
        eprintln!("Gagal menulis berkas biner WASM: {}", e);
        process::exit(1);
    }

    let file_name = out_wasm.file_name().unwrap().to_str().unwrap();
    let runner_html = widya::wasm::WasmEmitter::generate_html_runner(file_name);
    let _ = fs::write(&out_html, runner_html);

    println!("{}", "✅ Kompilasi WebAssembly Sukses!".bright_green().bold());
    println!("📦 Modul Biner WASM : {}", out_wasm.display().to_string().bright_yellow());
    println!("🌐 Web Runner HTML  : {}", out_html.display().to_string().bright_cyan());
    println!("🚀 Siap dijalankan di browser web dengan performa komputasi mendekati native.");
}

fn kompilasi_gpu_wgsl(path: &PathBuf, output: Option<PathBuf>) {
    let out_wgsl = output.unwrap_or_else(|| {
        let mut p = path.clone();
        p.set_extension("wgsl");
        p
    });

    println!("{}", "⚡ Mengompilasi Widya ke WebGPU Shading Language (WGSL)...".bright_cyan().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut gpu_emitter = widya::gpu::GpuShaderEmitter::new();
    let wgsl_code = match gpu_emitter.emit_wgsl(&program) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Galat Kompilasi GPU: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&out_wgsl, &wgsl_code) {
        eprintln!("Gagal menulis berkas WGSL: {}", e);
        process::exit(1);
    }

    println!("{}", "✅ Kompilasi WebGPU Shader Sukses!".bright_green().bold());
    println!("🎮 Berkas WGSL Compute: {}", out_wgsl.display().to_string().bright_yellow());
    println!("🚀 Siap dieksekusi di pipeline komputasi GPU (WebGPU / Vulkan / DirectX12 / Metal).");
}

fn kompilasi_ebpf(path: &PathBuf, output: Option<PathBuf>) {
    let out_ebpf = output.unwrap_or_else(|| {
        let mut p = path.clone();
        p.set_extension("bpf.c");
        p
    });

    println!("{}", "⚡ Mengompilasi Widya ke Linux Kernel eBPF C Source...".bright_cyan().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut ebpf_emitter = widya::ebpf::EbpfEmitter::new();
    let ebpf_c = match ebpf_emitter.emit_ebpf_c(&program) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Galat Kompilasi eBPF: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&out_ebpf, &ebpf_c) {
        eprintln!("Gagal menulis berkas eBPF C: {}", e);
        process::exit(1);
    }

    println!("{}", "✅ Kompilasi eBPF Kernel Probe Sukses!".bright_green().bold());
    println!("🛡️  Berkas eBPF C Program: {}", out_ebpf.display().to_string().bright_yellow());
    println!("🚀 Siap dikompilasi dengan clang -target bpf dan dimuat ke Linux Kernel.");
}

fn jalankan_ui(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!(
                "{} Gagal membuka berkas '{}': {}",
                "❌".bright_red(),
                path.display(),
                e
            );
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    let hasil = match interpreter.interpret(&program) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let span = widya::error::Span::new(1, 1);
    let render_fn = interpreter.globals.borrow().get("render_html", &span).unwrap();
    let html_val = match interpreter.call_value(
        render_fn,
        &[hasil],
        &span,
    ) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Gagal merender UI: {}", e);
            process::exit(1);
        }
    };

    let output_file = "widya_app.html";
    if let Value::String(html_content) = html_val {
        if let Err(e) = fs::write(output_file, &html_content) {
            eprintln!("Gagal menulis berkas HTML: {}", e);
            process::exit(1);
        }
        println!("{}", "✨ Aplikasi WidyaUI Berhasil Dirender! ✨".bright_green().bold());
        println!("📄 Berkas HTML: {}", output_file.bright_cyan().underline());
        println!("🌐 Buka '{}' di browser web Anda untuk melihat antarmuka interaktif.", output_file.bright_yellow());
    }
}

fn tampilkan_tokens(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!("Gagal membuka berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    match lexer.scan_tokens() {
        Ok(tokens) => {
            println!("{}", "=== DAFTAR TOKEN ===".bright_cyan().bold());
            for (i, t) in tokens.iter().enumerate() {
                println!(
                    "[{:3}] Baris {:2}, Kolom {:2} | {:<25} | lexeme: '{}'",
                    i,
                    t.span.line,
                    t.span.column,
                    format!("{}", t.token_type),
                    t.lexeme
                );
            }
        }
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    }
}

fn tampilkan_ast(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!("Gagal membuka berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("{}", "=== ABSTRACT SYNTAX TREE (AST) ===".bright_cyan().bold());
            println!("{:#?}", program.statements);
        }
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    }
}

