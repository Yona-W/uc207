mod botmanager;
mod commands;
mod textgen;

use std::collections::HashMap;
use std::sync::Mutex;

use botmanager::{BotManagerData};
use serenity::prelude::{GatewayIntents};
use serenity::{Client};
use textgen::api::TextgenApi;
use textgen::character::Character;

#[tokio::main]
async fn main() {
    let token = match std::env::var("DISCORD_TOKEN"){
        Ok(str) => str,
        Err(_) => panic!("Missing DISCORD_TOKEN environment variable")
    };

    let api = TextgenApi::init("config.json").expect("Unable to initialize textgn API");
    let characters = Character::load_all("characters").expect("Error loading characters");
    let invited_characters = HashMap::new();

    let manager_data = BotManagerData {
        characters: characters,
        invited_characters: invited_characters
    };

    let mut client = Client::builder(&token, 
            GatewayIntents::MESSAGE_CONTENT |
            GatewayIntents::DIRECT_MESSAGES |
            GatewayIntents::GUILD_MEMBERS |
            GatewayIntents::GUILD_MESSAGES |
            GatewayIntents::GUILD_WEBHOOKS |
            GatewayIntents::GUILDS
        )
        .event_handler(botmanager::BotManager
            {
                api: api,
                data: Mutex::new(manager_data)
            }
        )
        .await.expect("Error creating client");

    if let Err(error) = client.start().await {
        panic!("Error starting client: {:?}", error);
    }
}