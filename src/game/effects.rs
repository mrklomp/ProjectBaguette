use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::game::triggers::Trigger;
use crate::game::engine::utils::{ChooseRandomMut, IdString};
use crate::data::card_template::{EffectTemplate,CardTemplate};
use crate::game::state::{GameState, PlayerId};
use crate::game::targets::Target;
use crate::game::engine::draw::{draw_n, draw_n_with_filter};
use crate::game::enums::{CardType, Rarity, Races};
use crate::game::card::Card;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Effect {

    Spellpower { amount: Option<i32> },

    Damage {
        amount: Option<i32>,
        repeat: Option<i32>,
        target: Option<Target>,
        random: Option<bool>,
        trigger: Option<String>,
        condition: Option<EffectCondition>,
    },
    Heal {
        amount: Option<i32>,
        target: Option<Target>,
        trigger: Option<String>,
    },
    Draw {
        amount: Option<i32>,
        target: Option<Target>,
        filter: Option<HashMap<String, serde_json::Value>>,
        trigger: Option<String>,
    },
    Buff {
        attack: Option<i32>,
        health: Option<i32>,
        amount: Option<i32>,
        random: Option<bool>,
        duration: Option<String>,
        filter: Option<Value>,
        target: Option<Target>,
        trigger: Option<Trigger>,
    },
    Summon {
        amount: Option<i32>,
        card_id: Option<String>,
        filter: Option<std::collections::HashMap<String, serde_json::Value>>,
        source: Option<String>,
        destination: Option<String>,
        random: Option<bool>,
        trigger: Option<String>,
    },
    AddCardToHand {
        amount: Option<i32>,
        card_id: Option<String>,
        card: Option<String>,
        random: Option<bool>,
        source_pool: Option<serde_json::Value>,
        zone: Option<String>,
        trigger: Option<String>,
    },
    Discover {
        amount: Option<i32>,
        source_pool: Option<serde_json::Value>,
        condition: Option<HashMap<String, serde_json::Value>>,
        trigger: Option<String>,
    },
    Aura {
        effect: Box<serde_json::Value>,
        target: Option<String>,
    },
    GrantDeathrattle {
        effect: Box<serde_json::Value>,
        target: Option<String>,
        random: Option<bool>,
        trigger: Option<String>,
    },
    GrantMechanic {
        mechanic: String,
        target: Option<String>,
        trigger: Option<String>,
    },
    Destroy {
        target: Option<String>,
        condition: Option<HashMap<String, serde_json::Value>>,
        trigger: Option<String>,
    },
    SetHealth {
        value: Option<i32>,
        target: Option<String>,
        trigger: Option<String>,
    },
    Overload { amount: Option<i32> },
    GainArmor {
        amount: Option<i32>,
        trigger: Option<String>,
    },
    EquipWeapon {
        card_id: Option<String>,
        trigger: Option<String>,
    },
    Tradeable,
    CantAttack,
    SwapStats {
        target: Option<String>,
        trigger: Option<String>,
    },
    ReturnToHand {
        target: Option<String>,
        zone: Option<String>,
        trigger: Option<String>,
    },
    CopyCardToHand {
        from: Option<serde_json::Value>,
        to: Option<serde_json::Value>,
        card_type: Option<String>,
        amount: Option<i32>,
        trigger: Option<String>,
    },
    Discard {
        amount: Option<i32>,
        random: Option<bool>,
        trigger: Option<String>,
    },
    ModifyCost {
        mode: Option<String>,
        amount: Option<i32>,
        target: Option<String>,
        filter: Option<HashMap<String, serde_json::Value>>,
        value: Option<i32>,
        duration: Option<String>,
        condition: Option<HashMap<String, serde_json::Value>>,
        trigger: Option<String>,
    },
    Elusive,
    #[serde(other)]
    Unknown,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectCondition {
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl Effect {
    pub fn from_template(template: &EffectTemplate) -> Self {
        match template.effect_type.to_ascii_lowercase().as_str() {           
            "taunt" | "charge" | "rush" | "divine_shield" | "lifesteal"
            | "poisonous" | "reborn" | "stealth" | "windfury" | "megawindfury" => {
                return Effect::Unknown
            }
            "add_card_to_hand" => Effect::AddCardToHand {
                amount: None,
                card_id: None,
                card: None,
                random: None,
                source_pool: None,
                zone: None,
                trigger: None,
            },
            "banish_temporarily" => Effect::Unknown,
            "buff" => {
    let attack   = template.extra.get("attack").and_then(|v| v.as_i64()).map(|x| x as i32);
    let health   = template.extra.get("health").and_then(|v| v.as_i64()).map(|x| x as i32);
    let amount   = template.extra.get("amount").and_then(|v| v.as_i64()).map(|x| x as i32);
    let random   = template.extra.get("random").and_then(|v| v.as_bool());
    let duration = template.extra.get("duration").and_then(|v| v.as_str()).map(|s| s.to_string());
    let filter   = template.extra.get("filter").cloned();
    let target   = template
        .extra
        .get("target")
        .and_then(|v| v.as_str())
        .and_then(|s| crate::game::targets::Target::from_str(s));

    Effect::Buff {
        attack,
        health,
        amount,
        random,
        duration,
        filter,
        target,
        trigger: template.trigger.clone(),
    }
}
            "choose" => Effect::Unknown,
            "copy_card_to_hand" => Effect::CopyCardToHand {
                from: None,
                to: None,
                card_type: None,
                amount: None,
                trigger: None,
            },
            "damage" => {
                let target_str = template.extra.get("target").and_then(|v| v.as_str());
                let parsed_target = target_str.and_then(Target::from_str);
                Effect::Damage {
                    amount: template.extra.get("amount").and_then(|v| v.as_i64().map(|x| x as i32)),
                    repeat: template.extra.get("repeat").and_then(|v| v.as_i64().map(|x| x as i32)),
                    target: parsed_target,
                    random: template.extra.get("random").and_then(|v| v.as_bool()),
                    trigger: template.extra.get("trigger").and_then(|v| v.as_str().map(|s| s.to_string())),
                    condition: None,
                }
            }
            "destroy" => Effect::Destroy {
                target: None,
                condition: None,
                trigger: None,
            },
            "discard" => Effect::Discard {
                amount: None,
                random: None,
                trigger: None,
            },
            "discover" => Effect::Discover {
                amount: None,
                source_pool: None,
                condition: None,
                trigger: None,
            },
            "discover_mystery" => Effect::Unknown,
            "draw" => {
                let target_str = template.extra.get("target").and_then(|v| v.as_str());
                let parsed_target = target_str.and_then(Target::from_str);

                Effect::Draw {
                    amount: template.extra.get("amount").and_then(|v| v.as_i64().map(|x| x as i32)),
                    target: parsed_target,
                    filter: template.extra.get("filter").and_then(|v| v.as_object().map(|m| m.clone().into_iter().collect())),
                    trigger: template.extra.get("trigger").and_then(|v| v.as_str().map(|s| s.to_string())),
                }
            },

            "freeze" => Effect::Unknown,
            "gain_corpse" => Effect::Unknown,
            "gain_health" => Effect::Unknown,
            "grant_mechanic" => Effect::GrantMechanic {
                mechanic: "".to_string(),
                target: None,
                trigger: None,
            },
            "heal" => {
                let target_str = template.extra.get("target").and_then(|v| v.as_str());
                let parsed_target = target_str.and_then(Target::from_str);
                Effect::Heal {
                    amount: template.extra.get("amount").and_then(|v| v.as_i64().map(|x| x as i32)),
                    target: parsed_target,
                    trigger: template.extra.get("trigger").and_then(|v| v.as_str().map(|s| s.to_string())),
                }
            },
            "modify_cost" => Effect::ModifyCost {
                mode: None,
                amount: None,
                target: None,
                filter: None,
                value: None,
                duration: None,
                condition: None,
                trigger: None,
            },
            "replay_cards" => Effect::Unknown,
            "return_to_hand" => Effect::ReturnToHand {
                target: None,
                zone: None,
                trigger: None,
            },
            "set_health" => Effect::SetHealth {
                value: None,
                target: None,
                trigger: None,
            },
            "silence" => Effect::Unknown,
            "spend_corpse" => Effect::Unknown,
            "summon" => {
                Effect::Summon {
                    amount: template.extra.get("amount").and_then(|v| v.as_i64().map(|x| x as i32)),
                    card_id: template.extra.get("card_id").and_then(|v| v.as_str().map(|s| s.to_string())),
                    filter: template.extra.get("filter").and_then(|v| v.as_object().map(|m| m.clone().into_iter().collect())),
                    source: template.extra.get("source").and_then(|v| v.as_str().map(|s| s.to_string())),
                    destination: template.extra.get("destination")
                        .or_else(|| template.extra.get("zone"))
                        .or_else(|| template.extra.get("target"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                    random: template.extra.get("random").and_then(|v| v.as_bool()),
                    trigger: template.extra.get("trigger").and_then(|v| v.as_str().map(|s| s.to_string())),
                }

            },
            "swap_stats" => Effect::SwapStats {
                target: None,
                trigger: None,
            },
            "trigger_deathrattle" => Effect::Unknown,
            _ => Effect::Unknown,
        }
    }
}
/// Enlève tous les serviteurs morts sur le board de chaque joueur.
pub fn remove_dead_minions(state: &mut GameState) {
    for player in state.players.values_mut() {
        let before = player.zones.board.len();
        player.zones.board.retain(|m| m.status.current_health.unwrap_or(1) > 0);
        let after = player.zones.board.len();
        if before != after {
            println!(
                "[CLEANUP] {}/{} minions retirés du board de {:?} (morts)",
                before - after, before, player.id
            );
        }
    }
}

fn card_has_any_race(card: &crate::game::card::Card, races: &[Races]) -> bool {
    match &card.races {
        Some(card_races) => card_races.iter().any(|r| races.contains(r)),
        None => false,
    }
}

fn apply_buff_values(
    card: &mut crate::game::card::Card,
    add_atk: i32,
    add_hp: i32,
) {
    if add_atk != 0 {
        card.status.attack_modifiers += add_atk;
    }
    if add_hp != 0 {
        // max_health et current_health sont Option<i32>; on les traite prudemment
        let cur = card.status.current_health.unwrap_or(0);
        let maxh = card.max_health.unwrap_or(card.health.unwrap_or(0));
        card.max_health = Some(maxh + add_hp);
        card.status.current_health = Some(cur + add_hp);
    }
}

pub fn apply_effect(
    state: &mut GameState,
    player_id: &PlayerId,
    effect: &Effect,
    chooser: &dyn crate::game::engine::choose::Chooser,
    card_templates: &HashMap<String, CardTemplate>,
) {
    let owner_id = player_id;  // ou player_id, selon le vrai nom
    match effect {
        // ---- DAMAGE ----
        Effect::Damage { amount: Some(dmg), repeat, target: Some(target), .. } => {
            let times = repeat.unwrap_or(1);
            for _ in 0..times {
                match target {
                    Target::AnyCharacter => {
                        let mut valid = Vec::new();
                        for (pid, player) in state.players.iter() {
                            valid.push((pid.clone(), "hero".to_string(), None));
                            for (i, _) in player.zones.board.iter().enumerate() {
                                valid.push((pid.clone(), "minion".to_string(), Some(i)));
                            }
                        }
                        let choice = chooser.choose(
                            state,
                            crate::game::engine::choose::Choice::Target {
                                valid_targets: (0..valid.len()).collect(),
                            }
                        );
                        if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                            let (tgt_pid, tgt_type, opt_idx) = &valid[valid_targets[0]];
                            let tgt_player = state.players.get_mut(tgt_pid).unwrap();
                            if tgt_type == "hero" {
                                tgt_player.stats.health -= *dmg;
                                println!("[DAMAGE] Damage {} to hero {:?}", dmg, tgt_pid);
                            } else if let Some(idx) = opt_idx {
                                if let Some(minion) = tgt_player.zones.board.get_mut(*idx) {
                                    minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) - *dmg);
                                    println!("[DAMAGE] Damage {} to minion {}", dmg, minion.name);
                                }
                            }
                        }
                    }

                    Target::AnyMinion => {
                        let mut valid = Vec::new();
                        for (pid, player) in state.players.iter() {
                            for (i, _) in player.zones.board.iter().enumerate() {
                                valid.push((pid.clone(), i));
                            }
                        }
                        let choice = chooser.choose(
                            state,
                            crate::game::engine::choose::Choice::Target {
                                valid_targets: (0..valid.len()).collect(),
                            }
                        );
                        if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                            let (tgt_pid, idx) = &valid[valid_targets[0]];
                            let tgt_player = state.players.get_mut(tgt_pid).unwrap();
                            if let Some(minion) = tgt_player.zones.board.get_mut(*idx) {
                                minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) - *dmg);
                                println!("[DAMAGE] Damage {} to minion {}", dmg, minion.name);
                            }
                        }
                    }

                    Target::EnemyCharacter => {
                        let opponent_id = player_id.opponent();
                        let opponent = state.players.get(&opponent_id).unwrap();
                        let mut valid = Vec::new();
                        valid.push(("hero", None));
                        for (i, _) in opponent.zones.board.iter().enumerate() {
                            valid.push(("minion", Some(i)));
                        }
                        let choice = chooser.choose(
                            state,
                            crate::game::engine::choose::Choice::Target {
                                valid_targets: (0..valid.len()).collect(),
                            }
                        );
                        if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                            let (kind, opt_idx) = &valid[valid_targets[0]];
                            let opponent = state.players.get_mut(&opponent_id).unwrap();
                            if *kind == "hero" {
                                opponent.stats.health -= *dmg;
                                println!("[DAMAGE] Damage {} to ENEMY hero", dmg);
                            } else if let Some(idx) = opt_idx {
                                if let Some(minion) = opponent.zones.board.get_mut(*idx) {
                                    minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) - *dmg);
                                    println!("[DAMAGE] Damage {} to ENEMY minion {}", dmg, minion.name);
                                }
                            }
                        }
                    }

                    Target::FriendlyCharacter => {
                        let player = state.players.get(player_id).unwrap();
                        let mut valid = Vec::new();
                        valid.push(("hero", None));
                        for (i, _) in player.zones.board.iter().enumerate() {
                            valid.push(("minion", Some(i)));
                        }
                        let choice = chooser.choose(
                            state,
                            crate::game::engine::choose::Choice::Target {
                                valid_targets: (0..valid.len()).collect(),
                            }
                        );
                        if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                            let (kind, opt_idx) = &valid[valid_targets[0]];
                            let player = state.players.get_mut(player_id).unwrap();
                            if *kind == "hero" {
                                player.stats.health -= *dmg;
                                println!("[DAMAGE] Damage {} to FRIENDLY hero", dmg);
                            } else if let Some(idx) = opt_idx {
                                if let Some(minion) = player.zones.board.get_mut(*idx) {
                                    minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) - *dmg);
                                    println!("[DAMAGE] Damage {} to FRIENDLY minion {}", dmg, minion.name);
                                }
                            }
                        }
                    }

                    Target::AllEnemyCharacter => {
                        let opponent_id = player_id.opponent();
                        if let Some(opponent) = state.players.get_mut(&opponent_id) {
                            // Minions adverses
                            for minion in opponent.zones.board.iter_mut() {
                                minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) - *dmg);
                                println!("[DAMAGE] Damage {} to ENEMY minion {}", dmg, minion.name);
                            }
                            // Héros adverse
                            opponent.stats.health -= *dmg;
                            println!("[DAMAGE] Damage {} to ENEMY hero", dmg);
                        }
                    }

                    Target::FriendlyMinion => {
                        let player = state.players.get(player_id).unwrap();
                        let valid: Vec<_> = player.zones.board.iter().enumerate().map(|(i, _)| i).collect();
                        let choice = chooser.choose(
                            state,
                            crate::game::engine::choose::Choice::Target {
                                valid_targets: valid.clone(),
                            }
                        );
                        if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                            let idx = valid_targets[0];
                            let player = state.players.get_mut(player_id).unwrap();
                            if let Some(minion) = player.zones.board.get_mut(idx) {
                                minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) - *dmg);
                                println!("[DAMAGE] Damage {} to FRIENDLY minion {}", dmg, minion.name);
                            }
                        }
                    }

                    Target::EnemyMinion => {
                        let opponent_id = player_id.opponent();
                        let opponent = state.players.get(&opponent_id).unwrap();
                        let valid: Vec<_> = opponent.zones.board.iter().enumerate().map(|(i, _)| i).collect();

                        if valid.is_empty() {
                            println!("[DAMAGE] Pas de minion ennemi valide pour cibler");
                            return;
                        }

                        let choice = chooser.choose(
                            state,
                            crate::game::engine::choose::Choice::Target {
                                valid_targets: valid.clone(),
                            }
                        );

                        if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                            if valid_targets.is_empty() {
                                println!("[DAMAGE] Aucun target valide choisi");
                                return;
                            }
                            let idx = valid_targets[0];
                            let opponent = state.players.get_mut(&opponent_id).unwrap();
                            if let Some(minion) = opponent.zones.board.get_mut(idx) {
                                minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) - *dmg);
                                println!("[DAMAGE] Damage {} to ENEMY minion {}", dmg, minion.name);
                            }
                        }
                    }


                    _ => {
                        // Garde ta logique précédente pour les autres cibles (EnemyHero, AllEnemyMinion, etc)
                    }
                }
                // === SUPPRESSION GÉNÉRIQUE DES MORTS APRÈS CHAQUE DÉGÂT ===
                remove_dead_minions(state);
            }
        }

        // ---- HEAL ----
        Effect::Heal { amount: Some(heal), target: Some(target), .. } => {
            match target {
                Target::FriendlyHero => {
                    if let Some(player) = state.players.get_mut(player_id) {
                        player.heal(*heal);
                        println!("[HEAL] {} soigne son héros de {} PV (nouveaux PV : {})", player_id.id_string(), heal, player.stats.health);
                    }
                }
                Target::FriendlyCharacter => {
                    let player = state.players.get(player_id).unwrap();
                    let mut valid = Vec::new();
                    valid.push(("hero", None));
                    for (i, _) in player.zones.board.iter().enumerate() {
                        valid.push(("minion", Some(i)));
                    }
                    let choice = chooser.choose(
                        state,
                        crate::game::engine::choose::Choice::Target { valid_targets: (0..valid.len()).collect() }
                    );
                    if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                        let (kind, opt_idx) = &valid[valid_targets[0]];
                        let player = state.players.get_mut(player_id).unwrap();
                        if *kind == "hero" {
                            player.heal(*heal);
                            println!("[HEAL] {} soigne son héros de {} PV (nouveaux PV : {})", player_id.id_string(), heal, player.stats.health);
                        } else if let Some(idx) = opt_idx {
                            if let Some(minion) = player.zones.board.get_mut(*idx) {
                                minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) + *heal);
                                println!("[HEAL] {} soigne {} de {} PV (nouveaux PV : {})", player_id.id_string(), minion.name, heal, minion.status.current_health.unwrap_or(0));
                            }
                        }
                    }
                }
                Target::AnyCharacter => {
                    let mut valid = Vec::new();
                    for (pid, player) in state.players.iter() {
                        valid.push((pid.clone(), "hero".to_string(), None));
                        for (i, _) in player.zones.board.iter().enumerate() {
                            valid.push((pid.clone(), "minion".to_string(), Some(i)));
                        }
                    }
                    let choice = chooser.choose(
                        state,
                        crate::game::engine::choose::Choice::Target { valid_targets: (0..valid.len()).collect() }
                    );
                    if let crate::game::engine::choose::Choice::Target { valid_targets } = choice {
                        let (tgt_pid, tgt_type, opt_idx) = &valid[valid_targets[0]];
                        let tgt_player = state.players.get_mut(tgt_pid).unwrap();
                        if tgt_type == "hero" {
                            tgt_player.heal(*heal);
                            println!("[HEAL] Heal {} to hero {:?}", heal, tgt_pid);
                        } else if let Some(idx) = opt_idx {
                            if let Some(minion) = tgt_player.zones.board.get_mut(*idx) {
                                minion.status.current_health = Some(minion.status.current_health.unwrap_or(0) + *heal);
                                println!("[HEAL] Heal {} to minion {}", heal, minion.name);
                            }
                        }
                    }
                }
                _ => println!("[HEAL] Target '{:?}' non encore géré pour Heal", target),
            }
        }

        // ---- DRAW ----
        Effect::Draw { amount, filter, target, .. } => {
            let amt = amount.unwrap_or(1);
            if let Some(f) = filter {
                if let Some(player) = state.players.get_mut(player_id) {
                    println!("[DRAW] {} va piocher {} carte(s) filtrée(s) ({:?})", player_id.id_string(), amt, f);
                    draw_n_with_filter(player, amt, f.clone());
                }
            } else if let Some(tgt) = target {
                match tgt {
                    Target::FriendlyHero => {
                        if let Some(player) = state.players.get_mut(player_id) {
                            println!("[DRAW] {} va piocher {} carte(s)", player_id.id_string(), amt);
                            draw_n(player, amt);
                        }
                    }
                    Target::EnemyHero => {
                        let opp_id = player_id.opponent();
                        if let Some(opponent) = state.players.get_mut(&opp_id) {
                            println!("[DRAW] {} va piocher {} carte(s)", opp_id.id_string(), amt);
                            draw_n(opponent, amt);
                        }
                    }
                    _ => println!("[DRAW] Target '{:?}' non géré dans Draw", tgt),
                }
            } else {
                if let Some(player) = state.players.get_mut(player_id) {
                    println!("[DRAW] {} va piocher {} carte(s)", player_id.id_string(), amt);
                    draw_n(player, amt);
                }
            }
        }

        // ---- BUFF ----
        Effect::Buff {
    attack,
    health,
    amount,
    random: _,
    duration: _,
    filter: _,
    target,
    trigger: _,
} => {
    // ΔATK et ΔPV (fallback simple : "amount" = bonus d'ATK si "attack" absent)
    let add_atk = (*attack).or(*amount).unwrap_or(0);
    let add_hp  = (*health).unwrap_or(0);

    println!(
        "[BUFF] owner={:?} target={:?} atk:+{} hp:+{} amount={:?}",
        owner_id, target, add_atk, add_hp, amount
    );

    // petit helper local pour appliquer le buff à un serviteur
    fn apply_buff_to(minion: &mut Card, add_atk: i32, add_hp: i32) {
        if add_atk != 0 {
            minion.status.attack_modifiers += add_atk;
            let before = crate::game::engine::utils::minion_stats_string(minion);

            // ATTAQUE
            minion.status.attack_modifiers += add_atk;

            // VIE (courante + max si présent)
            if add_hp != 0 {
                if let Some(h) = minion.status.current_health {
                    minion.status.current_health = Some(h + add_hp);
                }
                if let Some(mh) = minion.max_health {
                    minion.max_health = Some(mh + add_hp);
                }
            }

            let after = crate::game::engine::utils::minion_stats_string(minion);
            println!(
                "[BUFF→APPLIED] {}  ==>  {}   ({}; +{}/+{})",
                before, after, minion.name, add_atk, add_hp
            );

        }
        if add_hp != 0 {
            // met à jour max, base et PV courants de façon cohérente
            let prev_max = minion.max_health.unwrap_or(minion.health.unwrap_or(0));
            minion.max_health = Some(prev_max + add_hp);

            let prev_base = minion.health.unwrap_or(prev_max);
            minion.health = Some(prev_base + add_hp);

            let prev_cur = minion.status.current_health.unwrap_or(prev_base);
            minion.status.current_health = Some(prev_cur + add_hp);
        }
    }

    match target {
        // Tous les serviteurs alliés
        Some(Target::AllFriendlyMinion) => {
            let p = state.players.get_mut(owner_id).unwrap();
            for m in &mut p.zones.board {
                apply_buff_to(m, add_atk, add_hp);
            }
        }
        // Tous les serviteurs (alliés + ennemis)
        Some(Target::AllMinion) => {
            for pid in [*owner_id, owner_id.opponent()] {
                let p = state.players.get_mut(&pid).unwrap();
                for m in &mut p.zones.board {
                    apply_buff_to(m, add_atk, add_hp);
                }
            }
        }
        // Un serviteur allié (simplifié : on prend le premier)
        Some(Target::FriendlyMinion) | Some(Target::SelfTarget) => {
            let p = state.players.get_mut(owner_id).unwrap();
            if let Some(m) = p.zones.board.first_mut() {
                apply_buff_to(m, add_atk, add_hp);
            }
        }
        // Par défaut : pas encore géré
        other => {
            println!("[BUFF] cible {:?} non gérée pour l’instant (ΔATK={}, ΔPV={})", other, add_atk, add_hp);
        }
    }
}

        // ---- SUMMON ----

        Effect::Summon {
    amount,
    card_id,
    filter,
    source,
    destination,
    random,
    trigger: _,
} => {
    let how_many = amount.unwrap_or(1);
    // Par défaut, on cible le board du joueur courant, sauf si destination explicite
    let dest_player_id = match destination.as_deref() {
        Some("opponent_board") => player_id.opponent(),
        // Ajoute d'autres cas spécifiques si besoin
        _ => *player_id,
    };

    // Cas 1 : Summon d'un ou plusieurs minions par card_id (token classique, Bear, Nerubian, etc)
    if let Some(cid) = card_id {
        for _ in 0..how_many {
            // À adapter selon ta fonction de création de carte/token !
            let template = card_templates.get(cid.as_str()).unwrap();

            let mut new_minion = template.to_card();

            // Tu veux probablement push dans le board du dest_player_id
            let dest_player = state.players.get_mut(&dest_player_id).unwrap();
            dest_player.zones.board.push(new_minion);
            println!("[SUMMON] {} invoque {}", dest_player_id.id_string(), cid);
        }
        return;
    }

    // Cas 2 : Summon depuis une main/zone (Dirty Rat…)
    if let (Some(src), Some(dest), Some(is_random)) = (source.as_ref(), destination.as_ref(), random) {
        if src == "hand" && dest == "opponent_board" && *is_random {
            let opponent_id = player_id.opponent();
            let opponent = state.players.get_mut(&opponent_id).unwrap();
            if !opponent.zones.hand.is_empty() {
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                let idx = (0..opponent.zones.hand.len()).collect::<Vec<_>>()
                    .choose(&mut rng).copied().unwrap_or(0);
                let mut minion = opponent.zones.hand.remove(idx);
                println!("[SUMMON] Dirty Rat sort {} de la main adverse et le pose sur le board !", minion.name);
                opponent.zones.board.push(minion);
            }
            return;
        }
        // Ajoute d'autres cas si besoin (main du joueur courant, etc)
    }

    // Cas 3 : Summon via filter (Sneed’s, Maze Guide…)
    if let Some(filt) = filter {
        // Ex: filtre par type minion et rareté légendaire (à adapter selon filt)
        let pool: Vec<_> = card_templates.values()
            .filter(|tpl| {
                tpl.card_type == CardType::Minion
                    && tpl.rarity == Some(Rarity::Legendary)
            })
            .collect();

        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        if let Some(template) = pool.choose(&mut rng) {
            let mut new_minion = template.to_card();
            println!(
                "[SUMMON] {} invoque {} ({})",
                dest_player_id.id_string(),
                template.card_name,          // ou template.card_name selon ton struct
                template.card_id
            );

            let dest_player = state.players.get_mut(&dest_player_id).unwrap();
            dest_player.zones.board.push(new_minion);
        }
     return;
}


    // Cas 4 : Summon une copie de soi-même (self_copy)
    if let Some(target) = destination.as_ref().or(card_id.as_ref()) {
        if target == "self_copy" {
            // Cherche le minion sur le board du joueur courant
            if let Some(player) = state.players.get_mut(player_id) {
                if let Some(myself) = player.zones.board.last().cloned() {
                    for _ in 0..how_many {
                        let mut copy = myself.clone();
                        // Reset les status nécessaires
                        copy.status.current_health = copy.max_health;
                        player.zones.board.push(copy);
                    }
                    println!("[SUMMON] {} se clone ({})", myself.name, how_many);
                }
            }
            return;
        }
    }

    // TODO: gérer d’autres variantes exotiques si besoin.
}


                // ---- AUTRES ----
                _ => {
                    println!("[APPLY] Effet non encore géré : {:?}", effect);
                }
            }
        }
