use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum VaultError {

    Unauthorized = 1,

    InsufficientBalance = 2,

    InsufficientCollateral = 3,

    HealthFactorTooLow = 4,

    InvalidAmount = 5,

    MathOverflow = 6,
}