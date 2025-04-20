use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    Other(String),
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::RequestError(e) => write!(f, "Request error: {}", e),
            ApiError::ParseError(e) => write!(f, "Parse error: {}", e),
            ApiError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> ApiError {
        ApiError::RequestError(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> ApiError {
        ApiError::ParseError(err)
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> ApiError {
        ApiError::Other(err)
    }
}