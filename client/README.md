# Miden Lending Protocol - Rust Client

A command-line interface for interacting with the Miden Lending Protocol.

## Features

- 🏦 Deploy and manage lending pool accounts
- 💰 Deposit and withdraw assets
- 🔒 Supply collateral and borrow assets
- 📊 Query reserve data and account information
- 💹 Manage price oracle
- 🏥 Calculate health factors

## Installation

```bash
cd client
cargo build --release
```

## Configuration

The client connects to a Miden node via RPC. Default endpoint: `http://localhost:57291`

Initialize the protocol:
```bash
cargo run -- init --rpc http://localhost:57291
```

## Usage

### Account Management

Create a user lending account:
```bash
# Private account (default)
cargo run -- create-account

# Public account
cargo run -- create-account --storage-mode public
```

Deploy protocol contracts:
```bash
# Deploy lending pool
cargo run -- deploy-pool

# Deploy price oracle
cargo run -- deploy-oracle
```

### Lending Operations

Deposit assets to the pool:
```bash
cargo run -- deposit --asset-id 1 --amount 1000
# Asset IDs: 1=USDC, 2=DAI, 3=WETH
```

Withdraw assets:
```bash
cargo run -- withdraw --asset-id 1 --amount 500
```

### Borrowing Operations

Supply collateral:
```bash
cargo run -- supply-collateral --asset-id 3 --amount 1
```

Borrow assets:
```bash
cargo run -- borrow --asset-id 1 --amount 1000
```

Repay debt:
```bash
cargo run -- repay --asset-id 1 --amount 500
```

### Queries

Get account information:
```bash
cargo run -- get-account-info
```

Get reserve data:
```bash
cargo run -- get-reserve-data --asset-id 1
```

Get asset price:
```bash
cargo run -- get-price --asset-id 3
```

Calculate health factor:
```bash
cargo run -- health-factor
```

### Oracle Management (Admin)

Update asset price:
```bash
cargo run -- update-price --asset-id 1 --price 100000000
# Price with 8 decimals: 100000000 = $1.00
```

## Asset IDs

- `1` - USDC (Stablecoin)
- `2` - DAI (Stablecoin)
- `3` - WETH (Wrapped Ether)
- `4` - WBTC (Wrapped Bitcoin)

## Development Status

### ✅ Completed
- [x] CLI structure
- [x] Command definitions
- [x] Module scaffolding

### 🚧 In Progress
- [ ] Miden client integration
- [ ] Account creation implementation
- [ ] Transaction builders

### 📋 Todo
- [ ] Note script integration
- [ ] Full transaction execution
- [ ] Error handling
- [ ] Integration tests

## Architecture

```
client/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── config.rs        # Configuration management
│   ├── accounts.rs      # Account creation and deployment
│   ├── transactions.rs  # Transaction building and execution
│   └── utils.rs         # Helper functions
└── Cargo.toml
```

## Dependencies

- `miden-client` - Miden blockchain client
- `miden-lib` - Miden base libraries
- `miden-objects` - Core Miden objects
- `miden-tx` - Transaction utilities
- `tokio` - Async runtime
- `clap` - CLI parsing
- `tracing` - Logging

## Examples

### Complete Flow

1. Initialize and deploy:
```bash
cargo run -- init
cargo run -- deploy-pool
cargo run -- deploy-oracle
cargo run -- create-account
```

2. Supply collateral and borrow:
```bash
cargo run -- supply-collateral --asset-id 3 --amount 1
cargo run -- borrow --asset-id 1 --amount 1000
cargo run -- health-factor
```

3. Repay and withdraw:
```bash
cargo run -- repay --asset-id 1 --amount 1000
cargo run -- withdraw-collateral --asset-id 3 --amount 1
```

## Notes

- All amounts are in the smallest unit of the asset
- Prices use 8-decimal precision (Chainlink format)
- Health factor must be ≥ 1.0 to avoid liquidation
- Client-side proving reduces on-chain costs

## Resources

- [Miden Documentation](https://0xmiden.github.io/miden-docs/)
- [Miden Client Tutorial](https://0xmiden.github.io/miden-docs/imported/miden-tutorials/)
