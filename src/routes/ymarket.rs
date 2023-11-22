use crate::{
    errors::Result,
    models::{
        market::{MarketCartRequest, MarketCartResponse},
        AppState,
    },
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

fn check_token(headers: HeaderMap) -> bool {
    let my_token = "2445D16D457F43A3AE3C2F484DDA31021F000001998BF23F";
    match headers.get("Authorization") {
        Some(v) => v.to_str().unwrap() == my_token,
        None => false,
    }
}

pub async fn ymwebhook(headers: HeaderMap) -> Result<StatusCode> {
    if !check_token(headers) {
        return Ok(StatusCode::FORBIDDEN);
    }
    Ok(StatusCode::OK)
}
pub async fn cart(
    headers: HeaderMap,
    State(_state): State<AppState>,
    Json(payload): Json<MarketCartRequest>,
) -> Result<Json<MarketCartResponse>> {
    if !check_token(headers) {
        return Err(crate::errors::MyError::TokenError);
    }
    let resp = MarketCartResponse::new(payload);
    Ok(Json(resp))
}
pub async fn cart_for_test(
    headers: HeaderMap,
    Json(payload): Json<MarketCartRequest>,
) -> Result<Json<MarketCartResponse>> {
    if !check_token(headers) {
        return Err(crate::errors::MyError::TokenError);
    }
    let resp = MarketCartResponse::new(payload);
    Ok(Json(resp))
}
pub async fn order_accept(headers: HeaderMap) -> Result<StatusCode> {
    if !check_token(headers) {
        return Ok(StatusCode::FORBIDDEN);
    }
    Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {

    use super::*;
    use axum::body::Body;
    use axum::http::{self, Request, StatusCode};
    use axum::{routing::post, Router};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn request_from_market() {
        let app = Router::new().route("/api/v1/ymwebhook", post(ymwebhook));
        let response_without_headers = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/ymwebhook")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let k = "Authorization".to_string();
        let v = "2445D16D457F43A3AE3C2F484DDA31021F000001998BF23F".to_string();
        let response_with_headers = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/ymwebhook")
                    .header(k, v)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response_without_headers.status(), StatusCode::FORBIDDEN);
        assert_eq!(response_with_headers.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn cart_test() {
        let test_response = json!({

            "cart":
            {
              "businessId": 8085591,
              "currency": "RUR",
              "deliveryCurrency": "RUR",
              "delivery":
              {
                "region":
                {
                  "id": 213,
                  "name": "Москва",
                  "type": "CITY",
                  "parent":
                  {
                    "id": 1,
                    "name": "Москва и Московская область",
                    "type": "SUBJECT_FEDERATION",
                    "parent":
                    {
                      "id": 3,
                      "name": "Центральный федеральный округ",
                      "type": "COUNTRY_DISTRICT",
                      "parent":
                      {
                        "id": 225,
                        "name": "Россия",
                        "type": "COUNTRY"
                      }
                    }
                  }
                },
                "address":
                {
                  "country": "Россия",
                  "city": "Москва",
                  "subway": "Проспект Вернадского",
                  "street": "Ленинский проспект",
                  "house": "90",
                  "floor": "6"
                }
              },
              "items":
              [
                {
                  "feedId": 12345,
                  "offerId": "4609283881",
                  "offerName": "Чайник электрический 100 W",
                  "feedCategoryId": "35",
                  "fulfilmentShopId": 1234567,
                  "count": 1
                },
                {
                  "feedId": 12346,
                  "offerId": "4607632101",
                  "offerName": "Тостер",
                  "feedCategoryId": "35",
                  "fulfilmentShopId": 1234567,
                  "count": 1
                }
              ]
            }
        });
        let app = Router::new().route("/api/v1/ymwebhook/cart", post(cart_for_test));
        let k = "Authorization".to_string();
        let v = "2445D16D457F43A3AE3C2F484DDA31021F000001998BF23F".to_string();
        let response_without_headers = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/ymwebhook/cart")
                    .header("Content-Type".to_string(), "application/json".to_string())
                    .body(Body::from(serde_json::to_string(&test_response).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let response_with_headers = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/ymwebhook/cart")
                    .header("Content-Type".to_string(), "application/json".to_string())
                    .header(k.clone(), v.clone())
                    .body(Body::from(
                        serde_json::to_string_pretty(&test_response).unwrap(),
                    ))
                    .unwrap(), // .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response_without_headers.status(), StatusCode::FORBIDDEN);
        assert_eq!(response_with_headers.status(), StatusCode::OK);
    }
}
