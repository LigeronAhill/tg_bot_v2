use crate::errors::{MyError, Result};
use crate::models::tg::{MessageToBot, Update};

#[derive(Clone)]
pub struct Bot {
    pub token: String,
    pub api_url: String,
    pub client: reqwest::Client,
}
impl Bot {
    pub fn new(token: String) -> Self {
        Self {
            token: token.clone(),
            api_url: format!("https://api.telegram.org/bot{}/", token.clone()),
            client: reqwest::Client::new(),
        }
    }
    pub async fn send_message(&self, chat_id: i64, text: String) -> Result<()> {
        let method = "sendMessage";
        let url = format!("{}{}", self.api_url, method);
        let ans = MessageToBot::new(chat_id, text);
        self.client
            .post(url)
            .json(&ans)
            .send()
            .await
            .map_err(|_| MyError::ReqwestError)?;
        Ok(())
    }
}
impl Update {
    pub fn parse_commands(&self) -> (bool, String) {
        if let Some(message) = self.message.to_owned() {
            if let Some(entities) = message.entities {
                for entity in entities {
                    if entity.entity_type == "bot_command" {
                        return (true, message.text);
                    }
                }
            }
        }
        (false, String::new())
    }

    pub fn parse_text(&self) -> Result<(String, i32)> {
        if let Some(message) = self.message.to_owned() {
            if message.text.chars().count() < 4 {
                return Err(MyError::Static(String::from(
                    "Слишком мало букав! Введите хотя бы 4 начальные буквы названия коллекции.",
                )));
            }
            let parts_of_message = message.text.split_whitespace();
            let mut words = vec![];
            let mut numbers = vec![];
            for part in parts_of_message {
                if part.parse::<i32>().is_ok() {
                    numbers.push(part.parse::<i32>().unwrap());
                } else {
                    match part.len() < 4 {
                        true => {
                            return Err(MyError::Static(String::from(
                                "Слишком мало букв для поиска",
                            )));
                        }
                        false => words.push(part.to_string()),
                    }
                }
            }

            if words.len() > 1 || numbers.len() > 1 {
                return Err(MyError::Static(String::from(
                    "Слишком много запросов... Введите коллекцию и цвет.",
                )));
            }
            Ok((words[0].to_string(), numbers[0]))
        } else {
            Err(MyError::Static(String::from(
                "Некорректное сообщение. Введите коллекцию и цвет.",
            )))
        }
    }
    pub fn parse_file(&self) -> Result<String> {
        if let Some(msg) = self.message.clone() {
            if let Some(doc) = msg.document {
                return Ok(doc.file_id);
            }
        }
        Err(MyError::Static(String::from("no file")))
    }
}
