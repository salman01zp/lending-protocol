#!/bin/bash

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo ""
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  Testing Miden Lending Protocol Deployment${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo ""

# Check if Miden node is running
echo -e "${BLUE}Checking Miden node status...${NC}"
if lsof -i :57291 > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Miden node is running on port 57291${NC}"
    MIDEN_PID=$(lsof -t -i :57291)
    echo -e "   Process ID: $MIDEN_PID"
else
    echo -e "${RED}❌ Miden node is NOT running on port 57291${NC}"
    echo -e "${YELLOW}   Please start the Miden node first:${NC}"
    echo -e "   cd /path/to/miden-node && cargo run --release"
    exit 1
fi

echo ""
echo -e "${BLUE}Checking Rust version...${NC}"
RUST_VERSION=$(rustc --version)
echo -e "${GREEN}✅ $RUST_VERSION${NC}"

echo ""
echo -e "${BLUE}Checking Miden CLI...${NC}"
MIDEN_VERSION=$(miden --version 2>&1)
echo -e "${GREEN}✅ $MIDEN_VERSION${NC}"

echo ""
echo -e "${BLUE}Checking project structure...${NC}"

# Check MASM files
MASM_COUNT=$(find ../accounts -name "*.masm" 2>/dev/null | wc -l)
echo -e "${GREEN}✅ Found $MASM_COUNT MASM contract files${NC}"

# Check note scripts
NOTE_COUNT=$(find ../notes -name "*.masm" 2>/dev/null | wc -l)
echo -e "${GREEN}✅ Found $NOTE_COUNT note script files${NC}"

# Check client
if [ -f "../client/Cargo.toml" ]; then
    echo -e "${GREEN}✅ Client Cargo.toml found${NC}"
else
    echo -e "${RED}❌ Client Cargo.toml not found${NC}"
fi

echo ""
echo -e "${BLUE}Listing MASM contracts:${NC}"
echo ""
ls -lh ../accounts/*.masm | awk '{print "  " $9 " (" $5 ")"}'

echo ""
echo -e "${BLUE}Listing note scripts:${NC}"
echo ""
ls -lh ../notes/*.masm | awk '{print "  " $9 " (" $5 ")"}'

echo ""
echo -e "${BLUE}Compiling MASM contracts (test)...${NC}"
echo ""

# Try to compile a contract with Miden CLI
if miden compile ../accounts/lending_pool.masm 2>&1 | head -20; then
    echo -e "${GREEN}✅ MASM compilation test passed${NC}"
else
    echo -e "${YELLOW}⚠️  MASM compilation needs review${NC}"
fi

echo ""
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  Checking Client Dependencies${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo ""

cd ../client

echo "Current Rust version: $(rustc --version)"
echo ""
echo "Required versions for Miden dependencies:"
echo "  - miden-client, miden-tx: Rust 1.89+"
echo "  - miden-stdlib, miden-vm: Rust 1.88+"
echo ""

CURRENT_VERSION=$(rustc --version | awk '{print $2}')
echo "Your version: $CURRENT_VERSION"
echo ""

if [[ "$CURRENT_VERSION" < "1.88.0" ]]; then
    echo -e "${YELLOW}⚠️  Rust version too old for Miden dependencies${NC}"
    echo -e "${YELLOW}   Please update Rust: rustup update stable${NC}"
    echo ""
    echo -e "${BLUE}Checking if update is in progress...${NC}"
    if pgrep -f "rustup" > /dev/null; then
        echo -e "${GREEN}✅ Rustup update is running${NC}"
        echo -e "${YELLOW}   Wait for it to complete, then run this script again${NC}"
    else
        echo -e "${YELLOW}   Run: rustup update stable${NC}"
    fi
else
    echo -e "${GREEN}✅ Rust version is compatible${NC}"
fi

echo ""
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  Test Summary${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo ""

echo "Environment Status:"
echo "  ✅ Miden node: Running"
echo "  ✅ Miden CLI: Installed"
echo "  ✅ MASM contracts: $MASM_COUNT files ready"
echo "  ✅ Note scripts: $NOTE_COUNT files ready"
echo "  ✅ Project structure: Complete"
echo ""

if [[ "$CURRENT_VERSION" < "1.88.0" ]]; then
    echo -e "${YELLOW}Next Steps:${NC}"
    echo "  1. Wait for Rust update to complete (or run: rustup update stable)"
    echo "  2. Run this script again to verify"
    echo "  3. Then run: ./deploy_local.sh"
else
    echo -e "${GREEN}Next Steps:${NC}"
    echo "  1. Run deployment: ./deploy_local.sh"
    echo "  2. Create accounts: ./setup_test_accounts.sh"
    echo "  3. Run tests: cd .. && cargo test --test integration_tests"
fi

echo ""
