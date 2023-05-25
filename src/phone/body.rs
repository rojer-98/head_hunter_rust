use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::utils::QueryHandler;

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct ResumePhoneConfirmBody {
    pub phone: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct ResumePhoneGenerateCodeBody {
    pub phone: String,
}
