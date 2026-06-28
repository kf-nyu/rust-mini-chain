#!/usr/bin/env bash

set -euo pipefail

echo "==> Formatting..."
cargo fmt

echo
echo "==> Checking..."
cargo check

echo
echo "==> Running Clippy..."
cargo clippy --all-targets -- -D warnings

echo
echo "==> Running all tests..."
cargo test --all-targets

echo
echo "✅ All checks passed!"
