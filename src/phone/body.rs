use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use crate::{
    implement_query_handler,
    utils::{HError, QueryHandler},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumePhoneConfirmBody {
    pub phone: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumePhoneGenerateCodeBody {
    pub phone: String,
}

implement_query_handler!(ResumePhoneConfirmBody, ResumePhoneGenerateCodeBody);
