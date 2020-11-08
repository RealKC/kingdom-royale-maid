use crate::data::Prefix;

use super::prelude::*;

#[command]
#[description("Shows the rules you must abide by while playing Kingdom Royale")]
#[aliases("howtoplay", "how2play", "h2p")]
pub async fn rules(ctx: &Context, msg: &Message) -> CommandResult {
    let author_name = match msg.guild_id {
        Some(guild_id) => msg
            .author
            .nick_in(ctx, guild_id)
            .await
            .unwrap_or_else(|| msg.author.name.clone()),
        None => msg.author.name.clone(),
    };
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    if let Some(ava) = msg.author.avatar_url(){
                        a.icon_url(ava);
                    }
                    a.name(author_name);
                    a
                })
                .title("Kingdom Royale rules")
                .field("Metarules", r#"
1. No game talk outside of the server (ok you can DM me sometimes)
2. No ghosting, you died, you're dead, cya
3. Respect the RP etiquette

~~Spreadsheets are encouraged~~
                "#, false)
                .field("Game explanation", r#"
This game is a killer game, to be a little more precise, this is a game in which everyone tries to steal the king's throne.

All of you have been assigned classes.

There is a universal time limit! Your food supply consists of seven portions of solid food. You will not be hungry if you eat one of these, but if you fail to eat one, you will become a mummy due to hunger!

You win if your classe's win condition has been fulfilled.
                "#, false)
                .field("How to play", r#"
You can join a game using `!join`, and leave it using `!leave`.

Depending on the role you get, you might have to choose whether to kill a player or not, this is done by reacting to the bot's message. (You should understand what I mean when playing).

There are many commands you can use `!help` to discover them all.
                "#, false)
                .colour(0x7289DA)
            })
        })
        .await?;
    Ok(())
}

#[command]
#[aliases("intro")]
#[description("Gives the user a short intro to what is happening")]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let prefix = data.get::<Prefix>().unwrap();

    msg
        .channel_id
        .say(
            ctx,
            format!(r#"
Hello {0}! You are playing Kingdom Royale! Try saying {1}help real quick. That should show you all the commands I can do. If you wanna see info about a specific command, just do {1}help <command name>"#,
                            msg.author.mention(),
                            prefix,
                    )
            )
        .await?;

    Ok(())
}
