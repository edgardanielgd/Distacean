use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::{features, handlers::Handler};

// Bind discord events to this handler
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if author is not a bot
        if msg.author.bot {
            return;
        }

        // Check if message starts with a valid prefix
        let cropped_command =
            match crate::utils::messages_checks::check_message_prefix(&msg.content) {
                Ok(message) => message,
                Err(_) => return,
            };

        features::on_message(ctx, cropped_command, msg).await;
    }
}
