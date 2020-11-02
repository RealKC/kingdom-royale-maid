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
