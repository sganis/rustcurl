// src/backend/curl_backend.rs

//! curl backend - wraps the existing curl implementation

use super::HttpBackend;
use crate::curl::{config::RequestConfig, error::RequestError, response::Response};

pub struct CurlBackend;

impl CurlBackend {
    pub fn new() -> Self {
        Self
    }
}

impl HttpBackend for CurlBackend {
    fn name(&self) -> &'static str {
        "curl"
    }

    fn version(&self) -> &'static str {
        curl::Version::num()
    }

    fn perform_request(&self, config: &RequestConfig) -> Result<Response, RequestError> {
        // Delegate to the existing curl implementation
        crate::curl::request::perform_request(config)
    }
}
