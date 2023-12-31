use std::time::Duration;

use axum::routing::{delete, get, post, put};
use axum::Router;
use db::Storage;
use models::market::MarketClient;
use models::moy_sklad::MoySklad;
use models::telegram::update::xl::stock_process;
use models::woocommerce::Woo;
use mongodb::Database;
use routes::telegram::sync_events;
use shuttle_secrets::SecretStore;

pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod tg;
use routes::*;
use tokio::time::sleep;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::MongoDb] db: Database,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let token = secret_store.get("TG_TOKEN").expect("token not set!");
    let my_tg_id = secret_store.get("MY_TG_ID").expect("no tg id of my!");
    let safira_group_tg_id = secret_store.get("GROUP_TG_ID").expect("no group id!");
    let ms_token = secret_store.get("MS_TOKEN").expect("no ms token!");
    let woo_token_1 = secret_store
        .get("WOO_TOKEN_1")
        .expect("no woocommerce token!");
    let woo_token_2 = secret_store
        .get("WOO_TOKEN_2")
        .expect("no woocommerce token!");
    let yandex_token = secret_store.get("YANDEX_TOKEN").expect("no yandex token!");

    let market_token = secret_store.get("MARKET_TOKEN").expect("no market token");
    let storage = Storage::new(&db).await;
    let _ = storage.name_index_create().await;

    let bot = tg::Bot::new(&token, &my_tg_id, &safira_group_tg_id);
    let ms_client = MoySklad::new(ms_token).await;
    let woo_client = Woo::new(woo_token_1, woo_token_2).await;
    let market_client = MarketClient::new(&market_token, &yandex_token).await;
    let app_state = models::AppState::new(storage, bot, ms_client, woo_client, market_client);
    let state = app_state.clone();

    let router = Router::new()
        .route("/health", get(health))
        .route("/api/v1/telegram", post(telegram))
        .route("/api/v1/syncproducts", get(sync_products))
        .route("/api/v1/mswebhook", post(ms_webhook))
        .route("/api/v1/woowebhook", post(woo_webhook))
        .route("/api/v1/wooproduct", post(woo_product))
        .route("/api/v1/ymwebhook", post(routes::ymarket::ymwebhook))
        .route("/api/v1/ymwebhook/cart", post(routes::ymarket::cart))
        .route(
            "/api/v1/ymwebhook/order/accept",
            post(routes::ymarket::order_accept),
        )
        .route(
            "/api/v1/ymwebhook/order/status",
            post(routes::ymarket::order_status),
        )
        .route("/api/v1/ymwebhook/stocks", post(routes::ymarket::stocks))
        .route(
            "/api/v1/ymwebhook/order/cancellation/notify",
            post(routes::ymarket::order_cancelation_notify),
        )
        .route("/api/v1/create", post(create_product))
        .route("/api/v1/read", get(get_products))
        .route("/api/v1/sync", get(sync_products))
        .route("/api/v1/stock", get(get_stock))
        .route("/api/v1/read/:id", get(get_product_by_id))
        .route("/api/v1/find/:name", get(get_product_by_name))
        .route("/api/v1/update/:id", put(update_product))
        .route("/api/v1/delete/:id", delete(delete_product))
        .with_state(app_state.clone());

    tokio::spawn(async move {
        loop {
            let _ = sync_events(state.clone()).await;
            let _ = stock_process(&state).await;
            sleep(Duration::from_millis(60000)).await;
        }
    });
    tokio::spawn(async move {
        loop {
            let categories: Vec<i64> = vec![2226];
            let mut products = vec![];
            for category in categories {
                if let Ok(pr) = app_state.woo_client.products_by_category(category).await {
                    products.extend(pr)
                }
            }
            for p in &products {
                println!("{p:#?}");
            }
            let _ = app_state.market_client.update_mapping(products).await;
            sleep(Duration::from_secs(60)).await;
        }
    });

    Ok(router.into())
}
