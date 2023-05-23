use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use crate::{
    implement_query_handler,
    utils::{deserialize_url, serialize_url, HError, QueryHandler},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumePhoneConfirmQuery {
    pub locale: Option<String>,
    pub host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeShouldSendSMSQuery {
    pub phone: String,
    pub locale: Option<String>,
    pub host: Option<String>,
}

implement_query_handler!(ResumePhoneConfirmQuery, ResumeShouldSendSMSQuery);
