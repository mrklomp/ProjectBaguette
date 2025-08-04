use std::collections::HashMap;
use crate::game::enums::{CardClass, CardType, Rarity, SpellSchool, Races};
use crate::game::effects::Effect;
use crate::game::triggers::Trigger;
use crate::game::keywords::Keywords;


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
    pub keywords: Keywords,
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
        if self.has_kw(Keywords::MEGAWINDFURY) {
            4
        } else if self.has_kw(Keywords::WINDFURY) {
            2
        } else {
            1
        }
    }

    /// Retourne `true` si le serviteur possède le mot-clé donné.
    pub fn has_kw(&self, kw: Keywords) -> bool {
        self.keywords.has(kw)
    }
    /// Ajoute un mot-clé (utile pour Reborn, buffs, etc.).
    pub fn add_kw(&mut self, kw: Keywords) {
        self.keywords |= kw;
    }
    /// Retire un mot-clé (ex. perdre Divine Shield).
    pub fn remove_kw(&mut self, kw: Keywords) {
        self.keywords.remove(kw);
    }
}
