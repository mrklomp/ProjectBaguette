use crate::game::{
    event::GameEvent,
    state::{GameState, PlayerId},
};
use crate::game::engine::choose::RandomChooser;
use crate::game::effects::{apply_effect, Effect};
use std::collections::HashMap;

pub fn dispatch_events(state: &mut GameState) {
    let mut chooser = RandomChooser;
    let card_templates: HashMap<String, crate::data::card_template::CardTemplate> = HashMap::new();

    // Limiteur d'événements uniquement en debug (pas en release)
    #[cfg(debug_assertions)]
    const MAX_EVENTS_DEBUG: usize = 50;
    #[cfg(debug_assertions)]
    let mut guard = 0usize;

    while let Some(event) = state.event_queue.pop_front() {
        #[cfg(debug_assertions)]
        {
            guard += 1;
            println!("event #{}: {:?}", guard, event);
            if guard > MAX_EVENTS_DEBUG {
                println!("⚠️ guard dispatch_events STOP ({MAX_EVENTS_DEBUG} événements)");
                break;
            }
            // Affiche les triggers de la carte jouée — utile en debug
            if let GameEvent::CardPlayed { ref card_id, .. } = event {
                for (&pid, pl) in &state.players {
                    for c in &pl.zones.board {
                        if c.card_id == *card_id {
                            println!(
                                "DBG: on-board {} (owner={:?}) has {} trigger(s): {:?}",
                                c.name,
                                pid,
                                c.triggers.len(),
                                c.triggers.iter().map(|t| &t.when).collect::<Vec<_>>()
                            );
                        }
                    }
                }
            }
        }

        // 1) Collecte (emprunts immuables uniquement)
        let mut pending: Vec<(PlayerId, Effect)> = Vec::new();
        for (&owner_id, player) in &state.players {
            for card in &player.zones.board {
                for trig in &card.triggers {
                    if trig.matches(&event, owner_id, &card.card_id) {
                        #[cfg(debug_assertions)]
                        println!("➡️  Trigger {:?} sur {}", trig.when, card.name);
                        pending.push((owner_id, trig.effect.clone()));
                    }
                }
            }
        }

        // 2) Application (emprunt mutable ensuite)
        for (owner_id, eff) in pending {
            apply_effect(state, &owner_id, &eff, &mut chooser, &card_templates);
        }
    }
}
