use super::prelude::*;
use crate::game::{RoleName, SubstitutionStatus};

#[command]
pub async fn substitute(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();

    match game {
        Some(game) => {
            let mut game = game.write().await;

            {
                let player = game.players().get(&msg.author.id);
                if player.is_none() {
                    msg.reply(
                        ctx,
                        ", you can't 「 substitute 」  with someone when you aren't in a game!",
                    )
                    .await?;
                    return Ok(());
                }
                let player = player.unwrap();

                if player.role_name() != RoleName::King {
                    msg.reply(
                        ctx,
                        ", you can't 「 substitute 」 if you're not the 「 King 」 .",
                    )
                    .await?;
                    return Ok(());
                }

                let mut aliveness_statuses = vec![];
                for player in game.players() {
                    if [RoleName::King, RoleName::TheDouble].contains(&player.1.role_name()) {
                        aliveness_statuses.push((player.1.is_alive(), player.1.role_name()));
                    }
                }
                assert!(aliveness_statuses.len() == 2);
                if aliveness_statuses[0].1 == RoleName::TheDouble {
                    aliveness_statuses.swap(0, 1);
                }
                if !aliveness_statuses[0].0 {
                    msg.reply(ctx, ", you can't 「 substitute 」 when you're dead")
                        .await?;
                    return Ok(());
                }

                if game.king_has_substituted() {
                    msg.reply(ctx, ", you can't 「 substitute 」 more than once per game")
                        .await?;
                    return Ok(());
                }

                if !aliveness_statuses[1].0 {
                    msg.reply(
                        ctx,
                        ", you can't 「 substitute 」 when 「 The Double 」 is dead",
                    )
                    .await?;
                    return Ok(());
                }
            }

            game.set_king_substitution_status(SubstitutionStatus::CurrentlyIs);
        }
        None => {
            msg.reply(
                ctx,
                ", you can't 「 substitute 」  with someone when you're not in a game!",
            )
            .await?;
        }
    }

    Ok(())
}
