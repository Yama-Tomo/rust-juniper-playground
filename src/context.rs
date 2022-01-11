use crate::data_sources::Database;

pub struct Context {
    pub db: Database,
}

pub fn create() -> Context {
    Context {
        db: Database::new(),
    }
}
