use soroban_sdk::{contracttype, Address, Env};

use crate::types::{GovernanceConfig, Proposal, Vote};
use crate::GovernanceError;

// ── Storage keys ──────────────────────────────────────────────────────────────
//
// Contributor note: all persistent data lives under these keys.
// - Instance storage  → config that rarely changes (admin, weight contract, quorum).
// - Persistent storage → per-proposal and per-vote data that must survive
//   state archival. Remember to call `env.storage().persistent().extend_ttl()`
//   when reading long-lived entries (Phase 3).

#[contracttype]
pub enum DataKey {
    /// Singleton governance config (instance storage).
    Config,
    /// Running count of proposals; also used as the next proposal ID.
    ProposalCount,
    /// Proposal data keyed by ID (persistent storage).
    Proposal(u64),
    /// Vote data keyed by (proposal_id, voter) (persistent storage).
    Vote(u64, Address),
    // TODO (Phase 2): DelegatedTo(Address)  — maps delegator → delegate.
    // TODO (Phase 2): DelegatedPower(Address) — accumulated delegated weight.
}

// ── Initializer ───────────────────────────────────────────────────────────────

/// Persist the initial governance config. Panics if already initialized.
///
/// Steps for contributor extending this:
/// 1. Add new fields to `GovernanceConfig` in types.rs.
/// 2. Accept them as parameters here and store them in the config struct.
/// 3. Add a corresponding getter below.
pub fn init(
    env: &Env,
    admin: &Address,
    weight_contract: &Address,
    quorum_bps: u32,
    voting_period_ledgers: u32,
) {
    if env.storage().instance().has(&DataKey::Config) {
        panic!("{}", GovernanceError::AlreadyInitialized as u32);
    }
    let config = GovernanceConfig {
        admin: admin.clone(),
        weight_contract: weight_contract.clone(),
        quorum_bps,
        voting_period_ledgers,
    };
    env.storage().instance().set(&DataKey::Config, &config);
    env.storage().instance().set(&DataKey::ProposalCount, &0u64);
}

// ── Config getters ────────────────────────────────────────────────────────────

pub fn config(env: &Env) -> GovernanceConfig {
    env.storage().instance().get(&DataKey::Config).unwrap()
}

#[allow(dead_code)]
pub fn admin(env: &Env) -> Address {
    config(env).admin
}

#[allow(dead_code)]
pub fn weight_contract(env: &Env) -> Address {
    config(env).weight_contract
}

pub fn quorum_bps(env: &Env) -> u32 {
    config(env).quorum_bps
}

pub fn voting_period_ledgers(env: &Env) -> u32 {
    config(env).voting_period_ledgers
}

// ── Proposal counter ──────────────────────────────────────────────────────────

pub fn proposal_count(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::ProposalCount)
        .unwrap_or(0)
}

/// Increment and return the new proposal ID (1-indexed).
pub fn increment_proposal_count(env: &Env) -> u64 {
    let next = proposal_count(env) + 1;
    env.storage().instance().set(&DataKey::ProposalCount, &next);
    next
}

// ── Proposal CRUD ─────────────────────────────────────────────────────────────
//
// Contributor note: proposals use `persistent` storage because they must
// survive ledger state archival. When reading a proposal, consider calling
// `extend_ttl` to keep it live (Phase 3 hardening task).

pub fn save_proposal(env: &Env, proposal: &Proposal) {
    env.storage()
        .persistent()
        .set(&DataKey::Proposal(proposal.id), proposal);
}

pub fn get_proposal(env: &Env, id: u64) -> Proposal {
    env.storage()
        .persistent()
        .get(&DataKey::Proposal(id))
        .unwrap_or_else(|| panic!("{}", GovernanceError::ProposalNotFound as u32))
}

// ── Vote CRUD ─────────────────────────────────────────────────────────────────
//
// Contributor note: votes are also persistent. The key includes the voter
// address so existence checks are O(1) — no iteration needed.

pub fn save_vote(env: &Env, proposal_id: u64, vote: &Vote) {
    env.storage()
        .persistent()
        .set(&DataKey::Vote(proposal_id, vote.voter.clone()), vote);
}

pub fn get_vote(env: &Env, proposal_id: u64, voter: &Address) -> Option<Vote> {
    env.storage()
        .persistent()
        .get(&DataKey::Vote(proposal_id, voter.clone()))
}

// ── Delegation (Phase 2) ──────────────────────────────────────────────────────
//
// TODO: implement the following functions when delegation is added.
//
// /// Record that `delegator` has delegated their voting power to `delegate`.
// /// Steps:
// /// 1. Ensure `delegator` has not already delegated (or revoke first).
// /// 2. Store DataKey::DelegatedTo(delegator) → delegate.
// /// 3. Add delegator's current weight to DataKey::DelegatedPower(delegate).
// pub fn set_delegation(env: &Env, delegator: &Address, delegate: &Address) { todo!() }
//
// /// Remove an existing delegation.
// pub fn revoke_delegation(env: &Env, delegator: &Address) { todo!() }
//
// /// Return the address this delegator has delegated to, if any.
// pub fn get_delegate(env: &Env, delegator: &Address) -> Option<Address> { todo!() }
//
// /// Return the total delegated power accumulated by `delegate`.
// pub fn get_delegated_power(env: &Env, delegate: &Address) -> i128 { todo!() }
