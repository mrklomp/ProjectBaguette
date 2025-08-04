// src/game/keywords.rs
use bitflags::bitflags;

/// Regroupe tous les mots-clé Hearthstone sous forme de bits.
bitflags! {
    #[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
    pub struct Keywords: u16 {
        const TAUNT         = 0b0000_0001;
        const CHARGE        = 0b0000_0010;
        const RUSH          = 0b0000_0100;
        const DIVINE_SHIELD = 0b0000_1000;
        const LIFESTEAL     = 0b0001_0000;
        const POISONOUS     = 0b0010_0000;
        const REBORN        = 0b0100_0000;
        const STEALTH        = 0b1000_0000;
        const WINDFURY        = 0b1000_0000;
        const MEGAWINDFURY    = 0b0001_0000_0000;

    }
}

impl Keywords {
    /// Construit les flags à partir du champ « mechanics » du JSON.
    pub fn from_mechanics(mechanics: &[String]) -> Self {
        mechanics.iter().fold(Keywords::empty(), |mut acc, m| {
            match m.as_str() {
                "Taunt"          => acc |= Keywords::TAUNT,
                "Charge"         => acc |= Keywords::CHARGE,
                "Rush"           => acc |= Keywords::RUSH,
                "Divine Shield"  => acc |= Keywords::DIVINE_SHIELD,
                "Lifesteal"      => acc |= Keywords::LIFESTEAL,
                "Poisonous"      => acc |= Keywords::POISONOUS,
                "Reborn"         => acc |= Keywords::REBORN,
                "Stealth"        => acc |= Keywords::STEALTH,
                "Windfury"       => acc |= Keywords::WINDFURY,
                "Mega-Windfury"  => acc |= Keywords::MEGAWINDFURY,
                _ => {}
            }
            acc
        })
    }

    /// Alias pratique : `card.has(Keywords::TAUNT)`.
    #[inline]
    pub fn has(self, other: Keywords) -> bool {
        self.intersects(other)
    }
}
