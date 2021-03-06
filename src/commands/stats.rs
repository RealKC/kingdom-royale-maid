use crate::data::stats::{self, CommandStatisticsContainer, StartupTime};

use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::{fmt::Write, time::Instant};

#[cfg(not(feature = "deterministic"))]
use tracing::trace;

#[command]
#[description("Shows a number of different statistics about the bot")]
#[aliases("statistics")]
pub async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
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
    #[cfg(target_os = "linux")]
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

    #[cfg(target_os = "linux")]
    let system_version = ctx.data.read().await.get::<stats::SystemVersion>().cloned();

    let mut embed = CreateEmbed::default();

    let mut description = format!(
        r#"
    Total amount of command invocations: {invocations}.

    Uptime: {uptime}

    "#,
        invocations = stats.total_command_invocations,
        uptime = get_formatted_uptime(startup_time)?
    );

    #[cfg(target_os = "linux")]
    if let Some(system_version) = system_version {
        let version = format!(
            r#"**System**
```{}```"#,
            system_version
        );
        description = format!("{}{}", description, version);
    }

    embed.title("Statistics").description(description).field(
        "Command invocations",
        command_invocations,
        true,
    );

    #[cfg(target_os = "linux")]
    {
        use humansize::{file_size_opts as size_options, FileSize};

        embed.field(
            "Memory usage",
            format!(
                "PSS: {}\nRSS: {}\n USS(no shared): {}",
                pss.file_size(size_options::BINARY).unwrap(),
                uss.file_size(size_options::BINARY).unwrap(),
                rss.file_size(size_options::BINARY).unwrap()
            ),
            true,
        );
    }

    #[cfg(not(feature = "deterministic"))]
    if let Ok(version) = crate::resources::version() {
        use crate::data::ReqwestClient;
        use reqwest::Method;

        let github_url = format!(
            "https://github.com/RealKC/kingdom-royale-maid/commit/{}",
            version.commit
        );
        trace!("Commit: {}", github_url);

        let commit_is_online = {
            let client = ctx
                .data
                .read()
                .await
                .get::<ReqwestClient>()
                .cloned()
                .expect("There must always be a Reqwest client in the typemap");
            let response = client.request(Method::GET, &github_url).send().await;
            if let Ok(response) = response {
                response.status().is_success()
            } else {
                false
            }
        };

        let commit = format!(
            "Commit: {}",
            if commit_is_online {
                format!("[{sha}]({url})", sha = version.commit, url = github_url)
            } else {
                version.commit
            }
        );

        embed.field(
            "Build info",
            format!(
                r#"
Built on: {date}
{commit}
Has untracked changes: {not_clean}
        "#,
                date = version.timestamp,
                commit = commit,
                not_clean = !version.clean_worktree
            ),
            true,
        );
    }

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
