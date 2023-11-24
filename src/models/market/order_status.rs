use serde::{Deserialize, Serialize};

use super::{
    order::{Buyer, Delivery, Item, PaymentType, TaxSystem},
    Currency, PaymentMethod,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    pub order: Order,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub business_id: Option<i64>,
    pub creation_date: Option<String>,
    pub currence: Option<Currency>,
    pub fake: Option<bool>,
    pub id: Option<i64>,
    pub payment_type: Option<PaymentType>,
    pub payment_method: Option<PaymentMethod>,
    pub status: Option<Status>,
    pub substatus: Option<Substatus>,
    pub vehicle_number: Option<String>,
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
    pub electronic_acceptance_certificate_code: Option<String>,
    pub notes: Option<String>,
    pub buyer: Option<Buyer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[default]
    CANCELLED,
    PROCESSING,
    RESERVED,
    UNPAID,
    OTHER(String),
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Substatus {
    #[default]
    PendingExpired,
    ProcessingExpired,
    ReservationExpired,
    ReservationFailed,
    ShopPendingCancelled,
    UserChangedMind,
    UserForgotToUseBonus,
    UserNotPaid,
    CourierNotFound,
    Other(String),
}
