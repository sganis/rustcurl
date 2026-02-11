// src/curl/error.rs

use std::fmt;

#[derive(Debug)]
pub enum RequestError {
    #[cfg(feature = "curl")]
    Curl(curl::Error),
    Io(std::io::Error),
    #[allow(dead_code)]
    Config(String),
    #[allow(dead_code)]
    Http(String), // Generic HTTP error for non-curl backends
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "curl")]
            RequestError::Curl(e) => write!(f, "curl error: {e}"),
            RequestError::Io(e) => write!(f, "io error: {e}"),
            RequestError::Config(msg) => write!(f, "config error: {msg}"),
            RequestError::Http(msg) => write!(f, "http error: {msg}"),
        }
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "curl")]
            RequestError::Curl(e) => Some(e),
            RequestError::Io(e) => Some(e),
            RequestError::Config(_) | RequestError::Http(_) => None,
        }
    }
}

#[cfg(feature = "curl")]
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

impl RequestError {
    pub fn hint(&self) -> Option<&'static str> {
        match self {
            #[cfg(feature = "curl")]
            RequestError::Curl(e) if e.is_couldnt_resolve_host() => Some(
                "Hint: DNS resolution failed. If behind a corporate proxy, set HTTPS_PROXY or use -x <proxy-url>",
            ),
            #[cfg(feature = "curl")]
            RequestError::Curl(e) if e.is_couldnt_resolve_proxy() => Some(
                "Hint: Could not resolve proxy hostname. Check your proxy URL",
            ),
            #[cfg(feature = "curl")]
            RequestError::Curl(e) if e.is_ssl_connect_error() || e.is_peer_failed_verification() => Some(
                "Hint: SSL error. Try --insecure (-k), --cacert <path>, or --ssl-no-revoke for revocation issues",
            ),
            #[cfg(feature = "curl")]
            RequestError::Curl(e) if format!("{e}").contains("revocation") => Some(
                "Hint: Certificate revocation check failed. Try --ssl-no-revoke to disable revocation checks",
            ),
            #[cfg(feature = "curl")]
            RequestError::Curl(e) if format!("{e}").contains("407") => Some(
                "Hint: Proxy requires authentication (407). Try --proxy-negotiate for Kerberos/SPNEGO or --proxy-user <user:pass>",
            ),
            _ => None,
        }
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

    #[test]
    fn hint_none_for_config_error() {
        let err = RequestError::Config("bad".into());
        assert!(err.hint().is_none());
    }

    #[test]
    fn hint_none_for_io_error() {
        let err = RequestError::Io(std::io::Error::new(std::io::ErrorKind::Other, "x"));
        assert!(err.hint().is_none());
    }
}
