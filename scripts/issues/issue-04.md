## Summary

The current quorum check compares `votes_for` against total cast votes. This means a proposal with 1 For and 0 Against passes a 50% quorum. This issue upgrades the check to compare against the token's total supply, so quorum reflects actual participation.

## Context

- `contracts/governance/src/proposal.rs` — `finalize()` has a `// TODO: fetch total supply from token-weight contract` comment.
- `contracts/governance/src/types.rs` — `Proposal` has a `// TODO (Phase 2): pub total_supply_at_snapshot: i128` comment.

## What to implement

### 1. Add `total_supply_at_snapshot` to `Proposal`

```rust
pub total_supply_at_snapshot: i128,
```

Set it in `proposal::create()` by calling `token::Client::new(env, &token_addr).total_supply()` (read the token address from `storage::weight_contract`).

### 2. Update `finalize()` quorum check

Replace:
```rust
let total = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
let approved = total > 0 && proposal.votes_for * 10_000 / total >= quorum_bps;
```

With:
```rust
let approved = proposal.total_supply_at_snapshot > 0
    && proposal.votes_for * 10_000 / proposal.total_supply_at_snapshot >= quorum_bps;
```

## Acceptance criteria

- [ ] `Proposal` stores `total_supply_at_snapshot`.
- [ ] `finalize()` uses total supply as the quorum denominator.
- [ ] Test: mint 1000 tokens total, have 1 voter with 400 tokens vote For, set quorum to 50% — assert proposal is Rejected (400/1000 = 40% < 50%).
- [ ] Test: same setup but voter holds 600 tokens — assert Approved.
- [ ] `cargo test --all` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.

## Complexity

**Medium**
