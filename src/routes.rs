use crate::{
    db::{self, find_products},
    models::{tg::Update, AppState, Product},
    tg,
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
            let ans = tg::message_unwrap(msg);
            let _ = client.post(url).json(&ans).send().await;
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
pub async fn ymwebhook(
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
    let collection = app_state.db.collection::<Product>(db::PRODUCT_COL);
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
pub async fn get_products(State(app_state): State<AppState>) -> impl IntoResponse {
    let result: Vec<Product> = vec![];
    match find_products(&app_state.db).await {
        Ok(products) => Json(products),
        Err(_) => Json(result),
    }
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
