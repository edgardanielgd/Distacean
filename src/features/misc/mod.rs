use crate::features::FeatureDescription;
use serenity::{model::channel::Message, prelude::*};

pub mod avatar;

pub fn get_description() -> FeatureDescription {
    FeatureDescription {
        name: "Misc".to_string(),
        description: "Miscellaneous commands".to_string(),
        aliases: vec!["misc".to_string()],
        examples: vec!["!av".to_string()],
        subfeatures: vec![avatar::get_description()],
    }
}

pub async fn on_message(ctx: Context, command: String, msg: Message) {
    let ctx = ctx.clone();
    let msg = msg.clone();
    tokio::spawn(async move {
        avatar::on_message(ctx, command, msg).await;
    });
}
