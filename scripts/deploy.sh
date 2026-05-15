#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# WaveEscrow Deploy Script
#
# Prerequisites:
#   - cargo-stylus:  cargo install cargo-stylus
#   - Rust nightly:  rustup toolchain install nightly
#   - wasm target:   rustup target add wasm32-unknown-unknown
#
# Usage:
#   export RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
#   export PRIVATE_KEY=0x...
#   bash scripts/deploy.sh
# ---------------------------------------------------------------------------
set -euo pipefail

RPC_URL="${RPC_URL:-https://sepolia-rollup.arbitrum.io/rpc}"
PRIVATE_KEY="${PRIVATE_KEY:-}"

if [ -z "$PRIVATE_KEY" ]; then
    echo "ERROR: PRIVATE_KEY environment variable is not set."
    exit 1
fi

echo "==> Building WaveEscrow..."
cargo stylus build

echo "==> Estimating gas..."
cargo stylus estimate --endpoint "$RPC_URL" --private-key "$PRIVATE_KEY"

echo "==> Deploying..."
cargo stylus deploy \
    --endpoint "$RPC_URL" \
    --private-key "$PRIVATE_KEY"

echo "==> Done."
