use std::io::Cursor;

use calamine::{open_workbook_from_rs, Reader, Xlsx};
use serde::{Deserialize, Serialize};

use crate::models::AppState;

pub async fn stock_update(state: &AppState, uri: &str, name: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let response: Vec<u8> = client.get(uri).send().await?.bytes().await?.to_vec();
    let cursor = Cursor::new(response);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    let sheets = workbook.sheet_names().to_owned();
    let range = workbook.worksheet_range(&sheets[0]).unwrap()?;
    let mut result = vec![];
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        }
        let Some(brand) = row[0].get_string() else {
            continue;
        };
        let Some(collection) = row[1].get_string() else {
            continue;
        };
        let Some(article) = row[2].get_string() else {
            continue;
        };
        let Some(width) = row[3].get_int() else {
            continue;
        };
        let Some(free_m) = row[4].get_float() else {
            continue;
        };
        let Some(free_sqm) = row[5].get_float() else {
            continue;
        };
        result.push(Carpetland {
            brand: brand.to_string(),
            collection: collection.to_string(),
            article: article.to_string(),
            width: width as i32,
            free_m,
            free_sqm,
        })
    }
    let test = match name.contains("Carpetland") {
        true => "passed",
        false => "failed",
    };
    let text = format!("file: {}\ntest: {}\n{:#?}", name, test, result[2]);
    state.bot.send_message_admin(&text).await?;
    Ok(())
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Carpetland {
    pub brand: String,
    pub collection: String,
    pub article: String,
    pub width: i32,
    pub free_m: f64,
    pub free_sqm: f64,
}
