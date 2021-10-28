#[derive(Debug, thiserror::Error)]
pub enum DecodingError {
    #[error("Missing field {field}")]
    MissingField { field: String },
    #[error("Unexpected field {field}")]
    UnexpectedField { field: String },
    #[error("Expected object: {expected_object}, found {actual_object}")]
    UnexpectedObject {
        expected_object: String,
        actual_object: String,
    },
    #[error("Document ended to soon")]
    UnexpectedEof,
    #[error("Unknown error")]
    Unknown,
}

impl DecodingError {
    pub fn missing_field<F: ToString>(field: F) -> Self {
        Self::MissingField {
            field: field.to_string(),
        }
    }

    pub fn unexpected_field<F: ToString>(field: F) -> Self {
        Self::UnexpectedField {
            field: field.to_string(),
        }
    }

    pub fn unexpected_object<E: ToString, A: ToString>(
        expected_object: E,
        actual_object: A,
    ) -> Self {
        Self::UnexpectedObject {
            expected_object: expected_object.to_string(),
            actual_object: actual_object.to_string(),
        }
    }
}
