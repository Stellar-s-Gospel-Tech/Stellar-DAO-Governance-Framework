## Summary

Add a `snapshot_ledger` field to `Proposal` so voting weight is read from the ledger at which the proposal was created, not the current ledger. This prevents flash-loan attacks where an attacker borrows tokens, votes, then returns them in the same transaction.

## Context

- `contracts/governance/src/types.rs` — `Proposal` struct has a `// TODO (Phase 2): pub snapshot_ledger: u32` comment.
- `contracts/token-weight/src/lib.rs` — has a commented-out `get_weight_at(voter, snapshot_ledger)` stub.
- `contracts/token-weight/src/storage.rs` — has a commented-out `record_snapshot` / `get_snapshot` section.

## What to implement

### 1. token-weight contract

Uncomment and implement `get_weight_at(voter, snapshot_ledger)`:

```rust
pub fn get_weight_at(env: Env, voter: Address, snapshot_ledger: u32) -> i128 {
    // Read the cached snapshot from persistent storage.
    // Panic with TokenWeightError::SnapshotExpired if not found.
    storage::get_snapshot(&env, &voter, snapshot_ledger)
}
```

Implement `storage::record_snapshot` and `storage::get_snapshot` using `DataKey::SnapshotBalance(Address, u32)`.

### 2. governance contract

- Add `snapshot_ledger: u32` to `Proposal` (set to `env.ledger().sequence()` in `proposal::create`).
- In `voting::cast_vote`, call `get_weight_at(voter, proposal.snapshot_ledger)` instead of `get_weight(voter)`.

## Acceptance criteria

- [ ] `Proposal` has a `snapshot_ledger` field.
- [ ] `token-weight` exposes `get_weight_at(voter, snapshot_ledger)`.
- [ ] `cast_vote` uses `get_weight_at` with the proposal's snapshot ledger.
- [ ] Test: mint tokens, advance ledger, transfer tokens away, vote — assert weight reflects the snapshot balance, not the current balance.
- [ ] `cargo test --all` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.

## Complexity

**High**
