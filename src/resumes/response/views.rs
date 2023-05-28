use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    dictionary::LogoUrls,
    utils::{deserialize_url, serialize_url, RequestError},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeViews {
    pub found: Option<i64>,
    pub items: Option<Vec<ResumeViewsItem>>,
    pub page: Option<i64>,
    pub pages: Option<i64>,
    pub per_page: Option<i64>,
    pub resume: Option<Resume>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeViewsItem {
    pub created_at: Option<String>,
    pub employer: Option<Employer>,
    pub viewed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employer {
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub alternate_url: Option<Url>,
    pub id: Option<String>,
    pub logo_urls: Option<LogoUrls>,
    pub name: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub url: Option<Url>,
    pub vacancies_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub alternate_url: Option<Url>,
    pub id: Option<String>,
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub url: Option<Url>,
}
