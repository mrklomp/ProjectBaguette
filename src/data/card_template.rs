use serde::Deserialize;
use std::collections::HashMap;
use crate::game::card::{Card, CardStatus};
use crate::game::enums::{CardClass, CardType, Rarity, SpellSchool, Races};
use crate::game::effects::Effect;
use crate::game::triggers::{TriggerDef, Trigger};
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

impl CardTemplate {
    pub fn to_card(&self) -> Card {
        let mut triggered_effects: HashMap<Trigger, Vec<Effect>> = HashMap::new();
        
        let effects: Vec<Effect> = self.effects.as_ref()
            .map(|effs| effs.iter().map(Effect::from_template).collect())
            .unwrap_or_default();

        if let Some(effs) = &self.effects {
            for eff in effs {
                let effect = Effect::from_template(eff);
                if let Some(trigger) = &eff.trigger {
                    triggered_effects.entry(trigger.clone()).or_default().push(effect);
                }
            }
        }


        let mut triggers: Vec<TriggerDef> = Vec::new();

        if let Some(effect_tpls) = &self.effects {
            for tpl_eff in effect_tpls {
                if let Some(trig_kind) = &tpl_eff.trigger {
                    let eff = Effect::from_template(tpl_eff);
                    triggers.push(TriggerDef { when: trig_kind.clone(), effect: eff });
                }
            }
        }

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
            keywords: self.keywords(),
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
            triggered_effects,
        }
    }

    pub fn keywords(&self) -> Keywords {
        Keywords::from_mechanics(&self.mechanics)
    }
}
