use crate::{db::Storage, tg::Bot};

pub mod market;
pub mod moy_sklad;
pub mod product;
pub mod tg;
pub mod wordpress;

#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub bot: Bot,
}
