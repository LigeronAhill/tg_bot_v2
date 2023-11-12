use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::models;

const URL: &str = "https://api.telegram.org/bot";

pub async fn get_updates_webhook(
    State(app_state): State<models::AppState>,
    Json(update): Json<models::tg::Update>,
) -> impl IntoResponse {
    println!("{:#?}", update);
    match update.message {
        Some(message) => {
            let ans = answer(message.chat.id, message.text).await;
            let client = reqwest::Client::new();
            let uri = format!("{}{}/{}", URL, app_state.token, "sendMessage");
            let _ = client.post(uri).json(&ans).send().await;
            StatusCode::OK
        }
        None => StatusCode::OK,
    }
}

pub async fn answer(chat_id: i64, msg: String) -> models::tg::MessageToBot {
    let answer = format!("Получила такое сообщение: {}", msg);
    models::tg::MessageToBot::new(chat_id, answer)
}
