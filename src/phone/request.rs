use crate::{
    request_and_convert,
    utils::{HError, QueryHandler, RequestError},
};

pub async fn resume_phone_confirm<T: QueryHandler>(query: T) -> Result<RequestError, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_phone_confirm",
        method: POST,
        query,
        RequestError
    )
}

pub async fn resume_should_send_sms<T: QueryHandler>(query: T) -> Result<RequestError, HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_should_send_sms",
        method: GET,
        query,
        RequestError
    )
}
