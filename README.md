# SubRosa Sealed-Bid Auctions

SubRosa is a private auction protocol built for the Stellar ecosystem. It solves the problem of front-running and bid leakage in tokenized real-world asset and NFT auctions by using a commit-reveal flow on Soroban.

## Why this matters to Stellar

Stellar is increasingly being used for tokenized assets, real-world finance, and programmable value transfer. In those markets, auction transparency can backfire: when bid intent is visible before settlement, opportunistic actors can front-run bids, distort pricing, and make auctions unfair.

SubRosa gives Stellar markets a private auction mechanism that preserves fairness and confidentiality. This is valuable for:
- tokenized real estate and debt auctions
- NFT drops and collections
- private treasury auctions
- institutional bid rounds
- MEV-resistant smart-contract auction design

## Problem solved

Most blockchain auctions expose bid intent too early. On a public chain, the highest bid can be observed and exploited before final settlement. SubRosa prevents that by requiring sealed commitments during the bidding period and proof-of-ownership during the reveal phase.

## Architecture

The project is organized into a standard Soroban workspace:
- `contracts/subrosa_auction` — the Rust smart contract
- `frontend` — wallet-enabled Next.js + TypeScript UI
- `scripts` — deployment helpers for Stellar Testnet
- `docs` — architecture and security notes

## Core contract flow

1. Seller creates an auction with bid and reveal deadlines.
2. Bidders submit a sealed commitment and escrow collateral.
3. Bidders reveal their amount/hash later in the reveal window.
4. After the reveal deadline, anyone can finalize the auction.
5. The contract transfers the escrowed asset to the winner and the winning payment to the seller.
6. Losing bidders claim their collateral refunds through an authenticated refund call.

## Security design

The contract follows secure Soroban practices:
- authorization checks on privileged actions
- strict deadline enforcement
- explicit error handling for all state transitions
- collateral validation before lock-up
- pausable emergency control
- storage TTL extension for persistent state durability

## Tech stack
- Rust
- Soroban SDK
- Stellar Testnet deployment
- Next.js 14
- TypeScript
- Freighter wallet integration

## Project structure

```text
subrosa_auction/
├── Cargo.toml
├── contracts/
│   └── subrosa_auction/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── types.rs
│           ├── test.rs
│           └── integration_test.rs
├── docs/
│   ├── ARCHITECTURE.md
│   └── SECURITY.md
├── scripts/
│   └── deploy.sh
├── frontend/
│   ├── package.json
│   ├── README.md
│   └── src/
│       ├── app/
│       ├── components/
│       └── lib/
├── README.md
└── .gitignore
```

## Quick start

### Smart contract

```bash
cd contracts/subrosa_auction
cargo test
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

### Deploy to Stellar Testnet

```bash
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

## Security and best practices

- secure by design with explicit auth and validation
- modular contract logic for maintainability
- test-first development for core lifecycle flows
- modern UI patterns and wallet integration
- clear documentation for deployment and auditing

## Roadmap

- add auction cancellation and dispute handling
- add auction cancellation and dispute handling
- add multi-token support
- add seller dashboards and analytics
- add production monitoring and event indexing

## License

This project is intended as a starter and demonstration for private auction infrastructure on Stellar.
