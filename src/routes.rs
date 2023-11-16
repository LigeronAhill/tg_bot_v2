use crate::{
    db::{self, find_product_by_id, find_products},
    models::{tg::Update, AppState, Product},
    tg,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::Bson;
use serde_json::{json, Value};

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn telegram(
    State(state): State<AppState>,
    Json(payload): Json<Update>,
) -> impl IntoResponse {
    match payload.message {
        Some(msg) => {
            let text = tg::message_unwrap(&msg);
            match state.bot.send_message(msg.chat.id, text).await {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
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
        Ok(result) => find_product_by_id(app_state, result.inserted_id.to_string()).await,
        Err(_) => Err(Json(json!({"status": "error creating product"}))),
    }
}
pub async fn get_products(State(app_state): State<AppState>) -> impl IntoResponse {
    let result: Vec<Product> = vec![];
    match find_products(&app_state.db).await {
        Ok(products) => Json(products),
        Err(_) => Json(result),
    }
}
pub async fn get_product_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    find_product_by_id(app_state, id).await
}

pub async fn update_product(
    State(_app_state): State<AppState>,
    Path(_id): Path<Bson>,
) -> impl IntoResponse {
    StatusCode::OK
}
pub async fn delete_product(
    State(_app_state): State<AppState>,
    Path(_id): Path<Bson>,
) -> impl IntoResponse {
    StatusCode::OK
}
