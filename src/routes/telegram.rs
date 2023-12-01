use anyhow::Result;

use crate::models::{
    moy_sklad::{product::ProductFromMoySklad, Action},
    woocommerce::product::WooProductCreateUpdate,
    AppState,
};
pub const WOO: &str = "https://safira.club//wp-json/wc/v3/products";
pub async fn clear_events(state: AppState) -> Result<()> {
    let events = state.storage.get_all_events().await?;
    for event in events {
        state.storage.delete_event(event).await?;
    }
    Ok(())
}
pub async fn sync_events(state: AppState) -> Result<String> {
    let events = state.clone().storage.get_all_events().await?;
    let mut count = 0;
    for event in events {
        let Some(uri) = event.meta.href else {
            state.clone().storage.delete_event(event).await?;
            continue;
        };
        match event.action {
            Action::CREATE => create_woo_product(state.clone(), uri).await?,
            Action::UPDATE => update_woo_product(state.clone(), uri).await?,
            Action::DELETE => delete_woo_product(state.clone(), uri).await?,
        }
        count += 1;
    }
    let result = format!("{count} updated");
    Ok(result)
}
pub async fn create_woo_product(state: AppState, uri: String) -> Result<()> {
    let client = reqwest::Client::builder().gzip(true).build()?;
    let ms_product: ProductFromMoySklad = client
        .get(&uri)
        .bearer_auth(&state.tokens.ms_token)
        .send()
        .await?
        .json()
        .await?;
    let prod = WooProductCreateUpdate::from_ms(state.clone(), ms_product.clone()).await?;
    client
        .post(WOO)
        .basic_auth(&state.tokens.woo_token_1, Some(&state.tokens.woo_token_2))
        .json(&prod)
        .send()
        .await?;
    Ok(())
}
pub async fn update_woo_product(state: AppState, uri: String) -> Result<()> {
    let client = reqwest::Client::builder().gzip(true).build()?;
    let ms_product: ProductFromMoySklad = client
        .get(&uri)
        .bearer_auth(&state.tokens.ms_token)
        .send()
        .await?
        .json()
        .await?;
    let prod = WooProductCreateUpdate::from_ms(state.clone(), ms_product.clone()).await?;
    let Some(sku) = ms_product.article else {
        return Err(anyhow::Error::msg("NO SKU!!!"));
    };
    let id = get_woo_id(state.clone(), sku.clone()).await?;
    let url = format!("{}/{}", WOO, id);
    client
        .put(url)
        .basic_auth(&state.tokens.woo_token_1, Some(&state.tokens.woo_token_2))
        .json(&prod)
        .send()
        .await?;
    Ok(())
}
pub async fn delete_woo_product(state: AppState, uri: String) -> Result<()> {
    let client = reqwest::Client::builder().gzip(true).build()?;
    let ms_response = client
        .get(uri)
        .bearer_auth(&state.tokens.ms_token)
        .send()
        .await?;
    let val = ms_response.json::<serde_json::Value>().await?;
    let sku_opt = val["article"].as_str();
    let Some(sku) = sku_opt else {
        return Err(anyhow::Error::msg("NO SKU!"));
    };
    let woo_id = get_woo_id(state.clone(), sku.to_string()).await?;
    let url = format!("{}/{}", WOO, woo_id);
    client
        .delete(url)
        .basic_auth(&state.tokens.woo_token_1, Some(&state.tokens.woo_token_2))
        .send()
        .await?;
    Ok(())
}
pub async fn get_woo_id(state: AppState, sku: String) -> Result<i64> {
    let client = reqwest::Client::builder().gzip(true).build()?;
    let response = client
        .get(WOO)
        .query(&[("sku", sku)])
        .basic_auth(state.tokens.woo_token_1, Some(state.tokens.woo_token_2))
        .send()
        .await?;
    let id = response.json::<Vec<serde_json::Value>>().await?[0]["id"]
        .as_i64()
        .unwrap();
    Ok(id)
}
