
use soroban_sdk::{contracttype,Address};

#[derive(Clone)]
#[contracttype]

pub enum DataKey{
    TokenA,
    TokenB,
    ReserveA,
    ReserveB,
    TotalShares,
    Share(Address)
}