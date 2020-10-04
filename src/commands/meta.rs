use super::prelude::*;
use serenity::model::id::UserId;
use serenity::utils::MessageBuilder;

#[command]
pub async fn purpose(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let reply = MessageBuilder::new()
        .push("Hmph?! Stop staring at me ")
        .mention(&msg.author)
        .push("!! You want to know why I exist? Ah, to help my lazy master run this silly game of course~")
        .build();
    msg.channel_id.say(&ctx.http, reply).await?;
    Ok(())
}

#[command]
pub async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let info = ctx.http.get_current_application_info().await?;
    let owner = if let Some(team) = info.team {
        team.owner_user_id
    } else {
        info.owner.id
    };
    let owner = owner.to_user(ctx).await?;

    let bot_info = ctx.http.get_current_user().await?;
    let bot_avatar = bot_info.avatar_url().unwrap_or("".to_owned());
    let bot_name = bot_info.name;

    let (kc_name, kc_discrim) = {
        let kc = UserId::from(165912402716524544);
        let kc = kc.to_user(ctx).await?;
        (kc.name, kc.discriminator)
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    let owner_icon = owner.avatar_url().unwrap_or("".to_string());
                    let owner_name = owner.name;
                    let owner_discrim = owner.discriminator;
                    a.icon_url(owner_icon)
                        .name(format!("{}#{}", owner_name, owner_discrim))
                })
                .thumbnail(bot_avatar)
                .description(
                    format!(r#"
{} is a Discord bot which allows you to play the game called *Kingdom Royale* from the light novel Utsuro no Hako to Zero no Maria(en. The Empty Box and Zeroth Maria) written by Eiji Mikage 
                "#, bot_name)
                ).field("Image licensing", "See [this](https://github.com/RealKC/kingdom-royale-maid/blob/master/res/README.md)", true)
                .footer(|f| {
                    f.text(format!("Licensed under the AGPL v3.0 to {}#{}", kc_name, kc_discrim))
                })
            })
        })
        .await?;
    Ok(())
}
