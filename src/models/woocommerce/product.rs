use serde::{Deserialize, Serialize};

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
    pub stock_quantity: Option<i64>,
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    #[default]
    Simple,
    Grouped,
    External,
    Variable,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    Draft,
    Pending,
    Private,
    #[default]
    Publish,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    #[default]
    Visible,
    Catalog,
    Search,
    Hidden,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaxStatus {
    #[default]
    Taxable,
    Shipping,
    None,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StockStatus {
    #[default]
    InStock,
    OutOfStock,
    OnBackOrder,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackOrder {
    #[default]
    No,
    Notify,
    Yes,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadsProperties {
    pub id: Option<String>,
    pub name: Option<String>,
    pub file: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionsProperties {
    pub length: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoriesProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagsProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
}
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributesProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub position: Option<i64>,
    pub visible: Option<bool>,
    pub variation: Option<bool>,
    pub options: Option<Vec<String>>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultAttributesProperties {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub option: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataProperties {
    pub id: Option<i64>,
    pub key: Option<String>,
    pub value: Option<serde_json::Value>,
}
