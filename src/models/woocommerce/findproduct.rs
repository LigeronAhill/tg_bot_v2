use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindProduct {
    pub id: i64,
    pub name: String,
    pub date_modified: String,
    pub sku: String,
    pub price: String,
    pub regular_price: String,
    pub sale_price: String,
    pub stock_quantity: i64,
    pub variations: Vec<i64>,
}
