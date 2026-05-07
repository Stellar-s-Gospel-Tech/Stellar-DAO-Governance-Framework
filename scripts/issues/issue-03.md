## Summary

`governance::execute()` currently just marks a proposal as `Executed` without dispatching any on-chain action. This issue implements the cross-contract dispatch so approved proposals can actually move funds, update config, or call any contract.

## Context

- `contracts/governance/src/proposal.rs` — `execute()` has a `// TODO (Phase 2): decode action_payload and dispatch cross-contract call` comment.
- `contracts/governance/src/types.rs` — `Proposal.action_payload` is a `Bytes` field intended to carry the encoded call.

## What to implement

### 1. Define `ActionPayload` in `types.rs`

```rust
#[contracttype]
pub struct ActionPayload {
    pub contract: Address,
    pub function: soroban_sdk::Symbol,
    pub args: soroban_sdk::Vec<soroban_sdk::Val>,
}
```

### 2. Implement dispatch in `proposal::execute`

```rust
if !proposal.action_payload.is_empty() {
    let payload: ActionPayload = env.from_xdr(&proposal.action_payload).unwrap();
    env.invoke_contract::<()>(&payload.contract, &payload.function, payload.args);
}
```

### 3. Signalling-only proposals

If `action_payload` is empty, skip dispatch and just mark as `Executed`. This is already the behaviour — preserve it.

## Acceptance criteria

- [ ] `ActionPayload` struct is defined in `types.rs`.
- [ ] `execute()` decodes and dispatches the payload when non-empty.
- [ ] Empty payload proposals still execute without error.
- [ ] Test: create a proposal with a payload targeting the treasury's `spend()`, approve it, execute it, assert the recipient received funds.
- [ ] `cargo test --all` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.

## Complexity

**High**
