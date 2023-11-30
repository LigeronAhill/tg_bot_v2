use crate::models::{
    moy_sklad::product::ProductFromMoySklad, woocommerce::product::ProductFromWoo, AppState,
};
pub async fn sync_categories(state: AppState) -> anyhow::Result<()> {
    let client = reqwest::Client::builder().gzip(true).build()?;
    let uri = "https://api.moysklad.ru/api/remap/1.2/entity/productfolder";
    let woo_uri = "https://safira.club/wp-json/wc/v3/";
    let categories = client
        .get(uri)
        .bearer_auth(&state.tokens.ms_token.clone())
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    for category in categories["rows"].as_array().unwrap() {
        let query = [("search", category["name"].as_str().unwrap())];
        let categories_from_woo = client
            .get(woo_uri)
            .query(&query)
            .basic_auth(
                state.tokens.woo_token_1.clone(),
                Some(state.tokens.woo_token_2.clone()),
            )
            .send()
            .await?
            .json::<Vec<serde_json::Value>>()
            .await?;
        if categories_from_woo.len() > 1 && categories_from_woo.is_empty() {
            let text = format!(
                "Category {} failed sync",
                category["name"].as_str().unwrap()
            );
            let _ = state.bot.send_message(state.tokens.my_tg_id, text).await;
        } else {
            let update = [(
                "externalCode",
                categories_from_woo[0]["id"].as_str().unwrap(),
            )];
            let category_url = category["meta"]["href"].as_str().unwrap();
            client
                .put(category_url)
                .bearer_auth(&state.tokens.ms_token.clone())
                .json(&update)
                .send()
                .await?;
        }
    }
    Ok(())
}

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
            .json::<ProductFromMoySklad>()
            .await?;

        if !product.path_name.contains("Не для интернета")
            && !product.path_name.contains("Услуги")
            && !product.path_name.contains("Сопутствующие товары")
            && product.article.is_some()
        {
            match event.action.as_str() {
                "CREATE" => {
                    let woo_url = "https://safira.club/wp-json/wc/v3/products";
                    let params = [("sku".to_string(), product.article.clone().unwrap())];
                    let products_from_woo: Vec<ProductFromWoo> = client
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
                        todo!()
                    } else {
                        let f_id = format!("{}", products_from_woo[0].id);
                        let category_id = format!(
                            "{}",
                            products_from_woo[0].clone().categories.unwrap()[0]
                                .id
                                .unwrap()
                        );
                        let cat_url = product.product_folder.unwrap().meta.href.unwrap();
                        let mut cat_upd = std::collections::HashMap::new();
                        cat_upd.insert("externalCode", category_id);
                        client
                            .put(cat_url)
                            .bearer_auth(state.tokens.ms_token.clone())
                            .json(&cat_upd)
                            .send()
                            .await?;
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
                "UPDATE" => {
                    let woo_url = "https://safira.club/wp-json/wc/v3/products";
                    let params = [("sku".to_string(), product.article.clone().unwrap())];
                    let products_from_woo: Vec<ProductFromWoo> = client
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

                    let f_id = format!("{}", products_from_woo[0].id);
                    let category_id = format!(
                        "{}",
                        products_from_woo[0].clone().categories.unwrap()[0]
                            .id
                            .unwrap()
                    );
                    let cat_url = product.product_folder.unwrap().meta.href.unwrap();
                    let mut cat_upd = std::collections::HashMap::new();
                    cat_upd.insert("externalCode", category_id);
                    client
                        .put(cat_url)
                        .bearer_auth(state.tokens.ms_token.clone())
                        .json(&cat_upd)
                        .send()
                        .await?;
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
                "DELETE" => {
                    let woo_url = format!(
                        "https://safira.club/wp-json/wc/v3/products/{}",
                        product.external_code
                    );
                    match client
                        .delete(woo_url)
                        .basic_auth(
                            state.tokens.woo_token_1.clone(),
                            Some(state.tokens.woo_token_2.clone()),
                        )
                        .send()
                        .await
                    {
                        Ok(_) => state.storage.delete_event(event).await?,
                        Err(_) => continue,
                    }
                }
                _ => continue,
            }
        } else {
            state.storage.delete_event(event).await?;
        }
    }
    Ok(())
}
