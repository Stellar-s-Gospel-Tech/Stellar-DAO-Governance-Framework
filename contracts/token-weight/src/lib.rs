#![no_std]

mod error;
mod storage;
mod types;

pub use error::TokenWeightError;
pub use types::WeightStrategy;

use soroban_sdk::{contract, contractimpl, token, Address, Env};

/// TokenWeightContract resolves voting power for a given address.
///
/// It is called by the governance contract at vote time to determine how
/// much weight a voter's choice carries.
///
/// # Contributor guide
///
/// Phase 2 tasks:
/// - Add `get_weight_at(voter, snapshot_ledger)` to prevent flash-loan attacks.
///   Steps:
///   1. Add `snapshot_ledger: u32` parameter.
///   2. Call `storage::get_snapshot(env, voter, snapshot_ledger)` instead of
///      reading the live balance.
///   3. Update governance `voting::cast_vote` to pass `proposal.snapshot_ledger`.
///
/// Phase 3 tasks:
/// - Implement `Quadratic` strategy: return `isqrt(balance)`.
///   Use integer square root — no floating point in Soroban.
/// - Implement `Capped` strategy: return `min(balance, cap)`.
/// - Add `update_token(new_token)` admin function to rotate the token address.
#[contract]
pub struct TokenWeightContract;

#[contractimpl]
impl TokenWeightContract {
    /// One-time initializer.
    ///
    /// - `admin`    — can update the token address or strategy (Phase 3).
    /// - `strategy` — `NativeToken` or `LpToken`.
    /// - `token`    — SAC token (or LP token) address whose balance = weight.
    pub fn initialize(env: Env, admin: Address, strategy: WeightStrategy, token: Address) {
        storage::init(&env, &admin, strategy, &token);
    }

    /// Return the current voting weight of `voter`.
    ///
    /// Currently returns the live token balance. This is vulnerable to
    /// flash-loan manipulation — see Phase 2 task above.
    ///
    /// Strategy dispatch:
    /// - `NativeToken` → `token.balance(voter)`
    /// - `LpToken`     → same call; the LP token address is stored as `token`.
    ///
    /// TODO (Phase 2): replace with `get_weight_at(voter, snapshot_ledger)`.
    /// TODO (Phase 3): dispatch to quadratic / capped calculation based on strategy.
    pub fn get_weight(env: Env, voter: Address) -> i128 {
        let token_addr = storage::token(&env);
        let _strategy = storage::strategy(&env); // used for dispatch in Phase 3
        token::Client::new(&env, &token_addr).balance(&voter)
    }

    // ── Phase 2 stub ─────────────────────────────────────────────────────────
    //
    // TODO: implement snapshot-based weight.
    //
    // /// Return the voting weight of `voter` at `snapshot_ledger`.
    // /// Steps:
    // /// 1. Call storage::get_snapshot(env, voter, snapshot_ledger).
    // /// 2. If not cached, try to read archival state (advanced — see Soroban docs
    // ///    on state archival: https://developers.stellar.org/docs/build/guides/archival).
    // /// 3. Panic with TokenWeightError::SnapshotExpired if unavailable.
    // pub fn get_weight_at(env: Env, voter: Address, snapshot_ledger: u32) -> i128 {
    //     todo!()
    // }

    // ── Admin (Phase 3) ──────────────────────────────────────────────────────
    //
    // TODO: implement token rotation and WASM upgrade.
    //
    // /// Update the token address. Admin only.
    // /// Steps:
    // /// 1. Read admin from storage; call admin.require_auth().
    // /// 2. Overwrite DataKey::Token with new_token.
    // pub fn update_token(env: Env, caller: Address, new_token: Address) { todo!() }
    //
    // /// Upgrade the contract WASM. Admin only.
    // pub fn upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) { todo!() }

    // ── Views ─────────────────────────────────────────────────────────────────

    pub fn get_strategy(env: Env) -> WeightStrategy {
        storage::strategy(&env)
    }

    pub fn get_token(env: Env) -> Address {
        storage::token(&env)
    }
}

mod test;
