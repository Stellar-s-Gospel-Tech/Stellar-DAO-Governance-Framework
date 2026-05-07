use soroban_sdk::{contracttype, Address, Env};

use crate::types::{SpendRecord, TreasuryConfig};
use crate::TreasuryError;

// ── Storage keys ──────────────────────────────────────────────────────────────
//
// Contributor note:
// - `Config` uses instance storage (small, frequently read).
// - `SpendRecord` uses persistent storage (must survive state archival).
// - Add `DailySpend(u32)` keyed by ledger-day when daily limits are implemented.

#[contracttype]
pub enum DataKey {
    /// Singleton treasury config (instance storage).
    Config,
    /// Running count of spend records; also used as the next record index.
    SpendCount,
    /// Spend audit log keyed by sequential index (persistent storage).
    SpendRecord(u64),
    // TODO (Phase 3): DailySpend(u32) — total spent in a given ledger-day window.
}

// ── Initializer ───────────────────────────────────────────────────────────────

/// Persist the initial treasury config. Panics if already initialized.
///
/// Steps for contributor extending this (Phase 3 — spend caps):
/// 1. Add `spend_cap_per_tx: i128` and `daily_spend_limit: i128` to `TreasuryConfig`.
/// 2. Accept them as parameters here.
/// 3. Add a `set_spend_cap` admin function in lib.rs to update them post-deploy.
pub fn init(env: &Env, admin: &Address, governance_contract: &Address) {
    if env.storage().instance().has(&DataKey::Config) {
        panic!("{}", TreasuryError::AlreadyInitialized as u32);
    }
    let config = TreasuryConfig {
        admin: admin.clone(),
        governance_contract: governance_contract.clone(),
    };
    env.storage().instance().set(&DataKey::Config, &config);
    env.storage().instance().set(&DataKey::SpendCount, &0u64);
}

// ── Config ────────────────────────────────────────────────────────────────────

pub fn config(env: &Env) -> TreasuryConfig {
    env.storage().instance().get(&DataKey::Config).unwrap()
}

/// Update the governance contract address. Admin only.
///
/// Steps:
/// 1. Read config, replace `governance_contract`, write back.
/// 2. Emit a config-updated event (Phase 2).
///
/// TODO: call this from `lib.rs::set_governance_contract` once implemented.
#[allow(dead_code)]
pub fn update_governance_contract(env: &Env, new_governance: &Address) {
    let mut cfg = config(env);
    cfg.governance_contract = new_governance.clone();
    env.storage().instance().set(&DataKey::Config, &cfg);
}

// ── Spend audit log ───────────────────────────────────────────────────────────

/// Append a spend record to the audit log.
///
/// Contributor note: when `proposal_id` is added to `SpendRecord` (Phase 2),
/// pass it through here from `lib.rs::spend`.
pub fn record_spend(env: &Env, token: &Address, recipient: &Address, amount: i128) {
    let count: u64 = env
        .storage()
        .instance()
        .get(&DataKey::SpendCount)
        .unwrap_or(0);
    let record = SpendRecord {
        token: token.clone(),
        recipient: recipient.clone(),
        amount,
        ledger: env.ledger().sequence(),
    };
    env.storage()
        .persistent()
        .set(&DataKey::SpendRecord(count), &record);
    env.storage()
        .instance()
        .set(&DataKey::SpendCount, &(count + 1));
}

pub fn spend_count(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::SpendCount)
        .unwrap_or(0)
}

pub fn get_spend_record(env: &Env, index: u64) -> Option<SpendRecord> {
    env.storage().persistent().get(&DataKey::SpendRecord(index))
}

// ── Daily limit helpers (Phase 3) ─────────────────────────────────────────────
//
// TODO: implement rolling daily spend tracking.
//
// /// Return the total amount spent in the current ledger-day window.
// /// Steps:
// /// 1. Compute `day_key = env.ledger().sequence() / LEDGERS_PER_DAY`.
// /// 2. Read DataKey::DailySpend(day_key) from instance storage (default 0).
// pub fn daily_spent(env: &Env) -> i128 { todo!() }
//
// /// Add `amount` to today's running total.
// pub fn accumulate_daily_spend(env: &Env, amount: i128) { todo!() }
