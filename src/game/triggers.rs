use serde::{Serialize, Deserialize};
use crate::game::event::GameEvent;
use crate::game::effects::Effect;
use crate::game::state::PlayerId;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    AfterHeroAttack,
    AfterPlay,
    AfterSelfAttack,
    AfterSummon,
    AfterAnyDamage,
    Battlecry,
    Combo,
    Deathrattle,
    EndOfTurn,
    StartOfTurn,
    EndOfEnemyTurn,
    StartOfEnemyTurn,
    OnAttack,
    OnFriendlyUndeadDeath,
    OnKill,
    OnSpellCast,
    Outcast,
    WhenDrawn,
}

#[derive(Debug, Clone)]
pub struct TriggerDef {
    pub when: Trigger,
    pub effect: Effect,
}

impl TriggerDef {
    pub fn matches(&self, event: &GameEvent, owner: PlayerId, card_id: &str) -> bool {
        match (&self.when, event) {
            (Trigger::Battlecry,
             GameEvent::CardPlayed { card_id: played_id, owner: ev_owner }) =>
                 ev_owner == &owner && played_id == card_id,

            (Trigger::Deathrattle,
             GameEvent::MinionDied { card_id: dead_id, owner: ev_owner }) =>
                 ev_owner == &owner && dead_id == card_id,

            (Trigger::StartOfTurn,
             GameEvent::TurnStart { player }) =>
                 player == &owner,

            (Trigger::EndOfTurn,
             GameEvent::TurnEnd { player }) =>
                 player == &owner,

            _ => false,
        }
    }
}

