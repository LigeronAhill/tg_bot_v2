use crate::models::moy_sklad::Audit;
use crate::models::woocommerce::WebhookOrder;
use crate::models::{product::Product, tg::Update, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use tracing::info;

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn telegram(
    State(state): State<AppState>,
    Json(payload): Json<Update>,
) -> impl IntoResponse {
    match payload.message {
        Some(msg) => {
            let words = msg.text.split_whitespace();
            let mut color: i32 = 0;
            let mut col = String::new();
            for word in words {
                match word.parse::<i32>() {
                    Ok(number) => color = number,
                    Err(_) => col = word.to_string(),
                }
            }
            let text = format!("Вы искали коллекцию {col} в цвете {color}?");
            match state.bot.send_message(msg.chat.id, text).await {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        None => StatusCode::OK,
    }
}
pub async fn ms_webhook(
    State(state): State<AppState>,
    // Json(payload): Json<Option<serde_json::Value>>,
    Json(payload): Json<Audit>,
) -> impl IntoResponse {
    let products = payload
        .take_product_from_moy_sklad(state.tokens.clone())
        .await;
    match products {
        Ok(products) => {
            for product in products {
                let text = format!("{product:#?}");
                state
                    .bot
                    .send_message(state.tokens.my_tg_id, text)
                    .await
                    .unwrap();
            }
        }
        Err(_) => {
            info!("Error getting products from MS");
        }
    }
    StatusCode::OK
}
pub async fn ymwebhook(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let _text: String = serde_json::from_value(payload).unwrap();
    StatusCode::OK
}
pub async fn woo_webhook(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let text = match serde_json::from_value::<WebhookOrder>(payload.clone()) {
        Ok(order) => format!("{order:#?}"),
        Err(_) => format!("Order ID: {}\nTotal: {}\n", payload["id"], payload["total"]),
    };
    state
        .bot
        .send_message(state.tokens.my_tg_id, text)
        .await
        .unwrap();
    StatusCode::OK
}
pub async fn create_product(
    State(app_state): State<AppState>,
    Json(payload): Json<Product>,
) -> crate::errors::Result<Json<Product>> {
    let result = app_state.storage.create_product(payload).await?;
    Ok(Json(result))
}

pub async fn get_products(
    State(app_state): State<AppState>,
) -> crate::errors::Result<Json<Vec<Product>>> {
    let result = app_state.storage.find_all_products().await?;
    Ok(Json(result))
}
pub async fn get_product_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> crate::errors::Result<Json<Product>> {
    let result = app_state.storage.find_product_by_id(id).await?;
    Ok(Json(result))
}

pub async fn update_product(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Json(upd_product): Json<Product>,
) -> crate::errors::Result<()> {
    app_state.storage.update_product(id, upd_product).await?;
    Ok(())
}
pub async fn delete_product(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> crate::errors::Result<()> {
    app_state.storage.delete_product(id).await?;
    Ok(())
}
pub async fn get_product_by_name(
    State(app_state): State<AppState>,
    Path(name): Path<String>,
) -> crate::errors::Result<Json<Vec<Product>>> {
    let result = app_state.storage.find_product_by_name(name).await?;
    Ok(Json(result))
}
