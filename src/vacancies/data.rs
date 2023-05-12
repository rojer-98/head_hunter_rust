use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use self::helpers::{deserialize_url, serialize_url};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Vacancy {
    pub accept_handicapped: bool,
    pub accept_incomplete_resumes: bool,
    pub accept_kids: bool,
    pub accept_temporary: bool,
    pub address: Address,
    pub allow_messages: bool,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub alternate_url: Option<Url>,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub apply_alternate_url: Option<Url>,
    pub archived: bool,
    pub area: Area,

    pub billing_type: IdAndName,
    pub branded_description: String,

    pub code: String,
    pub contacts: Contacts,
    pub created_at: DateTime<Utc>,

    pub department: IdAndName,
    pub description: String,
    pub driver_license_types: Vec<DriverLicenseType>,

    pub employer: Employer,
    pub employment: IdAndName,
    pub experience: IdAndName,

    pub has_test: bool,

    pub id: String,
    pub initial_created_at: DateTime<Utc>,
    pub insider_interview: InsiderInterview,
    pub key_skills: Vec<KeySkill>,

    pub languages: Vec<Language>,

    pub name: String,

    pub premium: bool,

    pub professional_roles: Vec<IdAndName>,
    pub published_at: DateTime<Utc>,

    pub response_letter_required: bool,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub response_url: Option<Url>,

    pub salary: Salary,
    pub schedule: IdAndName,
    pub specializations: Vec<Option<String>>,

    pub test: Test,
    #[serde(rename = "type")]
    pub _type: IdAndName,

    pub working_days: Vec<IdAndName>,
    pub working_time_intervals: Vec<IdAndName>,
    pub working_time_modes: Vec<IdAndName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Test {
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Salary {
    pub currency: String,
    pub from: Option<u32>,
    pub gross: bool,
    pub to: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Language {
    pub id: String,
    pub level: IdAndName,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeySkill {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsiderInterview {
    pub id: String,
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
    pub blacklisted: bool,
    pub id: String,
    pub logo_urls: LogoUrls,
    pub name: String,
    pub trusted: bool,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DriverLicenseType {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Area {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdAndName {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Contacts {
    pub email: String,
    pub name: String,
    pub phones: Vec<Phone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Phone {
    pub city: String,
    pub comment: Option<String>,
    pub country: String,
    pub number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Address {
    pub building: String,
    pub city: String,
    pub description: String,
    pub lat: f64,
    pub lng: f64,
    pub metro_stations: Vec<MetroStation>,
    pub street: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetroStation {
    pub lat: f64,
    pub line_id: String,
    pub line_name: String,
    pub lng: f64,
    pub station_id: String,
    pub station_name: String,
}

mod helpers {
    use serde::{
        de::{Deserialize, Deserializer, Error as _},
        ser::Serializer,
    };
    use url::Url;

    use std::borrow::Cow;

    pub fn deserialize_url<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Url>, D::Error> {
        let intermediate = <Option<Cow<'de, str>>>::deserialize(de)?;

        match intermediate.as_deref() {
            None | Some("") => Ok(None),
            Some(non_empty_string) => Url::parse(non_empty_string)
                .map(Some)
                .map_err(D::Error::custom),
        }
    }

    pub fn serialize_url<S>(url: &Option<Url>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(u) = url {
            s.serialize_str(u.as_str())
        } else {
            s.serialize_none()
        }
    }
}
