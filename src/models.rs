use bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

use crate::{db::Storage, tg::Bot};

use self::{
    market::MarketClient,
    moy_sklad::{MSAttributeDTO, MSCategoryDTO},
    woocommerce::{WOOAttributeDTO, WOOCategoryDTO},
};

pub mod market;
pub mod moy_sklad;
pub mod product;
pub mod telegram;
pub mod woocommerce;
#[derive(Clone)]
pub struct AppState {
    pub storage: Storage,
    pub bot: Bot,
    pub ms_client: moy_sklad::MoySklad,
    pub woo_client: woocommerce::Woo,
    pub market_client: MarketClient,
}
impl AppState {
    pub fn new(
        storage: Storage,
        bot: Bot,
        ms_client: moy_sklad::MoySklad,
        woo_client: woocommerce::Woo,
        market_client: MarketClient,
    ) -> Self {
        Self {
            storage,
            bot,
            ms_client,
            woo_client,
            market_client,
        }
    }
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
