# Soroban DeFi Project

A comprehensive collection of DeFi protocols built on the Soroban smart contract platform. This workspace contains modules for lending, automated market making (AMM), and decentralized governance.

## Contracts Overview

### 1. DeFi Vault (`contracts/vault`)
A collateralized lending and borrowing protocol.
- **Core Logic**: Manage user deposits, collateralized loans, and interest accrual.
- **Safety**: Automated liquidation mechanisms based on health factor thresholds.

### 2. Swap Pool (`contracts/swap_pool`)
A constant product AMM (Automated Market Maker).
- **Liquidity**: Add/Remove liquidity with proportional LP share emission.
- **Swaps**: Swap Token A for Token B with a built-in 0.3% fee model.

### 3. Governance (`contracts/governance`)
A decentralized proposal and voting system.
- **Democracy**: Proposal creation with configurable quorum and voting power snapshots.
- **Automation**: On-chain execution of passed proposals.

---

## Detailed Project Structure

```text
.
├── contracts
│   ├── vault               # --- Lending & Borrowing Module ---
│   │   ├── src
│   │   │   ├── lib.rs      # Entry points (deposit, borrow, liquidate)
│   │   │   ├── storage.rs  # Persistence keys (UserDeposit, TotalBorrows)
│   │   │   ├── interest.rs # Dynamic interest rate modeling
│   │   │   ├── events.rs   # Standardized event emissions
│   │   │   ├── errors.rs   # Protocol-specific error codes
│   │   │   └── test.rs     # Integration and unit tests
│   │   └── Cargo.toml      # Contract configuration
│   │
│   ├── swap_pool           # --- AMM Swap Module ---
│   │   ├── src
│   │   │   ├── lib.rs      # Swap logic, liquidity management
│   │   │   ├── storage.rs  # Reserve and Share storage keys
│   │   │   ├── math.rs     # Square root and price impact calculations
│   │   │   └── test.rs     # Constant product invariant tests
│   │   └── Cargo.toml      # Contract configuration
│   │
│   └── governance          # --- Governance Module ---
│       ├── src
│       │   ├── lib.rs      # Proposal lifecycle and voting logic
│       │   ├── storage.rs  # Proposal structs and snapshot keys
│       │   └── test.rs     # Quorum and execution flow tests
│       └── Cargo.toml      # Contract configuration
│
├── Cargo.toml              # Workspace dependency management
└── README.md               # Main project documentation
```

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (target `wasm32-unknown-unknown`)
- [Stellar CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup#install-the-stellar-cli)

### Build
To build all contracts, ensure you have the `wasm32-unknown-unknown` target installed:
```bash
rustup target add wasm32-unknown-unknown
```

Then build from the project root:
```bash
stellar contract build
```

### Test
Run the full test suite (15 passing tests expected):
```bash
cargo test
```

> [!TIP]
> Each contract also has its own `src/test.rs` which can be executed individually by navigating to the contract directory and running `cargo test`.
