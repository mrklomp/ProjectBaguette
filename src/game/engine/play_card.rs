use crate::game::state::{GameState, PlayerId};
use crate::game::enums::CardType;
use crate::game::targets::Target;
use crate::game::effects::{Effect, apply_effect};
use crate::game::engine::utils::{minion_stats_string, IdString};
use crate::game::triggers::Trigger;


pub fn play_card_at_index(
    state: &mut GameState,
    player_id: &PlayerId,
    hand_index: usize,
    chooser: &dyn crate::game::engine::choose::Chooser,
    card_templates: &std::collections::HashMap<String, crate::data::card_template::CardTemplate>,
) -> bool {
    // Attention, on NE récupère plus player tout de suite
    if let Some(player) = state.players.get_mut(player_id) {
        if hand_index >= player.zones.hand.len() {
            println!("⚠️ Index {} invalide pour la main du joueur.", hand_index);
            return false;
        }
    }

    // On retire la carte (besoin de réemprunter plus tard)
    let card = {
        let player = state.players.get_mut(player_id).unwrap();
        player.zones.hand.remove(hand_index)
    };

    // On paie le mana avant tout
    let can_play = {
        let player = state.players.get_mut(player_id).unwrap();
        player.stats.mana.spend(card.cost)
    };

    if can_play {
        match card.card_type {
            CardType::Weapon => {
                let player = state.players.get_mut(player_id).unwrap();
                if let Some(old_weapon) = player.stats.weapon.replace(card.clone()) {
                    player.zones.graveyard.push(old_weapon);
                    println!(
                        "{} brise son ancienne arme et équipe {} (ATK {}, DUR {})",
                        player.id_string(),
                        card.name,
                        card.attack.unwrap_or(0),
                        card.health.unwrap_or(0)
                    );
                } else {
                    println!(
                        "{} équipe l'arme {} (ATK {}, DUR {})",
                        player.id_string(),
                        card.name,
                        card.attack.unwrap_or(0),
                        card.health.unwrap_or(0)
                    );
                }
                true
            }
            CardType::Minion => {
                println!(
                    "{} joue la carte: {}",
                    player_id.id_string(),
                    minion_stats_string(&card)
                );


                // === Effets de battlecry (appliqués AVANT de placer sur le board)
                if let Some(effects) = card.triggered_effects.get(&crate::game::triggers::Trigger::Battlecry) {
                    for effect in effects {
                        apply_effect(state, player_id, effect, chooser, card_templates);
                    }
                }


                // Place la carte sur le board APRÈS l'effet
                let mut card = card;
                card.status.just_played = true;
                let player = state.players.get_mut(player_id).unwrap();
                player.zones.board.push(card);
                true
            }
            _ => {
                let player = state.players.get_mut(player_id).unwrap();
                println!(
                    "{} joue une carte non gérée: {}",
                    player.id_string(),
                    card.name
                );
                true
            }
        }
    } else {
        // Pas assez de mana → remettre la carte
        let player = state.players.get_mut(player_id).unwrap();
        println!(
            "❌ Pas assez de mana pour jouer {} (coût {})",
            card.name, card.cost
        );
        player.zones.hand.insert(hand_index, card);
        false
    }
}
