#![allow(unused)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Player {
        Self {
            health: 100,
            mana: if level < 10 { None } else { Some(100) },
            level: level
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None
        }    
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                // self.health = self.health - min(self.health, mana_cost);
                self.health = self.health.saturating_sub(mana_cost);
                0
            },
            Some(mana) if mana_cost > mana => 0,
            Some(mana) => {
               self.mana = Some(mana.saturating_sub(mana_cost));
               mana_cost.checked_mul(2).expect("mana cost more than double!")
            },
        }
    }
}
