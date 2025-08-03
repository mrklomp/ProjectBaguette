use crate::game::state::PlayerId;

pub trait IdString {
    fn id_string(&self) -> String;
}

// Implémentation pour PlayerId (pour le debug/affichage)
impl IdString for PlayerId {
    fn id_string(&self) -> String {
        match self {
            PlayerId::Player1 => "Joueur 1".to_string(),
            PlayerId::Player2 => "Joueur 2".to_string(),
        }
    }
}

use rand::seq::SliceRandom;

pub trait ChooseRandomMut<T> {
    fn choose_random_mut(&mut self) -> Option<&mut T>;
}

impl<T> ChooseRandomMut<T> for Vec<T> {
    fn choose_random_mut(&mut self) -> Option<&mut T> {
        self.as_mut_slice().choose_mut(&mut rand::thread_rng())
    }
}

/// Affiche le nom et les stats d'un minion au format "Nom [ATK|PV]"
pub fn minion_stats_string(minion: &crate::game::card::Card) -> String {
    format!("{} [{}|{}]", minion.name, minion.effective_attack(), minion.status.current_health.unwrap_or(0))
}
