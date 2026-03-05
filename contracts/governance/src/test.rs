#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address,
    Env,
    String,
};


fn setup_governance(env: &Env) -> (GovernanceClient<'_>, Address, Address, Address) {
    let admin = Address::generate(env);
    let user1 = Address::generate(env);
    let user2 = Address::generate(env);

    let contract_id = env.register_contract(None, Governance);
    let gov = GovernanceClient::new(env, &contract_id);

    gov.initialize(&admin);

    env.mock_all_auths();

    // set voting power
    gov.set_voting_power(&user1, &100);
    gov.set_voting_power(&user2, &50);

    (gov, admin, user1, user2)
}


#[test]
fn test_propose() {
    let env = Env::default();
    let (gov, _admin, user1, _user2) = setup_governance(&env);

    let proposal_id = gov.propose(
        &user1,
        &String::from_str(&env, "Increase staking rewards"),
        &50,
    );

    assert_eq!(proposal_id, 0);

    let proposal = gov.get_proposal(&proposal_id);

    assert_eq!(proposal.proposer, user1);
    assert_eq!(proposal.description, String::from_str(&env, "Increase staking rewards"));
    assert_eq!(proposal.vote_for, 0);
    assert_eq!(proposal.vote_against, 0);
    assert!(!proposal.executed);
    assert_eq!(proposal.quorum, 50);
}


#[test]
fn test_vote() {
    let env = Env::default();
    let (gov, _admin, user1, user2) = setup_governance(&env);

    let proposal_id = gov.propose(
        &user1,
        &String::from_str(&env, "Update fees"),
        &50,
    );

    // user1 votes for
    gov.vote(&user1, &proposal_id, &true);

    // user2 votes against
    gov.vote(&user2, &proposal_id, &false);

    let proposal = gov.get_proposal(&proposal_id);

    assert_eq!(proposal.vote_for, 100);
    assert_eq!(proposal.vote_against, 50);
}


#[test]
fn test_execute() {
    let env = Env::default();
    let (gov, _admin, user1, user2) = setup_governance(&env);

    let proposal_id = gov.propose(
        &user1,
        &String::from_str(&env, "Add new token"),
        &50,
    );

    gov.vote(&user1, &proposal_id, &true);
    gov.vote(&user2, &proposal_id, &false);

    // quorum reached: 100+50 >= 50
    gov.execute(&proposal_id);

    let proposal = gov.get_proposal(&proposal_id);
    assert!(proposal.executed);
}

#[test]
fn test_voting_power() {
    let env = Env::default();
    let (gov, _admin, user1, user2) = setup_governance(&env);

    let vp1 = gov.get_voting_power(&user1);
    let vp2 = gov.get_voting_power(&user2);

    assert_eq!(vp1, 100);
    assert_eq!(vp2, 50);
}