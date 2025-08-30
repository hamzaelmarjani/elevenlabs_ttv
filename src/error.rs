use std::fmt;

/// All possible errors that can occur when using the ElevenLabs API
#[derive(Debug)]
pub enum ElevenLabsTTVError {
    /// HTTP request failed (network issues, timeout, etc.)
    RequestError(reqwest::Error),

    /// API returned an error status code
    ApiError { status: u16, message: String },

    /// Failed to parse JSON response
    ParseError(reqwest::Error),

    /// Invalid API key or authentication failed
    AuthenticationError(String),

    /// Rate limit exceeded
    RateLimitError {
        retry_after: Option<u64>, // seconds
        message: String,
    },

    /// Quota exceeded (not enough credits)
    QuotaExceededError(String),

    /// Invalid input parameters
    ValidationError(String),
}

impl fmt::Display for ElevenLabsTTVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElevenLabsTTVError::RequestError(e) => write!(f, "Request failed: {}", e),
            ElevenLabsTTVError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            ElevenLabsTTVError::ParseError(e) => write!(f, "Failed to parse response: {}", e),
            ElevenLabsTTVError::AuthenticationError(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
            ElevenLabsTTVError::RateLimitError {
                retry_after,
                message,
            } => match retry_after {
                Some(seconds) => write!(
                    f,
                    "Rate limit exceeded (retry in {}s): {}",
                    seconds, message
                ),
                None => write!(f, "Rate limit exceeded: {}", message),
            },
            ElevenLabsTTVError::QuotaExceededError(msg) => write!(f, "Quota exceeded: {}", msg),
            ElevenLabsTTVError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ElevenLabsTTVError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ElevenLabsTTVError::RequestError(e) => Some(e),
            ElevenLabsTTVError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ElevenLabsTTVError {
    fn from(error: reqwest::Error) -> Self {
        // Check if it's a specific HTTP status error
        if let Some(status) = error.status() {
            let status_code = status.as_u16();
            match status_code {
                401 => ElevenLabsTTVError::AuthenticationError("Invalid API key".to_string()),
                429 => {
                    // Try to extract retry-after header if available
                    ElevenLabsTTVError::RateLimitError {
                        retry_after: None, // Could be enhanced to parse Retry-After header
                        message: "Too many requests".to_string(),
                    }
                }
                402 => ElevenLabsTTVError::QuotaExceededError("Insufficient credits".to_string()),
                _ => ElevenLabsTTVError::ApiError {
                    status: status_code,
                    message: error.to_string(),
                },
            }
        } else {
            ElevenLabsTTVError::RequestError(error)
        }
    }
}
