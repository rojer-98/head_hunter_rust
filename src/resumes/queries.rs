use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use crate::{
    dictionary::{Hosts, Locales},
    implement_query_handler,
    utils::{HError, QueryHandler},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeQuery {
    pub locale: Option<Locales>,
    pub host: Option<Hosts>,
}

implement_query_handler!(ResumeQuery);
