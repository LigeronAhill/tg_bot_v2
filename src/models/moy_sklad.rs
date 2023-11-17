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
    pub async fn test_api(self, token: String) -> String {
        let client = reqwest::Client::builder().gzip(true).build().unwrap();
        let uri = self.meta.href;
        let response_body = client.get(uri).bearer_auth(token).send().await.unwrap();
        let result = response_body
            .text()
            .await
            .unwrap_or("cant get text from response".to_string());
        result
    }
}
