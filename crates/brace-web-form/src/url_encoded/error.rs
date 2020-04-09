use std::error::Error;
use std::fmt::{self, Display};

use brace_web_core::error::{PayloadError, ResponseError};
use brace_web_core::http::StatusCode;

#[derive(Debug)]
pub enum UrlEncodedError {
    Chunked,
    ContentType,
    Overflow { size: usize, limit: usize },
    Parse,
    Payload(PayloadError),
    UnknownLength,
}

impl Display for UrlEncodedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Chunked => write!(f, "Unable to decode chunked transfer encoding"),
            Self::ContentType => write!(f, "Content type error"),
            Self::Overflow { size, limit } => write!(
                f,
                "Payload size ({} bytes) exceeds limit ({} bytes)",
                size, limit
            ),
            Self::Parse => write!(f, "Parse error"),
            Self::Payload(payload) => write!(f, "Error reading payload: {}", payload),
            Self::UnknownLength => write!(f, "Payload size is not known"),
        }
    }
}

impl Error for UrlEncodedError {}

impl ResponseError for UrlEncodedError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Overflow { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::UnknownLength => StatusCode::LENGTH_REQUIRED,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<PayloadError> for UrlEncodedError {
    fn from(err: PayloadError) -> Self {
        Self::Payload(err)
    }
}
