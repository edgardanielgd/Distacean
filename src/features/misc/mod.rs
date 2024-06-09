pub struct MiscFeature;

use crate::features::{Feature, FeatureDescription};
use serenity::{model::channel::Message, prelude::*};

impl Feature for MiscFeature {
    async fn on_message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}
pub fn get_description() -> FeatureDescription {
    FeatureDescription {
        name: "Misc".to_string(),
        description: "Miscellaneous commands".to_string(),
        aliases: vec!["misc".to_string()],
        examples: vec!["!ping".to_string()],
        subfeatures: vec![],
    }
}
