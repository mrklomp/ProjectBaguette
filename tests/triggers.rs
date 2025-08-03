use hearthstone_sim::game::player::Player;
use hearthstone_sim::game::state::{GameState, PlayerId};
use hearthstone_sim::game::card::{Card, CardStatus};
use hearthstone_sim::game::enums::{CardType, CardClass};
use hearthstone_sim::game::effects::Effect;
use hearthstone_sim::game::engine::attack::perform_attack;

#[test]
fn test_attack_with_divine_shield_and_rush() {
    let mut player1 = Player::new(PlayerId::Player1, vec![], CardClass::Neutral);
    let mut player2 = Player::new(PlayerId::Player2, vec![], CardClass::Neutral);

    let attacker = Card {
        card_id: "attacker".to_string(),
        name: "Rush Minion".to_string(),
        card_type: CardType::Minion,
        card_class: CardClass::Neutral,
        cost: 3,
        attack: Some(3),
        health: Some(5),
        max_health: Some(5),
        status: CardStatus {
            current_health: Some(5),
            attack_modifiers: 0,
            health_modifiers: 0,
            silenced: false,
            frozen: false,
            has_attacked: false,
            just_played: true,
            attacks_this_turn: 0,
        },
        effects: vec![Effect::Rush],
        native_effects: vec![Effect::Rush],
        text: None,
        tags: std::collections::HashMap::new(),
        spell_school: None,
        races: None,
        triggered_effects: std::collections::HashMap::new(),
    };
    player1.zones.board.push(attacker);

    let defender = Card {
        card_id: "defender".to_string(),
        name: "Divine Shield Minion".to_string(),
        card_type: CardType::Minion,
        card_class: CardClass::Neutral,
        cost: 4,
        attack: Some(4),
        health: Some(6),
        max_health: Some(6),
        status: CardStatus {
            current_health: Some(6),
            attack_modifiers: 0,
            health_modifiers: 0,
            silenced: false,
            frozen: false,
            has_attacked: false,
            just_played: false,
            attacks_this_turn: 0,
        },
        effects: vec![Effect::DivineShield],
        native_effects: vec![Effect::DivineShield],
        text: None,
        tags: std::collections::HashMap::new(),
        spell_school: None,
        races: None,
        triggered_effects: std::collections::HashMap::new(),
    };
    player2.zones.board.push(defender);

    let mut state = GameState::new(player1, player2);

    let player1_ptr: *mut hearthstone_sim::game::player::Player = state.players.get_mut(&PlayerId::Player1).unwrap() as *mut _;
let player2_ptr: *mut hearthstone_sim::game::player::Player = state.players.get_mut(&PlayerId::Player2).unwrap() as *mut _;

let attacker_mut = unsafe { &mut (*player1_ptr).zones.board[0] };
let defender_mut = unsafe { &mut (*player2_ptr).zones.board[0] };

        let (attacker_dead, defender_dead, damage_by_attacker, damage_by_defender) = perform_attack(attacker_mut, defender_mut);

        println!("Attacker dead: {}", attacker_dead);
        println!("Defender dead: {}", defender_dead);
        println!("Damage dealt by attacker: {}", damage_by_attacker);
        println!("Damage dealt by defender: {}", damage_by_defender);

        assert_eq!(attacker_dead, false);
        assert_eq!(defender_dead, false);
        assert!(attacker_mut.status.current_health.unwrap() < 5);
        assert!(defender_mut.effects.iter().all(|e| *e != Effect::DivineShield)); // Divine Shield doit être consommé
    
}
