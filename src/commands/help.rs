use serenity::prelude::*;
use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, Delimiter, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

#[help]
#[individual_command_tip = "Hello! If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut args_message = args.rest();
    let prefix = std::env::var("MAID_PREFIX").unwrap();

    if args_message.starts_with(&prefix) {
        args_message = args_message.trim_start_matches(&prefix);
    }

    let args = Args::new(args_message, &[Delimiter::Single(' ')]);

    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
