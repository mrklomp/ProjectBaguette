use std::collections::HashMap;
use crate::game::enums::{CardClass, CardType, Rarity, SpellSchool, Races};
use crate::game::effects::Effect;
use crate::game::triggers::Trigger;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Card {
    pub card_id: String,
    pub name: String,
    pub cost: u8,
    pub card_type: CardType,
    pub attack: Option<i32>,
    pub health: Option<i32>,
    pub max_health: Option<i32>,
    pub text: Option<String>,
    pub card_class: CardClass,
    pub tags: HashMap<String, i32>,
    pub status: CardStatus,
    pub effects: Vec<Effect>,
    pub native_effects: Vec<Effect>,
    pub spell_school: Option<SpellSchool>,
    pub races: Option<Vec<Races>>,
    pub triggered_effects: HashMap<Trigger, Vec<Effect>>,
}

#[derive(Debug, Clone)]
pub struct CardStatus {
    pub current_health: Option<i32>,
    pub attack_modifiers: i32,
    pub health_modifiers: i32,
    pub silenced: bool,
    pub frozen: bool,
    pub has_attacked: bool,
    pub just_played: bool,
    pub attacks_this_turn: u8,
}

impl Card {
    pub fn effective_attack(&self) -> i32 {
        self.attack.unwrap_or(0) + self.status.attack_modifiers
    }
    pub fn effective_health(&self) -> i32 {
        self.status.current_health.unwrap_or(0)
    }
    pub fn max_attacks_per_turn(&self) -> u8 {
        if self.effects.contains(&Effect::Windfury) { 2 } else { 1 }
    }
}
