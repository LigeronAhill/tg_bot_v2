use crate::models::{
    moy_sklad::{product::ProductFromMoySklad, Action},
    woocommerce::product::ProductFromWoo,
    AppState,
};
pub async fn sync_events(state: AppState) -> anyhow::Result<()> {
    let events = state.storage.get_all_events().await?;
    let client = reqwest::Client::builder().gzip(true).build()?;
    for event in events {
        if let Some(product_url) = event.meta.href.clone() {
            let product = client
                .get(&product_url)
                .bearer_auth(&state.tokens.ms_token)
                .send()
                .await?
                .json::<ProductFromMoySklad>()
                .await?;
            if product.path_name.contains("Не для интернета")
                || product.path_name.contains("Услуги")
                || product.path_name.contains("Сопутствующие товары")
            {
                state.storage.delete_event(event).await?
            } else {
                match event.action {
                    Action::CREATE => todo!(),
                    Action::UPDATE => {
                        let Some(sku) = product.article else {
                            return Err(anyhow::Error::msg("NO SKU!"));
                        };
                        let woo_url = "https://safira.club/wp-json/wc/v3/products";
                        let product_url = format!("{woo_url}/{}", product.external_code);
                        let woo_response = client
                            .get(product_url)
                            .basic_auth(&state.tokens.woo_token_1, Some(&state.tokens.woo_token_2))
                            .send()
                            .await?;
                        if woo_response.status() == axum::http::StatusCode::OK {
                            todo!()
                        } else {
                            let params = [("sku".to_string(), sku)];
                            let woo_response = client
                                .get(woo_url)
                                .query(&params)
                                .basic_auth(
                                    &state.tokens.woo_token_1,
                                    Some(&state.tokens.woo_token_2),
                                )
                                .send()
                                .await?
                                .json::<Vec<ProductFromWoo>>()
                                .await?;
                            match woo_response.len() {
                                1 => {
                                    let product_from_woo = woo_response[0].clone();
                                    if let Some(cat_url) =
                                        &product.product_folder.unwrap().meta.href
                                    {
                                        let cat_id =
                                            &product_from_woo.categories.unwrap()[0].id.unwrap();
                                        let mut cat_upd = std::collections::HashMap::new();
                                        cat_upd.insert("externalCode", &cat_id);
                                        client
                                            .put(cat_url)
                                            .bearer_auth(&state.tokens.ms_token)
                                            .json(&cat_upd)
                                            .send()
                                            .await?;
                                        let mut upd = std::collections::HashMap::new();
                                        upd.insert("externalCode", &product_from_woo.id);
                                        if let Some(p_url) = product.meta.href {
                                            let response = client
                                                .put(&p_url)
                                                .bearer_auth(&state.tokens.ms_token)
                                                .json(&upd)
                                                .send()
                                                .await?;
                                            match response.status() {
                                                axum::http::StatusCode::OK => {
                                                    state.storage.delete_event(event).await?
                                                }
                                                _ => continue,
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    let error = format!("Can't find product {}", product.name);
                                    return Err(anyhow::Error::msg(error));
                                }
                            }
                        }
                    }
                    Action::DELETE => {
                        let woo_url = format!(
                            "https://safira.club/wp-json/wc/v3/products/{}",
                            product.external_code
                        );
                        let response = client
                            .delete(woo_url)
                            .basic_auth(
                                state.tokens.woo_token_1.clone(),
                                Some(state.tokens.woo_token_2.clone()),
                            )
                            .send()
                            .await?;
                        match response.status() {
                            axum::http::StatusCode::OK => state.storage.delete_event(event).await?,
                            _ => continue,
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
