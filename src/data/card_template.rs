use serde::Deserialize;
use std::collections::HashMap;

use crate::game::card::{Card, CardStatus};
use crate::game::enums::{CardClass, CardType, Rarity, SpellSchool, Races};
use crate::game::effects::Effect;
use crate::game::triggers::{Trigger, TriggerDef};
use crate::game::keywords::Keywords;

#[derive(Debug, Deserialize, Clone)]
pub struct CardTemplate {
    pub card_id: String,
    pub card_name: String,
    pub card_class: CardClass,
    pub card_type: CardType,
    pub cost: Option<u8>,
    pub set: Option<String>,
    pub rarity: Option<Rarity>,
    pub collectible: Option<bool>,
    pub spell_school: Option<SpellSchool>,
    pub rune_cost: Option<HashMap<String, u8>>,
    pub attack: Option<i32>,
    pub health: Option<i32>,
    #[serde(default)]
    pub mechanics: Vec<String>,
    pub races: Option<Vec<Races>>,
    pub effects: Option<Vec<EffectTemplate>>,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EffectTemplate {
    #[serde(rename = "type")]
    pub effect_type: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
    pub trigger: Option<Trigger>,
}

// Fonction pour charger le fichier JSON
pub fn load_card_templates(path: &str) -> Result<HashMap<String, CardTemplate>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let cards: HashMap<String, CardTemplate> = serde_json::from_reader(reader)?;
    Ok(cards)
}


// ---------------------------------------------------------------------------
//  Conversion CardTemplate → Card
// ---------------------------------------------------------------------------
impl CardTemplate {
    pub fn to_card(&self) -> Card {
        // ① Convertit tous les EffectTemplate en Effect runtime
        let effects: Vec<Effect> = self
            .effects
            .as_ref()
            .map(|tpls| tpls.iter().map(Effect::from_template).collect())
            .unwrap_or_default();

        // ② Construit la liste des TriggerDef associés
        let mut triggers: Vec<TriggerDef> = Vec::new();
        if let Some(effect_tpls) = &self.effects {
            for (tpl, eff) in effect_tpls.iter().zip(effects.iter()) {
                if let Some(trig_kind) = &tpl.trigger {
                    triggers.push(TriggerDef {
                        when: trig_kind.clone(),
                        effect: eff.clone(),
                    });
                }
            }
        }


        // ④ Construit la carte finale
        Card {
            card_id: self.card_id.clone(),
            name: self.card_name.clone(),
            cost: self.cost.unwrap_or(0),
            card_type: self.card_type.clone(),
            attack: self.attack,
            health: self.health,
            max_health: self.health,
            text: self.text.clone(),
            card_class: self.card_class.clone(),
            tags: HashMap::new(),
            keywords: self.keywords(),     // <-- si tu as ajouté ce champ à Card
            status: CardStatus {
                current_health: self.health,
                attack_modifiers: 0,
                health_modifiers: 0,
                silenced: false,
                frozen: false,
                has_attacked: false,
                just_played: true,
                attacks_this_turn: 0,
            },
            effects: effects.clone(),
            native_effects: effects,
            spell_school: self.spell_school.clone(),
            races: self.races.clone(),       
            triggers,                      
        }
    }

    pub fn keywords(&self) -> Keywords {
        Keywords::from_mechanics(&self.mechanics)
    }
}
