#![no_std]
mod errors;
mod events;
mod storage;

use soroban_sdk::{contract, contractimpl, Address, Env};

use errors::VaultError;
use storage::DataKey;

#[contract]

pub struct VaultManager;

#[contractimpl]

impl VaultManager {
    pub fn initialize(
        env: Env,
        admin: Address,
        base_rate: i128,
        multiplier: i128,
        collateral_factor: i128,
    ) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::BaseRate, &base_rate);
        env.storage()
            .instance()
            .set(&DataKey::Multiplier, &multiplier);
        env.storage()
            .instance()
            .set(&DataKey::CollateralFactor, &collateral_factor);
        env.storage()
            .instance()
            .set(&DataKey::TotalDeposits, &0i128);
        env.storage().instance().set(&DataKey::TotalBorrows, &0i128);
    }

    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();
        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let key = DataKey::UserDeposit(user.clone());
        let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        let new_balance = balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;
        env.storage().persistent().set(&key, &new_balance);

        let total: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalDeposits)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalDeposits, &(total + amount));
        events::deposit(&env, user, amount);
        Ok(())
    }

    pub fn withdraw(env: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();
        let key = DataKey::UserDeposit(user.clone());
        let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if balance < amount {
            return Err(VaultError::InsufficientBalance);
        }
        let new_balance = balance - amount;
        env.storage().persistent().set(&key, &new_balance);
        Ok(())
    }

    pub fn borrow(env: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();

        let deposit = env
            .storage()
            .persistent()
            .get(&DataKey::UserDeposit(user.clone()))
            .unwrap_or(0);
        let collateral_factor = env
            .storage()
            .persistent()
            .get(&DataKey::CollateralFactor)
            .unwrap_or(0);
        let max_borrow = deposit * collateral_factor / 100;

        let current_borrow = env
            .storage()
            .persistent()
            .get(&DataKey::UserBorrow(user.clone()))
            .unwrap_or(0);
        if current_borrow + amount > max_borrow {
            return Err(VaultError::InsufficientCollateral);
        }
        env.storage().persistent().set(
            &DataKey::UserBorrow(user.clone()),
            &(current_borrow + amount),
        );
        events::borrow(&env, user, amount);
        Ok(())
    }

    pub fn repay(env: Env, user: Address, amount: i128) -> Result<(), VaultError> {
        user.require_auth();

        let borrowed: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserBorrow(user.clone()))
            .unwrap_or(0);
        if borrowed < amount {
            return Err(VaultError::InvalidAmount);
        }

        env.storage()
            .persistent()
            .set(&DataKey::UserBorrow(user.clone()), &(borrowed - amount));
        events::repay(&env, user, amount);

        Ok(())
    }

    pub fn get_deposit(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserDeposit(user))
            .unwrap_or(0)
    }
    pub fn get_borrow(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserBorrow(user))
            .unwrap_or(0)
    }
}
