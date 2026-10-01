use serde::{Deserialize, Serialize};

/// ============================================================================
/// SEG-Y Rev 2.0 Binary Seismic Reflection Data Parser
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegyBinaryHeader {
    pub sample_interval_us: u16,
    pub samples_per_trace: u16,
    pub data_sample_format: u16, // 1 = 4-byte IBM floating point, 5 = 4-byte IEEE floating point
    pub traces_per_ensemble: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeismicTrace {
    pub trace_sequence_number: u32,
    pub source_x: i32,
    pub source_y: i32,
    pub group_x: i32,
    pub group_y: i32,
    pub samples: Vec<f32>,
}

pub struct SegyDataset {
    pub binary_header: SegyBinaryHeader,
    pub traces: Vec<SeismicTrace>,
}

impl SegyDataset {
    pub fn new(sample_interval: u16, samples_per_trace: u16) -> Self {
        Self {
            binary_header: SegyBinaryHeader {
                sample_interval_us: sample_interval,
                samples_per_trace,
                data_sample_format: 5, // IEEE float
                traces_per_ensemble: 1,
            },
            traces: Vec::new(),
        }
    }

    pub fn add_trace(&mut self, seq: u32, sx: i32, sy: i32, gx: i32, gy: i32, samples: Vec<f32>) {
        self.traces.push(SeismicTrace {
            trace_sequence_number: seq,
            source_x: sx,
            source_y: sy,
            group_x: gx,
            group_y: gy,
            samples,
        });
    }
}
