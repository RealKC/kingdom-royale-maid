use serenity::{model::id::UserId, prelude::Mentionable};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DeathCause {
    Sorcery,
    Beheading,
    Assassination,
    Starvation,
    Stab(UserId),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SubstitutionStatus {
    HasNot,
    CurrentlyIs,
    Has,
}

impl fmt::Display for DeathCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeathCause::Sorcery => write!(f, "was burnt to a crisp using sorcery."),
            DeathCause::Beheading => write!(f, "was beheaded."),
            DeathCause::Assassination => write!(f, "was assassinated."),
            DeathCause::Starvation => write!(f, "became a mumy due to starvation."),
            DeathCause::Stab(id) => write!(f, "was stabbed by {}", id.mention()),
        }
    }
}
