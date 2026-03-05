#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address,
    Env
};

use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};


fn create_token_contract(e: &Env, admin: &Address) -> Address {

    let token_id = e.register_stellar_asset_contract(admin.clone());
    token_id
}

fn create_vault(e: &Env) -> (Address, VaultManagerClient<'_>, Address, Address, Address, Address) {

    let admin = Address::generate(e);
    let user = Address::generate(e);
    let liquidator = Address::generate(e);

    let token_addr = create_token_contract(e, &admin);

    let vault_id = e.register_contract(None, VaultManager);

    let vault = VaultManagerClient::new(e, &vault_id);

    vault.initialize(
        &admin,
        &token_addr,
        &5,    // base rate
        &10,   // multiplier
        &90,   // collateral factor
        &80,   // liquidation threshold
    );

    (vault_id, vault, user, admin, liquidator, token_addr)
}


#[test]
fn test_deposit() {

    let env = Env::default();
    env.mock_all_auths();

    let (_id, vault, user, _admin, _, token_addr) = create_vault(&env);

    let token_admin = StellarAssetClient::new(&env, &token_addr);

    token_admin.mint(&user, &1000);

    vault.deposit(&user, &500);

    let balance = vault.get_deposit(&user);

    assert_eq!(balance, 500);
}


#[test]
fn test_borrow() {

    let env = Env::default();
    env.mock_all_auths();

    let (_id, vault, user, _admin, _, token_addr) = create_vault(&env);

    let token_admin = StellarAssetClient::new(&env, &token_addr);

    token_admin.mint(&user, &1000);

    vault.deposit(&user, &800);

    vault.borrow(&user, &300);

    let borrowed = vault.get_borrow(&user);

    assert_eq!(borrowed, 300);
}


#[test]
fn test_repay() {

    let env = Env::default();
    env.mock_all_auths();

    let (_id, vault, user, _admin, _, token_addr) = create_vault(&env);

    let token_admin = StellarAssetClient::new(&env, &token_addr);

    token_admin.mint(&user, &1000);

    vault.deposit(&user, &800);

    vault.borrow(&user, &200);

    vault.repay(&user, &100);

    let remaining = vault.get_borrow(&user);

    assert_eq!(remaining, 100);
}


#[test]
fn test_withdraw() {

    let env = Env::default();
    env.mock_all_auths();

    let (_id, vault, user, _admin, _, token_addr) = create_vault(&env);

    let token_admin = StellarAssetClient::new(&env, &token_addr);

    token_admin.mint(&user, &1000);

    vault.deposit(&user, &600);

    vault.withdraw(&user, &200);

    let balance = vault.get_deposit(&user);

    assert_eq!(balance, 400);
}


#[test]
fn test_health_factor() {

    let env = Env::default();
    env.mock_all_auths();

    let (_id, vault, user, _admin, _, token_addr) = create_vault(&env);

    let token_admin = StellarAssetClient::new(&env, &token_addr);

    token_admin.mint(&user, &1000);

    vault.deposit(&user, &1000);

    vault.borrow(&user, &500);

    let health = vault.health_factor(&user);

    assert!(health > 100);
}


#[test]
fn test_liquidation() {

    let env = Env::default();
    env.mock_all_auths();

    let (_id, vault, user, _admin, liquidator, token_addr) = create_vault(&env);

    let token_admin = StellarAssetClient::new(&env, &token_addr);

    token_admin.mint(&user, &1000);

    vault.deposit(&user, &1000);

    vault.borrow(&user, &850);

    vault.liquidate(&user, &liquidator);

    let deposit = vault.get_deposit(&user);
    let borrow = vault.get_borrow(&user);

    assert_eq!(deposit, 0);
    assert_eq!(borrow, 0);
}