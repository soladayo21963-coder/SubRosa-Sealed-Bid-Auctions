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

: "${ADMIN_ADDRESS:?Set ADMIN_ADDRESS to the funded Testnet admin address}"
: "${PAYMENT_TOKEN_ID:?Set PAYMENT_TOKEN_ID to the collateral token contract address}"

stellar keys show admin >/dev/null 2>&1 || {
  echo "Missing Stellar CLI identity 'admin'. Import or create it before deploying."
  exit 1
}

echo "Deploying to Stellar Testnet..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_FILE" \
  --source-account admin \
  --network testnet)

echo "Contract deployed successfully: $CONTRACT_ID"
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source-account admin \
  --network testnet \
  -- initialize \
  --admin "$ADMIN_ADDRESS" \
  --token "$PAYMENT_TOKEN_ID"

echo "NEXT_PUBLIC_SUBROSA_CONTRACT_ID=$CONTRACT_ID" > ./frontend/.env.local
echo "NEXT_PUBLIC_SOROBAN_NETWORK=testnet" >> ./frontend/.env.local
