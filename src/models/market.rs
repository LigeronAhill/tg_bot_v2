use chrono::Days;

use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
pub mod catalog;
pub mod offer;
pub mod order;
pub mod order_status;
pub mod price;
pub mod shops;
pub mod stock;
const FROM_TIME: &str = "10:00";
const TO_TIME: &str = "21:00";
const OUTLET: &str = "1";
#[derive(Clone)]
pub struct MarketClient {
    token: String,
    check_token: String,
    client: reqwest::Client,
    campaign_id: i64,
    business_id: i64,
}
impl MarketClient {
    pub async fn new(token: &str, check_token: &str) -> Self {
        let client = reqwest::Client::new();
        let response: shops::CampaignsResponse = client
            .get("https://api.partner.market.yandex.ru/campaigns")
            .bearer_auth(token)
            .send()
            .await
            .expect("error getting campaigns")
            .json()
            .await
            .expect("error unmarshaling campaigns");
        let mut campaign_id: i64 = 0;
        let mut business_id: i64 = 0;
        for campaign in &response.campaigns {
            if campaign.placement_type.as_str() == "DBS" {
                campaign_id = campaign.id;
                business_id = campaign.business.id;
            }
        }
        Self {
            token: token.to_owned(),
            check_token: check_token.to_owned(),
            client,
            campaign_id,
            business_id,
        }
    }
    pub fn token(&self) -> String {
        self.token.clone()
    }
    pub fn check_token(&self) -> String {
        self.check_token.clone()
    }
    pub fn campaign_id(&self) -> i64 {
        self.campaign_id
    }
    pub fn business_id(&self) -> i64 {
        self.business_id
    }
    pub fn client(&self) -> reqwest::Client {
        self.client.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCartResponse {
    pub cart: ResponseCart,
}
impl MarketCartResponse {
    pub fn new(request: MarketCartRequest) -> Self {
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
        let req_items = match request.cart.items {
            Some(items) => items,
            None => vec![],
        };
        for item in req_items {
            let new_item = ResponseItem::from_request(item);
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
            service_name: String::from("Собственная служба"),
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
    /// sku
    pub offer_id: String,
    pub count: i32,
    pub delivery: bool,
    pub seller_inn: String,
}
impl ResponseItem {
    fn from_request(item: Item) -> Self {
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
    Credit,
    ExternalCertificate,
    B2bAccountPrepayment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliveryType {
    #[default]
    DELIVERY,
    PICKUP,
    DIGITAL,
    POST,
}
//  -------------------FROM MARKET-----------------------------------
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCartRequest {
    pub cart: Cart,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cart {
    pub business_id: Option<i64>,
    pub currency: Option<Currency>,
    pub delivery_currency: Option<Currency>,
    pub delivery: Option<Delivery>,
    pub items: Option<Vec<Item>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delivery {
    pub estimated: Option<bool>,
    pub region: Option<Region>,
    pub address: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<RegionType>,
    pub parent: Option<Box<Region>>,
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
    pub entrance: Option<String>,
    pub entryphone: Option<String>,
    pub apartment: Option<String>,
    pub recipient: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub notes: Option<String>,
    pub outlet_phones: Option<Vec<String>>,
    pub schedule: Option<Vec<Schedule>>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub from_day: Option<String>,
    pub to_day: Option<String>,
    pub from_time: Option<String>,
    pub to_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub feed_id: i64,
    /// sku
    pub offer_id: String,
    pub count: i32,
    pub offer_name: String,
    pub feed_category_id: String,
    pub fulfilment_shop_id: Option<i64>,
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
