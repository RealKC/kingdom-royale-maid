use super::prelude::*;

use serenity::model::misc::Mentionable;

#[command("gameinfo")]
#[only_in(guilds)]
#[description("Shows info(such as players and started status) about a game")]
pub async fn game_info(ctx: &Context, msg: &Message) -> CommandResult {
    let game_guard = match get_game_guard(ctx).await {
        Ok(guard) => guard,
        Err(err) => {
            msg.reply(
                ctx,
                "You can't get info about a game if there's none running!",
            )
            .await?;
            return Err(err);
        }
    };
    let game = game_guard.read().await;

    let (players_field_name, players_field_value) = {
        if !game.is_started() {
            let mut players = String::new();
            let joined_users = game
                .joined_users()
                .expect("joined users should be Some before a game start");
            for user in joined_users.iter() {
                players.push_str(&user.mention().to_string());
                players.push('\n');
            }
            if !players.is_empty() {
                (format!("Players ({})", joined_users.len()), players)
            } else {
                ("Players".to_string(), "None have joined yet :(".to_string())
            }
        } else {
            let mut players_string = String::new();

            let all_alive_have_won = game.all_alive_have_won();
            let players = game.players().expect("players should be Some");

            for player in players.iter() {
                if player.1.is_alive() {
                    let mention = player.0.mention().to_string();
                    players_string.push_str(&mention);
                    if all_alive_have_won {
                        players_string.push_str("(Victory!)");
                    }
                    players_string.push('\n');
                } else {
                    let mention = format! {"~~{}~~\n", player.0.mention()};
                    players_string.push_str(&mention);
                }
            }
            (format!("Players ({})", players.len()), players_string)
        }
    };
    let fields = vec![
        ("Host", game.host().mention().to_string(), false),
        (&players_field_name, players_field_value, true),
        (
            "Meeting room",
            game.meeting_room().mention().to_string(),
            true,
        ),
        (
            "Announcement channel",
            game.announcement_channel().mention().to_string(),
            true,
        ),
        (
            "Player role",
            game.player_role().mention().to_string(),
            true,
        ),
    ];

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    if !game.is_started() {
                        if game.can_start() {
                            a.icon_url("https://cdn.discordapp.com/emojis/764529845756493885.png")
                                .name("Not started")
                        } else {
                            a.icon_url("https://cdn.discordapp.com/emojis/764529845756493885.png")
                                .name("Not started (waiting for players)")
                        }
                    } else {
                        a.icon_url("https://cdn.discordapp.com/emojis/764529758998102037.png")
                            .name(&game.state_name())
                    }
                })
                .title("Kingdom Royale")
                .fields(fields)
                .colour({
                    if !game.is_started() {
                        if game.can_start() {
                            0xdea712 // Yellow
                        } else {
                            0xbf2419 // Red
                        }
                    } else {
                        0x0dd910 // Green
                    }
                })
                .footer(|f| {
                    if let Some(ava) = msg.author.avatar_url() {
                        f.icon_url(ava);
                    }
                    f.text(if !game.is_started() {
                        msg.author.name.clone()
                    } else {
                        format!(
                            "{} | {} day",
                            msg.author.name,
                            cardinal_to_ordinal(game.day().expect("a day"))
                        )
                    })
                });

                e
            })
        })
        .await?;
    Ok(())
}

/// Takes a cardinal number and returns its ordinal version as a string
fn cardinal_to_ordinal(number: u8) -> String {
    let last_digit = number % 10;
    let number_is_teen = (10..20).contains(&number);

    if number_is_teen {
        format!("{}th", number)
    } else {
        match last_digit {
            1 => format!("{}st", number),
            2 => format!("{}nd", number),
            3 => format!("{}rd", number),
            _ => format!("{}th", number),
        }
    }
}
