use bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::{db::Storage, tg::Bot};

use self::{
    moy_sklad::{MSAttributeDTO, MSCategoryDTO},
    woocommerce::{WOOAttributeDTO, WOOCategoryDTO},
};

pub mod market;
pub mod moy_sklad;
pub mod product;
pub mod woocommerce;
#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub bot: Bot,
    pub ms_client: moy_sklad::MoySklad,
    pub woo_client: woocommerce::Woo,
    pub tokens: Tokens,
}
impl AppState {
    pub async fn new(
        db: &Database,
        bot_token: String,
        ms_token: String,
        woo_token_1: String,
        woo_token_2: String,
        my_tg_id: i64,
        safira_group_tg_id: i64,
        yandex_token: String,
        market_token: String,
    ) -> Self {
        Self {
            storage: Storage::new(db).await,
            bot: Bot::new(bot_token),
            ms_client: moy_sklad::MoySklad::new(ms_token.clone()).await,
            woo_client: woocommerce::Woo::new(woo_token_1.clone(), woo_token_2.clone()).await,
            tokens: Tokens {
                my_tg_id,
                safira_group_tg_id,
                ms_token,
                woo_token_1,
                woo_token_2,
                yandex_token,
                market_token,
            },
        }
    }
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub ms_id: String,
    pub woo_id: i64,
    pub name: String,
}
impl CategoryDB {
    pub fn from_sources(ms: Vec<MSCategoryDTO>, woo: Vec<WOOCategoryDTO>) -> Vec<Self> {
        let mut result = vec![];
        for c1 in &ms {
            for c2 in &woo {
                if c2.name == c1.name {
                    result.push(Self {
                        id: None,
                        ms_id: c1.id.clone(),
                        woo_id: c2.id,
                        name: c1.name.clone(),
                    })
                }
            }
        }
        result
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttributeDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub ms_id: String,
    pub woo_id: i64,
    pub name: String,
}
impl AttributeDB {
    pub fn from_sources(ms: Vec<MSAttributeDTO>, woo: Vec<WOOAttributeDTO>) -> Vec<Self> {
        let mut result = vec![];
        for c1 in &ms {
            for c2 in &woo {
                if c2.name == c1.name {
                    result.push(Self {
                        id: None,
                        ms_id: c1.id.clone(),
                        woo_id: c2.id,
                        name: c1.name.clone(),
                    })
                }
            }
        }
        result
    }
}
