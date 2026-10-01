use serde::{Deserialize, Serialize};

/// ============================================================================
/// Neuromorphic Chip Hardware Abstraction Layer (Intel Loihi / SynSense)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphicCoreConfig {
    pub core_id: u32,
    pub num_neurons: u32,
    pub num_synapses: u32,
    pub voltage_precision_bits: u8,
    pub weight_precision_bits: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikePacket {
    pub src_core: u32,
    pub src_neuron: u32,
    pub dst_core: u32,
    pub dst_neuron: u32,
    pub timestamp_tick: u64,
}

pub struct NeuromorphicHal {
    pub total_cores: u32,
    pub clock_frequency_mhz: f64,
    pub active_spikes: Vec<SpikePacket>,
}

impl NeuromorphicHal {
    pub fn new(cores: u32) -> Self {
        Self {
            total_cores: cores,
            clock_frequency_mhz: 100.0,
            active_spikes: Vec::new(),
        }
    }

    pub fn route_spike(&mut self, src_c: u32, src_n: u32, dst_c: u32, dst_n: u32, tick: u64) {
        self.active_spikes.push(SpikePacket {
            src_core: src_c,
            src_neuron: src_n,
            dst_core: dst_c,
            dst_neuron: dst_n,
            timestamp_tick: tick,
        });
    }
}
