use super::item::Items;
use super::roles::{Role, RoleName};
use super::{DeathCause, Game};
use serenity::model::id::{ChannelId, UserId};

pub struct Player {
    id: UserId,
    role: Box<(dyn Role + Send + Sync)>,
    alive: bool,
    room: ChannelId,
    secret_meeting_partner: Option<UserId>,
    death_cause: Option<DeathCause>,
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
            death_cause: None,
            alive: true,
            secret_meeting_partner: None,
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

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_dead(&mut self, cause: DeathCause) {
        self.alive = false;
        self.death_cause = Some(cause);
    }

    pub fn death_cause(&self) -> Option<DeathCause> {
        self.death_cause
    }

    pub fn give_item_to(&mut self, other: &mut Player, name: &str) {
        let my_item = self.items.get_item_mut(name);
        let their_item = other.items.get_item_mut(name);

        my_item.0 -= 1;
        their_item.0 += 1;
    }

    pub fn items_mut(&mut self) -> &mut Items {
        &mut self.items
    }

    pub fn can_do_special_action(&self, game: &Game) -> bool {
        self.role.can_do_special_action(game)
    }

    pub fn act(&self, target: &mut Player) {
        self.role.act(target)
    }

    pub fn win_condition_achieved(&self, game: &Game) -> bool {
        self.role.win_condition_achieved(game)
    }

    pub fn role_name(&self) -> RoleName {
        self.role.name()
    }
}
