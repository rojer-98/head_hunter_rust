use serde::{Deserialize, Serialize};
use serde_url_params::to_string;

use derive::Query;

use crate::{
    dictionary::{Hosts, Locales},
    utils::QueryHandler,
};

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct PhoneConfirmQuery {
    pub locale: Option<Locales>,
    pub host: Option<Hosts>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Query)]
pub struct ResumeShouldSendSMSQuery {
    pub phone: String,
    pub locale: Option<Locales>,
    pub host: Option<Hosts>,
}
