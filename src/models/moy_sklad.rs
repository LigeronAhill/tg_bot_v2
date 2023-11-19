use crate::errors::{MyError, Result};
use crate::models::product::Product;
use crate::models::Tokens;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Audit {
    #[serde(rename = "auditContext")]
    pub audit_context: AuditContext,
    pub events: Vec<Event>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditContext {
    pub meta: Meta,
    pub uid: String,
    pub moment: String,
}

impl Audit {
    pub async fn take_product_from_moy_sklad(self, tokens: Tokens) -> Result<Vec<Product>> {
        let client = reqwest::Client::builder().gzip(true).build().unwrap();
        let mut result_slice: Vec<Product> = vec![];
        for event in self.events {
            let uri = event.meta.href;
            let response_body = client
                .get(uri)
                .bearer_auth(tokens.ms_token.clone())
                .send()
                .await
                .map_err(|_| MyError::ReqwestError)?;
            let result = response_body
                .json::<ProductFromMoySklad>()
                .await
                .map_err(|_| MyError::ReqwestError)?;
            let product = Product::from_ms(result)?;
            result_slice.push(product);
        }

        Ok(result_slice)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    #[serde(rename = "type")]
    pub audit_type: AuditType,
    pub href: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuditType {
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "variant")]
    Variant,
    #[serde(rename = "audit")]
    Audit,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub meta: Meta,
    pub action: Action,
    #[serde(rename = "accountId")]
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    CREATE,
    UPDATE,
    DELETE,
}

#[derive(Serialize, Deserialize)]
pub struct BarCodes {
    pub ean13: String,
}

#[derive(Serialize, Deserialize)]
pub struct PriceTypeMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub price_type: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct PriceType {
    pub meta: PriceTypeMeta,
    pub id: String,
    pub name: String,
    #[serde(rename = "externalCode")]
    pub external_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SalePrice {
    pub value: f64,
    pub currency: Data,
    #[serde(rename = "priceType")]
    pub price_type: PriceType,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub value: f64,
    pub currency: Data,
}

#[derive(Serialize, Deserialize)]
pub struct AttachmentsMeta {
    pub href: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Attachments {
    pub meta: AttachmentsMeta,
}

#[derive(Serialize, Deserialize)]
pub struct GroupMeta {
    pub href: String,
    #[serde(rename = "metadataHref")]
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub meta: GroupMeta,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub meta: ProductFromMoySkladMeta,
}

#[derive(Serialize, Deserialize)]
pub struct ProductFromMoySkladMeta {
    pub href: String,
    #[serde(rename = "metadataHref")]
    pub metadata_href: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "uuidHref")]
    pub uuid_href: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProductFromMoySklad {
    pub meta: ProductFromMoySkladMeta,
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub owner: Data,
    pub shared: bool,
    pub group: Group,
    pub updated: String,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    #[serde(rename = "externalCode")]
    pub external_code: String,
    pub archived: bool,
    #[serde(rename = "pathName")]
    pub path_name: String,
    #[serde(rename = "productFolder")]
    pub product_folder: Data,
    #[serde(rename = "effectiveVat")]
    pub effective_vat: i64,
    #[serde(rename = "effectiveVatEnabled")]
    pub effective_vat_enabled: bool,
    pub vat: i64,
    #[serde(rename = "vatEnabled")]
    pub vat_enabled: bool,
    #[serde(rename = "useParentVat")]
    pub use_parent_vat: bool,
    pub images: Option<Attachments>,
    #[serde(rename = "minPrice")]
    pub min_price: Option<Price>,
    #[serde(rename = "salePrices")]
    pub sale_prices: Vec<SalePrice>,
    #[serde(rename = "buyPrice")]
    pub buy_price: Option<Price>,
    pub barcodes: Vec<BarCodes>,
    pub supplier: Option<Data>,
    #[serde(rename = "paymentItemType")]
    pub payment_item_type: Option<String>,
    #[serde(rename = "discountProhibited")]
    pub discount_prohibited: bool,
    pub article: Option<String>,
    pub weight: f64,
    pub volume: f64,
    #[serde(rename = "variantsCount")]
    pub variants_count: i64,
    #[serde(rename = "isSerialTrackable")]
    pub is_serial_trackable: bool,
    #[serde(rename = "trackingType")]
    pub tracking_type: Option<String>,
    pub files: Option<Attachments>,
}
