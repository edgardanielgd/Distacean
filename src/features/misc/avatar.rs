use crate::features::FeatureDescription;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::{model::channel::Message, prelude::*};

pub fn get_description() -> FeatureDescription {
    FeatureDescription {
        name: "Avatar".to_string(),
        description: "Get a user's avatar!".to_string(),
        aliases: vec!["avatar".to_string(), "av".to_string()],
        examples: vec!["!avatar".to_string()],
        subfeatures: vec![],
    }
}

pub async fn on_message(ctx: Context, command: String, msg: Message) {
    if command.starts_with("av") {
        let user_avatar = match msg.author.avatar_url() {
            Some(url) => url,
            None => "https://img.freepik.com/free-vector/illustration-share-icon_53876-5622.jpg"
                .to_string(),
        };

        let result = msg.channel_id.send_message(
            &ctx.http,
            CreateMessage::new().embed(
                CreateEmbed::default()
                    .title("Your avatar")
                    .image(user_avatar),
            ),
        );

        if let Err(why) = result.await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
}
