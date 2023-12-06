use serde::{Deserialize, Serialize};

use crate::models::AppState;
pub async fn sync_categories(state: &AppState) -> anyhow::Result<String> {
    let url = "https://api.moysklad.ru/api/remap/1.2/entity/productfolder";

    let response: MSResponseCategories = state
        .ms_client
        .client()
        .get(url)
        .bearer_auth(state.ms_client.token())
        .send()
        .await?
        .json()
        .await?;

    for category in &response.rows {
        if let Some(woo_id) = state.storage.category_id(category.name.clone()).await {
            update_external_code(state, &category.meta.href, woo_id).await?;
        }
    }
    Ok(String::from("Done"))
}
pub async fn sync_products(state: &AppState) -> anyhow::Result<String> {
    let mut next_url = "https://api.moysklad.ru/api/remap/1.2/entity/product".to_string();

    loop {
        let response: MSResponse = state
            .ms_client
            .client()
            .get(&next_url)
            .bearer_auth(state.ms_client.token())
            .send()
            .await?
            .json()
            .await?;

        for product in &response.rows {
            if let Some(sku) = &product.article {
                if let Ok(woo_id) = state.woo_client.get_woo_id(sku).await {
                    update_external_code(state, &product.meta.href, woo_id).await?;
                }
            }
        }
        if let Some(next) = response.meta.next_href {
            next_url = next;
        } else {
            return Ok(String::from("Done"));
        }
    }
}
async fn update_external_code(state: &AppState, uri: &str, woo_id: i64) -> anyhow::Result<()> {
    let external_code = woo_id.to_string();
    let mut update = std::collections::HashMap::new();
    update.insert("externalCode", &external_code);
    state
        .ms_client
        .client()
        .put(uri)
        .bearer_auth(state.ms_client.token())
        .json(&update)
        .send()
        .await?;
    Ok(())
}

// -----------------------MoySklad------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MSResponse {
    pub meta: ResponseMeta,
    pub rows: Vec<ProductFromMs>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MSResponseCategories {
    pub meta: ResponseMeta,
    pub rows: Vec<CategoryFromMs>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMeta {
    pub size: i64,
    pub limit: i64,
    pub offset: i64,
    pub next_href: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryFromMs {
    pub meta: ProductMeta,
    pub id: String,
    pub name: String,
    pub external_code: String,
    pub article: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFromMs {
    pub meta: ProductMeta,
    pub id: String,
    pub external_code: String,
    pub article: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductMeta {
    pub href: String,
}
