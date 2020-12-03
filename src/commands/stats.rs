use crate::data::stats::{CommandStatisticsContainer, StartupTime, SystemVersion};

use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::{fmt::Write, time::Instant};

#[command]
#[description("Shows a number of different statistics about the bot")]
#[aliases("statistics")]
pub async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    use humansize::{file_size_opts as size_options, FileSize};

    let stats_lock = ctx
        .data
        .read()
        .await
        .get::<CommandStatisticsContainer>()
        .cloned()
        .expect("ctx.data should have a CommandStatisticsContainer in it. Always");
    let stats = stats_lock.read().await;

    let command_invocations = {
        let mut contents = String::new();

        for (k, v) in &stats.command_invocations {
            let _ = writeln!(contents, "- {name}: {amount} times", name = k, amount = v);
        }

        contents
    };

    // This SO answers describes what PSS is, and why I use it <https://stackoverflow.com/a/13754307>
    let (uss, pss, rss) = {
        use procfs::process::Process;

        let myself = Process::myself()?;
        //let _maps = myself.maps()?;
        let smaps = myself.smaps()?;

        //info!("{:?}", _maps);

        let mut pss: u64 = 0;
        let mut uss: u64 = 0;
        for (_, data) in smaps {
            pss += data.map.get("Pss").unwrap_or(&0);
            uss += data.map.get("Private_Clean").unwrap_or(&0)
                + data.map.get("Private_Dirty").unwrap_or(&0);
        }

        (uss, pss, myself.stat.rss)
    };

    let startup_time = *ctx
        .data
        .read()
        .await
        .get::<StartupTime>()
        .expect("ctx.data should always have a StartupTime in it");
    let system_version = ctx
        .data
        .read()
        .await
        .get::<SystemVersion>()
        .expect("ctx.data should always have a SystemVersion in it")
        .clone();

    let mut embed = CreateEmbed::default();

    embed
        .title("Statistics")
        .description(format!(
            r#"
Total amount of command invocations: {invocations}.

Uptime: {uptime}

**System:**
```{ver}```
"#,
            invocations = stats.total_command_invocations,
            uptime = get_formatted_uptime(startup_time)?,
            ver = system_version
        ))
        .field("Command invocations", command_invocations, true)
        .field(
            "Memory usage",
            format!(
                "PSS: {}\nRSS: {}\n USS(no shared): {}",
                pss.file_size(size_options::BINARY).unwrap(),
                uss.file_size(size_options::BINARY).unwrap(),
                rss.file_size(size_options::BINARY).unwrap()
            ),
            true,
        );

    msg.channel_id
        .send_message(ctx, |m| m.set_embed(embed))
        .await?;

    Ok(())
}

fn get_formatted_uptime(
    startup_time: Instant,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let elapsed_since_startup = chrono::Duration::from_std(startup_time.elapsed())?;

    let weeks = elapsed_since_startup.num_weeks();
    let days = elapsed_since_startup.num_days();
    let hours = elapsed_since_startup.num_hours();
    let minutes = elapsed_since_startup.num_minutes();
    let seconds = elapsed_since_startup.num_seconds();

    if weeks != 0 {
        Ok(format!(
            "{} weeks, {} days, {}h{}m{}s",
            weeks,
            days % 7,
            hours % 24,
            minutes % 60,
            seconds % 60
        ))
    } else {
        Ok(format!(
            "{} days, {}h{}m{}s",
            days,
            hours % 24,
            minutes % 60,
            seconds % 60
        ))
    }
}
