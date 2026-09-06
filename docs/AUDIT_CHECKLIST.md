# Pre-Mainnet Audit Checklist

This repository is a Testnet candidate, not an independently audited financial protocol.

## Contract review

- [x] Authorization on admin, seller, bidder, and refund actions
- [x] Seller asset escrow before auction activation
- [x] Duplicate bid rejection
- [x] Commitment validation with auction-domain separation
- [x] Reveal and settlement deadline checks
- [x] Winner excess collateral handling
- [x] Losing and unrevealed bidder refunds
- [x] Refunds remain available during pause
- [ ] Independent third-party audit
- [ ] Formal review of Soroban storage TTL assumptions
- [ ] Mainnet token and issuer configuration review
- [ ] Fuzz/property tests for accounting conservation

## Frontend and operations

- [x] Freighter wallet connection
- [x] Soroban transaction simulation and signing
- [x] Confirmation polling and failure reporting
- [x] Seller creation workflow
- [x] Auction discovery workflow
- [x] Finalization and refund controls
- [ ] Indexer-backed discovery for large auction volumes
- [ ] Wallet network/account change handling
- [ ] Testnet deployment and live end-to-end test
- [ ] Mainnet deployment approval and monitoring

## Required external review

Before mainnet use, an independent Soroban auditor should verify authorization, token transfer semantics, accounting conservation, TTL behavior, denial-of-service resistance, and all failure paths. Never treat passing unit tests as a substitute for an audit.
