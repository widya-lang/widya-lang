//! Comprehensive Integration Test for Voice Streaming, LoRA, TUI Dashboard, Federated RAG, and PQC Hardware Wallet
//! Menguji seluruh modul frontier pelengkap secara terpadu.

use widya::ai::voice_stream::{VoiceStreamEngine, AudioSampleRate};
use widya::ai::lora::LoraLinearLayer;
use widya::ai::federated_rag::{FederatedRagEngine, LocalVectorDoc};
use widya::depin::tui_dashboard::{DepinTuiDashboard, TuiNodeMetricsSnapshot};
use widya::kripto::hardware_wallet::{SovereignHardwareWallet, AirGappedTransactionEnvelope};

#[test]
fn test_realtime_voice_streaming_and_vad() {
    let mut voice = VoiceStreamEngine::new(AudioSampleRate::Rate16kHz, 400);

    // Audio hening
    let chunk_silence = voice.process_input_audio_chunk(vec![0i16; 320], 0);
    assert!(!chunk_silence.is_speech_active);

    // Audio suara pengguna aktif
    let chunk_speech = voice.process_input_audio_chunk(vec![2500i16; 320], 20);
    assert!(chunk_speech.is_speech_active);

    // Sintesis jawaban suara TTS
    let tts_stream = VoiceStreamEngine::synthesize_speech_stream("Halo dari Widya AI", AudioSampleRate::Rate16kHz);
    assert_eq!(tts_stream.len(), 4);
    assert_eq!(tts_stream[0].sample_rate, AudioSampleRate::Rate16kHz);
}

#[test]
fn test_lora_parameter_efficient_finetuning() {
    // Transformer Projection: [2048, 2048]
    let mut lora = LoraLinearLayer::new(2048, 2048, 16, 32.0);

    // Hitung efisiensi parameter:
    // Full = 2048 * 2048 = 4.194.304 parameter
    // LoRA = (16 * 2048) + (2048 * 16) = 65.536 parameter (>98.4% hemat VRAM)
    assert_eq!(lora.trainable_parameters_count(), 65536);

    let dummy_x = vec![0.5f32; 2048];
    let grad_a = vec![0.01f32; lora.lora_a.len()];
    let grad_b = vec![0.02f32; lora.lora_b.len()];

    lora.step_optimizer(&grad_a, &grad_b, 0.1);
    let delta = lora.forward_lora_delta(&dummy_x);
    assert_eq!(delta.len(), 2048);
}

#[test]
fn test_depin_node_tui_rendering() {
    let snapshot = TuiNodeMetricsSnapshot {
        node_id: "node_bandung_gpu".to_string(),
        device_name: "Widya Workstation RTX 4090".to_string(),
        gpu_utilization_percent: 88,
        gpu_temp_celsius: 46.2,
        vram_used_mb: 20480,
        vram_total_mb: 24576,
        current_tflops: 82.0,
        wallet_balance_idr: 4_500_000,
        active_peers_count: 64,
        active_compute_job: Some("LoRA IndoBERT Fine-Tuning #4".to_string()),
    };

    let dashboard_text = DepinTuiDashboard::render_dashboard(&snapshot);
    assert!(dashboard_text.contains("WIDYA-OS SOVEREIGN DEPIN COMPUTE NODE"));
    assert!(dashboard_text.contains("node_bandung_gpu"));
    assert!(dashboard_text.contains("46.2°C"));
    assert!(dashboard_text.contains("Rp 4,500,000"));
}

#[test]
fn test_federated_rag_multi_node_knowledge() {
    let mut kemenkes = FederatedRagEngine::new(0.6);
    kemenkes.index_local_document(LocalVectorDoc {
        doc_id: "doc_satusehat".to_string(),
        title: "Panduan Integrasi SatuSehat FHIR".to_string(),
        content: "Protokol REST HL7 FHIR untuk pertukaran rekam medis elektronik...".to_string(),
        embedding: vec![0.95, 0.05, 0.0],
    });

    let mut bi = FederatedRagEngine::new(0.6);
    bi.index_local_document(LocalVectorDoc {
        doc_id: "doc_bifast".to_string(),
        title: "Spesifikasi ISO 20022 BI-FAST".to_string(),
        content: "Protokol transfer dana ritel real-time 24/7 nasional...".to_string(),
        embedding: vec![0.05, 0.95, 0.0],
    });

    let query_medis = vec![0.90, 0.10, 0.0];
    let res_medis = kemenkes.search_local("node_kemenkes", &query_medis, 1);
    let res_bank = bi.search_local("node_bi", &query_medis, 1);

    let global_best = FederatedRagEngine::aggregate_multi_node_results(vec![res_medis, res_bank], 1);
    assert_eq!(global_best.len(), 1);
    assert_eq!(global_best[0].document_id, "doc_satusehat");
}

#[test]
fn test_pqc_cold_hardware_wallet_airgap() {
    let mut cold_wallet = SovereignHardwareWallet::new("airgap_entropy_seed_widya_bank_01", "123456");

    let envelope = AirGappedTransactionEnvelope {
        transaction_id: "tx_airgap_commission_withdrawal".to_string(),
        sender_wallet: cold_wallet.wallet_address().to_string(),
        recipient_wallet: "rekening_bank_bca_ahmad".to_string(),
        amount_idr: 50_000_000,
        fee_idr: 2_500,
        nonce: 10,
        signature: None,
        is_signed: false,
    };

    assert!(cold_wallet.unlock("123456").unwrap());
    let signed = cold_wallet.sign_air_gapped_transaction(envelope).unwrap();
    assert!(signed.is_signed);

    // Verifikasi tanda tangan Dilithium-5
    assert!(SovereignHardwareWallet::verify_transaction(&signed, cold_wallet.public_key()));
}
