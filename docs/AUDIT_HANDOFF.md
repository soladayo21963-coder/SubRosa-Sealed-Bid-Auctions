# Independent Security Audit Handoff

## Audit status

This repository has received an internal engineering review and live Testnet exercise. It has **not** received an independent security audit. Passing tests and a successful Testnet run are not substitutes for external review.

## Scope

An independent Soroban auditor should review:

1. Authorization and signer requirements for initialization, seller actions, bidder actions, settlement, refunds, and pause control.
2. Payment-token and asset-token transfer semantics, including distinct-token auctions.
3. Accounting conservation across seller escrow, bidder collateral, winning payment, winner excess refund, loser refunds, no-bid auctions, and failed reveals.
4. Commitment encoding and domain separation:
   `i128 bid amount || 32-byte salt || u64 auction ID`, all big-endian.
5. Duplicate-bid rejection and replay/cross-auction resistance.
6. Deadline boundary behavior and settlement finality.
7. Storage TTL behavior for long-lived auctions and bids.
8. Pause behavior and whether users can always recover refundable funds.
9. Denial-of-service risks from auction discovery and unbounded user-controlled operations.
10. Error handling and token-transfer failure paths.

## Reproduction commands

```bash
cargo test --workspace
stellar contract build
cd frontend && npm ci && npm run build
```

## Materials to provide an auditor

- the Git repository and commit under review
- `contracts/subrosa_auction/src/lib.rs`
- `contracts/subrosa_auction/src/types.rs`
- unit and integration tests
- `docs/SECURITY.md`
- `docs/ARCHITECTURE.md`
- `docs/AUDIT_CHECKLIST.md`
- `docs/TESTNET_DEMO.md`
- the exact deployed WASM hash and network contract ID

## Acceptance criteria

The audit should produce a written report containing:

- severity-ranked findings
- affected functions and reproducible steps
- accounting and authorization analysis
- recommended remediations
- retest status for every finding
- explicit statement of whether mainnet deployment is recommended

Never provide a secret key, seed phrase, or Freighter password to an auditor, agent, or repository.
