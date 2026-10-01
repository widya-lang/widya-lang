use std::collections::HashMap;
use widya::certification::{ExamEngine, ExamLevel, QuestionBank};

#[test]
fn test_exam_levels_parsing() {
    assert_eq!(ExamLevel::from_str("WCA"), Some(ExamLevel::WCA));
    assert_eq!(ExamLevel::from_str("associate"), Some(ExamLevel::WCA));
    assert_eq!(ExamLevel::from_str("dasar"), Some(ExamLevel::WCA));

    assert_eq!(ExamLevel::from_str("WCP"), Some(ExamLevel::WCP));
    assert_eq!(ExamLevel::from_str("profesional"), Some(ExamLevel::WCP));

    assert_eq!(ExamLevel::from_str("WCE"), Some(ExamLevel::WCE));
    assert_eq!(ExamLevel::from_str("pakar"), Some(ExamLevel::WCE));

    assert_eq!(ExamLevel::from_str("WCWD"), Some(ExamLevel::WCWD));
    assert_eq!(ExamLevel::from_str("WCMD"), Some(ExamLevel::WCMD));
    assert_eq!(ExamLevel::from_str("WCDD"), Some(ExamLevel::WCDD));
    assert_eq!(ExamLevel::from_str("WCSS"), Some(ExamLevel::WCSS));
    assert_eq!(ExamLevel::from_str("WCDA"), Some(ExamLevel::WCDA));
    assert_eq!(ExamLevel::from_str("WCCD"), Some(ExamLevel::WCCD));
    assert_eq!(ExamLevel::from_str("WCEI"), Some(ExamLevel::WCEI));

    assert_eq!(ExamLevel::from_str("INVALID_LEVEL"), None);
}

#[test]
fn test_academic_discount_pricing() {
    // WCA Normal: 750.000 -> 70% off = 225.000
    assert_eq!(ExamLevel::WCA.standard_fee_idr(), 750_000);
    assert_eq!(ExamLevel::WCA.academic_discount_fee_idr(), 225_000);

    // WCP Normal: 1.750.000 -> 70% off = 525.000
    assert_eq!(ExamLevel::WCP.standard_fee_idr(), 1_750_000);
    assert_eq!(ExamLevel::WCP.academic_discount_fee_idr(), 525_000);

    // WCE Normal: 3.500.000 -> 70% off = 1.050.000
    assert_eq!(ExamLevel::WCE.standard_fee_idr(), 3_500_000);
    assert_eq!(ExamLevel::WCE.academic_discount_fee_idr(), 1_050_000);

    // Spesialisasi Normal: 2.250.000 -> 70% off = 675.000
    assert_eq!(ExamLevel::WCWD.standard_fee_idr(), 2_250_000);
    assert_eq!(ExamLevel::WCWD.academic_discount_fee_idr(), 675_000);

    // Validasi email institusi pendidikan
    assert!(ExamLevel::is_academic_email("mahasiswa@ui.ac.id"));
    assert!(ExamLevel::is_academic_email("guru@smkn1.sch.id"));
    assert!(ExamLevel::is_academic_email("dosen@itb.ac.id"));
    assert!(!ExamLevel::is_academic_email("developer@gmail.com"));
    assert!(!ExamLevel::is_academic_email("engineer@corp.com"));
}

#[test]
fn test_question_bank_retrieval() {
    let levels = vec![
        ExamLevel::WCA,
        ExamLevel::WCP,
        ExamLevel::WCE,
        ExamLevel::WCWD,
        ExamLevel::WCMD,
        ExamLevel::WCDD,
        ExamLevel::WCSS,
        ExamLevel::WCDA,
        ExamLevel::WCCD,
        ExamLevel::WCEI,
    ];

    for lvl in levels {
        let (mc, coding) = QuestionBank::get_exam_questions(&lvl);
        assert!(!mc.is_empty(), "MC questions should not be empty for {:?}", lvl);
        assert!(!coding.is_empty(), "Coding challenges should not be empty for {:?}", lvl);
    }
}

#[test]
fn test_wca_coding_challenge_grading_pass() {
    let (_, challenges) = QuestionBank::get_exam_questions(&ExamLevel::WCA);
    let tambah_challenge = &challenges[0];

    let candidate_solution = r#"
    fungsi tambah(a, b) {
        diketahui hasil = a + b
        kembalikan hasil
    }
    "#;

    let (score, logs) = ExamEngine::grade_coding_challenge(tambah_challenge, candidate_solution);
    assert_eq!(score, 100.0, "Submission should get 100% pass score. Logs: {:?}", logs);
}

#[test]
fn test_wca_coding_challenge_grading_fail() {
    let (_, challenges) = QuestionBank::get_exam_questions(&ExamLevel::WCA);
    let tambah_challenge = &challenges[0];

    let wrong_solution = r#"
    fungsi tambah(a, b) {
        kembalikan a - b
    }
    "#;

    let (score, logs) = ExamEngine::grade_coding_challenge(tambah_challenge, wrong_solution);
    assert_eq!(score, 0.0, "Wrong solution should get 0%. Logs: {:?}", logs);
}

#[test]
fn test_full_exam_evaluation_and_certificate_issuance() {
    let mut mc_answers = HashMap::new();
    mc_answers.insert(1, 1); // Benar (index 1)
    mc_answers.insert(2, 1); // Benar (index 1)
    mc_answers.insert(3, 1); // Benar (index 1)
    mc_answers.insert(4, 0); // Benar (index 0)
    mc_answers.insert(5, 0); // Benar (index 0)
    mc_answers.insert(6, 0); // Benar (index 0)
    mc_answers.insert(7, 0); // Benar (index 0)
    mc_answers.insert(8, 0); // Benar (index 0)

    let mut coding_submissions = HashMap::new();
    coding_submissions.insert(
        101,
        r#"
        fungsi tambah(a, b) {
            kembalikan a + b
        }
        "#.to_string(),
    );
    coding_submissions.insert(
        102,
        r#"
        fungsi apakah_genap(n) {
            jika (n % 2 == 0) {
                kembalikan "GENAP"
            } selain_itu {
                kembalikan "GANJIL"
            }
        }
        "#.to_string(),
    );
    coding_submissions.insert(
        103,
        r#"
        fungsi maksimum(a, b) {
            jika (a > b) {
                kembalikan a
            } selain_itu {
                kembalikan b
            }
        }
        "#.to_string(),
    );

    let result = ExamEngine::evaluate_exam(
        "Raden Wijaya",
        "3171019999990001",
        ExamLevel::WCA,
        &mc_answers,
        &coding_submissions,
    );

    assert!(result.passed, "Candidate should pass the exam! Detailed log: {:?}", result.breakdown);
    assert_eq!(result.final_score, 100.0);
    assert!(result.certificate.is_some(), "Certificate must be generated upon passing!");

    let cert = result.certificate.unwrap();
    assert_eq!(cert.candidate_name, "Raden Wijaya");
    assert!(cert.certificate_id.starts_with("WIDYA-WCA-"));
    assert!(cert.verification_url.starts_with("https://verify.widya.dev/cert/"));
    
    let rendered_badge = cert.render_ascii_badge();
    assert!(rendered_badge.contains("DEWAN SERTIFIKASI WIDYA-LANG"));
    assert!(rendered_badge.contains("Raden Wijaya"));
    assert!(rendered_badge.contains("WCA"));
}

#[test]
fn test_export_certification_modules_pdf() {
    let output_dir = std::path::Path::new("modul_sertifikasi_pdf");
    let generated = ExamEngine::export_all_modules_pdf(output_dir).expect("Should export all certification PDFs");
    
    assert_eq!(generated.len(), 10, "Should generate 10 module PDFs");
    for path in generated {
        assert!(path.exists(), "PDF file must exist on disk: {:?}", path);
        let metadata = std::fs::metadata(&path).expect("File metadata should be readable");
        assert!(metadata.len() > 100, "PDF file must not be empty");
    }
}
