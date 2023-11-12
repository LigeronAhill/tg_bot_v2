use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::models;

pub async fn get_updates_webhook(
    State(_app_state): State<models::AppState>,
    Json(update): Json<models::tg::Update>,
) -> impl IntoResponse {
    println!("{:#?}", update);
    StatusCode::OK
}
