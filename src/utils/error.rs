use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::*;

#[derive(Error, Debug)]
pub enum HError {
    #[error(transparent)]
    Std {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    Utf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },
    #[error("Error with std mutex or rwlock")]
    Sync,
    #[error("Digest auth error happened: {source}")]
    Digest {
        #[from]
        source: digest::DigestError,
    },
    #[error("Reqwest error happened: {source}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("Serde json error happened: {source}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error("Regex error happened: {source}")]
    Regex {
        #[from]
        source: regex::Error,
    },
    #[error("Serde url error happened: {source}")]
    SerdeUrl {
        #[from]
        source: serde_url_params::Error,
    },
    #[error("params to connection is not set")]
    NotSet,
    #[error("api is not supported")]
    NotAvialiableApi,
    #[error("auth token cannot be convert")]
    AuthTokenConvert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OauthError {
    #[serde(rename = "token-revoked")]
    TokenRevoked,
    #[serde(rename = "token-expired")]
    TokenExpired,
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub struct RequestError {
    pub request_id: Option<String>,
    pub description: Option<String>,
    pub bad_argument: Option<String>,
    pub bad_arguments: Option<Vec<BadArgument>>,
    pub errors: Option<Vec<RequestErrorInner>>,
    pub oauth_error: Option<String>,
}

impl RequestError {
    pub fn is_error(&self) -> bool {
        self.description.is_some() || self.errors.is_some() || self.oauth_error.is_some()
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request id {:?}\nDescription: {:?}\nInners: {:?}",
            self.request_id, self.description, self.errors
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub struct RequestErrorInner {
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub value: Option<String>,
}

impl Display for RequestErrorInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.error_type, self.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadArgument {
    pub description: Option<String>,
    pub name: Option<String>,
}
