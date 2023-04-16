use serenity::{builder::{self, CreateInteractionResponseData}, model::prelude::interaction::application_command::ApplicationCommandInteraction};

use crate::botmanager::{BotManager};

pub fn register (command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand
{
    command
        .name("fence")
        .description("Create a fence - Bots won't see any messages on the other side of the fence")
}

pub fn run (command: &ApplicationCommandInteraction, manager: &BotManager, msg: &mut CreateInteractionResponseData){
    msg.content("--- Message Fence ---\nBots won't see any messages above this one!");
}