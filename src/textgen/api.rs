use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::{fs, error::Error, time::Duration};

use super::character::Character;

#[derive(Serialize, Deserialize)]
pub struct TextgenApi {
    #[serde(skip)]
    client: Client,
    textgen_url: String,
    model_url: String,
    temperature: f32,
    top_p: f32,
    typical_p: f32,
    repetition_penalty: f32,
    encoder_repetition_penalty: f32,
    top_k: f32,
    min_length: i32,
    no_repeat_ngram_size: i32,
    num_beams: i32,
    penalty_alpha: f32,
    length_penalty: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub speaker: String,
    pub content: String
}

impl TextgenApi{
    pub fn init(config_path: &str) -> Result<TextgenApi, Box<dyn Error>> {
        let json = fs::read_to_string(config_path)?;
        let mut textgen_api: TextgenApi = serde_json::from_str(&json)?;
        textgen_api.client = reqwest::Client::new();
        
        return Ok(textgen_api);
    }

    pub fn make_prompt(&self, character: &Character, history: &Vec<Message>) -> Result<String, Box<dyn Error>> {
        let patterns = &[
            "[[NAME]]",
            "[[PERSONA]]",
            "[[EXAMPLE]]",
            "[[CONTEXT]]"
        ];
        let replace = &[
            &character.char_name,
            &character.char_persona,
            &Message::format_conversation(&character.example_dialogue),
            &Message::format_conversation(&history)
        ];
        let template = fs::read_to_string("prompt_template.txt")?;

        let filled_template = aho_corasick::AhoCorasick::new(patterns)
            .replace_all(&template, replace);

        Ok(filled_template)
    }

    pub async fn check_model(&self) -> Option<String> {
        let url = &self.model_url;
        let response = match self.client.get(url).send().await{
            Ok(resp) => resp,
            Err(err) => {
                println!("Couldn't connect to textgen endpoint: {:?}", err);
                return None;
            }
        };
        let text = match response.text().await {
            Ok(text) => {text},
            Err(err) => {
                println!("Couldn't get text from textgen endpoint: {:?}", err);
                return None;
            }
        };
        let json: Value = serde_json::from_str(&text).expect("Couldn't deserialize response from textgen endpoint");
        Some(String::from(json["result"].as_str().expect("Result wasn't a string")))
    }

    pub async fn request(&self, prompt: String) -> Result<String, Box<dyn Error>> {
        /*
        let body = json!({
            "temperature": self.temperature,
            "top_p": self.top_p,
            "typical": self.typical_p,
            "rep_pen": self.repetition_penalty,
            "encoder_repetition_penalty": self.encoder_repetition_penalty,
            "top_k": self.top_k,
            "min_length": self.min_length,
            "no_repeat_ngram_size": self.no_repeat_ngram_size,
            "num_beams": self.num_beams,
            "penalty_alpha": self.penalty_alpha,
            "length_penalty": self.length_penalty,
            "seed": rand::random::<i64>(),
            "prompt": prompt
        }).to_string();
        */

        let cursed_inner_json_string = json!(
            [ 
                prompt,
                { 
                    "max_new_tokens": 200, 
                    "do_sample": true, 
                    "temperature": self.temperature, 
                    "top_p": self.top_p, 
                    "typical_p": self.typical_p, 
                    "repetition_penalty": self.repetition_penalty, 
                    "encoder_repetition_penalty": self.encoder_repetition_penalty, 
                    "top_k": self.top_k, 
                    "min_length": self.min_length, 
                    "no_repeat_ngram_size": self.no_repeat_ngram_size, 
                    "num_beams": self.num_beams, 
                    "penalty_alpha": self.penalty_alpha, 
                    "length_penalty": self.length_penalty, 
                    "early_stopping": false, 
                    "seed": -1,
                    "add_bos_token":false,
                    "truncation_length":2000,
                    "custom_stopping_strings":[],
                    "ban_eos_token":true
                } 
            ] 
        ).to_string();

        let body = json!(
            {
                "data": [cursed_inner_json_string]
            }
        ).to_string();

        println!("Sending API request: {}", body);

        let url = &self.textgen_url;
        let response = self.client.post(url)
            .timeout(Duration::from_secs(20))
            .body(body)
            .send()
            .await?;

        let response_text = response.text().await.expect("No text?");
        println!("{}", response_text);
        let json: Value = serde_json::from_str(&response_text).expect("Couldn't deserialize response");
        let text = match json["data"][0].as_str() {
            Some(value) => value,
            None => return Err(string_error::new_err("API returned no result"))
        };

        let response_only = text.replace(&prompt, "");

        return Ok(String::from(response_only));
    }
}

impl Message {
    pub fn to_string(&self) -> String
    {
        [&self.speaker, ": ", &self.content].join("")
    }

    pub fn format_conversation(messages: &Vec<Message>) -> String
    {
        let collection: Vec<String> = messages.into_iter()
            .map(|msg| msg.to_string())
            .collect();
        collection.join("\n")
    }
}