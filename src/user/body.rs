use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::utils::QueryHandler;

pub trait MeChange {}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct MeChangeNameBody {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct MeSetInSearch {
    pub is_in_search: bool,
}

impl MeChange for MeChangeNameBody {}
impl MeChange for MeSetInSearch {}
