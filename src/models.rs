pub mod market;
pub mod moy_sklad;
pub mod tg;
pub mod wordpress;
use anyhow::Result;
use mongodb::{bson::oid::ObjectId, Database};
use serde::{Deserialize, Serialize};

use self::tg::MessageToBot;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: Option<i32>,
    pub stock: Option<f64>,
}
impl Product {
    pub async fn new(name: String) -> Self {
        Product {
            id: None,
            name,
            price: None,
            stock: None,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub bot: Bot,
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
        let ans = MessageToBot::new(chat_id, text);
        self.client.post(url).json(&ans).send().await?;
        Ok(())
    }
}
