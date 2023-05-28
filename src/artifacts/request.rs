use std::fmt::Display;

use crate::{
    artifacts::{
        ArtifactLoadSuccess, DescriptionBody, Photo, PhotoConditions, Portfolio,
        PortfolioConditions,
    },
    dictionary::DefaultQuery,
    request_and_convert,
    utils::{HError, QueryHandler, RequestError},
};

pub async fn get_portfolio_conditions(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<PortfolioConditions, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/artifacts/portfolio/conditions"),
        method: GET,
        access_token: access_token,
        optional query,
        PortfolioConditions
    )
}

pub async fn put_artifact<T: Display>(
    access_token: Option<String>,
    id: T,
    query: Option<DefaultQuery>,
    body: DescriptionBody,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/artifacts/{id}"),
        method: PUT,
        access_token: access_token,
        optional query,
        RequestError,
        body
    )
}

pub async fn delete_artifact<T: Display>(
    access_token: Option<String>,
    id: T,
    query: Option<DefaultQuery>,
) -> Result<RequestError, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/artifacts/{id}"),
        method: DELETE,
        access_token: access_token,
        optional query,
        RequestError
    )
}

// TODO: implement reqwest multi-part sending file
pub async fn post_artifact<T: Display>(
    access_token: Option<String>,
    id: T,
    query: Option<DefaultQuery>,
) -> Result<ArtifactLoadSuccess, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/artifacts/{id}"),
        method: POST,
        access_token: access_token,
        optional query,
        ArtifactLoadSuccess
    )
}

pub async fn get_portfolio(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<Portfolio, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/artifacts/portfolio"),
        method: GET,
        access_token: access_token,
        optional query,
        Portfolio
    )
}

pub async fn get_photo_conditions(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<PhotoConditions, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/artifacts/photo/conditions"),
        method: GET,
        access_token: access_token,
        optional query,
        PhotoConditions
    )
}

pub async fn get_photo(
    access_token: Option<String>,
    query: Option<DefaultQuery>,
) -> Result<Photo, HError> {
    request_and_convert!(
        url: format!("https://api.hh.ru/artifacts/photo"),
        method: GET,
        access_token: access_token,
        optional query,
        Photo
    )
}
