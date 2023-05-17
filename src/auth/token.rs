use serde::{Deserialize, Serialize};

use crate::utils::HError;

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
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,

    pub summary: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,

    pub request_id: Option<String>,
    pub description: Option<String>,
    pub errors: Option<Vec<TokenError>>,
    pub oauth_error: Option<String>,
}

impl Token {
    pub fn is_success(&self) -> bool {
        self.access_token.is_some()
            && self.token_type.is_some()
            && self.expires_in.is_some()
            && self.refresh_token.is_some()
    }

    pub fn is_not_found(&self) -> bool {
        self.summary.is_some() && self.error.is_some() && self.error_description.is_some()
    }

    pub fn is_bad(&self) -> bool {
        self.request_id.is_some()
            && self.description.is_some()
            && self.errors.is_some()
            && self.oauth_error.is_some()
    }

    pub fn to_success(self) -> Result<SuccessToken, HError> {
        if self.is_success() {
            Ok(SuccessToken {
                access_token: self.access_token.unwrap(),
                token_type: self.token_type.unwrap(),
                expires_in: self.expires_in.unwrap(),
                refresh_token: self.refresh_token.unwrap(),
            })
        } else {
            Err(HError::AuthTokenConvert)
        }
    }
}
