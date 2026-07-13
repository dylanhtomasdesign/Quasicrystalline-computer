#!/bin/bash
set -e
echo "=== Generating Senemosia ISO ==="
./dev/build.sh
mkdir -p iso_root/boot
mkdir -p iso_root/EFI/BOOT
cp target/x86_64-unknown-none/release/senemosia_kernel iso_root/boot/ 2>/dev/null || true
cp limine.cfg iso_root/boot/ 2>/dev/null || true
echo "=== ISO structure ready ==="
