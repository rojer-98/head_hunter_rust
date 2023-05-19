use crate::{
    auth::Token,
    request_and_convert,
    utils::{HError, QueryHandler},
};

pub async fn get_auth<T: QueryHandler>(query: T) -> Result<Token, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/oauth/token"),
        method: POST,
        query,
        Token
    )
}
