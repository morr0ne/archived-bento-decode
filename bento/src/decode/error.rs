#[derive(Debug, thiserror::Error)]
pub enum DecodingError {
    #[error("")]
    MissingDictionaryValue,
    #[error("Missing field {field}")]
    MissingField { field: String },
    #[error("Unexpected field {field}")]
    UnexpectedField { field: String },
    #[error("Expected object: {expected_object}, found {actual_object}")]
    UnexpectedObject {
        expected_object: &'static str,
        actual_object: &'static str,
    },
    #[error("Document ended to soon")]
    UnexpectedEof,
    #[error("Unknown error")]
    Unknown,
}

impl DecodingError {
    pub const fn missing_field(field: String) -> Self {
        Self::MissingField { field }
    }

    pub const fn unexpected_field(field: String) -> Self {
        Self::UnexpectedField { field }
    }

    pub const fn unexpected_object(
        expected_object: &'static str,
        actual_object: &'static str,
    ) -> Self {
        Self::UnexpectedObject {
            expected_object,
            actual_object,
        }
    }
}
