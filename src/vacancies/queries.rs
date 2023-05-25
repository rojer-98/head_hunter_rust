use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::{
    dictionary::{Hosts, Locales},
    utils::{HError, QueryHandler},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Query)]
pub struct VacanciesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub text: Option<String>,
    pub search_field: Option<String>,
    pub experience: Option<String>,
    pub employment: Option<String>,
    pub schedule: Option<String>,
    pub area: Option<String>,
    pub metro: Option<String>,
    pub professional_roles: Option<String>,
    pub industry: Option<String>,
    pub employer_id: Option<String>,
    pub currency: Option<String>,
    pub salary: Option<u32>,
    pub label: Option<String>,
    pub only_with_salary: Option<bool>,
    pub period: Option<u32>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub top_lat: Option<u32>,
    pub bottom_lat: Option<u32>,
    pub left_lng: Option<u32>,
    pub right_lng: Option<u32>,
    pub order_by: Option<String>,
    pub sort_point_lat: Option<u32>,
    pub sort_point_lng: Option<u32>,
    pub clusters: Option<bool>,
    pub describe_arguments: Option<bool>,
    pub no_magic: Option<bool>,
    pub premium: Option<bool>,
    pub responses_count_enabled: Option<bool>,
    pub part_time: Option<String>,
    pub locale: Option<String>,
    pub host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Query)]
pub struct VacancyQuery {
    pub locale: Option<Locales>,
    pub host: Option<Hosts>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Query)]
pub struct VisitorsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub locale: Option<String>,
    pub host: Option<String>,
}
