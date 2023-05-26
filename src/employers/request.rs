use std::fmt::Display;

use crate::{
    dictionary::DefaultQuery,
    employers::{TemplateQuery, TemplateResponse},
    request_and_convert,
    utils::{HError, QueryHandler},
};

pub async fn get_employers_vacancy_areas_active<T: Display>(
    access_token: Option<String>,
    employer_id: T,
    query: Option<DefaultQuery>,
) -> Result<TemplateResponse, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/employers/{employer_id}/vacancy_areas/active"),
        method: GET,
        access_token: access_token,
        optional query,
        TemplateResponse
    )
}

pub async fn get_template<T: Display>(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<TemplateResponse, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/template"),
        method: GET,
        access_token: access_token,
        optional query,
        TemplateResponse
    )
}
