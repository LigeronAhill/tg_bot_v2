use serde::Deserialize;
use serde::Serialize;

use super::Meta;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFromMoySklad {
    /// ID учетной записи
    pub account_id: String,
    pub alcoholic: Option<Alcoholic>,
    pub archived: bool,
    pub article: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub barcodes: Option<Vec<BarCode>>,
    /// Закупочная цена
    pub buy_price: Option<Price>,
    /// Код товара
    pub code: Option<String>,
    pub country: Option<Meta>,
    pub description: Option<String>,
    pub discount_prohibited: bool,
    /// Реальный НДС %
    pub effective_vat: Option<i64>,
    pub effective_vat_enabled: Option<bool>,
    pub external_code: String,
    pub files: Option<Files>,
    /// Метаданные отдела сотрудника
    pub group: Group,
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
    pub supplier: Option<Meta>,
    pub sync_id: Option<String>,
    pub tax_system: Option<TaxSystem>,
    /// Серийные номера
    pub things: Option<Vec<String>>,
    pub tnved: Option<String>,
    pub tracking_type: Option<TrackingType>,
    pub uom: Option<Meta>,
    pub updated: String,
    pub use_parent_vat: bool,
    pub variants_count: i64,
    pub vat: Option<i64>,
    pub vat_enabled: Option<bool>,
    pub volume: Option<i64>,
    pub weight: Option<i64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
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
    pub value: AttributeValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    #[default]
    DefaultValue,
    Dictionary(Dictionary),
    String(String),
    Int(i64),
    Float(f64),
    Flag(bool),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dictionary {
    pub meta: Meta,
    pub name: String,
}
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
