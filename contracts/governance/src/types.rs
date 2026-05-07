use soroban_sdk::{contracttype, Address, Bytes, String};

/// The three choices a voter can make on a proposal.
///
/// Contributor note: if you add a new variant (e.g. `Veto`), also update
/// `voting::cast_vote` to handle it and add the matching tally field to `Proposal`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

/// The lifecycle state of a proposal.
///
/// State machine:
///   Active ──(voting period ends)──► finalize() ──► Approved | Rejected
///   Approved ──(timelock passes)──► execute() ──► Executed
///
/// Contributor note: add a `Cancelled` variant (Phase 3) so the proposer or
/// admin can abort a proposal before voting ends.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
    // TODO (Phase 3): Cancelled,
}

/// A DAO proposal stored on-chain.
///
/// Contributor note — fields to add in later phases:
/// - `snapshot_ledger: u32`  — ledger at which token balances are snapshotted
///                             (prevents flash-loan vote manipulation).
/// - `timelock_end_ledger: u32` — earliest ledger at which `execute()` is allowed.
/// - `creation_threshold: i128` — minimum token balance required to have created this.
/// - `total_supply_at_snapshot: i128` — for quorum-against-supply checks.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    /// Auto-incrementing ID assigned at creation.
    pub id: u64,
    /// Address that submitted the proposal.
    pub proposer: Address,
    /// Short human-readable title.
    pub title: String,
    /// Full description / rationale.
    pub description: String,
    /// ABI-encoded cross-contract call to execute if approved.
    /// Empty bytes = signalling-only proposal (no on-chain action).
    pub action_payload: Bytes,
    /// Running tally of weighted votes.
    pub votes_for: i128,
    pub votes_against: i128,
    pub votes_abstain: i128,
    /// Current lifecycle state.
    pub status: ProposalStatus,
    /// Ledger sequence number at which voting closes.
    pub end_ledger: u32,
    // TODO (Phase 2): pub snapshot_ledger: u32,
    // TODO (Phase 3): pub timelock_end_ledger: u32,
}

/// A single vote cast by one address on one proposal.
///
/// Stored keyed by (proposal_id, voter) so duplicate votes are impossible.
///
/// Contributor note: add `delegated_from: Option<Address>` when delegation
/// is implemented (Phase 2) so the audit trail is preserved.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Vote {
    pub voter: Address,
    pub choice: VoteChoice,
    /// Token-weighted voting power at the time of the vote.
    pub weight: i128,
    // TODO (Phase 2): pub delegated_from: Option<Address>,
}

/// Parameters passed to `GovernanceContract::initialize`.
///
/// Kept as a struct so the initializer signature stays stable as new
/// config fields are added.
///
/// Contributor note: add `min_proposal_threshold: i128` (Phase 2) and
/// `timelock_ledgers: u32` (Phase 3) here.
#[contracttype]
#[derive(Clone, Debug)]
pub struct GovernanceConfig {
    pub admin: Address,
    pub weight_contract: Address,
    /// Minimum share of votes required for approval, in basis points (1 bps = 0.01%).
    /// e.g. 5000 = 50% of cast votes must be For.
    pub quorum_bps: u32,
    /// How many ledgers a proposal stays open for voting.
    /// ~1 ledger ≈ 5 seconds on Stellar mainnet, so 17280 ≈ 1 day.
    pub voting_period_ledgers: u32,
    // TODO (Phase 2): pub min_proposal_threshold: i128,
    // TODO (Phase 3): pub timelock_ledgers: u32,
}
