use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::utils::{deserialize_url, serialize_url};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Vacancies {
    pub items: Vec<Vacancy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Vacancy {
    pub accept_handicapped: Option<bool>,
    pub accept_incomplete_resumes: Option<bool>,
    pub accept_kids: Option<bool>,
    pub accept_temporary: Option<bool>,
    pub address: Option<Address>,
    pub allow_messages: Option<bool>,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub alternate_url: Option<Url>,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub apply_alternate_url: Option<Url>,
    pub archived: bool,
    pub area: Area,

    pub billing_type: Option<IdAndName>,
    pub branded_description: Option<Option<String>>,

    pub code: Option<Option<String>>,
    pub contacts: Option<Contacts>,
    pub created_at: DateTime<Utc>,

    pub department: Option<IdAndName>,
    pub description: Option<Option<String>>,
    pub driver_license_types: Option<Vec<DriverLicenseType>>,

    pub employer: Option<Employer>,
    pub employment: Option<IdAndName>,
    pub experience: Option<IdAndName>,

    pub has_test: Option<bool>,

    pub id: Option<String>,
    pub initial_created_at: Option<DateTime<Utc>>,
    pub insider_interview: Option<InsiderInterview>,
    pub key_skills: Option<Vec<KeySkill>>,

    pub languages: Option<Vec<Language>>,

    pub name: Option<String>,

    pub premium: bool,

    pub professional_roles: Vec<Option<IdAndName>>,
    pub published_at: DateTime<Utc>,

    pub response_letter_required: bool,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub response_url: Option<Url>,

    pub salary: Option<Salary>,
    pub schedule: Option<IdAndName>,
    pub specializations: Option<Vec<Option<Option<String>>>>,

    pub test: Option<Test>,
    #[serde(rename = "type")]
    pub _type: Option<IdAndName>,

    pub working_days: Vec<Option<IdAndName>>,
    pub working_time_intervals: Vec<Option<IdAndName>>,
    pub working_time_modes: Vec<Option<IdAndName>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Test {
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Salary {
    pub currency: Option<String>,
    pub from: Option<u32>,
    pub gross: bool,
    pub to: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Language {
    pub id: Option<String>,
    pub level: Option<IdAndName>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeySkill {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsiderInterview {
    pub id: Option<String>,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogoUrls {
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    #[serde(rename = "90")]
    pub _90: Option<Url>,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    #[serde(rename = "240")]
    pub _240: Option<Url>,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub original: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Employer {
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub alternate_url: Option<Url>,
    pub blacklisted: Option<bool>,
    pub id: Option<String>,
    pub logo_urls: Option<LogoUrls>,
    pub name: Option<String>,
    pub trusted: bool,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DriverLicenseType {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Area {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdAndName {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Contacts {
    pub email: Option<String>,
    pub name: Option<String>,
    pub phones: Vec<Phone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Phone {
    pub city: Option<String>,
    pub comment: Option<Option<String>>,
    pub country: Option<String>,
    pub number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Address {
    pub building: Option<String>,
    pub city: Option<String>,
    pub description: Option<String>,
    pub lat: f64,
    pub lng: f64,
    pub metro_stations: Vec<MetroStation>,
    pub street: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetroStation {
    pub lat: f64,
    pub line_id: Option<String>,
    pub line_name: Option<String>,
    pub lng: f64,
    pub station_id: Option<String>,
    pub station_name: Option<String>,
}
