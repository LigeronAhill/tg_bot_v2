use std::io::Cursor;

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
async fn carpetland_process(_state: &AppState, cursor: Cursor<Vec<u8>>) -> anyhow::Result<()> {
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    let sheets = workbook.sheet_names().to_owned();
    let range = workbook.worksheet_range(&sheets[0]).unwrap()?;
    let mut result = vec![];
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        } else {
            let brand = match row[0].get_string() {
                Some(brand) => brand.to_string(),
                None => continue,
            };
            let collection = match row[1].get_string() {
                Some(collection) => collection.to_string(),
                None => continue,
            };
            let article = match row[2].get_string() {
                Some(article) => article.to_string(),
                None => continue,
            };
            let width = match row[3].get_string() {
                Some(width) => match width.parse::<i32>() {
                    Ok(width) => width,
                    Err(_) => continue,
                },
                None => continue,
            };
            let free_m = match row[4].get_float() {
                Some(free_m) => free_m,
                None => continue,
            };
            let free_sqm = match row[5].get_float() {
                Some(free_sqm) => free_sqm,
                None => continue,
            };
            result.push(RawCarpetland {
                brand,
                collection,
                article,
                width,
                free_m,
                free_sqm,
            });
        }
    }
    Ok(())
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
