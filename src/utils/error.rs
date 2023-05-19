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
pub struct RequestError {
    pub request_id: Option<String>,
    pub description: Option<String>,
    pub errors: Option<Vec<RequestErrorInner>>,
    pub oauth_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestErrorInner {
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub value: Option<String>,
}
