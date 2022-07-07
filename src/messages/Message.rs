use crate::messages::Welcome::Welcome;
use crate::messages::Subscribe::Subscribe;
use crate::messages::Subscribe_result::SubscribeResult;
use crate::messages::Challenge::Challenge;
use crate::messages::ChallengeResult::ChallengeResult;
use crate::messages::EndOfGame::EndOfGame;
use crate::messages::PublicPlayer::PublicPlayer;
use crate::messages::RoundSummary::RoundSummary;

#[derive(Debug)]

pub enum Message {
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}