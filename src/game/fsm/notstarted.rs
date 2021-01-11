//! Initial state
//!
//! The purpose of this state is for players to join a game, and for it to be explained to them.

use super::*;
use crate::{
    game::roles::{Role, RoleHolder},
    helpers::perms,
};

use rand::{seq::SliceRandom, thread_rng};
use serenity::{
    framework::standard::CommandResult,
    model::{
        channel::ChannelType,
        id::{RoleId, UserId},
    },
};
use std::fmt;

/// Struct which represents the state of the game when it is not started
#[derive(Debug, Clone)]
pub(super) struct NotStarted {
    pub(super) joined_users: Vec<UserId>,
}

impl NotStarted {
    fn can_start(&self) -> bool {
        self.joined_users.len() == 6
    }
}

impl GameState for NotStarted {}
impl_wrap!(NotStarted);

impl GameMachine<NotStarted> {
    pub(super) async fn next(mut self, ctx: &Context) -> CommandResult<GameMachine<ABlock>> {
        assert!(self.state.can_start());

        use crate::game::roles::{King, Knight, Prince, Revolutionary, Sorcerer, TheDouble};
        let mut roles: Vec<RoleHolder> = vec![
            King.wrap(),
            Knight.wrap(),
            Prince.wrap(),
            Revolutionary.wrap(),
            Sorcerer.wrap(),
            TheDouble.wrap(),
        ];

        roles.shuffle(&mut thread_rng());

        // I'm a sucker for plot accuracy, these should be all
        let watch_colours = vec!["blue", "beige", "orange", "green", "black", "red"];
        let mut current_room: u8 = 1;

        let at_everyone_perms = perms::make_denied_override_for_role(RoleId {
            0: self.metadata.guild.0,
        });

        let rooms_category = self
            .metadata
            .guild
            .create_channel(ctx, |c| c.name("Rooms").kind(ChannelType::Category))
            .await?
            .id;

        let mut next = ABlock::new(BTreeMap::new(), 0, SubstitutionStatus::HasNot);

        for new_player in self.state.joined_users.iter_mut().zip(watch_colours.iter()) {
            let channel = self
                .metadata
                .guild
                .create_channel(ctx, |c| {
                    c.name(format!("room-{}", current_room))
                        .category(rooms_category)
                })
                .await?;

            next.players_mut().insert(
                *new_player.0,
                Player::new(
                    *new_player.0,
                    roles.remove(0),
                    channel.id,
                    new_player.1.to_string(),
                ),
            );

            self.metadata
                .guild
                .member(ctx, *new_player.0)
                .await?
                .add_role(ctx, self.metadata.player_role)
                .await?;

            channel.create_permission(ctx, &at_everyone_perms).await?;

            channel
                .create_permission(ctx, &perms::make_allowed_override_for_user(*new_player.0))
                .await?;

            channel.say(ctx, format!(r#"
You look around the room you see yourself in. You see a toilet and a washbowl, a table with a jute bag on top of it in the center of the room, and a 20-inch screen in the center of the room.

You reach inside the bag and take out one item after another.
A ball-point pen.
A memo book.
A {} digital watch.
Seven portions of solid food.
Some kind of a tablet.

And a heavy-duty knife.
            "#, new_player.1)).await?;

            current_room += 1;
        }

        Ok(GameMachine::<ABlock> {
            state: next,
            metadata: self.metadata,
        })
    }

    #[inline]
    pub(super) fn joined_users(&self) -> &Vec<UserId> {
        &self.state.joined_users
    }

    pub fn join(&mut self, id: UserId) -> JoinResult {
        if self.state.joined_users.len() < 6 {
            if id == self.metadata.host {
                Err(JoinError::YoureTheHost)
            } else if self.state.joined_users.contains(&id) {
                Err(JoinError::AlreadyIn)
            } else {
                self.state.joined_users.push(id);
                Ok(())
            }
        } else {
            Err(JoinError::GameFull)
        }
    }

    pub fn leave(&mut self, id: UserId) -> LeaveResult {
        if id == self.metadata.host {
            Err(LeaveError::YoureTheHost)
        } else if !self.state.joined_users.contains(&id) {
            Err(LeaveError::NotInAGame)
        } else {
            let mut user_idx = 7;
            for user in self.state.joined_users.iter().enumerate() {
                if *user.1 == id {
                    user_idx = user.0;
                }
            }
            assert!(user_idx < 7);
            self.state.joined_users.remove(user_idx);
            Ok(())
        }
    }
}

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type JoinResult = StdResult<(), JoinError>;
pub type LeaveResult = StdResult<(), LeaveError>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JoinError {
    GameFull,
    YoureTheHost,
    AlreadyIn,
    GameStarted,
}

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use JoinError::*;
        match self {
            GameFull => write!(f, "You can't join a full game"),
            YoureTheHost => write!(f, "You can't be both The Host, and a player"), // technically not following canon
            AlreadyIn => write!(f, "You can't join a game multiple times"),
            GameStarted => write!(f, "You can't join a game that started already"),
        }
    }
}

impl std::error::Error for JoinError {}

#[derive(Copy, Clone, Debug)]
pub enum LeaveError {
    NotInAGame,
    YoureTheHost,
    GameStarted,
}

impl fmt::Display for LeaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LeaveError::*;
        match self {
            NotInAGame => write!(f, "You can't leave a game if you're not in one"),
            YoureTheHost => write!(
                f,
                "You can't leave a game if you're The Host, why would you anyway?"
            ),
            GameStarted => write!(f, "You can't leave a game that started."),
        }
    }
}
