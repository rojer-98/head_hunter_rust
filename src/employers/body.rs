use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::utils::QueryHandler;

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct ResumeVisibilityIds {
    pub items: Option<Vec<ResumeVisibilityId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct ResumeVisibilityId {
    pub id: Option<String>,
}
