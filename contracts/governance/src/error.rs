use soroban_sdk::contracterror;

/// All errors the governance contract can return.
///
/// Contributor note: when adding a new error, assign the next sequential
/// integer and document when it is raised. Keep this list in sync with
/// the TypeScript SDK error map (sdk/src/errors.ts — Phase 4).
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum GovernanceError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    ProposalNotFound = 3,
    /// Raised when trying to vote on a proposal that is not in `Active` state.
    ProposalNotActive = 4,
    /// Raised when trying to vote after `end_ledger` has passed.
    VotingPeriodEnded = 5,
    /// Raised when trying to finalize before `end_ledger` has passed.
    VotingPeriodNotEnded = 6,
    /// Raised when the same address tries to vote twice on the same proposal.
    AlreadyVoted = 7,
    /// Raised when trying to execute a proposal that was not approved.
    ProposalNotApproved = 8,
    /// Raised when trying to execute a proposal that was already executed.
    ProposalAlreadyExecuted = 9,
    Unauthorized = 10,
    // TODO (Phase 2): BelowProposalThreshold = 11,
    // TODO (Phase 3): TimelockNotExpired = 12,
    // TODO (Phase 3): ProposalCancelled = 13,
}
