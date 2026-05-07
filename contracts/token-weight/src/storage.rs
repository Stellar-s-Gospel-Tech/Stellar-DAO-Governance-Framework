use soroban_sdk::{contracttype, Address, Env};

use crate::types::WeightStrategy;
use crate::TokenWeightError;

// ── Storage keys ──────────────────────────────────────────────────────────────
//
// Contributor note:
// - All config uses instance storage (small, always needed).
// - Snapshot data (Phase 2) should use persistent storage keyed by ledger number.

#[contracttype]
pub enum DataKey {
    /// Address of the contract admin.
    Admin,
    /// The active WeightStrategy variant.
    Strategy,
    /// The SAC token (or LP token) whose balance determines voting weight.
    Token,
    // TODO (Phase 2): SnapshotBalance(Address, u32) — cached balance at a ledger.
    // TODO (Phase 3): SpendCap — if a capped strategy is added.
}

// ── Initializer ───────────────────────────────────────────────────────────────

/// Persist the initial config. Panics if already initialized.
///
/// Steps for contributor adding a new strategy (Phase 3):
/// 1. Add any strategy-specific config fields to a new `StrategyConfig` struct.
/// 2. Store it under a new `DataKey::StrategyConfig` entry here.
/// 3. Read it in `get_weight` to parameterize the calculation.
pub fn init(env: &Env, admin: &Address, strategy: WeightStrategy, token: &Address) {
    if env.storage().instance().has(&DataKey::Admin) {
        panic!("{}", TokenWeightError::AlreadyInitialized as u32);
    }
    env.storage().instance().set(&DataKey::Admin, admin);
    env.storage().instance().set(&DataKey::Strategy, &strategy);
    env.storage().instance().set(&DataKey::Token, token);
}

// ── Getters ───────────────────────────────────────────────────────────────────

pub fn strategy(env: &Env) -> WeightStrategy {
    env.storage().instance().get(&DataKey::Strategy).unwrap()
}

pub fn token(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Token).unwrap()
}

#[allow(dead_code)]
pub fn admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

// ── Snapshot helpers (Phase 2) ────────────────────────────────────────────────
//
// TODO: implement snapshot balance caching to prevent flash-loan attacks.
//
// The idea: when a proposal is created, the governance contract calls
// `record_snapshot(voter, ledger)` for each voter, or the weight contract
// reads archival state directly. On vote, `get_weight_at(voter, snapshot_ledger)`
// is called instead of the live balance.
//
// /// Cache the balance of `voter` at `ledger` in persistent storage.
// /// Steps:
// /// 1. Call `token::Client::new(env, &token(env)).balance(voter)`.
// /// 2. Store under DataKey::SnapshotBalance(voter.clone(), ledger).
// /// 3. Set a TTL long enough to cover the voting period.
// pub fn record_snapshot(env: &Env, voter: &Address, ledger: u32) { todo!() }
//
// /// Return the cached balance of `voter` at `ledger`.
// /// Panic with TokenWeightError::SnapshotExpired if the entry has been archived.
// pub fn get_snapshot(env: &Env, voter: &Address, ledger: u32) -> i128 { todo!() }
