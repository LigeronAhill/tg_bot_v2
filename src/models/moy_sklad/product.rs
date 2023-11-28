use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::Meta;
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFromMoySklad {
    /// ID учетной записи
    pub account_id: Option<String>,
    pub alcoholic: Option<Alcoholic>,
    pub archived: Option<bool>,
    pub article: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub barcodes: Option<Vec<BarCode>>,
    /// Закупочная цена
    pub buy_price: Option<Price>,
    /// Код товара
    pub code: Option<String>,
    pub country: Option<Country>,
    pub description: Option<String>,
    pub discount_prohibited: Option<bool>,
    /// Реальный НДС %
    pub effective_vat: Option<i64>,
    pub effective_vat_enabled: Option<bool>,
    pub external_code: String,
    pub files: Option<Files>,
    /// Метаданные отдела сотрудника
    pub group: Option<Group>,
    pub id: String,
    pub images: Option<Images>,
    pub is_serial_trackable: Option<bool>,
    pub meta: Meta,
    pub min_price: Option<Price>,
    pub minimum_balance: Option<i64>,
    pub name: String,
    pub owner: Option<Owner>,
    /// Упаковки Товара
    pub packs: Option<Vec<Pack>>,
    /// Управление состоянием частичного выбытия маркированного товара. «true» - возможность включена.
    pub partial_disposal: Option<bool>,
    /// Наименование группы, в которую входит Товар
    pub path_name: String,
    pub payment_item_type: Option<PaymentItemType>,
    pub ppe_type: Option<i64>,
    /// Метаданные группы Товара
    pub product_folder: Option<ProductFolder>,
    pub sale_prices: Option<Vec<SalePrice>>,
    pub shared: bool,
    pub supplier: Option<Supplier>,
    pub sync_id: Option<String>,
    pub tax_system: Option<TaxSystem>,
    /// Серийные номера
    pub things: Option<Vec<String>>,
    pub tnved: Option<String>,
    pub tracking_type: Option<TrackingType>,
    pub uom: Option<Uom>,
    pub updated: String,
    pub use_parent_vat: bool,
    pub variants_count: i64,
    pub vat: Option<i64>,
    pub vat_enabled: Option<bool>,
    pub volume: Option<f64>,
    pub weight: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    pub meta: Meta,
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
pub struct Files {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Images {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub meta: Meta,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductFolder {
    pub meta: Meta,
}
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alcoholic {
    pub excize: Option<bool>,
    #[serde(rename = "type")]
    pub alchogole_type: Option<i64>,
    pub strength: Option<f64>,
    pub volume: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub meta: Meta,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Value,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarCode {
    pub ean13: Option<String>,
    pub ean8: Option<String>,
    pub code128: Option<String>,
    pub gtin: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub value: f64,
    pub currency: Meta,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pack {
    pub barcodes: Option<Vec<BarCode>>,
    pub id: Option<String>,
    pub quantity: Option<f64>,
    pub uom: Option<Meta>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentItemType {
    #[default]
    Good,
    ExcizableGood,
    CompoundPaymentItem,
    AnotherPaymentItem,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalePrice {
    pub value: f64,
    pub currency: Meta,
    pub price_type: PriceType,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceType {
    pub meta: Meta,
    pub id: String,
    pub name: String,
    pub external_code: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxSystem {
    #[default]
    SimplifiedTaxSystemIncomeOutcome,
    GeneralTaxSystem,
    PatentBased,
    PresumptiveTaxSystem,
    SimplifiedTaxSystemIncome,
    TaxSystemSameAsGroup,
    UnifiedAgriculturalTax,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrackingType {
    BeerAlcohol,
    Electronics,
    LpClothes,
    LpLinens,
    Milk,
    Ncp,
    #[default]
    NotTracked,
    Otp,
    Perfumery,
    Shoes,
    Tires,
    Tobacco,
    Water,
}
