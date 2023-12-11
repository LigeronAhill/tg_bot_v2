use anyhow::Result;
use serde::{Deserialize, Serialize};

use self::product::{MetaDataCreate, ProductFromWoo};

use super::{
    moy_sklad::product::{ProductFromMoySklad, SalePrice},
    AppState,
};
pub mod findproduct;
pub mod product;
#[derive(Clone)]
pub struct Woo {
    client_key: String,
    client_secret: String,
    client: reqwest::Client,
}

impl Woo {
    pub async fn new(client_key: String, client_secret: String) -> Self {
        Self {
            client_key,
            client_secret,
            client: reqwest::Client::builder()
                .gzip(true)
                .build()
                .expect("error building client"),
        }
    }
    pub fn client(&self) -> reqwest::Client {
        self.client.clone()
    }
    pub fn client_key(&self) -> String {
        self.client_key.clone()
    }
    pub fn client_secret(&self) -> String {
        self.client_secret.clone()
    }
    pub async fn get_categories(&self) -> Result<Vec<WOOCategoryDTO>> {
        let mut final_result = vec![];
        let mut page = 1;
        while page < 5 {
            let uri = format!(
                "https://safira.club/wp-json/wc/v3/products/categories?page={}&per_page=99",
                page
            );
            match self
                .client
                .get(&uri)
                .basic_auth(self.client_key(), Some(self.client_secret()))
                .send()
                .await?
                .json::<Vec<WOOCategoryDTO>>()
                .await
            {
                Ok(result) => final_result.extend(result),
                Err(e) => println!("{} on {}", e, page),
            }
            page += 1;
        }
        Ok(final_result)
    }
    pub async fn get_attributes(&self) -> Result<Vec<WOOAttributeDTO>> {
        let result: Vec<WOOAttributeDTO> = self
            .client
            .get("https://safira.club/wp-json/wc/v3/products/attributes")
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
    pub async fn create_product(
        &self,
        state: &AppState,
        product: ProductFromMoySklad,
    ) -> Result<i64> {
        let prod = product::WooProductCreate::from_ms(state, product.clone()).await?;
        let uri = product.meta.href.unwrap();
        let prices = product.sale_prices;
        let response = self
            .client
            .post("https://safira.club/wp-json/wc/v3/products")
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .json(&prod)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let id_value = response.get("id").ok_or(anyhow::Error::msg("no id!!!"))?;
        let id = id_value.as_i64().ok_or(anyhow::Error::msg("wrong id!"))?;
        if product.variants_count != 0 {
            self.update_variations(state, id, prices).await?
        }
        state.ms_client.update_external_code(&uri, id).await?;
        Ok(id)
    }
    pub async fn update_product(
        &self,
        state: &AppState,
        product: ProductFromMoySklad,
    ) -> Result<i64> {
        let prod = product::WooProductCreate::from_ms(state, product.clone()).await?;
        let prices = product.clone().sale_prices;
        let id: i64 = match product.external_code.parse() {
            Ok(id) => id,
            Err(_) => match self
                .get_woo_id(
                    &product
                        .article
                        .clone()
                        .ok_or(anyhow::Error::msg("no article!"))?,
                )
                .await
            {
                Ok(id) => id,
                Err(_) => self.create_product(state, product.clone()).await?,
            },
        };
        let url = format!("https://safira.club/wp-json/wc/v3/products/{}", id);
        self.client
            .put(&url)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .json(&prod)
            .send()
            .await?;
        if product.variants_count != 0 {
            self.update_variations(state, id, prices).await?;
        }
        let uri = product.meta.href.unwrap();
        state.ms_client.update_external_code(&uri, id).await?;
        Ok(id)
    }

    pub async fn delete_product(&self, product: ProductFromMoySklad) -> Result<i64> {
        let id: i64 = product.external_code.parse()?;
        let url = format!("https://safira.club/wp-json/wc/v3/products/{}", id);
        self.client
            .delete(&url)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?;
        Ok(id)
    }
    pub async fn retrieve_product(&self, id: i64) -> Result<ProductFromWoo> {
        let url = format!("https://safira.club/wp-json/wc/v3/products/{id}");
        let result = self
            .client
            .get(&url)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?
            .json::<ProductFromWoo>()
            .await?;
        Ok(result)
    }
    pub async fn create_category(&self, category_name: &str, parent_id: i64) -> Result<i64> {
        let params = CategoryToCreate {
            name: category_name.to_owned(),
            parent: parent_id,
        };
        let response = self
            .client
            .post("https://safira.club/wp-json/wc/v3/products/categories")
            .json(&params)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?;
        let value: serde_json::Value = response.json().await?;
        let id = value["id"].as_i64().unwrap();
        Ok(id)
    }
    pub async fn update_category(&self, id: i64, name: &str, parent_id: i64) -> Result<()> {
        let params = CategoryToCreate {
            name: name.to_owned(),
            parent: parent_id,
        };
        let url = format!("https://safira.club/wp-json/wc/v3/products/categories/{id}");
        self.client
            .put(&url)
            .json(&params)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?;
        Ok(())
    }
    pub async fn delete_category(&self, id: i64) -> Result<()> {
        let url = format!("https://safira.club/wp-json/wc/v3/products/categories/{id}");
        self.client
            .delete(&url)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?;
        Ok(())
    }
    pub async fn get_woo_id(&self, sku: &String) -> Result<i64> {
        let response = self
            .client
            .get("https://safira.club/wp-json/wc/v3/products")
            .query(&[("sku", sku)])
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?;
        let vec_id = response.json::<Vec<serde_json::Value>>().await?;
        if vec_id.is_empty() {
            Err(anyhow::Error::msg("no id"))
        } else {
            Ok(vec_id[0]["id"].as_i64().unwrap())
        }
    }
    pub async fn get_variation_id(&self, product_id: i64, sku: &String) -> Result<i64> {
        let uri = format!(
            "https://safira.club/wp-json/wc/v3/products/{}/variations",
            product_id
        );
        let response = self
            .client
            .get(&uri)
            .query(&[("sku", sku)])
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?;
        let vec_id = response.json::<Vec<serde_json::Value>>().await?;
        if vec_id.is_empty() {
            Err(anyhow::Error::msg("no id"))
        } else {
            Ok(vec_id[0]["id"].as_i64().unwrap())
        }
    }
    pub async fn get_variations(&self, id: i64) -> Result<Vec<VariationResponse>> {
        let url = format!("https://safira.club/wp-json/wc/v3/products/{id}/variations");
        let variations: Vec<VariationResponse> = self
            .client
            .get(&url)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?
            .json()
            .await?;
        Ok(variations)
    }

    pub async fn update_variations(
        &self,
        state: &AppState,
        id: i64,
        prices: Option<Vec<SalePrice>>,
    ) -> Result<()> {
        let variations = self.get_variations(id).await?;
        let product = self.retrieve_product(id).await?;
        let sku = product
            .clone()
            .sku
            .ok_or(anyhow::Error::msg("error getting sku"))?;
        let mut regular_price = String::from("0");
        let mut sale_price: Option<String> = None;
        if let Some(prices) = prices {
            for price in prices {
                if price.price_type.name.as_str() == "Цена продажи" {
                    let Some(cur_url) = price.currency.meta.href else {
                        return Err(anyhow::Error::msg("no currency!"));
                    };
                    let ms_curr_response = state
                        .ms_client
                        .client()
                        .get(cur_url)
                        .bearer_auth(&state.ms_client.token())
                        .send()
                        .await?;
                    let curr_val: serde_json::Value = ms_curr_response.json().await?;
                    let rate = curr_val["rate"].as_f64().unwrap();
                    regular_price = format!("{}", (rate * price.value) / 100.0);
                } else if price.price_type.name.as_str() == "Акция" && price.value > 0.0 {
                    let Some(cur_url) = price.currency.meta.href else {
                        return Err(anyhow::Error::msg("no currency!"));
                    };
                    let ms_curr_response = state
                        .ms_client
                        .client()
                        .get(cur_url)
                        .bearer_auth(&state.ms_client.token())
                        .send()
                        .await?;
                    let curr_val: serde_json::Value = ms_curr_response.json().await?;
                    let rate = curr_val["rate"].as_f64().unwrap();
                    sale_price = Some(format!("{}", (rate * price.value) / 100.0));
                }
            }
        }

        for variation in &variations {
            let url = format!(
                "https://safira.club/wp-json/wc/v3/products/{}/variations/{}",
                id, variation.id
            );
            let mut min_quantity = 1.0;
            let mut quantity_step = 1.0;
            let mut meta_data = vec![];
            let mut sku = sku.clone();
            let attribute = variation.attributes[0].clone();
            if attribute.name.as_str() == "Ширина рулона, м" {
                quantity_step = 0.1;
                let c = attribute.option.clone();
                let w: Vec<&str> = c.split_whitespace().collect();
                min_quantity = w[0].parse().unwrap_or(4.0);
                let w_to_sku: i32 = w[0].parse().unwrap_or(4);
                sku = format!("{}_{}", sku, w_to_sku);
                let meta_data_step = MetaDataCreate {
                    key: "_alg_wc_pq_step".to_string(),
                    value: format!("{quantity_step}"),
                };
                let meta_data_min = MetaDataCreate {
                    key: "_alg_wc_pq_min".to_string(),
                    value: format!("{min_quantity}"),
                };
                meta_data.push(meta_data_min);
                meta_data.push(meta_data_step);
            } else if attribute.name.as_str() == "Площадь упаковки, м2" {
                let c = attribute.option;
                quantity_step = c.parse().unwrap_or_default();
                min_quantity = c.parse().unwrap_or_default();
                let meta_data_step = MetaDataCreate {
                    key: "_alg_wc_pq_step".to_string(),
                    value: format!("{quantity_step}"),
                };
                let meta_data_min = MetaDataCreate {
                    key: "_alg_wc_pq_min".to_string(),
                    value: format!("{min_quantity}"),
                };
                meta_data.push(meta_data_min);
                meta_data.push(meta_data_step);
            } else {
                let meta_data_step = MetaDataCreate {
                    key: "_alg_wc_pq_step".to_string(),
                    value: format!("{quantity_step}"),
                };
                let meta_data_min = MetaDataCreate {
                    key: "_alg_wc_pq_min".to_string(),
                    value: format!("{min_quantity}"),
                };
                meta_data.push(meta_data_min);
                meta_data.push(meta_data_step);
            }
            let req = VariationUpdateRequest {
                sku,
                regular_price: regular_price.clone(),
                sale_price: sale_price.clone(),
                meta_data,
            };
            self.client
                .put(&url)
                .basic_auth(self.client_key(), Some(self.client_secret()))
                .json(&req)
                .send()
                .await?;
        }
        Ok(())
    }
    pub async fn find_product(&self, request: &str) -> Result<String> {
        let url = format!("https://safira.club/wp-json/wc/v3/products?search={request}");
        let response: Vec<findproduct::FindProduct> = self
            .client
            .get(&url)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?
            .json()
            .await?;
        let mut result = String::new();
        if !response.is_empty() {
            for product in &response {
                if let Some(variations) = product.clone().variations {
                    if variations.is_empty() {
                        result.push_str(&format!(
                            "\n\nНазвание: {}\nЦена: {}\nВ наличии: {}\n",
                            product.name,
                            product.price,
                            product.stock_quantity.unwrap_or(0)
                        ));
                    } else {
                        for variation in &variations {
                            let uri = format!(
                                "https://safira.club/wp-json/wc/v3/products/{}/variations/{}",
                                product.id, variation
                            );
                            let var_response: findproduct::FindProduct = self
                                .client
                                .get(&uri)
                                .basic_auth(self.client_key(), Some(self.client_secret()))
                                .send()
                                .await?
                                .json()
                                .await?;
                            result.push_str(&format!(
                                "\n\nНазвание: {}\nВариант: {}\nЦена: {}\nВ наличии: {}\n",
                                product.name,
                                var_response.name,
                                var_response.price,
                                var_response.stock_quantity.unwrap_or(0)
                            ));
                        }
                    }
                }
            }
        }
        Ok(result)
    }
    pub async fn products_by_category(
        &self,
        category_id: i64,
    ) -> anyhow::Result<Vec<ProductFromWoo>> {
        let mut page = 1;
        let uri = format!(
            "https://safira.club/wp-json/wc/v3/products?page={}&per_page=10&category={}",
            page, category_id
        );
        let mut result = vec![];
        loop {
            let response: Vec<ProductFromWoo> = self
                .client
                .get(&uri)
                .basic_auth(self.client_key(), Some(self.client_secret()))
                .send()
                .await?
                .json()
                .await?;
            if response.is_empty() {
                break;
            }
            result.extend(response);
            page += 1;
        }
        Ok(result)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariationUpdateRequest {
    pub sku: String,
    pub regular_price: String,
    pub sale_price: Option<String>,
    pub meta_data: Vec<MetaDataCreate>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariationResponse {
    pub id: i64,
    pub attributes: Vec<VariationAttribute>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariationAttribute {
    pub id: i64,
    pub name: String,
    pub option: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryToCreate {
    pub name: String,
    pub parent: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WOOCategoryDTO {
    pub id: i64,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WOOAttributeDTO {
    pub id: i64,
    pub name: String,
}

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
