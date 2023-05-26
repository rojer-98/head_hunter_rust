use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResponse {
    pub template: Option<String>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}
