use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::{Database, IndexModel};
use serde::{Deserialize, Serialize};

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::MongoDb] db: Database) -> shuttle_axum::ShuttleAxum {
    create_index_on_product_name(&db).await;
    //    let state = AppState { database: db };
    let router = Router::new()
        .route("/health", get(health))
        //        .with_state(state.clone());
        .layer(Extension(db));

    Ok(router.into())
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Product {
    pub name: String,
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}
async fn create_index_on_product_name(db: &Database) {
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
