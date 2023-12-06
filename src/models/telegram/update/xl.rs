use std::io::Cursor;

use calamine::{open_workbook_from_rs, Reader, Xlsx};

use crate::models::AppState;

pub async fn stock_update(state: &AppState, uri: &str, _name: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let response: Vec<u8> = client.get(uri).send().await?.bytes().await?.to_vec();
    // let response: Vec<u8> = client.get(uri).send().await?.bytes().await?.to_vec();
    let cursor = Cursor::new(response);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    // let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    let sheets = workbook.sheet_names().to_owned();
    let mut text = String::from("failed");
    if let Some(Ok(range)) = workbook.worksheet_range(&sheets[0]) {
        for row in range.rows() {
            text.push_str(&format!("row = {:?}, row[0] = {:?}\n", row, row[0]));
        }
    }
    state.bot.send_message_admin(&sheets[0]).await?;
    Ok(())
}
