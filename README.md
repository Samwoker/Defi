# Soroban DeFi Vault

A collateralized lending and borrowing vault built on the Soroban smart contract platform. This project allows users to deposit tokens as collateral and borrow against them, with liquidation mechanisms ensuring system solvency.

## Features

- **Deposits & Withdrawals**: Securely deposit tokens to earn potential interest or build collateral.
- **Collateralized Borrowing**: Borrow tokens against your deposited collateral based on a configurable collateral factor.
- **Liquidation**: Automated liquidation threshold to protect the protocol from bad debt.
- **Health Factor**: Real-time health factor calculation for user positions.
- **Interest Rates**: Configurable interest rate model with base rate and multiplier (logic implemented in `interest.rs`).

## Project Structure

```text
.
├── contracts
│   └── vault
│       ├── src
│       │   ├── lib.rs        # Main contract logic and entry points
│       │   ├── storage.rs    # Data storage keys and structures
│       │   ├── interest.rs   # Interest rate calculation logic
│       │   ├── events.rs     # Event emission definitions
│       │   ├── errors.rs     # Custom error types
│       │   └── test.rs       # Contract tests
│       ├── Cargo.toml        # Contract-specific configuration
│       └── Makefile          # Build and test shortcuts
├── Cargo.toml                # Workspace configuration
└── README.md                 # Project documentation
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Stellar CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup#install-the-stellar-cli)

### Build

To build the contract, run:

```bash
cd contracts/vault
cargo build --target thumv7em-none-eabihf --release
```

Or using the Makefile:

```bash
cd contracts/vault
make build
```

### Test

To run the automated tests:

```bash
cd contracts/vault
cargo test
```

## Contract Initialization

The vault needs to be initialized with several parameters:

- **Admin**: The address with administrative privileges.
- **Token Address**: The address of the Stellar Asset to be used in the vault.
- **Base Rate**: The starting interest rate.
- **Multiplier**: The rate at which interest increases with utilization.
- **Collateral Factor**: Percentage of deposit that can be borrowed (e.g., 75 for 75%).
- **Liquidation Threshold**: Percentage at which a position becomes eligible for liquidation (e.g., 85 for 85%).
