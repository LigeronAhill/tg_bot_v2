use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignsResponse {
    pub campaigns: Vec<Campaign>,
    pub pager: Pager,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    pub domain: String,
    pub id: i64,
    pub client_id: i64,
    pub business: Business,
    pub placement_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pager {
    pub total: i64,
    pub from: i64,
    pub to: i64,
    pub current_page: i64,
    pub pages_count: i64,
    pub page_size: i64,
}
