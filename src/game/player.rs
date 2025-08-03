use crate::game::card::Card;
use crate::game::state::PlayerId;
use crate::game::enums::CardClass;
use crate::game::engine::utils::IdString;


pub const MAX_HAND: usize = 10;
pub const MAX_BOARD: usize = 7;
pub const MAX_MANA: u8 = 10;
pub const MAX_HEALTH: i32 = 30;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub stats: PlayerStats,
    pub zones: PlayerZones,
    pub fatigue_counter: u32,
    pub overload_pending: u8,
    pub hero_has_attacked: bool,
}

#[derive(Debug)]
pub struct PlayerStats {
    pub health: i32,
    pub max_health: i32,
    pub armor: u32,
    pub mana: Mana,
    pub class: CardClass,
    pub weapon: Option<Card>,
    pub hero_power: Option<Card>,
    pub extra_turns: i32,
}

#[derive(Debug)]
pub struct Mana {
    pub current: u8,
    pub max: u8,
}

impl Mana {
    pub fn new() -> Self { Self { current: 0, max: 0 } }
    pub fn refill(&mut self) { self.current = self.max; }
    pub fn spend(&mut self, amount: u8) -> bool {
        if self.current >= amount { self.current -= amount; true } else { false }
    }
    pub fn gain_max(&mut self, amount: u8) {
        self.max = (self.max + amount).min(MAX_MANA);
    }
}

#[derive(Debug)]
pub struct PlayerZones {
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub board: Vec<Card>,
    pub graveyard: Vec<Card>,
    pub secrets: Vec<Card>,
    pub set_aside: Vec<Card>,
    pub dormant: Vec<Card>,
}

impl Player {
    pub fn id_string(&self) -> String {
        self.id.id_string()
    }

    pub fn new(id: PlayerId, deck: Vec<Card>, class: CardClass) -> Self {
        Self {
            id,
            stats: PlayerStats {
                health: MAX_HEALTH,
                max_health: MAX_HEALTH,
                armor: 0,
                mana: Mana::new(),
                class,
                weapon: None,
                hero_power: None,
                extra_turns: 0,
            },
            zones: PlayerZones {
                deck,
                hand: Vec::new(),
                board: Vec::new(),
                graveyard: Vec::new(),
                secrets: Vec::new(),
                set_aside: Vec::new(),
                dormant: Vec::new(),
            },
            fatigue_counter: 0,
            overload_pending: 0,
            hero_has_attacked: false,
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        if let Some(card) = self.zones.deck.pop() {
            if self.zones.hand.len() < MAX_HAND {
                self.zones.hand.push(card.clone());
            } else {
                self.zones.graveyard.push(card.clone());
            }
            Some(card)
        } else {
            self.fatigue_counter += 1;
            self.take_damage(self.fatigue_counter as i32);
            None
        }
    }

    pub fn start_turn(&mut self) {
        if self.stats.mana.max < MAX_MANA {
            self.stats.mana.gain_max(1);
        }
        self.stats.mana.refill();
        self.hero_has_attacked = false;
        for minion in self.zones.board.iter_mut() {
        minion.status.attacks_this_turn = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        let old_health = self.stats.health;
        self.stats.health = (self.stats.health + amount).min(self.stats.max_health);
        println!(
            "Le joueur {:?} est soigné de {} PV (PV avant : {}, après : {})",
            self.id, amount, old_health, self.stats.health
            );
        }

    pub fn take_damage(&mut self, amount: i32) {
        let mut remaining = amount;
        if self.stats.armor > 0 {
            let absorbed = remaining.min(self.stats.armor as i32);
            self.stats.armor -= absorbed as u32;
            remaining -= absorbed;
        }
        self.stats.health -= remaining.max(0);
    }

    pub fn is_dead(&self) -> bool {
        self.stats.health <= 0
    }
}
