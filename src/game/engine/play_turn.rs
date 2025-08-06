use crate::game::{
    engine::{
        attack::perform_attack_phase,
        choose::{Choice, Chooser},
        events::dispatch_events,
        play_card::play_card_at_index,
        draw::draw_card,
    },
    event::GameEvent,
    state::{GameState, PlayerId},
};
use crate::logger::log_simple_state_to_file;

/// Joue un tour complet pour le joueur courant
pub fn play_turn(
    state: &mut GameState,
    chooser: &mut dyn Chooser,
    game_id: u64,
    card_templates: &std::collections::HashMap<String, crate::data::card_template::CardTemplate>,
) {
    let current_id   = state.current_player;
    let opponent_id  = current_id.opponent();

    // ─── 0. Log début de tour ────────────────────────────────────────────────
    println!("\n--- Tour {} : {:?} ---", state.round, current_id);

    // ─── 1. Événement Start-of-Turn ───────────────────────────────────────────
    state
        .event_queue
        .push_back(GameEvent::TurnStart { player: current_id });
    dispatch_events(state);

    // ---- 1. Réinitialise just_played / has_attacked / attacks_this_turn ----
    if let Some(player) = state.players.get_mut(&current_id) {
    for minion in player.zones.board.iter_mut() {
        if minion.status.just_played {
            minion.status.just_played = false;
        }
        minion.status.has_attacked = false;
        minion.status.attacks_this_turn = 0; // ← AJOUT
    }
}

    // ─── 3. Pioche automatique ───────────────────────────────────────────────
    if let Some(player) = state.players.get_mut(&current_id) {
        let before = player.zones.hand.len();
        draw_card(player);
        let after  = player.zones.hand.len();
        if after > before {
            println!("{:?} pioche 1 carte => {} cartes en main", current_id, after);
        }
    }

    // ─── 4. Log d’état simple (optionnel) ────────────────────────────────────
    log_simple_state_to_file(
        "parsing/all_games.jsonl",
        game_id,
        state,
        state.current_player,
        state.current_player.opponent(),
    );

    // ─── 5. Affiche les mains ────────────────────────────────────────────────
    for (id, label) in [(current_id, "Joueur Actif"), (opponent_id, "Adversaire")] {
        if let Some(player) = state.players.get(&id) {
            let noms: Vec<_> = player.zones.hand.iter().map(|c| c.name.as_str()).collect();
            println!("{} ({id:?}) : {} cartes en main {:?}", label, noms.len(), noms);
        }
    }

    // ─── 6. Boucle “jouer des cartes” ────────────────────────────────────────
    loop {
        let player = state.players.get(&current_id).unwrap();
        let hand   = player.zones.hand.clone();
        let mana   = player.stats.mana.current;

        match chooser.choose(state, Choice::PlayCard { hand, mana }) {
            Choice::PlayCardIndex(i) => {
                println!("🎮 Joueur {current_id:?} joue la carte en position {i}");
                if !play_card_at_index(state, &current_id, i, chooser, card_templates) {
                    println!("⚠️  Erreur lors du jeu de la carte");
                    break;
                }
            }
            _ => break, // Aucun coup jouable
        }
    }

    // ─── 7. Phase d’attaque ──────────────────────────────────────────────────
    perform_attack_phase(state, &current_id, &opponent_id, chooser, card_templates);

    // ─── 8. Événement End-of-Turn ────────────────────────────────────────────
    state
        .event_queue
        .push_back(GameEvent::TurnEnd { player: current_id });
    dispatch_events(state);

    // ─── 9. Affiche PV fin de tour ───────────────────────────────────────────
    let p1 = &state.players[&PlayerId::Player1];
    let p2 = &state.players[&PlayerId::Player2];
    println!(
        "=== Fin du tour : PV Héros ===\n  Player1: {} PV\n  Player2: {} PV\n",
        p1.stats.health, p2.stats.health
    );

    // ─── 10. Vérifie fin de partie puis passe le tour ────────────────────────
    state.check_game_over();
    state.switch_turn(); // passe au joueur suivant
}
