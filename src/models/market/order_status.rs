use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{
    order::{PaymentType, TaxSystem},
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
    pub creation_date: Option<NaiveDateTime>,
    pub currence: Option<Currency>,
    pub fake: Option<bool>,
    pub id: Option<i64>,
    pub payment_type: Option<PaymentType>,
    pub payment_method: Option<PaymentMethod>,
    pub status: Option<Status>,
    pub substatus: Option<Substatus>,
    pub vehicle_number: Option<String>,
    pub tax_system: Option<TaxSystem>,
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
