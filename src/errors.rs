use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
#[derive(Debug, Clone, serde::Serialize)]
pub enum MyError {
    Static(String),
    ProductBuildError,
    DbError,
    ReqwestError,
    MsgParseError,
    TokenError,
}
pub type Result<T> = core::result::Result<T, MyError>;

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self.clone() {
            MyError::DbError => "something went wrong in DB".to_string(),
            MyError::ReqwestError => "something went wrong in Reqwest".to_string(),
            MyError::Static(s) => s,
            MyError::ProductBuildError => "failed product build".to_string(),
            MyError::MsgParseError => "failed to parse message".to_string(),
            MyError::TokenError => "invalid token".to_string(),
        };
        let status = match self {
            MyError::Static(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::ProductBuildError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::ReqwestError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::MsgParseError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::TokenError => StatusCode::FORBIDDEN,
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (status, body).into_response()
    }
}
