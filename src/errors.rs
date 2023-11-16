use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
#[derive(Debug, Clone)]
pub enum MyError {
    DbError,
    ReqwestError,
}
pub type Result<T> = core::result::Result<T, MyError>;

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::DbError => "something went wrong in DB",
            MyError::ReqwestError => "something went wrong in Reqwest",
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
