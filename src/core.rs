use std::collections::{HashMap, HashSet};

pub type CandidateId = String;

/// The configuration for an election, stored in the Raft state machine.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum VotingMethod {
    Irv,
    Schulze,
    Star,
    Borda,
}

/// A single, concrete type that can represent *any* ballot format in the Raft log.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BallotData {
    /// For IRV, Borda, Condorcet (ordered from 1st choice to last)
    Ordinal(Vec<CandidateId>),
    /// For Score and STAR voting (Candidate -> Score)
    Cardinal(HashMap<CandidateId, u8>),
    /// For Approval voting
    Approval(HashSet<CandidateId>),
}
