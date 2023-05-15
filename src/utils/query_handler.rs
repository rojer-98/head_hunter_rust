use crate::HError;

pub trait QueryHandler {
    fn into_query_string(&self) -> Result<String, HError>;
}

#[macro_export]
macro_rules! implement_query_handler {
    ( $( $class:ident ),* ) => {
        $(
            impl QueryHandler for $class {
                fn into_query_string(&self) -> Result<String, HError> {
                    Ok(to_string(self)?)
                }
            }
        )*
    };
}
