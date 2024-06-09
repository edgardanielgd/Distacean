use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::features::{misc::MiscFeature, Feature};

pub struct Handler;

// Bind discord events to this handler
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message is from the bot itself
        if msg.author.bot {
            return;
        }

        // Check if the message is a command
        if msg.content.starts_with("!") {
            MiscFeature.on_message(ctx, msg).await;
        }
    }
}
