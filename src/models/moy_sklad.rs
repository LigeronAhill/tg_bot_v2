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
impl Event {}
