use crate::models::{AppState, Product};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde_json::{json, Value};
use teloxide::requests::Requester;

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn mswebhook(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let text: String = serde_json::from_value(payload).unwrap();
    state
        .bot
        .send_message(state.test_id, text)
        .await
        .expect("cant send msg");
    StatusCode::OK
}
pub async fn create_product(
    State(app_state): State<AppState>,
    Json(payload): Json<Product>,
) -> impl IntoResponse {
    let collection = app_state.db.collection::<Product>("products");
    match collection.insert_one(payload, None).await {
        Ok(result) => {
            match collection
                .find_one(doc! {"_id": result.inserted_id}, None)
                .await
            {
                Ok(Some(product)) => Ok(Json(product)),
                _ => Err(Json(json!({"status": "error"}))),
            }
        }
        Err(_) => Err(Json(json!({"status": "error"}))),
    }
}
pub async fn get_products() -> impl IntoResponse {
    StatusCode::OK
}
pub async fn get_product_by_id() -> impl IntoResponse {
    StatusCode::OK
}
pub async fn update_product() -> impl IntoResponse {
    StatusCode::OK
}
pub async fn delete_product() -> impl IntoResponse {
    StatusCode::OK
}
