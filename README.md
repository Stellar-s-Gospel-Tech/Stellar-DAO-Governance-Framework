# Stellar DAO Governance Framework

A Soroban smart contract framework for building DAOs on Stellar — proposals, voting, and treasury management with token-weighted or LP-token-weighted voting power.

## Problem

No standard governance tooling exists on Stellar. Projects building DAOs must roll their own proposal and voting logic from scratch, leading to fragmented, untested implementations. This framework provides a composable, auditable baseline.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   DAO Frontend / SDK                │
└────────────────────────┬────────────────────────────┘
                         │
          ┌──────────────▼──────────────┐
          │      governance contract    │  ← proposals, voting, finalize, execute
          └──────┬──────────────┬───────┘
                 │              │
    ┌────────────▼───┐   ┌──────▼──────────┐
    │ token-weight   │   │    treasury     │
    │   contract     │   │    contract     │
    └────────────────┘   └─────────────────┘
         ↑ balance()           ↑ spend()
    SAC token / LP token    any SAC token
```

### Contracts

| Contract | Purpose |
|---|---|
| `governance` | Proposal lifecycle: create → vote → finalize → execute |
| `treasury` | Holds DAO funds; releases only on governance instruction |
| `token-weight` | Resolves voting power from a native or LP token balance |

## Quickstart

### Prerequisites

- Rust + `wasm32v1-none` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli)

```bash
rustup target add wasm32v1-none
cargo install --locked stellar-cli
```

### Build

```bash
stellar contract build
```

### Test

```bash
cargo test
```

### Deploy (Testnet)

```bash
# 1. Deploy token-weight
stellar contract deploy \
  --wasm target/wasm32v1-none/release/token_weight.wasm \
  --network testnet --source <YOUR_KEY>

# 2. Deploy treasury
stellar contract deploy \
  --wasm target/wasm32v1-none/release/treasury.wasm \
  --network testnet --source <YOUR_KEY>

# 3. Deploy governance (pass token-weight address during initialize)
stellar contract deploy \
  --wasm target/wasm32v1-none/release/governance.wasm \
  --network testnet --source <YOUR_KEY>
```

## Docs

| Document | Description |
|---|---|
| [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) | Design decisions, storage layout, cross-contract call flow |
| [docs/CONTRACT_REFERENCE.md](./docs/CONTRACT_REFERENCE.md) | Every public function, params, return types, errors |
| [ROADMAP.md](./ROADMAP.md) | Phase-by-phase task breakdown |
| [CONTRIBUTING.md](./CONTRIBUTING.md) | How to contribute |

## License

MIT
