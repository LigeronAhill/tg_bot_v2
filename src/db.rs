use mongodb::{bson::doc, options::IndexOptions, Database, IndexModel};

use crate::models::Product;

pub async fn create_index_on_product_name(db: &Database) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"name": 1})
        .options(options)
        .build();
    db.collection::<Product>("product")
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}
