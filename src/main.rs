use dotenvy::dotenv;
use serenity::prelude::*;
use std::env;

mod features;
mod handlers;

#[tokio::main]
async fn main() {
    // Load the .env file if it exists
    dotenv().ok();

    // Login with a bot token from the environment
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(handlers::Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
