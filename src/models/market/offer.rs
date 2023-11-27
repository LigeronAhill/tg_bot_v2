use serde::Deserialize;
use serde::Serialize;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferMappings {
    pub offer_mappings: Vec<UpdateOfferMappingDTO>,
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
