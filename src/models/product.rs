use crate::errors::{MyError, Result};
use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: i32,
    pub stock: Vec<f64>,
    pub ms_id: Uuid,
    pub created_at: DateTime<Local>,
    pub active: bool,
}
#[derive(Debug, Default)]
pub struct ProductBuilder {
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub price: Option<i32>,
    pub stock: Vec<f64>,
    pub ms_id: Option<Uuid>,
    pub created_at: Option<DateTime<Local>>,
    pub active: bool,
}

impl ProductBuilder {
    pub fn new() -> Self {
        ProductBuilder::default()
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn price(mut self, price: impl Into<i32>) -> Self {
        self.price = Some(price.into());
        self
    }
    pub fn stock(mut self, stock: impl Into<f64>) -> Self {
        self.stock.push(stock.into());
        self
    }
    pub fn ms_id(mut self, ms_id: impl Into<String>) -> Self {
        self.ms_id = Uuid::parse_str(&ms_id.into()).ok();
        self
    }
    pub fn build(self) -> Result<Product> {
        let Some(name) = self.name.clone() else {
            return Err(MyError::Static(String::from("No name!")));
        };
        let price = self.price.unwrap_or(0);
        let stock = self.stock.clone();
        let Some(ms_id) = self.ms_id else {
            return Err(MyError::Static(String::from("No ms_id!")));
        };
        Ok(Product {
            id: self.id,
            name,
            price,
            stock,
            ms_id,
            created_at: Local::now(),
            active: true,
        })
    }
}
