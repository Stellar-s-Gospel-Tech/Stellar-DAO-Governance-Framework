## Summary

Key state transitions in the governance and treasury contracts currently happen silently. This issue adds `soroban_sdk` events to `create_proposal`, `vote`, `finalize`, and `spend` so indexers and frontends can track activity without polling storage.

## Context

- `contracts/governance/src/voting.rs` — `// TODO (Phase 2): emit a Voted event` comment.
- `contracts/treasury/src/lib.rs` — `// TODO (Phase 2): emit a Spent event` comment.
- Soroban events docs: https://developers.stellar.org/docs/build/guides/events

## What to implement

Emit events using `env.events().publish()`. The first argument is a topics tuple, the second is the data value.

### governance — `proposal::create`
```rust
env.events().publish(
    (soroban_sdk::symbol_short!("created"), proposal.id),
    proposal.proposer.clone(),
);
```

### governance — `voting::cast_vote`
```rust
env.events().publish(
    (soroban_sdk::symbol_short!("voted"), proposal_id),
    (voter.clone(), choice.clone(), weight),
);
```

### governance — `proposal::finalize`
```rust
env.events().publish(
    (soroban_sdk::symbol_short!("finalized"), proposal_id),
    proposal.status.clone(),
);
```

### treasury — `lib::spend`
```rust
env.events().publish(
    (soroban_sdk::symbol_short!("spent"), token.clone()),
    (recipient.clone(), amount),
);
```

## Acceptance criteria

- [ ] All four events are emitted at the correct points.
- [ ] Tests assert events using `env.events().all()` after each call.
- [ ] No existing tests are broken.
- [ ] `cargo test --all` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.

## Complexity

**Trivial**
