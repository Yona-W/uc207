use serenity::{builder::{self, CreateInteractionResponseData}, model::prelude::{command::CommandOptionType, interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue}}};

use crate::botmanager::BotManager;

pub fn register (command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand
{
    command
        .name("invite")
        .description("Invite a character to this channel")
        .create_option(|option| {
            option
                .name("id")
                .description("The character's ID")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub fn run (command: &ApplicationCommandInteraction, manager: &BotManager, msg: &mut CreateInteractionResponseData){
    let mut data = manager.data.lock().unwrap();
    let options = &command.data.options;
    let character_id : &str = match options.get(0) {
        Some(opt) => {
            if let CommandDataOptionValue::String(id_str) = opt.resolved.as_ref().expect("Expected character ID field"){
                id_str as &str
            }
            else{
                msg.content("Expected character ID!");
                return;
            }
        }
        None => {
            msg.content("Expected character ID!");
            return;
        }
    };

    let selected_character = data.characters.get(character_id);
    let mut do_insert = false;

    match selected_character {
        Some(char) => {
            do_insert = true;
            msg.embed(|e| { e
                .title("Character invited!")
                .description([&char.char_name, " will now respond in this channel!"].join(""))
                .image(&char.avatar_url)
        });}
        None => {msg.content("The selected character ID doesn't exist!");}

    };
    if do_insert {
        data.invited_characters.insert(command.channel_id, String::from(character_id));
    }
}