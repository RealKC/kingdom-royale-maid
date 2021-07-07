use std::{fmt::Debug, unimplemented};

use super::{
    db,
    fsm::TimeBlock,
    item::{Count, Item, Items, Note},
    roles::{RoleHolder, RoleName},
    DeathCause,
};
use futures::TryStreamExt;
use serenity::{
    builder::CreateEmbed,
    framework::standard::CommandResult,
    model::id::{ChannelId, GuildId, UserId},
    prelude::*,
};
use sqlx::PgPool;
use tracing::{info, instrument};

pub type SecretMeeting = Option<(UserId, ChannelId)>;

#[derive(Clone)]
pub struct Player {
    id: UserId,
    guild_id: GuildId,
    role: RoleHolder,
    alive: bool,
    room: ChannelId,
}

impl Player {
    pub fn new(
        id: UserId,
        guild_id: GuildId,
        role: RoleHolder,
        room: ChannelId,
        _watch_colour: String,
    ) -> Self {
        // PONDER: We may want to allow disabling certain items
        //         If we do, how should that be handled? Should we just pass a reference to the Game and ask it for enabled items?
        Self {
            id,
            guild_id,
            role,
            room,
            alive: true,
        }
    }

    pub fn from_db(
        id: UserId,
        guild_id: GuildId,
        role: db::Role,
        room: ChannelId,
        alive: bool,
    ) -> Self {
        Self {
            id,
            guild_id,
            role: role.into(),
            alive,
            room,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn room(&self) -> ChannelId {
        self.room
    }

    #[instrument]
    pub async fn secret_meeting_partner_on(&self, day: u8, pool: &PgPool) -> Option<UserId> {
        sqlx::query!(
            r#"
SELECT visitor
FROM public.secret_meetings
WHERE day = $1 AND game_id = $2 AND host = $3 
    "#,
            day as i32,
            self.guild_id.0 as i64,
            self.id.0 as i64
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            info!("secret_meeting_partner_on: {:?}", e);
            e
        })
        .ok()?
        .map(|row| UserId(row.visitor as u64))
    }

    pub async fn get_secret_meetings_for_day(
        &self,
        day: u8,
        pool: &PgPool,
    ) -> Option<(SecretMeeting, SecretMeeting)> {
        let mut results = sqlx::query!(
            r#"
SELECT visitor, channel_id
FROM public.secret_meetings
WHERE day = $1 AND host = $2
        "#,
            day as i32,
            self.id.0 as i64
        )
        .fetch(pool);

        let mut meetings = vec![];
        while let Ok(Some(result)) = results.try_next().await {
            meetings.push((
                UserId(result.visitor as u64),
                ChannelId(result.channel_id as u64),
            ));
        }

        debug_assert!(meetings.len() == 2);

        if meetings.is_empty() {
            None
        } else {
            Some((meetings.get(0).copied(), meetings.get(1).copied()))
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub async fn set_dead(
        &mut self,
        cause: DeathCause,
        ctx: &Context,
        channel: ChannelId,
    ) -> CommandResult {
        self.alive = false;

        channel
            .say(ctx, format!("{} {}", self.id.mention(), cause))
            .await?;
        Ok(())
    }

    /// Function meant to be only used inside test functions
    pub fn set_dead_mock(&mut self) {
        self.alive = false;
    }

    pub async fn add_item(&mut self, item: Item, pool: &PgPool) -> CommandResult {
        self.items(pool).await?.add_item(item, pool).await
    }

    /// This method adds more one of `item_name` to this player's inventory
    pub async fn add_one_more_item(&self, _item_name: &str, _pool: &PgPool) -> CommandResult {
        todo!()
    }

    pub async fn get_item(
        &self,
        _item_name: &str,
        _pool: &PgPool,
    ) -> CommandResult<Option<(Count, Item)>> {
        todo!()
    }

    /// Same as `get_item` but it devreases the count of the item by one
    pub async fn take_item(
        &self,
        _item_name: &str,
        _pool: &PgPool,
    ) -> CommandResult<Option<(Count, Item)>> {
        todo!()
    }

    pub async fn get_inventory_string(&self, _pool: &PgPool) -> CommandResult<String> {
        todo!()
    }

    pub async fn add_note(&self, _text: &str, _when: &str, _pool: &PgPool) -> CommandResult {
        todo!()
    }

    pub async fn add_ripped_note(&self, _note: Note, _pool: &PgPool) -> CommandResult {
        todo!()
    }

    pub async fn rip_note(&self, _idx: usize, _pool: &PgPool) -> CommandResult<Option<Note>> {
        todo!()
    }

    pub async fn get_note(&self, _idx: usize, _pool: &PgPool) -> CommandResult<Option<Note>> {
        todo!()
    }

    pub async fn get_notes_between_as_embed(
        &self,
        _start: usize,
        _end: usize,
        _pool: &PgPool,
    ) -> CommandResult<Option<CreateEmbed>> {
        todo!()
    }

    pub async fn eat_or_starve(&self, _ctx: &Context, _pool: &PgPool) -> CommandResult {
        todo!()
    }

    pub fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        self.role.win_condition_achieved(block)
    }

    pub fn role_name(&self) -> RoleName {
        self.role.name()
    }

    async fn items(&self, pool: &PgPool) -> CommandResult<Items> {
        let mut res = sqlx::query!(
            r#"
SELECT count, name
FROM public.items
WHERE user_id = $1 AND game_id = $2
        "#,
            self.id.0 as i64,
            self.guild_id.0 as i64
        )
        .fetch(pool);

        let mut raw_items: Vec<(u8, String)> = Vec::with_capacity(6);

        while let Ok(Some(raw_item)) = res.try_next().await {
            raw_items.push((raw_item.count as u8, raw_item.name));
        }

        todo!()
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("role", &self.role.name())
            .field("alive", &self.alive)
            .field("room", &self.room)
            .finish()
    }
}
