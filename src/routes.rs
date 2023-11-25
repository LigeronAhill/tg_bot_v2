use crate::errors::Result;
use crate::models::{product::Product, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;
pub mod ymarket;
pub async fn health() -> StatusCode {
    StatusCode::OK
}
pub async fn telegram(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let mut text: String = match serde_json::to_string_pretty(&payload) {
        Ok(string) => string,
        Err(_) => "Что-то непонятное пришло".to_string(),
    };
    text.push_str("\n\n\n из телеграм");
    state.bot.send_message(state.tokens.my_tg_id, text).await?;
    Ok(StatusCode::OK)
}

pub async fn ms_webhook(
    State(state): State<AppState>,
    // Json(payload): Json<Option<serde_json::Value>>,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let mut text: String = match serde_json::to_string_pretty(&payload) {
        Ok(string) => string,
        Err(_) => "Что-то непонятное пришло".to_string(),
    };
    text.push_str("\n\n\n из Мой Склад");
    state.bot.send_message(state.tokens.my_tg_id, text).await?;
    Ok(StatusCode::OK)
}

pub async fn woo_webhook(
    State(state): State<AppState>,
    Json(payload): Json<Option<Value>>,
) -> Result<StatusCode> {
    if payload.is_some() {
        let mut text: String = match serde_json::to_string_pretty(&payload) {
            Ok(string) => string,
            Err(_) => "Что-то непонятное пришло".to_string(),
        };
        text.push_str("\n\n\n из WooCommerce");
        state.bot.send_message(state.tokens.my_tg_id, text).await?;
    }

    Ok(StatusCode::OK)
}
pub async fn create_product(
    State(app_state): State<AppState>,
    Json(payload): Json<Product>,
) -> Result<Json<Product>> {
    let result = app_state.storage.create_product(payload).await?;
    Ok(Json(result))
}

pub async fn get_products(State(app_state): State<AppState>) -> Result<Json<Vec<Product>>> {
    let result = app_state.storage.find_all_products().await?;
    Ok(Json(result))
}
pub async fn get_product_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Product>> {
    let result = app_state.storage.find_product_by_id(id).await?;
    Ok(Json(result))
}

pub async fn update_product(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Json(upd_product): Json<Product>,
) -> Result<()> {
    app_state.storage.update_product(id, upd_product).await?;
    Ok(())
}
pub async fn delete_product(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<()> {
    app_state.storage.delete_product(id).await?;
    Ok(())
}
pub async fn get_product_by_name(
    State(app_state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<Product>>> {
    let result = app_state.storage.find_product_by_name(name).await?;
    Ok(Json(result))
}
