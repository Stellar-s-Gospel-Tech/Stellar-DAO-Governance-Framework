use soroban_sdk::contracttype;

/// Determines how voting power is calculated.
///
/// Contributor note: adding a new strategy requires:
/// 1. Add the variant here.
/// 2. Update `TokenWeightContract::get_weight` in lib.rs to handle it.
/// 3. Add a test in test.rs that mints the relevant token type and asserts weight.
///
/// Planned strategies:
/// - `NativeToken` — weight = raw balance of a single SAC token. ✅ implemented.
/// - `LpToken`     — weight = LP token balance (e.g. Soroswap / Blend pool). ✅ scaffold.
/// - `Quadratic`   — weight = sqrt(balance), reduces whale dominance. TODO (Phase 3).
/// - `Capped`      — weight = min(balance, cap), hard ceiling per voter. TODO (Phase 3).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeightStrategy {
    /// weight = token.balance(voter)
    NativeToken,
    /// weight = lp_token.balance(voter)
    /// The LP token address is stored as the `token` in config.
    LpToken,
    // TODO (Phase 3): Quadratic,
    // TODO (Phase 3): Capped { max_weight: i128 },
}
