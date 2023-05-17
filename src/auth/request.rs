use log::trace;
use serde_json::from_str;

use crate::{
    auth::Token,
    utils::{request, AuthType, HError, Header, Method, QueryHandler, RequestType},
};

async fn get_auth<T: QueryHandler>(query: Option<T>) -> Result<Token, HError> {
    let raw_url = "https://api.hh.ru/oauth/token".to_string();

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
        Method::POST,
        Some(vec![Header::USER_AGENT]),
    )
    .await?;

    trace!("Req is {:?}", req);

    Ok(from_str::<Token>(&req)?)
}
