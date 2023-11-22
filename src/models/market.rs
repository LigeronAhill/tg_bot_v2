use crate::errors::MyError;
use crate::errors::Result;
use chrono::Days;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

const FROM_TIME: &str = "10:00";
const TO_TIME: &str = "21:00";
const OUTLET: &str = "1";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCartResponse {
    pub cart: ResponseCart,
}
impl MarketCartResponse {
    pub fn new(response: Value) -> Self {
        let days_start = Days::new(5);
        let days_end = Days::new(3);
        let day = Days::new(1);
        let from_date = Local::now().checked_add_days(days_start).unwrap();
        let to_date = from_date.checked_add_days(days_end).unwrap();
        // .format("%d-%m-%Y")
        // .to_string();
        let mut intervals: Vec<Interval> = vec![];
        let mut dates = vec![];
        let mut date = from_date;
        while date <= to_date {
            dates.push(date);
            date = date.checked_add_days(day).unwrap();
        }
        for date_for_interval in dates {
            let interval = Interval {
                date: date_for_interval.format("%d-%m-%Y").to_string(),
                from_time: FROM_TIME.to_string(),
                to_time: TO_TIME.to_string(),
            };
            intervals.push(interval)
        }
        let mut new_items: Vec<ResponseItem> = vec![];
        if let Some(items) = response["cart"]["items"].as_array() {
            items.iter().for_each(|item| {
                let new_item = ResponseItem::from_request(item).unwrap();
                new_items.push(new_item)
            });
        }
        let dates = Dates {
            from_date: from_date.format("%d-%m-%Y").to_string(),
            to_date: to_date.format("%d-%m-%Y").to_string(),
            intervals,
        };
        let mut outlets: Vec<Outlet> = vec![];
        let outlet = Outlet {
            code: OUTLET.to_string(),
        };
        outlets.push(outlet);
        let payment_methods: Vec<PaymentMethod> = vec![PaymentMethod::Yandex];
        let delivery_options = DeliveryOptions {
            id: String::from("1"),
            price: 2500.00,
            service_name: String::from("OWN DELIVERY"),
            delivery_type: DeliveryType::DELIVERY,
            dates,
            outlets,
            payment_methods: payment_methods.clone(),
        };
        Self {
            cart: ResponseCart {
                items: new_items,
                delivery_currency: Currency::RUR,
                delivery_options,
                payment_methods,
            },
        }
    }
    pub fn new_test_struct(response: MarketCartRequest) -> Self {
        let days_start = Days::new(5);
        let days_end = Days::new(3);
        let day = Days::new(1);
        let from_date = Local::now().checked_add_days(days_start).unwrap();
        let to_date = from_date.checked_add_days(days_end).unwrap();
        // .format("%d-%m-%Y")
        // .to_string();
        let mut intervals: Vec<Interval> = vec![];
        let mut dates = vec![];
        let mut date = from_date;
        while date <= to_date {
            dates.push(date);
            date = date.checked_add_days(day).unwrap();
        }
        for date_for_interval in dates {
            let interval = Interval {
                date: date_for_interval.format("%d-%m-%Y").to_string(),
                from_time: FROM_TIME.to_string(),
                to_time: TO_TIME.to_string(),
            };
            intervals.push(interval)
        }
        let mut new_items: Vec<ResponseItem> = vec![];
        for item in response.cart.items {
            let new_item = ResponseItem::from_request_test(item);
            new_items.push(new_item)
        }
        let dates = Dates {
            from_date: from_date.format("%d-%m-%Y").to_string(),
            to_date: to_date.format("%d-%m-%Y").to_string(),
            intervals,
        };
        let mut outlets: Vec<Outlet> = vec![];
        let outlet = Outlet {
            code: OUTLET.to_string(),
        };
        outlets.push(outlet);
        let payment_methods: Vec<PaymentMethod> = vec![PaymentMethod::Yandex];
        let delivery_options = DeliveryOptions {
            id: String::from("1"),
            price: 2500.00,
            service_name: String::from("OWN DELIVERY"),
            delivery_type: DeliveryType::DELIVERY,
            dates,
            outlets,
            payment_methods: payment_methods.clone(),
        };
        Self {
            cart: ResponseCart {
                items: new_items,
                delivery_currency: Currency::RUR,
                delivery_options,
                payment_methods,
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseCart {
    pub items: Vec<ResponseItem>,
    pub delivery_currency: Currency,
    pub delivery_options: DeliveryOptions,
    pub payment_methods: Vec<PaymentMethod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseItem {
    pub feed_id: i64,
    pub offer_id: String,
    pub count: i32,
    pub delivery: bool,
    pub seller_inn: String,
}
impl ResponseItem {
    pub fn from_request(item: &Value) -> Result<Self> {
        let feed_id = item
            .get("feedId")
            .ok_or(MyError::MsgParseError)?
            .to_string()
            .parse::<i64>()
            .map_err(|_| MyError::MsgParseError)?;
        let offer_id = item
            .get("offerId")
            .ok_or(MyError::MsgParseError)?
            .to_string();
        let count = item
            .get("count")
            .ok_or(MyError::MsgParseError)?
            .to_string()
            .parse::<i32>()
            .map_err(|_| MyError::MsgParseError)?;
        Ok(Self {
            feed_id,
            offer_id,
            count,
            delivery: true,
            seller_inn: String::from("772878900927"),
        })
    }
    fn from_request_test(item: Item) -> Self {
        Self {
            feed_id: item.feed_id,
            offer_id: item.offer_id,
            count: item.count,
            delivery: true,
            seller_inn: String::from("772878900927"),
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryOptions {
    pub id: String,
    pub price: f64,
    pub service_name: String,
    #[serde(rename = "type")]
    pub delivery_type: DeliveryType,
    pub dates: Dates,
    pub outlets: Vec<Outlet>,
    pub payment_methods: Vec<PaymentMethod>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outlet {
    pub code: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dates {
    pub from_date: String,
    pub to_date: String,
    pub intervals: Vec<Interval>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub date: String,
    pub from_time: String,
    pub to_time: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethod {
    ApplePay,
    CardOnDelivery,
    CashOnDelivery,
    GooglePay,
    Sbp,
    TinkoffCredit,
    TinkoffInstallments,
    #[default]
    Yandex,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliveryType {
    #[default]
    DELIVERY,
    PICKUP,
    DIGITAL,
}
// region start: -------------------FROM MARKET-----------------------------------
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCartRequest {
    pub cart: Cart,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cart {
    pub business_id: i64,
    pub currency: Currency,
    pub delivery_currency: Currency,
    pub delivery: Delivery,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delivery {
    pub estimated: bool,
    pub region: Region,
    pub address: Address,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: RegionType,
    pub parent: Box<Region>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country: Option<String>,
    pub city: Option<String>,
    pub subway: Option<String>,
    pub street: Option<String>,
    pub house: Option<String>,
    pub block: Option<String>,
    pub floor: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub feed_id: i64,
    pub offer_id: String,
    pub count: i32,
    pub offer_name: String,
    pub feed_category_id: String,
    pub fulfilment_shop_id: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Currency {
    #[default]
    RUR,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegionType {
    #[default]
    City,
    CityDistrict,
    Continent,
    Country,
    CountryDistrict,
    MetroStation,
    MonorailStation,
    OthersUniversal,
    OverseasTerritory,
    Region,
    SecondaryDistrict,
    Settlement,
    SubjectFederation,
    SubjectFederationDistrict,
    Suburb,
    Village,
}
