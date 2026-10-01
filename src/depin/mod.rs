//! Widya Sovereign DePIN & Private Blockchain Compute Grid (mod.rs)
//! Menyediakan integrasi perangkat WidyaOS, distribusi tugas AI terdesentralisasi,
//! dan pembagian hasil komputasi berbasis Proof-of-Useful-Work (PoUW).

pub mod compute_ledger;
pub mod staking_slashing;
pub mod thermal_battery_guard;
pub mod content_addressable_store;
pub mod spot_auction;
pub mod governance_dao;
pub mod tui_dashboard;

pub use compute_ledger::{
    DepinComputeGridEngine,
    WorkerDeviceNode,
    ComputeJob,
    ComputeChunk,
    JobStatus,
    ProofOfUsefulWork,
    BlockchainTransaction,
    PrivateBlock,
};
pub use staking_slashing::{
    StakingSlashingManager,
    NodeStakeAccount,
    SlashingReason,
    SlashingRecord,
};
pub use thermal_battery_guard::{
    ThermalBatteryGuard,
    DeviceHardwareState,
    ComputeThrottlePolicy,
};
pub use content_addressable_store::{
    ContentAddressableStore,
    DistributedModelManifest,
    ModelChunkMeta,
};
pub use spot_auction::{
    SpotComputeAuctionEngine,
    ComputeOrder,
    OrderType,
    MatchedTrade,
};
pub use governance_dao::{
    DepinDaoEngine,
    DaoProposal,
    ProposalStatus,
    GovernanceAction,
};
pub use tui_dashboard::{
    DepinTuiDashboard,
    TuiNodeMetricsSnapshot,
};



