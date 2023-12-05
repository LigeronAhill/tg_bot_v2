use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{moy_sklad::product::ProductFromMoySklad, AppState};
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
    ) -> Result<()> {
        let prod = product::WooProductCreate::from_ms(state, product).await?;
        self.client
            .post("https://safira.club/wp-json/wc/v3/products")
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .json(&prod)
            .send()
            .await?;
        Ok(())
    }
    pub async fn update_product(
        &self,
        state: &AppState,
        product: ProductFromMoySklad,
    ) -> Result<()> {
        let prod = product::WooProductCreate::from_ms(state, product.clone()).await?;
        let Some(sku) = product.article.clone() else {
            return Err(anyhow::Error::msg("NO SKU!!!"));
        };
        match self.get_woo_id(&sku).await {
            Ok(id) => {
                let url = format!("https://safira.club/wp-json/wc/v3/products/{}", id);
                self.client
                    .put(&url)
                    .basic_auth(self.client_key(), Some(self.client_secret()))
                    .json(&prod)
                    .send()
                    .await?;
            }
            Err(_) => self.create_product(state, product.clone()).await?,
        }

        Ok(())
    }
    pub async fn delete_product(&self, product: ProductFromMoySklad) -> Result<()> {
        let Some(sku) = product.article else {
            return Err(anyhow::Error::msg("NO SKU!!!"));
        };
        let id = self.get_woo_id(&sku).await?;
        let url = format!("https://safira.club/wp-json/wc/v3/products/{}", id);
        self.client
            .delete(&url)
            .basic_auth(self.client_key(), Some(self.client_secret()))
            .send()
            .await?;
        Ok(())
    }

    async fn create_category(&self, category_name: &str, parent_id: i64) -> Result<i64> {
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
    pub async fn get_woo_id(&self, sku: &String) -> Result<i64> {
        let response = self
            .client
            .get("https://safira.club//wp-json/wc/v3/products")
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
