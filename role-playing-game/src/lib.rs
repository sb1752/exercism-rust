use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                level: self.level,
                mana: if self.level >= 10 { Some(100) } else { None },
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = self.health - min(self.health, mana_cost);
                0
            }
            Some(mana) if mana < mana_cost => 0,
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                2 * mana_cost
            }
        }
    }
}
