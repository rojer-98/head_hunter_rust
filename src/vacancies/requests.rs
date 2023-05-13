use serde_json::from_str;

use crate::{
    utils::{request, AuthType, HError, Header, Method, RequestType},
    vacancies::Vacancies,
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

    println!("Req is: {:?}", req);

    Ok(from_str::<Vacancies>(&req)?)
}
