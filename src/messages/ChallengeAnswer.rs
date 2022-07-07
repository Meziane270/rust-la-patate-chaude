use crate::md5Models::MD5HashCashOutput;
#[derive(Debug)]

pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}