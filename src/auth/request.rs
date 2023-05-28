use crate::{
    auth::Token,
    request_and_convert,
    utils::{HError, QueryHandler},
};

pub async fn oauth<T: QueryHandler>(query: T) -> Result<Token, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/oauth/token"),
        method: POST,
        access_token: None,
        query,
        Token
    )
}

pub async fn authorize<T: QueryHandler>(query: T) -> Result<Token, HError> {
    request_and_convert!(
        url: format!("https://hh.ru/oauth/authorize"),
        method: GET,
        access_token: None,
        query,
        Token
    )
}
