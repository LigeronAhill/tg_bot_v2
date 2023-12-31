use crate::errors::{MyError, Result};
use crate::models::moy_sklad::Event;
use crate::models::product::Product;
use crate::models::{AppState, AttributeDB, CategoryDB};
use futures::TryStreamExt;

use mongodb::{
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
    Collection, Database, IndexModel,
};
use serde::{Deserialize, Serialize};
// use serde::{Deserialize, Serialize};

pub const PRODUCT_COL: &str = "product";
pub const EVENT_COL: &str = "event";
pub const CATEGORIES_COL: &str = "category";
pub const ATTRIBUTES_COL: &str = "attribute";
pub const STOCK_COL: &str = "stock";

#[derive(Clone, Debug)]
pub struct Storage {
    product: Collection<Product>,
    event: Collection<Event>,
    category: Collection<CategoryDB>,
    attribute: Collection<AttributeDB>,
    stock: Collection<Stock>,
}
impl Storage {
    pub async fn new(db: &Database) -> Self {
        Self {
            product: db.collection::<Product>(PRODUCT_COL),
            event: db.collection::<Event>(EVENT_COL),
            category: db.collection::<CategoryDB>(CATEGORIES_COL),
            attribute: db.collection::<AttributeDB>(ATTRIBUTES_COL),
            stock: db.collection::<Stock>(STOCK_COL),
        }
    }
    pub async fn name_index_create(&self) -> Result<()> {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(options.clone())
            .build();
        self.product
            .create_index(model.clone(), None)
            .await
            .map_err(|_| MyError::DbError)?;
        self.category
            .create_index(model.clone(), None)
            .await
            .map_err(|_| MyError::DbError)?;
        self.attribute
            .create_index(model.clone(), None)
            .await
            .map_err(|_| MyError::DbError)?;
        Ok(())
    }
    pub async fn create_product(&self, product: Product) -> Result<Product> {
        let inserted_product = self
            .product
            .insert_one(product, None)
            .await
            .map_err(|_| MyError::DbError)?;
        match self
            .product
            .find_one(doc! {"_id": inserted_product.inserted_id}, None)
            .await
        {
            Ok(Some(product)) => Ok(product),
            _ => Err(MyError::DbError),
        }
    }
    pub async fn find_all_products(&self) -> Result<Vec<Product>> {
        let mut cursor = self
            .product
            .find(None, None)
            .await
            .map_err(|_| MyError::DbError)?;
        let mut result: Vec<Product> = vec![];
        while let Some(product) = cursor.try_next().await.map_err(|_| MyError::DbError)? {
            result.push(product)
        }
        Ok(result)
    }
    pub async fn find_product_by_id(&self, id: String) -> Result<Product> {
        let oid = ObjectId::parse_str(id).map_err(|_| MyError::DbError)?;
        match self.product.find_one(doc! {"_id": oid}, None).await {
            Ok(Some(product)) => Ok(product),
            _ => Err(MyError::DbError),
        }
    }
    pub async fn find_product_by_name(&self, name: String) -> Result<Vec<Product>> {
        let mut cursor = self
            .product
            .find(doc! {"name":{"$regex": name, "$options": "i"}}, None)
            .await
            .map_err(|_| MyError::DbError)?;
        let mut result: Vec<Product> = vec![];
        while let Some(product) = cursor.try_next().await.map_err(|_| MyError::DbError)? {
            result.push(product)
        }
        Ok(result)
    }
    pub async fn update_product(&self, id: String, upd_product: Product) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| MyError::DbError)?;
        let filter = doc! {"_id": oid};
        let new_product = doc! {
            "$set":
            {
                "name": upd_product.name,
                "price": upd_product.price,
                "stock": upd_product.stock,
            },
        };
        match self.product.update_one(filter, new_product, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(MyError::DbError),
        }
    }
    pub async fn delete_product(&self, id: String) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| MyError::DbError)?;
        match self.product.delete_one(doc! {"_id": oid}, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(MyError::DbError),
        }
    }
    pub async fn add_events(&self, events: Vec<Event>) -> Result<()> {
        let _inserted_events = self
            .event
            .insert_many(events, None)
            .await
            .map_err(|_| MyError::DbError)?;
        Ok(())
    }
    pub async fn get_all_events(&self) -> anyhow::Result<Vec<Event>> {
        let mut cursor = self.event.find(None, None).await?;
        let mut result: Vec<Event> = vec![];
        while let Some(event) = cursor.try_next().await? {
            result.push(event)
        }
        Ok(result)
    }
    pub async fn delete_event(&self, event: Event) -> anyhow::Result<()> {
        let oid = event.id;
        self.event.delete_one(doc! {"_id": oid}, None).await?;
        Ok(())
    }
    pub async fn sync_categories_and_attributes(&self, state: &AppState) -> anyhow::Result<()> {
        let ms = state.ms_client.get_categories().await?;
        println!("{} from MoySklad", &ms.len());
        let woo = state.woo_client.get_categories().await?;
        println!("{} from woo", &woo.len());
        let categories = CategoryDB::from_sources(ms, woo);
        println!("{} converted", &categories.len());
        // let _ = self.category.insert_many(categories, None).await;
        let mut ins = 0;
        for category in &categories {
            match self.category.insert_one(category, None).await {
                Ok(_) => ins += 1,
                Err(_) => continue,
            }
        }
        println!("{ins} inserted");
        let ms_attrs = state.ms_client.get_attributes().await?;
        let woo_attrs = state.woo_client.get_attributes().await?;
        let mut attributes = AttributeDB::from_sources(ms_attrs, woo_attrs);
        attributes.push(AttributeDB {
            id: None,
            ms_id: "no id".to_string(),
            woo_id: 87,
            name: "Страна".to_string(),
        });
        for attribute in &attributes {
            let _ = self.attribute.insert_one(attribute, None).await;
        }
        Ok(())
    }
    pub async fn category_id(&self, name: String) -> Option<i64> {
        match self.category.find_one(doc! {"name": name}, None).await {
            Ok(Some(category)) => Some(category.woo_id),
            _ => None,
        }
    }
    pub async fn attribute_id(&self, name: &String) -> Option<i64> {
        match self.attribute.find_one(doc! {"name": name}, None).await {
            Ok(Some(attribute)) => Some(attribute.woo_id),
            _ => None,
        }
    }
    pub async fn add_stock(&self, stock: Vec<Stock>) -> anyhow::Result<()> {
        let _result = self.stock.insert_many(stock, None).await?;
        Ok(())
    }
    pub async fn get_stock(&self) -> anyhow::Result<Vec<Stock>> {
        let mut cursor = self.stock.find(None, None).await?;
        let mut result = vec![];
        while let Some(stock) = cursor.try_next().await? {
            result.push(stock)
        }
        Ok(result)
    }
    pub async fn delete_stock(&self, stock: Stock) -> anyhow::Result<()> {
        let oid = stock.id;
        self.stock.delete_one(doc! {"_id": oid}, None).await?;
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stock {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sku: String,
    pub quantity: f64,
}
