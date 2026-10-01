//! Autonomous Agent Skill Market & Micro-Settlement Protocol (agent_market.rs)
//! Memungkinkan agen AI bertransaksi, mempublikasikan keterampilan spesialis, dan membayar royalti mikro secara otonom.

use std::collections::HashMap;

/// Keterampilan Agen yang Ditawarkan di Pasar (Agent Skill Listing)
#[derive(Debug, Clone, PartialEq)]
pub struct AgentSkill {
    pub skill_id: String,
    pub provider_agent_id: String,
    pub description: String,
    pub price_per_call_idr: u64, // Biaya per panggilan dalam Rupiah
}

/// Transaksi Pembayaran Mikro Antar-Agen
#[derive(Debug, Clone, PartialEq)]
pub struct MicroSettlement {
    pub tx_id: String,
    pub payer_agent_id: String,
    pub payee_agent_id: String,
    pub amount_idr: u64,
    pub settled: bool,
}

/// Pasar Keterampilan & Settlement Engine
pub struct AgentSkillMarket {
    skills: HashMap<String, AgentSkill>,
    balances: HashMap<String, u64>,
}

impl AgentSkillMarket {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            balances: HashMap::new(),
        }
    }

    pub fn set_agent_balance(&mut self, agent_id: &str, balance_idr: u64) {
        self.balances.insert(agent_id.to_string(), balance_idr);
    }

    pub fn register_skill(&mut self, skill: AgentSkill) {
        self.skills.insert(skill.skill_id.clone(), skill);
    }

    /// Eksekusi pemanggilan skill dengan penyelesaian transaksi mikro otomatis
    pub fn invoke_skill(&mut self, caller_agent_id: &str, skill_id: &str) -> Result<MicroSettlement, String> {
        let skill = self.skills.get(skill_id)
            .ok_or_else(|| format!("Skill '{}' tidak ditemukan di marketplace", skill_id))?
            .clone();

        let caller_balance = self.balances.get(caller_agent_id).copied().unwrap_or(0);
        if caller_balance < skill.price_per_call_idr {
            return Err(format!("Saldo agen '{}' tidak mencukupi untuk memanggil skill (Butuh Rp {}, Ada Rp {})",
                caller_agent_id, skill.price_per_call_idr, caller_balance));
        }

        // Potong saldo pemanggil dan tambahkan saldo penyedia skill
        self.balances.insert(caller_agent_id.to_string(), caller_balance - skill.price_per_call_idr);
        let payee_balance = self.balances.get(&skill.provider_agent_id).copied().unwrap_or(0);
        self.balances.insert(skill.provider_agent_id.clone(), payee_balance + skill.price_per_call_idr);

        let tx = MicroSettlement {
            tx_id: format!("settle_{:x}", caller_balance ^ skill.price_per_call_idr),
            payer_agent_id: caller_agent_id.to_string(),
            payee_agent_id: skill.provider_agent_id,
            amount_idr: skill.price_per_call_idr,
            settled: true,
        };

        Ok(tx)
    }

    pub fn get_balance(&self, agent_id: &str) -> u64 {
        self.balances.get(agent_id).copied().unwrap_or(0)
    }
}
