use crate::core::{BallotData, CandidateId};

#[derive(Debug)]
pub enum TallyError {
    InvalidBallotType,
    TieFound,
    EmptyElection,
    ExhaustedBallots,
}

pub trait TallyEngine {
    fn tally(&self, ballots: &[BallotData]) -> Result<CandidateId, TallyError>;
}

pub mod irv;
