use std::io::Cursor;

use calamine::{open_workbook_from_rs, RangeDeserializerBuilder, Reader, Xlsx};
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
    let iter = RangeDeserializerBuilder::new().from_range::<_, RawCarpetland>(&range)?;
    let test = match name.contains("Carpetland") {
        true => "passed",
        false => "failed",
    };
    for row in iter {
        match row {
            Ok(row) => result.push(row),
            Err(_) => continue,
        }
    }
    let text = format!("file: {}\ntest: {}\n{:#?}", name, test, result[2]);
    state.bot.send_message_admin(&text).await?;
    Ok(())
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawCarpetland {
    pub brand: String,
    pub collection: String,
    pub article: String,
    pub width: String,
    pub free_m: String,
    pub free_sqm: String,
}
