use std::collections::HashMap;
use crate::game::player::Player;
use crate::game::engine::utils::IdString;
use std::collections::VecDeque;
use crate::game::event::GameEvent;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerId {
    Player1,
    Player2,
}

impl PlayerId {
    pub fn opponent(&self) -> PlayerId {
        match self {
            PlayerId::Player1 => PlayerId::Player2,
            PlayerId::Player2 => PlayerId::Player1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamePhase {
    Mulligan,
    InProgress,
    GameOver,
}

#[derive(Debug)]
pub struct GameState {
    pub players: HashMap<PlayerId, Player>,
    pub current_player: PlayerId,
    pub round: u32,
    pub phase: GamePhase,
    pub winner: Option<PlayerId>,
    pub event_queue: VecDeque<GameEvent>,
}

impl GameState {
    pub fn new(player1: Player, player2: Player) -> Self {
        // 1. deux variables distinctes
        let mut players = HashMap::new();
        let event_queue = VecDeque::new();

        players.insert(PlayerId::Player1, player1);
        players.insert(PlayerId::Player2, player2);

        Self {
            players,
            current_player: PlayerId::Player1,
            round: 1,
            phase: GamePhase::InProgress,
            winner: None,
            event_queue,
        }
    }

    pub fn current_player(&self) -> &Player {
        self.players.get(&self.current_player).unwrap()
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        self.players.get_mut(&self.current_player).unwrap()
    }

    pub fn opponent_player(&self) -> &Player {
        self.players.get(&self.current_player.opponent()).unwrap()
    }

    pub fn opponent_player_mut(&mut self) -> &mut Player {
        self.players.get_mut(&self.current_player.opponent()).unwrap()
    }

    pub fn switch_turn(&mut self) {
        // Vérifie l'extra turn du joueur courant
        let cur_id = self.current_player;
        let cur_player = self.players.get_mut(&cur_id).unwrap();

        if cur_player.stats.extra_turns > 0 {
            cur_player.stats.extra_turns -= 1;
            // Il rejoue, pas de changement de joueur ni d'incrément de round
            println!("{} gagne un tour supplémentaire ! (restant : {})", cur_id.id_string(), cur_player.stats.extra_turns);
            cur_player.start_turn();
        } else {
            // Tour classique : passe à l'adversaire
            self.current_player = self.current_player.opponent();

            // Incrémente round SEULEMENT quand Player1 commence
            if self.current_player == PlayerId::Player1 {
                self.round += 1;
            }

            self.current_player_mut().start_turn();
        }
    }


    pub fn is_game_over(&self) -> bool {
        matches!(self.phase, GamePhase::GameOver)
    }

    pub fn check_game_over(&mut self) -> bool {
        let p1_dead = self.players.get(&PlayerId::Player1).unwrap().is_dead();
        let p2_dead = self.players.get(&PlayerId::Player2).unwrap().is_dead();

        if p1_dead || p2_dead {
            self.winner = if p1_dead && !p2_dead {
                Some(PlayerId::Player2)
            } else if p2_dead && !p1_dead {
                Some(PlayerId::Player1)
            } else {
                None // match nul
            };
            self.phase = GamePhase::GameOver;
            true
        } else {
            self.phase = GamePhase::InProgress;
            false
        }
    }
}
