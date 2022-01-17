use juniper::{graphql_value, FieldError};
use std::fmt::Display;

pub fn data_load_error<T: Display>(e: T) -> FieldError {
    FieldError::new(e, graphql_value!({ "code": "DATA_LOAD_ERROR" }))
}
