use crate::errors::Result;
use crate::models::moy_sklad::Audit;
use crate::models::woocommerce::product::ProductFromWoo;
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
// fn check_token(headers: HeaderMap, token: String) -> bool {
//     match headers.get("Authorization") {
//         Some(v) => v.to_str().unwrap() == token,
//         None => false,
//     }
// }
pub async fn telegram(State(state): State<AppState>, Json(payload): Json<Value>) -> StatusCode {
    if payload["message"]["text"] == "/sync" {
        let events = state.storage.get_all_events().await.expect("db error");
        let client = reqwest::Client::builder().gzip(true).build().unwrap();
        for event in events {
            let uri = event.meta.href.as_ref().unwrap();
            let product = client
                .get(uri.clone())
                .bearer_auth(&state.tokens.ms_token.clone())
                .send()
                .await
                .expect("ms reqwest error")
                .json::<crate::models::moy_sklad::product::ProductFromMoySklad>()
                .await
                .expect("parse from ms error");

            if !product.path_name.contains("Не для интернета")
                && !product.path_name.contains("Услуги")
                && !product.path_name.contains("Сопутствующие товары")
                && product.article.is_some()
            {
                let woo_url = "https://safira.club/wp-json/wc/v3/products";
                let params = [("sku".to_string(), product.article.clone().unwrap())];
                let products_from_woo: Vec<ProductFromWoo> = client
                    .get(woo_url)
                    .query(&params)
                    .basic_auth(
                        state.tokens.woo_token_1.clone(),
                        Some(state.tokens.woo_token_2.clone()),
                    )
                    .send()
                    .await
                    .expect("woo reqwest error")
                    .json()
                    .await
                    .expect("parse from woo error");
                if products_from_woo.is_empty() {
                    // TODO: create product in woo!!!
                    continue;
                } else {
                    let f_id = format!("{}", products_from_woo[0].id);
                    let mut upd = std::collections::HashMap::new();
                    upd.insert("externalCode", f_id);
                    match client
                        .put(uri)
                        .bearer_auth(state.tokens.ms_token.clone())
                        .json(&upd)
                        .send()
                        .await
                    {
                        Ok(_) => state
                            .storage
                            .delete_event(event)
                            .await
                            .expect("delete from db error"),
                        Err(_) => continue,
                    }
                }
            }
        }
    } else {
        let mut text: String = match serde_json::to_string_pretty(&payload) {
            Ok(string) => string,
            Err(_) => "Что-то непонятное пришло".to_string(),
        };
        text.push_str("\n\n\n из телеграм");
        state
            .bot
            .send_message(state.tokens.my_tg_id, text)
            .await
            .expect("error sending tg message");
    }
    StatusCode::OK
}

pub async fn ms_webhook(
    State(state): State<AppState>,
    Json(payload): Json<Audit>,
) -> Result<StatusCode> {
    state.storage.add_events(payload.events.clone()).await?;
    let text = format!("Received {} updates", payload.events.len());
    state.bot.send_message(state.tokens.my_tg_id, text).await?;
    Ok(StatusCode::OK)
}
pub async fn woo_product(
    State(state): State<AppState>,
    Json(payload): Json<ProductFromWoo>,
) -> Result<StatusCode> {
    let text = payload.name;
    state.bot.send_message(state.tokens.my_tg_id, text).await?;
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
