use super::prelude::*;

use once_cell::sync::Lazy;
use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::check, CommandOptions, Reason},
};

// Fancy quotes
// “” -> for flavour excerpts from the book
// 「  」 -> for emphasis
// 『  』 -> for stronger emphasis

macro_rules! role_embed {
    ($name:ident, $name_str:expr, $image:expr, $description:expr, $colour:expr, $skills:expr, $victory:expr) => {
        static $name: Lazy<CreateEmbed> = Lazy::new(|| {
            CreateEmbed::default()
                .title($name_str)
                .thumbnail($image)
                .description($description)
                .colour($colour)
                .field("『 Skills 』", $skills, false)
                .field("『 Victory conditions 』", $victory, false)
                .footer(|f| f.text("Eiji Mikage"))
                .clone()
        });
    };
}

role_embed!(
    KING,
    "『 King 』",
    "https://github.com/RealKC/kingdom-royale-maid/raw/master/res/king.png",
    r#"“He is the king who has ascended to the throne by assassinating the previous ruler and has carried out many invasions. Having a distrustful personality, he's scheming murder of the ones that threaten his throne. He does not notice that his distrust makes others lose their loyalty for him.

He can request his subordinates to commit 「 murder 」, but he cannot force themm because he fears their animosity could become directed at him.

A land ruled by a man that cannot trust others is unlikely to have a bright future.”
"#,
    0xad42f5, // Nice royal blue
    r#"
「 Murder 」
He can select a player he wants to kill and request the 「 Sorcerer 」 or 「 Knight 」 to execute this action. He does not need to select.
    
「 Substitution 」
He can once avoid being the target of 「 Assassination 」 by changing roles with 「 The Double 」 for a single day. If he was selected as the target on this day, 「 The Double 」 will die instead of the 「 King 」.
"#,
"To protect his throne. (Elimination of the ones that threaten the King's throne - 「 Prince 」 「 Revolutionary 」 "
);

role_embed!(
    PRINCE,
    "『 Prince 』",
    "https://github.com/RealKC/kingdom-royale-maid/raw/master/res/prince.png",
    r#"
“An ambitious person. He was originally only at the third place in the inheritance order of the king's rank. But taking advantage of the king's mistrust, he made him murder the other princes and moved up to the first place. He acquired anti-magic to guard himself against this mistrust.

If he comes to the throne, this land is likely to turn into a worse dictatorship than it was before.”
    "#,
    0xfcf403, // Yellow for the crown he wishes
    r#"
「 Throne Succession」
He becomes able to use 「 Murder 」 once the 「 King 」 and 「 The Double 」 die. 

「 Anti-magic 」
He cannot be killed by 「 Sorcery 」."#,
    "To become the king. (Elimination of 「 King 」 「 The Double 」 「 Revolutionary 」)"
);

role_embed!(
    THE_DOUBLE,
    "『 The Double 』",
    "https://github.com/RealKC/kingdom-royale-maid/raw/master/res/the_double.png",
    r#"
“An ex-farmer who is loyal to the 「 King 」 and looks exactly the same as him. He is not really ambitious, but he can absolutely not allow the 「 Prince 」 to become the king since he was always made a fool by him.

If he, with no ideals, becomes the king, this land is likely to fall into ruin in no time.”
"#,
    0x417505, // Green like his old fields
    "
「 Inheritance 」
If the 「 King 」 dies or 「 Substitution 」 was executed, he becomes able to use 「 Murder 」.",
    "Death of the ones that try to kill him. (Death of 「 Prince 」 「 Revolutionary 」)"
);

role_embed!(
    SORCERER,
    "『 Sorcerer 』",
    "https://github.com/RealKC/kingdom-royale-maid/raw/master/res/sorcerer.png",
    r#"
“A subordinate of the 「 King 」. He is the teacher of the 「 Prince 」 in magic and also gets on well with the 「 Prince 」. He is satisfied as long he can pursue his studies in magic and has no interest in the king's throne whatsoever.

No matter how much he can raise his magic skills, nobody will value a person that secludes himself in his shell.”"#,
    0x50E3C2, // A nice teal for his magic
    r#"「 Sorcery 」
He can choose whether to effectively kill the character that was selected by 「 Murder 」. The targeted character will become a burnt corpse."#,
    "To survive."
);

role_embed!(
    KNIGHT,
    "『 Knight 』",
    "https://github.com/RealKC/kingdom-royale-maid/raw/master/res/knight.png",
    r#"
“A subordinate of the 「 King 」. While being a subordinate, he is plotting revenge on the royal family for they have ruined his homeland. He believes firmly that he can only attain happiness by exterminating the royal family.

As a matter of course, a man that has drowned in his own feelings of loss will only fall into the darkness of misfortune.”"#,
    0x9B9B9B, // Grey for his armour
    r#"「 Deathblow 」
He can choose whether to effectively kill the character that was selected by 「 Murder 」. Only executable when the 「 Sorcerer 」 is dead. The targeted character will die due to beheading.
"#,
    "To take revenge. (Death of 「 King 」 「 Prince 」)"
);

role_embed!(
    REVOLUTIONARY,
    "『 Revolutionary 』",
    "https://github.com/RealKC/kingdom-royale-maid/raw/master/res/revolutionary.png",
    r#"
“He is the right arm of the 「 King 」. Because of his competence, he realized that this land is going to fall into ruin if it goes on like this. Hence, he prepared himself to take over the land.

A ruler that has accumulated feelings of bitterness due to assassinations is incapable of leading a kingdom. At most he will be assassinated himself.”"#,
    0xD0021B, // Red for the blood he spills
    r#"「 Assassination 」
He can assassinate the selected character. He does not need to select one. The targeted character will become a strangulated corpse."#,
    "To become the king. (Murder of 「 King 」 「 Prince 」 「 The Double 」)"
);

async fn say_role(ctx: &Context, msg: &Message, role: &CreateEmbed) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| m.set_embed(role.clone()))
        .await?;
    Ok(())
}

#[check]
#[name = "IsGood"]
pub async fn perms_are_good(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let mut passed = false;
    let info = ctx.http.get_current_application_info().await;
    if let Ok(info) = info {
        if let Some(team) = info.team {
            for member in team.members {
                if member.user.id == msg.author.id {
                    passed = true;
                }
            }
        } else if info.owner.id == msg.author.id {
            passed = true;
        }

        if let Some(guild) = msg.guild(ctx).await {
            if let Ok(member) = guild.member(ctx, msg.author.id).await {
                let permissions = member.permissions(ctx).await;
                if let Ok(permissions) = permissions {
                    passed = permissions.administrator() || permissions.manage_messages();
                }
            }
        }
    }

    if passed {
        return Ok(());
    }

    Err(Reason::UserAndLog{
        user: "You need either the Manage Messages permission or to be Administrator".into(),
        log: "user lacks permissions to run this command (needs either Manage messages/Administrator, or to be the owner of the bot".into()
    })
}

#[command]
#[only_in(guilds)]
#[checks(IsGood)]
#[description("Shows information about the 6 roles available in Kingdom Royale")]
pub async fn roles(ctx: &Context, msg: &Message) -> CommandResult {
    say_role(ctx, msg, &*KING).await?;
    say_role(ctx, msg, &*PRINCE).await?;
    say_role(ctx, msg, &*THE_DOUBLE).await?;
    say_role(ctx, msg, &*SORCERER).await?;
    say_role(ctx, msg, &*KNIGHT).await?;
    say_role(ctx, msg, &*REVOLUTIONARY).await?;
    Ok(())
}

#[command("roleinfo")]
#[only_in(guilds)]
#[description("Shows information about a specific role")]
#[aliases("rinfo")]
pub async fn role_info(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let role = args.remains();
    if role.is_none() {
        msg.reply(ctx, "You need to write a role you want info about")
            .await?;
        return Ok(());
    }
    let role = role.unwrap().to_lowercase();
    match role.as_str() {
        "king" => say_role(ctx, msg, &*KING).await?,
        "prince" => say_role(ctx, msg, &*PRINCE).await?,
        "double" | "the double" => say_role(ctx, msg, &*THE_DOUBLE).await?,
        "sorcerer" => say_role(ctx, msg, &*SORCERER).await?,
        "knight" => say_role(ctx, msg, &*KNIGHT).await?,
        "revolutionary" => say_role(ctx, msg, &*REVOLUTIONARY).await?,
        _ => msg
            .reply(ctx, "That's not a valid role!")
            .await
            .map(|_| ())?,
    };
    Ok(())
}
