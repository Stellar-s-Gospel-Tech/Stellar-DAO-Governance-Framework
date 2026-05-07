#![cfg(test)]

//! Treasury contract tests.
//!
//! Contributor guide — how to add tests:
//! - Call `setup()` for a fresh env with a deployed treasury and a funded SAC token.
//! - Use `env.mock_all_auths()` so `require_auth()` passes in tests.
//! - Use `TokenClient::balance()` to assert balances after spend calls.

use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

use crate::{TreasuryContract, TreasuryContractClient};

/// Deploy treasury, create a SAC token, and mint 10_000 units to the treasury.
///
/// Returns `(client, governance_address, token_address, admin_address)`.
fn setup(env: &Env) -> (TreasuryContractClient<'_>, Address, Address, Address) {
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(env, &contract_id);

    let admin = Address::generate(env);
    let governance = Address::generate(env);
    client.initialize(&admin, &governance);

    let token_admin = Address::generate(env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    StellarAssetClient::new(env, &token_id.address()).mint(&contract_id, &10_000);

    (client, governance, token_id.address(), admin)
}

// ── Core spend ────────────────────────────────────────────────────────────────

#[test]
fn test_spend_transfers_tokens() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _governance, token, _admin) = setup(&env);
    let recipient = Address::generate(&env);

    client.spend(&token, &recipient, &500);

    assert_eq!(TokenClient::new(&env, &token).balance(&recipient), 500);
}

#[test]
fn test_spend_reduces_treasury_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _governance, token, _admin) = setup(&env);
    let recipient = Address::generate(&env);

    let before = client.balance(&token);
    client.spend(&token, &recipient, &1_000);
    let after = client.balance(&token);

    assert_eq!(before - after, 1_000);
}

#[test]
fn test_spend_records_audit_log() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _governance, token, _admin) = setup(&env);
    let recipient = Address::generate(&env);

    assert_eq!(client.spend_count(), 0);
    client.spend(&token, &recipient, &200);
    assert_eq!(client.spend_count(), 1);

    let record = client.get_spend_record(&0).unwrap();
    assert_eq!(record.amount, 200);
    assert_eq!(record.recipient, recipient);
}

// ── TODO tests (Phase 3) ──────────────────────────────────────────────────────
//
// #[test]
// fn test_spend_cap_exceeded_rejected() {
//     // Steps:
//     // 1. Initialize with spend_cap_per_tx = 100.
//     // 2. Try to spend 101; assert SpendCapExceeded error.
// }
//
// #[test]
// fn test_daily_limit_exceeded_rejected() {
//     // Steps:
//     // 1. Initialize with daily_spend_limit = 500.
//     // 2. Spend 300, then spend 300 again in the same ledger-day.
//     // 3. Assert the second call fails with DailyLimitExceeded.
// }
//
// #[test]
// fn test_unauthorized_spend_rejected() {
//     // Steps:
//     // 1. Call spend() without mocking auth for the governance contract.
//     // 2. Assert the call fails with an auth error.
// }
//
// #[test]
// fn test_set_governance_contract() {
//     // Steps:
//     // 1. Deploy treasury; record original governance address.
//     // 2. Call set_governance_contract() with a new address (admin auth).
//     // 3. Assert get_config().governance_contract == new address.
//     // 4. Assert old governance address can no longer call spend().
// }
