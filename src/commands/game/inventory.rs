use serenity::builder::CreateEmbed;

use super::prelude::*;

#[command]
#[aliases("bag")]
#[description("Allows you to inspect the items you have in your bag")]
pub async fn inventory(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let game = data.get::<GameContainer>();
    if game.is_none() {
        msg.reply_err(
            ctx,
            "you can't look into your bag when there's no game running".into(),
        )
        .await?;
        return Ok(());
    }
    let game = game.unwrap().read().await;

    let player = game.players().get(&msg.author.id);
    if player.is_none() {
        msg.reply_err(
            ctx,
            "you can't look into your bag when you aren't in the game".into(),
        )
        .await?;
        return Ok(());
    }
    let player = player.unwrap();

    let items = player.items();

    let mut inventory = String::new();
    for item in items.iter() {
        inventory.push_str(&format!("{} ({})\n", item.1.name, item.0));
    }

    let mut embed = CreateEmbed::default();
    embed
        .title("Your inventory")
        .color(0x8B572A)
        .image("https://github.com/RealKC/kingdom-royale-maid/raw/master/res/the_jute_bag.png")
        .field("Items", inventory, true);

    msg.channel_id
        .send_message(ctx, |m| m.set_embed(embed))
        .await?;

    Ok(())
}