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

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerDef {
    pub when: Trigger,
    pub effect: crate::game::effects::Effect,
}

impl TriggerDef {
    /// `owner_id` = contrôleur du permanent qui porte le trigger
    /// `self_card_id` = id de la carte qui porte le trigger
    pub fn matches(
        &self,
        event: &crate::game::event::GameEvent,
        owner_id: crate::game::state::PlayerId,
        self_card_id: &str,
    ) -> bool {
        use crate::game::event::GameEvent::*;
        use Trigger::*;

        match (&self.when, event) {
            // Battlecry : on ne déclenche que si l’événement concerne CETTE carte
            (Battlecry, CardPlayed { card_id, owner }) => {
                *owner == owner_id && card_id == self_card_id
            }

            // Deathrattle : on ne déclenche que si CETTE carte vient de mourir
            (Deathrattle, MinionDied { card_id, owner }) => {
                *owner == owner_id && card_id == self_card_id
            }

            // Début/fin de tour du contrôleur
            (StartOfTurn, TurnStart { player }) => *player == owner_id,
            (EndOfTurn,   TurnEnd   { player }) => *player == owner_id,

            // Début/fin de tour adverse
            (StartOfEnemyTurn, TurnStart { player }) => *player == owner_id.opponent(),
            (EndOfEnemyTurn,   TurnEnd   { player }) => *player == owner_id.opponent(),

            // Par défaut : pas de match
            _ => false,
        }
    }
}
