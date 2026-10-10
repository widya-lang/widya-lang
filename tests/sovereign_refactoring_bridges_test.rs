//! Integration Test: Sovereign Refactoring Bridges
//! Memverifikasi prinsip refaktorisasi, interoperabilitas tipe (DRY), dan eliminasi duplikasi logika
//! antar subsistem Widya-Lang (Maritime, RTOS, IIoT, 5G Telecom, Neuro, 3D Rasterizer, Space, Quantum, ZK, eBPF).

use std::collections::HashMap;
use widya::automotive::canbus::CanFrame as AutoCanFrame;
use widya::game3d::math3d::{Mat4, Vec3};
use widya::game3d::mesh::Mesh3D;
use widya::game3d::pure_rasterizer::SoftwareRasterizer;
use widya::iiot::pure_industrial::{CanFrame as IiotCanFrame, CanId};
use widya::kripto::pure_zk::FieldElement;
use widya::maritime::ais_decoder::{AisDecoder, AisPositionReport};
use widya::multimedia::image::{ImageBuffer, RgbaPixel};
use widya::neuro::eeg_filter::EegBandPowers;
use widya::neuro::pure_neuro::{MotorImageryClass, PureBciEngine, PureEegBandPower, PureLifNeuron};
use widya::neuro::spiking_nn::LifNeuron;
use widya::quantum::pure_quantum::Complex;
use widya::quantum::qasm_parser::QuantumCircuit;
use widya::quantum::state_vector_sim::Complex64;
use widya::rtos::CanFrame as RtosCanFrame;
use widya::space::ccsds_packet::CcsdsSpacePacket;
use widya::telecom::gtp_u::{GtpMessageType, GtpPacket};
use widya::telecom::pure_5g::PureGtpuHeader;
use widya::zk::{R1csConstraint, ZkEngine};

#[test]
fn test_ais_maritime_unified_bridge() {
    let report = AisDecoder::decode_position_report(525119000, -6.1234, 106.8765, 14.5, 95.0, 96);
    assert_eq!(report.mmsi, 525119000);
    assert!((report.latitude - (-6.1234)).abs() < 1e-4);
    assert!((report.longitude - 106.8765).abs() < 1e-4);
    assert_eq!(report.true_heading_deg, Some(96));

    // Test Serde JSON serialization
    let json = serde_json::to_string(&report).expect("Serialize AisPositionReport failed");
    let deserialized: AisPositionReport = serde_json::from_str(&json).expect("Deserialize AisPositionReport failed");
    assert_eq!(deserialized.mmsi, report.mmsi);
}

#[test]
fn test_can_frame_cross_subsystem_conversion() {
    // 1. Automotive -> IIoT
    let auto_frame = AutoCanFrame::new_standard(0x123, &[0xAA, 0xBB, 0xCC]);
    let iiot_frame: IiotCanFrame = auto_frame.clone().into();
    assert_eq!(iiot_frame.id, CanId::Standard(0x123));
    assert_eq!(iiot_frame.dlc, 3);
    assert_eq!(iiot_frame.data[0], 0xAA);

    // 2. IIoT -> Automotive
    let auto_roundtrip: AutoCanFrame = iiot_frame.clone().into();
    assert_eq!(auto_roundtrip.id, 0x123);
    assert!(!auto_roundtrip.is_extended);
    assert_eq!(auto_roundtrip.data[1], 0xBB);

    // 3. RTOS -> IIoT
    let rtos_frame = RtosCanFrame {
        id: 0x456,
        is_extended: false,
        dlc: 2,
        data: [0x11, 0x22, 0, 0, 0, 0, 0, 0],
    };
    let iiot_from_rtos: IiotCanFrame = rtos_frame.clone().into();
    assert_eq!(iiot_from_rtos.id, CanId::Standard(0x456));
    assert_eq!(iiot_from_rtos.data[0], 0x11);

    // 4. IIoT -> RTOS
    let rtos_roundtrip: RtosCanFrame = iiot_from_rtos.into();
    assert_eq!(rtos_roundtrip.id, 0x456);
    assert_eq!(rtos_roundtrip.dlc, 2);

    // 5. Automotive <-> RTOS
    let rtos_from_auto: RtosCanFrame = auto_frame.clone().into();
    assert_eq!(rtos_from_auto.id, 0x123);
    let auto_from_rtos: AutoCanFrame = rtos_from_auto.into();
    assert_eq!(auto_from_rtos.id, 0x123);
}

#[test]
fn test_telecom_gtpu_bridge() {
    let payload = vec![0x45, 0x00, 0x00, 0x28]; // IPv4 header
    let gtp = GtpPacket::new_t_pdu(0x10203040, 9, payload.clone());

    // Convert into PureGtpuHeader + payload
    let (header, p): (PureGtpuHeader, Vec<u8>) = gtp.clone().into();
    assert_eq!(header.teid, 0x10203040);
    assert_eq!(header.qfi, Some(9));
    assert_eq!(p, payload);

    // Convert back into GtpPacket
    let gtp_roundtrip: GtpPacket = (header, p).into();
    assert_eq!(gtp_roundtrip.header.teid, 0x10203040);
    assert_eq!(gtp_roundtrip.header.message_type, GtpMessageType::TPdu);
    assert_eq!(gtp_roundtrip.payload, payload);
}

#[test]
fn test_neuro_lif_and_eeg_bridges() {
    // 1. LifNeuron <-> PureLifNeuron
    let legacy_neuron = LifNeuron::new();
    let mut pure_neuron: PureLifNeuron = legacy_neuron.into();
    assert_eq!(pure_neuron.v_membrane, -70.0);
    assert_eq!(pure_neuron.v_threshold, -50.0);

    // Step pure neuron
    let fired = pure_neuron.step(1.0, 25.0); // -70 + 25 = -45 >= -50 -> spike!
    assert!(fired);
    assert_eq!(pure_neuron.v_membrane, -75.0); // reset

    // Convert back to LifNeuron
    let roundtrip_neuron: LifNeuron = pure_neuron.into();
    assert_eq!(roundtrip_neuron.membrane_potential, -75.0);

    // 2. EegBandPowers <-> PureEegBandPower
    let signal: Vec<f64> = (0..256).map(|i| (2.0 * std::f64::consts::PI * 10.0 * (i as f64) / 256.0).sin()).collect();
    let pure_bands = PureBciEngine::compute_band_powers(&signal, 256.0);
    assert!(pure_bands.alpha > 0.0);

    let legacy_bands: EegBandPowers = pure_bands.into();
    assert!(legacy_bands.alpha_8_13hz > 0.0);
    assert!(legacy_bands.engagement_index >= 0.0);

    let pure_roundtrip: PureEegBandPower = legacy_bands.into();
    assert_eq!(pure_roundtrip.alpha, pure_bands.alpha);

    // 3. MotorImageryClass classification
    let cls = PureBciEngine::classify_motor_imagery(0.2, 0.8);
    assert_eq!(cls, MotorImageryClass::RightHand);
}

#[test]
fn test_game3d_mesh_to_software_rasterizer() {
    let cube = Mesh3D::cube(1.0);
    let mut rasterizer = SoftwareRasterizer::new(64, 64);
    let mut fb = ImageBuffer::new(64, 64, RgbaPixel::BLACK);

    let model = Mat4::IDENTITY;
    let view = Mat4::look_at(Vec3::new(0.0, 0.0, -3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
    let proj = Mat4::perspective_fov(std::f32::consts::PI / 3.0, 1.0, 0.1, 100.0);
    let vp = view.multiply(&proj);

    rasterizer.render_mesh3d(&cube, &model, &vp, &mut fb);

    // Pastikan rasterizer menghasilkan piksel yang di-render di framebuffer
    let non_black_pixels = fb.pixels.iter().filter(|p| **p != RgbaPixel::BLACK).count();
    assert!(non_black_pixels > 0, "Cube harus ter-render ke framebuffer pure rasterizer");
}

#[test]
fn test_space_ccsds_packet_and_frame_roundtrip() {
    let packet = CcsdsSpacePacket::new_telemetry(0x123, 42, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let tm_frame = packet.wrap_in_tm_transfer_frame(0x042, 1, 10, 5);

    assert_eq!(tm_frame.spacecraft_id, 0x042);
    assert_eq!(tm_frame.virtual_channel_id, 1);

    // Ekstrak kembali CcsdsSpacePacket dari data payload TM Transfer Frame
    let extracted = tm_frame.extract_space_packet().expect("Extract Space Packet failed");
    assert_eq!(extracted.header.apid, 0x123);
    assert_eq!(extracted.header.packet_sequence_count, 42);
    assert_eq!(extracted.user_data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
}


#[test]
fn test_quantum_circuit_execution_and_complex_conversion() {
    // 1. Complex <-> Complex64
    let c = Complex::new(0.7071, 0.0);
    let c64: Complex64 = c.into();
    assert_eq!(c64.re, 0.7071);
    let c_roundtrip: Complex = c64.into();
    assert_eq!(c_roundtrip.re, 0.7071);

    // 2. OpenQASM QuantumCircuit -> execute_pure
    let qasm = "
    OPENQASM 2.0;
    qreg q[2];
    h q[0];
    cx q[0], q[1];
    ";
    let circuit = QuantumCircuit::parse_qasm(qasm).expect("Parse QASM failed");
    let sim = circuit.execute_pure().expect("Execute pure failed");

    // Bell state (|00> + |11>) / sqrt(2)
    assert_eq!(sim.num_qubits, 2);
    assert!(sim.is_normalized(1e-5));
    let probs = sim.get_probabilities();
    assert!((probs[0] - 0.5).abs() < 1e-4); // |00> ~ 50%
    assert!((probs[3] - 0.5).abs() < 1e-4); // |11> ~ 50%
}


#[test]
fn test_zk_constraints_to_pure_circuit() {
    let constraints = vec![
        R1csConstraint {
            a_var: "x".to_string(),
            b_var: "y".to_string(),
            c_var: "z".to_string(),
        },
    ];

    let pure_circuit = ZkEngine::to_pure_circuit(&constraints);
    assert_eq!(pure_circuit.constraints.len(), 1);

    let mut witness = HashMap::new();
    witness.insert("x".to_string(), FieldElement::new(3));
    witness.insert("y".to_string(), FieldElement::new(7));
    witness.insert("z".to_string(), FieldElement::new(21));

    assert!(pure_circuit.is_satisfied(&witness));

    // Non-satisfying witness
    witness.insert("z".to_string(), FieldElement::new(22));
    assert!(!pure_circuit.is_satisfied(&witness));
}

#[test]
fn test_ebpf_emitter_pure_elf() {
    let mut emitter = widya::ebpf::EbpfEmitter::new();
    emitter.add_xdp_rule("TCP", 80, "XDP_DROP");
    let elf_bytes = emitter.emit_pure_elf();

    assert!(elf_bytes.len() >= 64);
    assert_eq!(&elf_bytes[0..4], &[0x7F, b'E', b'L', b'F']); // ELF magic
}
