use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CardType {
    Enchantment,
    Hero,
    HeroPower,
    Location,
    Minion,
    Spell,
    Weapon,
    #[serde(other)]
    Unknown,
}

impl CardType {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().replace('_', "").as_str() {
            "ENCHANTMENT" => CardType::Enchantment,
            "HERO" => CardType::Hero,
            "HEROPOWER" => CardType::HeroPower,
            "LOCATION" => CardType::Location,
            "MINION" => CardType::Minion,
            "SPELL" => CardType::Spell,
            "WEAPON" => CardType::Weapon,
            _ => CardType::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SpellSchool {
    Arcane,
    Fel,
    Fire,
    Frost,
    Holy,
    Nature,
    Shadow,
    #[serde(other)]
    Unknown,
}

impl SpellSchool {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().replace('_', "").as_str() {
            "ARCANE" => SpellSchool::Arcane,
            "FEL" => SpellSchool::Fel,
            "FIRE" => SpellSchool::Fire,
            "FROST" => SpellSchool::Frost,
            "HOLY" => SpellSchool::Holy,
            "NATURE" => SpellSchool::Nature,
            "SHADOW" => SpellSchool::Shadow,
            _ => SpellSchool::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Races {
    All,
    Beast,
    Demon,
    Draenei,
    Dragon,
    Elemental,
    Mechanical,
    Murloc,
    Naga,
    Pirate,
    Quilboar,
    Totem,
    Undead,
    #[serde(other)]
    Unknown,
}

impl Races {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().replace('_', "").as_str() {
            "ALL" => Races::All,
            "BEAST" => Races::Beast,
            "DEMON" => Races::Demon,
            "DRAENEI" => Races::Draenei,
            "DRAGON" => Races::Dragon,
            "ELEMENTAL" => Races::Elemental,
            "MECHANICAL" => Races::Mechanical,
            "MURLOC" => Races::Murloc,
            "NAGA" => Races::Naga,
            "PIRATE" => Races::Pirate,
            "QUILBOAR" => Races::Quilboar,
            "TOTEM" => Races::Totem,
            "UNDEAD" => Races::Undead,
            _ => Races::Unknown,
        }
    }
}

impl std::fmt::Display for Races {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Races::All => "ALL",
            Races::Beast => "BEAST",
            Races::Demon => "DEMON",
            Races::Draenei => "DRAENEI",
            Races::Dragon => "DRAGON",
            Races::Elemental => "ELEMENTAL",
            Races::Mechanical => "MECHANICAL",
            Races::Murloc => "MURLOC",
            Races::Naga => "NAGA",
            Races::Pirate => "PIRATE",
            Races::Quilboar => "QUILBOAR",
            Races::Totem => "TOTEM",
            Races::Undead => "UNDEAD",
            Races::Unknown => "UNKNOWN",
        };
        write!(f, "{s}")
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CardClass {
    Deathknight,
    Demonhunter,
    Dream,
    Druid,
    Hunter,
    Mage,
    Neutral,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
    Whizbang,
    #[serde(other)]
    Unknown,
}

impl CardClass {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().replace('_', "").as_str() {
            "DEATHKNIGHT" => CardClass::Deathknight,
            "DEMONHUNTER" => CardClass::Demonhunter,
            "DREAM" => CardClass::Dream,
            "DRUID" => CardClass::Druid,
            "HUNTER" => CardClass::Hunter,
            "MAGE" => CardClass::Mage,
            "NEUTRAL" => CardClass::Neutral,
            "PALADIN" => CardClass::Paladin,
            "PRIEST" => CardClass::Priest,
            "ROGUE" => CardClass::Rogue,
            "SHAMAN" => CardClass::Shaman,
            "WARLOCK" => CardClass::Warlock,
            "WARRIOR" => CardClass::Warrior,
            "WHIZBANG" => CardClass::Whizbang,
            _ => CardClass::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Rarity {
    Free,
    Common,
    Rare,
    Epic,
    Legendary,
    #[serde(other)]
    Unknown,
}

impl Rarity {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().replace('_', "").as_str() {
            "FREE" => Rarity::Free,
            "COMMON" => Rarity::Common,
            "RARE" => Rarity::Rare,
            "EPIC" => Rarity::Epic,
            "LEGENDARY" => Rarity::Legendary,
            _ => Rarity::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Zone {
    Hand,
    Board,
    Deck,
    Graveyard,
    Secret,
    SetAside,
    Dormant,
    Unknown, // Ajoute une variante Unknown pour l’erreur
}

impl Zone {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().replace('_', "").as_str() {
            "hand" => Zone::Hand,
            "board" => Zone::Board,
            "deck" => Zone::Deck,
            "graveyard" => Zone::Graveyard,
            "secret" => Zone::Secret,
            "setaside" => Zone::SetAside,
            "dormant" => Zone::Dormant,
            _ => Zone::Unknown,
        }
    }
}

impl fmt::Display for SpellSchool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}