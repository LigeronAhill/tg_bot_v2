use crate::{
    errors::Result,
    models::{
        market::{
            order::{AcceptResponse, OrderAccept},
            order_status::OrderStatus,
            stock::{StocksRequest, StocksResponse},
            MarketCartRequest, MarketCartResponse,
        },
        AppState,
    },
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
const TEST_TOKEN: &str = "TOKENTOTEST20232311";
fn check_token(headers: HeaderMap, token: String) -> bool {
    match headers.get("Authorization") {
        Some(v) => v.to_str().unwrap() == token,
        None => false,
    }
}

pub async fn ymwebhook(State(state): State<AppState>, headers: HeaderMap) -> Result<StatusCode> {
    if !check_token(headers, state.tokens.yandex_token) {
        return Ok(StatusCode::FORBIDDEN);
    }
    Ok(StatusCode::OK)
}
pub async fn ymwebhook_for_test(headers: HeaderMap) -> Result<StatusCode> {
    if !check_token(headers, TEST_TOKEN.to_string()) {
        return Ok(StatusCode::FORBIDDEN);
    }
    Ok(StatusCode::OK)
}
pub async fn cart(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<MarketCartRequest>,
) -> Result<Json<MarketCartResponse>> {
    if !check_token(headers, state.tokens.yandex_token) {
        return Err(crate::errors::MyError::TokenError);
    }
    let resp = MarketCartResponse::new(payload);
    Ok(Json(resp))
}
pub async fn cart_for_test(
    headers: HeaderMap,
    Json(payload): Json<MarketCartRequest>,
) -> Result<Json<MarketCartResponse>> {
    if !check_token(headers, TEST_TOKEN.to_string()) {
        return Err(crate::errors::MyError::TokenError);
    }
    let resp = MarketCartResponse::new(payload);
    Ok(Json(resp))
}
pub async fn order_accept(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<OrderAccept>,
) -> Result<Json<AcceptResponse>> {
    if !check_token(headers, state.tokens.yandex_token) {
        return Err(crate::errors::MyError::TokenError);
    }
    let _ = payload;

    Ok(Json(AcceptResponse::new()))
}
pub async fn order_accept_for_test(
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode> {
    if !check_token(headers, TEST_TOKEN.to_string()) {
        return Ok(StatusCode::FORBIDDEN);
    }
    let _ = payload;
    Ok(StatusCode::OK)
}
pub async fn order_status(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<OrderStatus>,
) -> Result<StatusCode> {
    if !check_token(headers, state.tokens.yandex_token) {
        return Err(crate::errors::MyError::TokenError);
    }
    let _ = payload;

    Ok(StatusCode::OK)
}
pub async fn order_status_for_test(
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode> {
    if !check_token(headers, TEST_TOKEN.to_string()) {
        return Ok(StatusCode::FORBIDDEN);
    }
    let _ = payload;
    Ok(StatusCode::OK)
}
pub async fn order_cancelation_notify(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<OrderStatus>,
) -> Result<StatusCode> {
    if !check_token(headers, state.tokens.yandex_token) {
        return Err(crate::errors::MyError::TokenError);
    }
    let _ = payload;

    Ok(StatusCode::OK)
}
pub async fn order_cancelation_notify_for_test(
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode> {
    if !check_token(headers, TEST_TOKEN.to_string()) {
        return Ok(StatusCode::FORBIDDEN);
    }
    let _ = payload;
    Ok(StatusCode::OK)
}
pub async fn stocks(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<StocksRequest>,
) -> Result<Json<StocksResponse>> {
    if !check_token(headers, state.tokens.yandex_token) {
        return Err(crate::errors::MyError::TokenError);
    }
    let resp = StocksResponse::test(payload);
    Ok(Json(resp))
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
        let app = Router::new().route("/api/v1/ymwebhook", post(ymwebhook_for_test));
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
        let v = TEST_TOKEN.to_string();
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
        let v = TEST_TOKEN.to_string();
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
    #[tokio::test]
    async fn order_accept_test() {
        let test_response = json!({
            "order": {
                "businessId": 3675591,
                "currency": "RUR",
                "fake": false,
                "id": 12345,
                "paymentType": "PREPAID",
                "paymentMethod": "YANDEX",
                "taxSystem": "OSN",
                "subsidyTotal": 150,
                "buyerItemsTotalBeforeDiscount": 5800,
                "buyerTotalBeforeDiscount": 6150,
                "buyerItemsTotal": 5650,
                "buyerTotal": 6000,
                "itemsTotal": 5650,
                "total": 6000,
                "totalWithSubsidy": 6150,
                "deliveryTotal": 350,
                "items": [
                    {
                        "count": 3,
                        "feedCategoryId": "35",
                        "fulfilmentShopId": 1234567,
                        "feedId": 12345,
                        "offerId": "4609283881",
                        "offerName": "Чайник электрический 100 W",
                        "price": 1150,
                        "buyer-price": 1150,
                        "buyerPriceBeforeDiscount": 1200,
                        "priceBeforeDiscount": 1200,
                        "subsidy": 50,
                        "vat": "VAT_20",
                        "promos": [
                            {
                                "marketPromoId": "abc",
                                "subsidy": 50,
                                "type": "MARKET_DEAL"
                            }
                        ]
                    }                ],
                "notes": "Привезите побыстрее, пожалуйста!"
            }
        });

        let app = Router::new().route(
            "/api/v1/ymwebhook/order/accept",
            post(order_accept_for_test),
        );
        let k = "Authorization".to_string();
        let v = TEST_TOKEN.to_string();
        let response_without_headers = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/ymwebhook/order/accept")
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
                    .uri("/api/v1/ymwebhook/order/accept")
                    .header("Content-Type".to_string(), "application/json".to_string())
                    .header(k.clone(), v.clone())
                    .body(Body::from(serde_json::to_string(&test_response).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response_without_headers.status(), StatusCode::FORBIDDEN);
        assert_eq!(response_with_headers.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn order_status_test() {
        let test_response = json!({
          "order":
          {
            "businessId": 95291,
            "creationDate": "15-09-2020 00:42:42",
            "currency": "RUR",
            "fake": false,
            "id": 12345,
            "paymentType": "PREPAID",
            "paymentMethod": "YANDEX",
            "status": "PROCESSING",
            "taxSystem": "OSN",
            "subsidyTotal": 150,
            "buyerItemsTotalBeforeDiscount": 5800,
            "buyerTotalBeforeDiscount": 6150,
            "buyerItemsTotal": 5650,
            "buyerTotal": 6000,
            "itemsTotal": 5650,
            "total": 6000,
            "totalWithSubsidy": 6150,
            "deliveryTotal": 350,
            "buyer":
            {
              "id": "LEgMQuuxR8",
              "lastName": "Иванов",
              "firstName": "Иван",
              "middleName": "Иванович"
            },
            "delivery":
            {
              "price": 350,
              "serviceName": "СПСР",
              "type": "DELIVERY",
              "dispatchType": "BUYER",
              "vat": "VAT_10",
              "address":
              {
                "country": "Россия",
                "city": "Москва",
                "subway": "Проспект Вернадского",
                "street": "Ленинский проспект",
                "house": "90",
                "floor": "6"
              },
              "dates":
              {
                "fromDate": "15-09-2020",
                "toDate": "15-09-2020",
                "fromTime": "09:00",
                "toTime": "21:00"
              },

            },
            "items":
            [
               {
                "count": 1,
                "feedCategoryId": "41",
                "fulfilmentShopId": 1234567,
                "feedId": 12345,
                "offerId": "4607632101",
                "offerName": "Тостер",
                "price": 2200,
                "buyer-price": 2200,
                "buyerPriceBeforeDiscount": 2200,
                "priceBeforeDiscount": 1200,
                "subsidy": 0,
                "vat": "VAT_20"
              }
            ],
            "notes": "Не работает звонок. Пожалуйста, стучите."
          }
        }
        );

        let app = Router::new().route(
            "/api/v1/ymwebhook/order/status",
            post(order_status_for_test),
        );
        let k = "Authorization".to_string();
        let v = TEST_TOKEN.to_string();
        let response_without_headers = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/ymwebhook/order/status")
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
                    .uri("/api/v1/ymwebhook/order/status")
                    .header("Content-Type".to_string(), "application/json".to_string())
                    .header(k.clone(), v.clone())
                    .body(Body::from(serde_json::to_string(&test_response).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response_without_headers.status(), StatusCode::FORBIDDEN);
        assert_eq!(response_with_headers.status(), StatusCode::OK);
    }
}
