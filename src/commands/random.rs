use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};

// Repeats what the user passed as argument but ensures that user and role
// mentions are replaced with a safe textual alternative.
#[command]
pub async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        // By default roles, users, and channel mentions are cleaned.
        ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings).await;

    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
