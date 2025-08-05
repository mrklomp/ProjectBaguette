// src/game/card.rs
use std::collections::HashMap;

use crate::game::{
    enums::{CardClass, CardType, SpellSchool, Races},
    effects::Effect,
    keywords::Keywords,
    triggers::TriggerDef,
};

#[derive(Debug, Clone)]
pub struct Card {
    // ────────────────────────────────────────────────────────────────  infos fixes
    pub card_id:     String,
    pub name:        String,
    pub cost:        u8,
    pub card_type:   CardType,
    pub card_class:  CardClass,

    // ────────────────────────────────────────────────────────────────  stats
    pub attack:      Option<i32>,
    pub health:      Option<i32>,
    pub max_health:  Option<i32>,

    // ────────────────────────────────────────────────────────────────  texte & tags
    pub text:        Option<String>,
    pub tags:        HashMap<String, i32>,

    // ────────────────────────────────────────────────────────────────  keywords
    pub keywords:    Keywords,

    // ────────────────────────────────────────────────────────────────  état runtime
    pub status:      CardStatus,

    // ────────────────────────────────────────────────────────────────  effets
    pub effects:         Vec<Effect>,   // tous les effets « actifs »
    pub native_effects:  Vec<Effect>,   // copie d’origine (utile pour silence)
    pub triggers:        Vec<TriggerDef>,

    // ────────────────────────────────────────────────────────────────  méta
    pub spell_school: Option<SpellSchool>,
    pub races:        Option<Vec<Races>>,
}

#[derive(Debug, Clone)]
pub struct CardStatus {
    pub current_health:    Option<i32>,
    pub attack_modifiers:  i32,
    pub health_modifiers:  i32,
    pub silenced:          bool,
    pub frozen:            bool,
    pub has_attacked:      bool,
    pub just_played:       bool,
    pub attacks_this_turn: u8,
}

// ───────────────────────────────────────────────────────────────────────── helpers
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

    /// Vérifie la présence d’un mot-clé.
    pub fn has_kw(&self, kw: Keywords) -> bool {
        self.keywords.has(kw)
    }
    /// Ajoute un mot-clé (ex. Reborn, buff…).
    pub fn add_kw(&mut self, kw: Keywords) {
        self.keywords |= kw;
    }
    /// Retire un mot-clé (perte de Divine Shield, silence…).
    pub fn remove_kw(&mut self, kw: Keywords) {
        self.keywords.remove(kw);
    }
}
