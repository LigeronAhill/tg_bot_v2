use crate::{errors::Result, models::moy_sklad::Audit};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;

use crate::models::{product::Product, tg::Update, AppState};

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
pub async fn mswebhook(
    State(state): State<AppState>,
    // Json(payload): Json<Option<serde_json::Value>>,
    Json(payload): Json<Audit>,
) -> impl IntoResponse {
    let ms_token = state.tokens.ms_token.clone();
    let tg = state.tokens.my_tg_id;
    state
        .bot
        .send_message(tg, "update...".to_string())
        .await
        .unwrap();
    for event in payload.events {
        state
            .bot
            .send_message(tg, event.test_api(ms_token.to_owned()).await)
            .await
            .unwrap();
    }
    // if let Some(entity) = payload {
    //     let msg = serde_json::from_value::<Audit>(entity.clone());
    //     match msg {
    //         Ok(audit) => {
    //             state
    //                 .bot
    //                 .send_message(
    //                     state.tokens.my_tg_id,
    //                     String::from("Получила обновление..."),
    //                 )
    //                 .await
    //                 .ok();
    //             for event in audit.events.clone() {
    //                 state
    //                     .bot
    //                     .send_message(
    //                         state.tokens.my_tg_id,
    //                         event.test_api(state.tokens.ms_token.to_owned()).await,
    //                     )
    //                     .await
    //                     .unwrap();
    //             }
    // let text = format!("{:#?}", audit);
    // state
    //     .bot
    //     .send_message(state.tokens.my_tg_id, text)
    //     .await
    //     .ok();
    // }
    // Err(_) => {
    //     state
    //         .bot
    //         .send_message(
    //             state.tokens.my_tg_id,
    //             String::from("Странное обновление..."),
    //         )
    //         .await
    //         .ok();
    // state
    //     .bot
    //     .send_message(
    //         state.tokens.my_tg_id,
    //         serde_json::to_string_pretty(&entity).unwrap_or("i dont know".to_string()),
    //     )
    //     .await
    //     .ok();
    //         }
    //     }
    // };
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
