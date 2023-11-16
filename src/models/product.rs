use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: Option<i32>,
    pub stock: Option<Vec<f64>>,
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
