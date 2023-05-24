use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    dictionary::{
        Actions, Area, Download, Education, Experience, Gender, Platform, Salary, TotalExperience,
    },
    utils::{deserialize_url, serialize_url},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Visitors {
    pub found: i64,

    pub hidden_on_page: i64,

    pub items: Vec<VisitorsItem>,

    pub page: i64,
    pub pages: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VisitorsItem {
    pub actions: Actions,
    pub age: i64,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub alternate_url: Option<Url>,
    pub area: Area,

    pub can_view_full_info: bool,
    pub certificate: Vec<String>,
    pub created_at: DateTime<Utc>,

    pub download: Download,

    pub education: Education,
    pub experience: Vec<Experience>,

    pub favorited: bool,
    pub first_name: Option<String>,

    pub gender: Gender,

    pub hidden_fields: Option<Vec<String>>,

    pub id: String,

    pub last_name: Option<String>,

    pub marked: bool,
    pub middle_name: Option<String>,

    pub negotiations_history: NegotiationsHistory,

    pub owner: Owner,

    pub photo: Option<String>,
    pub platform: Platform,

    pub salary: Salary,

    pub title: String,
    pub total_experience: TotalExperience,

    pub updated_at: String,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub url: Option<Url>,

    pub viewed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationsHistory {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Owner {
    pub comments: Option<Comments>,
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Comments {
    pub counters: Counters,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Counters {
    pub total: i64,
}
