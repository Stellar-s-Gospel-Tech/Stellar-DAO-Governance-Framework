#![cfg(test)]

//! Token-weight contract tests.
//!
//! Contributor guide — how to add tests:
//! - Use `env.register_stellar_asset_contract_v2()` to create a mock SAC token.
//! - Use `StellarAssetClient::mint()` to give voters a balance.
//! - Assert `get_weight()` returns the minted balance.
//! - For LP token tests, deploy a mock LP token contract and use `LpToken` strategy.

use soroban_sdk::{testutils::Address as _, token::StellarAssetClient, Address, Env};

use crate::{TokenWeightContract, TokenWeightContractClient, WeightStrategy};

fn setup_native(env: &Env) -> (TokenWeightContractClient<'_>, Address) {
    let token_admin = Address::generate(env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());

    let contract_id = env.register(TokenWeightContract, ());
    let client = TokenWeightContractClient::new(env, &contract_id);

    let admin = Address::generate(env);
    client.initialize(&admin, &WeightStrategy::NativeToken, &token_id.address());

    (client, token_id.address())
}

// ── NativeToken strategy ──────────────────────────────────────────────────────

#[test]
fn test_get_weight_native_token() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, token) = setup_native(&env);
    let voter = Address::generate(&env);

    StellarAssetClient::new(&env, &token).mint(&voter, &1_000);

    assert_eq!(client.get_weight(&voter), 1_000);
}

#[test]
fn test_zero_weight_for_empty_wallet() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _token) = setup_native(&env);
    let voter = Address::generate(&env);

    assert_eq!(client.get_weight(&voter), 0);
}

#[test]
fn test_get_strategy_returns_correct_variant() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _token) = setup_native(&env);
    assert_eq!(client.get_strategy(), WeightStrategy::NativeToken);
}

// ── LpToken strategy ──────────────────────────────────────────────────────────

#[test]
fn test_get_weight_lp_token() {
    let env = Env::default();
    env.mock_all_auths();

    // LP token is just another SAC in tests — the strategy tag is what differs.
    let lp_admin = Address::generate(&env);
    let lp_token = env.register_stellar_asset_contract_v2(lp_admin.clone());
    StellarAssetClient::new(&env, &lp_token.address()).mint(&Address::generate(&env), &0);

    let contract_id = env.register(TokenWeightContract, ());
    let client = TokenWeightContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin, &WeightStrategy::LpToken, &lp_token.address());

    let voter = Address::generate(&env);
    StellarAssetClient::new(&env, &lp_token.address()).mint(&voter, &2_500);

    assert_eq!(client.get_weight(&voter), 2_500);
    assert_eq!(client.get_strategy(), WeightStrategy::LpToken);
}

// ── TODO tests (Phase 2) ──────────────────────────────────────────────────────
//
// #[test]
// fn test_snapshot_weight_is_balance_at_snapshot_ledger() {
//     // Steps:
//     // 1. Mint 1000 tokens to voter at ledger 10.
//     // 2. Record snapshot at ledger 10.
//     // 3. Advance ledger; voter transfers all tokens away.
//     // 4. Call get_weight_at(voter, 10); assert result == 1000 (not 0).
// }
//
// #[test]
// fn test_snapshot_expired_error() {
//     // Steps:
//     // 1. Record a snapshot at ledger N.
//     // 2. Advance ledger far enough that the snapshot TTL expires.
//     // 3. Call get_weight_at(voter, N); assert SnapshotExpired error.
// }
//
// TODO tests (Phase 3):
//
// #[test]
// fn test_quadratic_weight() {
//     // Steps:
//     // 1. Initialize with WeightStrategy::Quadratic.
//     // 2. Mint 100 tokens to voter.
//     // 3. Assert get_weight(voter) == 10 (isqrt(100)).
// }
//
// #[test]
// fn test_capped_weight() {
//     // Steps:
//     // 1. Initialize with WeightStrategy::Capped { max_weight: 500 }.
//     // 2. Mint 1000 tokens to voter.
//     // 3. Assert get_weight(voter) == 500.
// }
