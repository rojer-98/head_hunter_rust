use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Locales {
    RU,
    EN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Hosts {
    #[serde(rename = "hh.ru")]
    RU,
    #[serde(rename = "rabota.by")]
    RABOTA,
    #[serde(rename = "hh1.az")]
    AZ,
    #[serde(rename = "hh.uz")]
    UZ,
    #[serde(rename = "hh.kz")]
    KZ,
    #[serde(rename = "headhunter.ge")]
    GE,
    #[serde(rename = "headhunter.kg")]
    KG,
}
