use serde::{Deserialize, Serialize};

use crate::errors::{MyError, Result};
pub fn parse_text(text: String) -> String {
    text.to_string()
}
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
        let ans = ForwardMessage::new(chat_id, text);
        self.client
            .post(url)
            .json(&ans)
            .send()
            .await
            .map_err(|_| MyError::ReqwestError)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForwardMessage {
    chat_id: i64,
    text: String,
}
impl ForwardMessage {
    pub fn new(chat_id: i64, text: String) -> Self {
        Self { chat_id, text }
    }
}
