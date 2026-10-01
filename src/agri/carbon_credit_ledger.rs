use serde::{Deserialize, Serialize};

/// ============================================================================
/// Regenerative Agriculture Carbon Offset MRV Ledger (ISO 14064-2)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonOffsetCertificate {
    pub certificate_id: String,
    pub farm_id: String,
    pub area_hectares: f64,
    pub soil_organic_carbon_increase_pct: f64,
    pub sequestered_co2_tons: f64,
    pub verifier_agency: String,
    pub is_retired: bool,
}

pub struct AgriCarbonLedger {
    pub certificates: Vec<CarbonOffsetCertificate>,
}

impl AgriCarbonLedger {
    pub fn new() -> Self {
        Self {
            certificates: Vec::new(),
        }
    }

    /// Kalkulasi penyerapan karbon tanah berdasarkan peningkatan fraksi SOC (Soil Organic Carbon)
    pub fn calculate_and_issue(
        &mut self,
        farm_id: &str,
        area_ha: f64,
        delta_soc_pct: f64,
        soil_bulk_density_g_cm3: f64,
    ) -> CarbonOffsetCertificate {
        // Rumus IPCC Tier 2: CO2 = Area (ha) * Depth (0.3m) * Bulk Density * Delta SOC (%) * 3.67 (C to CO2 factor) * 100
        let depth_m = 0.30;
        let soil_mass_tons_per_ha = area_ha * depth_m * 10000.0 * soil_bulk_density_g_cm3;
        let carbon_tons = soil_mass_tons_per_ha * (delta_soc_pct / 100.0);
        let co2_tons = carbon_tons * 3.6667;

        let cert = CarbonOffsetCertificate {
            certificate_id: format!("AGRI-CO2-{}-{}", farm_id, self.certificates.len() + 1),
            farm_id: farm_id.to_string(),
            area_hectares: area_ha,
            soil_organic_carbon_increase_pct: delta_soc_pct,
            sequestered_co2_tons: co2_tons,
            verifier_agency: "Widya MRV Carbon Verifier".to_string(),
            is_retired: false,
        };

        self.certificates.push(cert.clone());
        cert
    }
}
