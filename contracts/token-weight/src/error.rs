use soroban_sdk::contracterror;

/// All errors the token-weight contract can return.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TokenWeightError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    /// The requested snapshot ledger is too old and has been archived.
    /// Raised when snapshot-based weight is implemented (Phase 2).
    SnapshotExpired = 4,
    // TODO (Phase 3): UnsupportedStrategy = 5,
}
