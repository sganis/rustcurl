// src/curl/error.rs

use std::fmt;

#[derive(Debug)]
pub enum RequestError {
    Curl(curl::Error),
    Io(std::io::Error),
    Config(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Curl(e) => write!(f, "curl error: {e}"),
            RequestError::Io(e) => write!(f, "io error: {e}"),
            RequestError::Config(msg) => write!(f, "config error: {msg}"),
        }
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::Curl(e) => Some(e),
            RequestError::Io(e) => Some(e),
            RequestError::Config(_) => None,
        }
    }
}

impl From<curl::Error> for RequestError {
    fn from(e: curl::Error) -> Self {
        RequestError::Curl(e)
    }
}

impl From<std::io::Error> for RequestError {
    fn from(e: std::io::Error) -> Self {
        RequestError::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn display_config_error() {
        let err = RequestError::Config("bad url".to_string());
        assert_eq!(format!("{err}"), "config error: bad url");
    }

    #[test]
    fn display_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err = RequestError::Io(io_err);
        let msg = format!("{err}");
        assert!(msg.contains("io error"));
        assert!(msg.contains("file missing"));
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let err: RequestError = io_err.into();
        assert!(matches!(err, RequestError::Io(_)));
    }

    #[test]
    fn error_trait_source() {
        let err = RequestError::Config("x".into());
        assert!(err.source().is_none());

        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "y");
        let err = RequestError::Io(io_err);
        assert!(err.source().is_some());
    }
}
