use serde::Deserialize;
use serde::Serialize;

use self::product::ProductFromMoySklad;

use super::AppState;
pub mod product;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    pub audit_context: AuditContext,
    pub events: Vec<Event>,
}

impl Audit {
    pub async fn test_get_product(&self, app_state: AppState) -> String {
        let mut result: Vec<String> = vec![];
        for event in &self.events {
            let uri = event.meta.href.as_ref().unwrap();
            let client = reqwest::Client::new();
            let Ok(response) = client
                .get(uri)
                .bearer_auth(&app_state.tokens.ms_token)
                .send()
                .await
            else {
                return String::from("Reqwest failed");
            };
            let Ok(product) = response.json::<ProductFromMoySklad>().await else {
                return String::from("Can't parse JSON");
            };
            let str_product = format!("{product:#?}\n");
            result.push(str_product)
        }
        result.join("\n")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditContext {
    pub meta: Meta,
    pub uid: String,
    pub moment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub href: Option<String>,
    pub metadata_href: Option<String>,
    pub media_type: Option<String>,
    pub uuid_href: Option<String>,
    pub download_href: Option<String>,
    pub size: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub meta: Meta,
    pub action: String,
    pub account_id: String,
}
