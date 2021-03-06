use super::prelude::*;
use crate::game::item::Item;

use serenity::model::id::UserId;

#[command("give")]
#[only_in(guilds)]
#[description(
    r#"This command allows you to give some items away. You cannot give away your memo book, ballpoint pen or tablet.

Valid item names are: "food", "food bar", "food bars", "knife", "watch". (without quotes)

Note that you have to mention someone as the target.

(Usage and Sample usage do not include the prefix, but it still must be used)
"#)]
#[usage("<target user mention> <item>")]
#[example("@KC#7788 food")]
#[checks(StandardGameCheck, UserIsPlaying)]
pub async fn give_item(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let mut game = game_guard.write().await;

    let giver = game.player_mut(msg.author.id).expect("needed");

    let target = args.single::<UserId>();
    if target.is_err() {
        msg.reply(
            ctx,
            r#"
You need to specify a valid user to give an item to.

Note that the syntax of this command is `!give <TARGET> <WHAT>`, you'd use it like: `!give @MyFriend food`"#,
        )
        .await?;
        return Ok(());
    }
    let target = target.unwrap();

    let what = args.remains();
    if what.is_none() {
        msg.reply(ctx, r#"
You need to specify an item to give it away.

Note that the syntax of this command is `!give <TARGET> <WHAT>`, you'd use it like: `!give @MyFriend food`"#)
            .await?;
        return Ok(());
    }
    let what_as_str = what.unwrap();
    let watch_name = giver.items().get_item("watch").1.name.clone();

    let what = parse_item(what_as_str, watch_name.as_ref());
    if what.is_err() {
        msg.reply(ctx, what.unwrap_err()).await?;
        return Ok(());
    }
    let giver_item = giver.items_mut().get_item_mut(what.unwrap().as_ref());
    if giver_item.0 == 0 {
        msg.reply(ctx, "You can't give away items you don't have")
            .await?;
        return Ok(());
    }

    giver_item.0 -= 1;
    // };/
    // drop(giver); // stop borrowing `game` as mut here

    match game.player_mut(target) {
        Some(target) => {
            let what = parse_item(what_as_str, watch_name.as_ref());
            if what.is_err() {
                msg.reply(ctx, what.unwrap_err()).await?;
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
        None => {
            msg.reply(
                ctx,
                "You can't give an item to someone who's not in the game",
            )
            .await?;
            return Err("idk".into());
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
        _ => Err(format!("You can't give away a '{}'", name)),
    }
}
