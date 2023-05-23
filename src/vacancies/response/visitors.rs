use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    utils::{deserialize_url, serialize_url},
    vacancies::{Area, Salary},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Visitors {
    pub found: i64,

    pub hidden_on_page: i64,

    pub items: Vec<Item>,

    pub page: i64,
    pub pages: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Actions {
    pub download: Download,
    pub download_with_contact: DownloadWithContact,

    pub get_with_contact: GetWithContact,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Download {
    pub pdf: Pdf,
    pub rtf: Rtf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Pdf {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Rtf {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadWithContact {
    pub pdf: Pdf,
    pub rtf: Rtf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWithContact {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Education {
    pub level: Level,
    pub primary: Vec<Primary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Level {
    pub id: String,
    pub name: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Experience {
    pub area: Area,
    pub company: String,
    pub company_id: String,
    pub company_url: String,
    pub employer: Option<String>,
    pub end: String,
    pub industries: Vec<Industry>,
    pub industry: Industry,
    pub position: String,
    pub start: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Industry {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Gender {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NegotiationsHistory {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Owner {
    pub comments: Comments,
    pub id: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Platform {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TotalExperience {
    pub months: i64,
}
