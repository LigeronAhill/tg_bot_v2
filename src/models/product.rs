use super::moy_sklad::product::ProductFromMoySklad;
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
    pub article: String,
    pub width: Vec<String>,
    pub stock: Vec<f64>,
    pub category: String,
    pub ms_id: Uuid,
    pub variants: i64,
    pub created_at: DateTime<Local>,
    pub active: bool,
}
impl Product {
    pub fn from_ms(product: ProductFromMoySklad) -> Result<Self> {
        let mut base_price = 0;
        // TODO: currencies!!!
        // TODO: discount!!!
        // TODO: archived!!!
        for price in product.sale_prices {
            if price.price_type.name.as_str() == "Цена продажи" {
                base_price = (price.value / 100.00) as i32;
            }
        }
        let article = match product.article {
            Some(art) => art,
            None => String::new(),
        };
        let mut width = String::new();
        if let Some(attributes) = product.attributes {
            for attribute in attributes {
                if attribute.name.as_str() == "Ширина рулона, м" {
                    width = attribute
                        .value
                        .to_string()
                        .replace("\"", "")
                        .replace("\\", "")
                }
            }
        }
        let result = ProductBuilder::new()
            .name(product.name)
            .price(base_price)
            .article(article)
            .width(width)
            .ms_id(product.id)
            .category(product.path_name)
            .variants(product.variants_count)
            .build()?;
        Ok(result)
    }
}
#[derive(Debug, Default)]
pub struct ProductBuilder {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub article: Option<String>,
    pub width: Vec<String>,
    pub stock: Vec<f64>,
    pub category: Option<String>,
    pub ms_id: Option<Uuid>,
    pub variants: Option<i64>,
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
    pub fn article(mut self, article: impl Into<String>) -> Self {
        self.article = Some(article.into());
        self
    }
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width.push(width.into());
        self
    }
    pub fn stock(mut self, stock: impl Into<f64>) -> Self {
        self.stock.push(stock.into());
        self
    }
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
    pub fn ms_id(mut self, ms_id: impl Into<String>) -> Self {
        self.ms_id = Uuid::parse_str(&ms_id.into()).ok();
        self
    }
    pub fn variants(mut self, variants: i64) -> Self {
        self.variants = Some(variants);
        self
    }
    pub fn build(self) -> Result<Product> {
        let Some(name) = self.name.clone() else {
            return Err(MyError::Static(String::from("No name!")));
        };
        let Some(article) = self.article.clone() else {
            return Err(MyError::ProductBuildError);
        };
        let Some(category) = self.category.clone() else {
            return Err(MyError::Static(String::from("No category!")));
        };
        let price = self.price.unwrap_or(0);
        let stock = self.stock.clone();
        let width = self.width;
        let Some(ms_id) = self.ms_id else {
            return Err(MyError::Static(String::from("No ms_id!")));
        };
        let Some(variants) = self.variants else {
            return Err(MyError::Static(String::from("No variants!")));
        };
        Ok(Product {
            id: None,
            name,
            price,
            article,
            width,
            stock,
            category,
            ms_id,
            variants,
            created_at: Local::now(),
            active: true,
        })
    }
}
