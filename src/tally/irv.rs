use std::collections::{HashMap, HashSet};

use super::{TallyEngine, TallyError};
use crate::core::{BallotData, CandidateId};

pub struct IrvEngine;

impl TallyEngine for IrvEngine {
    fn tally(&self, ballots: &[BallotData]) -> Result<CandidateId, TallyError> {
        let mut ranked: Vec<&Vec<CandidateId>> = Vec::new();
        for ballot in ballots {
            match ballot {
                BallotData::Ordinal(v) => ranked.push(v),
                _ => return Err(TallyError::InvalidBallotType),
            }
        }

        if ranked.is_empty() {
            return Err(TallyError::EmptyElection);
        }

        let all_candidates: HashSet<&CandidateId> =
            ranked.iter().flat_map(|b| b.iter()).collect();

        if all_candidates.is_empty() {
            return Err(TallyError::EmptyElection);
        }

        let mut eliminated: HashSet<&CandidateId> = HashSet::new();

        loop {
            let mut counts: HashMap<&CandidateId, usize> = HashMap::new();
            let mut active = 0usize;

            for ballot in &ranked {
                for candidate in ballot.iter() {
                    if !eliminated.contains(candidate) {
                        *counts.entry(candidate).or_insert(0) += 1;
                        active += 1;
                        break;
                    }
                }
            }

            if active == 0 {
                return Err(TallyError::ExhaustedBallots);
            }

            let majority = active / 2 + 1;
            if let Some((winner, _)) = counts.iter().find(|(_, &c)|
                                                          c >= majority) {
                return Ok((*winner).clone());
            }

            let min_votes = counts
                .values()
                .min()
                .ok_or(TallyError::EmptyElection)?;
            let min_candidate = counts
                .iter()
                .find(|(_, &c)| c == *min_votes)
                .map(|(c, _)| *c)
                .ok_or(TallyError::EmptyElection)?;

            eliminated.insert(min_candidate);
        }
    }
}
