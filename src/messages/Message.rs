use crate::messages::Welcome::Welcome;
use crate::messages::Subscribe::Subscribe;
use crate::messages::Subscribe_result::SubscribeResult;
use crate::messages::PublicPlayer::PublicPlayer;
use crate::messages::Challenge::Challenge;
use crate::messages::ChallengeResult::ChallengeResult;
use crate::messages::RoundSummary::RoundSummary;
use crate::messages::EndOfGame::EndOfGame;

#[derive(Debug)]

pub enum Message {
    Excluded(String),
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}