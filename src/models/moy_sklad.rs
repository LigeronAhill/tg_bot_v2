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
    pub action: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
}

// {
//     "auditContext":
//         {
//             "meta":
//                 {
//                     "type":"audit",
//                     "href":"https://api.moysklad.ru/api/remap/1.2/audit/cd294b42-8540-11ee-0a80-098d0010c4a0"
//                 },
//             "uid":"admin@provotorov2",
//             "moment":"2023-11-17 14:59:46"
//         },
//     "events":[
//         {
//             "meta":
//                 {
//                     "type":"product",
//                     "href":"https://api.moysklad.ru/api/remap/1.2/entity/product/491735b5-d619-11ed-0a80-0f1c001b53bb"
//                 },
//             "action":"UPDATE",
//             "accountId":"7feaddfe-d5e2-11ed-0a80-06c30000973f"
//         }
//     ]
// }
