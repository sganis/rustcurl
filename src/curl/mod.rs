// src/curl/mod.rs

pub mod args;
pub mod config;
pub mod error;
pub mod request;
pub mod response;

pub use args::{parse_args, parse_credentials, print_usage};
pub use config::{Method, RequestConfig};
pub use error::RequestError;
pub use request::perform_request;
pub use response::{Response, Timing};
