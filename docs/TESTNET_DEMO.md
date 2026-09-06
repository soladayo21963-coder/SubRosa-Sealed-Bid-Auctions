# Stellar Testnet Demonstration

## Network

- Network: Stellar Testnet
- Stellar CLI: 28.0.0
- Admin account: `meridian-admin`
- Payment token contract: `CDIOIJWB226EI7SZHSOGLVDW5HKQOAHJSUTZLFDXTP32KDSTGQE5U55`
- Auction contract: `CBZ6WIHYHADTGDMFUSF6JRIRKT7423JKF7NVDDVPHYROZYOWZXEEE76N`
- Contract WASM hash at deployment: `6556ecba534cb13eb1c78b2e9fd5679a89f5d9d27eea55c329cb6ac17072d6b9`

## Recorded transactions

- Payment token deployment: completed on Testnet
- Auction contract deployment: completed on Testnet
- Contract initialization: completed on Testnet
- Auction creation: auction `1` and auction `2` were created
- Bidder trustline: completed
- Bidder funding: `1,000,000` SUBR transferred
- Sealed bid submission: `f975e9393a9f261b53b4a58e261fa12c8fea9051d8d3fa0acc06875927834856`
- Auction finalization: `8d770dd3ea5712bc2310bbb1268dfb15afa281d8e9eea456d9b575015c6c2159`
- Bidder refund: `ac8e4b112e294ff5619ad5efffeac7180f4299c52ec828acda6039f968df6b29`

## Important version note

The deployed Testnet WASM predates the current domain-separated commitment format in the repository. The current source hashes:

```text
16-byte big-endian i128 bid amount || 32-byte salt || 8-byte big-endian auction ID
```

The historical deployed contract rejected that newer commitment during reveal, as expected for a version mismatch. The invalid bid was not accepted, the auction finalized without a winner, and the full `800000` SUBR collateral was refunded.

For a successful reveal demonstration, deploy the current source again, initialize that new contract, create a new auction, and generate the commitment with the current frontend or current documented encoding. Do not present the historical contract ID as a deployment of a different source revision.
