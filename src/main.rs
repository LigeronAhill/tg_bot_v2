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
        },
    };
    let router = Router::new()
        .route("/health", get(routes::health))
        .route("/api/v1/telegram", post(routes::telegram))
        .route("/api/v1/mswebhook", post(routes::mswebhook))
        .route("/api/v1/ymwebhook", post(routes::ymwebhook))
        .route("/api/v1/create", post(routes::create_product))
        .route("/api/v1/read", get(routes::get_products))
        .route("/api/v1/read/:id", get(routes::get_product_by_id))
        .route("/api/v1/find/:name", get(routes::get_product_by_name))
        .route("/api/v1/update/:id", put(routes::update_product))
        .route("/api/v1/delete/:id", delete(routes::delete_product))
        .with_state(app_state);

    Ok(router.into())
}
