mod data;
mod game;
mod logger;

use crate::data::card_template::{load_card_templates, CardTemplate};
use crate::game::engine::play_turn::play_turn;
use crate::game::player::Player;
use crate::game::state::{GameState, PlayerId};
use crate::game::engine::choose::RandomChooser;
use crate::logger::{log_deck_to_file, log_mulligan_to_file};

use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

const DECKS_LOG: &str = "parsing/decks.jsonl";
const MULLIGAN_LOG: &str = "parsing/mulligan.jsonl";
const STATE_LOG: &str = "parsing/all_games.jsonl";

fn main() {
    // Charge les templates (JSON)
    let templates = load_card_templates("cards/CORE2025.json")
        .expect("Erreur chargement cards.json");

    //println!("\n🔍 Vérification des effets parsés sur les minions :");
    //for (id, template) in &templates {
        //if format!("{:?}", template.card_type).to_ascii_uppercase() == "MINION" {
            //if let Some(effects) = &template.effects {
                //if !effects.is_empty() {
                    //println!("{}: {:?}", template.card_name, effects);
                //}
            //}
        //}
    //}

    for t in templates.values() {
        if let Some(effs) = &t.effects {
            println!("{}: {:?}", t.card_name, effs);
        }
    }


    // Statistiques de classes
    let mut counts_by_class: HashMap<String, usize> = HashMap::new();
    for t in templates.values() {
        let class = format!("{:?}", t.card_class).to_ascii_uppercase();
        *counts_by_class.entry(class).or_insert(0) += 1;
    }
    println!("\n📊 Répartition des cartes par classe :");
    for (class, count) in &counts_by_class {
        println!("- {} : {}", class, count);
    }

    // ========== SIMULATION ==========
    let n_games = 1;
    for game_id in 0..n_games {
        // --- Tirage des classes
        let class1 = random_class();
        let class2 = random_class();

        println!("\nClasse Joueur 1 : {}", class1);
        println!("Classe Joueur 2 : {}", class2);

        // --- Génération des decks
        let deck1 = generate_deck(&templates, &class1);
        let deck2 = generate_deck(&templates, &class2);

        // Print debug
        println!("\nDeck généré pour la classe {} :", class1);
        for card in &deck1 {
            println!("- {} [{} - {}]", card.card_name, format!("{:?}", card.card_class), format!("{:?}", card.card_type));
        }
        println!("\nDeck généré pour la classe {} :", class2);
        for card in &deck2 {
            println!("- {} [{} - {}]", card.card_name, format!("{:?}", card.card_class), format!("{:?}", card.card_type));
        }

        // (optionnel) logs
        log_deck_to_file(DECKS_LOG, game_id, "Player1", &class1, &deck1.iter().map(|t| t.to_card()).collect::<Vec<_>>());
        log_deck_to_file(DECKS_LOG, game_id, "Player2", &class2, &deck2.iter().map(|t| t.to_card()).collect::<Vec<_>>());


        // --- Création des joueurs et de l'état de jeu
        let player1 = Player::new(
            PlayerId::Player1,
            deck1.iter().map(|t| t.to_card()).collect(),
            deck1[0].card_class.clone()
        );
        let player2 = Player::new(
            PlayerId::Player2,
            deck2.iter().map(|t| t.to_card()).collect(),
            deck2[0].card_class.clone()
        );

        let mut state = GameState::new(player1, player2);
        let mut chooser = RandomChooser;

        // --- Toss pile/face pour déterminer qui commence
        let mut rng = rand::thread_rng();
        let first = if rng.gen_bool(0.5) {
            PlayerId::Player1
        } else {
            PlayerId::Player2
        };
        let second = first.opponent();
        println!("\n🎲 Le toss donne : {:?} commence la partie !", first);

        // --- Distribue les cartes de départ (PAS de mana ici, on le fera après mulligan)
        {
            let player = state.players.get_mut(&first).unwrap();
            for _ in 0..3 { player.draw_card(); }
        }
        {
            let player = state.players.get_mut(&second).unwrap();
            for _ in 0..4 { player.draw_card(); }
        }

        // --- PHASE DE MULLIGAN pour chaque joueur ---
        for id in [first, second] {
            let original_hand = state.players[&id].zones.hand.clone();
            // L'IA choisit ce qu'elle garde (ou tout garder par défaut)
            let keep = original_hand.clone(); // (simplifié, tu peux remettre ton IA après)

            // Cartes à mulligan
            let mut to_mulligan = Vec::new();
            for c in &original_hand {
                if !keep.iter().any(|k| k.card_id == c.card_id) {
                    to_mulligan.push(c.clone());
                }
            }
            let redraw_count = to_mulligan.len();

            // Mut borrow uniquement ici
            let player = state.players.get_mut(&id).unwrap();
            // Retire de la main toutes celles qui ne sont pas gardées
            player.zones.hand.retain(|c| keep.iter().any(|k| k.card_id == c.card_id));
            // Remet celles à mulligan dans le deck et mélange
            player.zones.deck.extend(to_mulligan.clone());
            player.zones.deck.shuffle(&mut rng);
            // Repiocher
            for _ in 0..redraw_count {
                player.draw_card();
            }
            log_mulligan_to_file(
                MULLIGAN_LOG,
                game_id,
                &format!("{:?}", id),
                &original_hand,
                &keep,
                &to_mulligan,
                &player.zones.hand,
            );

        }

        // --- Ajout de The Coin à la main du second joueur (NON mulliganable)
        {
            let player = state.players.get_mut(&second).unwrap();
            if let Some(template) = templates.get("GAME_005") {
                player.zones.hand.push(template.to_card());
                println!("🪙 The Coin ajoutée à la main de {:?}", second);
            } else {
                println!("⚠️ Carte The Coin (GAME_005) non trouvée dans les templates !");
            }
        }

        // --- Initialise le mana pour le joueur qui commence (après le mulligan)
        {
            let player = state.players.get_mut(&first).unwrap();
            player.stats.mana.max = 1;
            player.stats.mana.current = 1;
        }

        // --- Affiche la main de chaque joueur après le mulligan et The Coin
        for id in [first, second] {
            let player = state.players.get(&id).unwrap();
            println!(
                "Main de {:?} ({:?} cartes) : {:?}",
                id,
                player.zones.hand.len(),
                player.zones.hand.iter().map(|c| c.name.clone()).collect::<Vec<_>>()
            );
        }

        // --- Le joueur qui commence doit être le "current_player"
        state.current_player = first;

        // --- Simulation des tours
        while state.round <= 16 {
            if let Some(_winner) = state.winner {
                break;
            }
            play_turn(&mut state, &mut chooser, game_id, &templates);
        }
    }
    println!("Simulation terminée.");
}

// Sélectionne une classe au hasard
fn random_class() -> String {
    let classes = vec![
        "DRUID", "HUNTER", "MAGE", "PALADIN", "PRIEST", "ROGUE",
        "SHAMAN", "WARLOCK", "WARRIOR", "DEMONHUNTER", "DEATHKNIGHT",
    ];
    let mut rng = rand::thread_rng();
    classes.choose(&mut rng).unwrap().to_string()
}

// Génère un deck pour une classe donnée, compatible String
fn generate_deck(
    templates: &HashMap<String, CardTemplate>,
    class: &str,
) -> Vec<CardTemplate> {
    let mut rng = rand::thread_rng();
    let mut deck = Vec::new();
    let mut counts: HashMap<String, u8> = HashMap::new();

    let pool: Vec<_> = templates
        .values()
        .filter(|t| {
            format!("{:?}", t.card_type).to_ascii_uppercase() == "MINION"
                && t.cost.unwrap_or(99) >= 3
                && t.collectible.unwrap_or(false)
                && (
                    format!("{:?}", t.card_class).to_ascii_uppercase() == "NEUTRAL" ||
                    format!("{:?}", t.card_class).to_ascii_uppercase() == class
                )
        })
        .collect();

    if pool.len() < 30 {
        return deck;
    }

    while deck.len() < 30 {
        if let Some(template) = pool.choose(&mut rng) {
            let count = counts.entry(template.card_id.clone()).or_insert(0);
            if *count < 2 {
                deck.push((*template).clone());
                *count += 1;
            }
        }
    }
    deck
}
