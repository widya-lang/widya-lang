//! Integration Test Suite: Poin 4 (Advanced Type System, Pattern Matching, ADT, Borrow Checker, FFI)

use std::collections::HashMap;
use widya::typesystem::{Type, Span, Monomorphizer, TypeInferrer};
use widya::patternmatching::{Pattern, PatternValue, PatternMatcher, MatchArm, ADTUtils};
use widya::borrow_checker::{BorrowChecker, OwnershipState};
use widya::ffi::{FfiManager, FfiType, FfiFunctionSignature};
use widya::error::Span as ErrSpan;

#[test]
fn test_adt_pattern_matching_result_and_option() {
    // 1. Uji Destructuring Hasil::Sukses(100.0)
    let sukses_val = PatternValue::sukses(PatternValue::Number(100.0));
    let sukses_pat = Pattern::Constructor {
        name: "Hasil".to_string(),
        variant: Some("Sukses".to_string()),
        subpatterns: vec![Pattern::Variable("nilai".to_string(), Span::dummy())],
        span: Span::dummy(),
    };

    let bindings = PatternMatcher::match_value(&sukses_val, &sukses_pat).expect("Harus cocok dengan Sukses");
    assert_eq!(bindings.get("nilai"), Some(&PatternValue::Number(100.0)));

    // 2. Uji Destructuring Hasil::Galat("Koneksi Putus")
    let galat_val = PatternValue::galat(PatternValue::String("Koneksi Putus".to_string()));
    let galat_pat = Pattern::Constructor {
        name: "Hasil".to_string(),
        variant: Some("Galat".to_string()),
        subpatterns: vec![Pattern::Variable("pesan".to_string(), Span::dummy())],
        span: Span::dummy(),
    };

    let err_bindings = PatternMatcher::match_value(&galat_val, &galat_pat).expect("Harus cocok dengan Galat");
    assert_eq!(err_bindings.get("pesan"), Some(&PatternValue::String("Koneksi Putus".to_string())));

    // 3. Uji Destructuring Opsi::Ada & Opsi::Kosong
    let ada_val = PatternValue::ada(PatternValue::Number(42.0));
    let ada_pat = Pattern::Constructor {
        name: "Opsi".to_string(),
        variant: Some("Ada".to_string()),
        subpatterns: vec![Pattern::Variable("v".to_string(), Span::dummy())],
        span: Span::dummy(),
    };
    let ada_bindings = PatternMatcher::match_value(&ada_val, &ada_pat).expect("Harus cocok dengan Ada");
    assert_eq!(ada_bindings.get("v"), Some(&PatternValue::Number(42.0)));
}

#[test]
fn test_pattern_matching_guards_and_exhaustiveness() {
    let arms = vec![
        MatchArm {
            pattern: Pattern::Constructor {
                name: "Hasil".to_string(),
                variant: Some("Sukses".to_string()),
                subpatterns: vec![Pattern::Variable("nominal".to_string(), Span::dummy())],
                span: Span::dummy(),
            },
            guard: Some(|b| {
                if let Some(PatternValue::Number(n)) = b.get("nominal") {
                    *n > 500.0
                } else {
                    false
                }
            }),
            body_label: "ARM_HIGH_VALUE".to_string(),
            span: Span::dummy(),
        },
        MatchArm {
            pattern: Pattern::Constructor {
                name: "Hasil".to_string(),
                variant: Some("Sukses".to_string()),
                subpatterns: vec![Pattern::Variable("nominal".to_string(), Span::dummy())],
                span: Span::dummy(),
            },
            guard: None,
            body_label: "ARM_STANDARD_VALUE".to_string(),
            span: Span::dummy(),
        },
        MatchArm {
            pattern: Pattern::Constructor {
                name: "Hasil".to_string(),
                variant: Some("Galat".to_string()),
                subpatterns: vec![Pattern::Wildcard(Span::dummy())],
                span: Span::dummy(),
            },
            guard: None,
            body_label: "ARM_ERROR_HANDLER".to_string(),
            span: Span::dummy(),
        },
    ];

    // Cek exhaustiveness untuk tipe Hasil
    let is_exhaustive = PatternMatcher::check_exhaustiveness(&arms, &Type::Result(Box::new(Type::Number), Box::new(Type::String)));
    assert!(is_exhaustive.is_ok(), "Pola Hasil harus exhaustive");

    // Evaluasi nilai 1000.0 (Harus masuk ARM_HIGH_VALUE karena guard)
    let val_1000 = PatternValue::sukses(PatternValue::Number(1000.0));
    let (arm, _) = PatternMatcher::evaluate_arms(&val_1000, &arms).expect("Harus ada arm yang cocok");
    assert_eq!(arm.body_label, "ARM_HIGH_VALUE");

    // Evaluasi nilai 200.0 (Harus masuk ARM_STANDARD_VALUE)
    let val_200 = PatternValue::sukses(PatternValue::Number(200.0));
    let (arm2, _) = PatternMatcher::evaluate_arms(&val_200, &arms).expect("Harus ada arm yang cocok");
    assert_eq!(arm2.body_label, "ARM_STANDARD_VALUE");
}

#[test]
fn test_generic_monomorphizer() {
    let mut mono = Monomorphizer::new();

    // Template tipe: Hasil<T, G>
    let generic_result = Type::Result(
        Box::new(Type::Generic("T".to_string())),
        Box::new(Type::Generic("G".to_string())),
    );

    let mut mapping = HashMap::new();
    mapping.insert("T".to_string(), Type::Number);
    mapping.insert("G".to_string(), Type::String);

    let concrete_type = mono.monomorphize(&generic_result, &mapping);
    assert_eq!(
        concrete_type,
        Type::Result(Box::new(Type::Number), Box::new(Type::String))
    );

    // Test Mangled Symbol untuk LLVM Backend
    let mangled = mono.mangle_symbol("kirim_dana", &[Type::Number, Type::String]);
    assert_eq!(mangled, "kirim_dana_angka_string");
}

#[test]
fn test_static_borrow_checker_aliasing_mutability() {
    let mut checker = BorrowChecker::new();
    let span = ErrSpan::new(1, 1);

    // 1. Multiple immutable borrows diperbolehkan (&T)
    assert!(checker.borrow_immutable("rekening", span.clone()).is_ok());
    assert!(checker.borrow_immutable("rekening", span.clone()).is_ok());

    // 2. Mutable borrow (&mut T) ditolak saat masih ada pembaca aktif
    let mut_err = checker.borrow_mutable("rekening", span.clone());
    assert!(mut_err.is_err(), "Harus gagal karena masih dipinjam secara imutabel");

    // 3. Lepaskan borrow imutabel
    checker.release_borrow("rekening");
    checker.release_borrow("rekening");

    // 4. Setelah dilepas, mutable borrow berhasil
    assert!(checker.borrow_mutable("rekening", span.clone()).is_ok());

    // 5. Tidak boleh meminjam lagi saat sedang dipinjam mutabel
    assert!(checker.borrow_immutable("rekening", span.clone()).is_err());
    assert!(checker.borrow_mutable("rekening", span.clone()).is_err());

    // 6. Use-after-move detection
    checker.release_borrow("rekening");
    checker.mark_moved("rekening", span.clone());
    assert!(checker.borrow_immutable("rekening", span.clone()).is_err());
}

#[test]
fn test_ffi_c_abi_registry() {
    let mut ffi = FfiManager::new();

    // Register prototype fungsi C standar (misal: puts, abs, sqrt)
    let sig = FfiFunctionSignature {
        name: "hitung_pajak".to_string(),
        symbol: "c_tax_calc".to_string(),
        library: "libfintech.so".to_string(),
        param_types: vec![FfiType::Float64, FfiType::Float64],
        return_type: FfiType::Float64,
        is_variadic: false,
    };
    ffi.register_signature(sig);

    assert!(ffi.load_library("libfintech", "/usr/lib/libfintech.so").is_ok());
    let sig_lookup = ffi.get_signature("hitung_pajak");
    assert!(sig_lookup.is_some());
    assert_eq!(sig_lookup.unwrap().return_type, FfiType::Float64);
}
