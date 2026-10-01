use serde::{Deserialize, Serialize};

/// ============================================================================
/// Radiation-Hardened Target Architecture & Triple Modular Redundancy (TMR)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadHardArch {
    BaeRad750,      // PowerPC Rad-Hard standard (Mars Rovers / James Webb)
    VoragoRiscV,    // RISC-V HARDSIL process
    GaislerLeon4,   // SPARC V8 Quad-core Rad-Hard (ESA missions)
    MicroBlazeTmr,  // FPGA Triple Modular Redundancy
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmrMemoryCell<T: Clone + PartialEq> {
    pub copy_a: T,
    pub copy_b: T,
    pub copy_c: T,
    pub single_event_upset_count: u64,
}

impl<T: Clone + PartialEq> TmrMemoryCell<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            copy_a: initial_value.clone(),
            copy_b: initial_value.clone(),
            copy_c: initial_value,
            single_event_upset_count: 0,
        }
    }

    /// Majority voting logic dengan auto-scrubbing (perbaikan bit-flip radiasi kosmik)
    pub fn read_and_scrub(&mut self) -> T {
        if self.copy_a == self.copy_b {
            if self.copy_c != self.copy_a {
                self.copy_c = self.copy_a.clone();
                self.single_event_upset_count += 1;
            }
            self.copy_a.clone()
        } else if self.copy_a == self.copy_c {
            self.copy_b = self.copy_a.clone();
            self.single_event_upset_count += 1;
            self.copy_a.clone()
        } else if self.copy_b == self.copy_c {
            self.copy_a = self.copy_b.clone();
            self.single_event_upset_count += 1;
            self.copy_b.clone()
        } else {
            // Triple fault fallback
            self.copy_a.clone()
        }
    }

    pub fn write(&mut self, val: T) {
        self.copy_a = val.clone();
        self.copy_b = val.clone();
        self.copy_c = val;
    }
}
