use log::trace;
use serde_json::from_str;

use crate::{
    utils::{request, AuthType, HError, Header, Method, RequestType},
    vacancies::{Vacancies, Vacancy},
};

pub async fn get_all_vacancies() -> Result<Vacancies, HError> {
    let req = request(
        RequestType::Reqwest,
        "https://api.hh.ru/vacancies".to_string(),
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

pub async fn get_vacancy_id(vacancy_id: usize) -> Result<Vacancy, HError> {
    let req = request(
        RequestType::Reqwest,
        format!("https://api.hh.ru/vacancies/{vacancy_id}"),
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
