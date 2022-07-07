use crate::messages::ReportedChallengeResult::ReportedChallengeResult;
#[derive(Debug)]

pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>,
}