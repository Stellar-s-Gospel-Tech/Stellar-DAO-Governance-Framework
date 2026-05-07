use soroban_sdk::{Address, Env};

use crate::storage;
use crate::types::{ProposalStatus, Vote, VoteChoice};
use crate::GovernanceError;

/// Cast a vote on an active proposal.
///
/// Current weight: placeholder value of `1` per voter (equal-weight voting).
///
/// Steps to wire up real token-weighted voting (Phase 2):
/// 1. Import the token-weight contract client:
///    `use token_weight::TokenWeightContractClient;`
///    (add `token-weight = { path = "../token-weight" }` to governance Cargo.toml)
/// 2. Read the weight contract address: `storage::weight_contract(env)`.
/// 3. Call `TokenWeightContractClient::new(env, &weight_addr).get_weight(&voter)`.
///    Pass `proposal.snapshot_ledger` once snapshot support is added.
/// 4. Replace the `let weight: i128 = 1;` line with the returned value.
/// 5. Add a test that mints different balances to different voters and asserts
///    that the tally reflects those balances.
///
/// Steps to add delegation support (Phase 2):
/// 1. After fetching `weight`, call `storage::get_delegated_power(env, voter)`
///    and add it to `weight`.
/// 2. Ensure the delegator cannot also vote directly (check delegation status).
pub fn cast_vote(env: &Env, voter: &Address, proposal_id: u64, choice: VoteChoice) {
    let mut proposal = storage::get_proposal(env, proposal_id);

    if proposal.status != ProposalStatus::Active {
        panic!("{}", GovernanceError::ProposalNotActive as u32);
    }
    if env.ledger().sequence() >= proposal.end_ledger {
        panic!("{}", GovernanceError::VotingPeriodEnded as u32);
    }
    if storage::get_vote(env, proposal_id, voter).is_some() {
        panic!("{}", GovernanceError::AlreadyVoted as u32);
    }

    // TODO (Phase 2): replace with real token-weight cross-contract call.
    let weight: i128 = 1;

    match choice {
        VoteChoice::For => proposal.votes_for += weight,
        VoteChoice::Against => proposal.votes_against += weight,
        VoteChoice::Abstain => proposal.votes_abstain += weight,
    }

    let vote = Vote {
        voter: voter.clone(),
        choice,
        weight,
    };

    storage::save_vote(env, proposal_id, &vote);
    storage::save_proposal(env, &proposal);

    // TODO (Phase 2): emit a `Voted` event via env.events().publish().
}
