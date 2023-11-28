use std::collections::HashMap;

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
        let mut result = String::new();
        for event in &self.events {
            let uri = event.meta.href.as_ref().unwrap();
            let client = reqwest::Client::builder().gzip(true).build()?;
            let response = client
                .get(uri.clone())
                .bearer_auth(&app_state.tokens.ms_token.clone())
                .send()
                .await?;
            let product = response.json::<ProductFromMoySklad>().await?;

            if !product.path_name.contains("Не для интернета")
                && !product.path_name.contains("Услуги")
                && !product.path_name.contains("Сопутствующие товары")
                && product.article.is_some()
            {
                let woo_url = "https://safira.club/wp-json/wc/v3/products";
                let params = [("sku".to_string(), product.article.unwrap())];
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
                    let f_id = format!("{}", products_from_woo[0].id);
                    let mut updated_external_code = HashMap::new();
                    updated_external_code.insert("externalCode", f_id);
                    let updated_product: ProductFromMoySklad = client
                        .put(uri)
                        .bearer_auth(app_state.tokens.ms_token.clone())
                        .json(&updated_external_code)
                        .send()
                        .await?
                        .json()
                        .await?;
                    result.push('\n');
                    result.push_str(&updated_product.external_code);
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(500));
        }
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
