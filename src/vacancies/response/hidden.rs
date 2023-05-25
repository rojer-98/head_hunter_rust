use serde::{Deserialize, Serialize};

use crate::{utils::RequestError, vacancies::Vacancy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VacanciesHidden {
    pub found: Option<i64>,
    pub items: Option<Vec<Vacancy>>,
    pub limit_reached: Option<bool>,
    pub page: Option<i64>,
    pub pages: Option<i64>,
    pub per_page: Option<i64>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}
