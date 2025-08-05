use std::collections::HashMap;
use crate::game::player::Player;

use crate::{
    data::card_template::CardTemplate,
    game::{
        card::Card,
        engine::{
            choose::Chooser,
            events::dispatch_events,
            utils::{minion_stats_string, IdString},
        },
        event::GameEvent,
        keywords::Keywords,
        state::{GameState, PlayerId},
    },
};

// ===========================================================================
// API publique
// ===========================================================================
pub fn perform_attack_phase(
    state: &mut GameState,
    current: &PlayerId,
    opponent: &PlayerId,
    _chooser: &dyn Chooser,
    _templates: &HashMap<String, CardTemplate>,
) {
    // ─────────────────────────────── attaques des serviteurs
    let mut guard = 0;
    loop {
        guard += 1;
        if guard > 5000 {
            println!("⚠️ guard ATTACK_PHASE stop après 50 itérations");
            break;
        }

        let attacker_id = match find_next_attacker(state, current) {
            Some(id) => id,
            None => break,
        };

        let defender_id = choose_defender(state, opponent);

        // Skip Rush sans cible
        if rush_cannot_attack(state, current, &attacker_id, defender_id.as_deref()) {
            continue;
        }

        match defender_id {
            Some(def_id) => fight_minion(state, current, opponent, &attacker_id, &def_id),
            None => attack_hero(state, current, opponent, &attacker_id),
        }
    }

    // ─────────────────────────────── arme du héros
    hero_weapon_attack(state, current, opponent);
}

// ===========================================================================
// Sélection attaquant / défenseur
// ===========================================================================
fn find_next_attacker(state: &GameState, pid: &PlayerId) -> Option<String> {
    state.players[pid]
        .zones
        .board
        .iter()
        .find(|m| {
            if m.status.attacks_this_turn >= m.max_attacks_per_turn() {
                false
            } else if m.status.just_played {
                m.has_kw(Keywords::CHARGE) || m.has_kw(Keywords::RUSH)
            } else {
                true
            }
        })
        .map(|m| m.card_id.clone())
}

fn choose_defender(state: &GameState, opp_id: &PlayerId) -> Option<String> {
    let opp_board = &state.players[opp_id].zones.board;

    if let Some(t) = opp_board
        .iter()
        .find(|m| m.has_kw(Keywords::TAUNT) && !m.has_kw(Keywords::STEALTH))
    {
        return Some(t.card_id.clone());
    }

    opp_board
        .iter()
        .find(|m| !m.has_kw(Keywords::STEALTH))
        .map(|m| m.card_id.clone())
}

fn rush_cannot_attack(
    state: &mut GameState,
    pid: &PlayerId,
    attacker_id: &str,
    defender_opt: Option<&str>,
) -> bool {
    let attacker = state.players[pid]
        .zones
        .board
        .iter()
        .find(|m| m.card_id == attacker_id)
        .unwrap();

    if attacker.has_kw(Keywords::RUSH)
        && !attacker.has_kw(Keywords::CHARGE)
        && attacker.status.just_played
        && defender_opt.is_none()
    {
        state.players.get_mut(pid).unwrap().zones.board
            .iter_mut()
            .find(|m| m.card_id == attacker_id)
            .unwrap()
            .status
            .attacks_this_turn += 1;
        true
    } else {
        false
    }
}

// ===========================================================================
// Combat serviteur ↔ serviteur
// ===========================================================================
/// combat entre deux serviteurs déjà identifiés
fn fight_minion(
    state: &mut GameState,
    current: &PlayerId,
    opponent: &PlayerId,
    attacker_id: &str,
    defender_id: &str,
) {
    // ── indices sur les boards (pas d’emprunt mutable ici)
    let att_idx = index_of(state, current, attacker_id);
    let def_idx = index_of(state, opponent, defender_id);

    // ── log
    println!(
        "{} attaque {}",
        minion_stats_string(&state.players[current].zones.board[att_idx]),
        minion_stats_string(&state.players[opponent].zones.board[def_idx])
    );

    // ── bloc mutable simultané (unsafe contrôlé)
    let (att_dead, def_dead, dmg_att, dmg_def, att_ls, def_ls) = {
        let current_ptr  = state.players.get_mut(current).unwrap()  as *mut Player;
        let opponent_ptr = state.players.get_mut(opponent).unwrap() as *mut Player;
        let (cur_mut, opp_mut) = unsafe { (&mut *current_ptr, &mut *opponent_ptr) };

        let attacker = &mut cur_mut.zones.board[att_idx];
        let defender = &mut opp_mut.zones.board[def_idx];

        // Capture des flags Lifesteal AVANT le combat
        let att_ls_local = attacker.has_kw(Keywords::LIFESTEAL);
        let def_ls_local = defender.has_kw(Keywords::LIFESTEAL);

        // Combat
        let (a_dead, d_dead, dmg_to_def, dmg_to_att) = perform_attack(attacker, defender);

        // Compter l'attaque tant qu'on a encore &mut attacker
        attacker.status.attacks_this_turn += 1;

        (a_dead, d_dead, dmg_to_def, dmg_to_att, att_ls_local, def_ls_local)
    };

    // Lifesteal (héros) uniquement si présent et si dégâts > 0
    if att_ls && dmg_att > 0 {
        state.players.get_mut(current).unwrap().heal(dmg_att);
    }
    if def_ls && dmg_def > 0 {
        state.players.get_mut(opponent).unwrap().heal(dmg_def);
    }

    // Morts / Reborn / Deathrattles via file d’événements
    handle_dead(
        state,
        current,
        opponent,
        att_dead.then_some(attacker_id.to_string()),
        def_dead.then_some(defender_id.to_string()),
    );

}

// ===========================================================================
// Combat serviteur ↦ héros
// ===========================================================================
fn attack_hero(
    state: &mut GameState,
    current: &PlayerId,
    opponent: &PlayerId,
    attacker_id: &str,
) {
    let att_idx = index_of(state, current, attacker_id);

    // On fait tout ce qui modifie l'attaquant + le héros adverse
    let (dmg, attacker_dead, att_ls) = {
        let current_ptr = state.players.get_mut(current).unwrap() as *mut Player;
        let opponent_ptr = state.players.get_mut(opponent).unwrap() as *mut Player;
        let (cur_mut, opp_mut) = unsafe { (&mut *current_ptr, &mut *opponent_ptr) };

        let attacker = &mut cur_mut.zones.board[att_idx];

        // Rush sans Charge : pas d'attaque héros
        if attacker.status.just_played
            && attacker.has_kw(Keywords::RUSH)
            && !attacker.has_kw(Keywords::CHARGE)
        {
            attacker.status.attacks_this_turn += 1;
            return;
        }

        let att_ls = attacker.has_kw(Keywords::LIFESTEAL);
        let dmg = attacker.effective_attack();

        opp_mut.take_damage(dmg as i32);

        // maj PV de l'attaquant (éventuelle riposte future non applicable ici)
        attacker.status.current_health = Some(attacker.effective_health());

        // très important : compter l'attaque pendant qu'on a encore &mut attacker
        attacker.status.attacks_this_turn += 1;

        (dmg, attacker.status.current_health.unwrap_or(0) <= 0, att_ls)
    };

    println!(
        "{} [{}|{}] attaque le héros adverse ({:?}) pour {}",
        state.players[current].zones.board[att_idx].name,
        state.players[current].zones.board[att_idx].effective_attack(),
        state.players[current].zones.board[att_idx].effective_health(),
        opponent,
        dmg
    );

    // Lifesteal uniquement si présent
    if att_ls && dmg > 0 {
        state.players.get_mut(current).unwrap().heal(dmg);
    }

    // Mort potentielle de l'attaquant
    handle_dead(
        state,
        current,
        opponent,
        attacker_dead.then_some(attacker_id.to_string()),
        None,
    );
}


// ===========================================================================
// Héros + arme
// ===========================================================================
fn hero_weapon_attack(state: &mut GameState, current: &PlayerId, opponent: &PlayerId) {
    // infos de base (aucun emprunt mutable)
    let (can_attack, w_atk, w_dur, target_is_hero) = {
        let p = &state.players[current];
        let can = matches!(&p.stats.weapon, Some(w) if !p.hero_has_attacked);
        let atk = p.stats.weapon.as_ref().map(|w| w.attack.unwrap_or(0)).unwrap_or(0);
        let dur = p.stats.weapon.as_ref().map(|w| w.health.unwrap_or(0)).unwrap_or(0);
        let hero = state.players[opponent].zones.board.is_empty();
        (can, atk, dur, hero)
    };
    if !can_attack || w_atk == 0 || w_dur == 0 {
        return;
    }

    // ── bloc 1 : dégâts infligés ---------------------------------------
let retaliation = {
    let current_ptr = state.players.get_mut(current).unwrap() as *mut Player;
    let opponent_ptr = state.players.get_mut(opponent).unwrap() as *mut Player;
    let (cur_mut, opp_mut) = unsafe { (&mut *current_ptr, &mut *opponent_ptr) };

    if target_is_hero {
        opp_mut.take_damage(w_atk as i32);
        0
    } else {
        // appliquer les dégâts de l'arme au serviteur
        let def = &mut opp_mut.zones.board[0];
        def.status.current_health = Some(def.effective_health() - w_atk);

        // riposte calculée AVANT suppression
        let retaliation = def.effective_attack();

        // retirer si mort
        if def.status.current_health.unwrap_or(0) <= 0 {
            opp_mut.zones.board.remove(0);
        }

        retaliation
    }
};

    // ── bloc 2 : riposte + durabilité ---------------------------------
    if retaliation > 0 {
        state.players.get_mut(current).unwrap().take_damage(retaliation as i32);
    }

    let player = state.players.get_mut(current).unwrap();
    if let Some(w) = &mut player.stats.weapon {
        w.health = w.health.map(|d| d.saturating_sub(1));
        if w.health == Some(0) {
            player.zones.graveyard.push(player.stats.weapon.take().unwrap());
        }
    }
    player.hero_has_attacked = true;
}

// ===========================================================================
// Gestion des morts
// ===========================================================================
fn handle_dead(
    state: &mut GameState,
    current: &PlayerId,
    opponent: &PlayerId,
    attacker_dead: Option<String>,
    defender_dead: Option<String>,
) {
    // ─── Attaquant mort ────────────────────────────────────────────────
    if let Some(id) = attacker_dead {
        reborn_pass(state, current);                       // Reborn d’abord
        // 1. retire du board
        state.players
            .get_mut(current)
            .unwrap()
            .zones
            .board
            .retain(|m| m.card_id != id);
        // 2. pousse l’évènement une fois que la carte n’existe plus
        state.event_queue.push_back(GameEvent::MinionDied {
            card_id: id,
            owner: *current,
        });
    }

    // ─── Défenseur mort ────────────────────────────────────────────────
    if let Some(id) = defender_dead {
        reborn_pass(state, opponent);
        state.players
            .get_mut(opponent)
            .unwrap()
            .zones
            .board
            .retain(|m| m.card_id != id);
        state.event_queue.push_back(GameEvent::MinionDied {
            card_id: id,
            owner: *opponent,
        });
    }

    // dispatch une fois la file remplie
    dispatch_events(state);
}

fn reborn_pass(state: &mut GameState, pid: &PlayerId) {
    for m in &mut state.players.get_mut(pid).unwrap().zones.board {
        if m.status.current_health.unwrap_or(0) <= 0 && m.has_kw(Keywords::REBORN) {
            println!("{} revient en vie grâce à Reborn !", m.name);
            m.remove_kw(Keywords::REBORN);
            m.status.current_health = Some(1);
        }
    }
}


// ===========================================================================
// Combat élémentaire
// ===========================================================================
pub fn perform_attack(attacker: &mut Card, defender: &mut Card) -> (bool, bool, i32, i32) {
    let att_atk = attacker.effective_attack();
    let def_atk = defender.effective_attack();

    // Divine Shield
    let mut dmg_def = att_atk;
    if defender.has_kw(Keywords::DIVINE_SHIELD) && att_atk > 0 {
        defender.remove_kw(Keywords::DIVINE_SHIELD);
        dmg_def = 0;
    }
    let mut dmg_att = def_atk;
    if attacker.has_kw(Keywords::DIVINE_SHIELD) && def_atk > 0 {
        attacker.remove_kw(Keywords::DIVINE_SHIELD);
        dmg_att = 0;
    }

    attacker.status.current_health = Some(attacker.effective_health() - dmg_att);
    defender.status.current_health = Some(defender.effective_health() - dmg_def);

    // Poisonous
    if attacker.has_kw(Keywords::POISONOUS) && dmg_def > 0 && !defender.has_kw(Keywords::DIVINE_SHIELD) {
        defender.status.current_health = Some(0);
    }
    if defender.has_kw(Keywords::POISONOUS) && dmg_att > 0 && !attacker.has_kw(Keywords::DIVINE_SHIELD) {
        attacker.status.current_health = Some(0);
    }

    attacker.status.has_attacked = true;
    attacker.remove_kw(Keywords::STEALTH);

    let att_dead = attacker.status.current_health.unwrap_or(0) <= 0;
    let def_dead = defender.status.current_health.unwrap_or(0) <= 0;

    (att_dead, def_dead, dmg_def.max(0), dmg_att.max(0))
}

// ===========================================================================
// Utilitaire d’index
// ===========================================================================
fn index_of(state: &GameState, pid: &PlayerId, card_id: &str) -> usize {
    state.players[pid]
        .zones
        .board
        .iter()
        .position(|m| m.card_id == card_id)
        .expect("card should exist on board")
}
