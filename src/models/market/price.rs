use serde::Deserialize;
use serde::Serialize;

use super::offer::CurrencyId;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offers {
    /// Список товаров с ценами.
    pub offers: Vec<UpdateBusinessOfferPriceDTO>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBusinessOfferPriceDTO {
    /// sku
    pub offer_id: String,
    pub price: UpdatePriceWithDiscountDTO,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePriceWithDiscountDTO {
    pub value: i64,
    pub curreny_id: CurrencyId,
    /// Цена до скидки.
    pub discount_base: i64,
}
