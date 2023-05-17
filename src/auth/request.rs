use log::trace;
use serde_json::from_str;

use crate::{
    auth::Token,
    utils::{request, AuthType, HError, Header, Method, QueryHandler, RequestType},
};

pub async fn get_auth<T: QueryHandler>(query: T) -> Result<Token, HError> {
    let raw_url = "https://api.hh.ru/oauth/token".to_string();

    let ser_q = query.into_query_string()?;
    let url = format!("{raw_url}?{}", ser_q);

    let req = request(
        RequestType::Reqwest,
        url,
        None,
        (None, None),
        AuthType::No,
        Method::POST,
        Some(vec![Header::USER_AGENT]),
    )
    .await?;

    trace!("Req is {:?}", req);

    Ok(from_str::<Token>(&req)?)
}
