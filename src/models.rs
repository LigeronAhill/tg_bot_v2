pub mod market;
pub mod moy_sklad;
pub mod tg;
pub mod wordpress;
use mongodb::{bson::oid::ObjectId, Database};
use serde::{Deserialize, Serialize};

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
    pub token: String,
}
