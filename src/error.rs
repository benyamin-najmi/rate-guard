use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitError {
    ZeroCapacity,
    ZeroLimit,
    ZeroWindow,
    InvalidRefillRate,
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCapacity => write!(f, "capacity must be greater than zero"),
            Self::ZeroLimit => write!(f, "limit must be greater than zero"),
            Self::ZeroWindow => write!(f, "window duration must be greater than zero"),
            Self::InvalidRefillRate => write!(f, "refill rate must be greater than zero"),
        }
    }
}

impl Error for RateLimitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_produces_readable_messages() {
        assert_eq!(
            RateLimitError::ZeroCapacity.to_string(),
            "capacity must be greater than zero"
        );
        assert_eq!(
            RateLimitError::ZeroLimit.to_string(),
            "limit must be greater than zero"
        );
        assert_eq!(
            RateLimitError::ZeroWindow.to_string(),
            "window duration must be greater than zero"
        );
        assert_eq!(
            RateLimitError::InvalidRefillRate.to_string(),
            "refill rate must be greater than zero"
        );
    }

    #[test]
    fn error_trait_is_implemented() {
        fn assert_error<E: Error>(_: &E) {}
        assert_error(&RateLimitError::ZeroCapacity);
    }
}