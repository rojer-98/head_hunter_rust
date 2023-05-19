use crate::{
    request_and_convert,
    utils::{HError, QueryHandler},
};

pub async fn resume_phone_confirm<T: QueryHandler>(query: T) -> Result<(), HError> {
    request_and_convert!(
        url: "https://api.hh.ru/resume_phone_confirm",
        method: POST,
        query,
        ()
    )
}
