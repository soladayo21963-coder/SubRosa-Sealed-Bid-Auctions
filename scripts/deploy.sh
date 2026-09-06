#!/usr/bin/env bash
set -euo pipefail

echo "Building SubRosa WASM binary..."
cd "$(dirname "$0")/.."

stellar contract build

WASM_FILE="target/wasm32v1-none/release/subrosa_auction.wasm"

if [[ ! -f "$WASM_FILE" ]]; then
  echo "WASM artifact not found at $WASM_FILE"
  exit 1
fi

echo "Deploying to Stellar Testnet..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_FILE" \
  --source admin \
  --network testnet)

echo "Contract deployed successfully: $CONTRACT_ID"
echo "NEXT_PUBLIC_SUBROSA_CONTRACT_ID=$CONTRACT_ID" > ./frontend/.env.local
