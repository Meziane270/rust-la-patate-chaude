use crate::SubscribeError::SubscribeError;
#[derive(Debug)]

pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}