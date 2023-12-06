use crate::errors::{MyError, Result};
use crate::models::moy_sklad::{Audit, Event};
use crate::models::woocommerce::product::ProductFromWoo;
use crate::models::{product::Product, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;
pub mod sync;
pub mod telegram;
pub mod ymarket;
pub async fn health() -> StatusCode {
    StatusCode::OK
}
pub async fn sync_products(State(state): State<AppState>) -> Result<Json<String>> {
    let result = sync::sync_categories(&state)
        .await
        .map_err(|e| MyError::Static(e.to_string()))?;
    Ok(Json(result))
}
pub async fn telegram(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::telegram::update::Update>,
) -> Result<StatusCode> {
    match payload.process(&state).await {
        Ok(_) => println!("Ok"),
        Err(e) => println!("{e:?}"),
    }
    Ok(StatusCode::OK)
}

pub async fn ms_webhook(
    State(state): State<AppState>,
    Json(payload): Json<Audit>,
) -> Result<StatusCode> {
    state.storage.add_events(payload.events.clone()).await?;
    let text = format!("Received {} updates", payload.events.len());
    let _ = state.bot.send_message_admin(&text).await;
    Ok(StatusCode::OK)
}
pub async fn woo_product(
    State(state): State<AppState>,
    Json(payload): Json<ProductFromWoo>,
) -> Result<StatusCode> {
    let text = payload.name;
    let _ = state.bot.send_message_admin(&text).await;
    Ok(StatusCode::OK)
}

pub async fn woo_webhook(
    State(state): State<AppState>,
    payload: Option<Json<Value>>,
) -> Result<StatusCode> {
    if payload.is_some() {
        let Json(payload) = payload.unwrap();
        let x = payload["number"].clone();
        let mut text: String = match serde_json::to_string(&x) {
            Ok(string) => string,
            Err(_) => "Что-то непонятное пришло".to_string(),
        };
        text.push_str("\n\n\n из WooCommerce");
        let _ = state.bot.send_message_admin(&text).await;
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

pub async fn get_products(State(app_state): State<AppState>) -> Result<Json<Vec<Event>>> {
    let result = app_state
        .storage
        .get_all_events()
        .await
        .map_err(|_| MyError::DbError)?;
    // let result = app_state.storage.find_all_products().await?;
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
