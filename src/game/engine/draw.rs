use serde_json::Value;
use crate::game::player::Player;
use crate::game::enums::{SpellSchool, Races};
use std::collections::HashMap;
use crate::game::card::Card;

pub fn draw_card(player: &mut Player) {
    if let Some(card) = player.zones.deck.pop() {
        if player.zones.hand.len() < 10 {
            println!("{} pioche: {}", player.id_string(), card.name);
            player.zones.hand.push(card);
        } else {
            println!(
                "{} a une main pleine ! La carte {} est brûlée.",
                player.id_string(),
                card.name
            );
        }
    } else {
        // Gestion de la fatigue simple : -1 PV par carte qu'on ne peut plus piocher
        println!(
            "{} n'a plus de cartes dans son deck ! Fatigue : -1 PV",
            player.id_string()
        );
        player.stats.health -= 1;

        if player.stats.health <= 0 {
            println!("{} meurt de fatigue !", player.id_string());
        }
    }
}

pub fn draw_n(player: &mut Player, n: i32) -> Vec<crate::game::card::Card> {
    let mut drawn = Vec::new();
    for _ in 0..n {
        if let Some(card) = player.zones.deck.pop() {
            if player.zones.hand.len() < 10 {
                println!("{} pioche: {}", player.id_string(), card.name);
                player.zones.hand.push(card.clone());
                drawn.push(card);
            } else {
                println!(
                    "{} a une main pleine ! La carte {} est brûlée.",
                    player.id_string(),
                    card.name
                );
            }
        } else {
            // Gestion de la fatigue simple : -1 PV par carte qu'on ne peut plus piocher
            println!(
                "{} n'a plus de cartes dans son deck ! Fatigue : -1 PV",
                player.id_string()
            );
            player.stats.health -= 1;
            if player.stats.health <= 0 {
                println!("{} meurt de fatigue !", player.id_string());
            }
        }
    }
    drawn
}

pub fn draw_n_with_filter(player: &mut Player, n: i32, filter: HashMap<String, Value>) {
    let mut to_draw = Vec::new();

    for card in &player.zones.deck {
        if to_draw.len() as i32 >= n {
            break;
        }
        let mut matches = true;
        for (k, v) in &filter {
            match (k.as_str(), v) {
                ("spellschool", Value::String(school)) => {
                    matches &= card.spell_school
                        .as_ref()
                        .map_or(false, |ss| ss.to_string().eq_ignore_ascii_case(school));
                },
                // Remplace la branche ("races", Value::String(race)) => ... par ceci :
                ("races", Value::String(race)) => {
                    matches &= card.races
                        .as_ref()
                        .map_or(false, |vec| vec.iter().any(|r| r.to_string().eq_ignore_ascii_case(race)));
                },
                ("races", Value::Array(races_array)) => {
                    // Ici races_array est un Vec<Value> de Strings genre ["BEAST", "DRAGON"]
                    matches &= card.races.as_ref().map_or(false, |vec| {
                        races_array.iter().any(|filter_race| {
                            filter_race.as_str().map_or(false, |race_str| {
                                vec.iter().any(|r| r.to_string().eq_ignore_ascii_case(race_str))
                            })
                        })
                    });
                },

                ("cost", Value::Number(c)) => {
                    matches &= Some(card.cost) == Some(c.as_u64().unwrap_or(255) as u8);
                },
                _ => {}
            }
        }
        if matches {
            to_draw.push(card.card_id.clone());
        }
    }

    // Ensuite, pour chaque carte à piocher :
    for card_id in to_draw {
        // Trouver la carte dans le deck et la retirer
        if let Some(pos) = player.zones.deck.iter().position(|c| c.card_id == card_id) {
            let card = player.zones.deck.remove(pos);
            if player.zones.hand.len() < 10 {
                println!("{} pioche (filtré): {}", player.id_string(), card.name);
                player.zones.hand.push(card);
            } else {
                // Optionnel: brûler la carte si main pleine
                println!("Main pleine ! La carte {} est brûlée.", card.name);
            }
        }
    }
}

