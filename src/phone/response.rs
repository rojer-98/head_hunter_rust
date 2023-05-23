use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumePhone {
    pub city: Option<String>,
    pub country: Option<String>,
    pub formatted: Option<String>,
    pub need_verification: Option<bool>,
    pub number: Option<String>,
    pub restricted_country: Option<bool>,
    pub verified: Option<bool>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeGeneratedCode {
    pub can_request_code_again_in: Option<String>,
    pub notification_type: Option<String>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}
