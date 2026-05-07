# Contract Reference

Quick reference for every public function across all three contracts.

---

## governance

### `initialize(admin, weight_contract, quorum_bps, voting_period_ledgers)`

One-time setup. Must be called immediately after deploy.

| Param | Type | Description |
|---|---|---|
| `admin` | `Address` | Can upgrade the contract (Phase 3) |
| `weight_contract` | `Address` | Deployed `token-weight` contract address |
| `quorum_bps` | `u32` | Approval threshold in basis points (5000 = 50%) |
| `voting_period_ledgers` | `u32` | How long proposals stay open (~17280 ≈ 1 day) |

---

### `create_proposal(proposer, title, description, action_payload) → u64`

Creates a new proposal. Returns the proposal ID (1-indexed).

Requires auth from `proposer`.

| Param | Type | Description |
|---|---|---|
| `proposer` | `Address` | Must sign the transaction |
| `title` | `String` | Short title |
| `description` | `String` | Full rationale |
| `action_payload` | `Bytes` | Encoded cross-contract call, or empty for signalling-only |

---

### `vote(voter, proposal_id, choice)`

Cast a vote on an active proposal. Requires auth from `voter`.

| Param | Type | Description |
|---|---|---|
| `voter` | `Address` | Must sign the transaction |
| `proposal_id` | `u64` | Target proposal |
| `choice` | `VoteChoice` | `For`, `Against`, or `Abstain` |

Errors: `ProposalNotActive`, `VotingPeriodEnded`, `AlreadyVoted`

---

### `finalize(proposal_id) → ProposalStatus`

Tallies votes and sets the final status. Can be called by anyone after the voting period ends.

Returns `Approved` or `Rejected`.

Errors: `VotingPeriodNotEnded`

---

### `execute(proposal_id)`

Executes the on-chain action of an approved proposal.

Errors: `ProposalNotApproved`

> Phase 2: will dispatch the cross-contract call encoded in `action_payload`.
> Phase 3: will enforce a timelock delay.

---

### Views

| Function | Returns | Description |
|---|---|---|
| `get_proposal(id)` | `Proposal` | Full proposal struct |
| `get_vote(id, voter)` | `Option<Vote>` | Vote cast by `voter` on proposal `id` |
| `proposal_count()` | `u64` | Total proposals created |
| `get_config()` | `GovernanceConfig` | Current governance parameters |

---

## treasury

### `initialize(admin, governance_contract)`

One-time setup.

| Param | Type | Description |
|---|---|---|
| `admin` | `Address` | Can rotate the governance contract address |
| `governance_contract` | `Address` | The only address allowed to call `spend()` |

---

### `spend(token, recipient, amount)`

Transfer `amount` of `token` to `recipient`. Requires auth from the governance contract.

| Param | Type | Description |
|---|---|---|
| `token` | `Address` | SAC token contract address |
| `recipient` | `Address` | Destination address |
| `amount` | `i128` | Amount in the token's smallest unit |

> Phase 3: will enforce per-tx spend cap and daily limit.

---

### Views

| Function | Returns | Description |
|---|---|---|
| `balance(token)` | `i128` | Treasury's current balance of `token` |
| `get_config()` | `TreasuryConfig` | Admin and governance contract addresses |
| `spend_count()` | `u64` | Total number of spend records |
| `get_spend_record(index)` | `Option<SpendRecord>` | Spend audit entry at `index` |

---

## token-weight

### `initialize(admin, strategy, token)`

One-time setup.

| Param | Type | Description |
|---|---|---|
| `admin` | `Address` | Can update token address (Phase 3) |
| `strategy` | `WeightStrategy` | `NativeToken` or `LpToken` |
| `token` | `Address` | SAC or LP token whose balance = voting weight |

---

### `get_weight(voter) → i128`

Returns the current voting weight of `voter` (their token balance).

> Phase 2: will be replaced by `get_weight_at(voter, snapshot_ledger)` to prevent flash-loan attacks.

---

### Views

| Function | Returns | Description |
|---|---|---|
| `get_strategy()` | `WeightStrategy` | Active weight strategy |
| `get_token()` | `Address` | Token used for weight calculation |

---

## Types

### `VoteChoice`
```
For | Against | Abstain
```

### `ProposalStatus`
```
Active → Approved | Rejected → Executed
```

### `WeightStrategy`
```
NativeToken | LpToken
```

### `Proposal`
```
id, proposer, title, description, action_payload,
votes_for, votes_against, votes_abstain, status, end_ledger
```

### `Vote`
```
voter, choice, weight
```

### `SpendRecord`
```
token, recipient, amount, ledger
```
