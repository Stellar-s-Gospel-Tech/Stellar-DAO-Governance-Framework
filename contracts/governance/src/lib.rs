#![no_std]

mod error;
mod proposal;
mod storage;
mod types;
mod voting;

pub use error::GovernanceError;
pub use types::{GovernanceConfig, Proposal, ProposalStatus, Vote, VoteChoice};

use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String};

/// GovernanceContract manages the full lifecycle of DAO proposals:
///   create_proposal → vote → finalize → execute
///
/// Voting power is resolved by the `token-weight` contract, keeping this
/// contract agnostic to whether weight comes from a native token or an LP token.
///
/// # Contributor guide
///
/// Phase 2 tasks in this file:
/// - Wire `create_proposal` to enforce a minimum token balance threshold.
/// - Wire `vote` to call `token-weight::get_weight` instead of using weight=1.
/// - Add `delegate` / `undelegate` entry points.
///
/// Phase 3 tasks:
/// - Add `cancel_proposal` (proposer or admin only).
/// - Enforce timelock between `finalize` and `execute`.
/// - Add `upgrade` entry point for WASM upgrades (admin only).
#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// One-time initializer.
    ///
    /// - `admin`                  — address that can upgrade the contract (Phase 3).
    /// - `weight_contract`        — deployed `token-weight` contract address.
    /// - `quorum_bps`             — approval threshold in basis points (5000 = 50%).
    /// - `voting_period_ledgers`  — how long proposals stay open (~17280 ≈ 1 day).
    pub fn initialize(
        env: Env,
        admin: Address,
        weight_contract: Address,
        quorum_bps: u32,
        voting_period_ledgers: u32,
    ) {
        storage::init(
            &env,
            &admin,
            &weight_contract,
            quorum_bps,
            voting_period_ledgers,
        );
    }

    /// Create a new proposal. Returns the new proposal ID.
    ///
    /// `action_payload` — ABI-encoded cross-contract call to run on approval.
    /// Pass `Bytes::new(&env)` for a signalling-only (no on-chain action) proposal.
    ///
    /// TODO (Phase 2): reject if proposer's token balance < min_proposal_threshold.
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        title: String,
        description: String,
        action_payload: Bytes,
    ) -> u64 {
        proposer.require_auth();
        proposal::create(&env, &proposer, title, description, action_payload)
    }

    /// Cast a vote on an active proposal.
    ///
    /// TODO (Phase 2): weight is currently 1 per voter; replace with
    /// `token-weight::get_weight` cross-contract call.
    pub fn vote(env: Env, voter: Address, proposal_id: u64, choice: VoteChoice) {
        voter.require_auth();
        voting::cast_vote(&env, &voter, proposal_id, choice);
    }

    /// Tally votes and mark the proposal Approved or Rejected.
    /// Can be called by anyone once the voting period has ended.
    pub fn finalize(env: Env, proposal_id: u64) -> ProposalStatus {
        proposal::finalize(&env, proposal_id)
    }

    /// Execute the on-chain action of an approved proposal.
    ///
    /// TODO (Phase 2): decode `action_payload` and dispatch cross-contract call.
    /// TODO (Phase 3): enforce timelock before allowing execution.
    pub fn execute(env: Env, proposal_id: u64) {
        proposal::execute(&env, proposal_id);
    }

    // ── Delegation (Phase 2) ─────────────────────────────────────────────────
    //
    // TODO: implement delegate / undelegate.
    //
    // /// Delegate voting power to another address.
    // /// Steps:
    // /// 1. `delegator.require_auth()`
    // /// 2. Ensure delegator has not already delegated.
    // /// 3. Call `storage::set_delegation(env, delegator, delegate)`.
    // pub fn delegate(env: Env, delegator: Address, delegate: Address) { todo!() }
    //
    // /// Revoke an existing delegation.
    // pub fn undelegate(env: Env, delegator: Address) { todo!() }

    // ── Admin (Phase 3) ──────────────────────────────────────────────────────
    //
    // TODO: implement upgrade and cancel_proposal.
    //
    // /// Upgrade the contract WASM. Admin only.
    // /// Steps:
    // /// 1. `admin.require_auth()` — read admin from storage::config().
    // /// 2. Call `env.deployer().update_current_contract_wasm(new_wasm_hash)`.
    // pub fn upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) { todo!() }
    //
    // /// Cancel an active proposal. Proposer or admin only.
    // pub fn cancel_proposal(env: Env, caller: Address, proposal_id: u64) { todo!() }

    // ── Views ────────────────────────────────────────────────────────────────

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        storage::get_proposal(&env, proposal_id)
    }

    pub fn get_vote(env: Env, proposal_id: u64, voter: Address) -> Option<Vote> {
        storage::get_vote(&env, proposal_id, &voter)
    }

    pub fn proposal_count(env: Env) -> u64 {
        storage::proposal_count(&env)
    }

    pub fn get_config(env: Env) -> GovernanceConfig {
        storage::config(&env)
    }
}

mod test;
