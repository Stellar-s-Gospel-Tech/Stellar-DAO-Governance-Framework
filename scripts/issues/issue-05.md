## Summary

Anyone can currently create a proposal regardless of their token balance. This issue adds a minimum token balance requirement so only genuine stakeholders can submit proposals.

## Context

- `contracts/governance/src/lib.rs` — `create_proposal()` has a `// TODO (Phase 2): reject if proposer's token balance < min_proposal_threshold` comment.
- `contracts/governance/src/types.rs` — `GovernanceConfig` has a `// TODO (Phase 2): pub min_proposal_threshold: i128` comment.
- `contracts/governance/src/storage.rs` — `init()` has a matching TODO comment.

## What to implement

### 1. Add `min_proposal_threshold` to `GovernanceConfig`

```rust
pub min_proposal_threshold: i128,
```

Update `storage::init` and `GovernanceContract::initialize` to accept and store this value.

### 2. Enforce the threshold in `create_proposal`

In `governance/src/lib.rs`, before calling `proposal::create`:

```rust
let weight_addr = storage::weight_contract(&env);
let weight = token_weight::TokenWeightContractClient::new(&env, &weight_addr)
    .get_weight(&proposer);
let threshold = storage::config(&env).min_proposal_threshold;
if weight < threshold {
    panic!("{}", GovernanceError::BelowProposalThreshold as u32);
}
```

### 3. Add the error variant

In `error.rs`:
```rust
BelowProposalThreshold = 11,
```

## Acceptance criteria

- [ ] `GovernanceConfig` has `min_proposal_threshold`.
- [ ] `initialize` accepts the threshold parameter.
- [ ] `create_proposal` panics with `BelowProposalThreshold` when the proposer's balance is below the threshold.
- [ ] Test: set threshold to 100, try to create a proposal with a proposer holding 50 tokens — assert it fails.
- [ ] Test: proposer holding 100 tokens — assert proposal is created successfully.
- [ ] `cargo test --all` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.

## Complexity

**Medium**
