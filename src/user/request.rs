use std::fmt::Display;

use crate::{
    dictionary::DefaultQuery,
    request_and_convert,
    user::{MeChange, UserInfo},
    utils::{HError, QueryHandler, RequestError},
};

pub async fn get_user(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<UserInfo, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/me"),
        method: GET,
        access_token: access_token,
        optional query,
        UserInfo
    )
}

pub async fn post_user<T: MeChange + QueryHandler>(
    access_token: Option<String>,
    query: DefaultQuery,
    body: T,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/me"),
        method: POST,
        access_token: access_token,
        query,
        RequestError,
        body
    )
}
