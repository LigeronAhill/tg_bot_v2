use crate::{db::Storage, tg::Bot};

pub mod market;
pub mod moy_sklad;
pub mod product;
// pub mod woocommerce;
#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub bot: Bot,
    pub tokens: Tokens,
}
#[derive(Clone)]
pub struct Tokens {
    pub my_tg_id: i64,
    pub safira_group_tg_id: i64,
    pub ms_token: String,
    pub woo_token_1: String,
    pub woo_token_2: String,
    pub yandex_token: String,
    pub market_token: String,
}
