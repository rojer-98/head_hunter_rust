use serde::{Deserialize, Serialize};
use url::Url;

use crate::utils::{deserialize_url, serialize_url};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Locales {
    RU,
    EN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Hosts {
    #[serde(rename = "hh.ru")]
    RU,
    #[serde(rename = "rabota.by")]
    RABOTA,
    #[serde(rename = "hh1.az")]
    AZ,
    #[serde(rename = "hh.uz")]
    UZ,
    #[serde(rename = "hh.kz")]
    KZ,
    #[serde(rename = "headhunter.ge")]
    GE,
    #[serde(rename = "headhunter.kg")]
    KG,
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
pub struct Download {
    pub pdf: Option<Pdf>,
    pub rtf: Option<Pdf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pdf {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Actions {
    pub download: Option<Download>,
    pub download_with_contact: Option<DownloadWithContact>,

    pub get_with_contact: Option<GetWithContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Rtf {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadWithContact {
    pub pdf: Option<Pdf>,
    pub rtf: Option<Rtf>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWithContact {
    pub url: Option<String>,
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
pub struct Salary {
    pub currency: Option<String>,
    pub from: Option<u32>,
    pub gross: Option<bool>,
    pub to: Option<u32>,
    pub amount: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub area: Option<Area>,
    pub company: Option<String>,
    pub company_id: Option<String>,
    pub company_url: Option<String>,
    pub employer: Option<String>,
    pub end: Option<String>,
    pub industries: Option<Vec<Option<Industry>>>,
    pub industry: Option<Gender>,
    pub position: Option<String>,
    pub start: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gender {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platform {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalExperience {
    pub months: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub level: Option<Gender>,
    pub primary: Option<Vec<Option<Primary>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Primary {
    pub name: String,
    pub name_id: Option<String>,
    pub organization: String,
    pub organization_id: Option<String>,
    pub result: String,
    pub result_id: Option<String>,
    pub year: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Level {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Industry {
    pub id: String,
    pub name: String,
}
