use serde::{Deserialize, Serialize};

/// ============================================================================
/// Lab Streaming Layer (LSL) Protocol for EEG / EMG / Multimodal Biosensors
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LslChannelFormat {
    pub label: String,
    pub unit: String,
    pub channel_type: String, // "EEG", "EMG", "EOG", "ECG"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LslStreamInfo {
    pub name: String,
    pub stream_type: String,
    pub channel_count: usize,
    pub nominal_srate_hz: f64,
    pub source_id: String,
    pub channels: Vec<LslChannelFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LslSampleChunk {
    pub stream_name: String,
    pub timestamp_epoch_sec: f64,
    pub samples: Vec<f64>, // Length must match channel_count
}

impl LslStreamInfo {
    pub fn new_standard_10_20_eeg(name: &str, srate: f64) -> Self {
        let standard_channels = vec![
            "Fp1", "Fp2", "F7", "F3", "Fz", "F4", "F8",
            "T3", "C3", "Cz", "C4", "T4",
            "T5", "P3", "Pz", "P4", "T6",
            "O1", "O2",
        ];

        let channels = standard_channels
            .into_iter()
            .map(|ch| LslChannelFormat {
                label: ch.to_string(),
                unit: "microvolts".to_string(),
                channel_type: "EEG".to_string(),
            })
            .collect();

        Self {
            name: name.to_string(),
            stream_type: "EEG".to_string(),
            channel_count: 19,
            nominal_srate_hz: srate,
            source_id: "WIDYA_BCI_10_20".to_string(),
            channels,
        }
    }
}
