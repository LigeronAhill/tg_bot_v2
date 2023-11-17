use crate::errors::{MyError, Result};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Audit {
    #[serde(rename = "auditContext")]
    pub audit_context: AuditContext,
    pub events: Vec<Event>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditContext {
    pub meta: Meta,
    pub uid: String,
    pub moment: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    #[serde(rename = "type")]
    pub audit_type: String,
    pub href: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub meta: Meta,
    pub action: Action,
    #[serde(rename = "accountId")]
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    CREATE,
    UPDATE,
    DELETE,
}
impl Event {
    pub async fn test_api(self, token: String) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let uri = self.meta.href;
        let response_body = client
            .get(uri)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|_| MyError::Static(String::from("request error")))?
            .json::<serde_json::Value>()
            .await
            .map_err(|_| MyError::Static(String::from("json error")))?;
        Ok(response_body)
    }
}
