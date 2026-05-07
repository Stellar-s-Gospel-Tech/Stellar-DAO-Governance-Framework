# Architecture

This document describes the design of the Stellar DAO Governance Framework — how the three contracts relate to each other, what each one owns, and the decisions behind the structure.

---

## Overview

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
         ↑ get_weight()        ↑ spend()
    SAC token / LP token    any SAC token
```

The three contracts are deliberately separated so each can be upgraded or replaced independently:

| Contract | Owns | Called by |
|---|---|---|
| `governance` | Proposal state, vote tallies, config | Anyone (create/vote), anyone (finalize), governance itself (execute) |
| `treasury` | DAO funds (any SAC token) | Governance contract only |
| `token-weight` | Voting power resolution logic | Governance contract |

---

## Contract Responsibilities

### governance

The central contract. Manages the full proposal lifecycle:

```
create_proposal()
      │
      ▼
   Active  ──── vote() ────►  tally accumulates
      │
      │  (voting_period_ledgers pass)
      ▼
  finalize()
      │
      ├── votes_for / total >= quorum_bps  ──► Approved
      └── otherwise                        ──► Rejected
                                                │
                                         execute() (Phase 2+)
                                                │
                                            Executed
```

Key design decisions:
- **Quorum in basis points** — `quorum_bps = 5000` means 50% of cast votes must be For. This is currently majority-of-cast, not majority-of-supply (Phase 2 upgrades this).
- **action_payload** — proposals carry an optional encoded cross-contract call. Empty bytes = signalling-only. Dispatch is a Phase 2 task.
- **Voting period in ledgers** — ~1 ledger ≈ 5 seconds on Stellar mainnet. `17280 ledgers ≈ 1 day`.

### treasury

Holds DAO funds. The only way to move funds out is via `spend()`, which requires auth from the governance contract address. This means a proposal must be created, voted on, approved, and executed before any funds move.

Deposits are implicit — anyone can transfer SAC tokens to the treasury contract address.

Every spend is recorded in an append-only audit log (`SpendRecord`).

### token-weight

Decouples voting power resolution from governance logic. The governance contract calls `get_weight(voter)` here instead of reading token balances directly. This means:

- Swapping from native token to LP token weight requires only redeploying `token-weight` and updating the address in governance config.
- Future strategies (quadratic, capped) can be added without touching governance.

Currently returns the live token balance. Phase 2 adds snapshot-based weight to prevent flash-loan attacks.

---

## Storage Layout

### governance

| Key | Storage type | Description |
|---|---|---|
| `Config` | Instance | Admin, weight contract, quorum, voting period |
| `ProposalCount` | Instance | Running proposal ID counter |
| `Proposal(id)` | Persistent | Full proposal struct |
| `Vote(id, voter)` | Persistent | Vote struct per (proposal, voter) pair |

### treasury

| Key | Storage type | Description |
|---|---|---|
| `Config` | Instance | Admin, governance contract address |
| `SpendCount` | Instance | Running spend record counter |
| `SpendRecord(index)` | Persistent | Immutable spend audit entry |

### token-weight

| Key | Storage type | Description |
|---|---|---|
| `Admin` | Instance | Admin address |
| `Strategy` | Instance | `NativeToken` or `LpToken` |
| `Token` | Instance | SAC / LP token address |

---

## Cross-Contract Call Flow (Phase 2)

Once fully wired, a vote will flow like this:

```
voter → governance.vote(proposal_id, For)
              │
              └─► token_weight.get_weight(voter)
                        │
                        └─► token.balance(voter)  [SAC call]
                        ◄── weight: i128
              ◄── weight
        tally += weight
```

And an execution:

```
anyone → governance.execute(proposal_id)
               │
               └─► decode action_payload → { contract, function, args }
               └─► env.invoke_contract(contract, function, args)
               └─► treasury.spend(token, recipient, amount)  [if payload targets treasury]
```

---

## Phase Roadmap Summary

| Phase | Focus |
|---|---|
| 1 (current) | Skeleton contracts, passing tests, contributor scaffolding |
| 2 | Token-weighted voting, snapshot weight, cross-contract dispatch, proposal threshold |
| 3 | Timelock, spend caps, delegation, upgradeable contracts, fuzz tests |
| 4 | TypeScript SDK, reference frontend, indexer integration |

See [ROADMAP.md](../ROADMAP.md) for the full task breakdown.
