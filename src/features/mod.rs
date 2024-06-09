use serenity::model::channel::Message;
use serenity::prelude::*;

pub mod misc;

pub struct FeatureDescription {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub examples: Vec<String>,

    pub subfeatures: Vec<FeatureDescription>,
}

pub fn get_description() -> FeatureDescription {
    FeatureDescription {
        name: "Features".to_string(),
        description: "Features".to_string(),
        aliases: vec![],
        examples: vec![],
        subfeatures: vec![misc::get_description()],
    }
}

pub async fn on_message(ctx: Context, command: String, msg: Message) {
    let ctx = ctx.clone();
    let msg = msg.clone();
    tokio::spawn(async move {
        misc::on_message(ctx, command, msg).await;
    });
}
