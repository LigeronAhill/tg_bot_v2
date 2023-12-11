use serde::Deserialize;
use serde::Serialize;

use crate::models::woocommerce::product::ProductFromWoo;
use crate::models::AppState;

use super::MarketClient;
impl MarketClient {
    pub async fn update_mapping(&self, state: &AppState) -> anyhow::Result<()> {
        let uri = format!(
            "https://api.partner.market.yandex.ru/businesses/{}/offer-mappings/update",
            self.business_id()
        );
        let categories: Vec<i64> = vec![2226];
        let mut products = vec![];
        for category in categories {
            let pr = state.woo_client.products_by_category(category).await?;
            products.extend(pr)
        }
        let update = OfferMappings::from_ms(products);
        self.client
            .post(&uri)
            .bearer_auth(self.token())
            .json(&update)
            .send()
            .await?;
        Ok(())
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferMappings {
    pub offer_mappings: Vec<UpdateOfferMappingDTO>,
}
impl OfferMappings {
    pub fn from_ms(products: Vec<ProductFromWoo>) -> Self {
        let mut offer_mappings = vec![];
        for product in &products {
            let offer = UpdateOfferDTO::from_ms(product);
            let uom = UpdateOfferMappingDTO {
                offer,
                mapping: None,
            };
            offer_mappings.push(uom)
        }
        Self { offer_mappings }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct UpdateOfferMappingDTO {
    pub offer: UpdateOfferDTO,
    pub mapping: Option<UpdateMappingDTO>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct UpdateOfferDTO {
    /// sku
    pub offer_id: String,
    /// Составляйте название по схеме: тип + бренд или производитель + модель + особенности, если есть (например, цвет, размер или вес) и количество в упаковке.
    pub name: String,
    /// Указывайте конкретные категории — например, набор ножей лучше отнести к категории Столовые приборы, а не просто Посуда.
    pub category: String,
    /// Ссылки на изображения товара
    pub pictures: Vec<String>,
    pub videos: Option<Vec<String>>,
    /// Название бренда или производителя. Должно быть записано так, как его пишет сам бренд.
    pub vendor: String,
    pub barcodes: Option<Vec<String>>,
    pub description: String,
    /// Страна, где был произведен товар.
    pub manufacturer_countries: Option<Vec<String>>,
    /// Габариты упаковки и вес товара.
    pub weight_dimensions: Option<OfferWeightDimensionsDTO>,
    /// Артикул товара от производителя.
    pub vendor_code: Option<String>,
    pub tags: Option<Vec<String>>,
    pub shelf_life: Option<TimePeriodDTO>,
    pub life_time: Option<TimePeriodDTO>,
    pub guarantee_period: Option<TimePeriodDTO>,
    /// Код товара в единой Товарной номенклатуре внешнеэкономической деятельности (ТН ВЭД) — 10 или 14 цифр без пробелов.
    pub customs_commodity_code: Option<String>,
    pub certificates: Option<Vec<String>>,
    /// Количество грузовых мест. Параметр используется, если товар представляет собой несколько коробок, упаковок и так далее. Например, кондиционер занимает два места — внешний и внутренний блоки в двух коробках. Для товаров, занимающих одно место, не передавайте этот параметр.
    pub box_count: Option<i32>,
    /// Состояние уцененного товара.
    pub condition: Option<OfferConditionDTO>,
    #[serde(rename = "type")]
    /// Особый тип товара. Указывается, если товар — книга, аудиокнига, лекарство, музыка, видео или поставляется под заказ.
    pub offer_type: Option<OfferType>,
    /// Признак цифрового товара. Укажите true, если товар доставляется по электронной почте.
    pub downloadable: Option<bool>,
    /// Параметр включает для товара пометку 18+. Устанавливайте ее только для товаров, которые относятся к удовлетворению сексуальных потребностей.
    pub adult: Option<bool>,
    /// Если товар не предназначен для детей младше определенного возраста, укажите это.
    pub age: Option<AgeDTO>,
    /// Характеристики, которые есть только у товаров конкретной категории — например, диаметр колес велосипеда или материал подошвы обуви.
    pub params: Option<Vec<OfferParamDTO>>,
    /// Себестоимость — затраты на самостоятельное производство товара или закупку у производителя или поставщиков.
    pub purchase_price: Option<BasePriceDTO>,
    /// Дополнительные расходы на товар. Например, на доставку или упаковку.
    pub additional_expenses: Option<BasePriceDTO>,
    /// Цена для скидок с Маркетом. Маркет может компенсировать до половины скидки. Назначьте минимальную цену до вычета тарифов, по которой готовы продавать товар, а мы рассчитаем скидку и размер софинансирования. Если Маркет не готов софинансировать скидку, покупатель её не увидит.
    pub cofinance_price: Option<BasePriceDTO>,
}
impl UpdateOfferDTO {
    pub fn from_ms(product: &ProductFromWoo) -> Self {
        let pictures = match product.clone().images {
            Some(list) => {
                let mut res = vec![];
                for i in &list {
                    let Some(image) = i.clone().src else {
                        continue;
                    };
                    res.push(image)
                }
                res
            }
            None => vec![],
        };
        let mut vendor = String::new();
        let mut manufacturer_countries = None;
        let mut length = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        let mut params_vec = vec![];
        if let Some(attrs) = product.attributes.clone() {
            for attribute in &attrs {
                let Some(name) = attribute.name.clone() else {
                    continue;
                };
                if name.as_str() == "Бренд" {
                    let Some(options) = attribute.options.clone() else {
                        continue;
                    };
                    vendor = options[0].clone();
                }
                if name.as_str() == "Страна" {
                    let Some(options) = attribute.options.clone() else {
                        continue;
                    };
                    manufacturer_countries = Some(vec![options[0].clone()])
                }
                if name.as_str() == "Ширина рулона, м" {
                    let Some(options) = attribute.options.clone() else {
                        continue;
                    };
                    let w: f64 = options[0].parse().unwrap_or(0.0);
                    width = w * 100.0;
                    length = 1.0 / width;
                }
                if name.as_str() == "Общая толщина, мм" {
                    let Some(options) = attribute.options.clone() else {
                        continue;
                    };
                    let h: f64 = options[0].parse().unwrap_or(0.0);
                    height = h / 10.0;
                }
                if name.as_str() == "Размер плитки, см" {
                    let Some(_) = attribute.options.clone() else {
                        continue;
                    };
                    height *= 4.0;
                    width = 50.0;
                    length = 50.0;
                }
                if let Some(options) = attribute.options.clone() {
                    let param = OfferParamDTO {
                        name,
                        value: options[0].clone(),
                    };
                    params_vec.push(param);
                }
            }
        }
        let w = OfferWeightDimensionsDTO {
            length,
            width,
            height,
            weight: product
                .clone()
                .weight
                .unwrap_or(String::from("0.0"))
                .parse()
                .unwrap(),
        };
        Self {
            offer_id: product.id.to_string(),
            name: product.name.clone(),
            category: String::from("Ковролин и ковровая плитка"),
            pictures,
            videos: None,
            vendor,
            barcodes: None,
            description: product
                .clone()
                .description
                .unwrap_or("Цена указана за один квадратный метр".to_string()),
            manufacturer_countries,
            weight_dimensions: Some(w),
            vendor_code: product.sku.clone(),
            tags: None,
            shelf_life: None,
            life_time: None,
            guarantee_period: None,
            customs_commodity_code: None,
            certificates: None,
            box_count: None,
            condition: None,
            offer_type: None,
            downloadable: None,
            adult: None,
            age: None,
            params: Some(params_vec),
            purchase_price: None,
            additional_expenses: None,
            cofinance_price: None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMappingDTO {
    /// Идентификатор карточки на Маркете.
    pub market_sku: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct OfferWeightDimensionsDTO {
    /// Длина упаковки в см.
    pub length: f64,
    /// Ширина упаковки в см.
    pub width: f64,
    /// Высота упаковки в см.
    pub height: f64,
    /// Вес товара в кг с учетом упаковки (брутто).
    pub weight: f64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct TimePeriodDTO {
    pub time_period: i64,
    pub time_unit: TimeUnitType,
    pub comment: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct OfferConditionDTO {
    #[serde(rename = "type")]
    pub condition_type: Option<OfferConditionType>,
    pub quality: Option<OfferConditionQualityType>,
    pub reason: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferType {
    Medicine,
    Book,
    Audiobook,
    ArtistTitle,
    #[default]
    OnDemand,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeDTO {
    pub value: i64,
    pub age_unit: AgeUnitType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferParamDTO {
    pub name: String,
    pub value: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasePriceDTO {
    pub value: i64,
    pub currency_id: CurrencyId,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CurrencyId {
    #[default]
    RUR,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeUnitType {
    HOUR,
    DAY,
    WEEK,
    MONTH,
    #[default]
    YEAR,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OfferConditionType {
    #[default]
    PREOWNED,
    SHOWCASESAMPLE,
    REFURBISHED,
    REDUCTION,
    RENOVATED,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OfferConditionQualityType {
    #[default]
    PERFECT,
    EXCELLENT,
    GOOD,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgeUnitType {
    #[default]
    YEAR,
    MONTH,
}
