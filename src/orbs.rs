use std::collections::VecDeque;

use crate::spells::Spell;

#[derive(Debug)]
pub enum Orb {
    Quas,
    Wex,
    Exort,
}

#[derive(Debug)]
pub struct Orbs(VecDeque<Option<Orb>>);

impl Orbs {
    pub fn new() -> Self {
        Self(VecDeque::from([None, None, None]))
    }

    pub fn push(&mut self, orb: Orb) {
        self.0.pop_front();
        self.0.push_back(Some(orb));
    }

    pub fn invoke(&self) -> Option<Spell> {
        if self.0.len() != 3 || self.0.iter().any(Option::is_none) {
            return None;
        }

        let orbs = self.0.iter().flatten().fold((0, 0, 0), |mut acc, orb| {
            match orb {
                Orb::Quas => acc.0 += 1,
                Orb::Wex => acc.1 += 1,
                Orb::Exort => acc.2 += 1,
            }

            acc
        });

        match orbs {
            (3, 0, 0) => Some(Spell::ColdSnap),
            (0, 3, 0) => Some(Spell::EMP),
            (0, 0, 3) => Some(Spell::SunStrike),
            (2, 1, 0) => Some(Spell::GhostWalk),
            (2, 0, 1) => Some(Spell::IceWall),
            (1, 2, 0) => Some(Spell::Tornado),
            (0, 2, 1) => Some(Spell::Alacrity),
            (1, 0, 2) => Some(Spell::ForgeSpirit),
            (0, 1, 2) => Some(Spell::ChaosMeteor),
            (1, 1, 1) => Some(Spell::DeafeningBlast),
            _ => None,
        }
    }
}
