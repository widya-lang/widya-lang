use serde::{Deserialize, Serialize};

/// ============================================================================
/// IEEE 802.1Qbv Time-Aware Shaper (TAS) TSN Scheduler
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateControlEntry {
    pub gate_states: u8,       // Bitmask 8 queues (1 = Open, 0 = Closed)
    pub time_interval_ns: u32, // Durasi time slice dalam nanodetik
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsnSchedule {
    pub cycle_time_ns: u32,
    pub admin_control_list: Vec<GateControlEntry>,
}

pub struct TsnTimeAwareShaper {
    pub port_name: String,
    pub schedule: TsnSchedule,
    pub current_time_offset_ns: u32,
    pub current_entry_index: usize,
}

impl TsnTimeAwareShaper {
    pub fn new(port: &str, cycle_time_ns: u32) -> Self {
        Self {
            port_name: port.to_string(),
            schedule: TsnSchedule {
                cycle_time_ns,
                admin_control_list: Vec::new(),
            },
            current_time_offset_ns: 0,
            current_entry_index: 0,
        }
    }

    pub fn add_gate_slot(&mut self, gate_states: u8, interval_ns: u32) {
        self.schedule.admin_control_list.push(GateControlEntry {
            gate_states,
            time_interval_ns: interval_ns,
        });
    }

    /// Cek apakah antrean prioritas tertentu (0..7) sedang diizinkan transmisi pada waktu saat ini
    pub fn is_queue_open(&self, queue_index: u8, offset_ns: u32) -> bool {
        if self.schedule.admin_control_list.is_empty() || self.schedule.cycle_time_ns == 0 {
            return true;
        }

        let cycle_offset = offset_ns % self.schedule.cycle_time_ns;
        let mut accumulated_ns = 0;

        for entry in &self.schedule.admin_control_list {
            accumulated_ns += entry.time_interval_ns;
            if cycle_offset < accumulated_ns {
                let mask = 1 << (queue_index & 0x07);
                return (entry.gate_states & mask) != 0;
            }
        }

        false
    }
}
