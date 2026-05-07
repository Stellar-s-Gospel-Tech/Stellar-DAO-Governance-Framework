#![no_std]

mod error;
mod storage;
mod types;

pub use error::TreasuryError;
pub use types::{SpendRecord, TreasuryConfig};

use soroban_sdk::{contract, contractimpl, token, Address, Env};

/// TreasuryContract holds DAO funds and releases them only on instruction
/// from the governance contract (i.e. an executed proposal).
///
/// Supports any SAC-compatible token (XLM, USDC, etc.).
///
/// # Contributor guide
///
/// Phase 2 tasks:
/// - Add `proposal_id: u64` parameter to `spend()` and store it in `SpendRecord`.
/// - Add `set_governance_contract()` so the DAO can rotate the governance address.
///
/// Phase 3 tasks:
/// - Add per-tx spend cap: read `config.spend_cap_per_tx` in `spend()` and
///   panic with `TreasuryError::SpendCapExceeded` if `amount` exceeds it.
/// - Add daily spend limit: accumulate via `storage::accumulate_daily_spend()`
///   and panic with `TreasuryError::DailyLimitExceeded` if exceeded.
/// - Add `upgrade()` for WASM upgrades (admin only).
#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    /// One-time initializer.
    ///
    /// - `admin`               — can rotate the governance contract address.
    /// - `governance_contract` — the only address allowed to call `spend()`.
    pub fn initialize(env: Env, admin: Address, governance_contract: Address) {
        storage::init(&env, &admin, &governance_contract);
    }

    /// Transfer `amount` of `token` to `recipient`.
    ///
    /// Only callable by the governance contract (enforced via `require_auth`).
    ///
    /// TODO (Phase 2): add `proposal_id: u64` parameter and store it in the
    /// spend record for a full audit trail.
    ///
    /// TODO (Phase 3): check per-tx spend cap and daily limit before transferring.
    pub fn spend(env: Env, token: Address, recipient: Address, amount: i128) {
        let config = storage::config(&env);
        config.governance_contract.require_auth();

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &recipient, &amount);

        storage::record_spend(&env, &token, &recipient, amount);

        // TODO (Phase 2): emit a Spent event via env.events().publish().
    }

    /// Return the treasury's current balance of `token`.
    ///
    /// Deposits are implicit — anyone can transfer tokens to this contract address.
    pub fn balance(env: Env, token: Address) -> i128 {
        token::Client::new(&env, &token).balance(&env.current_contract_address())
    }

    // ── Admin functions (Phase 2 / 3) ────────────────────────────────────────
    //
    // TODO (Phase 2): implement set_governance_contract.
    //
    // /// Rotate the governance contract address. Admin only.
    // /// Steps:
    // /// 1. Read config; assert caller == config.admin via require_auth().
    // /// 2. Call storage::update_governance_contract(env, new_governance).
    // /// 3. Emit a GovernanceUpdated event.
    // pub fn set_governance_contract(env: Env, caller: Address, new_governance: Address) {
    //     todo!()
    // }
    //
    // TODO (Phase 3): implement upgrade.
    //
    // /// Upgrade the contract WASM. Admin only.
    // pub fn upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) { todo!() }

    // ── Views ─────────────────────────────────────────────────────────────────

    pub fn get_config(env: Env) -> TreasuryConfig {
        storage::config(&env)
    }

    pub fn spend_count(env: Env) -> u64 {
        storage::spend_count(&env)
    }

    pub fn get_spend_record(env: Env, index: u64) -> Option<SpendRecord> {
        storage::get_spend_record(&env, index)
    }
}

mod test;
