use std::collections::HashMap;
use widya::ai::{
    VectorDatabase, DistanceMetric, AgentRuntime, AgentProfile, SovereignTool, ToolParameter,
    ToolCallResponse, SovereignInferenceEngine, ModelGenerationConfig,
};
use widya::microvm::{MicroVmSandbox, MicroVmConfig, MicroVmExitCode};
use widya::meshnet::{NusantaraMeshRouter, P2pPeer, MeshPacket, DidEngine};
use widya::twin::{DigitalTwinScene, TwinEntity, SpatialCoordinate};
use widya::flow::{VisualFlowGraph, FlowNode, FlowNodeType, FlowConnection, FlowTranspiler};

#[test]
fn test_ai_native_vector_db_and_autonomous_agent() {
    // 1. Vector Database Test
    let vdb = VectorDatabase::new(4, DistanceMetric::Cosine);
    let mut meta1 = HashMap::new();
    meta1.insert("kategori".to_string(), "hukum".to_string());
    vdb.insert("uu-cipta-kerja", vec![0.1, 0.8, 0.2, 0.5], meta1.clone()).unwrap();

    let mut meta2 = HashMap::new();
    meta2.insert("kategori".to_string(), "teknologi".to_string());
    vdb.insert("widya-spec", vec![0.9, 0.1, 0.7, 0.3], meta2).unwrap();

    let query = vec![0.85, 0.15, 0.65, 0.35];
    let results = vdb.search(&query, 2, None).unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].id, "widya-spec");
    assert!(results[0].score > 0.95);

    // 2. Autonomous Agent Test
    let tools = vec![
        SovereignTool {
            name: "query_widyabase".to_string(),
            description: "Mencari data pada basis data WidyaBase".to_string(),
            parameters: vec![ToolParameter {
                name: "query".to_string(),
                param_type: "string".to_string(),
                description: "SQL Query".to_string(),
                required: true,
            }],
        }
    ];
    let profile = AgentProfile {
        name: "AsistenPintar".to_string(),
        role: "Database Analyst".to_string(),
        system_prompt: "Anda adalah agen pintar Widya".to_string(),
        temperature: 0.2,
        tools,
    };
    let agent = AgentRuntime::new(profile);
    agent.add_user_message("Tolong cari data warga aktif di database");

    let (reasoning, tool_call) = agent.decide_action("Tolong cari data warga");
    assert!(tool_call.is_some());
    let call = tool_call.unwrap();
    assert_eq!(call.tool_name, "query_widyabase");

    agent.receive_tool_result(ToolCallResponse {
        tool_name: "query_widyabase".to_string(),
        result: "100 data ditemukan".to_string(),
        success: true,
    });
    assert_eq!(agent.get_conversation_history().len(), 3);

    // 3. Local GGUF Inference Test
    let engine = SovereignInferenceEngine::new("qwen2.5-coder-7b", 8192);
    let prompt = engine.format_chatml_prompt("Anda adalah AI Widya", "Jelaskan apa itu Widya-Lang");
    let response = engine.generate_tokens(&prompt, &ModelGenerationConfig::default());
    assert!(response.contains("Widya-Lang"));
}

#[test]
fn test_microvm_wasm_serverless_sandboxing() {
    let mut vm = MicroVmSandbox::new(MicroVmConfig {
        max_memory_bytes: 64 * 1024,
        max_fuel: 100,
        allow_network: false,
        allow_filesystem: false,
    });

    // Bytecode with print (0x01) and memory expand (0x02)
    let bytecode = vec![0x01, 0x02, 0x00];
    let result = vm.execute_bytecode(&bytecode);
    assert_eq!(result.exit_code, MicroVmExitCode::Success(0));
    assert!(result.stdout.contains("WidyaVM Sandboxed Output: OK"));
    assert_eq!(result.fuel_consumed, 3);

    // Bytecode attempting unauthorized network call (0x03)
    let mut network_restricted_vm = MicroVmSandbox::new(MicroVmConfig::default());
    let bad_bytecode = vec![0x03];
    let violation_result = network_restricted_vm.execute_bytecode(&bad_bytecode);
    match violation_result.exit_code {
        MicroVmExitCode::CapabilityViolation(msg) => assert!(msg.contains("Akses jaringan dilarang")),
        _ => panic!("Seharusnya memicu pelanggaran kapabilitas sandbox"),
    }
}

#[test]
fn test_p2p_meshnet_and_verifiable_credentials() {
    let router = NusantaraMeshRouter::new("node_jakarta_01");
    
    // Register BLE / LoRa Peer
    router.register_peer(P2pPeer {
        peer_id: "node_bandung_02".to_string(),
        transport_type: "LORA".to_string(),
        signal_rssi: -65,
        last_seen: 1727700000,
    });

    // Send multi-hop packet
    let packet = MeshPacket {
        packet_id: "pkt_emergency_991".to_string(),
        source_did: "did:widya:bandung".to_string(),
        destination_did: "node_jakarta_01".to_string(),
        payload: b"STATUS_BENCANA_TERKENDALI".to_vec(),
        ttl_hops: 5,
        hops_taken: 0,
    };
    assert!(router.route_packet(packet.clone()));
    // Duplicate packet must be rejected to prevent infinite routing loop
    assert!(!router.route_packet(packet));
    assert_eq!(router.read_inbox().len(), 1);

    // Test W3C DID & Verifiable Credential
    let did_doc = DidEngine::create_did("a1b2c3d4e5f67890");
    assert_eq!(did_doc.id, "did:widya:a1b2c3d4e5f67890");

    let mut claims = HashMap::new();
    claims.insert("nama".to_string(), "Budi Santoso".to_string());
    claims.insert("nik".to_string(), "3171012345670001".to_string());
    claims.insert("kewarganegaraan".to_string(), "WNI".to_string());

    let vc = DidEngine::issue_credential(&did_doc.id, "did:widya:dukcapil_pusat", claims, "KTPDigitalIndonesia");
    assert!(DidEngine::verify_credential(&vc));
    assert_eq!(vc.credential_subject.get("nama").unwrap(), "Budi Santoso");
}

#[test]
fn test_spatial_digital_twin_and_telemetry() {
    let scene = DigitalTwinScene::new("Smart City Nusantara IKN");
    
    let sensor_node = TwinEntity {
        id: "sensor-plts-01".to_string(),
        name: "PLTS Solar Array 5MW".to_string(),
        entity_type: "PLTS_Solar".to_string(),
        coordinate: SpatialCoordinate {
            lat: -0.9634,
            lon: 116.7023,
            altitude_m: 45.0,
            local_x: 120.0,
            local_y: 0.0,
            local_z: 340.0,
        },
        telemetry_metrics: HashMap::new(),
        status: "NORMAL".to_string(),
    };
    scene.add_entity(sensor_node);

    // Update telemetry to trigger critical status
    scene.update_telemetry("sensor-plts-01", "suhu_panel", 88.5).unwrap();
    let entities = scene.get_entities();
    assert_eq!(entities[0].status, "CRITICAL");

    // Generate WebGPU visualizer HTML
    let html = scene.generate_webgpu_scene_html();
    assert!(html.contains("Smart City Nusantara IKN"));
    assert!(html.contains("renderCanvas"));
}

#[test]
fn test_widyaflow_visual_workflow_transpiler() {
    let mut config_webhook = HashMap::new();
    config_webhook.insert("path".to_string(), "/api/v1/pembayaran".to_string());

    let mut config_db = HashMap::new();
    config_db.insert("sql".to_string(), "INSERT INTO transaksi (id, nominal) VALUES (1, 50000)".to_string());

    let graph = VisualFlowGraph {
        flow_id: "flow_auto_checkout_01".to_string(),
        flow_name: "Otomasi Checkout QRIS".to_string(),
        nodes: vec![
            FlowNode {
                id: "node_1".to_string(),
                name: "Tangkap Webhook".to_string(),
                node_type: FlowNodeType::WebhookTrigger,
                config: config_webhook,
            },
            FlowNode {
                id: "node_2".to_string(),
                name: "Simpan ke Database".to_string(),
                node_type: FlowNodeType::DatabaseQuery,
                config: config_db,
            },
            FlowNode {
                id: "node_3".to_string(),
                name: "Kirim Notifikasi".to_string(),
                node_type: FlowNodeType::SendNotification,
                config: HashMap::new(),
            },
        ],
        connections: vec![
            FlowConnection { from_node: "node_1".to_string(), to_node: "node_2".to_string() },
            FlowConnection { from_node: "node_2".to_string(), to_node: "node_3".to_string() },
        ],
    };

    let generated_wya = FlowTranspiler::transpile_to_widya_code(&graph);
    assert!(generated_wya.contains("fungsi utama()"));
    assert!(generated_wya.contains("web::tangkap_webhook(\"/api/v1/pembayaran\")"));
    assert!(generated_wya.contains("widyabase::eksekusi_sql"));
    assert!(generated_wya.contains("notifikasi::kirim_telegram"));
}
