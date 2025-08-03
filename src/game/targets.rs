use serde::{Serialize, Deserialize};
use crate::game::enums::Zone;
use crate::game::enums::CardType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    // Single targets
    AnyCharacter, 
    AnyMinion,       
    EnemyCharacter,      
    FriendlyCharacter,   
    EnemyMinion,
    FriendlyMinion,
    OtherMinion,
    SelfTarget,
    EnemyHero,
    FriendlyHero,

    // Multi-targets (AOE)
    AllEnemyCharacter,
    AllEnemyMinion,
    AllFriendlyMinion,
    AllFriendlyCharacter,
    AllMinion,
    AllOtherMinion,

    // Utility
    AdjacentFriendlyMinion,
    LowestHealthEnemy,
    NextFriendlyCard,

    // Locations & misc (laisser, tu n’utilises pas encore)
    DeckCardsNotStartingInDeck,
    EnemyLocation,
    FriendlyLocation,
    AnyLocation,
    OpponentBoard,
    OpponentHeroPower,
    OpponentSpells,
    OpponentWeapon,
    OtherCharacter,
    OtherFriendlyMinion,
    OtherPlayer,
    SelfCopy,
    SummonedMinion,
    HandMinion { zone: Zone, card_type: CardType },
}

impl Target {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "any_character" => Some(Target::AnyCharacter),
            "any_minion" => Some(Target::AnyMinion),
            "enemy_character" => Some(Target::EnemyCharacter),
            "friendly_character" => Some(Target::FriendlyCharacter),
            "enemy_minion" => Some(Target::EnemyMinion),
            "friendly_minion" => Some(Target::FriendlyMinion),
            "other_minion" => Some(Target::OtherMinion),
            "self" => Some(Target::SelfTarget),
            "enemy_hero" => Some(Target::EnemyHero),
            "friendly_hero" => Some(Target::FriendlyHero),
            "all_enemy_character" => Some(Target::AllEnemyCharacter),
            "all_enemy_minion" => Some(Target::AllEnemyMinion),
            "all_friendly_minion" => Some(Target::AllFriendlyMinion),
            "all_friendly_character" => Some(Target::AllFriendlyCharacter),
            "all_minion" => Some(Target::AllMinion),
            "all_other_minion" => Some(Target::AllOtherMinion),
            "adjacent_friendly_minion" => Some(Target::AdjacentFriendlyMinion),
            "lowest_health_enemy" => Some(Target::LowestHealthEnemy),
            "next_friendly_card" => Some(Target::NextFriendlyCard),
            "deck_cards_not_starting_in_deck" => Some(Target::DeckCardsNotStartingInDeck),
            "enemy_location" => Some(Target::EnemyLocation),
            "friendly_location" => Some(Target::FriendlyLocation),
            "any_location" => Some(Target::AnyLocation),
            "opponent_board" => Some(Target::OpponentBoard),
            "opponent_hero_power" => Some(Target::OpponentHeroPower),
            "opponent_spells" => Some(Target::OpponentSpells),
            "opponent_weapon" => Some(Target::OpponentWeapon),
            "other_character" => Some(Target::OtherCharacter),
            "other_friendly_minion" => Some(Target::OtherFriendlyMinion),
            "other_player" => Some(Target::OtherPlayer),
            "self_copy" => Some(Target::SelfCopy),
            "summoned_minion" => Some(Target::SummonedMinion),
            // HandMinion paramétré (ignore pour l’instant)
            _ => None,
        }
    }
}
