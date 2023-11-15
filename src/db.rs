use crate::models::Product;
use anyhow::Result;
use mongodb::{bson::doc, options::IndexOptions, Database, IndexModel};

pub const PRODUCT_COL: &str = "product";

pub async fn create_index_on_product_name(db: &Database) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"name": 1})
        .options(options)
        .build();
    db.collection::<Product>(PRODUCT_COL)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

pub async fn find_products(db: &Database) -> Result<Vec<Product>> {
    use futures::stream::TryStreamExt;
    let mut cursor = db
        .collection::<Product>(PRODUCT_COL)
        .find(None, None)
        .await?;
    let mut result: Vec<Product> = vec![];
    while let Some(product) = cursor.try_next().await? {
        result.push(product)
    }
    Ok(result)
}
