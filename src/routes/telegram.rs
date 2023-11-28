use crate::models::AppState;

pub async fn sync_events(state: AppState) -> anyhow::Result<()> {
    let events = state.storage.get_all_events().await?;
    let client = reqwest::Client::builder().gzip(true).build()?;
    for event in events {
        let uri = event.meta.href.as_ref().unwrap();
        let product = client
            .get(uri.clone())
            .bearer_auth(&state.tokens.ms_token.clone())
            .send()
            .await?
            .json::<crate::models::moy_sklad::product::ProductFromMoySklad>()
            .await?;

        if !product.path_name.contains("Не для интернета")
            && !product.path_name.contains("Услуги")
            && !product.path_name.contains("Сопутствующие товары")
            && product.article.is_some()
        {
            let woo_url = "https://safira.club/wp-json/wc/v3/products";
            let params = [("sku".to_string(), product.article.clone().unwrap())];
            let products_from_woo: Vec<crate::models::woocommerce::product::ProductFromWoo> =
                client
                    .get(woo_url)
                    .query(&params)
                    .basic_auth(
                        state.tokens.woo_token_1.clone(),
                        Some(state.tokens.woo_token_2.clone()),
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
                let mut upd = std::collections::HashMap::new();
                upd.insert("externalCode", f_id);
                match client
                    .put(uri)
                    .bearer_auth(state.tokens.ms_token.clone())
                    .json(&upd)
                    .send()
                    .await
                {
                    Ok(_) => state.storage.delete_event(event).await?,
                    Err(_) => continue,
                }
            }
        }
    }
    Ok(())
}
