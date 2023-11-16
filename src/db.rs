use crate::errors::{MyError, Result};
use crate::models::product::Product;
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
        let result = self
            .product
            .find_product_by_id(inserted_product.inserted_id.to_string())
            .await
            .map_err(|_| MyError::DbError)?;
        Ok(result)
    }
    pub async fn find_all_products(&self) -> Result<Vec<Product>> {
        use futures::stream::TryStreamExt;
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
        let oid = ObjectId::parse_str(id).unwrap();
        match self.product.find_one(doc! {"_id": oid}, None).await {
            Ok(Some(product)) => Ok(product),
            _ => Err(MyError::DbError),
        }
    }
}
