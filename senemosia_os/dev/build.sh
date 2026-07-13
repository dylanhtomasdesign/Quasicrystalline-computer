#!/bin/bash
set -e
echo "=== Building Senemosia Kernel ==="
rustup target add x86_64-unknown-none 2>/dev/null || true
cd "$(dirname "$0")/.." && cargo build --release
echo "=== Build complete ==="
