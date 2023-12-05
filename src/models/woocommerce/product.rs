use serde::{Deserialize, Serialize};

use crate::models::{moy_sklad::product::ProductFromMoySklad, AppState};
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WooProductCreate {
    pub name: String,
    #[serde(rename = "type")]
    pub product_type: ProductType,
    pub status: ProductStatus,
    pub catalog_visibility: Visibility,
    pub short_description: Option<String>,
    pub sku: String,
    pub regular_price: String,
    pub sale_price: Option<String>,
    pub manage_stock: bool,
    pub backorders: BackOrder,
    pub weight: String,
    pub shipping_class: ShippingClass,
    pub categories: Vec<CategoryCreate>,
    pub attributes: Vec<AttributesProperties>,
    pub default_attributes: Vec<VariationProperties>,
    pub meta_data: Vec<MetaDataProperties>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryCreate {
    pub id: i64,
}
impl WooProductCreate {
    pub async fn from_ms(state: &AppState, product: ProductFromMoySklad) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder().gzip(true).build()?;
        let product_type = match product.variants_count {
            0 => ProductType::Simple,
            _ => ProductType::Variable,
        };
        let Some(sku) = product.article.clone() else {
            return Err(anyhow::Error::msg("NO SKU!!!"));
        };
        let Some(prices) = product.sale_prices.clone() else {
            return Err(anyhow::Error::msg("NO PRICES!!!"));
        };
        let mut regular_price = String::new();
        let mut sale_price_from_ms: Option<String> = None;
        for price in prices {
            if price.price_type.name.as_str() == "Цена продажи" {
                let Some(cur_url) = price.currency.meta.href else {
                    return Err(anyhow::Error::msg("no currency!"));
                };
                let ms_curr_response = client
                    .get(cur_url)
                    .bearer_auth(&state.ms_client.token())
                    .send()
                    .await?;
                let curr_val: serde_json::Value = ms_curr_response.json().await?;
                let rate = curr_val["rate"].as_f64().unwrap();
                regular_price = format!("{}", (rate * price.value) / 100.0);
            } else if price.price_type.name.as_str() == "Акция" && price.value > 0.0 {
                let Some(cur_url) = price.currency.meta.href else {
                    return Err(anyhow::Error::msg("no currency!"));
                };
                let ms_curr_response = client
                    .get(cur_url)
                    .bearer_auth(&state.ms_client.token())
                    .send()
                    .await?;
                let curr_val: serde_json::Value = ms_curr_response.json().await?;
                let rate = curr_val["rate"].as_f64().unwrap();
                sale_price_from_ms = Some(format!("{}", (rate * price.value) / 100.0));
            }
        }
        let shipping_class = match product.path_name.contains("Ковровая плитка") {
            true => ShippingClass::Small,
            false => ShippingClass::Large,
        };
        let category_path: Vec<&str> = product.path_name.split('/').collect();
        let parent_cat = category_path[0];
        let parent_id = state
            .storage
            .category_id(parent_cat.to_owned())
            .await
            .unwrap();
        let category_name = category_path[category_path.len() - 1];
        let mut categories = Vec::new();
        let id = match state.storage.category_id(category_name.to_string()).await {
            Some(id) => id,
            None => {
                state
                    .woo_client
                    .create_category(category_name, parent_id)
                    .await?
            }
        };
        categories.push(CategoryCreate { id });
        let mut attributes = match product.attributes.clone() {
            Some(attributes_from_ms) => {
                let mut attrs: Vec<AttributesProperties> = vec![];
                if let Some(cntr) = product.country.clone() {
                    let cntr_url = cntr.meta.href.unwrap();
                    let cntr_val: serde_json::Value = client
                        .get(cntr_url)
                        .bearer_auth(&state.ms_client.token())
                        .send()
                        .await?
                        .json()
                        .await?;
                    let opt = cntr_val["name"].as_str().unwrap().to_string();

                    let country = AttributesProperties {
                        id: Some(87),
                        name: Some(String::from("Страна")),
                        position: None,
                        visible: Some(true),
                        variation: Some(false),
                        options: Some(vec![opt]),
                    };
                    attrs.push(country);
                }
                for attribute_from_ms in attributes_from_ms {
                    let opt = match attribute_from_ms.attribute_type.as_str() {
                        "customentity" => attribute_from_ms.value["name"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                        _ => attribute_from_ms.value.as_str().unwrap().to_string(),
                    };
                    let id = state.storage.attribute_id(&attribute_from_ms.name).await;
                    let attr = AttributesProperties {
                        id,
                        name: Some(attribute_from_ms.name),
                        position: None,
                        visible: Some(true),
                        variation: Some(false),
                        options: Some(vec![opt]),
                    };
                    attrs.push(attr)
                }
                attrs
            }
            None => vec![],
        };
        let mut default_attributes = vec![];
        if product.variants_count != 0 {
            let characteristics = state.ms_client.get_variants(state, &product).await?;
            let mut options = vec![];
            for char in &characteristics {
                options.push(char.option.clone())
            }
            let var_attr = AttributesProperties {
                id: Some(characteristics[0].id),
                name: Some(characteristics[0].name.clone()),
                position: None,
                visible: Some(true),
                variation: Some(true),
                options: Some(options),
            };
            attributes.push(var_attr);
            default_attributes.push(characteristics[0].clone());
        }
        // let pt = category_path[0];
        // let mdp = match pt {
        //     "Ковровая плитка" => {}
        //     _ => {}
        // };
        let catalog_visibility = match product.archived {
            true => Visibility::Hidden,
            false => Visibility::Visible,
        };
        let status = match product.archived {
            true => ProductStatus::Private,
            false => ProductStatus::Publish,
        };
        Ok(Self {
            name: product.name,
            product_type,
            status,
            catalog_visibility,
            short_description: product.description,
            sku,
            regular_price,
            sale_price: sale_price_from_ms,
            manage_stock: true,
            backorders: BackOrder::Yes,
            weight: product.weight.unwrap_or(0.0).to_string(),
            shipping_class,
            categories,
            attributes,
            default_attributes,
            meta_data: vec![],
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShippingClass {
    #[default]
    Large,
    Small,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductFromWoo {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub permalink: Option<String>,
    pub date_created: Option<String>,
    pub date_created_gmt: Option<String>,
    pub date_modified: Option<String>,
    pub date_modified_gmt: Option<String>,
    #[serde(rename = "type")]
    pub product_type: Option<ProductType>,
    pub status: Option<ProductStatus>,
    pub featured: Option<bool>,
    pub catalog_visibility: Option<Visibility>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub sku: Option<String>,
    pub price: Option<String>,
    pub regular_price: Option<String>,
    pub sale_price: Option<String>,
    pub date_on_sale_from: Option<String>,
    pub date_on_sale_from_gmt: Option<String>,
    pub date_on_sale_to: Option<String>,
    pub date_on_sale_to_gmt: Option<String>,
    pub price_html: Option<String>,
    pub on_sale: Option<bool>,
    pub purchasable: Option<bool>,
    pub total_sales: Option<i64>,
    #[serde(rename = "virtual")]
    pub product_virtual: Option<bool>,
    pub downloadable: Option<bool>,
    pub downloads: Option<Vec<DownloadsProperties>>,
    pub download_limit: Option<i64>,
    pub download_expiry: Option<i64>,
    pub external_url: Option<String>,
    pub button_text: Option<String>,
    pub tax_status: Option<TaxStatus>,
    pub tax_class: Option<String>,
    pub manage_stock: Option<bool>,
    pub stock_quantity: Option<f64>,
    pub stock_status: Option<StockStatus>,
    pub backorders: Option<BackOrder>,
    pub backorder_allowed: Option<bool>,
    pub backordered: Option<bool>,
    pub sold_individually: Option<bool>,
    pub weight: Option<String>,
    pub dimensions: Option<DimensionsProperties>,
    pub shipping_required: Option<bool>,
    pub shipping_taxable: Option<bool>,
    pub shipping_class: Option<String>,
    pub shipping_class_id: Option<i64>,
    pub reviews_allowed: Option<bool>,
    pub average_rating: Option<String>,
    pub rating_count: Option<i64>,
    pub related_ids: Option<Vec<i64>>,
    pub upsell_ids: Option<Vec<i64>>,
    pub cross_sell_ids: Option<Vec<i64>>,
    pub parent_id: Option<i64>,
    pub purchase_note: Option<String>,
    pub categories: Option<Vec<CategoriesProperties>>,
    pub tags: Option<Vec<TagsProperties>>,
    pub images: Option<Vec<ImagesProperties>>,
    pub attributes: Option<Vec<AttributesProperties>>,
    pub default_attributes: Option<Vec<DefaultAttributesProperties>>,
    pub variations: Option<Vec<i64>>,
    pub grouped_products: Option<Vec<i64>>,
    pub menu_order: Option<i64>,
    pub meta_data: Option<Vec<MetaDataProperties>>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    #[default]
    Simple,
    Grouped,
    External,
    Variable,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    Draft,
    Pending,
    Private,
    #[default]
    Publish,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    #[default]
    Visible,
    Catalog,
    Search,
    Hidden,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaxStatus {
    #[default]
    Taxable,
    Shipping,
    None,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StockStatus {
    #[default]
    InStock,
    OutOfStock,
    OnBackOrder,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackOrder {
    #[default]
    No,
    Notify,
    Yes,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadsProperties {
    pub id: Option<String>,
    pub name: Option<String>,
    pub file: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionsProperties {
    pub length: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoriesProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub parent: Option<i64>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagsProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagesProperties {
    pub id: Option<i64>,
    pub date_created: Option<String>,
    pub date_created_gmt: Option<String>,
    pub date_modified: Option<String>,
    pub date_modified_gmt: Option<String>,
    pub src: Option<String>,
    pub name: Option<String>,
    pub alt: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributesProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub position: Option<i64>,
    pub visible: Option<bool>,
    pub variation: Option<bool>,
    pub options: Option<Vec<String>>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariationProperties {
    pub id: i64,
    pub name: String,
    pub option: String,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultAttributesProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub option: Option<String>,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataProperties {
    pub id: Option<i64>,
    pub key: Option<String>,
    pub value: Option<serde_json::Value>,
}
