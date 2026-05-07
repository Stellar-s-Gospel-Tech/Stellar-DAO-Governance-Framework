## Summary

Add vote delegation so token holders can assign their voting power to another address. This is important for passive holders who want to participate in governance without actively voting on every proposal.

## Context

- `contracts/governance/src/lib.rs` — has commented-out `delegate` / `undelegate` stubs.
- `contracts/governance/src/storage.rs` — has a commented-out `Delegation` section with `DataKey::DelegatedTo` and `DataKey::DelegatedPower`.
- `contracts/governance/src/voting.rs` — has a `// TODO (Phase 2): add delegated power` comment.

## What to implement

### 1. Storage keys

Uncomment in `storage.rs`:
```rust
DelegatedTo(Address),
DelegatedPower(Address),
```

Implement:
- `set_delegation(env, delegator, delegate)` — stores the mapping and adds delegator's weight to delegate's accumulated power.
- `revoke_delegation(env, delegator)` — removes the mapping and subtracts the weight.
- `get_delegate(env, delegator) -> Option<Address>`
- `get_delegated_power(env, delegate) -> i128`

### 2. Entry points in `lib.rs`

```rust
pub fn delegate(env: Env, delegator: Address, delegate: Address) {
    delegator.require_auth();
    storage::set_delegation(&env, &delegator, &delegate);
}

pub fn undelegate(env: Env, delegator: Address) {
    delegator.require_auth();
    storage::revoke_delegation(&env, &delegator);
}
```

### 3. Apply delegated power in `voting::cast_vote`

```rust
let own_weight = weight_client.get_weight(voter);
let delegated = storage::get_delegated_power(env, voter);
let weight = own_weight + delegated;
```

Also ensure a delegator cannot vote directly while their power is delegated.

## Acceptance criteria

- [ ] `delegate` and `undelegate` entry points exist and require auth.
- [ ] A delegate's vote carries their own weight plus all delegated weight.
- [ ] A delegator cannot vote directly while delegation is active.
- [ ] Test: delegator holds 500 tokens, delegate holds 200. Delegate votes For — assert `votes_for == 700`.
- [ ] Test: delegator revokes, then votes directly — assert their weight is counted once.
- [ ] `cargo test --all` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.

## Complexity

**High**
