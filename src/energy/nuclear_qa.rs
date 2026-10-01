use serde::{Deserialize, Serialize};

/// ============================================================================
/// ASME NQA-1 Nuclear Quality Assurance & Reactor Safety Interlock
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyIntegrityState {
    NormalOperation,
    AlertCaution,
    ScramEmergencyTrip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearCoreTelemetry {
    pub core_temperature_celsius: f64,
    pub neutron_flux_fraction: f64, // 0.0 to 1.5 (1.0 = Nominal full power)
    pub primary_coolant_pressure_bar: f64,
    pub control_rod_insertion_pct: f64,
    pub safety_state: SafetyIntegrityState,
}

pub struct ReactorTripSystem {
    pub max_safe_temp_celsius: f64,
    pub max_neutron_flux: f64,
    pub min_coolant_pressure_bar: f64,
}

impl ReactorTripSystem {
    pub fn new() -> Self {
        Self {
            max_safe_temp_celsius: 345.0, // Typical PWR / SMR upper safety limit
            max_neutron_flux: 1.08,      // 108% overpower trip
            min_coolant_pressure_bar: 135.0,
        }
    }

    /// Evaluasi kondisi reaktor untuk SCRAM / Emergency Trip otomatis (ASME NQA-1)
    pub fn evaluate_scram(&self, telem: &mut NuclearCoreTelemetry) -> bool {
        let over_temp = telem.core_temperature_celsius > self.max_safe_temp_celsius;
        let over_flux = telem.neutron_flux_fraction > self.max_neutron_flux;
        let low_pressure = telem.primary_coolant_pressure_bar < self.min_coolant_pressure_bar;

        if over_temp || over_flux || low_pressure {
            telem.safety_state = SafetyIntegrityState::ScramEmergencyTrip;
            telem.control_rod_insertion_pct = 100.0; // Jatuhkan semua batang kendali gravitasi
            true
        } else if telem.core_temperature_celsius > (self.max_safe_temp_celsius - 15.0) {
            telem.safety_state = SafetyIntegrityState::AlertCaution;
            false
        } else {
            telem.safety_state = SafetyIntegrityState::NormalOperation;
            false
        }
    }
}
