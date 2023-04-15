use std::collections::HashMap;
use std::sync::Mutex;

use serenity::http::Typing;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::{Message, Ready, GuildId, ChannelId, Activity};
use serenity::prelude::{Context, EventHandler};
use serenity::{async_trait};

use crate::commands;
use crate::textgen::api::{TextgenApi};
use crate::textgen::character::Character;

pub struct BotManager
{
    pub api: TextgenApi,
    pub data: Mutex<BotManagerData>
}

pub struct BotManagerData
{
    pub characters: HashMap<String, Character>,
    pub invited_characters: HashMap<ChannelId, String>
}

#[async_trait]
impl EventHandler for BotManager {
    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let model = self.api.check_model().await.expect("Failed connecting to textgen API");
        println!("Using model: {}", model);

        context.set_activity(Activity::playing(model)).await;

        for guild in ready.guilds {
            println!("Registering commands for server: {:?}", guild.id.name(&context.cache).ok_or("UNKNOWN"));
            GuildId::set_application_commands(&guild.id, &context.http, |commands| {
                commands
                    .create_application_command(|cmd| commands::list::register(cmd))
                    .create_application_command(|cmd| commands::invite::register(cmd))
                    .create_application_command(|cmd| commands::uninvite::register(cmd))
            }).await.expect("Failed registering commands");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        match command.data.name.as_str() {
                            "invite" => {commands::invite::run(&command, &self, message)},
                            "uninvite" => {commands::uninvite::run(&command, &self, message)},
                            "list" => {commands::list::run(&command, &self, message)},
                            _ => {message.content("Command not implemented");}
                        };
                        return message;
                    })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn message(&self, context: Context, msg: Message) {
        if msg.author.bot {
            return; // No infinite loops pls
        }

        let messages = msg.channel_id.messages(&context, |builder| builder.limit(15)).await.expect("Failed getting message history");
        let cache = &context.cache;
        let history : Vec<crate::textgen::api::Message> = messages.iter()
            .map(|discord_msg| crate::textgen::api::Message {
                speaker: String::from(&discord_msg.author.name),
                content: discord_msg.content_safe(cache)
            })
            .filter(|message| !message.content.is_empty())
            .rev()
            .collect();

        let mut prompt = None;
        {
            let data = self.data.try_lock().unwrap();
            let character_in_channel = data.invited_characters.get(&msg.channel_id);
            match character_in_channel {
                Some(character) => {
                    let character_def = data.characters.get(character).expect("Character is invited but not loaded");
                    prompt = Some(self.api.make_prompt(character_def, &history).expect("Failed making prompt"));
                },
                None => return
            };
        }


        match prompt {
            Some(prompt_value) => {
                let typing = msg.channel_id.start_typing(&context.http).expect("Failed saying I'm typing");
                let result = match self.api.request(prompt_value).await
                {
                    Ok(res) => res,
                    Err(err) => {println!("Received no response from API: {:?}", err); String::from("*No response*")}
                };
                msg.channel_id.send_message(&context, |msg| msg.content(result)).await.expect("Error sending message");
                typing.stop().expect("Failed saying I'm no longer typing");
            }
            None => {return;}
        }
    }
}