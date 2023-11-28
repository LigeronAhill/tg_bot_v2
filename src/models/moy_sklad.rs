use self::product::ProductFromMoySklad;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use super::woocommerce::product::ProductFromWoo;
use super::AppState;
pub mod product;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    pub audit_context: AuditContext,
    pub events: Vec<Event>,
}

impl Audit {
    pub async fn test_get_product(&self, app_state: AppState) -> Result<String> {
        let mut result: Vec<String> = vec![];
        for event in &self.events {
            let uri = event.meta.href.as_ref().unwrap();
            let client = reqwest::Client::builder().gzip(true).build()?;
            let response = client
                .get(uri)
                .bearer_auth(&app_state.tokens.ms_token)
                .send()
                .await?;
            let product = response.json::<ProductFromMoySklad>().await?;
            if !product.path_name.contains("Не для интернета")
                && !product.path_name.contains("Услуги")
                && !product.path_name.contains("Сопутствующие товары")
            {
                result.push(product.name)
            }
        }
        Ok(result.join("\n"))
    }
    pub async fn sync_products_foreign_codes(&self, app_state: AppState) -> Result<String> {
        let mut updated_products = vec![];
        let mut woo_products = vec![];
        let client = reqwest::Client::builder().gzip(true).build()?;
        for event in self.events.clone() {
            let uri = event.meta.href.as_ref().unwrap();

            let mut product = client
                .get(uri.clone())
                .bearer_auth(&app_state.tokens.ms_token.clone())
                .send()
                .await?
                .json::<ProductFromMoySklad>()
                .await?;

            if !product.path_name.contains("Не для интернета")
                && !product.path_name.contains("Услуги")
                && !product.path_name.contains("Сопутствующие товары")
                && product.article.is_some()
            {
                let woo_url = "https://safira.club/wp-json/wc/v3/products";
                let params = [("sku".to_string(), product.article.clone().unwrap())];
                let products_from_woo: Vec<ProductFromWoo> = client
                    .get(woo_url)
                    .query(&params)
                    .basic_auth(
                        app_state.tokens.woo_token_1.clone(),
                        Some(app_state.tokens.woo_token_2.clone()),
                    )
                    .send()
                    .await?
                    .json()
                    .await?;
                if products_from_woo.is_empty() {
                    // TODO: create product in woo!!!
                    continue;
                } else {
                    woo_products.push(products_from_woo[0].clone());
                    let f_id = format!("{}", products_from_woo[0].id);
                    product.external_code = f_id;
                    updated_products.push(product.clone());
                }
            }
        }
        let upd_uri = "https://api.moysklad.ru/api/remap/1.2/entity/product";
        let ms_updated_products: Vec<ProductFromMoySklad> = client
            .post(upd_uri)
            .bearer_auth(app_state.tokens.ms_token.clone())
            .json(&updated_products)
            .send()
            .await?
            .json()
            .await?;
        let result = format!(
            "From ms: {}; from woo: {}\nUpdated: {}",
            updated_products.len(),
            woo_products.len(),
            ms_updated_products.len(),
        );
        Ok(result)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditContext {
    pub meta: Meta,
    pub uid: String,
    pub moment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub href: Option<String>,
    pub metadata_href: Option<String>,
    pub media_type: Option<String>,
    pub uuid_href: Option<String>,
    pub download_href: Option<String>,
    pub size: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub meta: Meta,
    pub action: String,
    pub account_id: String,
}
