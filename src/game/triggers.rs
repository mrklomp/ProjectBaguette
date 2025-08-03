use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    AfterHeroAttack,
    AfterPlay,
    AfterSelfAttack,
    AfterSummon,
    AfterAnyDamage,
    Battlecry,
    Combo,
    Deathrattle,
    EndOfTurn,
    StartOfTurn,
    EndOfEnemyTurn,
    StartOfEnemyTurn,
    OnAttack,
    OnFriendlyUndeadDeath,
    OnKill,
    OnSpellCast,
    Outcast,
    WhenDrawn,
}
