#![recursion_limit = "256"]
use axum::routing::{delete, get, post, put};
use axum::Router;
use db::Storage;
use models::Tokens;
use mongodb::Database;
use shuttle_secrets::SecretStore;
use tg::Bot;

pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod tg;
use routes::*;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::MongoDb] db: Database,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let token = secret_store.get("TG_TOKEN").expect("token not set!");
    let my_tg_id = secret_store
        .get("MY_TG_ID")
        .expect("no tg id of my!")
        .parse::<i64>()
        .expect("cant parse id");
    let safira_group_tg_id = secret_store
        .get("GROUP_TG_ID")
        .expect("no group id!")
        .parse::<i64>()
        .expect("cant parse group id");
    let ms_token = secret_store.get("MS_TOKEN").expect("no ms token!");
    let storage = Storage::new(&db).await;
    let woo_token_1 = secret_store
        .get("WOO_TOKEN_1")
        .expect("no woocommerce token!");
    let woo_token_2 = secret_store
        .get("WOO_TOKEN_2")
        .expect("no woocommerce token!");
    let yandex_token = secret_store.get("YANDEX_TOKEN").expect("no yandex token!");
    let market_token = secret_store.get("market_token").expect("no market token");
    storage
        .name_index_create()
        .await
        .expect("can't create index!");
    let bot = Bot::new(token);
    let app_state = models::AppState {
        storage,
        bot,
        tokens: Tokens {
            my_tg_id,
            safira_group_tg_id,
            ms_token,
            woo_token_1,
            woo_token_2,
            yandex_token,
            market_token,
        },
    };
    let router = Router::new()
        .route("/health", get(health))
        .route("/api/v1/telegram", post(telegram))
        .route("/api/v1/mswebhook", post(ms_webhook))
        .route("/api/v1/woowebhook", post(woo_webhook))
        .route("/api/v1/ymwebhook", post(routes::ymarket::ymwebhook))
        .route("/api/v1/ymwebhook/cart", post(routes::ymarket::cart))
        .route(
            "/api/v1/ymwebhook/order/accept",
            post(routes::ymarket::order_accept),
        )
        // .route(
        //     "/api/v1/ymwebhook/order/status",
        //     post(routes::ymarket::order_status),
        // )
        // .route("/api/v1/ymwebhook/stocks", post(routes::ymarket::stocks))
        // .route(
        //     "/api/v1/ymwebhook/order/cacellation/notify",
        //     post(routes::ymarket::notify),
        // )
        .route("/api/v1/create", post(create_product))
        .route("/api/v1/read", get(get_products))
        .route("/api/v1/read/:id", get(get_product_by_id))
        .route("/api/v1/find/:name", get(get_product_by_name))
        .route("/api/v1/update/:id", put(update_product))
        .route("/api/v1/delete/:id", delete(delete_product))
        .with_state(app_state);

    Ok(router.into())
}
