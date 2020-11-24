use super::prelude::*;
use crate::game::{item::Item, GameState};

use serenity::model::id::UserId;

#[command("give")]
#[description(
    r#"This command allows you to give some items away. You cannot give away your memo book, ballpoint pen or tablet.

Valid item names are: "food", "food bar", "food bars", "knife", "watch". (without quotes)

Note that you have to mention someone as the target.

(Usage and Sample usage do not include the prefix, but it still must be used)
"#)]
#[usage("<target user mention> <item>")]
#[example("@KC#7788 food")]
pub async fn give_item(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if game.is_none() {
        msg.reply_err(
            ctx,
            "you can't give items when there's no game running".into(),
        )
        .await?;
        return Ok(());
    }

    let mut game = game.unwrap().write().await;
    let game_state = game.state();
    let players = game.players_mut();

    let giver = msg.author.id;
    let giver = players.get_mut(&giver);
    if giver.is_none() {
        msg.reply_err(ctx, "you can't give items when you're not in a game".into())
            .await?;
        return Ok(());
    }

    if game_state == GameState::NotStarted {
        msg.reply_err(ctx, "you can't give items before a game starts".into())
            .await?;
        return Ok(());
    }

    if game_state == GameState::GameEnded {
        msg.reply_err(ctx, "you can't give items after a game has ended".into())
            .await?;
        return Ok(());
    }

    let target = args.single::<UserId>();
    if target.is_err() {
        msg.reply_err(
            ctx,
            r#"
you need to specify a valid user to give an item to.

Note that the syntax of this command is `!give <TARGET> <WHAT>`, you'd use it like: `!give @MyFriend food`"#.into(),
        )
        .await?;
        return Ok(());
    }
    let target = target.unwrap();

    let what = args.remains();
    if what.is_none() {
        msg.reply_err(ctx, r#"
you need to specify an item to give it away.

Note that the syntax of this command is `!give <TARGET> <WHAT>`, you'd use it like: `!give @MyFriend food`"#.into())
            .await?;
        return Ok(());
    }
    let what = what.unwrap();
    let mut watch_name = None;

    if let Some(giver) = giver {
        watch_name = Some(giver.items().get_item("watch").1.name.clone());
        let what = parse_item(what, watch_name.as_ref().unwrap().as_ref());
        if what.is_err() {
            msg.reply_err(ctx, what.unwrap_err()).await?;
            return Ok(());
        }
        let giver_item = giver.items_mut().get_item_mut(what.unwrap().as_ref());
        if giver_item.0 == 0 {
            msg.reply_err(ctx, "you can't give away items you don't have".into())
                .await?;
            return Ok(());
        }

        giver_item.0 -= 1;
    };

    let target = players.get_mut(&target);
    if target.is_none() {
        msg.reply_err(
            ctx,
            "you can't give an item to someone who's not in the game".into(),
        )
        .await?;
        return Ok(());
    }
    if let Some(target) = target {
        let what = parse_item(what, watch_name.unwrap().as_ref());
        if what.is_err() {
            msg.reply_err(ctx, what.unwrap_err()).await?;
            return Ok(());
        }

        if !what.as_ref().unwrap().contains("watch") {
            let target_item = target.items_mut().get_item_mut(what.unwrap().as_ref());

            target_item.0 += 1;
        } else {
            target.add_item(Item {
                name: what.unwrap(),
                edible: false,
                weapon: false,
            });
        }
    }
    Ok(())
}

fn parse_item(name: &str, watch: &str) -> Result<String, String> {
    let name = name.to_lowercase();
    match name.as_ref() {
        "food" | "food bar" | "food bars" => Ok(Item::FOOD_NAME.into()),
        "knife" => Ok("Knife".into()),
        "watch" => Ok(watch.into()),
        _ => Err(format!("you can't give away a '{}'", name)),
    }
}
