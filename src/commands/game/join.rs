use super::prelude::*;
use tracing::info;

#[command]
#[only_in(guilds)]
#[bucket = "join_leave_ratelimit_bucket"]
#[description("Allows you to join a game that has yet to start and that has less than 6 players")]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let game = data.get_mut::<GameContainer>();

    if game.is_some() {
        let mut game = game.unwrap().write().await;

        let member = msg.member(ctx).await?;
        let mut member_may_have_admin_perms = member.permissions(ctx).await?.administrator();
        member_may_have_admin_perms |= msg.guild(ctx).await.unwrap().owner_id == msg.author.id;

        if member_may_have_admin_perms {
            msg.reply(
                ctx,
                "You can't join a game if you're the Owner of a server or an administrator!",
            )
            .await?;
        } else {
            let result = game.join(msg.author.id);
            if result.is_ok() {
                info!("Successfully added a new user to the game");
                msg.reply(
                    ctx,
                    format!(
                        "You've joined {}'s Kingdom Royale game.",
                        game.host().to_user(ctx).await?
                    ),
                )
                .await?;
            } else {
                info!("Couldn't add new user, error is {:?}", result.unwrap_err());
                msg.reply(ctx, format!("{}", result.unwrap_err())).await?;
            }
        }
    } else {
        info!("User tried joining inexistent user");
        msg.reply(ctx, "you can't join a game if there aren't any in progress")
            .await?;
    }
    Ok(())
}
