
#![no_std]
mod storage;
mod math;

use soroban_sdk::{contract,contractimpl,Address,Env};
use soroban_sdk::token;
use storage::DataKey;
use math::*;


#[contract]
pub struct SwapPool;

#[contractimpl]
impl SwapPool {

    pub fn initialize(
        env:Env,
        token_a:Address,
        token_b:Address
    ){
        env.storage().instance().set(&DataKey::TokenA,&token_a);
        env.storage().instance().set(&DataKey::TokenB,&token_b);
        env.storage().instance().set(&DataKey::ReserveA, &0i128);
        env.storage().instance().set(&DataKey::ReserveB, &0i128);
        env.storage().instance().set(&DataKey::TotalShares, &0i128);
    }

    pub fn add_liquidity(
        env:Env,
        user:Address,
        amount_a:i128,
        amount_b:i128
    ){
        user.require_auth();
        let token_a:Address = env.storage().instance().get(&DataKey::TokenA).unwrap();
        let token_b:Address = env.storage().instance().get(&DataKey::TokenB).unwrap();

        let mut reserve_a:i128 = env.storage().instance().get(&DataKey::ReserveA).unwrap();
        let mut reserve_b:i128 = env.storage().instance().get(&DataKey::ReserveB).unwrap();

        let mut total_shares:i128 = env.storage().instance().get(&DataKey::TotalShares).unwrap();

        let token_a_client = token::Client::new(&env,&token_a);
        let token_b_client = token::Client::new(&env,&token_b);

        let shares;

        if total_shares == 0 {
            shares = sqrt(amount_a * amount_b);
        }else{
            let shares_a = amount_a * total_shares / reserve_a;
            let shares_b = amount_b * total_shares /reserve_b;
            shares = if shares_a < shares_b {
                shares_a
            }else{
                shares_b
            };
        }

        let user_key = DataKey::Share(user.clone());
        let user_share:i128 = env.storage().persistent().get(&user_key).unwrap_or(0);
        env.storage().persistent().set(&user_key, &(user_share + shares));
        total_shares += shares;
        reserve_a +=amount_a;
        reserve_b +=amount_b;

        env.storage().instance().set(&DataKey::TotalShares,&total_shares);
        env.storage().instance().set(&DataKey::ReserveA,&reserve_a);
        env.storage().instance().set(&DataKey::ReserveB,&reserve_b);
        
    }

    pub fn remove_liquidity(
        env:Env,user:Address,shares:i128
    ){
        user.require_auth();
        let token_a:Address = env.storage().instance().get(&DataKey::TokenA).unwrap();
        let token_b:Address = env.storage().instance().get(&DataKey::TokenB).unwrap();

        let mut reserve_a:i128 = env.storage().instance().get(&DataKey::ReserveA).unwrap();
        let mut reserve_b:i128 = env.storage().instance().get(&DataKey::ReserveB).unwrap();

        let mut total_shares:i128 = env.storage().instance().get(&DataKey::TotalShares).unwrap();

        let user_key = DataKey::Share(user.clone());
        let user_share:i128 = env.storage().persistent().get(&user_key).unwrap_or(0);

        if shares > user_share {
            panic!("not enough shares")
        }
        let amount_a = shares * reserve_a / total_shares;
        let amount_b = shares * reserve_b / total_shares;

        let token_a_client = token::Client::new(&env,&token_a);
        let token_b_client = token::Client::new(&env,&token_b);

        token_a_client.transfer(&env.current_contract_address(),&user,amount_a);
        token_b_client.transfer(&env.current_contract_address(),&user,amount_b);

        env.storage().persistent().set(&user_key,&(user_share - shares));

        env.storage().instance().set(&DataKey::ReserveA ,&(reserve_a - amount_a));
        env.storage().instance().set(&DataKey::ReserveB, &(reserve_b - amount_b));
        
    }

    pub fn swap_a_for_b(env:Env,user:Address,amount_in:i128) -> i128{
        user_require_auth();
        let token_a:Address = env.storage().instance().get(&DataKey::TokenA).unwrap();
        let token_b:Address = env.storage().instance().get(&DataKey::TokenB).unwrap();

        let mut reserve_a:i128 = env.storage().instance().get(&DataKey::ReserveA).unwrap();
        let mut reserve_b:i128 = env.storage().instance().get(&DataKey::ReserveB).unwrap();

        let token_a_client = token::Client::new(&env,&token_a);
        let token_b_client = token::Client::new(&env,&token_b);

        token_a_client.transfer(&user,&env.current_contract_address(),amount_in);

        let amount_out = get_amount_out(amount_in, reserve_a, reserve_b);   

        token_b_client.transfer(&env.current_contract_address(),&user,amount_out);

        env.storage().instance().set(&DataKey::ReserveA,&(reserve_a + amount_in));
        env.storage().instance().set(&DataKey::ReserveB,&(reserve_b - amount_out));


        amount_out  
    }
     
    pub fn get_reserves(env:Env) ->(i128,i128){
        let reserve_a:i128 = env.storage().instance().get(&DataKey::ReserveA).unwrap();
        let reserve_b:i128 = env.storage().instance().get(&DataKey::ReserveB).unwrap();
        (reserve_a,reserve_b)
    
    
}   