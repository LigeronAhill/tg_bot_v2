use std::{collections::HashMap, io::Cursor};

use calamine::{open_workbook_from_rs, Reader, Xlsx};
use serde::{Deserialize, Serialize};

use crate::models::AppState;

pub async fn stock_update(state: &AppState, uri: &str, name: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let response: Vec<u8> = client.get(uri).send().await?.bytes().await?.to_vec();
    let cursor = Cursor::new(response);

    if name.contains("Carpetland") || name.contains("Склад КОНТРАКТ") {
        carpetland_process(state, cursor.clone()).await?;
    }

    Ok(())
}
async fn carpetland_process(state: &AppState, cursor: Cursor<Vec<u8>>) -> anyhow::Result<()> {
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    let sheets = workbook.sheet_names().to_owned();
    let range = workbook.worksheet_range(&sheets[0]).unwrap()?;
    let mut stock_map = HashMap::new();
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
    for (sku, _stock) in stock_map {
        let _id = state.woo_client.get_woo_id(&sku).await?;
    }

    Ok(())
}
fn capitalize_first(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawCarpetland {
    pub brand: String,
    pub collection: String,
    pub article: String,
    pub width: i32,
    pub free_m: f64,
    pub free_sqm: f64,
}
