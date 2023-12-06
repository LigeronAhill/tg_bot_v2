use anyhow::Result;

use crate::models::{
    moy_sklad::{Action, EventType},
    AppState,
};
pub const WOO: &str = "https://safira.club//wp-json/wc/v3/products";
pub async fn clear_events(state: AppState) -> Result<()> {
    let events = state.storage.get_all_events().await?;
    for event in events {
        state.storage.delete_event(event).await?;
    }
    Ok(())
}
pub async fn sync_events(state: AppState) -> Result<String> {
    let events = state.clone().storage.get_all_events().await?;
    for event in events {
        let uri = event.meta.href.to_owned();
        match event.meta.event_type {
            EventType::Product => {
                let product = state.ms_client.retrieve_product(&uri).await?;
                if product.path_name.contains("Не для интернета")
                    || product.path_name.contains("Сопутствующие товары")
                    || product.path_name.contains("Услуги")
                {
                    continue;
                } else {
                    match event.action {
                        Action::CREATE => {
                            let _ = state.woo_client.create_product(&state, product).await?;
                        }
                        Action::UPDATE => {
                            let _ = state.woo_client.update_product(&state, product).await?;
                        }
                        Action::DELETE => {
                            let _ = state.woo_client.delete_product(product).await?;
                        }
                    }
                    state.storage.delete_event(event).await?;
                }
            }
            EventType::Productfolder => match event.action {
                Action::CREATE => {
                    let (_id, category_name, parent_id) =
                        state.ms_client.retrieve_category(&event.meta.href).await?;
                    let cat_id = state
                        .woo_client
                        .create_category(&category_name, parent_id)
                        .await?;
                    state.ms_client.update_external_code(&uri, cat_id).await?;
                    state.storage.delete_event(event).await?;
                }
                Action::UPDATE => {
                    let (id, name, parent_id) =
                        state.ms_client.retrieve_category(&event.meta.href).await?;
                    state
                        .woo_client
                        .update_category(id, &name, parent_id)
                        .await?;
                    state.storage.delete_event(event).await?;
                }
                Action::DELETE => {
                    state.storage.delete_event(event).await?;
                    continue;
                }
            },
            EventType::Variant => {
                state.storage.delete_event(event).await?;
                continue;
            }
            EventType::String(_) => {
                state.storage.delete_event(event).await?;
                continue;
            }
        }
    }
    Ok(String::from("Update succefull"))
}
