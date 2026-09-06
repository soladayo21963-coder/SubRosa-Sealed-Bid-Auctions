# Security Notes

## Core security model

SubRosa protects bidder intent through a commit-reveal pattern. Bidders must commit to a hash before reveal time and only reveal at a later stage. This prevents front-running while still allowing deterministic winner selection.

## Key safeguards
- authorization checks using `require_auth()`
- strict time-window validation for bidding and reveal phases
- collateral validation before accepting sealed bids
- duplicate-bid rejection so collateral cannot be overwritten
- commitment domain separation using `bid_amount || salt || auction_id`
- seller asset escrow before an auction becomes active
- winner excess-collateral and loser refund paths after settlement
- pause switch for emergency response
- explicit error codes instead of unchecked state transitions

## State invariants

- Every active auction asset is held by the contract, not the seller.
- A bidder has at most one bid record per auction.
- A revealed bid cannot be revealed twice.
- Settlement happens only after the reveal deadline.
- The contract pays at most the winning bid to the seller and returns all remaining bidder collateral through authenticated claims.
- Refunds remain available while the emergency pause is active so a pause cannot permanently trap user funds.

## Known operational risks

- The admin key controls the pause switch and must be secured with a dedicated signer.
- Persistent storage TTL must be monitored and extended for long-lived deployments.
- Testnet deployment must use a configured payment token and a funded CLI identity.
- This prototype still needs an independent audit before mainnet use.

## Best practices
- keep contract logic simple and deterministic
- avoid state changes in reveal logic beyond winner tracking
- validate all input ranges before mutation
- extend TTL for persistent storage to avoid accidental eviction
- audit and test all edge cases before mainnet deployment
