#!/usr/bin/env bash
set -uo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0
WARN=0

pass() { echo -e "  ${GREEN}PASS${NC}: $1"; ((PASS++)); }
fail() { echo -e "  ${RED}FAIL${NC}: $1"; ((FAIL++)); }
warn() { echo -e "  ${YELLOW}WARN${NC}: $1"; ((WARN++)); }

cd "$(dirname "$0")"

echo "=== Xarph Test Suite ==="
echo ""

# Phase 1: Compilation
echo "--- Phase 1: Compilation ---"
if cargo check --workspace 2>&1 | grep -q "^error"; then
    fail "workspace does not compile"
else
    pass "workspace compiles"
fi

echo ""

# Phase 2: Unit tests
echo "--- Phase 2: Unit Tests ---"
TEST_OUTPUT=$(cargo test --workspace 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: FAILED"; then
    fail "some unit tests failed"
    echo "$TEST_OUTPUT" | grep "FAILED" | head -5
else
    TEST_COUNT=$(echo "$TEST_OUTPUT" | grep "test result: ok" | grep -oP '\d+ passed' | grep -oP '\d+' | awk '{s+=$1} END {print s}')
    pass "all unit tests pass ($TEST_COUNT tests)"
fi

echo ""

# Phase 3: Compositor headless tests
echo "--- Phase 3: Compositor Headless Tests ---"
if cargo test -p xarph-wm 2>&1 | grep -q "test result: FAILED"; then
    fail "xarph-wm tests failed"
else
    pass "xarph-wm tests pass"
fi

echo ""

# Phase 4: Shell tests
echo "--- Phase 4: Shell Tests ---"
if cargo check -p xarph-shell 2>&1 | grep -q "^error"; then
    fail "xarph-shell does not compile"
else
    pass "xarph-shell compiles"
fi

echo ""

# Phase 5: Peripheral tests
echo "--- Phase 5: Peripheral Tests ---"
for crate in xarph-lock xarph-network xarph-settings xarph-admin xarph-services xarph-sdk Xarhives; do
    if cargo check -p "$crate" 2>&1 | grep -q "^error"; then
        fail "$crate does not compile"
    else
        pass "$crate compiles"
    fi
done

echo ""

# Phase 6: Config validation
echo "--- Phase 6: Config Validation ---"
if [ -f "xarph-wm/resources/default-config.kdl" ]; then
    pass "default config exists"
else
    warn "default config not found"
fi

echo ""

# Phase 7: Packaging validation
echo "--- Phase 7: Packaging Validation ---"
if [ -f "PKGBUILD" ]; then
    pass "PKGBUILD exists"
else
    fail "PKGBUILD not found"
fi
if [ -f "xarph-install.sh" ]; then
    pass "xarph-install.sh exists"
else
    fail "xarph-install.sh not found"
fi
if [ -f "xarph-uninstall.sh" ]; then
    pass "xarph-uninstall.sh exists"
else
    fail "xarph-uninstall.sh not found"
fi

echo ""

# Phase 8: Dead reference checks
echo "--- Phase 8: Dead Reference Checks ---"
if grep -r "xarph-launcher" --include="*.sh" --include="PKGBUILD" . 2>/dev/null | grep -v "Binary file" | grep -v ".git/" | grep -v "xarph-test.sh" > /dev/null 2>&1; then
    fail "xarph-launcher ghost references found"
else
    pass "no xarph-launcher ghost references"
fi
if grep -r "niri\." --include="*.service" --include="*.target" --include="*.desktop" --include="*.conf" . 2>/dev/null | grep -v ".git/" > /dev/null 2>&1; then
    warn "niri references found in packaging files"
else
    pass "no niri references in packaging files"
fi

echo ""

# Phase 9: Build artifact checks
echo "--- Phase 9: Build Artifact Checks ---"
if [ -f "xarph-wm/Cargo.lock" ]; then
    warn "stale xarph-wm/Cargo.lock found (workspace has root Cargo.lock)"
else
    pass "no stale xarph-wm/Cargo.lock"
fi

# Check for dead files
DEAD_FILES=("niri.service" "niri-shutdown.target" "niri-session" "niri.desktop" "niri-portals.conf")
DEAD_FOUND=0
for f in "${DEAD_FILES[@]}"; do
    if [ -f "$f" ]; then
        warn "dead file found: $f"
        DEAD_FOUND=1
    fi
done
if [ "$DEAD_FOUND" -eq 0 ]; then
    pass "no dead files"
fi

echo ""

# Summary
echo "=== Summary ==="
echo -e "  ${GREEN}PASS${NC}: $PASS"
echo -e "  ${YELLOW}WARN${NC}: $WARN"
echo -e "  ${RED}FAIL${NC}: $FAIL"
echo ""

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
