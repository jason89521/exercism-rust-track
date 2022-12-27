// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        if self.level >= 10 {
            Some(Player {
                health: 100,
                mana: Some(100),
                ..*self
            })
        } else {
            Some(Player {
                health: 100,
                mana: None,
                ..*self
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if (mana_cost > mana) {
                    0
                } else {
                    self.mana.insert(mana - mana_cost);
                    mana_cost * 2
                }
            }
            None => {
                match self.health.checked_sub(mana_cost) {
                    Some(new_health) => self.health = new_health,
                    None => self.health = 0,
                }
                0
            }
        }
    }
}
