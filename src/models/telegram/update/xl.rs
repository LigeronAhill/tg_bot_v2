use std::io::Cursor;

use calamine::{open_workbook_from_rs, Reader, Xlsx};
use serde::{Deserialize, Serialize};

use crate::models::AppState;

pub async fn stock_update(state: &AppState, uri: &str, _name: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let response: Vec<u8> = client.get(uri).send().await?.bytes().await?.to_vec();
    let cursor = Cursor::new(response);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    let sheets = workbook.sheet_names().to_owned();
    let range = workbook.worksheet_range(&sheets[0]).unwrap()?;
    let mut result = vec![];
    // let test = match name.contains("Carpetland") {
    //     true => "passed",
    //     false => "failed",
    // };
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        } else {
            let brand = match row[0].get_string() {
                Some(brand) => brand.to_string(),
                None => format!("row {i} - 1"),
            };
            let collection = match row[1].get_string() {
                Some(collection) => collection.to_string(),
                None => format!("row {i} - 2"),
            };
            let article = match row[2].get_string() {
                Some(article) => article.to_string(),
                None => format!("row {i} - 3"),
            };
            let width = match row[3].get_string() {
                Some(width) => match width.parse::<i32>() {
                    Ok(width) => width,
                    Err(_) => 69,
                },
                None => 1 + i as i32,
            };
            let free_m = match row[4].get_float() {
                Some(free_m) => free_m,
                None => 1.0 + i as f64,
            };
            let free_sqm = match row[5].get_float() {
                Some(free_sqm) => free_sqm,
                None => 1.0 + i as f64,
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
    let text = format!("{:#?}", result[0]);
    state.bot.send_message_admin(&text).await?;
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
