# Security Notes

## Core security model

SubRosa protects bidder intent through a commit-reveal pattern. Bidders must commit to a hash before reveal time and only reveal at a later stage. This prevents front-running while still allowing deterministic winner selection.

## Key safeguards
- authorization checks using `require_auth()`
- strict time-window validation for bidding and reveal phases
- collateral validation before accepting sealed bids
- pause switch for emergency response
- explicit error codes instead of unchecked state transitions

## Best practices
- keep contract logic simple and deterministic
- avoid state changes in reveal logic beyond winner tracking
- validate all input ranges before mutation
- extend TTL for persistent storage to avoid accidental eviction
- audit and test all edge cases before mainnet deployment
