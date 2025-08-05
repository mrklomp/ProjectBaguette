use crate::game::{
    event::GameEvent,
    state::GameState,
    triggers::TriggerDef,
};
use crate::game::state::PlayerId;
use crate::game::effects::{Effect,apply_effect};
use std::collections::HashMap;
use crate::game::engine::choose::RandomChooser;

/// Vide la file d’événements et déclenche les triggers correspondants.
pub fn dispatch_events(state: &mut GameState) {
    let mut guard = 0;                       // ← hors du while (une seule fois)

    let mut chooser = RandomChooser;
    let card_templates = HashMap::new();

    while let Some(event) = state.event_queue.pop_front() {
        guard += 1;
        println!("event #{guard}: {:?}", event);   // ordre d’arrivée

        if guard > 20 {                           // ← 20 suffit
            println!("⚠️ guard dispatch_events STOP (20 événements)");
            break;
        }
    }


    while let Some(event) = state.event_queue.pop_front() {
        println!("🔔 Event : {:?}", event);

        // 1) On collecte d’abord tout ce qui doit se déclencher
        let mut pending: Vec<(PlayerId, Effect)> = Vec::new();

        for (&owner_id, player) in &mut state.players {
            for card in &player.zones.board {        // <- emprunt immuable seulement
                for trig in &card.triggers {
                    if trig.matches(&event, owner_id, &card.card_id) {
                        println!("➡️  Trigger {:?} sur {}", trig.when, card.name);
                        pending.push((owner_id, trig.effect.clone()));
                    }
                }
            }
        }

        // 2) L’emprunt sur state.players est libéré ici (fin du for)
        //    On peut maintenant emprunter `&mut state` pour apply_effect.
        for (owner_id, eff) in pending {
            apply_effect(
                state,
                &owner_id,
                &eff,
                &mut chooser,
                &card_templates,
            );
        }
    }
}
