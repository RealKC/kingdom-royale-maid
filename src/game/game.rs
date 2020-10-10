use crate::game::{player::Player, roles::RoleName};
use serenity::model::id::{GuildId, UserId};
use std::collections::HashMap;
use std::fmt;

type Host = UserId;

pub struct Game {
    guild: GuildId,
    state: GameState,
    host: Host,
    players: HashMap<UserId, Player>, // 6
    joined_users: Vec<UserId>,        // only ever used in Pregame
    king_murder_target: UserId,
    day: u8,
}

impl Game {
    pub fn new(guild_: GuildId, host_: Host) -> Self {
        Self {
            guild: guild_,
            state: GameState::Pregame,
            host: host_,
            players: Default::default(),
            joined_users: Default::default(),
            king_murder_target: Default::default(),
            day: 1,
        }
    }

    pub fn join(&mut self, id: UserId) -> JoinResult {
        if self.joined_users.len() < 6 {
            if id == self.host {
                Err(JoinError::YoureTheHost)
            } else if self.joined_users.contains(&id) {
                Err(JoinError::AlreadyIn)
            } else {
                self.joined_users.push(id);
                Ok(())
            }
        } else {
            Err(JoinError::GameFull)
        }
    }

    pub fn leave(&mut self, id: UserId) -> LeaveResult {
        if id == self.host {
            Err(LeaveError::YoureTheHost)
        } else if !self.joined_users.contains(&id) {
            Err(LeaveError::NotInAGame)
        } else {
            let mut user_idx = 7;
            for user in self.joined_users.iter().enumerate() {
                if *user.1 == id {
                    user_idx = user.0;
                }
            }
            assert!(user_idx < 7);
            self.joined_users.remove(user_idx);
            Ok(())
        }
    }

    pub fn can_start(&self) -> bool {
        self.players.len() == 6
    }

    pub fn joined_users(&self) -> &Vec<UserId> {
        &self.joined_users
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn players(&self) -> &HashMap<UserId, Player> {
        &self.players
    }

    pub fn host(&self) -> Host {
        self.host
    }

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

    pub fn is_prince_alive(&self) -> bool {
        self.is_alive(RoleName::Prince)
    }

    pub fn is_the_double_alive(&self) -> bool {
        self.is_alive(RoleName::TheDouble)
    }

    pub fn is_sorcerer_alive(&self) -> bool {
        self.is_alive(RoleName::Sorcerer)
    }

    pub fn is_revolutionary_alive(&self) -> bool {
        self.is_alive(RoleName::Revolutionary)
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

type JoinResult = Result<(), JoinError>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JoinError {
    GameFull,
    YoureTheHost,
    AlreadyIn,
}

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use JoinError::*;
        match self {
            GameFull => write!(f, ", you can't join a full game"),
            YoureTheHost => write!(f, ", you can't be both The Host, and a player"), // technically not following canon
            AlreadyIn => write!(f, ", you can't join a game multiple times"),
        }
    }
}

type LeaveResult = Result<(), LeaveError>;

#[derive(Copy, Clone, Debug)]
pub enum LeaveError {
    NotInAGame,
    YoureTheHost,
}

impl fmt::Display for LeaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LeaveError::*;
        match self {
            NotInAGame => write!(f, ", you can't leave a game if you're not in one"),
            YoureTheHost => write!(
                f,
                ", you can't leave a game if you're The Host, why would you anyway?"
            ),
        }
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
