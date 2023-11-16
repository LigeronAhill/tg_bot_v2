use axum::routing::{delete, get, post, put};
use axum::Router;
use models::Bot;
use mongodb::Database;
use shuttle_secrets::SecretStore;

pub mod db;
pub mod models;
pub mod routes;
pub mod tg;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::MongoDb] db: Database,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    db::create_index_on_product_name(&db).await;
    let token = secret_store.get("TG_TOKEN").expect("token not set!");
    let bot = Bot::new(token);
    let app_state = models::AppState { db, bot };
    let router = Router::new()
        .route("/health", get(routes::health))
        .route("/api/v1/telegram", post(routes::telegram))
        .route("/api/v1/mswebhook", post(routes::mswebhook))
        .route("/api/v1/ymwebhook", post(routes::ymwebhook))
        .route("/api/v1/create", post(routes::create_product))
        .route("/api/v1/read", get(routes::get_products))
        .route("/api/v1/read/:id", get(routes::get_product_by_id))
        .route("/api/v1/update/:id", put(routes::update_product))
        .route("/api/v1/delete/:id", delete(routes::delete_product))
        .with_state(app_state);

    Ok(router.into())
}
