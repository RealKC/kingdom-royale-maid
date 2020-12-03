use crate::ShardManagerContainer;

use super::prelude::*;

#[command]
#[owners_only]
pub async fn shutdown(ctx: &Context, _: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = data
        .get::<ShardManagerContainer>()
        .expect("ctx.data should always have a ShardManagerContainer");

    shard_manager.lock().await.shutdown_all().await;

    Ok(())
}
