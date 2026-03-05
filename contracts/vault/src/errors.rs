use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]

pub enum VaultError {
    Unauthorized = 1,
    InsufficientBalance = 2,
    InsufficientCollateral = 3,
    MathOverflow = 4,
    InvalidAmount = 5,
}
