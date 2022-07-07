use crate::md5Models::MD5HashCashInput;
#[derive(Debug)]

pub enum Challenge {
    MD5HashCash(MD5HashCashInput)
}