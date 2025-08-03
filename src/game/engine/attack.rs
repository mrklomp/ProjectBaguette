use crate::game::state::{GameState, PlayerId};
use crate::game::card::Card;
use crate::game::effects::{Effect, apply_effect};
use crate::game::engine::utils::{minion_stats_string, IdString};
use crate::game::triggers::Trigger;
use std::collections::HashMap;
use crate::data::card_template::CardTemplate;



pub fn perform_attack_phase(state: &mut GameState, current_id: &PlayerId, opponent_id: &PlayerId,
                            chooser: &dyn crate::game::engine::choose::Chooser,card_templates: &HashMap<String, CardTemplate>,) {
    loop {
        let attacker_id_opt = {
            let player = state.players.get(current_id).unwrap();
            player.zones.board
                .iter()
                .find(|minion| {
                    if minion.status.attacks_this_turn >= minion.max_attacks_per_turn() {
                        false
                    } else if minion.status.just_played {
                        minion.effects.contains(&Effect::Charge) || minion.effects.contains(&Effect::Rush)
                    } else {
                        true
                    }
                })
                .map(|minion| minion.card_id.clone())
        };

        if let Some(attacker_id) = attacker_id_opt {
            // Recherche de la cible
            let defender_id_opt = {
                let opponent = state.players.get(opponent_id).unwrap();
                let taunt_minions: Vec<_> = opponent.zones.board
                    .iter()
                    .filter(|m| m.effects.contains(&Effect::Taunt) && !m.effects.contains(&Effect::Stealth))
                    .collect();

                if !taunt_minions.is_empty() {
                    taunt_minions.first().map(|m| m.card_id.clone())
                } else {
                    let possible_targets: Vec<_> = opponent.zones.board
                        .iter()
                        .filter(|m| !m.effects.contains(&Effect::Stealth))
                        .collect();
                    if !possible_targets.is_empty() {
                        possible_targets.first().map(|m| m.card_id.clone())
                    } else {
                        None
                    }
                }
            };

            // RUSH: skip le tour si pas de minion adverse
            {
                let player = state.players.get(current_id).unwrap();
                if let Some(attacker) = player.zones.board.iter().find(|c| c.card_id == attacker_id) {
                    let is_rush = attacker.effects.contains(&Effect::Rush);
                    let is_charge = attacker.effects.contains(&Effect::Charge);

                    if defender_id_opt.is_none() && attacker.status.just_played && is_rush && !is_charge {
                        // Incrémente attacks_this_turn quand on skip une attaque, pour éviter boucle infinie
                        if let Some(attacker_mut) = state.players.get_mut(current_id)
                            .and_then(|pl| pl.zones.board.iter_mut().find(|c| c.card_id == attacker_id))
                        {
                            attacker_mut.status.attacks_this_turn += 1;
                            println!("[DEBUG] {} (Rush) n'a pas pu attaquer ce tour (attaques: {})", attacker_mut.name, attacker_mut.status.attacks_this_turn);
                        }
                        continue;
                    }
                }
            }

            if let Some(defender_id) = defender_id_opt {
                // --- Attaque un minion adverse ---
                let (attacker_dead, defender_dead, damage_by_attacker, damage_by_defender);
                let attacker_has_lifesteal;
                let defender_has_lifesteal;
                {
                    let (player, opponent) = {
                        let player_ptr: *mut crate::game::player::Player = state.players.get_mut(current_id).unwrap() as *mut _;
                        let opponent_ptr: *mut crate::game::player::Player = state.players.get_mut(opponent_id).unwrap() as *mut _;
                        unsafe { (&mut *player_ptr, &mut *opponent_ptr) }
                    };

                    let maybe_attacker = player.zones.board.iter_mut()
                        .find(|c| c.card_id == attacker_id);
                    if maybe_attacker.is_none() { continue; }
                    let attacker = maybe_attacker.unwrap();

                    let maybe_defender = opponent.zones.board.iter_mut()
                        .find(|c| c.card_id == defender_id);
                    if maybe_defender.is_none() { continue; }
                    let defender = maybe_defender.unwrap();

                    println!(
                        "{} attaque {}",
                        minion_stats_string(&attacker),
                        minion_stats_string(&defender)
                    );


                    attacker_has_lifesteal = attacker.effects.contains(&Effect::Lifesteal);
                    defender_has_lifesteal = defender.effects.contains(&Effect::Lifesteal);

                    (attacker_dead, defender_dead, damage_by_attacker, damage_by_defender)
                        = perform_attack(attacker, defender);

                    println!(
                        "Résultat : {}, {}",
                        minion_stats_string(&attacker),
                        minion_stats_string(&defender)
                    );

                }
                if attacker_has_lifesteal && damage_by_attacker > 0 {
                    let player = state.players.get_mut(current_id).unwrap();
                    player.heal(damage_by_attacker as i32);
                }
                if defender_has_lifesteal && damage_by_defender > 0 {
                    let opponent_player = state.players.get_mut(opponent_id).unwrap();
                    opponent_player.heal(damage_by_defender as i32);
                }

                if attacker_dead {
                    // Reborn, avant tout
                    let player = state.players.get_mut(current_id).unwrap();
                    for minion in player.zones.board.iter_mut() {
                        if minion.status.current_health.unwrap_or(0) <= 0
                            && minion.effects.contains(&Effect::Reborn)
                        {
                            println!("{} revient en vie grâce à Reborn !", minion.name);
                            minion.effects = minion.native_effects.iter()
                                .filter(|e| **e != Effect::Reborn)
                                .cloned()
                                .collect();
                            minion.status.current_health = Some(1);
                        }
                    }
                    // Clone les effets à appliquer
                    let effects_to_apply = {
                        let player = state.players.get(current_id).unwrap();
                        player.zones.board
                            .iter()
                            .find(|c| c.card_id == attacker_id)
                            .and_then(|minion| minion.triggered_effects.get(&crate::game::triggers::Trigger::Deathrattle).cloned())
                    };
                    println!("{} est mort", attacker_id);

                    if let Some(effects) = effects_to_apply {
                        for effect in effects {
                            apply_effect(state, current_id, &effect, chooser, card_templates);
                        }
                    }

                    // Maintenant seulement tu fais le retain
                    let player = state.players.get_mut(current_id).unwrap();
                    player.zones.board.retain(|c| c.card_id != attacker_id || c.status.current_health.unwrap_or(0) > 0);
                }

                if defender_dead {
                    let opponent = state.players.get_mut(opponent_id).unwrap();
                    for minion in opponent.zones.board.iter_mut() {
                        if minion.status.current_health.unwrap_or(0) <= 0
                            && minion.effects.contains(&Effect::Reborn)
                        {
                            println!("{} revient en vie grâce à Reborn !", minion.name);
                            minion.effects = minion.native_effects.iter()
                                .filter(|e| **e != Effect::Reborn)
                                .cloned()
                                .collect();
                            minion.status.current_health = Some(1);
                        }
                    }
                    // Clone les effets à appliquer
                    let effects_to_apply = {
                        let opponent = state.players.get(opponent_id).unwrap();
                        opponent.zones.board
                            .iter()
                            .find(|c| c.card_id == defender_id)
                            .and_then(|minion| minion.triggered_effects.get(&crate::game::triggers::Trigger::Deathrattle).cloned())
                    };
                    println!("{} est mort", defender_id);

                    if let Some(effects) = effects_to_apply {
                        for effect in effects {
                            apply_effect(state, current_id, &effect, chooser, card_templates);
                        }
                    }

    // Maintenant seulement tu fais le retain
    let opponent = state.players.get_mut(opponent_id).unwrap();
    opponent.zones.board.retain(|c| c.card_id != defender_id || c.status.current_health.unwrap_or(0) > 0);
}


                // --- Incrémente le compteur d’attaques pour Windfury ---
                let player = state.players.get_mut(current_id).unwrap();
                if let Some(attacker) = player.zones.board.iter_mut().find(|c| c.card_id == attacker_id) {
                    attacker.status.attacks_this_turn += 1;
                    println!(
                        "[DEBUG] {} a attaqué {} fois ce tour (max: {})",
                        attacker.name,
                        attacker.status.attacks_this_turn,
                        attacker.max_attacks_per_turn()
                    );
                }
            } else {
                // --- Attaque le héros adverse ---
                let (attacker_dead, hero_died);
                let mut lifesteal_to_heal = 0;
                {
                    let (player, opponent) = {
                        let player_ptr: *mut crate::game::player::Player = state.players.get_mut(current_id).unwrap() as *mut _;
                        let opponent_ptr: *mut crate::game::player::Player = state.players.get_mut(opponent_id).unwrap() as *mut _;
                        unsafe { (&mut *player_ptr, &mut *opponent_ptr) }
                    };

                    let attacker = match player.zones.board.iter_mut().find(|c| c.card_id == attacker_id) {
                        Some(a) => a,
                        None => continue,
                    };

                    let is_rush = attacker.effects.contains(&Effect::Rush);
                    let is_charge = attacker.effects.contains(&Effect::Charge);

                    if is_rush && !is_charge {
                        // Incrémente attacks_this_turn quand on skip une attaque, pour éviter boucle infinie
                        let player = state.players.get_mut(current_id).unwrap();
                        if let Some(attacker_mut) = player.zones.board.iter_mut().find(|c| c.card_id == attacker_id) {
                            attacker_mut.status.attacks_this_turn += 1;
                            println!("[DEBUG] {} (Rush) n'a pas pu attaquer le héros adverse (attaques: {})", attacker_mut.name, attacker_mut.status.attacks_this_turn);
                        }
                        continue;
                    }

                    let damage = attacker.effective_attack();
                    let attacker_has_lifesteal = attacker.effects.contains(&Effect::Lifesteal);
                    attacker.status.has_attacked = true;

                    println!(
                        "{} [{}|{}] attaque le héros adverse [{} PV] pour {} dégâts",
                        attacker.name,
                        attacker.effective_attack(),
                        attacker.effective_health(),
                        opponent.stats.health,
                        damage
                    );


                    opponent.take_damage(damage as i32);
                    hero_died = opponent.is_dead();

                    println!(
                        "PV du héros adverse [{}] après l’attaque : {}",
                        opponent_id.id_string(),
                        opponent.stats.health
                    );


                    attacker.status.current_health = Some(attacker.effective_health());
                    attacker_dead = attacker.status.current_health.unwrap_or(0) <= 0;

                    if attacker_has_lifesteal {
                        lifesteal_to_heal = damage;
                    }
                }
                if lifesteal_to_heal > 0 {
                    let player = state.players.get_mut(current_id).unwrap();
                    player.heal(lifesteal_to_heal as i32);
                }

                if attacker_dead {
                    let player = state.players.get_mut(current_id).unwrap();
                    for minion in player.zones.board.iter_mut() {
                        if minion.status.current_health.unwrap_or(0) <= 0
                            && minion.effects.contains(&Effect::Reborn)
                        {
                            println!("{} revient en vie grâce à Reborn !", minion.name);
                            minion.effects = minion.native_effects.iter()
                                .filter(|e| **e != Effect::Reborn)
                                .cloned()
                                .collect();
                            minion.status.current_health = Some(1);
                        }
                    }
                    let player = state.players.get_mut(current_id).unwrap();
                    println!("{} est mort", attacker_id);
                    player.zones.board.retain(|c| c.card_id != attacker_id || c.status.current_health.unwrap_or(0) > 0);
                }
                if hero_died {
                    println!("Le héros adverse est mort !");
                }

                // --- Incrémente le compteur d’attaques pour Windfury ---
                let player = state.players.get_mut(current_id).unwrap();
                if let Some(attacker) = player.zones.board.iter_mut().find(|c| c.card_id == attacker_id) {
                    attacker.status.attacks_this_turn += 1;
                    println!(
                        "[DEBUG] {} a attaqué {} fois ce tour (max: {})",
                        attacker.name,
                        attacker.status.attacks_this_turn,
                        attacker.max_attacks_per_turn()
                    );
                }
            }
        } else {
            // Aucun minion ne peut plus attaquer
            break;
        }
    }

    // --- Phase d'attaque du héros avec arme (inchangée) ---
    {
        let (can_attack, weapon_attack, weapon_health) = {
            let player = state.players.get(current_id).unwrap();
            if let Some(weapon) = &player.stats.weapon {
                if !player.hero_has_attacked {
                    (true, weapon.attack.unwrap_or(0), weapon.health.unwrap_or(0))
                } else {
                    (false, 0, 0)
                }
            } else {
                (false, 0, 0)
            }
        };

        if can_attack && weapon_attack > 0 && weapon_health > 0 {
            let target_is_hero = {
                let opponent = state.players.get(opponent_id).unwrap();
                opponent.zones.board.is_empty()
            };

            let (player, opponent) = {
                let player_ptr: *mut crate::game::player::Player = state.players.get_mut(current_id).unwrap() as *mut _;
                let opponent_ptr: *mut crate::game::player::Player = state.players.get_mut(opponent_id).unwrap() as *mut _;
                unsafe { (&mut *player_ptr, &mut *opponent_ptr) }
            };

            println!(
                "{} attaque avec son héros ({} ATK) {}",
                player.id_string(),
                weapon_attack,
                if target_is_hero { "le héros adverse" } else { "un serviteur adverse" }
            );

            if target_is_hero {
                opponent.take_damage(weapon_attack as i32);
            } else {
                let defender = &mut opponent.zones.board[0];
                defender.status.current_health = Some(defender.effective_health() - weapon_attack);
                player.take_damage(defender.effective_attack() as i32);

                println!(
                    "Le héros subit {} en retour (riposte du serviteur)",
                    defender.effective_attack()
                );

                if defender.status.current_health.unwrap_or(0) <= 0 {
                    println!("{} tue le serviteur adverse : {}", player.id_string(), defender.name);
                    opponent.zones.board.remove(0);
                }
            }

            if let Some(weapon) = &mut player.stats.weapon {
                weapon.health = weapon.health.map(|d| d.saturating_sub(1));
                println!("Durabilité de l'arme : {}", weapon.health.unwrap_or(0));
                if weapon.health == Some(0) {
                    println!("L’arme {} est détruite !", weapon.name);
                    let weapon_card = player.stats.weapon.take().unwrap();
                    player.zones.graveyard.push(weapon_card);
                }
            }

            player.hero_has_attacked = true;
        }
    }
}

// Effectue l'échange de coups entre deux minions, retourne :
// (attaquant est mort, défenseur est mort, dégâts infligés par l'attaquant, dégâts infligés par le défenseur)
pub fn perform_attack(attacker: &mut Card, defender: &mut Card) -> (bool, bool, i32, i32) {
    let attacker_attack = attacker.effective_attack();
    let defender_attack = defender.effective_attack();

    let defender_pv_avant = defender.effective_health();
    let attacker_pv_avant = attacker.effective_health();

    // --- Divine Shield DEFENSEUR ---
    let mut actual_damage_to_defender = attacker_attack;
    if defender.effects.contains(&Effect::DivineShield) && attacker_attack > 0 {
        println!(
            "{} est protégé par un Divine Shield (aucun dégât subi, bouclier perdu).",
            defender.name
        );
        defender.effects.retain(|e| *e != Effect::DivineShield);
        actual_damage_to_defender = 0;
    }

    // --- Divine Shield ATTAQUANT (riposte) ---
    let mut actual_damage_to_attacker = defender_attack;
    if attacker.effects.contains(&Effect::DivineShield) && defender_attack > 0 {
        println!(
            "{} est protégé par un Divine Shield (aucun dégât subi en riposte, bouclier perdu).",
            attacker.name
        );
        attacker.effects.retain(|e| *e != Effect::DivineShield);
        actual_damage_to_attacker = 0;
    }

    // --- Applique les dégâts restants (si pas de Divine Shield) ---
    attacker.status.current_health = Some(attacker.effective_health() - actual_damage_to_attacker);
    defender.status.current_health = Some(defender.effective_health() - actual_damage_to_defender);

    // --- POISONOUS ---
    let attacker_has_poison = attacker.effects.contains(&Effect::Poisonous);
    let defender_has_poison = defender.effects.contains(&Effect::Poisonous);

    if attacker_has_poison && actual_damage_to_defender > 0 && !defender.effects.contains(&Effect::DivineShield) {
        defender.status.current_health = Some(0);
        println!("{} tue instantanément {} grâce à Poisonous !", attacker.name, defender.name);
    }

    if defender_has_poison && actual_damage_to_attacker > 0 && !attacker.effects.contains(&Effect::DivineShield) {
        attacker.status.current_health = Some(0);
        println!("{} tue instantanément {} en riposte grâce à Poisonous !", defender.name, attacker.name);
    }

    attacker.status.has_attacked = true;
    attacker.effects.retain(|e| *e != Effect::Stealth);

    let attacker_dead = attacker.status.current_health.unwrap_or(0) <= 0;
    let defender_dead = defender.status.current_health.unwrap_or(0) <= 0;

    let damage_by_attacker = defender_pv_avant.min(actual_damage_to_defender).max(0);
    let damage_by_defender = attacker_pv_avant.min(actual_damage_to_attacker).max(0);

    (attacker_dead, defender_dead, damage_by_attacker, damage_by_defender)
}
