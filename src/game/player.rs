use std::fmt::Debug;

use super::{
    fsm::TimeBlock,
    item::{Item, Items},
    roles::{RoleHolder, RoleName},
    DeathCause,
};
use serenity::{
    framework::standard::CommandResult,
    model::id::{ChannelId, UserId},
    prelude::*,
};

pub type SecretMeeting = Option<(UserId, ChannelId)>;

#[derive(Clone)]
pub struct Player {
    id: UserId,
    role: RoleHolder,
    alive: bool,
    room: ChannelId,
    secret_meeting_partner: Option<UserId>,
    secret_meeting_channels: Vec<(SecretMeeting, SecretMeeting)>,
    items: Items,
}

impl Player {
    pub fn new(id: UserId, role: RoleHolder, room: ChannelId, watch_colour: String) -> Self {
        // PONDER: We may want to allow disabling certain items
        //         If we do, how should that be handled? Should we just pass a reference to the Game and ask it for enabled items?
        Self {
            id,
            role,
            room,
            alive: true,
            secret_meeting_partner: None,
            secret_meeting_channels: vec![],
            items: Items::new(watch_colour),
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn room(&self) -> ChannelId {
        self.room
    }

    pub fn secret_meeting_partner(&self) -> Option<UserId> {
        self.secret_meeting_partner
    }

    pub fn set_secret_meeting_partner(&mut self, partner: UserId) {
        self.secret_meeting_partner = Some(partner);
    }

    pub fn add_secret_meeting(&mut self, day: u8, channel: ChannelId) {
        let day = day as usize;
        self.secret_meeting_channels.resize(day, (None, None));
        let secret_meetings_for_day = self
            .secret_meeting_channels
            .get_mut(day)
            .expect("yeah we done goofed it seems");

        if secret_meetings_for_day.0.is_none() {
            secret_meetings_for_day.0 = Some((self.secret_meeting_partner.expect("We should have a secret_meeting_partner when we're adding secret rooms to players"), channel));
        } else {
            debug_assert!(secret_meetings_for_day.1.is_none());
            secret_meetings_for_day.1 = Some((self.secret_meeting_partner.expect("We should have a secret_meeting_partner when we're adding secret rooms to players"), channel));
        }
    }

    pub fn get_secret_meetings_for_day(&self, day: u8) -> Option<&(SecretMeeting, SecretMeeting)> {
        self.secret_meeting_channels.get(day as usize)
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

    pub fn items(&self) -> &Items {
        &self.items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.add_item(item)
    }

    pub fn items_mut(&mut self) -> &mut Items {
        &mut self.items
    }

    pub fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        self.role.win_condition_achieved(block)
    }

    pub fn role_name(&self) -> RoleName {
        self.role.name()
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("role", &self.role.name())
            .field("alive", &self.alive)
            .field("room", &self.room)
            .field("secret_meeting_partner", &self.secret_meeting_partner)
            .field("secret_meeting_channels", &self.secret_meeting_channels)
            .field("items", &self.items)
            .finish()
    }
}
