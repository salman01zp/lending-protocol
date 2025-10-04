#!/bin/bash

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo ""
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  👥 Creating Test Accounts${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo ""

cd "$(dirname "$0")/../client"

# Check if deployment info exists
if [ ! -f "deployment_info.json" ]; then
    echo -e "${YELLOW}⚠️  Warning: deployment_info.json not found${NC}"
    echo -e "${YELLOW}   Run ./scripts/deploy_local.sh first${NC}"
    echo ""
fi

echo -e "${BLUE}Creating test accounts...${NC}"
echo ""

# Create Alice (private account for regular user)
echo "Creating Alice's account (private)..."
ALICE_OUTPUT=$(cargo run --release -- create-account --storage-mode private 2>&1)
echo "$ALICE_OUTPUT"
ALICE_ID=$(echo "$ALICE_OUTPUT" | grep -o "User account created: [a-f0-9]*" | awk '{print $NF}' || echo "alice_pending")
echo -e "${GREEN}✅ Alice: $ALICE_ID${NC}"
echo ""

# Create Bob (private account for another user)
echo "Creating Bob's account (private)..."
BOB_OUTPUT=$(cargo run --release -- create-account --storage-mode private 2>&1)
echo "$BOB_OUTPUT"
BOB_ID=$(echo "$BOB_OUTPUT" | grep -o "User account created: [a-f0-9]*" | awk '{print $NF}' || echo "bob_pending")
echo -e "${GREEN}✅ Bob: $BOB_ID${NC}"
echo ""

# Create Charlie (public account for testing)
echo "Creating Charlie's account (public)..."
CHARLIE_OUTPUT=$(cargo run --release -- create-account --storage-mode public 2>&1)
echo "$CHARLIE_OUTPUT"
CHARLIE_ID=$(echo "$CHARLIE_OUTPUT" | grep -o "User account created: [a-f0-9]*" | awk '{print $NF}' || echo "charlie_pending")
echo -e "${GREEN}✅ Charlie: $CHARLIE_ID${NC}"
echo ""

# Create Liquidator (public account for liquidation testing)
echo "Creating Liquidator's account (public)..."
LIQUIDATOR_OUTPUT=$(cargo run --release -- create-account --storage-mode public 2>&1)
echo "$LIQUIDATOR_OUTPUT"
LIQUIDATOR_ID=$(echo "$LIQUIDATOR_OUTPUT" | grep -o "User account created: [a-f0-9]*" | awk '{print $NF}' || echo "liquidator_pending")
echo -e "${GREEN}✅ Liquidator: $LIQUIDATOR_ID${NC}"
echo ""

# Save account IDs for testing
cat > test_accounts.json << EOF
{
  "network": "local",
  "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "accounts": {
    "alice": {
      "id": "$ALICE_ID",
      "storage_mode": "private",
      "description": "Regular user for deposit/withdraw/borrow tests"
    },
    "bob": {
      "id": "$BOB_ID",
      "storage_mode": "private",
      "description": "Second user for multi-user tests"
    },
    "charlie": {
      "id": "$CHARLIE_ID",
      "storage_mode": "public",
      "description": "Public account user for testing"
    },
    "liquidator": {
      "id": "$LIQUIDATOR_ID",
      "storage_mode": "public",
      "description": "Liquidator account for liquidation tests"
    }
  }
}
EOF

echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo -e "${GREEN}  ✅ Test Accounts Created${NC}"
echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo ""
echo "📄 Account Information:"
echo "  Alice (private):     $ALICE_ID"
echo "  Bob (private):       $BOB_ID"
echo "  Charlie (public):    $CHARLIE_ID"
echo "  Liquidator (public): $LIQUIDATOR_ID"
echo ""
echo "📁 Saved to: test_accounts.json"
echo ""
echo "Next steps:"
echo "  1. Fund accounts with test assets (when available)"
echo "  2. Run tests: cargo test --test integration_tests"
echo "  3. Manual testing:"
echo "     cargo run -- get-account-info"
echo ""
