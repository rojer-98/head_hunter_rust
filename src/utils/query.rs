use crate::{HError, utils::RequestError};

pub trait QueryHandler {
    fn into_query_string(&self) -> Result<String, HError>;
}
