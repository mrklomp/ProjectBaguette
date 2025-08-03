use rand::seq::SliceRandom;

use crate::game::card::Card;
use crate::game::state::GameState;


/// Les différents types de choix qu'un joueur peut devoir faire
#[derive(Debug, Clone)]
pub enum Choice {
    Mulligan { hand: Vec<Card> },
    PlayCard { hand: Vec<Card>, mana: u8 },
    PlayCardIndex(usize),
    ChooseOne { options: Vec<Card> },
    Discover { options: Vec<Card> },
    Target { valid_targets: Vec<usize> },
    EndTurn,
}

/// Trait à implémenter pour toute stratégie de prise de décision (IA, CLI, etc.)
pub trait Chooser {
    fn choose(&self, state: &GameState, choice: Choice) -> Choice;
}

/// Implémentation aléatoire naïve
pub struct RandomChooser;

impl Chooser for RandomChooser {
    fn choose(&self, _state: &GameState, choice: Choice) -> Choice {
        match &choice {
            Choice::Mulligan { hand } => {
                // Garde tout (naïvement)
                Choice::Mulligan { hand: hand.clone() }
            }

            Choice::PlayCard { hand, mana } => {
                let playable: Vec<_> = hand
                    .iter()
                    .enumerate()
                    .filter(|(_, card)| card.cost <= *mana)
                    .map(|(i, _)| i)
                    .collect();

                if let Some(&index) = playable.choose(&mut rand::thread_rng()) {
                    Choice::PlayCardIndex(index)
                } else {
                    Choice::EndTurn
                }
            }


            Choice::ChooseOne { options } | Choice::Discover { options } => {
                if let Some(card) = options.choose(&mut rand::thread_rng()) {
                    Choice::ChooseOne {
                        options: vec![card.clone()],
                    }
                } else {
                    choice
                }
            }

            Choice::Target { valid_targets } => {
                if let Some(&target) = valid_targets.choose(&mut rand::thread_rng()) {
                    Choice::Target {
                        valid_targets: vec![target],
                    }
                } else {
                    choice
                }
            }

            Choice::EndTurn => Choice::EndTurn,
            _ => choice,
        }
    }
}

/// Méthode utilitaire pour extraire un index de carte à jouer
impl Choice {
    pub fn as_play_card(&self) -> Option<usize> {
        match self {
            Choice::PlayCardIndex(i) => Some(*i),
            _ => None,
        }
    }
}
