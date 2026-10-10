use std::f64::consts::PI;
use widya::neuro::pure_neuro::{
    MotorImageryClass, PureBciEngine, PureLifNeuron, SnnNetwork, StdpSynapse,
};

#[test]
fn test_lif_neuron_dynamics_and_refractory() {
    let mut neuron = PureLifNeuron::new(-70.0, -50.0, -75.0, 10.0);

    // Sub-threshold stimulus: potential increases but does not fire
    let fired = neuron.step(1.0, 5.0);
    assert!(!fired);
    assert!(neuron.v_membrane > -70.0);

    // Strong stimulus: potential reaches >= -50.0 and fires spike!
    let fired_strong = neuron.step(1.0, 30.0);
    assert!(fired_strong);
    assert_eq!(neuron.v_membrane, -75.0); // Reset potential
    assert!(neuron.current_refractory > 0);

    // During refractory period, stimulus is suppressed
    let fired_during_refractory = neuron.step(1.0, 100.0);
    assert!(!fired_during_refractory);
    assert_eq!(neuron.v_membrane, -75.0);
}

#[test]
fn test_stdp_synaptic_plasticity_ltp_and_ltd() {
    let mut syn = StdpSynapse::new(0, 1, 1.0);
    let initial_w = syn.weight;

    // 1. Long-Term Potentiation (LTP): Pre fires at t=10ms, Post fires at t=20ms (Pre before Post)
    syn.on_pre_spike(10.0);
    syn.on_post_spike(20.0);
    assert!(syn.weight > initial_w, "STDP LTP should increase synaptic weight");

    // 2. Long-Term Depression (LTD): Post fires at t=50ms, Pre fires at t=60ms (Post before Pre)
    let w_before_ltd = syn.weight;
    syn.on_post_spike(50.0);
    syn.on_pre_spike(60.0);
    assert!(syn.weight < w_before_ltd, "STDP LTD should decrease synaptic weight");
}

#[test]
fn test_snn_network_simulation() {
    let mut net = SnnNetwork::new(2);
    net.connect(0, 1, 0.5);

    // Run 50 steps of 1ms simulation
    let mut total_spikes = 0;
    for _ in 0..50 {
        let spikes = net.step(1.0, &[25.0, 0.0]); // stimulate neuron 0
        if spikes[0] {
            total_spikes += 1;
        }
    }
    assert!(total_spikes > 0, "Stimulated neuron must fire spikes");
}

#[test]
fn test_eeg_spectral_decomposition() {
    let sample_rate = 250.0; // 250 Hz
    let duration_s = 1.0;
    let n_samples = (sample_rate * duration_s) as usize;

    // Generate pure 10 Hz sinusoidal wave (Alpha band: 8-13 Hz)
    let mut alpha_signal = Vec::with_capacity(n_samples);
    for t in 0..n_samples {
        let time = t as f64 / sample_rate;
        let val = (2.0 * PI * 10.0 * time).sin();
        alpha_signal.push(val);
    }

    let powers = PureBciEngine::compute_band_powers(&alpha_signal, sample_rate);
    assert!(powers.total_power > 0.0);
    // Alpha power should dominate the signal (> 75% of total power)
    let alpha_ratio = powers.alpha / powers.total_power;
    assert!(alpha_ratio > 0.75, "10 Hz signal must be dominant in Alpha band, got ratio {}", alpha_ratio);
}

#[test]
fn test_bci_motor_imagery_classification() {
    // Right hand motor imagery causes contralateral Mu/Alpha desynchronization:
    // C3 (left motor cortex) power drops, so C4 power is relatively higher
    let class_right = PureBciEngine::classify_motor_imagery(0.2, 0.8);
    assert_eq!(class_right, MotorImageryClass::RightHand);

    // Left hand motor imagery causes C4 power to drop, so C3 is relatively higher
    let class_left = PureBciEngine::classify_motor_imagery(0.8, 0.2);
    assert_eq!(class_left, MotorImageryClass::LeftHand);

    // Balanced state
    let class_rest = PureBciEngine::classify_motor_imagery(0.5, 0.5);
    assert_eq!(class_rest, MotorImageryClass::Rest);
}
