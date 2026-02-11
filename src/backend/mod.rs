// src/backend/mod.rs

//! HTTP backend abstraction for rustcurl.
//!
//! Supports multiple backends via feature flags:
//! - backend-curl: Uses curl crate (libcurl) - default, mature SSPI support
//! - backend-reqwest: Uses reqwest crate with custom negotiate implementation

#[cfg(feature = "backend-curl")]
pub mod curl_backend;

#[cfg(feature = "backend-reqwest")]
pub mod reqwest_backend;

use crate::curl::{config::RequestConfig, error::RequestError, response::Response};

/// HTTP backend trait that both curl and reqwest implement
pub trait HttpBackend {
    /// Name of the backend (for --version output)
    fn name(&self) -> &'static str;

    /// Version of the backend library
    fn version(&self) -> &'static str;

    /// Execute an HTTP request
    fn perform_request(&self, config: &RequestConfig) -> Result<Response, RequestError>;
}

/// Get the active backend based on compile-time features
pub fn get_backend() -> Box<dyn HttpBackend> {
    #[cfg(feature = "backend-curl")]
    {
        Box::new(curl_backend::CurlBackend::new())
    }

    #[cfg(all(feature = "backend-reqwest", not(feature = "backend-curl")))]
    {
        Box::new(reqwest_backend::ReqwestBackend::new())
    }
}

/// Display backend information
#[allow(dead_code)]
pub fn backend_info() -> String {
    let backend = get_backend();
    format!("Backend: {} {}", backend.name(), backend.version())
}
