use super::{
    item::{Item, Items},
    roles::{Role, RoleName},
    DeathCause, Game,
};
use serenity::{
    framework::standard::CommandResult,
    model::id::{ChannelId, UserId},
    prelude::*,
};

type SecretMeeting = Option<(UserId, ChannelId)>;

pub struct Player {
    id: UserId,
    role: Box<(dyn Role + Send + Sync)>,
    alive: bool,
    room: ChannelId,
    secret_meeting_partner: Option<UserId>,
    secret_meeting_channels: Vec<(SecretMeeting, SecretMeeting)>,
    items: Items,
}

impl Player {
    pub fn new(
        id: UserId,
        role: Box<(dyn Role + Send + Sync)>,
        room: ChannelId,
        watch_colour: String,
    ) -> Self {
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
            assert!(secret_meetings_for_day.1.is_none());
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

    pub fn items(&self) -> &Items {
        &self.items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.add_item(item)
    }

    pub fn items_mut(&mut self) -> &mut Items {
        &mut self.items
    }

    pub fn win_condition_achieved(&self, game: &Game) -> bool {
        self.role.win_condition_achieved(game)
    }

    pub fn role_name(&self) -> RoleName {
        self.role.name()
    }
}
