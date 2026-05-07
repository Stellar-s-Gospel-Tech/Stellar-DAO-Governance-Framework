use soroban_sdk::contracterror;

/// All errors the treasury contract can return.
///
/// Contributor note: keep error codes stable — changing them is a breaking
/// change for any client that pattern-matches on the numeric value.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TreasuryError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    /// Caller is not the governance contract.
    Unauthorized = 3,
    /// Token balance is lower than the requested spend amount.
    InsufficientBalance = 4,
    /// Requested amount exceeds the per-tx spend cap.
    SpendCapExceeded = 5,
    // TODO (Phase 3): DailyLimitExceeded = 6,
}
