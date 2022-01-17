use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::data_sources::DataSources;

// NOTE: これがないと context を参照するオブジェクトを実装した際の型があわないので注意
impl juniper::Context for Context {}

pub struct Context {
    pub datasources: DataSources,
}

pub fn create(conn: &Arc<DatabaseConnection>) -> Context {
    Context {
        datasources: DataSources::new(conn),
    }
}
