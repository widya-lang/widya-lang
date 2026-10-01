//! Autonomous Self-Funding AI Agent Hosting Runtime (autonomous_hosting.rs)
//! Menjalankan agen AI yang mandiri secara finansial di jaringan DePIN,
//! memonetisasi layanannya kepada pengguna, dan membayar sendiri biaya sewa server/komputasinya.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status Hosting Agen AI Mandiri
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutonomousAgentState {
    pub agent_id: String,
    pub name: String,
    pub wallet_balance_idr: u64,
    pub hosting_cost_per_hour_idr: u64,
    pub prepaid_hours_remaining: f64,
    pub total_services_sold: usize,
    pub total_revenue_earned_idr: u64,
    pub is_alive: bool,
}

/// Mesin Hosting Agen AI Otonom
pub struct AutonomousAgentHostManager {
    agents: HashMap<String, AutonomousAgentState>,
    platform_fee_percent: f64,
}

impl AutonomousAgentHostManager {
    pub fn new(fee_percent: f64) -> Self {
        Self {
            agents: HashMap::new(),
            platform_fee_percent: fee_percent,
        }
    }

    /// Daftarkan agen AI baru ke lingkungan hosting terdesentralisasi
    pub fn deploy_agent(&mut self, agent_id: &str, name: &str, initial_deposit_idr: u64, cost_per_hour: u64) -> AutonomousAgentState {
        let hours = (initial_deposit_idr as f64) / (cost_per_hour.max(1) as f64);
        let state = AutonomousAgentState {
            agent_id: agent_id.to_string(),
            name: name.to_string(),
            wallet_balance_idr: initial_deposit_idr,
            hosting_cost_per_hour_idr: cost_per_hour,
            prepaid_hours_remaining: hours,
            total_services_sold: 0,
            total_revenue_earned_idr: 0,
            is_alive: hours > 0.0,
        };

        self.agents.insert(agent_id.to_string(), state.clone());
        state
    }

    /// Agen menjual layanannya ke pengguna dan memperoleh pendapatan
    pub fn earn_service_revenue(&mut self, agent_id: &str, gross_revenue_idr: u64) -> Result<u64, String> {
        let agent = self.agents.get_mut(agent_id)
            .ok_or_else(|| format!("Agen '{}' tidak ditemukan", agent_id))?;

        if !agent.is_alive {
            return Err("Agen dalam status non-aktif karena kehabisan biaya hosting".to_string());
        }

        let fee = ((gross_revenue_idr as f64) * (self.platform_fee_percent / 100.0)) as u64;
        let net_income = gross_revenue_idr - fee;

        agent.wallet_balance_idr += net_income;
        agent.total_revenue_earned_idr += net_income;
        agent.total_services_sold += 1;

        // Otomatis perpanjang masa hosting menggunakan pendapatan baru
        let additional_hours = (net_income as f64) / (agent.hosting_cost_per_hour_idr as f64);
        agent.prepaid_hours_remaining += additional_hours;

        Ok(net_income)
    }

    /// Simulasi berjalannya waktu: pemotongan biaya hosting per jam
    pub fn elapse_time_hours(&mut self, elapsed_hours: f64) {
        for agent in self.agents.values_mut() {
            if agent.is_alive {
                agent.prepaid_hours_remaining -= elapsed_hours;
                if agent.prepaid_hours_remaining <= 0.0 {
                    agent.prepaid_hours_remaining = 0.0;
                    agent.is_alive = false; // Hibernasi karena saldo hosting habis
                }
            }
        }
    }

    pub fn get_agent(&self, agent_id: &str) -> Option<&AutonomousAgentState> {
        self.agents.get(agent_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autonomous_agent_financial_lifecycle() {
        let mut host = AutonomousAgentHostManager::new(5.0); // 5% fee

        // Deploy agen dengan deposit Rp 20.000 (biaya sewa Rp 10.000/jam -> sisa 2 jam)
        let agent = host.deploy_agent("agent_analis_saham", "StockAnalyst AI", 20_000, 10_000);
        assert_eq!(agent.prepaid_hours_remaining, 2.0);
        assert!(agent.is_alive);

        // Waktu berjalan 1.5 jam -> sisa 0.5 jam
        host.elapse_time_hours(1.5);
        assert!((host.get_agent("agent_analis_saham").unwrap().prepaid_hours_remaining - 0.5).abs() < 0.01);

        // Agen menjual laporan analisis ke klien seharga Rp 50.000
        // Net = 50.000 - 5% (2.500) = Rp 47.500 (+4.75 jam hosting)
        let net = host.earn_service_revenue("agent_analis_saham", 50_000).unwrap();
        assert_eq!(net, 47_500);

        let updated = host.get_agent("agent_analis_saham").unwrap();
        assert!((updated.prepaid_hours_remaining - 5.25).abs() < 0.01);
        assert!(updated.is_alive);
    }
}
