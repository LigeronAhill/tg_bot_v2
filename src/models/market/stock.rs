use chrono::{DateTime, Local, SubsecRound};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StocksResponse {
    pub skus: Vec<StockDTO>,
}
impl StocksResponse {
    pub fn test(request: StocksRequest) -> Self {
        let count: i64 = 69;
        let mut skus: Vec<StockDTO> = vec![];
        let now = Local::now().round_subsecs(0);
        for sku in request.skus {
            let response_sku = StockDTO {
                sku,
                warehouse_id: request.warehouse_id,
                items: vec![StockItemDTO {
                    count,
                    stock_type: StockType::FIT,
                    updated_at: now,
                }],
            };
            skus.push(response_sku)
        }
        Self { skus }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockDTO {
    pub sku: String,
    pub warehouse_id: i64,
    pub items: Vec<StockItemDTO>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockItemDTO {
    pub count: i64,
    #[serde(rename = "type")]
    pub stock_type: StockType,
    pub updated_at: DateTime<Local>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StockType {
    #[default]
    FIT,
}
// ------------------FROM MARKET------------------------------------
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StocksRequest {
    pub warehouse_id: i64,
    pub skus: Vec<String>,
}
