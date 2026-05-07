#![cfg(test)]

//! Governance contract tests.
//!
//! Contributor guide — how to add tests:
//! - Each test should call `setup()` to get a fresh environment and client.
//! - Use `env.mock_all_auths()` so `require_auth()` calls don't fail in tests.
//! - Advance the ledger with `env.ledger().with_mut(|l| l.sequence_number += N)`
//!   to simulate time passing.
//! - Phase 2: replace the mock `weight_contract` address with a real deployed
//!   `TokenWeightContract` and mint tokens to voters to test weighted voting.

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Bytes, Env, String,
};

use crate::{GovernanceContract, GovernanceContractClient, ProposalStatus, VoteChoice};

/// Deploy the governance contract and initialize it with sensible defaults.
///
/// Returns `(client, admin, weight_contract_address)`.
///
/// Contributor note: when token-weight integration is added (Phase 2), change
/// `weight_contract` here to a real deployed `TokenWeightContract` instance.
fn setup(env: &Env) -> (GovernanceContractClient<'_>, Address, Address) {
    let contract_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(env, &contract_id);

    let admin = Address::generate(env);
    let weight_contract = Address::generate(env); // placeholder until Phase 2

    // quorum_bps=5000 (50%), voting_period_ledgers=100
    client.initialize(&admin, &weight_contract, &5000, &100);
    (client, admin, weight_contract)
}

// ── Basic lifecycle ───────────────────────────────────────────────────────────

#[test]
fn test_create_and_vote() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, _) = setup(&env);
    let proposer = Address::generate(&env);

    let id = client.create_proposal(
        &proposer,
        &String::from_str(&env, "Upgrade treasury limit"),
        &String::from_str(&env, "Raise the per-tx cap from 1000 to 5000 XLM"),
        &Bytes::new(&env),
    );
    assert_eq!(id, 1);

    let voter = Address::generate(&env);
    client.vote(&voter, &id, &VoteChoice::For);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.votes_for, 1);
    assert_eq!(proposal.status, ProposalStatus::Active);
}

#[test]
fn test_finalize_approved() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, _) = setup(&env);
    let proposer = Address::generate(&env);

    let id = client.create_proposal(
        &proposer,
        &String::from_str(&env, "Test proposal"),
        &String::from_str(&env, "Description"),
        &Bytes::new(&env),
    );

    // 6 For, 4 Against → 60% For ≥ 50% quorum → Approved.
    for _ in 0..6 {
        client.vote(&Address::generate(&env), &id, &VoteChoice::For);
    }
    for _ in 0..4 {
        client.vote(&Address::generate(&env), &id, &VoteChoice::Against);
    }

    env.ledger().with_mut(|l| l.sequence_number += 101);

    let status = client.finalize(&id);
    assert_eq!(status, ProposalStatus::Approved);
}

#[test]
fn test_finalize_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, _) = setup(&env);
    let proposer = Address::generate(&env);

    let id = client.create_proposal(
        &proposer,
        &String::from_str(&env, "Rejected proposal"),
        &String::from_str(&env, "Description"),
        &Bytes::new(&env),
    );

    // 4 For, 6 Against → 40% For < 50% quorum → Rejected.
    for _ in 0..4 {
        client.vote(&Address::generate(&env), &id, &VoteChoice::For);
    }
    for _ in 0..6 {
        client.vote(&Address::generate(&env), &id, &VoteChoice::Against);
    }

    env.ledger().with_mut(|l| l.sequence_number += 101);

    let status = client.finalize(&id);
    assert_eq!(status, ProposalStatus::Rejected);
}

#[test]
fn test_proposal_count_increments() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, _) = setup(&env);
    let proposer = Address::generate(&env);

    for i in 1u64..=3 {
        let id = client.create_proposal(
            &proposer,
            &String::from_str(&env, "Proposal"),
            &String::from_str(&env, "Desc"),
            &Bytes::new(&env),
        );
        assert_eq!(id, i);
    }
    assert_eq!(client.proposal_count(), 3);
}

// ── TODO tests (Phase 2) ──────────────────────────────────────────────────────
//
// Add the following tests when the corresponding features are implemented:
//
// #[test]
// fn test_vote_weight_reflects_token_balance() {
//     // Steps:
//     // 1. Deploy a real TokenWeightContract and a SAC token.
//     // 2. Mint 1000 tokens to voter_a and 500 to voter_b.
//     // 3. Both vote For; assert votes_for == 1500.
// }
//
// #[test]
// fn test_below_proposal_threshold_rejected() {
//     // Steps:
//     // 1. Set min_proposal_threshold = 1000 in initialize().
//     // 2. Try to create a proposal with a proposer holding 0 tokens.
//     // 3. Assert the call panics with BelowProposalThreshold.
// }
//
// #[test]
// fn test_duplicate_vote_rejected() {
//     // Steps:
//     // 1. Create a proposal and vote once.
//     // 2. Try to vote again with the same address.
//     // 3. Assert the call panics with AlreadyVoted.
// }
//
// #[test]
// fn test_vote_after_period_rejected() {
//     // Steps:
//     // 1. Create a proposal.
//     // 2. Advance ledger past end_ledger.
//     // 3. Try to vote; assert VotingPeriodEnded.
// }
//
// #[test]
// fn test_execute_approved_proposal() {
//     // Steps:
//     // 1. Create, vote, finalize to Approved.
//     // 2. Call execute(); assert status == Executed.
//     // 3. (Phase 2+) Assert the cross-contract action was dispatched.
// }
//
// #[test]
// fn test_delegation_transfers_weight() {
//     // Steps:
//     // 1. delegator holds 500 tokens; delegate holds 200.
//     // 2. delegator calls delegate(delegate_addr).
//     // 3. delegate votes For; assert votes_for == 700.
// }
