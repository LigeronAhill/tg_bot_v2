use chrono::Local;
use serde::{Deserialize, Serialize};

use super::{Address, Currency, DeliveryType, Outlet, PaymentMethod, Region};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptResponse {
    pub order: AcceptOrder,
}
impl AcceptResponse {
    pub fn new() -> Self {
        let now = Local::now();
        let day = chrono::Days::new(5);
        let shipment_date = now
            .checked_add_days(day)
            .unwrap()
            .format("%d-%m-%Y")
            .to_string();
        Self {
            order: AcceptOrder {
                accepted: true,
                id: "696969".to_string(),
                shipment_date,
            },
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptOrder {
    pub accepted: bool,
    pub id: String,
    pub shipment_date: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclineResponse {
    pub order: DeclineOrder,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclineOrder {
    pub accepted: bool,
    pub reason: String,
}

// -------------------------------FROM MARKET-------------------------------------
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAccept {
    order: Order,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub business_id: Option<i64>,
    pub currency: Option<Currency>,
    pub fake: Option<bool>,
    pub id: Option<i64>,
    pub payment_type: Option<PaymentType>,
    pub payment_method: Option<PaymentMethod>,
    pub tax_system: Option<TaxSystem>,
    pub buyer_items_total_before_discount: Option<f64>,
    pub buyer_total_before_discount: Option<f64>,
    pub buyer_items_total: Option<f64>,
    pub buyer_total: Option<f64>,
    pub items_total: Option<f64>,
    pub total: Option<f64>,
    pub subsidy_total: Option<f64>,
    pub total_with_subsidy: Option<f64>,
    pub delivery_total: Option<f64>,
    pub delivery: Option<Delivery>,
    pub items: Option<Vec<Item>>,
    pub notes: Option<String>,
    pub buyer: Option<Buyer>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub feed_id: i64,
    pub offer_id: String,
    pub offer_name: String,
    pub feed_category_id: String,
    pub fulfilment_shop_id: Option<i64>,
    pub count: i32,
    pub price: Option<f64>,
    #[serde(rename = "buyer-price")]
    pub buyer_price: Option<f64>,
    pub subsidy: Option<f64>,
    pub buyer_price_before_discount: Option<f64>,
    pub price_before_discount: Option<f64>,
    pub vat: Option<String>,
    pub promos: Option<Vec<Promo>>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promo {
    pub market_promo_id: Option<String>,
    pub subsidy: Option<f64>,
    #[serde(rename = "type")]
    pub promo_type: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buyer {
    #[serde(rename = "type")]
    pub buyer_type: BuyerType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuyerType {
    #[default]
    PERSON,
    BUSINESS,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delivery {
    pub dispatch_type: Option<DispatchType>,
    pub delivery_partner_type: Option<DeliveryPartnerType>,
    pub delivery_service_id: Option<i32>,
    pub shop_delivery_id: Option<String>,
    pub price: Option<f64>,
    pub service_name: Option<String>,
    #[serde(rename = "type")]
    pub delivery_type: Option<DeliveryType>,
    pub lift_type: Option<LiftType>,
    pub lift_price: Option<f64>,
    pub vat: Option<String>,
    pub shipments: Option<Vec<Shipment>>,
    pub address: Option<Address>,
    pub dates: Option<Dates>,
    pub outlet: Option<Outlet>,
    pub subsidy: Option<f64>,
    pub region: Option<Region>,
    pub id: Option<String>,
    #[serde(rename = "region_id")]
    pub region_id: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dates {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub from_time: Option<String>,
    pub to_time: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    pub id: Option<i64>,
    pub boxes: Option<Vec<String>>,
    pub weight: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub depth: Option<i64>,
    pub status: Option<ShipmentStatus>,
    pub shipment_date: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipmentStatus {
    #[default]
    Created,
    Error,
    New,
    ReadyToShip,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LiftType {
    #[default]
    NotNeeded,
    Manual,
    Elevator,
    CargoElevator,
    Free,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchType {
    #[default]
    Buyer,
    MarketBrandedOutlet,
    ShopOutlet,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliveryPartnerType {
    #[default]
    SHOP,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    #[default]
    Prepaid,
    Postpaid,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxSystem {
    Echn,
    Envd,
    Osn,
    Psn,
    Usn,
    #[default]
    UsnMinusCost,
}
// ---------------------TEST STRUCTS--------------
// use serde::Deserialize;
// use serde::Serialize;
// use serde_json::Value;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Root {
//     pub order: Order,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Order {
//     pub business_id: i64,
//     pub currency: String,
//     pub fake: bool,
//     pub id: i64,
//     pub payment_type: String,
//     pub payment_method: String,
//     pub tax_system: String,
//     pub subsidy_total: i64,
//     pub buyer_items_total_before_discount: i64,
//     pub buyer_total_before_discount: i64,
//     pub buyer_items_total: i64,
//     pub buyer_total: i64,
//     pub items_total: i64,
//     pub total: i64,
//     pub total_with_subsidy: i64,
//     pub delivery_total: i64,
//     pub delivery: Delivery,
//     pub items: Vec<Item>,
//     pub notes: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Delivery {
//     pub price: i64,
//     #[serde(rename = "region_id")]
//     pub region_id: String,
//     pub service_name: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub dispatch_type: String,
//     pub lift_type: String,
//     pub lift_price: i64,
//     pub vat: String,
//     pub shipments: Vec<Shipment>,
//     pub address: Address,
//     pub dates: Dates,
//     pub subsidy: i64,
//     pub region: Region,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Shipment {
//     pub id: i64,
//     pub status: String,
//     pub depth: i64,
//     pub height: i64,
//     pub weight: i64,
//     pub width: i64,
//     pub boxes: Vec<Value>,
//     pub shipment_date: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Address {
//     pub country: String,
//     pub city: String,
//     pub subway: String,
//     pub street: String,
//     pub house: String,
//     pub floor: String,
//     pub lon: f64,
//     pub lat: f64,
//     pub notes: String,
//     pub outlet_phones: Vec<String>,
//     pub schedule: Vec<Schedule>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Schedule {
//     pub from_day: String,
//     pub to_day: String,
//     pub from_time: String,
//     pub to_time: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Dates {
//     pub from_date: String,
//     pub to_date: String,
//     pub from_time: String,
//     pub to_time: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Region {
//     pub id: i64,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub parent: Parent,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Parent {
//     pub id: i64,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub parent: Parent2,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Parent2 {
//     pub id: i64,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub parent: Parent3,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Parent3 {
//     pub id: i64,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Item {
//     pub count: i64,
//     pub feed_category_id: String,
//     pub fulfilment_shop_id: i64,
//     pub feed_id: i64,
//     pub offer_id: String,
//     pub offer_name: String,
//     pub price: i64,
//     #[serde(rename = "buyer-price")]
//     pub buyer_price: i64,
//     pub buyer_price_before_discount: i64,
//     pub price_before_discount: i64,
//     pub subsidy: i64,
//     pub vat: String,
//     #[serde(default)]
//     pub promos: Vec<Promo>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Promo {
//     pub market_promo_id: String,
//     pub subsidy: i64,
//     #[serde(rename = "type")]
//     pub type_field: String,
// }
