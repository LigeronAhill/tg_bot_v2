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
    pub admin: i64,
    pub group: i64,
}
impl Bot {
    pub fn new(token: &str, admin: &str, group: &str) -> Self {
        Self {
            token: token.to_owned(),
            api_url: format!("https://api.telegram.org/bot{}/", token),
            client: reqwest::Client::new(),
            admin: admin.parse().unwrap(),
            group: group.parse().unwrap(),
        }
    }
    pub async fn send_message_admin(&self, text: &str) -> Result<()> {
        let method = "sendMessage";
        let url = format!("{}{}", self.api_url, method);
        let ans = ForwardMessage::new(self.admin, text.to_owned());
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
