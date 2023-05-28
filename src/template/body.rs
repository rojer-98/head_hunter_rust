use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::utils::QueryHandler;

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct TemplateBody {
    pub template: Option<String>,
}
