use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::{
    dictionary::{Hosts, Locales},
    utils::QueryHandler,
};

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct ResumeQuery {
    pub locale: Option<Locales>,
    pub host: Option<Hosts>,
}
