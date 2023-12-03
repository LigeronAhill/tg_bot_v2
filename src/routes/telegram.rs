use anyhow::Result;

use crate::models::{moy_sklad::Action, AppState};
pub const WOO: &str = "https://safira.club//wp-json/wc/v3/products";
pub async fn clear_events(state: AppState) -> Result<()> {
    let events = state.storage.get_all_events().await?;
    for event in events {
        state.storage.delete_event(event).await?;
    }
    Ok(())
}
pub async fn sync_events(state: AppState) -> Result<String> {
    let _ = state
        .storage
        .sync_categories_and_attributes(state.clone())
        .await;
    let events = state.clone().storage.get_all_events().await?;
    let mut count = 0;
    for event in events {
        let Some(uri) = event.meta.href.to_owned() else {
            state.clone().storage.delete_event(event).await?;
            continue;
        };
        let product = state.ms_client.retrieve_product(&uri).await?;
        match event.action {
            Action::CREATE => state.woo_client.create_product(&state, product).await?,
            Action::UPDATE => state.woo_client.update_product(&state, product).await?,
            Action::DELETE => state.woo_client.delete_product(product).await?,
        }
        state.storage.delete_event(event).await?;
        count += 1;
    }
    let result = format!("{count} updated");
    Ok(result)
}
