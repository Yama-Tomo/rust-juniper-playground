#[derive(Default, Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub type ValidationErrors = Vec<ValidationError>;
