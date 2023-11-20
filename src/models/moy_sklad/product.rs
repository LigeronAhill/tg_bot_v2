use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFromMoySklad {
    pub meta: ProductMeta,
    pub id: String,
    pub account_id: String,
    pub owner: Owner,
    pub shared: bool,
    pub group: Group,
    pub updated: String,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    pub external_code: String,
    pub archived: bool,
    pub path_name: String,
    pub product_folder: Option<ProductFolder>,
    pub effective_vat: Option<i64>,
    pub effective_vat_enabled: Option<bool>,
    pub vat: Option<i64>,
    pub vat_enabled: Option<bool>,
    pub use_parent_vat: bool,
    pub uom: Option<Uom>,
    pub images: Images,
    pub min_price: MinPrice,
    pub sale_prices: Vec<SalePrice>,
    pub buy_price: BuyPrice,
    pub barcodes: Vec<Barcode>,
    pub supplier: Option<Supplier>,
    pub attributes: Option<Vec<Attribute>>,
    pub payment_item_type: String,
    pub discount_prohibited: bool,
    pub country: Option<Country>,
    pub article: Option<String>,
    pub weight: f64,
    pub volume: f64,
    pub variants_count: i64,
    pub is_serial_trackable: bool,
    pub tracking_type: String,
    pub files: Files,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub product_type: String,
    pub media_type: String,
    pub uuid_href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub meta: OwnerMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub owner_type: String,
    pub media_type: String,
    pub uuid_href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub meta: GroupMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFolder {
    pub meta: ProductFolderMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductFolderMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub uuid_href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uom {
    pub meta: UomMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UomMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    pub meta: ImagesMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagesMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub size: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinPrice {
    pub value: f64,
    pub currency: Currency,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub meta: CurrencyMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub uuid_href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalePrice {
    pub value: f64,
    pub currency: Currency,
    pub price_type: PriceType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceType {
    pub meta: PriceTypeMeta,
    pub id: String,
    pub name: String,
    pub external_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTypeMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyPrice {
    pub value: f64,
    pub currency: Currency,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Barcode {
    pub ean13: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Supplier {
    pub meta: SupplierMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplierMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub uuid_href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub meta: AttributeMeta,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Value,
}
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum AttributeValue {
//     CustomAttributeValue(CustomAttributeValue),
//     #[default]
//     DefVal,
//     String(String),
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CustomAttributeValue {
//     pub meta: CustomAttributeValueMeta,
//     pub name: String,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomAttributeValueMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub uuid_href: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub meta: CountryMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub uuid_href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Files {
    pub meta: FilesMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub size: i64,
    pub limit: i64,
    pub offset: i64,
}
// ---- variants -----

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variants {
    pub context: Context,
    pub meta: VariantsMeta,
    pub rows: Vec<Variant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub employee: Employee,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub meta: EmployeeMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantsMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub size: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub meta: VariantMeta,
    pub id: String,
    pub account_id: String,
    pub updated: String,
    pub name: String,
    pub code: String,
    pub external_code: String,
    pub archived: bool,
    pub characteristics: Vec<Characteristic>,
    pub images: Images,
    pub sale_prices: Vec<SalePrice>,
    pub barcodes: Vec<Barcode>,
    pub discount_prohibited: bool,
    pub product: Product,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantMeta {
    pub href: String,
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
    pub uuid_href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    pub meta: CharacteristicMeta,
    pub id: String,
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub media_type: String,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Images {
//     pub meta: Meta5,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Meta5 {
//     pub href: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub media_type: String,
//     pub size: i64,
//     pub limit: i64,
//     pub offset: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SalePrice {
//     pub value: f64,
//     pub currency: Currency,
//     pub price_type: PriceType,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Currency {
//     pub meta: Meta6,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Meta6 {
//     pub href: String,
//     pub metadata_href: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub media_type: String,
//     pub uuid_href: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PriceType {
//     pub meta: Meta7,
//     pub id: String,
//     pub name: String,
//     pub external_code: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Meta7 {
//     pub href: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub media_type: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Barcode {
//     pub ean13: String,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub meta: ProductMeta,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ProductMeta {
//     pub href: String,
//     pub metadata_href: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub media_type: String,
//     pub uuid_href: String,
// }
