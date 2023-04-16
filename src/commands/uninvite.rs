use serenity::{builder::{self, CreateInteractionResponseData}, model::prelude::interaction::application_command::ApplicationCommandInteraction};

use crate::botmanager::{BotManager};

pub fn register (command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand
{
    command
        .name("uninvite")
        .description("Uninvite the current bot from this channel")
}

pub fn run (command: &ApplicationCommandInteraction, manager: &BotManager, msg: &mut CreateInteractionResponseData){
    let mut data = manager.data.lock().unwrap();

    let selected_character = data.invited_characters.get(&command.channel_id);

    match selected_character {
        Some(_) => {
            data.invited_characters.remove(&command.channel_id);
            msg.content("Bot uninvited!");
        }
        None => {msg.content("There is no active bot in this channel!");}
    };
}