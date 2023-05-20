use crate::{
    request_and_convert,
    utils::{HError, QueryHandler},
    vacancies::{Vacancies, VacanciesQuery, Vacancy, VacancyQuery, Visitors, VisitorsQuery},
};

pub async fn get_all_vacancies(query: Option<VacanciesQuery>) -> Result<Vacancies, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies"),
        method: GET,
        optional query,
        Vacancies
    )
}

pub async fn get_vacancy_id(
    vacancy_id: usize,
    query: Option<VacancyQuery>,
) -> Result<Vacancy, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/{vacancy_id}"),
        method: GET,
        optional query,
        Vacancy
    )
}

pub async fn get_vacancy_id_visitors(
    vacancy_id: usize,
    query: Option<VisitorsQuery>,
) -> Result<Visitors, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/vacancies/{vacancy_id}/visitors"),
        method: GET,
        optional query,
        Visitors
    )
}