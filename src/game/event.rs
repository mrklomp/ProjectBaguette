use crate::game::state::PlayerId;

#[derive(Debug, Clone)]
pub enum GameEvent {
    CardPlayed   { card_id: String, owner: PlayerId },
    MinionDied   { card_id: String, owner: PlayerId },
    TurnStart    { player: PlayerId },
    TurnEnd      { player: PlayerId },
}
