use serde::{Deserialize, Serialize};

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
    pub fn token(&self) -> String {
        self.token.clone()
    }
    pub async fn send_message_admin(&self, text: &str) -> anyhow::Result<()> {
        let method = "sendMessage";
        let url = format!("{}{}", self.api_url, method);
        let ans = ForwardMessage::new(self.admin, text.to_owned());
        self.client.post(url).json(&ans).send().await?;
        Ok(())
    }
    pub async fn send_message_group(&self, text: &str) -> anyhow::Result<()> {
        let method = "sendMessage";
        let url = format!("{}{}", self.api_url, method);
        let ans = ForwardMessage::new(self.group, text.to_owned());
        self.client.post(url).json(&ans).send().await?;
        Ok(())
    }
    pub async fn send_message_repl(&self, chat_id: i64, text: &str) -> anyhow::Result<()> {
        let method = "sendMessage";
        let url = format!("{}{}", self.api_url, method);
        let ans = ForwardMessage::new(chat_id, text.to_owned());
        self.client.post(url).json(&ans).send().await?;
        Ok(())
    }
    pub async fn get_file(&self, file_id: &str) -> anyhow::Result<String> {
        let method = "getFile";
        let url = format!("{}{}", self.api_url, method);
        let file = crate::models::telegram::update::File {
            file_id: file_id.to_owned(),
            file_path: None,
        };
        let response: crate::models::telegram::update::FileResponse = self
            .client
            .post(&url)
            .json(&file)
            .send()
            .await?
            .json()
            .await?;
        response
            .result
            .file_path
            .ok_or(anyhow::Error::msg("error getting file"))
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
