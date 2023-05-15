use log::trace;
use serde_json::from_str;

use crate::{
    utils::{request, AuthType, HError, Header, Method, QueryHandler, RequestType},
    vacancies::{Vacancies, VacanciesQuery, Vacancy, VacancyQuery},
};

pub async fn get_all_vacancies(query: Option<VacanciesQuery>) -> Result<Vacancies, HError> {
    let raw_url = "https://api.hh.ru/vacancies".to_string();

    let url = if let Some(q) = query {
        let ser_q = q.into_query_string()?;
        format!("{raw_url}?{}", ser_q)
    } else {
        raw_url
    };

    let req = request(
        RequestType::Reqwest,
        url,
        None,
        (None, None),
        AuthType::No,
        Method::GET,
        Some(vec![Header::USER_AGENT]),
    )
    .await?;

    trace!("Req is {:?}", req);

    Ok(from_str::<Vacancies>(&req)?)
}

pub async fn get_vacancy_id(
    vacancy_id: usize,
    query: Option<VacancyQuery>,
) -> Result<Vacancy, HError> {
    let raw_url = format!("https://api.hh.ru/vacancies/{vacancy_id}");

    let url = if let Some(q) = query {
        let ser_q = q.into_query_string()?;
        format!("{raw_url}?{}", ser_q)
    } else {
        raw_url
    };

    let req = request(
        RequestType::Reqwest,
        url,
        None,
        (None, None),
        AuthType::No,
        Method::GET,
        Some(vec![Header::USER_AGENT]),
    )
    .await?;

    trace!("Req is {:?}", req);

    Ok(from_str::<Vacancy>(&req)?)
}

pub async fn get_vacancy_id_visitors(vacancy_id: usize) -> Result<Vacancy, HError> {
    let req = request(
        RequestType::Reqwest,
        format!("https://api.hh.ru/vacancies/{vacancy_id}/visitors"),
        None,
        (None, None),
        AuthType::No,
        Method::GET,
        Some(vec![Header::USER_AGENT]),
    )
    .await?;

    trace!("Req is {:?}", req);

    Ok(from_str::<Vacancy>(&req)?)
}
