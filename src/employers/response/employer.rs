use serde::{Deserialize, Serialize};
use url::Url;

use crate::utils::{deserialize_url, serialize_url, RequestError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployersRegionActive {
    pub found: Option<i64>,
    pub items: Option<Vec<EmployersRegionActiveItem>>,
    pub page: Option<i64>,
    pub pages: Option<i64>,
    pub per_page: Option<i64>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployersRegionActiveItem {
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
pub struct EmployersDepartments {
    pub items: Option<Vec<EmployersDepartmentsItem>>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployersDepartmentsItem {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployersBrandedTemplates {
    pub items: Option<Vec<EmployersBrandedTemplatesItem>>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployersBrandedTemplatesItem {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employer {
    pub id: Option<String>,
    pub manager_id: Option<String>,
    pub name: Option<String>,
}
