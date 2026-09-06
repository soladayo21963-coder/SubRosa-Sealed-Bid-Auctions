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
1. Admin initializes the contract with the collateral/payment token.
2. Seller transfers the auction asset into escrow and creates an auction with bid and reveal deadlines.
3. Bidders submit one sealed commitment and collateral.
4. Reveal verifies `i128 bid amount || 32-byte salt || u64 auction ID` against SHA-256.
5. The highest valid bid becomes the winner; equal bids keep the first revealed bidder.
6. After the reveal deadline, anyone can finalize the auction.
7. The contract transfers the asset to the winner, pays the seller, and exposes authenticated refund claims.

## Accounting model

The configured token is the payment/collateral token. The asset token may be a different Stellar token. The seller escrows `asset_amount` during creation. At settlement, the contract pays the winning bid to the seller; the winner can claim any collateral excess and every losing or unrevealed bidder can claim their full collateral.
