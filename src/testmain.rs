mod data;
mod game;
mod logger;

fn main() {
    let templates = data::card_template::load_card_templates("cards/CORE2025.json").unwrap();

    println!("OK, {} cartes chargées !", templates.len());

    for (id, card) in &templates {
        print!("{}: {}", id, card.card_name);
        match &card.effects {
            Some(effects) if !effects.is_empty() => println!(" | effects = {:?}", effects),
            _ => println!(" | effects = (aucun)"),
        }
    }
}
