use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

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
    pub alternate_url: Option<String>,
    pub id: Option<String>,
    pub logo_urls: Option<LogoUrls>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub vacancies_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoUrls {
    #[serde(rename = "90")]
    pub the_90: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub alternate_url: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
}
