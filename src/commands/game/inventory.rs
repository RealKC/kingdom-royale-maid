use super::prelude::*;

use serenity::builder::CreateEmbed;

#[command]
#[aliases("bag")]
#[only_in(guilds)]
#[description("Allows you to inspect the items you have in your bag")]
#[checks(GameCheckAllowGameEnded, UserIsPlaying)]
pub async fn inventory(ctx: &Context, msg: &Message) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let game = game_guard.read().await;

    let pool = ctx
        .data
        .read()
        .await
        .get::<Db>()
        .cloned()
        .expect("Have a pool in ctx.data");

    let player = game.player(msg.author.id).expect("player");
    let inventory = player.get_inventory_string(&pool).await?;

    let mut embed = CreateEmbed::default();
    embed
        .title("Your inventory")
        .colour(0x8B572A)
        .image("https://github.com/RealKC/kingdom-royale-maid/raw/master/res/the_jute_bag.png")
        .field("Items", inventory, true);

    msg.channel_id
        .send_message(ctx, |m| m.set_embed(embed))
        .await?;

    Ok(())
}
