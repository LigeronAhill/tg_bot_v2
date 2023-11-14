use axum::routing::{delete, get, post, put};
use axum::Router;
use mongodb::Database;
use shuttle_secrets::SecretStore;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use tg::{schema, State};

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
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    let app_state = models::AppState { db };
    let router = Router::new()
        .route("/health", get(routes::health))
        .route("/api/v1/create", post(routes::create_product))
        .route("/api/v1/read", get(routes::get_products))
        .route("/api/v1/read/:id", get(routes::get_product_by_id))
        .route("/api/v1/update/:id", put(routes::update_product))
        .route("/api/v1/delete/:id", delete(routes::delete_product))
        .with_state(app_state);

    Ok(router.into())
}
