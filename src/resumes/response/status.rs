use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeStatus {
    pub blocked: Option<bool>,
    pub can_publish_or_update: Option<bool>,
    pub finished: Option<bool>,
    pub moderation_note: Option<Vec<StatusModerationNote>>,
    pub progress: Option<StatusProgress>,
    pub publish_url: Option<String>,
    pub status: Option<Status>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusModerationNote {
    pub id: Option<String>,
    pub name: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusProgress {
    pub mandatory: Option<Vec<Status>>,
    pub percentage: Option<i64>,
    pub recommended: Option<Vec<Status>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: Option<String>,
    pub name: Option<String>,
}
