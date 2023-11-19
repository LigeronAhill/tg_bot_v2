use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
#[derive(Debug, Clone, serde::Serialize)]
pub enum MyError {
    Static(String),
    ProductBuildError,
    DbError,
    ReqwestError,
    MsgParseError,
}
pub type Result<T> = core::result::Result<T, MyError>;

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::DbError => "something went wrong in DB".to_string(),
            MyError::ReqwestError => "something went wrong in Reqwest".to_string(),
            MyError::Static(s) => s,
            MyError::ProductBuildError => "failed product build".to_string(),
            MyError::MsgParseError => "failed to parse message".to_string(),
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
