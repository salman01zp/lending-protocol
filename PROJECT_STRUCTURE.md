# Miden Lending Protocol - Project Structure

## Overview
This is a decentralized lending and borrowing protocol built for the Miden blockchain using Miden Assembly (MASM) and Rust.

## Directory Structure

```
lending-protocol/
├── accounts/                      # MASM account contracts
│   ├── lending_pool.masm         # Main liquidity pool account
│   ├── user_lending.masm         # User lending account template
│   └── price_oracle.masm         # Price oracle account
│
├── modules/                       # Reusable MASM modules
│   └── interest_rate.masm        # Interest rate calculation module
│
├── notes/                         # Note scripts (to be implemented)
│   ├── deposit_note.masm         # Deposit asset note script
│   ├── withdraw_note.masm        # Withdraw asset note script
│   ├── borrow_note.masm          # Borrow request note script
│   └── liquidation_note.masm     # Liquidation note script
│
├── client/                        # Rust client application
│   ├── src/
│   │   └── main.rs               # Main client entry point
│   ├── Cargo.toml                # Rust dependencies
│   └── README.md                 # Client documentation
│
├── tests/                         # Test files
│   ├── integration_tests.rs      # Integration tests
│   └── unit_tests.rs             # Unit tests
│
├── scripts/                       # Deployment and utility scripts
│   └── deploy.rs                 # Deployment script
│
└── docs/                          # Documentation
    └── ARCHITECTURE.md           # Architecture documentation
```

## Core Components

### 1. Accounts (Smart Contracts)

#### LendingPool Account (`lending_pool.masm`)
- **Purpose**: Main liquidity pool that manages deposits, withdrawals, and tracks reserves
- **Features**:
  - Deposit/withdraw functionality
  - Interest rate updates
  - Reserve data management for multiple assets (USDC, DAI, WETH)
  - Utilization tracking

#### User Lending Account (`user_lending.masm`)
- **Purpose**: User's personal account for managing collateral and debt
- **Features**:
  - Supply/withdraw collateral
  - Record borrows and repayments
  - Health factor calculation
  - Local asset storage

#### Price Oracle Account (`price_oracle.masm`)
- **Purpose**: Stores and manages asset prices
- **Features**:
  - Get/update asset prices
  - Price timestamp tracking
  - Multi-asset price queries
  - 8-decimal precision (Chainlink-style)

### 2. Modules

#### Interest Rate Strategy (`interest_rate.masm`)
- **Purpose**: Calculate supply and borrow rates based on utilization
- **Model**: Two-slope interest rate model (similar to Aave)
- **Parameters**:
  - Optimal utilization: 90% (for stablecoins)
  - Slope 1: 4%
  - Slope 2: 60%
  - Reserve factor: 10%

### 3. Client (Rust)

The Rust client provides an interface to interact with the Miden lending protocol:
- Account creation and management
- Transaction building and execution
- Note creation and consumption
- Query functions for account data

## Asset Support

Currently configured for:
- **USDC** (Asset ID: 1) - Stablecoin
- **DAI** (Asset ID: 2) - Stablecoin
- **WETH** (Asset ID: 3) - Wrapped Ether
- **WBTC** (Asset ID: 4) - Wrapped Bitcoin (oracle only)

## Storage Architecture

### LendingPool Storage Slots
- 0-6: USDC reserve (liquidity, borrowed, rates, indices)
- 7-12: DAI reserve
- 13-18: WETH reserve

### User Account Storage Slots
- 0-2: Collateral (USDC, DAI, WETH)
- 3-5: Debt (USDC, DAI, WETH)
- 6: Health factor

### Oracle Storage Slots
- 0-3: Asset prices
- 4-7: Price timestamps

## Key Features

### 1. Account Abstraction
All users deploy their own lending account (smart contract) that manages their positions locally.

### 2. Note-Based Transfers
Assets are transferred between accounts using Miden's note system:
- Deposit: User → Pool (via deposit note)
- Borrow: Pool → User (via borrow note)
- Repay: User → Pool (via repayment note)

### 3. Privacy-First
- Private accounts: Off-chain state, on-chain commitments
- Public accounts: Full on-chain state (optional)

### 4. ZK Proofs
- Client-side transaction proving
- Health factor verification via ZK proofs
- Lower on-chain costs

## Development Status

### ✅ Completed
- [x] Project structure setup
- [x] Rust environment configuration
- [x] LendingPool MASM account
- [x] User Lending MASM account
- [x] Price Oracle MASM account
- [x] Interest Rate Strategy module

### 🚧 In Progress
- [ ] Note scripts (deposit, withdraw, borrow, liquidation)
- [ ] Rust client implementation
- [ ] Testing framework

### 📋 Todo
- [ ] Borrow/repay functionality
- [ ] Liquidation mechanism
- [ ] Integration tests
- [ ] Miden testnet deployment

## Getting Started

### Prerequisites
- Rust 1.87.0+
- Miden 0.9.0+
- Cargo

### Build
```bash
cd client
cargo build
```

### Run Tests
```bash
cargo test
```

### Deploy
```bash
# Deploy to Miden testnet
cargo run --bin deploy
```

## Configuration

### Interest Rate Parameters
Defined in `modules/interest_rate.masm`:
- Optimal Utilization: 90%
- Base Rate: 0%
- Slope 1: 4%
- Slope 2: 60%

### Liquidation Parameters
Defined in `accounts/user_lending.masm`:
- Liquidation Threshold: 85%
- Health Factor Requirement: ≥ 1.0

## Resources

- [Miden Documentation](https://0xmiden.github.io/miden-docs/)
- [Miden Assembly Reference](https://0xpolygonmiden.github.io/miden-vm/)
- [Miden Client Tutorial](https://0xmiden.github.io/miden-docs/imported/miden-tutorials/)

## License

MIT
