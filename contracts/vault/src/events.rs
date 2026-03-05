use soroban_sdk::{symbol_short, Address, Env};

pub fn deposit(env: &Env, user: Address, amount: i128) {
    env.events()
        .publish((symbol_short!("deposit"), user), amount);
}

pub fn withdraw(env: &Env, user: Address, amount: i128) {
    env.events()
        .publish((symbol_short!("withdraw"), user), amount);
}

pub fn borrow(env: &Env, user: Address, amount: i128) {
    env.events()
        .publish((symbol_short!("borrow"), user), amount);
}
pub fn repay(env: &Env, user: Address, amount: i128) {
    env.events().publish((symbol_short!("repay"), user), amount);
}
