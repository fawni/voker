use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Spell {
    Alacrity,
    ChaosMeteor,
    ColdSnap,
    DeafeningBlast,
    EMP,
    ForgeSpirit,
    GhostWalk,
    IceWall,
    SunStrike,
    Tornado,
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alacrity => write!(f, "Alacrity"),
            Self::ChaosMeteor => write!(f, "Chaos Meteor"),
            Self::ColdSnap => write!(f, "Cold Snap"),
            Self::DeafeningBlast => write!(f, "Deafening Blast"),
            Self::EMP => write!(f, "EMP"),
            Self::ForgeSpirit => write!(f, "Forge Spirit"),
            Self::GhostWalk => write!(f, "Ghost Walk"),
            Self::IceWall => write!(f, "Ice Wall"),
            Self::SunStrike => write!(f, "Sun Strike"),
            Self::Tornado => write!(f, "Tornado"),
        }
    }
}
