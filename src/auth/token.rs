use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundToken {
    pub summary: String,
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadToken {
    pub request_id: String,
    pub description: String,
    pub errors: Vec<TokenError>,
    pub oauth_error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    #[serde(flatten)]
    pub success_token: Option<SuccessToken>,
    #[serde(flatten)]
    pub bad_token: Option<BadToken>,
    #[serde(flatten)]
    pub not_found_token: Option<NotFoundToken>,
}
