use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

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
    pub alternate_url: Option<String>,
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
    pub url: Option<String>,
    pub views_url: Option<String>,
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Access {
    #[serde(rename = "type")]
    pub access_type: Option<Gender>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gender {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    pub download: Option<Download>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub pdf: Option<Pdf>,
    pub rtf: Option<Pdf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pdf {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
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
pub struct Education {
    pub level: Option<Gender>,
    pub primary: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub area: Option<Area>,
    pub company: Option<String>,
    pub company_id: Option<serde_json::Value>,
    pub company_url: Option<String>,
    pub employer: Option<serde_json::Value>,
    pub end: Option<String>,
    pub industries: Option<Vec<Option<serde_json::Value>>>,
    pub industry: Option<Gender>,
    pub position: Option<String>,
    pub start: Option<String>,
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
pub struct Platform {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Salary {
    pub amount: Option<i64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarVacancies {
    pub counters: Option<Counters>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counters {
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalExperience {
    pub months: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueUnion {
    String(String),
    ValueClass(ValueClass),
}
