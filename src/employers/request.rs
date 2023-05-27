use std::fmt::Display;

use crate::{
    dictionary::DefaultQuery,
    employers::{
        EmployersBrandedTemplates, EmployersDepartments, EmployersRegionActive, SavedSearchesQuery,
        SavedSearchesVacancies, SavedSearchesVacanciesItem, UpdateSavedSearchesQuery,
    },
    request_and_convert,
    utils::{HError, QueryHandler, RequestError},
    vacancies::VacanciesQuery,
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

pub async fn get_saved_searches_vacancies(
    access_token: Option<String>,
    query: Option<SavedSearchesQuery>,
) -> Result<SavedSearchesVacancies, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/saved_searches/vacancies"),
        method: GET,
        access_token: access_token,
        optional query,
        SavedSearchesVacancies
    )
}

pub async fn post_saved_searches_vacancies(
    access_token: Option<String>,
    query: Option<VacanciesQuery>,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/saved_searches/vacancies"),
        method: POST,
        access_token: access_token,
        optional query,
        RequestError
    )
}

pub async fn get_saved_searches_vacancy<T: Display>(
    access_token: Option<String>,
    id: T,
    query: Option<DefaultQuery>,
) -> Result<SavedSearchesVacanciesItem, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/saved_searches/vacancies/{id}"),
        method: GET,
        access_token: access_token,
        optional query,
        SavedSearchesVacanciesItem
    )
}

pub async fn put_saved_searches_vacancy<T: Display>(
    access_token: Option<String>,
    id: T,
    query: Option<UpdateSavedSearchesQuery>,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/saved_searches/vacancies/{id}"),
        method: PUT,
        access_token: access_token,
        optional query,
        RequestError
    )
}

pub async fn delete_saved_searches_vacancy<T: Display>(
    access_token: Option<String>,
    id: T,
    query: Option<DefaultQuery>,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/saved_searches/vacancies/{id}"),
        method: DELETE,
        access_token: access_token,
        optional query,
        RequestError
    )
}
