use std::{collections::HashMap, io::Cursor};

use calamine::{open_workbook_from_rs, Reader, Xlsx};

use crate::{db::Stock, models::AppState};

pub async fn stock_update(state: &AppState, uri: &str, name: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let response: Vec<u8> = client.get(uri).send().await?.bytes().await?.to_vec();
    let cursor = Cursor::new(response);

    if name.contains("Carpetland") || name.contains("Склад КОНТРАКТ") {
        carpetland_pre_process(state, cursor.clone()).await?;
    }

    Ok(())
}
pub async fn stock_process(state: &AppState) -> anyhow::Result<()> {
    let stock = state.storage.get_stock().await?;

    for s in stock {
        let url = format!(
            "https://safira.club//wp-json/wc/v3/products?sku={}",
            s.sku.clone()
        );
        let response: Vec<serde_json::Value> = state
            .woo_client
            .client()
            .get(&url)
            .basic_auth(
                state.woo_client.client_key(),
                Some(state.woo_client.client_secret()),
            )
            .send()
            .await?
            .json()
            .await?;
        if response.is_empty() {
            state.storage.delete_stock(s).await?;
            continue;
        }
        let pr_type = response[0]["type"]
            .as_str()
            .ok_or(anyhow::Error::msg("error getting product type"))?;
        let id = response[0]["id"]
            .as_i64()
            .ok_or(anyhow::Error::msg("error getting product id"))?;
        match pr_type {
            "variation" => {
                let mut product_sku_vec = s.sku.split('_').collect::<Vec<&str>>();
                product_sku_vec.pop();
                let sku = product_sku_vec.join("_");
                let url = format!(
                    "https://safira.club//wp-json/wc/v3/products?sku={}",
                    sku.clone()
                );
                let val: Vec<serde_json::Value> = state
                    .woo_client
                    .client()
                    .get(&url)
                    .basic_auth(
                        state.woo_client.client_key(),
                        Some(state.woo_client.client_secret()),
                    )
                    .send()
                    .await?
                    .json()
                    .await?;
                if val.is_empty() {
                    state.storage.delete_stock(s).await?;
                    continue;
                }
                let product_id = response[0]["id"]
                    .as_i64()
                    .ok_or(anyhow::Error::msg("error getting product id"))?;
                let uri = format!(
                    "https://safira.club/wp-json/wc/v3/products/{}/variations/{}",
                    product_id, id
                );
                let stock_val = s.quantity as i64;
                let mut update_map = HashMap::new();
                update_map.insert("stock_quantity", stock_val);
                state
                    .woo_client
                    .client()
                    .put(&uri)
                    .basic_auth(
                        state.woo_client.client_key(),
                        Some(state.woo_client.client_secret()),
                    )
                    .json(&update_map)
                    .send()
                    .await?;
                state.storage.delete_stock(s).await?;
            }
            _ => {
                let uri = format!("https://safira.club/wp-json/wc/v3/products/{}", id);
                let stock_val = s.quantity as i64;
                let mut update_map = HashMap::new();
                update_map.insert("stock_quantity", stock_val);
                state
                    .woo_client
                    .client()
                    .put(&uri)
                    .basic_auth(
                        state.woo_client.client_key(),
                        Some(state.woo_client.client_secret()),
                    )
                    .json(&update_map)
                    .send()
                    .await?;
                state.storage.delete_stock(s).await?;
            }
        }
    }
    // for s in stock {
    //     if let Ok(id) = state.woo_client.get_woo_id(&s.sku).await {
    //         if let Ok(product) = state.woo_client.retrieve_product(id).await {
    //             let url = match product.product_type.unwrap() {
    //                 ProductType::Simple => {
    //                     format!("https://safira.club/wp-json/wc/v3/products/{}", id)
    //                 }
    //                 _ => {
    //                     let mut product_sku_vec = s.sku.split('_').collect::<Vec<&str>>();
    //                     product_sku_vec.pop();
    //                     let sku = product_sku_vec.join("_");
    //                     let product_id = state.woo_client.get_woo_id(&sku).await?;
    //                     format!(
    //                         "https://safira.club/wp-json/wc/v3/products/{}/variations/{}",
    //                         product_id, id
    //                     )
    //                 }
    //             };
    //             let stock_val = s.quantity as i64;
    //             let mut update_map = HashMap::new();
    //             update_map.insert("stock_quantity", stock_val);
    //             state
    //                 .woo_client
    //                 .client()
    //                 .put(&url)
    //                 .basic_auth(
    //                     state.woo_client.client_key(),
    //                     Some(state.woo_client.client_secret()),
    //                 )
    //                 .json(&update_map)
    //                 .send()
    //                 .await?;
    //             state.storage.delete_stock(s).await?;
    //         } else {
    //             println!("error getting product in processing --{}--", s.sku);
    //             state.storage.delete_stock(s).await?;
    //         }
    //     } else {
    //         println!("error getting id in processing --{}--", s.sku);
    //         state.storage.delete_stock(s).await?;
    //     }
    // }

    Ok(())
}
fn capitalize_first(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
async fn carpetland_pre_process(state: &AppState, cursor: Cursor<Vec<u8>>) -> anyhow::Result<()> {
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    let sheets = workbook.sheet_names().to_owned();
    let range = workbook.worksheet_range(&sheets[0]).unwrap()?;
    let mut stock_map = HashMap::new();
    let mut result = vec![];
    for (i, r) in range.rows().enumerate() {
        if i == 0 {
            continue;
        } else {
            let Some(brand) = r[0].get_string() else {
                continue;
            };
            let Some(collection) = r[1].get_string() else {
                continue;
            };
            let Some(article_str) = r[2].get_string() else {
                continue;
            };
            let Some(w_str) = r[3].get_string() else {
                continue;
            };
            let Some(stock) = r[5].get_float() else {
                continue;
            };
            let brand = brand.trim().to_lowercase();
            let brand = capitalize_first(brand);
            let collection = if brand == "Vorwerk" {
                let coll_vec = collection.split('/').collect::<Vec<&str>>();
                coll_vec[0].trim().to_string()
            } else if brand == "Condor" || brand == "Halbmond" {
                collection.split(' ').collect::<Vec<&str>>().join("-")
            } else if brand == "Innova" {
                let coll_vec = collection.split(' ').collect::<Vec<&str>>();
                coll_vec[0].trim().to_string()
            } else {
                collection.to_string()
            };
            let article = if brand == "Vorwerk" || brand == "Condor" || brand == "Halbmond" {
                let article_vec = article_str.split(' ').collect::<Vec<&str>>();
                article_vec[0].to_string()
            } else if brand == "Innova" {
                if collection == "Аврора" || collection == "Галактика" {
                    let article_str = article_str.trim().to_string();
                    let article_vec = article_str.split(' ').collect::<Vec<&str>>();
                    article_vec[article_vec.len() - 1].to_string()
                } else {
                    let article_vec = article_str.split(' ').collect::<Vec<&str>>();
                    article_vec[0].to_string()
                }
            } else {
                article_str.to_string()
            };
            let sku = format!("{brand}_{collection}_{article}_{w_str}");
            let stock = match stock_map.get(&sku) {
                Some(val) => stock + val,
                None => stock,
            };
            stock_map.insert(sku, stock);
        }
    }
    for (sku, stock) in stock_map {
        result.push(Stock {
            id: None,
            sku,
            quantity: stock,
        })
    }
    state.storage.add_stock(result).await?;
    Ok(())
}
