use serenity::{
    builder::CreateEmbed,
    model::id::{GuildId, UserId},
    prelude::*,
};

pub async fn build_embed_for_murder_confirmation(
    ctx: &Context,
    user: UserId,
    guild: GuildId,
) -> Result<CreateEmbed, super::Error> {
    let user = user.to_user(ctx).await?;

    let ava = user.avatar_url().unwrap_or(user.default_avatar_url());

    let name = user.nick_in(ctx, guild).await.unwrap_or(user.name);

    let mut embed = CreateEmbed::default();
    embed
        .title(format!("The king has asked you to 「 Murder 」 {}", name))
        .image(ava);

    Ok(embed)
}
