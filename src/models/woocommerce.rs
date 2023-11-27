use serde::{Deserialize, Serialize};
pub mod product;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Href {
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: Vec<Href>,
    pub collection: Vec<Href>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderShippingLines {
    pub id: i64,
    pub method_title: String,
    pub method_id: String,
    pub instance_id: String,
    pub total: String,
    pub total_tax: String,
    pub taxes: Vec<TaxesProperty>,
    pub meta_data: Vec<LineItemsMetaData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxesProperty {
    pub id: i64,
    pub rate_code: String,
    pub rate_id: String,
    pub label: String,
    pub compound: bool,
    pub tax_total: String,
    pub shipping_tax_total: String,
    pub meta_data: Vec<OrderMetaData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub id: String,
    pub src: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineItemsMetaData {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub display_key: String,
    pub display_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderLineItems {
    pub id: i64,
    pub name: String,
    pub product_id: i64,
    pub variation_id: i64,
    pub quantity: i64,
    pub tax_class: String,
    pub subtotal: String,
    pub subtotal_tax: String,
    pub total: String,
    pub total_tax: String,
    pub taxes: Vec<TaxesProperty>,
    pub meta_data: Vec<LineItemsMetaData>,
    pub sku: String,
    pub price: i64,
    pub image: Image,
    pub parent_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderMetaData {
    pub id: i64,
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shipping {
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub address_1: String,
    pub address_2: String,
    pub city: String,
    pub state: String,
    pub postcode: String,
    pub country: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Billing {
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub address_1: String,
    pub address_2: String,
    pub city: String,
    pub state: String,
    pub postcode: String,
    pub country: String,
    pub email: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookOrder {
    pub id: i64,
    pub parent_id: i64,
    pub status: String,
    pub currency: String,
    pub version: String,
    pub prices_include_tax: bool,
    pub date_created: String,
    pub date_modified: String,
    pub discount_total: String,
    pub discount_tax: String,
    pub shipping_total: String,
    pub shipping_tax: String,
    pub cart_tax: String,
    pub total: String,
    pub total_tax: String,
    pub customer_id: i64,
    pub order_key: String,
    pub billing: Billing,
    pub shipping: Shipping,
    pub payment_method: String,
    pub payment_method_title: String,
    pub transaction_id: String,
    pub customer_ip_address: String,
    pub customer_user_agent: String,
    pub created_via: String,
    pub customer_note: String,
    pub date_completed: Option<String>,
    pub date_paid: Option<String>,
    pub cart_hash: String,
    pub number: String,
    pub meta_data: Vec<OrderMetaData>,
    pub line_items: Vec<OrderLineItems>,
    pub tax_lines: Vec<TaxLines>,
    pub shipping_lines: Vec<OrderShippingLines>,
    pub fee_lines: Vec<FeeLines>,
    pub coupon_lines: Vec<CouponLines>,
    pub refunds: Vec<Refund>,
    pub payment_url: String,
    pub is_editable: bool,
    pub needs_payment: bool,
    pub needs_processing: bool,
    pub date_created_gmt: String,
    pub date_modified_gmt: String,
    pub date_completed_gmt: Option<String>,
    pub date_paid_gmt: Option<String>,
    pub currency_symbol: String,
    #[serde(rename = "_links")]
    pub links: Links,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxLines {
    pub id: i64,
    pub rate_code: Option<String>,
    pub rate_id: Option<String>,
    pub label: Option<String>,
    pub compound: bool,
    pub tax_total: String,
    pub shipping_tax_total: Option<String>,
    pub meta_data: Vec<OrderMetaData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeLines {
    pub id: i64,
    pub name: String,
    pub tax_class: String,
    pub tax_status: Option<String>,
    pub total: String,
    pub total_tax: String,
    pub taxes: Vec<TaxesProperty>,
    pub meta_data: Vec<OrderMetaData>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CouponLines {
    pub id: i64,
    pub code: String,
    pub discount: String,
    pub discount_tax: String,
    pub meta_data: Vec<OrderMetaData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Refund {
    pub id: i64,
    pub reason: String,
    pub total: String,
}
