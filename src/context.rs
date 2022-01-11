use juniper;

use crate::data_sources::Database;

// NOTE: これがないと context を参照するオブジェクトを実装した際の型があわないので注意
impl juniper::Context for Context {}

pub struct Context {
    pub db: Database,
}

pub fn create() -> Context {
    Context {
        db: Database::new(),
    }
}
