use crate::game::{player::Player, roles::RoleName};
use crate::helpers::{
    choose_target::build_embed_for_target_choice,
    confirm_murder::build_embed_for_murder_confirmation, react::react_with,
};
use itertools::izip;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::model::{
    channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType},
    id::{ChannelId, GuildId, RoleId, UserId},
    Permissions,
};
use serenity::prelude::*;
use std::collections::BTreeMap;
use std::fmt;
use tracing::error;

type Host = UserId;
type StdResult<T, E> = std::result::Result<T, E>;
pub type Result = StdResult<(), Box<(dyn std::error::Error + Send + Sync)>>;

pub struct Game {
    guild: GuildId,
    meeting_room: ChannelId,
    announcement_channel: ChannelId,
    player_role: RoleId,
    state: GameState,
    host: Host,
    players: BTreeMap<UserId, Player>, // 6
    joined_users: Vec<UserId>,         // only ever used in Pregame
    king_murder_target: UserId,
    day: u8,
}

impl Game {
    pub fn new(
        guild: GuildId,
        host: Host,
        meeting_room: ChannelId,
        announcement_channel: ChannelId,
        player_role: RoleId,
    ) -> Self {
        Self {
            guild,
            meeting_room,
            announcement_channel,
            player_role,
            state: GameState::NotStarted,
            host,
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

    pub fn announcement_channel(&self) -> ChannelId {
        self.announcement_channel
    }

    pub fn player_role(&self) -> RoleId {
        self.player_role
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

    async fn close_meeting_room(&self, ctx: &Context) -> Result {
        self.meeting_room
            .create_permission(
                ctx,
                &PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::SEND_MESSAGES
                        | Permissions::READ_MESSAGES
                        | Permissions::READ_MESSAGE_HISTORY,
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
            let channel = self
                .guild
                .create_channel(ctx, |c| {
                    c.name(format!("room-{}", current_room))
                        .category(rooms_category)
                })
                .await?;

            self.players.insert(
                *new_player.0,
                Player::new(
                    *new_player.0,
                    roles.remove(0),
                    channel.id,
                    new_player.1.to_string(),
                ),
            );

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

    pub async fn transition_to_next_state(&mut self, ctx: &Context) -> Result {
        let all_alive_have_won = self.all_alive_have_won();

        match self.state {
            GameState::NotStarted => {
                error!("Function called before game started");
                panic!();
            }

            GameState::ABlock => {
                if all_alive_have_won {
                    self.state = GameState::GameEnded;
                } else {
                    self.state = GameState::BBlock;
                };

                self.open_meeting_room(ctx).await?;
            }
            GameState::BBlock => {
                if all_alive_have_won {
                    self.state = GameState::GameEnded;
                } else {
                    self.state = GameState::CBlock;
                }

                self.close_meeting_room(ctx).await?;
            }
            GameState::CBlock => {
                if all_alive_have_won {
                    self.state = GameState::GameEnded;
                } else {
                    self.state = GameState::DBlock;
                };
                self.select_secret_meeting_partners(ctx).await?;
                self.make_king_select_target(ctx).await?;
                self.make_assistant_choose(ctx).await?;
            }
            GameState::DBlock => {
                if all_alive_have_won {
                    self.state = GameState::GameEnded;
                } else {
                    self.state = GameState::EBlock;
                }

                self.open_meeting_room(ctx).await?;
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
        };

        Ok(())
    }

    pub async fn select_secret_meeting_partners(&mut self, ctx: &Context) -> Result {
        let rooms = self
            .players
            .iter()
            .map(|player| (*player.0, player.1.room()))
            .collect::<Vec<_>>();

        for user_and_room in rooms {
            let embed = build_embed_for_target_choice(
                ctx,
                &self.players.keys().map(|k| *k).collect::<Vec<_>>(),
                "Please select a partner for your secret meeting",
            )
            .await?;

            let msg = user_and_room
                .1
                .send_message(ctx, |m| m.set_embed(embed))
                .await?;

            static REACTIONS: [&str; 6] = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣"];
            react_with(ctx, &msg, &REACTIONS).await?;

            if let Some(reaction) = msg
                .await_reaction(&ctx)
                .author_id(user_and_room.0)
                .channel_id(user_and_room.1)
                .filter(|r| REACTIONS.contains(&r.emoji.to_string().as_str()))
                .await
            {
                let emoji = reaction.as_inner_ref().emoji.to_string();
                if let Ok(idx) = REACTIONS.binary_search(&emoji.as_str()) {
                    let id = self.players.keys().nth(idx).map(|o| *o);
                    match id {
                        Some(id) => {
                            self.players
                                .get_mut(&user_and_room.0)
                                .unwrap()
                                .set_secret_meeting_partner(id);
                        }
                        None => {
                            error!("Got a wrong reaction somehow");
                            panic!();
                        }
                    }
                }

                return Ok(());
            }
        }

        Err("Probably an error to arrive here".into())
    }

    pub async fn make_king_select_target(&mut self, ctx: &Context) -> Result {
        let king = {
            // May not necessarily be the king proper, but the next one in the hierarchy
            // Hierarchy: King -(dies)-> The Double -(dies)-> Prince

            let mut candidates = vec![];
            for player in &self.players {
                if player.1.role_name().is_king_like() {
                    candidates.push(player);
                }
            }

            candidates.sort_by(|a, b| {
                fn map_to_int(role: RoleName) -> u8 {
                    match role {
                        RoleName::King => 100,
                        RoleName::TheDouble => 50,
                        RoleName::Prince => 25,
                        _ => unreachable!(),
                    }
                };

                map_to_int(a.1.role_name()).cmp(&map_to_int(b.1.role_name()))
            });

            let mut res = None;
            for candidate in candidates {
                let player = candidate.1;
                if player.is_alive() {
                    res = Some(*candidate.0);
                }
            }
            res
        };

        if king.is_none() {
            return Ok(()); // I *think* this shouldn't happen as no nobility => someone won, already
        }

        let king = king.unwrap();

        if !self.is_sorcerer_alive() || !self.is_knight_alive() {
            self.players
                .get(&king)
                .unwrap()
                .room()
                .say(
                    ctx,
                    "You cannot ask the dead to commit murder for you. Maybe pick up that knife?",
                )
                .await?;
            return Ok(());
        }

        let embed = build_embed_for_target_choice(
            ctx,
            &self.players.keys().map(|k| *k).collect::<Vec<_>>(),
            "Please select a target for 「 Murder 」",
        )
        .await?;

        let msg = self
            .players
            .get(&king)
            .unwrap()
            .room()
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;

        static REACTIONS: [&str; 6] = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣"];
        react_with(ctx, &msg, &REACTIONS).await?;

        if let Some(reaction) = msg
            .await_reaction(&ctx)
            .author_id(self.players.get(&king).unwrap().id())
            .channel_id(self.players.get(&king).unwrap().room())
            .filter(|r| REACTIONS.contains(&r.emoji.to_string().as_str()))
            .await
        {
            let emoji = reaction.as_inner_ref().emoji.to_string();
            if let Ok(idx) = REACTIONS.binary_search(&emoji.as_str()) {
                let id = self.players.keys().nth(idx).map(|o| *o);
                match id {
                    Some(id) => {
                        self.king_murder_target = id;
                    }
                    None => {
                        error!("Got a wrong reaction somehow");
                        panic!();
                    }
                }
            }

            return Ok(());
        }

        Err("Probably an error to arrive here".into())
    }

    pub async fn make_assistant_choose(&mut self, ctx: &Context) -> Result {
        let embed =
            build_embed_for_murder_confirmation(ctx, self.king_murder_target, self.guild).await?;

        let sorc_or_knight = {
            let mut res = None;

            for player in &self.players {
                if player.1.is_alive()
                    && [RoleName::Knight, RoleName::Sorcerer].contains(&player.1.role_name())
                {
                    res = Some(player.0);
                    break;
                }
            }

            res
        };

        if sorc_or_knight.is_none() {
            return Ok(());
        }

        let sorc_or_knight = sorc_or_knight.unwrap();

        let msg = self
            .players
            .get(sorc_or_knight)
            .unwrap()
            .room()
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;

        static REACTIONS: [&str; 2] = ["🇾", "🇳"];
        react_with(ctx, &msg, &REACTIONS).await?;

        if let Some(reaction) = msg
            .await_reaction(&ctx)
            .filter(|r| REACTIONS.contains(&r.emoji.to_string().as_str()))
            .guild_id(self.guild)
            .author_id(*sorc_or_knight)
            .await
        {
            let emoji = reaction.as_inner_ref().emoji.to_string();
            if emoji.as_str() == REACTIONS[0] {
                let target = self.players.get_mut(&self.king_murder_target).unwrap();
                target.set_dead(target.role_name().into());
            }
            return Ok(());
        }

        Err("Reaching here is probably a bug".into())
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

    pub fn players(&self) -> &BTreeMap<UserId, Player> {
        &self.players
    }

    pub fn host(&self) -> Host {
        self.host
    }

    pub fn guild(&self) -> GuildId {
        self.guild
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn king_murder_target(&self) -> &Player {
        self.players.get(&self.king_murder_target).unwrap()
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

    pub fn is_knight_alive(&self) -> bool {
        self.is_alive(RoleName::Knight)
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
pub enum DeathCause {
    Sorcery,
    Beheading,
    Assassination,
    Starvation,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    NotStarted, // Recruiting phase and stuff
    // Blocks taken from the timetable in the book
    ABlock, // break, standby in own room
    BBlock, // Gathering in the big room, "First meeting"
    CBlock, // Secret meeting partner selection & meeting with them, King, Sorcerer, Knight can act, someone might die during this block
    DBlock, // Gathering in the big room, "Second meeting"
    EBlock, // Dinner, no food => death, Revolutionary can act
    FBlock, // Sleep & Break, is this useful?
    GameEnded,
}
