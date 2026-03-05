#![no_std]

mod storage;

use soroban_sdk::{contract, contractimpl, Env, Address, String};
use crate::storage::{DataKey, Proposal};

#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {

pub fn initialize(env: Env, admin: Address) {
    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage().instance().set(&DataKey::ProposalCount, &0u32);
}

pub fn set_voting_power(env: Env, user: Address, power: i128) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();

    env.storage().persistent().set(&DataKey::VotingPower(user), &power);
}

pub fn propose(env: Env, proposer: Address, description: String, quorum: i128) -> u32 {
    proposer.require_auth();

    let mut count: u32 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0);

    let proposal = Proposal {
        proposer: proposer.clone(),
        description,
        vote_for: 0,
        vote_against: 0,
        executed: false,
        quorum,
    };

    env.storage().instance().set(&DataKey::Proposal(count), &proposal);

    // Snapshot current voting power for the proposer (and potentially others if we had a list)
    // For this simple implementation, we'll assume voting power is fixed or we use current power if no snapshot.
    let voting_power: i128 = env.storage().persistent().get(&DataKey::VotingPower(proposer.clone())).unwrap_or(0);
    env.storage().instance().set(&DataKey::Snapshot(count, proposer), &voting_power);

    let id = count;
    count += 1;
    env.storage().instance().set(&DataKey::ProposalCount, &count);

    id
}

pub fn vote(env: Env, voter: Address, proposal_id: u32, support: bool) {
    voter.require_auth();

    // Check if already voted
    let voted_key = DataKey::ProposalVotes(proposal_id, voter.clone());
    if env.storage().persistent().has(&voted_key) {
        panic!("Already voted");
    }

    let mut proposal: Proposal = env.storage().instance().get(&DataKey::Proposal(proposal_id)).unwrap();

    if proposal.executed {
        panic!("Proposal already executed");
    }

    // Try to get snapshot, fallback to current power if not snapshotted
    let voting_power: i128 = env.storage().instance()
        .get(&DataKey::Snapshot(proposal_id, voter.clone()))
        .unwrap_or_else(|| {
            env.storage().persistent().get(&DataKey::VotingPower(voter.clone())).unwrap_or(0)
        });

    if voting_power <= 0 {
        panic!("No voting power");
    }

    if support {
        proposal.vote_for += voting_power;
    } else {
        proposal.vote_against += voting_power;
    }

    env.storage().persistent().set(&voted_key, &voting_power);
    env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
}

pub fn execute(env: Env, proposal_id: u32) {
    let mut proposal: Proposal = env.storage().instance().get(&DataKey::Proposal(proposal_id)).unwrap();

    if proposal.executed {
        panic!("Proposal already executed");
    }

    let total_votes = proposal.vote_for + proposal.vote_against;
    if total_votes < proposal.quorum {
        panic!("Quorum not reached");
    }

    if proposal.vote_for <= proposal.vote_against {
        panic!("Proposal did not pass");
    }

    proposal.executed = true;

    env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
}

pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
    env.storage().instance().get(&DataKey::Proposal(proposal_id)).unwrap()
}

pub fn get_voting_power(env: Env, user: Address) -> i128 {
    env.storage().persistent().get(&DataKey::VotingPower(user)).unwrap_or(0)
}

}

#[cfg(test)]
mod test;