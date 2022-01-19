use juniper::GraphQLObject;

use crate::data_sources::models;

#[derive(GraphQLObject, Default)]
pub struct ValidationError {
    #[graphql(description = "If empty, it means a general error (eg: record not found)")]
    pub field: Option<String>,
    pub message: String,
}

#[derive(GraphQLObject)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

pub fn to_graphql_validation_errors(err: models::ValidationErrors) -> ValidationErrors {
    ValidationErrors {
        errors: err
            .iter()
            .map(|err| ValidationError {
                field: Some(err.field.clone()),
                message: err.message.clone(),
            })
            .collect(),
    }
}
