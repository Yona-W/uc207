use std::{fs, error::Error, collections::HashMap, path::PathBuf};
use serde::{Serialize, Deserialize};

use super::api::Message;

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub char_name: String,
    pub char_description: String,
    pub char_persona: String,
    pub example_dialogue: Vec<Message>,
    pub avatar_url: String,
}

impl Character{
    pub fn load_all(characters_dir: &str) -> Result<HashMap<String, Character>, Box<dyn Error>> {
        let char_files = fs::read_dir(characters_dir)?;
        let mut char_dict = HashMap::new();
        for char_file_result in char_files {
            let char_file = match char_file_result {
                Ok(file) => file,
                Err(err) => {
                    println!("Error finding file: {:?}", err);
                    continue;
                }
            };

            let path = char_file.path();
            let id = 
                char_file.file_name()
                .to_str().unwrap_or("")
                .split(".").nth(0).unwrap_or("")
                .to_string();

            let loaded_character = match load_character(&path) {
                Ok(character) => character,
                Err(err) => {
                    println!("Error loading character with ID {} - {:?}", id, err);
                    continue;
                }
            };

            println!("Loading character {} with ID {}", loaded_character.char_name, id);
            char_dict.insert(id, loaded_character);
        }

        fn load_character(char_path: &PathBuf) -> Result<Character, Box<dyn Error>> {
            let json = fs::read_to_string(char_path)?;
            let character: Character = serde_json::from_str(&json)?;
            return Ok(character);
        }

        return Ok(char_dict);
    }
}