use crate::SubscribeResult::SubscribeResult;
#[derive(Debug)]

pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}