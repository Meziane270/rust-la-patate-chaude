use crate :: messages::PublicPlayer::PublicPlayer;
#[derive(Debug)]

pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}