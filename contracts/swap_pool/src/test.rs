#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address,
    Env,
};
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};


fn create_token(env: &Env, admin: &Address) -> Address {
    let token_id = env.register_stellar_asset_contract(admin.clone());
    token_id
}


fn setup_pool(env: &Env) -> (SwapPoolClient<'_>, Address, Address, Address, Address) {
    let admin = Address::generate(env);
    let user = Address::generate(env);
    // let trader = Address::generate(env);

    let token_a = create_token(env, &admin);
    let token_b = create_token(env, &admin);

    let pool_id = env.register_contract(None, SwapPool);
    let pool = SwapPoolClient::new(env, &pool_id);

    pool.initialize(&token_a, &token_b);

    (pool, token_a, token_b, user, admin)
}


#[test]
fn test_add_liquidity() {
    let env = Env::default();
    env.mock_all_auths();
    let (pool, token_a, token_b, user, admin) = setup_pool(&env);

    let token_a_admin = StellarAssetClient::new(&env, &token_a);
    let token_b_admin = StellarAssetClient::new(&env, &token_b);

    token_a_admin.mint(&user, &1000);
    token_b_admin.mint(&user, &1000);

    pool.add_liquidity(&user, &500, &500);

    let (reserve_a, reserve_b) = pool.get_reserves();
    assert_eq!(reserve_a, 500);
    assert_eq!(reserve_b, 500);

    let shares = pool.get_share(&user);
    assert!(shares > 0);
}

#[test]
fn test_remove_liquidity() {
    let env = Env::default();
    env.mock_all_auths();
    let (pool, token_a, token_b, user, admin) = setup_pool(&env);

    let token_a_admin = StellarAssetClient::new(&env, &token_a);
    let token_b_admin = StellarAssetClient::new(&env, &token_b);

    token_a_admin.mint(&user, &1000);
    token_b_admin.mint(&user, &1000);

    pool.add_liquidity(&user, &500, &500);

    let shares = pool.get_share(&user);

    pool.remove_liquidity(&user, &shares);

    let (reserve_a, reserve_b) = pool.get_reserves();
    assert_eq!(reserve_a, 0);
    assert_eq!(reserve_b, 0);

    let user_shares = pool.get_share(&user);
    assert_eq!(user_shares, 0);
}


#[test]
fn test_swap_a_for_b() {
    let env = Env::default();
    env.mock_all_auths();
    let (pool, token_a, token_b, user, admin) = setup_pool(&env);

    let token_a_admin = StellarAssetClient::new(&env, &token_a);
    let token_b_admin = StellarAssetClient::new(&env, &token_b);

    token_a_admin.mint(&user, &1000);
    token_b_admin.mint(&user, &1000);

    pool.add_liquidity(&user, &500, &500);

    let trader = Address::generate(&env);
    token_a_admin.mint(&trader, &100);

    let (reserve_a_before, reserve_b_before) = pool.get_reserves();

    let amount_out = pool.swap_a_for_b(&trader, &100);

    let (reserve_a_after, reserve_b_after) = pool.get_reserves();

    // Reserves changed correctly
    assert_eq!(reserve_a_after, reserve_a_before + 100);
    assert_eq!(reserve_b_after, reserve_b_before - amount_out);

    assert!(amount_out > 0);
}


#[test]
fn test_lp_shares_multiple_additions() {
    let env = Env::default();
    env.mock_all_auths();
    let (pool, token_a, token_b, user, admin) = setup_pool(&env);

    let user2 = Address::generate(&env);

    let token_a_admin = StellarAssetClient::new(&env, &token_a);
    let token_b_admin = StellarAssetClient::new(&env, &token_b);

    token_a_admin.mint(&user, &1000);
    token_b_admin.mint(&user, &1000);

    token_a_admin.mint(&user2, &1000);
    token_b_admin.mint(&user2, &1000);

    pool.add_liquidity(&user, &500, &500);

    let shares_user1 = pool.get_share(&user);

    pool.add_liquidity(&user2, &500, &500);

    let shares_user2 = pool.get_share(&user2);

    let total_shares = pool.get_total_shares();

    assert_eq!(total_shares, shares_user1 + shares_user2);
}


#[test]
fn test_invariant() {
    let env = Env::default();
    env.mock_all_auths();
    let (pool, token_a, token_b, user, admin) = setup_pool(&env);

    let token_a_admin = StellarAssetClient::new(&env, &token_a);
    let token_b_admin = StellarAssetClient::new(&env, &token_b);

    token_a_admin.mint(&user, &1000);
    token_b_admin.mint(&user, &1000);

    pool.add_liquidity(&user, &500, &500);

    let (reserve_a_before, reserve_b_before) = pool.get_reserves();
    let k_before = reserve_a_before * reserve_b_before;

    let trader = Address::generate(&env);
    token_a_admin.mint(&trader, &100);

    pool.swap_a_for_b(&trader, &100);

    let (reserve_a_after, reserve_b_after) = pool.get_reserves();
    let k_after = reserve_a_after * reserve_b_after;

    // Ensure invariant roughly holds (allow for fee)
    assert!(k_after >= k_before);
}