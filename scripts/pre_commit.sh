#!/usr/bin/env bash

set -euo pipefail

echo "==> Start fmt/check/clippy/tests"
./scripts/check.sh

echo
echo "==> Running custody demo..."
cargo run -- custody-demo

echo
echo "==> Running policy  demo..."
cargo run -- policy-demo
echo
echo "🎉 Ready to commit."
