use serde::{Deserialize, Serialize};
use url::Url;

use crate::utils::{deserialize_url, serialize_url, RequestError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub found: Option<i64>,
    pub items: Option<Vec<PhotoItem>>,
    pub page: Option<i64>,
    pub pages: Option<i64>,
    pub per_page: Option<i64>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoItem {
    pub description: Option<String>,
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub medium: Option<Url>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub small: Option<Url>,
    pub state: Option<PhotoState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoState {
    pub id: Option<String>,
    pub name: Option<String>,
}
