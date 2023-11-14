use crate::models::{
    tg::{MessageToBot, Update},
    AppState, Product,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde_json::{json, Value};

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn telegram(
    State(state): State<AppState>,
    Json(payload): Json<Update>,
) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let url = format!("https://api.telegram.org/bot{}/sendMessage", state.token);
    match payload.message {
        Some(msg) => {
            let ans = MessageToBot::new(
                msg.chat.id,
                format!("Я только что получила это: {}", msg.text),
            );
            client.post(url).json(&ans).send().await;
            StatusCode::OK
        }
        None => StatusCode::OK,
    }
}
pub async fn mswebhook(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let _text: String = serde_json::from_value(payload).unwrap();
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
