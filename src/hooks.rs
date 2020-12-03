use serenity::{
    framework::standard::Reason,
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::channel::Message,
    prelude::*,
};
use tracing::info;

#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    use crate::data::stats::CommandStatisticsContainer;

    ctx.data
        .read()
        .await
        .get::<CommandStatisticsContainer>()
        .expect("ctx.data should have a CommandStatisticsContainer in it. Always")
        .write()
        .await
        .add_invocation(command_name);

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
pub async fn after(
    _ctx: &Context,
    _msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => info!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(duration) = error {
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                &format!("Try this again in {} seconds.", duration.as_secs()),
            )
            .await;
    } else if let DispatchError::CheckFailed(_, reason) = error {
        match reason {
            Reason::User(error_msg) => {
                let _ = msg.reply(ctx, error_msg).await;
            }
            Reason::Log(log) => {
                info!("{}", log);
            }

            Reason::UserAndLog {
                user: error_msg,
                log: error_log,
            } => {
                let _ = msg.reply(ctx, error_msg).await;
                info!("{}", error_log);
            }
            _ => (),
        }
    }
}
