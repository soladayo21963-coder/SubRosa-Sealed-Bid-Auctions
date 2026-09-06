# Architecture Overview

## Problem addressed

Open order books and public bidding on Stellar tokenized assets expose bidder intent before settlement. This creates front-running risk, information leakage, and poor auction outcomes for real-world asset and NFT auctions.

## Solution

SubRosa introduces a sealed-bid auction protocol built on Soroban. Users submit collateral and a bid commitment during the bidding phase. During the reveal phase, they reveal the amount and salt. The contract verifies the hash and updates the current winning bid.

## Components
- Smart contract: on-chain auction lifecycle and winner tracking
- Frontend: wallet integration, auction UI, transaction UX
- Deployment scripts: Stellar Testnet setup and contract publishing
- Documentation: onboarding, security, and architecture guidance

## Lifecycle
1. Admin initializes the contract and token config.
2. Seller creates an auction with bid and reveal deadlines.
3. Bidders submit sealed commitments and collateral.
4. Reveal period verifies bid against the commitment hash.
5. Highest valid bid becomes the winner.
6. Settlement can be extended to payout logic, transfer flows, and asset release.
