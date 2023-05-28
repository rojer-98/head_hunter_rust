use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    dictionary::{Actions, Download, Education, Experience, Gender, Platform, TotalExperience},
    utils::RequestError,
    utils::{deserialize_url, serialize_url},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeMine {
    pub found: Option<i64>,
    pub items: Option<Vec<ResumeMineItem>>,
    pub page: Option<i64>,
    pub pages: Option<i64>,
    pub per_page: Option<i64>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeMineItem {
    pub access: Option<Access>,
    pub actions: Option<Actions>,
    pub age: Option<i64>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub alternate_url: Option<Url>,
    pub area: Option<Area>,
    pub blocked: Option<bool>,
    pub can_publish_or_update: Option<bool>,
    pub certificate: Option<Vec<Option<serde_json::Value>>>,
    pub contact: Option<Vec<Contact>>,
    pub created: Option<String>,
    pub created_at: Option<String>,
    pub download: Option<Download>,
    pub education: Option<Education>,
    pub experience: Option<Vec<Experience>>,
    pub finished: Option<bool>,
    pub first_name: Option<String>,
    pub gender: Option<Gender>,
    pub hidden_fields: Option<Vec<Gender>>,
    pub id: Option<String>,
    pub last_name: Option<String>,
    pub marked: Option<bool>,
    pub middle_name: Option<serde_json::Value>,
    pub new_views: Option<i64>,
    pub next_publish_at: Option<String>,
    pub paid_services: Option<Vec<PaidService>>,
    pub photo: Option<Photo>,
    pub platform: Option<Platform>,
    pub salary: Option<Salary>,
    pub similar_vacancies: Option<SimilarVacancies>,
    pub status: Option<Gender>,
    pub title: Option<String>,
    pub total_experience: Option<TotalExperience>,
    pub total_views: Option<i64>,
    pub updated: Option<String>,
    pub updated_at: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub url: Option<Url>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub views_url: Option<Url>,
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Access {
    #[serde(rename = "type")]
    pub access_type: Option<Gender>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub comment: Option<String>,
    pub need_verification: Option<bool>,
    pub preferred: Option<bool>,
    #[serde(rename = "type")]
    pub contact_type: Option<Gender>,
    pub value: Option<ValueUnion>,
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueClass {
    pub city: Option<String>,
    pub country: Option<String>,
    pub formatted: Option<String>,
    pub number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidService {
    pub active: Option<bool>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "40")]
    pub the_40: Option<String>,
    #[serde(rename = "100")]
    pub the_100: Option<String>,
    #[serde(rename = "500")]
    pub the_500: Option<String>,
    pub id: Option<String>,
    pub medium: Option<String>,
    pub small: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Salary {
    pub amount: Option<i64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarVacancies {
    pub counters: Option<Counters>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counters {
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueUnion {
    String(String),
    ValueClass(ValueClass),
}
