use std::fmt::Display;

use crate::{
    request_and_convert,
    template::{TemplateQuery, TemplateResponse},
    utils::{HError, QueryHandler},
};

pub async fn get_template<T: Display>(
    access_token: Option<String>,
    query: Option<TemplateQuery>,
) -> Result<TemplateResponse, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/template"),
        method: GET,
        access_token: access_token,
        optional query,
        TemplateResponse
    )
}
