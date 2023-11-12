pub mod tg;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Product {
    pub name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub token: String,
}
