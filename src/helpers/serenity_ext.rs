use serde_json::json;
use serenity::{
    async_trait,
    http::CacheHttp,
    model::{channel::Message, ModelError},
    prelude::*,
    Error,
};

type Result<T> = std::result::Result<T, Error>;

/// An extension to the serenity Message, contaning nicer formatting for my usecase
#[async_trait]
pub trait MaidReply {
    async fn reply_err(&self, cache_http: &Context, content: String) -> Result<Message>;
}

// The below implementation of `reply_err`(fenced by special comments) is taken from
// https://github.com/serenity-rs/serenity/blob/d74a0a61b818eb819c88e216e78dfb6ee1cb495f/src/model/channel/message.rs#L580
// with the following modifications
//  * the `#[cfg(feature = "cache")] block is removed
//  * the space in the format string is removed
//  * it uses &Context and String for the parameters instead of impl CacheHttp and impl Display (those didn't work for me)
// and as such I must also distribute the "permission notice"(aka ISC license),
// which I have replicated verbatim below(taken from serenity/LICENSE.md at commit 287245a6d0c693ed059ee463034db28180e105fa).

/**
 * ISC License (ISC)
 *
 * Copyright (c) 2016, Serenity Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any purpose
 * with or without fee is hereby granted, provided that the above copyright notice
 * and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
 * REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
 * INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
 * OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
 * THIS SOFTWARE.
 */

#[async_trait]
impl MaidReply for Message {
    async fn reply_err(&self, cache_http: &Context, content: String) -> Result<Message> {
        // BEGIN MODIFIED SERENITY CODE
        if let Some(length_over) = Message::overflow_length(&content) {
            return Err(Error::Model(ModelError::MessageTooLong(length_over)));
        }

        let gen = format!("{}, {}", self.author.mention(), content);

        let map = json!({
            "content": gen,
            "tts": false,
        });

        cache_http
            .http()
            .send_message(self.channel_id.0, &map)
            .await
        // END MODIFIED SERENITY CODE
    }
}
