use soroban_sdk::{contracttype, Address};

/// Singleton config stored in instance storage.
///
/// Contributor note — fields to add in later phases:
/// - `spend_cap_per_tx: i128`   — max tokens per single spend call (Phase 3).
/// - `daily_spend_limit: i128`  — rolling 24h spend ceiling (Phase 3).
/// - `timelock_ledgers: u32`    — delay between approval and execution (Phase 3).
#[contracttype]
#[derive(Clone, Debug)]
pub struct TreasuryConfig {
    /// Address that can update config (e.g. rotate governance contract).
    pub admin: Address,
    /// Only this address may call `spend()`.
    /// Should be the deployed governance contract.
    pub governance_contract: Address,
    // TODO (Phase 3): pub spend_cap_per_tx: i128,
    // TODO (Phase 3): pub daily_spend_limit: i128,
}

/// Immutable record of a single spend event, stored for auditability.
///
/// Contributor note: add `proposal_id: u64` (Phase 2) so each spend can be
/// traced back to the proposal that authorized it.
#[contracttype]
#[derive(Clone, Debug)]
pub struct SpendRecord {
    pub token: Address,
    pub recipient: Address,
    pub amount: i128,
    /// Ledger sequence at which the spend occurred.
    pub ledger: u32,
    // TODO (Phase 2): pub proposal_id: u64,
}
