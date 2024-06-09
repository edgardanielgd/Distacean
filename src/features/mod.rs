use serenity::{model::channel::Message, prelude::*};

pub mod misc;

pub struct FeatureDescription {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub examples: Vec<String>,

    pub subfeatures: Vec<FeatureDescription>,
}

pub trait Feature {
    async fn on_message(&self, ctx: Context, msg: Message);
}
