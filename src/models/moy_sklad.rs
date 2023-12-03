use anyhow::Result;
use mongodb::bson::oid::ObjectId;

use serde::Deserialize;
use serde::Serialize;

use self::product::ProductFromMoySklad;

use super::woocommerce::product::VariationProperties;
use super::AppState;

pub mod product;
#[derive(Clone)]
pub struct MoySklad {
    token: String,
    client: reqwest::Client,
}

impl MoySklad {
    pub async fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::builder()
                .gzip(true)
                .build()
                .expect("error building client"),
        }
    }
    pub fn client(&self) -> reqwest::Client {
        self.client.clone()
    }
    pub fn token(&self) -> String {
        self.token.clone()
    }
    pub async fn get_categories(&self) -> Result<Vec<MSCategoryDTO>> {
        let response: GetCategoriesResponse = self
            .client
            .get("https://api.moysklad.ru/api/remap/1.2/entity/productfolder")
            .bearer_auth(self.token())
            .send()
            .await?
            .json()
            .await?;
        let result = response.rows;
        Ok(result)
    }
    pub async fn get_attributes(&self) -> Result<Vec<MSAttributeDTO>> {
        let response: GetAttributeResponse = self
            .client
            .get("https://api.moysklad.ru/api/remap/1.2/entity/product/metadata/attributes")
            .bearer_auth(self.token())
            .send()
            .await?
            .json()
            .await?;
        let result = response.rows;
        Ok(result)
    }
    pub async fn retrieve_product(&self, uri: &String) -> Result<ProductFromMoySklad> {
        let ms_product: ProductFromMoySklad = self
            .client
            .get(uri)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(ms_product)
    }

    pub async fn get_variants(
        &self,
        state: &AppState,
        product: &ProductFromMoySklad,
    ) -> Result<Vec<VariationProperties>> {
        let mut result = vec![];
        let uri = format!(
            "https://api.moysklad.ru/api/remap/1.2/entity/variant?filter=productid={}",
            product.id
        );
        let response = self
            .client
            .get(uri)
            .bearer_auth(self.token())
            .send()
            .await?;
        let value: serde_json::Value = response.json().await?;
        let chars = value["rows"].as_array().unwrap();
        for char in chars {
            let name = char["characteristics"][0]["name"]
                .as_str()
                .unwrap()
                .to_string();
            let id = state.storage.attribute_id(&name).await.unwrap_or(0);
            let option = char["characteristics"][0]["value"]
                .as_str()
                .unwrap()
                .to_string();
            result.push(VariationProperties { id, name, option })
        }
        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoriesResponse {
    pub rows: Vec<MSCategoryDTO>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAttributeResponse {
    pub rows: Vec<MSAttributeDTO>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MSCategoryDTO {
    pub id: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MSAttributeDTO {
    pub id: String,
    pub name: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    pub audit_context: AuditContext,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditContext {
    pub meta: Meta,
    pub uid: String,
    pub moment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub href: Option<String>,
    pub metadata_href: Option<String>,
    pub media_type: Option<String>,
    pub uuid_href: Option<String>,
    pub download_href: Option<String>,
    pub size: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub meta: Meta,
    pub action: Action,
    // pub account_id: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    CREATE,
    #[default]
    UPDATE,
    DELETE,
}
