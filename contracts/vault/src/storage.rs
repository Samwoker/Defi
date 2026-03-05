use soroban_sdk::{contracttype, Address};
#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    Admin,
    TotalDeposits,
    TotalBorrows,
    BaseRate,
    Multiplier,
    CollateralFactor,
    UserDeposit(Address),
    UserBorrow(Address),
    LastAccrualLedger,
}
