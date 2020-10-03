use crate::game::{player::Player, roles::RoleName};
use serenity::model::id::{GuildId, UserId};
use std::collections::HashMap;

type Host = UserId;

pub struct Game {
    guild: GuildId,
    state: GameState,
    host: Host,
    players: HashMap<UserId, Player>, // 6
    king_murder_target: UserId,
    day: u8,
}

impl Game {
    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn kill(&mut self, target: UserId, _killed_by: KilledBy) {
        let player = self.players.get_mut(&target).unwrap();

        player.set_dead();
    }

    pub fn set_king_murder_target(&mut self, target: &Player) {
        self.king_murder_target = target.id();
    }

    pub fn king_murder_target(&mut self) -> &mut Player {
        self.players.get_mut(&self.king_murder_target).unwrap()
    }

    pub fn is_king_alive(&self) -> bool {
        self.is_alive(RoleName::King)
    }

    pub fn is_the_double_alive(&self) -> bool {
        self.is_alive(RoleName::TheDouble)
    }

    pub fn is_sorcerer_alive(&self) -> bool {
        self.is_alive(RoleName::Sorcerer)
    }

    fn is_alive(&self, role: RoleName) -> bool {
        for player in self.players.iter() {
            if player.1.role_name() == role {
                return player.1.is_alive();
            }
        }

        unreachable!("There should always be a {:?} in the game", role)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KilledBy {
    Sorcerer,
    Knight,
    Revolutionary,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    NotStarted, // Recruiting phase and stuff
    Pregame,    // for giving players an introduction to the game _in character_
    // Blocks taken from the timetable in the book
    ABlock, // break, standby in own room
    BBlock, // Gathering in the big room, "First meeting"
    CBlock, // Secret meeting partner selection & meeting with them, King, Sorcerer, Knight can act, someone might die during this block
    DBlock, // Gathering in the big room, "Second meeting"
    EBlock, // Dinner, no food => death, Revolutionary can act
    FBlock, // Sleep & Break, is this useful?
    GameEnded,
}
