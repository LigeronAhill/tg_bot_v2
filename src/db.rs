use crate::errors::{MyError, Result};
use crate::models::product::Product;
use futures::TryStreamExt;

use mongodb::{
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
    Collection, Database, IndexModel,
};
// use serde::{Deserialize, Serialize};

pub const PRODUCT_COL: &str = "product";
// pub const DIALIOG_COL: &str = "dialog";

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Dialog {
//     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//     pub id: Option<ObjectId>,
//     pub chat_id: i64,
//     pub chat_date_time: chrono::NaiveDateTime,
// }

#[derive(Clone, Debug)]
pub struct Storage {
    product: Collection<Product>,
    // dialog: Collection<Dialog>,
}
impl Storage {
    pub async fn new(db: &Database) -> Self {
        Self {
            product: db.collection::<Product>(PRODUCT_COL),
            // dialog: db.collection::<Dialog>(DIALIOG_COL),
        }
    }
    pub async fn name_index_create(&self) -> Result<()> {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(options)
            .build();
        self.product
            .create_index(model, None)
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
        // let result = self
        //     .find_product_by_id(inserted_product.inserted_id.to_string())
        //     .await
        //     .map_err(|_| MyError::DbError)?;
        // Ok(result)
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
    pub async fn find_all_ids(&self) -> Result<Vec<String>> {
        let mut cursor = self
            .product
            .find(None, None)
            .await
            .map_err(|_| MyError::DbError)?;
        let mut result: Vec<String> = vec![];
        while let Some(product) = cursor.try_next().await.map_err(|_| MyError::DbError)? {
            result.push(product.id.unwrap().to_string())
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
        // let oid = ObjectId::parse_str(id).map_err(|_| MyError::DbError)?;
        let mut result = self
            .product
            .find(doc! {"name":{"$regex": name, "$options": "i"}}, None)
            .await
            .map_err(|_| MyError::DbError)?;
        let mut result_vec: Vec<Product> = vec![];
        while let Some(product) = result.try_next().await.map_err(|_| MyError::DbError)? {
            result_vec.push(product)
        }
        Ok(result_vec)
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
}
