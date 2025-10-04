# lending-borrowing-protocol

Ethereum Lending & Borrowing Protocol - Complete Task Description
Project Overview
Build a production-ready decentralized lending and borrowing protocol on Ethereum mainnet. Users can supply ERC20 tokens to earn interest and borrow assets against their collateral with dynamic interest rates and liquidation mechanisms.
Tech Stack Requirements

Solidity Version: ^0.8.20
Framework: Foundry
Libraries: OpenZeppelin Contracts (ERC20, Ownable, ReentrancyGuard, SafeERC20)
Testing: Foundry + Chai for comprehensive test coverage
Network: Ethereum Mainnet (start with testnet deployment)


Smart Contract Architecture
1. LendingPool.sol (Main Protocol Contract)
Core Functionality:
SUPPLY OPERATIONS:
- deposit(address asset, uint256 amount)
  → Transfer tokens from user to pool
  → Mint interest-bearing tokens (aTokens) to user
  → Update reserve data and interest rates
  
- withdraw(address asset, uint256 amount)
  → Burn aTokens from user
  → Check if withdrawal doesn't break health factor
  → Transfer underlying tokens to user
  → Update reserve data and interest rates

BORROW OPERATIONS:
- borrow(address asset, uint256 amount, uint256 interestRateMode)
  → interestRateMode: 1=stable, 2=variable
  → Check borrowing power and health factor
  → Mint debt tokens to user
  → Transfer borrowed tokens to user
  → Update reserve data and interest rates
  
- repay(address asset, uint256 amount)
  → Transfer repayment from user
  → Burn debt tokens
  → Update reserve data and interest rates

LIQUIDATION:
- liquidate(address user, address collateralAsset, address debtAsset, uint256 debtToCover)
  → Check if user is undercollateralized (health factor < 1)
  → Calculate liquidation bonus (e.g., 5-10%)
  → Burn debt tokens
  → Transfer collateral to liquidator with bonus
  → Emit liquidation event
State Variables:

mapping(address => ReserveData) reserves - Asset reserve data
mapping(address => UserAccountData) userAccounts - User positions
address[] reservesList - List of supported assets
IInterestRateStrategy interestRateStrategy - Rate calculation contract
IPriceOracle priceOracle - Price feed for liquidations

Key Structs:
soliditystruct ReserveData {
    uint256 liquidityIndex;          // Cumulative liquidity index
    uint256 variableBorrowIndex;     // Cumulative variable borrow index
    uint256 currentLiquidityRate;    // Current supply APY
    uint256 currentVariableBorrowRate; // Current borrow APY
    uint256 lastUpdateTimestamp;     // Last rate update time
    address aTokenAddress;           // Interest-bearing token
    address variableDebtTokenAddress; // Variable debt token
    address stableDebtTokenAddress;  // Stable debt token (optional)
    uint16 liquidationThreshold;     // E.g., 8000 = 80%
    uint16 liquidationBonus;         // E.g., 10500 = 5% bonus
    uint16 reserveFactor;            // Protocol fee (e.g., 1000 = 10%)
    bool isActive;
    bool isFrozen;
}

struct UserAccountData {
    uint256 totalCollateralETH;
    uint256 totalDebtETH;
    uint256 availableBorrowsETH;
    uint256 currentLiquidationThreshold;
    uint256 ltv;                     // Loan-to-value ratio
    uint256 healthFactor;
}

2. AToken.sol (Interest-Bearing Token)
Purpose: ERC20 token representing supplied assets that automatically accrues interest
Key Features:

Inherits ERC20
balanceOf() returns principal + accrued interest
mint(address user, uint256 amount) - Only callable by LendingPool
burn(address user, uint256 amount) - Only callable by LendingPool
Auto-compounding interest calculation
Scalable balance implementation (uses index-based calculation)

Formula:
actualBalance = principalBalance * currentLiquidityIndex / userLiquidityIndex

3. DebtToken.sol (Variable & Stable Debt Tokens)
Purpose: Track borrowed amounts with accruing interest
VariableDebtToken.sol:

Interest rate changes with utilization
balanceOf() returns principal + accrued interest
mint() and burn() only by LendingPool
Index-based balance calculation

StableDebtToken.sol (Optional for v1):

Fixed interest rate at time of borrowing
More complex implementation
Consider implementing in v2


4. InterestRateStrategy.sol
Purpose: Calculate supply and borrow rates based on utilization
Algorithm (Two-Slope Model):
Utilization Rate (U) = Total Borrows / Total Liquidity

IF U < OPTIMAL_UTILIZATION (e.g., 80%):
  variableBorrowRate = baseRate + (U / optimalU) * slope1
  
IF U >= OPTIMAL_UTILIZATION:
  variableBorrowRate = baseRate + slope1 + ((U - optimalU) / (1 - optimalU)) * slope2

supplyRate = utilizationRate * variableBorrowRate * (1 - reserveFactor)
Key Functions:

calculateInterestRates(address reserve, uint256 availableLiquidity, uint256 totalVariableDebt, uint256 totalStableDebt, uint256 reserveFactor)
Returns: (liquidityRate, variableBorrowRate, stableBorrowRate)

Parameters per Asset:

Base rate: 0-2%
Slope 1: 4-7%
Slope 2: 60-300%
Optimal utilization: 70-90%


5. PriceOracle.sol
Purpose: Provide asset prices for collateral/debt valuation
Implementation Options:
Option A - Chainlink Integration (Recommended):

Use Chainlink Price Feeds for major assets
getAssetPrice(address asset) returns price in ETH or USD
Implement fallback mechanisms
Add price staleness checks

Option B - Custom Oracle:

Trusted oracle updater role
Time-weighted average price (TWAP)
Circuit breakers for extreme price movements

Key Functions:

getAssetPrice(address asset) returns (uint256)
getAssetsPrices(address[] assets) returns (uint256[])


6. LendingPoolConfigurator.sol
Purpose: Admin contract for protocol configuration
Functions:

initReserve(address asset, address aToken, address debtToken, address interestRateStrategy)
setReserveFactor(address asset, uint256 reserveFactor)
setLiquidationThreshold(address asset, uint256 threshold)
setLiquidationBonus(address asset, uint256 bonus)
enableBorrowingOnReserve(address asset)
freezeReserve(address asset) - Emergency pause
activateReserve(address asset)

Access Control:

Only protocol admin/governance can call
Consider Timelock for sensitive operations


7. LendingPoolAddressesProvider.sol
Purpose: Registry for all protocol contract addresses
Functions:

getLendingPool() returns (address)
setPriceOracle(address oracle)
setLendingPoolImpl(address pool) - For upgradeability
Stores addresses for: LendingPool, PriceOracle, InterestRateStrategy, Configurator


Key Calculations & Formulas
Health Factor:
healthFactor = (totalCollateralETH * liquidationThreshold) / totalDebtETH

If healthFactor < 1.0 → Position can be liquidated
Liquidation:
maxLiquidatableDebt = (totalDebt * LIQUIDATION_CLOSE_FACTOR) // e.g., 50%
collateralToSeize = (debtToCover * liquidationBonus * debtAssetPrice) / collateralAssetPrice
Interest Accrual:
index_new = index_old * (1 + rate * timeDelta / SECONDS_PER_YEAR)

Implementation Phases
Phase 1: Core Contracts (Week 1-2)

Implement LendingPool with deposit/withdraw
Create AToken with interest accrual
Implement basic InterestRateStrategy
Add ReentrancyGuard and SafeERC20

Phase 2: Borrowing (Week 2-3)

Implement borrow/repay functions
Create VariableDebtToken
Add health factor calculations
Integrate PriceOracle (Chainlink)

Phase 3: Liquidations (Week 3-4)

Implement liquidation logic
Add liquidation bonus mechanism
Create liquidation bots (off-chain)
Emergency pause functionality

Phase 4: Testing & Security (Week 4-6)

Unit tests for all contracts (>90% coverage)
Integration tests for user flows
Fork testing on mainnet data
Gas optimization
Security audit preparation


Testing Requirements
Unit Tests:

Test each function in isolation
Edge cases: zero amounts, max uint256, etc.
Access control checks
Event emissions

Integration Tests:

Full user journey: deposit → borrow → repay → withdraw
Multi-user scenarios
Liquidation scenarios with price changes
Interest accrual over time (use time manipulation)

Fuzz Testing:

Random amounts and sequences
Invariant testing (total supply = total deposits, etc.)

Fork Tests:

Test against real Ethereum state
Integrate with actual Chainlink oracles
Test with real ERC20 tokens (USDC, DAI, WETH)


Security Considerations
Critical Checks:

Reentrancy Protection: All external calls
Integer Overflow: Use Solidity 0.8+ built-in checks
Access Control: Proper modifiers on admin functions
Oracle Manipulation: Validate price freshness and deviations
Flash Loan Attacks: Consider using Checks-Effects-Interactions pattern
Rounding Errors: Always round in protocol's favor

Attack Vectors to Test:

Price oracle manipulation
Flash loan attacks to manipulate rates
Donation attacks (inflating reserve balances)
Liquidation front-running
Interest rate manipulation via utilization


Configuration Parameters (Example for USDC)
solidityUSDC Reserve Configuration:
- Optimal Utilization: 90%
- Base Variable Borrow Rate: 0%
- Variable Rate Slope 1: 4%
- Variable Rate Slope 2: 60%
- Liquidation Threshold: 85%
- Liquidation Bonus: 5%
- Reserve Factor: 10%
- LTV (Loan-to-Value): 80%

Deployment Checklist

Deploy AddressesProvider
Deploy LendingPool implementation
Deploy PriceOracle and set asset price feeds
Deploy InterestRateStrategy for each asset
Deploy AToken and DebtToken implementations
Deploy LendingPoolConfigurator
Initialize reserves for each supported asset (WETH, USDC, DAI, WBTC)
Set protocol parameters
Transfer admin to multisig/governance
Verify all contracts on Etherscan


Expected Deliverables
Smart Contracts:

 LendingPool.sol (500-800 lines)
 AToken.sol (200-300 lines)
 VariableDebtToken.sol (200-300 lines)
 InterestRateStrategy.sol (100-150 lines)
 PriceOracle.sol (100-200 lines)
 LendingPoolConfigurator.sol (300-400 lines)
 LendingPoolAddressesProvider.sol (100-150 lines)

Testing Suite:

 Minimum 50 unit tests
 20+ integration tests
 Gas optimization report
 Test coverage report (>90%)

Documentation:

 Function documentation (NatSpec)
 Architecture diagram
 User flow documentation
 Deployment guide

Scripts:

 Deployment scripts for testnet/mainnet
 Reserve initialization scripts
 Upgrade scripts (if using proxy pattern)


Gas Optimization Tips

Pack struct variables efficiently
Use uint128 instead of uint256 where possible
Cache storage variables in memory
Use unchecked blocks for safe math operations
Minimize storage writes
Batch operations where possible


Future Enhancements (Post-MVP)

Multi-chain Support: Deploy on L2s (Arbitrum, Optimism, Base)
Stable Rate Borrowing: Implement StableDebtToken
Flash Loans: Add flash loan functionality
Governance: Decentralized parameter updates
Isolation Mode: Isolated lending pools for risky assets
eMode: Efficiency mode for correlated assets (e.g., stablecoins)
Credit Delegation: Allow users to delegate borrowing power
Yield Strategies: Auto-compound strategies for aToken holders


Success Metrics

Zero critical vulnerabilities in audit
Gas cost per deposit: <100k gas
Gas cost per borrow: <150k gas
Test coverage: >90%
Support for 5+ assets at launch
Liquidation bot successfully liquidates undercollateralized positions