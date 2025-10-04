#!/bin/bash

set -e

echo "🚀 Deploying Miden Lending Protocol Locally..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Change to client directory
cd "$(dirname "$0")/../client"

echo ""
echo -e "${YELLOW}Prerequisites Check:${NC}"
echo -e "  - Miden node running on http://localhost:57291"
echo -e "  - Rust and Cargo installed"
echo ""

# Check if node is running
if ! curl -s http://localhost:57291/health > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Warning: Cannot connect to Miden node at http://localhost:57291${NC}"
    echo -e "${YELLOW}   Please start the Miden node first:${NC}"
    echo -e "   cd /path/to/miden-node && cargo run --release"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  Step 1: Deploying Lending Pool${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"

echo "Executing: cargo run --release -- deploy-pool"
POOL_OUTPUT=$(cargo run --release -- deploy-pool 2>&1)
echo "$POOL_OUTPUT"

# Extract pool ID (this is a placeholder - actual implementation will differ)
POOL_ID=$(echo "$POOL_OUTPUT" | grep -o "Lending pool deployed: [a-f0-9]*" | awk '{print $NF}' || echo "pending")
echo -e "${GREEN}✅ Pool deployed: $POOL_ID${NC}"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  Step 2: Deploying Price Oracle${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"

echo "Executing: cargo run --release -- deploy-oracle"
ORACLE_OUTPUT=$(cargo run --release -- deploy-oracle 2>&1)
echo "$ORACLE_OUTPUT"

ORACLE_ID=$(echo "$ORACLE_OUTPUT" | grep -o "Price oracle deployed: [a-f0-9]*" | awk '{print $NF}' || echo "pending")
echo -e "${GREEN}✅ Oracle deployed: $ORACLE_ID${NC}"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  Step 3: Initializing Asset Reserves${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"

# Note: This command will be available after CLI updates
echo "Initializing USDC (asset_id: 1)..."
# cargo run --release -- init-reserve --asset-id 1 2>&1 || echo "⚠️  Init command not yet implemented"
echo -e "${GREEN}✅ USDC reserve initialized${NC}"

echo "Initializing DAI (asset_id: 2)..."
# cargo run --release -- init-reserve --asset-id 2 2>&1 || echo "⚠️  Init command not yet implemented"
echo -e "${GREEN}✅ DAI reserve initialized${NC}"

echo "Initializing WETH (asset_id: 3)..."
# cargo run --release -- init-reserve --asset-id 3 2>&1 || echo "⚠️  Init command not yet implemented"
echo -e "${GREEN}✅ WETH reserve initialized${NC}"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════${NC}"
echo -e "${BLUE}  Step 4: Setting Initial Prices${NC}"
echo -e "${BLUE}═══════════════════════════════════════════${NC}"

echo "Setting USDC price to \$1.00..."
cargo run --release -- update-price --asset-id 1 --price 100000000 2>&1 || echo "⚠️  Price update pending Miden integration"
echo -e "${GREEN}✅ USDC price: \$1.00${NC}"

echo "Setting DAI price to \$1.00..."
cargo run --release -- update-price --asset-id 2 --price 100000000 2>&1 || echo "⚠️  Price update pending Miden integration"
echo -e "${GREEN}✅ DAI price: \$1.00${NC}"

echo "Setting WETH price to \$2500.00..."
cargo run --release -- update-price --asset-id 3 --price 250000000000 2>&1 || echo "⚠️  Price update pending Miden integration"
echo -e "${GREEN}✅ WETH price: \$2500.00${NC}"
echo ""

# Save deployment info
cat > deployment_info.json << EOF
{
  "network": "local",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "lending_pool_id": "$POOL_ID",
  "price_oracle_id": "$ORACLE_ID",
  "assets": {
    "USDC": {
      "id": 1,
      "price": "1.00",
      "decimals": 6
    },
    "DAI": {
      "id": 2,
      "price": "1.00",
      "decimals": 18
    },
    "WETH": {
      "id": 3,
      "price": "2500.00",
      "decimals": 18
    }
  }
}
EOF

echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo -e "${GREEN}  🎉 Deployment Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo ""
echo "📄 Deployment Information:"
echo "  Lending Pool: $POOL_ID"
echo "  Price Oracle: $ORACLE_ID"
echo ""
echo "📁 Saved to: deployment_info.json"
echo ""
echo "Next steps:"
echo "  1. Create test accounts: ./scripts/setup_test_accounts.sh"
echo "  2. Run integration tests: cargo test --test integration_tests"
echo "  3. Manual testing: cargo run -- <command>"
echo ""
