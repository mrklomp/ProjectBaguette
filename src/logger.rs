use crate::game::state::{GameState, PlayerId};
use crate::game::card::Card;
use serde::Serialize;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

/// Log minimal de l’état de la partie à chaque action/tour
#[derive(Serialize)]
pub struct SimpleLog {
    pub game_id: u64,
    pub turn: u32,
    pub player: String,
    pub mana: u8,
    pub hand_count: usize,
    pub opponent_hand_count: usize,
    pub deck_count: usize,
    pub opponent_deck_count: usize,
    pub board_minion_count: usize,
    pub opponent_board_minion_count: usize,
    pub board_attack_total: i32,
    pub opponent_board_attack_total: i32,
    pub phase: String,
    pub winner: Option<String>,
}

/// Log d’un deck initial (utile pour ré-analyser une partie)
#[derive(Serialize)]
pub struct DeckLog {
    pub game_id: u64,
    pub player: String,
    pub class: String,
    pub deck: Vec<String>,
}

/// Log détaillé du mulligan d’un joueur
#[derive(Serialize)]
pub struct MulliganLog {
    pub game_id: u64,
    pub player: String,
    pub initial_hand: Vec<String>,
    pub kept: Vec<String>,
    pub mulliganed: Vec<String>,
    pub redrawn: Vec<String>,
}

/// Log l’état simple du jeu dans un fichier JSONL
pub fn log_simple_state_to_file(
    file_path: &str,
    game_id: u64,
    state: &GameState,
    current_id: PlayerId,
    opponent_id: PlayerId,
) {
    // S’assure que le dossier existe (une seule fois en début de programme c’est suffisant, mais safe ici)
    create_dir_all("parsing").expect("Impossible de créer le dossier parsing");

    let player = state.players.get(&current_id).unwrap();
    let opponent = state.players.get(&opponent_id).unwrap();

    let log = SimpleLog {
        game_id,
        turn: state.round,
        player: format!("{:?}", current_id),
        mana: player.stats.mana.current,
        hand_count: player.zones.hand.len(),
        opponent_hand_count: opponent.zones.hand.len(),
        deck_count: player.zones.deck.len(),
        opponent_deck_count: opponent.zones.deck.len(),
        board_minion_count: player.zones.board.len(),
        opponent_board_minion_count: opponent.zones.board.len(),
        board_attack_total: player.zones.board.iter().map(|m| m.attack.unwrap_or(0)).sum(),
        opponent_board_attack_total: opponent.zones.board.iter().map(|m| m.attack.unwrap_or(0)).sum(),
        phase: format!("{:?}", state.phase),
        winner: state.winner.map(|w| format!("{:?}", w)),
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Impossible d'ouvrir le fichier de log");
    writeln!(file, "{}", serde_json::to_string(&log).unwrap())
        .expect("Impossible d'écrire dans le fichier de log");
}

/// Log le deck complet d’un joueur
pub fn log_deck_to_file(
    file_path: &str,
    game_id: u64,
    player: &str,
    class: &str,
    deck: &[Card],
) {
    create_dir_all("parsing").expect("Impossible de créer le dossier parsing");

    let log = DeckLog {
        game_id,
        player: player.to_string(),
        class: class.to_string(),
        deck: deck
            .iter()
            .map(|c| format!("{} [{:?} - {:?}]", c.name, c.card_class, c.card_type))
            .collect(),
    };
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Impossible d'ouvrir le fichier de log deck");
    writeln!(file, "{}", serde_json::to_string(&log).unwrap())
        .expect("Impossible d'écrire dans le fichier de log deck");
}

/// Log le choix de mulligan (cartes gardées/échangées)
pub fn log_mulligan_to_file(
    file_path: &str,
    game_id: u64,
    player: &str,
    initial_hand: &[Card],
    kept: &[Card],
    mulliganed: &[Card],
    redrawn: &[Card],
) {
    create_dir_all("parsing").expect("Impossible de créer le dossier parsing");

    let log = MulliganLog {
        game_id,
        player: player.to_string(),
        initial_hand: initial_hand.iter().map(|c| c.name.clone()).collect(),
        kept: kept.iter().map(|c| c.name.clone()).collect(),
        mulliganed: mulliganed.iter().map(|c| c.name.clone()).collect(),
        redrawn: redrawn.iter().map(|c| c.name.clone()).collect(),
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Impossible d'ouvrir le fichier de log mulligan");
    writeln!(file, "{}", serde_json::to_string(&log).unwrap())
        .expect("Impossible d'écrire dans le fichier de log mulligan");
}
