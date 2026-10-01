//! Decentralized Autonomous Governance & DePIN DAO Engine (governance_dao.rs)
//! Mengelola pengajuan proposal on-chain, pemungutan suara (voting) berbobot kontribusi komputasi/stake,
//! dan eksekusi otomatis pembaruan parameter jaringan DePIN.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status Proposal Tata Kelola
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Voting,
    Passed,
    Rejected,
    Executed,
}

/// Tindakan yang Diajukan dalam Proposal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GovernanceAction {
    UpdateSlashingPenaltyPercent(f64),
    UpdatePlatformTaxRate(f64),
    UpdateMinStakeRequired(u64),
    WhitelistAiModel(String),
}

/// Proposal Tata Kelola On-Chain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaoProposal {
    pub proposal_id: usize,
    pub title: String,
    pub proposer_wallet: String,
    pub action: GovernanceAction,
    pub yes_votes_weight: f64,
    pub no_votes_weight: f64,
    pub voters: HashMap<String, bool>, // wallet -> vote (true=yes, false=no)
    pub status: ProposalStatus,
    pub end_timestamp: u64,
}

/// Mesin Tata Kelola DePIN DAO
pub struct DepinDaoEngine {
    proposals: Vec<DaoProposal>,
    quorum_weight_threshold: f64,
}

impl DepinDaoEngine {
    pub fn new(quorum_threshold: f64) -> Self {
        Self {
            proposals: Vec::new(),
            quorum_weight_threshold: quorum_threshold,
        }
    }

    /// Buat proposal tata kelola baru
    pub fn create_proposal(
        &mut self,
        title: &str,
        proposer: &str,
        action: GovernanceAction,
        duration_secs: u64,
        now: u64,
    ) -> usize {
        let id = self.proposals.len();
        self.proposals.push(DaoProposal {
            proposal_id: id,
            title: title.to_string(),
            proposer_wallet: proposer.to_string(),
            action,
            yes_votes_weight: 0.0,
            no_votes_weight: 0.0,
            voters: HashMap::new(),
            status: ProposalStatus::Voting,
            end_timestamp: now + duration_secs,
        });
        id
    }

    /// Lakukan pemungutan suara dengan bobot komputasi/stake pemilik wallet
    pub fn cast_vote(&mut self, proposal_id: usize, voter_wallet: &str, support_yes: bool, weight: f64) -> Result<bool, String> {
        let prop = self.proposals.get_mut(proposal_id)
            .ok_or_else(|| format!("Proposal ID {} tidak ditemukan", proposal_id))?;

        if prop.status != ProposalStatus::Voting {
            return Err("Proposal tidak lagi menerima voting".to_string());
        }

        if prop.voters.contains_key(voter_wallet) {
            return Err("Wallet ini sudah memberikan suara sebelumnya".to_string());
        }

        prop.voters.insert(voter_wallet.to_string(), support_yes);
        if support_yes {
            prop.yes_votes_weight += weight;
        } else {
            prop.no_votes_weight += weight;
        }

        Ok(true)
    }

    /// Hitung hasil voting dan tetapkan status kelulusan proposal
    pub fn finalize_proposal(&mut self, proposal_id: usize) -> Result<ProposalStatus, String> {
        let prop = self.proposals.get_mut(proposal_id)
            .ok_or_else(|| format!("Proposal ID {} tidak ditemukan", proposal_id))?;

        let total_weight = prop.yes_votes_weight + prop.no_votes_weight;
        if total_weight < self.quorum_weight_threshold {
            prop.status = ProposalStatus::Rejected; // Gagal mencapai kuorum
        } else if prop.yes_votes_weight > prop.no_votes_weight {
            prop.status = ProposalStatus::Passed;
        } else {
            prop.status = ProposalStatus::Rejected;
        }

        Ok(prop.status)
    }

    pub fn get_proposal(&self, id: usize) -> Option<&DaoProposal> {
        self.proposals.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dao_governance_voting_flow() {
        let mut dao = DepinDaoEngine::new(100.0); // Kuorum minimal 100 TFLOPS/Stake weight

        let pid = dao.create_proposal(
            "Turunkan Pajak Platform Menjadi 0.2%",
            "wallet_komunitas_01",
            GovernanceAction::UpdatePlatformTaxRate(0.2),
            3600,
            1720000000,
        );

        // Node Surabaya dengan 70 weight vote YES
        assert!(dao.cast_vote(pid, "wallet_surabaya", true, 70.0).is_ok());

        // Node Bandung dengan 40 weight vote YES
        assert!(dao.cast_vote(pid, "wallet_bandung", true, 40.0).is_ok());

        // Node Medan dengan 20 weight vote NO
        assert!(dao.cast_vote(pid, "wallet_medan", false, 20.0).is_ok());

        // Total weight: 130 >= 100 (kuorum tercapai). YES = 110, NO = 20
        let result = dao.finalize_proposal(pid).unwrap();
        assert_eq!(result, ProposalStatus::Passed);

        let prop = dao.get_proposal(pid).unwrap();
        assert_eq!(prop.yes_votes_weight, 110.0);
        assert_eq!(prop.no_votes_weight, 20.0);
    }
}
