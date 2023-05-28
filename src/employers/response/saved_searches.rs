use serde::{Deserialize, Serialize};
use url::Url;

use crate::utils::{deserialize_url, serialize_url, RequestError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearchesVacancies {
    pub found: Option<i64>,
    pub items: Option<Vec<SavedSearchesVacanciesItem>>,
    pub page: Option<i64>,
    pub pages: Option<i64>,
    pub per_page: Option<i64>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearchesVacanciesItem {
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub items: Option<SavedSearchesVacanciesItemInner>,
    pub name: Option<String>,
    pub new_items: Option<SavedSearchesVacanciesItemInner>,
    pub subscription: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearchesVacanciesItemInner {
    pub count: Option<i64>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub url: Option<Url>,
}
