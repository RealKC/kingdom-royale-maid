use crate::{
    commands::{prelude::*, GameContainer},
    helpers::{confirm_murder::build_embed_for_murder_confirmation, react::react_with},
};

use serenity::model::channel::Message;

#[command("tests")]
#[description(
    "Prints the embed shown to the Sorcerer or Knight when the King chooses a murder target"
)]
#[only_in(guilds)]
pub async fn confirm_murder(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild, user) = {
        let data = ctx.data.read().await;
        let game = data.get::<GameContainer>();

        match game {
            Some(game) => {
                let game = game.read().await;
                (game.guild(), game.king_murder_target().unwrap())
            }
            None => (msg.guild_id.unwrap(), msg.author.id),
        }
    };

    let embed = build_embed_for_murder_confirmation(ctx, user, guild).await?;
    let sent_msg = msg
        .channel_id
        .send_message(ctx, |m| m.set_embed(embed))
        .await?;

    react_with(ctx, &sent_msg, &["🇾", "🇳"]).await?;

    Ok(())
}
