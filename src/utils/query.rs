use crate::{HError};

pub trait QueryHandler {
    fn into_query_string(&self) -> Result<String, HError>;
}
