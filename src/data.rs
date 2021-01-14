pub use reqwest::Client as Reqwest;
use serenity::{model::id::ChannelId, prelude::*};

pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = Reqwest;
}

pub struct Cdn;

impl TypeMapKey for Cdn {
    type Value = ChannelId;
}

pub struct Prefix;

impl TypeMapKey for Prefix {
    type Value = String;
}

pub mod stats {
    use serenity::prelude::RwLock;
    use std::{collections::HashMap, sync::Arc};
    use typemap_rev::TypeMapKey;

    pub struct StartupTime;

    impl TypeMapKey for StartupTime {
        type Value = std::time::Instant;
    }

    pub struct CommandStatisticsContainer;

    impl TypeMapKey for CommandStatisticsContainer {
        type Value = Arc<RwLock<CommandStatistiscs>>;
    }

    #[derive(Default)]
    pub struct CommandStatistiscs {
        pub command_invocations: HashMap<String, u64>,
        pub total_command_invocations: u64,
    }

    impl CommandStatistiscs {
        /// Increment the amount of times this command has been run, and the total amount of command invocations
        pub fn add_invocation(&mut self, command: &str) {
            let entry = self
                .command_invocations
                .entry(command.to_string())
                .or_insert(0);
            *entry += 1;
            self.total_command_invocations += 1;
        }
    }

    #[cfg(target_os = "linux")]
    pub struct SystemVersion;

    #[cfg(target_os = "linux")]
    impl TypeMapKey for SystemVersion {
        type Value = String;
    }
}
