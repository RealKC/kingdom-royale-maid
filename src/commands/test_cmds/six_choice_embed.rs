use crate::{
    commands::prelude::*,
    data::Cdn,
    game::{King, Player, RoleHolder, RoleName},
    helpers::{
        choose_target::{build_embed_for_target_choice, Players},
        react::react_with,
    },
};

use serenity::model::id::UserId;

#[command("testk")]
#[description("Used to test the algorithm that creates the embed for the time when the king-like player needs to choose one for murder")]
pub async fn king_test(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    generic_test(ctx, msg, args, RoleName::King).await
}

#[command("testr")]
#[description("Used to test the algorithm that creates an embed for the time when the revolutionary needs to kill")]
pub async fn rev_test(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    generic_test(ctx, msg, args, RoleName::Revolutionary).await
}

struct MockPlayers {
    ids: Vec<UserId>,
    players: Vec<Player>,
}

impl MockPlayers {
    async fn new(ctx: &Context, ids: Vec<UserId>) -> Self {
        let mut players = vec![];

        for (idx, id) in ids.iter().enumerate() {
            let mut player = Player::new(
                *id,
                RoleHolder::King(King),
                ctx.data
                    .read()
                    .await
                    .get::<Cdn>()
                    .copied()
                    .expect("There must always be a CDN in ctx.data"),
                "lol".to_string(),
            );

            if idx % 2 == 0 {
                player.set_dead_mock();
            }

            players.push(player);
        }

        Self { ids, players }
    }
}

impl Players for MockPlayers {
    fn players(&self) -> Vec<Player> {
        self.players.clone()
    }

    fn player_ids(&self) -> Vec<UserId> {
        self.ids.clone()
    }
}

async fn generic_test(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    role_kind: RoleName,
) -> CommandResult {
    let _typing = msg.channel_id.start_typing(&ctx.http)?;
    let mut player_ids = vec![];

    while let Ok(user_id) = args.single::<UserId>() {
        player_ids.push(user_id);
    }

    let players = MockPlayers::new(ctx, player_ids).await;

    let embed = build_embed_for_target_choice(
        ctx,
        &players,
        if role_kind.is_king_like() {
            "Please select a target for 「 Murder 」"
        } else {
            "Please select a target for 「 Assassination 」"
        },
    )
    .await?;

    let sent_msg = msg
        .channel_id
        .send_message(ctx, |m| m.set_embed(embed))
        .await?;

    react_with(ctx, &sent_msg, &["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣"]).await?;

    Ok(())
}
