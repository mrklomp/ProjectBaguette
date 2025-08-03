fn main() {
    let templates = load_card_templates("cards/CORE2025.json").expect("Erreur chargement cards.json");
    let mut battlecry_damage_cards = Vec::new();
    for t in templates.values() {
        if let Some(effs) = &t.effects {
            for e in effs {
                if e.effect_type == "damage" {
                    if let Some(trigger) = e.extra.get("trigger") {
                        if trigger == "battlecry" {
                            battlecry_damage_cards.push(t.clone());
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("Pool battlecry_damage_cards len = {}", battlecry_damage_cards.len());
    for t in &battlecry_damage_cards {
        println!("- {} : {:?}", t.card_name, t.effects);
    }
    if battlecry_damage_cards.is_empty() {
        println!("Aucune carte battlecry damage trouvée !");
        return;
    }
    // ... ici tu construis tes decks avec cette pool comme dans la réponse d'avant ...
}
