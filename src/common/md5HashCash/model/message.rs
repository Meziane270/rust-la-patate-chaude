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