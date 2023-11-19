use serde::{Deserialize, Serialize};

use crate::errors::{MyError, Result};
use crate::models::tgapi::message_kinds::media_kinds::{MediaKind, MediaText};
use crate::models::tgapi::message_kinds::{MessageCommon, MessageKind};
use crate::models::tgapi::{ChatId, Message, Update};
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
impl Update {
    pub async fn filter_msg(&self) -> String {
        match self.kind.clone() {
            crate::models::tgapi::UpdateKind::Message(msg) => match msg.text() {
                Some(text) => text.to_string(),
                None => "kawabangaa".to_string(),
            },
            _ => "the winter is close".to_string(),
        }
    }
}

impl Message {
    pub fn chat_id(&self) -> ChatId {
        self.chat.id.clone()
    }
    pub fn text(&self) -> Option<&str> {
        match &self.kind {
            MessageKind::Common(MessageCommon {
                media_kind: MediaKind::Text(MediaText { text, .. }),
                ..
            }) => Some(text),
            _ => None,
        }
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_text_works() {
        let text = vec![
            ("fedo 27", "fedo 27"),
            ("hi", "hi"),
            ("fd 196", "fd 196"),
            ("ben", "ben"),
            ("bently 105", "bently 105"),
            ("uralmash 123 inache vseh", "uralmash 123 inache vseh"),
            ("henry ford 1900 2023", "henry ford 1900 2023"),
            ("messa 03", "messa 03"),
            ("lyrica 8B72", "lyrica 8B72"),
            ("аврора светло-бежевый", "аврора светло-бежевый"),
        ];
        for frase in text {
            if frase.0 != parse_text(frase.1.to_string()).as_str() {
                panic!(
                    "{} ne {}",
                    frase.0.to_string(),
                    parse_text(frase.1.to_string()),
                )
            }
        }
    }
}
