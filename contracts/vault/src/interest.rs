use soroban_sdk::Env;
use crate::storage::DataKey;

pub fn utilization(env:&Env)->i128{

    let total_deposits:i128 = env.storage()
        .instance()
        .get(&DataKey::TotalDeposits)
        .unwrap_or(0);

    let total_borrows:i128 = env.storage()
        .instance()
        .get(&DataKey::TotalBorrows)
        .unwrap_or(0);

    if total_deposits == 0 {
        return 0
    }

    total_borrows * 100 / total_deposits
}

pub fn borrow_rate(env:&Env)->i128{

    let base:i128 = env.storage()
        .instance()
        .get(&DataKey::BaseRate)
        .unwrap_or(0);

    let multiplier:i128 = env.storage()
        .instance()
        .get(&DataKey::Multiplier)
        .unwrap_or(0);

    let util = utilization(env);

    base + (util * multiplier / 100)
}