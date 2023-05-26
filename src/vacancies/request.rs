use std::fmt::Display;

use crate::{
    dictionary::DefaultQuery,
    request_and_convert,
    utils::{HError, QueryHandler, RequestError},
    vacancies::{Vacancies, VacanciesQuery, Vacancy, Visitors, VisitorsQuery},
};

pub async fn get_all_vacancies(query: Option<VacanciesQuery>) -> Result<Vacancies, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies"),
        method: GET,
        access_token: None,
        optional query,
        Vacancies
    )
}

pub async fn get_vacancy_id<T: Display>(
    vacancy_id: T,
    query: Option<DefaultQuery>,
) -> Result<Vacancy, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/{vacancy_id}"),
        method: GET,
        access_token: None,
        optional query,
        Vacancy
    )
}

pub async fn get_vacancy_id_visitors<T: Display>(
    access_token: Option<String>,
    vacancy_id: T,
    query: Option<VisitorsQuery>,
) -> Result<Visitors, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/{vacancy_id}/visitors"),
        method: GET,
        access_token: access_token,
        optional query,
        Visitors
    )
}

pub async fn get_blacklisted_vacancies(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<Vacancies, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/blacklisted"),
        method: GET,
        access_token: access_token,
        optional query,
        Vacancies
    )
}

pub async fn put_to_blacklisted_vacancies<T: Display>(
    access_token: Option<String>,
    vacancy_id: T,
    query: Option<DefaultQuery>,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/blacklisted/{vacancy_id}"),
        method: PUT,
        access_token: access_token,
        optional query,
        RequestError
    )
}

pub async fn delete_from_blacklisted_vacancies<T: Display>(
    access_token: Option<String>,
    vacancy_id: T,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/blacklisted/{vacancy_id}"),
        method: DELETE,
        access_token: access_token,
        optional query,
        RequestError
    )
}

pub async fn get_all_favorited_vacancies(
    access_token: Option<String>,
    query: Option<VisitorsQuery>,
) -> Result<Vacancies, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/favorited"),
        method: GET,
        access_token: access_token,
        optional query,
        Vacancies
    )
}

pub async fn put_to_favorited_vacancies<T: Display>(
    access_token: Option<String>,
    vacancy_id: T,
    query: Option<DefaultQuery>,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/favorited/{vacancy_id}"),
        method: PUT,
        access_token: access_token,
        optional query,
        RequestError
    )
}

pub async fn delete_from_favorited_vacancies<T: Display>(
    access_token: Option<String>,
    vacancy_id: T,
    query: Option<DefaultQuery>,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/favorited/{vacancy_id}"),
        method: DELETE,
        access_token: access_token,
        optional query,
        RequestError
    )
}
