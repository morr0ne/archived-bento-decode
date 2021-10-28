#[derive(Debug, thiserror::Error)]
pub enum DecodingError {
    #[error("Missing field {field}")]
    MissingField { field: String },
    #[error("Unexpected field {field}")]
    UnexpectedField { field: String },
    #[error("Unknown error")]
    Unknown,
}

impl DecodingError {
    pub fn missing_field<T: ToString>(field: T) -> Self {
        Self::MissingField {
            field: field.to_string(),
        }
    }

    pub fn unexpected_field<T: ToString>(field: T) -> Self {
        Self::UnexpectedField {
            field: field.to_string(),
        }
    }
}
