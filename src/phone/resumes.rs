use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeConfirm {
    pub phone: String,
    pub code: String,
}
