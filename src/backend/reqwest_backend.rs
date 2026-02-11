// src/backend/reqwest_backend.rs

//! reqwest backend - uses reqwest crate with custom negotiate support

#![allow(dead_code)]

use super::HttpBackend;
use crate::curl::{
    config::{Method, RequestConfig},
    error::RequestError,
    response::Response,
};

pub struct ReqwestBackend;

impl ReqwestBackend {
    pub fn new() -> Self {
        Self
    }
}

impl HttpBackend for ReqwestBackend {
    fn name(&self) -> &'static str {
        "reqwest"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn perform_request(&self, config: &RequestConfig) -> Result<Response, RequestError> {
        // Use blocking reqwest since rustcurl is synchronous
        let client = build_client(config)?;
        let mut request_builder = match &config.method {
            Method::Get => client.get(&config.url),
            Method::Post => client.post(&config.url),
            Method::Put => client.put(&config.url),
            Method::Delete => client.delete(&config.url),
            Method::Head => client.head(&config.url),
            Method::Patch => client.patch(&config.url),
            Method::Options => client.request(reqwest::Method::OPTIONS, &config.url),
            Method::Custom(method) => {
                let method = reqwest::Method::from_bytes(method.as_bytes())
                    .map_err(|e| RequestError::Config(format!("Invalid method: {}", e)))?;
                client.request(method, &config.url)
            }
        };

        // Add headers
        for header_str in &config.headers {
            if let Some((name, value)) = header_str.split_once(':') {
                request_builder = request_builder.header(name.trim(), value.trim());
            }
        }

        // Add bearer token
        if let Some(ref token) = config.bearer {
            request_builder = request_builder.bearer_auth(token);
        }

        // Add body
        if let Some(ref data) = config.data {
            request_builder = request_builder.body(data.clone());
        }

        // Execute request
        let response = request_builder.send()?;

        // Convert response
        let status_code = response.status().as_u16() as u32;

        let mut headers = Vec::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.push(format!("{}: {}", name, value_str));
            }
        }

        let body = response.bytes()?.to_vec();

        Ok(Response {
            status_code,
            headers,
            body,
            timing: None, // reqwest doesn't expose detailed timing
        })
    }
}

fn build_client(config: &RequestConfig) -> Result<reqwest::blocking::Client, RequestError> {
    let mut builder = reqwest::blocking::Client::builder();

    // Authentication
    if config.negotiate {
        if config.username.is_some() || config.password.is_some() {
            // Use negotiate with credentials (fallback support)
            let username = config.username.as_deref().unwrap_or("");
            let password = config.password.as_deref().unwrap_or("");
            builder = builder.negotiate_with_credentials(username, password);
        } else {
            // Use negotiate with current user (Kerberos only)
            builder = builder.negotiate();
        }
    } else if config.ntlm {
        // NTLM not directly supported in reqwest negotiate feature
        // Would fall back to Basic if credentials provided
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            builder = builder.negotiate_with_credentials(username, password);
        }
    }

    // TLS options
    if config.insecure {
        builder = builder.tls_danger_accept_invalid_certs(true);
    }

    if let Some(ref path) = config.cacert {
        let cert = std::fs::read(path)?;
        builder = builder.add_root_certificate(reqwest::Certificate::from_pem(&cert)?);
    }

    // Proxy
    if let Some(ref proxy_url) = crate::curl::request::resolve_proxy(config) {
        let mut proxy = reqwest::Proxy::all(proxy_url)?;

        // Proxy authentication
        if config.proxy_negotiate || config.proxy_ntlm {
            // Note: reqwest doesn't support proxy negotiate/NTLM directly
            // This is a limitation compared to curl backend
            // Fall back to basic auth if proxy credentials are provided
            if let Some(ref user) = config.proxy_user {
                let pass = config.proxy_password.as_deref().unwrap_or("");
                proxy = proxy.basic_auth(user, pass);
            }
        } else if let Some(ref user) = config.proxy_user {
            let pass = config.proxy_password.as_deref().unwrap_or("");
            proxy = proxy.basic_auth(user, pass);
        }

        builder = builder.proxy(proxy);
    }

    if let Some(_noproxy) = crate::curl::request::resolve_noproxy(config) {
        builder = builder.no_proxy();
    }

    // Timeouts
    if let Some(d) = config.connect_timeout {
        builder = builder.connect_timeout(d);
    }

    if let Some(d) = config.max_time {
        builder = builder.timeout(d);
    }

    // Redirects
    if let Some(max) = config.max_redirs {
        builder = builder.redirect(reqwest::redirect::Policy::limited(max as usize));
    }

    // User agent
    if let Some(ref ua) = config.user_agent {
        builder = builder.user_agent(ua);
    }

    Ok(builder.build()?)
}

// Convert reqwest errors to RequestError
impl From<reqwest::Error> for RequestError {
    fn from(e: reqwest::Error) -> Self {
        RequestError::Http(e.to_string())
    }
}

// Note: From<std::io::Error> is already implemented in curl/error.rs
