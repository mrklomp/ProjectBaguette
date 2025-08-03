use crate::game::state::{GameState, PlayerId};
use crate::game::engine::attack::perform_attack_phase;
use crate::game::engine::draw::draw_card;
use crate::game::engine::choose::{Chooser, Choice};
use crate::game::engine::play_card::play_card_at_index;
use crate::logger::log_simple_state_to_file;
use crate::game::triggers::Trigger;

/// Joue un tour complet pour le joueur courant en utilisant un `Chooser`
pub fn play_turn(
    state: &mut GameState,
    chooser: &mut dyn Chooser,
    game_id: u64,
    card_templates: &std::collections::HashMap<String, crate::data::card_template::CardTemplate>,
) {
    let current_id = state.current_player;
    let opponent_id = current_id.opponent();

    // ---- PRINT DÉBUT DE TOUR ----
    println!("\n--- Tour {} : {:?} ---", state.round, current_id);

    // ---- TRIGGERS START OF TURN (Current & Enemy) ----
    {
        // StartOfTurn (joueur courant)
        if let Some(player) = state.players.get(&current_id) {
            let effects: Vec<_> = player.zones.board.iter()
                .filter_map(|m| m.triggered_effects.get(&Trigger::StartOfTurn))
                .flat_map(|v| v.iter().cloned())
                .collect();
            for effect in effects {
                crate::game::effects::apply_effect(state, &current_id, &effect, chooser, card_templates);
            }
        }
        // StartOfEnemyTurn (adversaire)
        if let Some(opponent) = state.players.get(&opponent_id) {
            let effects: Vec<_> = opponent.zones.board.iter()
                .filter_map(|m| m.triggered_effects.get(&Trigger::StartOfEnemyTurn))
                .flat_map(|v| v.iter().cloned())
                .collect();
            for effect in effects {
                crate::game::effects::apply_effect(state, &current_id, &effect, chooser, card_templates);
            }
        }
    }

    // ---- 1. Réinitialise just_played et has_attacked sur les minions du joueur courant ----
    if let Some(player) = state.players.get_mut(&current_id) {
        for minion in player.zones.board.iter_mut() {
            if minion.status.just_played {
                minion.status.just_played = false;
            }
            minion.status.has_attacked = false;
        }
    }

    // ---- 2. Pioche 1 carte ----
    if let Some(player) = state.players.get_mut(&current_id) {
        let avant = player.zones.hand.len();
        draw_card(player);
        let apres = player.zones.hand.len();
        if apres > avant {
            println!("{:?} pioche 1 carte => {} cartes en main", current_id, apres);
        }
    }

    // ---- 3. Log (optionnel) ----
    log_simple_state_to_file(
    "parsing/all_games.jsonl",
    game_id,
    state,
    state.current_player,
    state.current_player.opponent(),
);

    // ---- 4. Affiche la main des joueurs ----
    for (id, label) in [(current_id, "Joueur Actif"), (opponent_id, "Adversaire")] {
        if let Some(player) = state.players.get(&id) {
            let noms: Vec<_> = player.zones.hand.iter().map(|c| c.name.as_str()).collect();
            println!("{} ({:?}) : {} cartes en main {:?}", label, id, noms.len(), noms);
        }
    }

    // ---- 5. Boucle de jeu tant que le joueur a de quoi jouer (mana/carte) ----
    loop {
        let player = state.players.get(&current_id).unwrap();
        let hand = player.zones.hand.clone();
        let mana = player.stats.mana.current;
        let choice = chooser.choose(state, Choice::PlayCard { hand, mana });

        match choice {
            Choice::PlayCardIndex(i) => {
                println!("🎮 Joueur {:?} joue la carte en position {}", current_id, i);
                let success = play_card_at_index(state, &current_id, i, chooser, card_templates);
                if !success {
                    println!("⚠️ Erreur lors du jeu de la carte.");
                    break;
                }
            }
            _ => break, // Aucun choix de carte valide → on passe
        }
    }

    // ---- 6. Phase d’attaque ----
    perform_attack_phase(state, &current_id, &opponent_id, chooser, card_templates);


    

    // ---- 7. TRIGGERS END OF TURN (Current & Enemy) ----
    {
        // EndOfTurn (joueur courant)
        if let Some(player) = state.players.get(&current_id) {
            let effects: Vec<_> = player.zones.board.iter()
                .filter_map(|m| m.triggered_effects.get(&Trigger::EndOfTurn))
                .flat_map(|v| v.iter().cloned())
                .collect();
            for effect in effects {
                crate::game::effects::apply_effect(state, &current_id, &effect, chooser, card_templates);
            }
        }
        // EndOfEnemyTurn (adversaire)
        if let Some(opponent) = state.players.get(&opponent_id) {
            let effects: Vec<_> = opponent.zones.board.iter()
                .filter_map(|m| m.triggered_effects.get(&Trigger::EndOfEnemyTurn))
                .flat_map(|v| v.iter().cloned())
                .collect();
            for effect in effects {
                crate::game::effects::apply_effect(state, &current_id, &effect, chooser, card_templates);
            }
        }
    }

    // ---- 8. Affichage fin de tour ----
    let p1 = state.players.get(&PlayerId::Player1).unwrap();
    let p2 = state.players.get(&PlayerId::Player2).unwrap();
    println!(
        "=== Fin du tour : PV Héros ===\n  Player1: {} PV\n  Player2: {} PV\n",
        p1.stats.health,
        p2.stats.health
    );

    // ---- 9. Vérifie fin de partie, puis passe le tour ----
    state.check_game_over();

    // ---- 10. Passe au joueur suivant (switch turn) ----
    state.switch_turn();
}
