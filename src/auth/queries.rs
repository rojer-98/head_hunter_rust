use serde::{Deserialize, Serialize};
use serde_url_params::to_string;
use url::Url;

use derive::Query;

use crate::utils::{deserialize_url, serialize_url, QueryHandler};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct TakeAuthQuery {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    grant_type: GrantType,
    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    pub redirect_uri: Option<Url>,
}

impl TakeAuthQuery {
    pub fn new(client_id: String, client_secret: String, code: String, redirect_uri: Url) -> Self {
        Self {
            client_id,
            client_secret,
            code,
            redirect_uri: Some(redirect_uri),
            grant_type: GrantType::AuthorizationCode,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct RefreshAuthQuery {
    grant_type: GrantType,
    pub refresh_token: String,
}

impl RefreshAuthQuery {
    pub fn new(refresh_token: String) -> Self {
        Self {
            refresh_token,
            grant_type: GrantType::RefreshToken,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct TakeNewAuthQuery {
    pub client_id: String,
    pub client_secret: String,
    grant_type: GrantType,
}

impl TakeNewAuthQuery {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type: GrantType::ClientCredentials,
        }
    }
}
