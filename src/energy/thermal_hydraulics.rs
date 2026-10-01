use serde::{Deserialize, Serialize};

/// ============================================================================
/// Small Modular Reactor (SMR) Thermal Hydraulics Simulation
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalHydraulicsState {
    pub mass_flow_rate_kg_s: f64,
    pub inlet_temperature_c: f64,
    pub outlet_temperature_c: f64,
    pub heat_generation_mw: f64,
    pub departure_from_nucleate_boiling_ratio: f64, // DNBR (>1.3 is safe)
}

pub struct SmrThermalHydraulicsSolver {
    pub specific_heat_capacity: f64, // kJ/(kg*K) ~ 4.184 for water
}

impl SmrThermalHydraulicsSolver {
    pub fn new() -> Self {
        Self {
            specific_heat_capacity: 5.2, // Pressurized hot water ~ 150 bar
        }
    }

    /// Hitung temperatur outlet pendingin dan margin DNBR
    pub fn step_simulation(&self, inlet_temp: f64, flow_rate: f64, power_mw: f64) -> ThermalHydraulicsState {
        let delta_t = (power_mw * 1000.0) / (flow_rate * self.specific_heat_capacity);
        let outlet_temp = inlet_temp + delta_t;
        let dnbr = (320.0 - inlet_temp) / (delta_t.max(1.0)) * 1.5;

        ThermalHydraulicsState {
            mass_flow_rate_kg_s: flow_rate,
            inlet_temperature_c: inlet_temp,
            outlet_temperature_c: outlet_temp,
            heat_generation_mw: power_mw,
            departure_from_nucleate_boiling_ratio: dnbr,
        }
    }
}
