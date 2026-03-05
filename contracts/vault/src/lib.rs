#![no_std]

mod storage;
mod errors;
mod events;
mod interest;

use soroban_sdk::{
    contract,
    contractimpl,
    Address,
    Env
};

use storage::DataKey;
use errors::VaultError;

use soroban_sdk::token;

#[contract]
pub struct VaultManager;

#[contractimpl]
impl VaultManager {


pub fn initialize(
    env:Env,
    admin:Address,
    token_address:Address,
    base_rate:i128,
    multiplier:i128,
    collateral_factor:i128,
    liquidation_threshold:i128
){

    admin.require_auth();

    env.storage().instance().set(&DataKey::Admin,&admin);
    env.storage().instance().set(&DataKey::Token,&token_address);

    env.storage().instance().set(&DataKey::BaseRate,&base_rate);
    env.storage().instance().set(&DataKey::Multiplier,&multiplier);

    env.storage().instance().set(&DataKey::CollateralFactor,&collateral_factor);
    env.storage().instance().set(&DataKey::LiquidationThreshold,&liquidation_threshold);

    env.storage().instance().set(&DataKey::TotalDeposits,&0i128);
    env.storage().instance().set(&DataKey::TotalBorrows,&0i128);
}


pub fn deposit(
    env:Env,
    user:Address,
    amount:i128
)->Result<(),VaultError>{

    user.require_auth();

    if amount <=0{
        return Err(VaultError::InvalidAmount)
    }

    let token_addr:Address = env.storage()
        .instance()
        .get(&DataKey::Token)
        .unwrap();

    let token_client = token::Client::new(&env,&token_addr);

    token_client.transfer(
        &user,
        &env.current_contract_address(),
        &amount
    );

    let key = DataKey::UserDeposit(user.clone());

    let balance:i128 = env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(0);

    env.storage().persistent().set(
        &key,
        &(balance+amount)
    );

    let total:i128 = env.storage()
        .instance()
        .get(&DataKey::TotalDeposits)
        .unwrap_or(0);

    env.storage().instance().set(
        &DataKey::TotalDeposits,
        &(total+amount)
    );

    events::deposit(&env,user,amount);

    Ok(())
}


pub fn withdraw(
    env:Env,
    user:Address,
    amount:i128
)->Result<(),VaultError>{

    user.require_auth();

    let key = DataKey::UserDeposit(user.clone());

    let balance:i128 = env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(0);

    if balance < amount{
        return Err(VaultError::InsufficientBalance)
    }

    let borrow:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserBorrow(user.clone()))
        .unwrap_or(0);

    if borrow >0{
        let collateral_factor:i128 = env.storage()
            .instance()
            .get(&DataKey::CollateralFactor)
            .unwrap();

        let remaining = balance-amount;

        let max_borrow = remaining*collateral_factor/100;

        if borrow>max_borrow{
            return Err(VaultError::HealthFactorTooLow)
        }
    }

    let token_addr:Address = env.storage()
        .instance()
        .get(&DataKey::Token)
        .unwrap();

    let token_client = token::Client::new(&env,&token_addr);

    token_client.transfer(
        &env.current_contract_address(),
        &user,
        &amount
    );

    env.storage().persistent().set(
        &key,
        &(balance-amount)
    );

    events::withdraw(&env,user,amount);

    Ok(())
}

pub fn borrow(
    env:Env,
    user:Address,
    amount:i128
)->Result<(),VaultError>{

    user.require_auth();

    let deposit:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserDeposit(user.clone()))
        .unwrap_or(0);

    let collateral_factor:i128 = env.storage()
        .instance()
        .get(&DataKey::CollateralFactor)
        .unwrap();

    let max_borrow = deposit*collateral_factor/100;

    let borrowed:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserBorrow(user.clone()))
        .unwrap_or(0);

    if borrowed+amount > max_borrow{
        return Err(VaultError::InsufficientCollateral)
    }

    let token_addr:Address = env.storage()
        .instance()
        .get(&DataKey::Token)
        .unwrap();

    let token_client = token::Client::new(&env,&token_addr);

    token_client.transfer(
        &env.current_contract_address(),
        &user,
        &amount
    );

    env.storage().persistent().set(
        &DataKey::UserBorrow(user.clone()),
        &(borrowed+amount)
    );

    events::borrow(&env,user,amount);

    Ok(())
}

pub fn repay(
    env:Env,
    user:Address,
    amount:i128
)->Result<(),VaultError>{

    user.require_auth();

    let borrowed:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserBorrow(user.clone()))
        .unwrap_or(0);

    if amount>borrowed{
        return Err(VaultError::InvalidAmount)
    }

    let token_addr:Address = env.storage()
        .instance()
        .get(&DataKey::Token)
        .unwrap();

    let token_client = token::Client::new(&env,&token_addr);

    token_client.transfer(
        &user,
        &env.current_contract_address(),
        &amount
    );

    env.storage().persistent().set(
        &DataKey::UserBorrow(user.clone()),
        &(borrowed-amount)
    );

    events::repay(&env,user,amount);

    Ok(())
}


pub fn liquidate(
    env:Env,
    user:Address,
    liquidator:Address
){

    liquidator.require_auth();

    let borrow:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserBorrow(user.clone()))
        .unwrap_or(0);

    let deposit:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserDeposit(user.clone()))
        .unwrap_or(0);

    let threshold:i128 = env.storage()
        .instance()
        .get(&DataKey::LiquidationThreshold)
        .unwrap();

    let max = deposit*threshold/100;

    if borrow <= max{
        panic!("Position healthy")
    }

    env.storage().persistent().set(
        &DataKey::UserDeposit(user.clone()),
        &0i128
    );

    env.storage().persistent().set(
        &DataKey::UserBorrow(user.clone()),
        &0i128
    );

    events::liquidate(&env,user,liquidator);
}


pub fn health_factor(
    env:Env,
    user:Address
)->i128{

    let deposit:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserDeposit(user.clone()))
        .unwrap_or(0);

    let borrow:i128 = env.storage()
        .persistent()
        .get(&DataKey::UserBorrow(user.clone()))
        .unwrap_or(0);

    if borrow==0{
        return 1000
    }

    deposit*100/borrow
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

#[cfg(test)]
mod test;