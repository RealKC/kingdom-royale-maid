use crate::commands::prelude::*;
use crate::game::RoleName;
use crate::helpers::{choose_target::build_embed_for_target_choice, react::react_with};
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

async fn generic_test(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    role_kind: RoleName,
) -> CommandResult {
    let _typing = msg.channel_id.start_typing(&ctx.http)?;
    let mut players = vec![];

    while let Ok(user_id) = args.single::<UserId>() {
        players.push(user_id);
    }

    let embed = build_embed_for_target_choice(ctx, &players, role_kind).await?;

    let sent_msg = msg
        .channel_id
        .send_message(ctx, |m| m.set_embed(embed))
        .await?;

    react_with(ctx, &sent_msg, &["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣"]).await?;

    Ok(())
}
