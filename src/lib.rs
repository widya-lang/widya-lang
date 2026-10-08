#![recursion_limit = "512"]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod ast;
pub mod compiler;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod stdlib;
pub mod token;
pub mod value;
pub mod lsp;
pub mod lsp_enhanced;
pub mod studio;
pub mod notebook;
pub mod pm;
pub mod llvm;
pub mod wasm;
pub mod wasm_component;
pub mod gpu;
pub mod ebpf;
pub mod borrow_checker;
pub mod tools;
pub mod web;
pub mod runtime;
pub mod wpm;
pub mod wdoc;
pub mod typesystem;
pub mod patternmatching;
pub mod typeclasses;
pub mod typeparser;
pub mod mobile;
pub mod desktop;
pub mod kripto;
pub mod security_tools;
pub mod ffi;
pub mod profiler;
pub mod dap;
pub mod stdlib_ffi;
pub mod wasm_playground;
pub mod bootstrap;
pub mod widyabase;
pub mod fintech;
pub mod game3d;
pub mod web3;
pub mod healthtech;
pub mod iot;
pub mod ai;
pub mod automotive;
pub mod office;
pub mod smartcity;
pub mod container;
pub mod orchestrator;
pub mod streaming;
pub mod iam;
pub mod apm;
pub mod shield;
pub mod microvm;
pub mod adengine;
pub mod meshnet;
pub mod twin;
pub mod flow;
pub mod unikernel;
pub mod zk;
pub mod cdn;
pub mod rtos;
pub mod timetravel;
pub mod interop;
pub mod certification;
pub mod multimedia;
pub mod zerocopy;
pub mod wpm_resolver;
pub mod ui_engine;
pub mod security_linter;
pub mod formal_verifier;
pub mod bootstrap_selfhost;
pub mod timetravel_debugger;
pub mod wasm_grid;
pub mod depin;
pub mod telecom;
pub mod iiot;
pub mod quantum;
pub mod space;
pub mod energy;
pub mod neuro;
pub mod agri;
pub mod maritime;
pub mod mining;
pub mod materials;
pub mod identity;
pub mod a11y;
pub mod koperasi;
pub mod mbg;
pub mod arkas;
pub mod inventory;
pub mod library;
pub mod journal;
pub mod projectmgmt;
pub mod learn;
pub mod embedded;
pub mod benchmark;
pub mod sys;

use borrow_checker::BorrowChecker;
use error::Galat;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use serde_json::json;
use value::Value;

#[derive(Debug, Clone)]
pub struct CompileTarget {
    pub id: &'static str,
    pub nama: &'static str,
    pub deskripsi: &'static str,
    pub ekstensi_keluaran: &'static str,
    pub kategori: &'static str,
    pub tersedia: bool,
}

pub fn compile_targets() -> Vec<CompileTarget> {
    vec![
        CompileTarget {
            id: "native",
            nama: "Native Executable",
            deskripsi: "Kompilasi ke biner executable native OS target (Windows/Linux/macOS)",
            ekstensi_keluaran: if cfg!(windows) { "exe" } else { "out" },
            kategori: "Native/System",
            tersedia: true,
        },
        CompileTarget {
            id: "rust",
            nama: "Rust Source Code",
            deskripsi: "Transpile Widya-Lang ke source code Rust murni (kompilable dengan rustc)",
            ekstensi_keluaran: "rs",
            kategori: "Source Emitter",
            tersedia: true,
        },
        CompileTarget {
            id: "llvm",
            nama: "LLVM IR",
            deskripsi: "Emit LLVM Intermediate Representation untuk optimasi Clang/LLVM",
            ekstensi_keluaran: "ll",
            kategori: "Intermediate Representation",
            tersedia: true,
        },
        CompileTarget {
            id: "wasm",
            nama: "WebAssembly Binary",
            deskripsi: "Kompilasi ke bytecode WebAssembly (.wasm) + HTML runner untuk browser",
            ekstensi_keluaran: "wasm",
            kategori: "Web/Browser",
            tersedia: true,
        },
        CompileTarget {
            id: "wgsl",
            nama: "WebGPU WGSL Compute Shader",
            deskripsi: "Transpile Widya-Lang ke WebGPU Shading Language (WGSL) Compute",
            ekstensi_keluaran: "wgsl",
            kategori: "GPU/WebGPU",
            tersedia: true,
        },
        CompileTarget {
            id: "ebpf",
            nama: "Linux eBPF C Program",
            deskripsi: "Emit kode C untuk eBPF kernel tracing/networking (clang -target bpf)",
            ekstensi_keluaran: "bpf.c",
            kategori: "Kernel/Observability",
            tersedia: true,
        },
        CompileTarget {
            id: "android",
            nama: "Android App Package (APK/AAB)",
            deskripsi: "Kemas proyek aplikasi native Android lengkap dengan Gradle & Native Bridge",
            ekstensi_keluaran: "apk",
            kategori: "Mobile/Android",
            tersedia: true,
        },
        CompileTarget {
            id: "ios",
            nama: "iOS Application Bundle (IPA)",
            deskripsi: "Kemas proyek aplikasi native iOS lengkap dengan Xcode project & Swift bridge",
            ekstensi_keluaran: "ipa",
            kategori: "Mobile/iOS",
            tersedia: true,
        },
        CompileTarget {
            id: "desktop",
            nama: "Universal Desktop Application Bundle",
            deskripsi: "Kemas proyek aplikasi desktop native mandiri lintas platform (Windows, macOS, Linux)",
            ekstensi_keluaran: "bundle",
            kategori: "Desktop/CrossPlatform",
            tersedia: true,
        },
        CompileTarget {
            id: "desktop-windows",
            nama: "Windows Desktop Application (EXE)",
            deskripsi: "Kemas proyek aplikasi desktop native Windows lengkap dengan launcher dan asset bundle",
            ekstensi_keluaran: "exe",
            kategori: "Desktop/Windows",
            tersedia: true,
        },
    ]
}

pub fn compile_targets_json() -> String {
    let ct = compile_targets();
    let items: Vec<serde_json::Value> = ct.iter().map(|t| {
        json!({
            "id": t.id,
            "nama": t.nama,
            "deskripsi": t.deskripsi,
            "ekstensi_keluaran": t.ekstensi_keluaran,
            "kategori": t.kategori,
            "tersedia": t.tersedia
        })
    }).collect();
    json!({
        "sukses": true,
        "jumlah": items.len(),
        "target": items
    }).to_string()
}

pub fn compile_target_by_id(id: &str) -> Option<CompileTarget> {
    compile_targets().into_iter().find(|t| t.id == id)
}

/// Menjalankan kode sumber Widya-Lang dari string dengan validasi keamanan memori
pub fn jalankan(sumber: &str) -> Result<Value, Galat> {
    let mut lexer = Lexer::new(sumber);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    // Static Memory Safety & Borrow Checking Pass
    let mut borrow_checker = BorrowChecker::new();
    borrow_checker.check_program(&program)?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)
}
