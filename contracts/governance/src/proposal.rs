use soroban_sdk::{Address, Bytes, Env, String};

use crate::storage;
use crate::types::{Proposal, ProposalStatus};
use crate::GovernanceError;

/// Create a new proposal and return its ID.
///
/// Steps to extend (Phase 2 — proposal creation threshold):
/// 1. Call `token_weight::get_weight(env, proposer)` via cross-contract client.
/// 2. Read `config.min_proposal_threshold` from storage.
/// 3. Panic with `GovernanceError::BelowProposalThreshold` if weight < threshold.
/// 4. Store `snapshot_ledger: env.ledger().sequence()` on the proposal so
///    `voting::cast_vote` can pass it to `get_weight` for snapshot-based voting.
pub fn create(
    env: &Env,
    proposer: &Address,
    title: String,
    description: String,
    action_payload: Bytes,
) -> u64 {
    let id = storage::increment_proposal_count(env);
    let period = storage::voting_period_ledgers(env);
    let end_ledger = env.ledger().sequence() + period;

    let proposal = Proposal {
        id,
        proposer: proposer.clone(),
        title,
        description,
        action_payload,
        votes_for: 0,
        votes_against: 0,
        votes_abstain: 0,
        status: ProposalStatus::Active,
        end_ledger,
        // TODO (Phase 2): snapshot_ledger: env.ledger().sequence(),
    };

    storage::save_proposal(env, &proposal);
    id
}

/// Tally votes and set the final status of a proposal.
///
/// Current quorum logic: `votes_for / total_cast >= quorum_bps / 10_000`.
/// This is a majority-of-cast-votes check, NOT majority-of-total-supply.
///
/// Steps to upgrade to supply-based quorum (Phase 2):
/// 1. Add `total_supply_at_snapshot: i128` to `Proposal`.
/// 2. In `create()`, fetch total supply from the token contract and store it.
/// 3. Replace the `total` denominator below with `proposal.total_supply_at_snapshot`.
/// 4. Update tests to mint a known total supply and assert the new threshold.
pub fn finalize(env: &Env, proposal_id: u64) -> ProposalStatus {
    let mut proposal = storage::get_proposal(env, proposal_id);

    if proposal.status != ProposalStatus::Active {
        return proposal.status;
    }
    if env.ledger().sequence() < proposal.end_ledger {
        panic!("{}", GovernanceError::VotingPeriodNotEnded as u32);
    }

    let total = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
    let quorum_bps = storage::quorum_bps(env) as i128;

    let approved = total > 0 && proposal.votes_for * 10_000 / total >= quorum_bps;

    proposal.status = if approved {
        ProposalStatus::Approved
    } else {
        ProposalStatus::Rejected
    };

    storage::save_proposal(env, &proposal);
    proposal.status
}

/// Execute the on-chain action encoded in `action_payload`.
///
/// Currently marks the proposal as Executed without dispatching anything.
///
/// Steps to implement cross-contract dispatch (Phase 2):
/// 1. Define an `ActionPayload` struct in types.rs:
///    `{ contract: Address, function: Symbol, args: Vec<Val> }`
/// 2. Deserialize `proposal.action_payload` using `soroban_sdk::xdr::FromXdr`.
/// 3. Call `env.invoke_contract(&payload.contract, &payload.function, payload.args)`.
/// 4. Emit an `Executed` event with the proposal ID and return value.
///
/// Steps to add a timelock (Phase 3):
/// 1. Add `timelock_end_ledger: u32` to `Proposal` (set in `finalize()`).
/// 2. Panic with `GovernanceError::TimelockNotExpired` if
///    `env.ledger().sequence() < proposal.timelock_end_ledger`.
pub fn execute(env: &Env, proposal_id: u64) {
    let mut proposal = storage::get_proposal(env, proposal_id);

    if proposal.status != ProposalStatus::Approved {
        panic!("{}", GovernanceError::ProposalNotApproved as u32);
    }

    // TODO (Phase 3): enforce timelock.
    // TODO (Phase 2): decode action_payload and dispatch cross-contract call.

    proposal.status = ProposalStatus::Executed;
    storage::save_proposal(env, &proposal);
}
