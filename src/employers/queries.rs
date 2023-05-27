use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::{
    dictionary::{Hosts, Locales},
    utils::QueryHandler,
};

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct SavedSearchesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub locale: Option<Locales>,
    pub host: Option<Hosts>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct UpdateSavedSearchesQuery {
    pub name: Option<String>,
    pub subscription: Option<String>,
    pub locale: Option<Locales>,
    pub host: Option<Hosts>,
}
