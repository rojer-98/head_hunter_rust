use std::fmt::Display;

use crate::{
    dictionary::DefaultQuery,
    employers::{EmployersBrandedTemplates, EmployersDepartments, EmployersRegionActive},
    request_and_convert,
    utils::{HError, QueryHandler},
};

pub async fn get_employers_vacancy_areas_active<T: Display>(
    access_token: Option<String>,
    employer_id: T,
    query: Option<DefaultQuery>,
) -> Result<EmployersRegionActive, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/employers/{employer_id}/vacancy_areas/active"),
        method: GET,
        access_token: access_token,
        optional query,
        EmployersRegionActive
    )
}

pub async fn get_employers_departments<T: Display>(
    access_token: Option<String>,
    employer_id: T,
    query: Option<DefaultQuery>,
) -> Result<EmployersDepartments, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/employers/{employer_id}/departments"),
        method: GET,
        access_token: access_token,
        optional query,
        EmployersDepartments
    )
}

pub async fn get_branded_templates<T: Display>(
    access_token: Option<String>,
    employer_id: T,
    query: Option<DefaultQuery>,
) -> Result<EmployersBrandedTemplates, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/employers/{employer_id}/vacancy_branded_templates"),
        method: GET,
        access_token: access_token,
        optional query,
        EmployersBrandedTemplates
    )
}
