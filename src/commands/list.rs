use serenity::{builder::{self, CreateInteractionResponseData}, model::prelude::{command::CommandOptionType, interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue}}};

use crate::{botmanager::BotManager};

pub fn register (command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand
{
    command
        .name("list")
        .description("List registered bots")
        .create_option(|option| {
            option
                .name("page")
                .description("Which page of the list to display")
                .kind(CommandOptionType::Integer)
                .required(false)
        })
}

pub fn run (command: &ApplicationCommandInteraction, manager: &BotManager, msg: &mut CreateInteractionResponseData){
    let data = manager.data.lock().unwrap();
    let options = &command.data.options;
    let start_index = match options.get(0) {
        Some(opt) => {
            match opt.resolved.as_ref() {
                Some(CommandDataOptionValue::Integer(val)) => val as &i64,
                Some(_) => &0,
                None => &0
            }
        }
        None => &0
    };

    msg.embed(|e| { e
        .title("Bot profile List")
        .description("Use `/invite` to invite one of these bots to the current channel!");
        let mut index = 0;
        let mut added = 0;
        for (id, character) in &data.characters {
            if added > 25 {
                break;
            }
            if &index >= start_index {
                e.field(&character.char_name, ["`/invite ", &id, "`\n", &character.char_description].join(""), true);
                added += 1;
            }
            index += 1;
        }
        e
    });
}