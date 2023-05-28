use serde::{Deserialize, Serialize};

use crate::{
    dictionary::{Counters, Description, File, Type},
    utils::RequestError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioConditions {
    pub counters: Option<Counters>,
    pub fields: Option<PortfolioConditionsFields>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioConditionsFields {
    pub description: Option<Description>,
    pub file: Option<File>,
    #[serde(rename = "type")]
    pub fields_type: Option<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoConditions {
    pub counters: Option<Counters>,
    pub fields: Option<PhotoConditionsFields>,
    #[serde(flatten)]
    pub error: Option<RequestError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoConditionsFields {
    pub description: Option<Description>,
    pub file: Option<File>,
    #[serde(rename = "type")]
    pub fields_type: Option<Type>,
}
