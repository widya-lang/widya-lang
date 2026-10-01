//! Node Staking, Slashing & Reputation Engine for DePIN Grid (staking_slashing.rs)
//! Menjamin ketersediaan SLA 99.9% dan keamanan jaringan melalui sistem jaminan saldo (staking),
//! sanksi pemotongan saldo otomatis (slashing), dan skor reputasi node.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Alasan Pemotongan Saldo Jaminan (Slashing Reason)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlashingReason {
    TaskTimeout,
    CorruptedResult,
    MaliciousSybilAttempt,
    OfflineDuringCriticalJob,
}

/// Riwayat Sanksi Slashing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlashingRecord {
    pub node_id: String,
    pub job_id: String,
    pub reason: SlashingReason,
    pub slashed_amount_idr: u64,
    pub timestamp: u64,
}

/// Akun Staking & Reputasi Node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeStakeAccount {
    pub node_id: String,
    pub staked_balance_idr: u64,
    pub locked_in_jobs_idr: u64,
    pub successful_jobs_count: u32,
    pub failed_jobs_count: u32,
    pub reputation_score: f64, // 0.0 sampai 1.0 (1.0 = sempurna)
    pub is_slashed: bool,
}

impl NodeStakeAccount {
    pub fn new(node_id: &str, initial_stake_idr: u64) -> Self {
        Self {
            node_id: node_id.to_string(),
            staked_balance_idr: initial_stake_idr,
            locked_in_jobs_idr: 0,
            successful_jobs_count: 0,
            failed_jobs_count: 0,
            reputation_score: 1.0,
            is_slashed: false,
        }
    }

    pub fn available_stake(&self) -> u64 {
        self.staked_balance_idr.saturating_sub(self.locked_in_jobs_idr)
    }

    pub fn update_reputation(&mut self) {
        let total = self.successful_jobs_count + self.failed_jobs_count;
        if total == 0 {
            self.reputation_score = 1.0;
        } else {
            let ratio = (self.successful_jobs_count as f64) / (total as f64);
            self.reputation_score = (ratio * 0.9 + 0.1).clamp(0.0, 1.0);
        }
    }
}

/// Manajer Staking & Slashing DePIN
pub struct StakingSlashingManager {
    accounts: HashMap<String, NodeStakeAccount>,
    slashing_history: Vec<SlashingRecord>,
    min_stake_required_idr: u64,
    slashing_penalty_percent: f64,
}

impl StakingSlashingManager {
    pub fn new(min_stake_idr: u64, penalty_percent: f64) -> Self {
        Self {
            accounts: HashMap::new(),
            slashing_history: Vec::new(),
            min_stake_required_idr: min_stake_idr,
            slashing_penalty_percent: penalty_percent,
        }
    }

    /// Deposit saldo staking oleh node
    pub fn deposit_stake(&mut self, node_id: &str, amount_idr: u64) {
        let entry = self.accounts.entry(node_id.to_string())
            .or_insert_with(|| NodeStakeAccount::new(node_id, 0));
        entry.staked_balance_idr += amount_idr;
    }

    /// Kunci sejumlah jaminan saat node menerima tugas bernilai tinggi
    pub fn lock_stake_for_job(&mut self, node_id: &str, required_lock_idr: u64) -> Result<bool, String> {
        let acc = self.accounts.get_mut(node_id)
            .ok_or_else(|| format!("Akun staking untuk node '{}' belum terdaftar", node_id))?;

        if acc.available_stake() < required_lock_idr {
            return Err(format!(
                "Saldo staking node '{}' tidak mencukupi (Tersedia: Rp {}, Butuh: Rp {})",
                node_id, acc.available_stake(), required_lock_idr
            ));
        }

        acc.locked_in_jobs_idr += required_lock_idr;
        Ok(true)
    }

    /// Lepas kunci jaminan saat tugas berhasil diselesaikan
    pub fn release_stake_on_success(&mut self, node_id: &str, locked_amount_idr: u64) -> Result<(), String> {
        let acc = self.accounts.get_mut(node_id)
            .ok_or_else(|| format!("Akun staking untuk node '{}' tidak ditemukan", node_id))?;

        acc.locked_in_jobs_idr = acc.locked_in_jobs_idr.saturating_sub(locked_amount_idr);
        acc.successful_jobs_count += 1;
        acc.update_reputation();
        Ok(())
    }

    /// Eksekusi pemotongan saldo (Slashing) jika node melakukan pelanggaran
    pub fn execute_slash(
        &mut self,
        node_id: &str,
        job_id: &str,
        reason: SlashingReason,
        timestamp: u64,
    ) -> Result<u64, String> {
        let acc = self.accounts.get_mut(node_id)
            .ok_or_else(|| format!("Akun staking untuk node '{}' tidak ditemukan", node_id))?;

        let penalty_amount = ((acc.staked_balance_idr as f64) * (self.slashing_penalty_percent / 100.0)) as u64;
        let penalty_amount = penalty_amount.max(10_000); // Minimal penalti Rp 10.000
        let slashed = acc.staked_balance_idr.min(penalty_amount);

        acc.staked_balance_idr -= slashed;
        acc.locked_in_jobs_idr = 0;
        acc.failed_jobs_count += 1;
        acc.is_slashed = true;
        acc.update_reputation();

        self.slashing_history.push(SlashingRecord {
            node_id: node_id.to_string(),
            job_id: job_id.to_string(),
            reason,
            slashed_amount_idr: slashed,
            timestamp,
        });

        Ok(slashed)
    }

    pub fn get_account(&self, node_id: &str) -> Option<&NodeStakeAccount> {
        self.accounts.get(node_id)
    }

    pub fn get_slashing_history(&self) -> &[SlashingRecord] {
        &self.slashing_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_staking_and_success_flow() {
        let mut manager = StakingSlashingManager::new(50_000, 20.0);
        manager.deposit_stake("node_surabaya_01", 100_000);

        // Kunci Rp 30.000 untuk job
        assert!(manager.lock_stake_for_job("node_surabaya_01", 30_000).is_ok());
        let acc = manager.get_account("node_surabaya_01").unwrap();
        assert_eq!(acc.available_stake(), 70_000);

        // Selesaikan tugas
        assert!(manager.release_stake_on_success("node_surabaya_01", 30_000).is_ok());
        let acc = manager.get_account("node_surabaya_01").unwrap();
        assert_eq!(acc.available_stake(), 100_000);
        assert_eq!(acc.successful_jobs_count, 1);
        assert_eq!(acc.reputation_score, 1.0);
    }

    #[test]
    fn test_node_slashing_on_timeout() {
        let mut manager = StakingSlashingManager::new(50_000, 25.0); // 25% slash penalty
        manager.deposit_stake("node_nakal_02", 200_000);

        let slashed = manager.execute_slash("node_nakal_02", "job_critical_99", SlashingReason::TaskTimeout, 1720000000).unwrap();
        assert_eq!(slashed, 50_000); // 25% dari 200.000 = 50.000

        let acc = manager.get_account("node_nakal_02").unwrap();
        assert_eq!(acc.staked_balance_idr, 150_000);
        assert_eq!(acc.failed_jobs_count, 1);
        assert!(acc.reputation_score < 1.0);
    }
}
