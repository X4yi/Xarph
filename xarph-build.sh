#!/bin/sh
set -e

SRCDIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SRCDIR"

echo "=== Xarph Build & Install ==="

# 1. Build release
echo "[1/3] Building release binaries..."
cargo build --release --workspace

# 2. Check all binaries exist
echo "[2/3] Verifying binaries..."
for bin in xarph-shell xarph-settings xarph-lock xarph-services xarph-network xarph-admin xarph-wm Xarhives; do
    if [ ! -f "target/release/$bin" ]; then
        echo "ERROR: target/release/$bin not found"
        exit 1
    fi
done

# 3. Install
echo "[3/3] Installing (requires sudo)..."
sudo bash xarph-install.sh

echo ""
echo "Done. Select 'Xarph' from your display manager or run 'xarph-session'."
