use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::Meta;
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFromMoySklad {
    pub archived: bool,
    pub article: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    /// Закупочная цена
    pub buy_price: Option<Price>,
    /// Код товара
    pub code: Option<String>,
    pub country: Option<Country>,
    pub description: Option<String>,
    pub external_code: String,
    pub id: String,
    pub meta: Meta,
    pub name: String,
    /// Наименование группы, в которую входит Товар
    pub path_name: String,
    /// Метаданные группы Товара
    pub product_folder: Option<ProductFolder>,
    pub sale_prices: Option<Vec<SalePrice>>,
    pub supplier: Option<Supplier>,
    pub uom: Option<Uom>,
    pub updated: String,
    pub variants_count: i64,
    pub volume: Option<f64>,
    pub weight: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Supplier {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uom {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductFolder {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub meta: Meta,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub attribute_type: String,
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub value: f64,
    pub currency: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalePrice {
    pub value: f64,
    pub currency: Currency,
    pub price_type: PriceType,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceType {
    pub meta: Meta,
    pub id: String,
    pub name: String,
    pub external_code: String,
}
