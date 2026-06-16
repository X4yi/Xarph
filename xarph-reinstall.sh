#!/bin/sh
# xarph-reinstall - Cleanly remove and reinstall Xarph from the local workspace
#
# Usage: xarph-reinstall [--purge]
#   --purge   Also remove user config and data directories before reinstalling

set -e

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PURGE=false
if [ "$1" = "--purge" ]; then
    PURGE=true
fi

if [ "$PURGE" = true ]; then
    "${SCRIPT_DIR}/xarph-uninstall.sh" --purge
else
    "${SCRIPT_DIR}/xarph-uninstall.sh"
fi

echo ""
echo "Cleaning build artifacts..."
rm -rf "${SCRIPT_DIR}/target" "${SCRIPT_DIR}/target-pkg" "${SCRIPT_DIR}/pkg"

if command -v cargo >/dev/null 2>&1; then
    cargo clean --manifest-path "${SCRIPT_DIR}/Cargo.toml" >/dev/null 2>&1 || true
fi

if command -v makepkg >/dev/null 2>&1; then
    echo "Reinstalling package with makepkg..."
    cd "${SCRIPT_DIR}"
    makepkg -Csi
else
    echo "makepkg is not available. Build the package manually from ${SCRIPT_DIR}."
    exit 1
fi
