use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,                        // Governance admin
    ProposalCount,                // Total number of proposals
    Proposal(u32),                // Proposal data by ID
    ProposalVotes(u32, Address),  // User votes per proposal
    VotingPower(Address),          // Voting power per address
    Snapshot(u32, Address),        // Balance snapshot at proposal creation
}

#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub proposer: Address,
    pub description: String,
    pub vote_for: i128,
    pub vote_against: i128,
    pub executed: bool,
    pub quorum: i128, // minimal votes required
}