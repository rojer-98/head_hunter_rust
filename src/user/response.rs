use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    employers::Employer,
    utils::{deserialize_url, serialize_url, RequestError},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub auth_type: Option<String>,
    pub is_admin: Option<bool>,
    pub is_applicant: Option<bool>,
    pub is_application: Option<bool>,
    pub is_employer: Option<bool>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<String>,
    pub is_anonymous: Option<bool>,
    pub last_name: Option<String>,
    pub mid_name: Option<String>,
    pub middle_name: Option<String>,
    pub phone: Option<String>,
    pub counters: Option<Counters>,
    pub employer: Option<Employer>,
    pub is_in_search: Option<bool>,
    pub manager: Option<Manager>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub negotiations_url: Option<Url>,
    pub personal_manager: Option<PersonalManager>,
    pub profile_videos: Option<ProfileVideos>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub resumes_url: Option<Url>,
    pub user_statuses: Option<UserStatuses>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counters {
    pub new_resume_views: Option<i64>,
    pub resumes_count: Option<i64>,
    pub unread_negotiations: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileVideos {
    pub items: Option<Vec<Item>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub download_url: Option<DownloadUrl>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrl {
    pub expires_at: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatuses {
    pub job_search_status: Option<JobSearchStatus>,
    pub when_can_start_work_status: Option<WhenCanStartWorkStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSearchStatus {
    pub description: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhenCanStartWorkStatus {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manager {
    pub has_admin_rights: Option<bool>,
    pub has_multiple_manager_accounts: Option<bool>,
    pub id: Option<String>,
    pub is_main_contact_person: Option<bool>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub manager_settings_url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalManager {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<String>,
    pub is_available: Option<bool>,
    pub last_name: Option<String>,
    pub photo_urls: Option<PhotoUrls>,
    pub unavailable: Option<Unavailable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoUrls {
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub big: Option<Url>,
    #[serde(
        default,
        deserialize_with = "deserialize_url",
        serialize_with = "serialize_url"
    )]
    pub small: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unavailable {
    pub until: Option<String>,
}
