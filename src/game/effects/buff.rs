use super::Effect;
use crate::data::card_template::EffectTemplate;
use crate::game::targets::Target;
use crate::game::triggers::Trigger;

pub fn parse(template: &EffectTemplate) -> Effect {
    let attack = template.extra.get("attack").and_then(|v| v.as_i64()).map(|x| x as i32);
    let health = template.extra.get("health").and_then(|v| v.as_i64()).map(|x| x as i32);
    let target = template.extra.get("target").and_then(|v| v.as_str()).and_then(Target::from_str);

    Effect::Buff {
        attack,
        health,
        target,
        trigger: template.trigger.clone(),
    }
}
