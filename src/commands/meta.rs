use serenity::{model::Message, ,framework::{standard::{{macros::command, CommandResult}}}};
use serenity::prelude::*;

#[command]
async fn purpose(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let reply = MessageBuilder::new()
        .push("Hmph?! Stop staring at me ")
        .mention(&msg.author)
        .push("!! You want to know why I exist? Ah, to help my lazy master run this silly game of course~")
        .build();
    msg.channel_id.say(&ctx.http, reply).await;
    Ok(())
}
