use crate::messages::ChallengeResult::ChallengeAnswer;
#[derive(Debug)]

pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}