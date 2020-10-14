use crate::game::{player::Player, roles::RoleName};
use itertools::izip;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::model::{
    channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType},
    id::{ChannelId, GuildId, RoleId, UserId},
    Permissions,
};
use serenity::prelude::*;
use std::collections::HashMap;
use std::fmt;
use tracing::error;

type Host = UserId;
type StdResult<T, E> = std::result::Result<T, E>;
pub type Result = StdResult<(), Box<(dyn std::error::Error + Send + Sync)>>;

pub struct Game {
    guild: GuildId,
    meeting_room: ChannelId,
    player_role: RoleId,
    state: GameState,
    host: Host,
    players: HashMap<UserId, Player>, // 6
    joined_users: Vec<UserId>,        // only ever used in Pregame
    king_murder_target: UserId,
    day: u8,
}

impl Game {
    pub fn new(guild: GuildId, host: Host, meeting_room: ChannelId, player_role: RoleId) -> Self {
        Self {
            guild: guild,
            meeting_room: meeting_room,
            player_role: player_role,
            state: GameState::NotStarted,
            host: host,
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

    pub fn meeting_room(&self) -> ChannelId {
        self.meeting_room
    }

    pub fn player_role(&self) -> RoleId {
        self.player_role
    }
    pub async fn start_gathering(&mut self, ctx: &Context) -> Result {
        assert!([GameState::ABlock, GameState::CBlock].contains(&self.state));
        match self.state() {
            GameState::ABlock => {
                self.transition_to_next_state();
                self.open_meeting_room(ctx).await?;
            }
            GameState::CBlock => {
                self.transition_to_next_state();
                self.open_meeting_room(ctx).await?;
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    async fn open_meeting_room(&self, ctx: &Context) -> Result {
        self.meeting_room
            .create_permission(
                ctx,
                &PermissionOverwrite {
                    allow: Permissions::SEND_MESSAGES
                        | Permissions::READ_MESSAGES
                        | Permissions::READ_MESSAGE_HISTORY,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(self.player_role),
                },
            )
            .await?;
        Ok(())
    }

    pub fn can_start(&self) -> bool {
        self.joined_users.len() == 6
    }

    pub async fn start(&mut self, ctx: &Context) -> Result {
        assert!(self.can_start());

        use super::roles::{self, King, Knight, Prince, Revolutionary, Sorcerer, TheDouble};
        let mut roles: Vec<Box<(dyn roles::Role + Send + Sync)>> = vec![
            Box::new(King),
            Box::new(Knight),
            Box::new(Prince),
            Box::new(Revolutionary),
            Box::new(Sorcerer),
            Box::new(TheDouble),
        ];
        {
            let mut rng = thread_rng();
            roles.shuffle(&mut rng);
        }

        // I'm a sucker for plot accuracy, these should be all
        let watch_colours = vec!["blue", "beige", "orange", "green", "black", "red"];
        let mut current_room: u8 = 1;

        let at_everyone_perms = PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Member(UserId {
                0: *self.guild.as_u64(),
            }),
        };

        let rooms_category = self
            .guild
            .create_channel(ctx, |c| c.name("Rooms").kind(ChannelType::Category))
            .await?
            .id;

        for new_player in izip!(&mut self.joined_users, &watch_colours) {
            self.players.insert(
                *new_player.0,
                Player::new(*new_player.0, roles.remove(0), new_player.1.to_string()),
            );

            let channel = self
                .guild
                .create_channel(ctx, |c| {
                    c.name(format!("room-{}", current_room))
                        .category(rooms_category)
                })
                .await?;

            self.guild
                .member(ctx, *new_player.0)
                .await?
                .add_role(ctx, self.player_role)
                .await?;

            channel.create_permission(ctx, &at_everyone_perms).await?;

            channel
                .create_permission(
                    ctx,
                    &PermissionOverwrite {
                        allow: Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Member(*new_player.0),
                    },
                )
                .await?;

            channel.say(ctx, format!(r#"
You look around the room you see yourself in. You see a toilet and a washbowl, a table with a jute bag on top of it in the center of the room, and a 20-inch screen in the center of the room.

You reach inside the bag and take out one item after another.
A ball-point pen.
A memo book.
A {} digital watch.
Seven portions of solid food.
Some kind of a tablet.

And a heavy-dute knife.
            "#, new_player.1)).await?;

            current_room += 1;
        }

        Ok(())
    }

    pub async fn end(&mut self, ctx: &Context) -> Result {
        if !self.players.is_empty() {
            for player in self.players.iter() {
                self.guild
                    .member(ctx, player.0)
                    .await?
                    .remove_role(ctx, self.player_role)
                    .await?;
            }
        }

        Ok(())
    }

    pub fn joined_users(&self) -> &Vec<UserId> {
        &self.joined_users
    }

    pub fn transition_to_next_state(&mut self) {
        let all_alive_have_won = self.all_alive_have_won();

        match self.state {
            GameState::NotStarted => {
                error!("Function called before game started");
                panic!();
            }

            GameState::Pregame => todo!(),
            GameState::ABlock => {
                self.state = if all_alive_have_won {
                    GameState::GameEnded
                } else {
                    GameState::BBlock
                };
            }
            GameState::BBlock => {
                self.state = if all_alive_have_won {
                    GameState::GameEnded
                } else {
                    GameState::CBlock
                }
            }
            GameState::CBlock => {
                self.state = if all_alive_have_won {
                    GameState::GameEnded
                } else {
                    GameState::DBlock
                }
            }
            GameState::DBlock => {
                self.state = if all_alive_have_won {
                    GameState::GameEnded
                } else {
                    GameState::EBlock
                }
            }
            GameState::EBlock => {
                self.state = if all_alive_have_won {
                    GameState::GameEnded
                } else {
                    GameState::FBlock
                }
            }
            GameState::FBlock => {
                if all_alive_have_won {
                    self.state = GameState::GameEnded
                } else {
                    self.day += 1;
                    self.state = GameState::ABlock;
                }
            }
            GameState::GameEnded => {
                error!("Function called after game ended");
                panic!();
            }
        }
    }

    pub fn all_alive_have_won(&self) -> bool {
        for player in self.players.iter() {
            if !player.1.win_condition_achieved(self) {
                return false;
            }
        }

        true
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

type JoinResult = StdResult<(), JoinError>;

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

type LeaveResult = StdResult<(), LeaveError>;

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
