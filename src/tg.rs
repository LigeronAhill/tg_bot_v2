use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::models;

pub async fn get_updates_webhook(
    State(_app_state): State<models::AppState>,
    Json(updates): Json<Vec<models::tg::Update>>,
) -> impl IntoResponse {
    for update in updates {
        println!("{:#?}", update)
    }
    StatusCode::OK
}
