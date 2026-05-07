# Roadmap

This document tracks what is built, what is in progress, and what is planned.

---

## ✅ Phase 1 — Skeleton (current)

- [x] Workspace structure: `governance`, `treasury`, `token-weight`
- [x] Proposal lifecycle: create → vote → finalize → execute (skeleton)
- [x] Treasury: spend gated by governance contract
- [x] Token-weight: balance-based voting power (native token + LP token strategy)
- [x] Unit tests for each contract

---

## 🔨 Phase 2 — Core Logic

- [ ] **Snapshot voting** — record token balance at proposal creation ledger to prevent flash-loan manipulation
- [ ] **Cross-contract weight call** — governance calls `token-weight.get_weight()` at vote time
- [ ] **On-chain execution** — decode `action_payload` and dispatch cross-contract call from `execute()`
- [ ] **Quorum against total supply** — fetch total supply from token contract for accurate quorum check
- [ ] **Proposal creation threshold** — minimum token balance required to submit a proposal
- [ ] **Delegation** — allow token holders to delegate voting power to another address

---

## 🔭 Phase 3 — Hardening

- [ ] **Timelock** — mandatory delay between `finalize` and `execute` for security
- [ ] **Spend caps** — per-transaction and daily limits on treasury outflows
- [ ] **Multi-token treasury** — track balances across multiple SAC tokens
- [ ] **Upgradeable contracts** — admin-controlled WASM upgrade path
- [ ] **Fuzz tests** — property-based testing for edge cases
- [ ] **Audit** — third-party security review

---

## 🌐 Phase 4 — Ecosystem

- [ ] **TypeScript SDK** — typed client wrappers for frontend integration
- [ ] **Reference frontend** — minimal Next.js UI for proposal browsing and voting
- [ ] **Indexer integration** — Horizon / Mercury event indexing for proposal history
- [ ] **Multi-sig admin** — replace single admin with a multisig threshold
- [ ] **Docs site** — full developer documentation
