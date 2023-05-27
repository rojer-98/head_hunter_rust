use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactLoadSuccess {
    pub id: Option<String>,
    pub medium: Option<String>,
    pub small: Option<String>,
    pub state: Option<ArtifactLoadSuccessState>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactLoadSuccessState {
    pub id: Option<String>,
    pub name: Option<String>,
}
