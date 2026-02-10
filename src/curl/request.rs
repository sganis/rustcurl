// src/curl/request.rs

use curl::easy::{Auth, Easy, List, SslOpt};
use std::env;
use std::fs;
use std::time::Duration;

use super::config::{Method, RequestConfig};
use super::error::RequestError;
use super::response::{Response, Timing};

pub fn resolve_username(config: &RequestConfig) -> Option<String> {
    config
        .username
        .clone()
        .or_else(|| env::var("RUSTCURL_USER").ok())
}

pub fn resolve_password(config: &RequestConfig) -> Option<String> {
    config
        .password
        .clone()
        .or_else(|| env::var("RUSTCURL_PASSWORD").ok())
}

pub fn resolve_proxy(config: &RequestConfig) -> Option<String> {
    config
        .proxy
        .clone()
        .or_else(|| env::var("HTTPS_PROXY").ok())
        .or_else(|| env::var("HTTP_PROXY").ok())
        .or_else(|| env::var("ALL_PROXY").ok())
        .or_else(|| env::var("https_proxy").ok())
        .or_else(|| env::var("http_proxy").ok())
        .or_else(|| env::var("all_proxy").ok())
}

pub fn resolve_noproxy(config: &RequestConfig) -> Option<String> {
    config
        .noproxy
        .clone()
        .or_else(|| env::var("NO_PROXY").ok())
        .or_else(|| env::var("no_proxy").ok())
}

fn apply_method(easy: &mut Easy, config: &RequestConfig) -> Result<(), RequestError> {
    match &config.method {
        Method::Get => {}
        Method::Post => {
            easy.post(true)?;
        }
        Method::Put => {
            easy.put(true)?;
        }
        Method::Head => {
            easy.nobody(true)?;
        }
        method => {
            easy.custom_request(method.as_str())?;
        }
    }
    if config.head_only && config.method != Method::Head {
        easy.nobody(true)?;
    }
    Ok(())
}

fn apply_auth(easy: &mut Easy, config: &RequestConfig) -> Result<(), RequestError> {
    if config.negotiate {
        let mut auth = Auth::new();
        auth.gssnegotiate(true);
        easy.http_auth(&auth)?;
        easy.username(&resolve_username(config).unwrap_or_default())?;
        easy.password(&resolve_password(config).unwrap_or_default())?;
    } else if config.ntlm {
        let mut auth = Auth::new();
        auth.ntlm(true);
        easy.http_auth(&auth)?;
        easy.username(&resolve_username(config).unwrap_or_default())?;
        easy.password(&resolve_password(config).unwrap_or_default())?;
    } else if resolve_username(config).is_some() {
        easy.username(&resolve_username(config).unwrap())?;
        if let Some(ref pass) = resolve_password(config) {
            easy.password(pass)?;
        }
    }
    Ok(())
}

fn build_headers(config: &RequestConfig) -> Result<List, RequestError> {
    let mut list = List::new();
    for h in &config.headers {
        list.append(h)?;
    }
    if let Some(ref token) = config.bearer {
        list.append(&format!("Authorization: Bearer {token}"))?;
    }
    Ok(list)
}

fn apply_options(easy: &mut Easy, config: &RequestConfig) -> Result<(), RequestError> {
    if config.insecure {
        easy.ssl_verify_peer(false)?;
        easy.ssl_verify_host(false)?;
    }
    if let Some(ref path) = config.cacert {
        easy.cainfo(path)?;
    }
    if let Some(ref proxy_url) = resolve_proxy(config) {
        easy.proxy(proxy_url)?;
    }
    if let Some(ref noproxy_hosts) = resolve_noproxy(config) {
        easy.noproxy(noproxy_hosts)?;
    }
    if config.proxy_negotiate {
        let mut auth = Auth::new();
        auth.gssnegotiate(true);
        easy.proxy_auth(&auth)?;
        easy.proxy_username(&config.proxy_user.as_deref().unwrap_or(":"))?;
    } else if config.proxy_ntlm {
        let mut auth = Auth::new();
        auth.ntlm(true);
        easy.proxy_auth(&auth)?;
    }
    if let Some(ref pu) = config.proxy_user {
        easy.proxy_username(pu)?;
    }
    if let Some(ref pp) = config.proxy_password {
        easy.proxy_password(pp)?;
    }
    if config.proxy_insecure {
        easy.proxy_ssl_verify_peer(false)?;
        easy.proxy_ssl_verify_host(false)?;
    }
    if let Some(ref path) = config.proxy_cacert {
        easy.proxy_cainfo(path)?;
    }
    if let Some(d) = config.connect_timeout {
        easy.connect_timeout(d)?;
    }
    if let Some(d) = config.max_time {
        easy.timeout(d)?;
    }
    if let Some(ref path) = config.cookie {
        easy.cookie_file(path)?;
    }
    if let Some(ref path) = config.cookie_jar {
        easy.cookie_jar(path)?;
    }
    if config.compressed {
        easy.accept_encoding("")?;
    }
    let user_agent = config.user_agent.as_deref().unwrap_or(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"
    );
    easy.useragent(user_agent)?;
    if let Some(n) = config.max_redirs {
        easy.max_redirections(n)?;
    }
    if config.ssl_no_revoke {
        let mut ssl_opts = SslOpt::new();
        ssl_opts.no_revoke(true);
        easy.ssl_options(&ssl_opts)?;
        easy.proxy_ssl_options(&ssl_opts)?;
    }
    if config.verbose {
        easy.verbose(true)?;
    }
    Ok(())
}

fn apply_resolve(easy: &mut Easy, config: &RequestConfig) -> Result<(), RequestError> {
    if !config.resolve.is_empty() {
        let mut list = List::new();
        for entry in &config.resolve {
            list.append(entry)?;
        }
        easy.resolve(list)?;
    }
    Ok(())
}

fn collect_timing(easy: &mut Easy) -> Timing {
    Timing {
        dns: easy.namelookup_time().unwrap_or(Duration::ZERO),
        connect: easy.connect_time().unwrap_or(Duration::ZERO),
        tls: easy.appconnect_time().unwrap_or(Duration::ZERO),
        starttransfer: easy.starttransfer_time().unwrap_or(Duration::ZERO),
        total: easy.total_time().unwrap_or(Duration::ZERO),
        redirect: easy.redirect_time().unwrap_or(Duration::ZERO),
    }
}

pub fn perform_request(config: &RequestConfig) -> Result<Response, RequestError> {
    let mut easy = Easy::new();
    easy.url(&config.url)?;
    easy.follow_location(true)?;

    apply_method(&mut easy, config)?;
    apply_auth(&mut easy, config)?;

    let header_list = build_headers(config)?;
    easy.http_headers(header_list)?;

    if let Some(ref data) = config.data {
        easy.post_field_size(data.len() as u64)?;
        easy.post_fields_copy(data.as_bytes())?;
    }

    apply_options(&mut easy, config)?;
    apply_resolve(&mut easy, config)?;

    let mut headers: Vec<String> = Vec::new();
    let mut body: Vec<u8> = Vec::new();

    {
        let mut transfer = easy.transfer();

        transfer.header_function(|data| {
            if let Ok(header) = std::str::from_utf8(data) {
                let trimmed = header.trim();
                if !trimmed.is_empty() {
                    headers.push(trimmed.to_string());
                }
            }
            true
        })?;

        transfer.write_function(|data| {
            body.extend_from_slice(data);
            Ok(data.len())
        })?;

        transfer.perform()?;
    }

    let status_code = easy.response_code()?;

    let timing = if config.show_timing {
        Some(collect_timing(&mut easy))
    } else {
        None
    };

    if let Some(ref path) = config.output {
        fs::write(path, &body)?;
        return Ok(Response {
            status_code,
            headers,
            body: Vec::new(),
            timing,
        });
    }

    Ok(Response {
        status_code,
        headers,
        body,
        timing,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn username_from_config() {
        let cfg = RequestConfig::new("https://x.com").username("cfguser");
        assert_eq!(resolve_username(&cfg).unwrap(), "cfguser");
    }

    #[test]
    fn username_from_env() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe { env::set_var("RUSTCURL_USER", "envuser") };
        let cfg = RequestConfig::new("https://x.com");
        let result = resolve_username(&cfg);
        unsafe { env::remove_var("RUSTCURL_USER") };
        assert_eq!(result.unwrap(), "envuser");
    }

    #[test]
    fn username_config_takes_priority() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe { env::set_var("RUSTCURL_USER", "envuser") };
        let cfg = RequestConfig::new("https://x.com").username("cfguser");
        let result = resolve_username(&cfg);
        unsafe { env::remove_var("RUSTCURL_USER") };
        assert_eq!(result.unwrap(), "cfguser");
    }

    #[test]
    fn username_none_when_unset() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe { env::remove_var("RUSTCURL_USER") };
        let cfg = RequestConfig::new("https://x.com");
        assert!(resolve_username(&cfg).is_none());
    }

    #[test]
    fn password_from_config() {
        let cfg = RequestConfig::new("https://x.com").password("cfgpass");
        assert_eq!(resolve_password(&cfg).unwrap(), "cfgpass");
    }

    #[test]
    fn password_from_env() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe { env::set_var("RUSTCURL_PASSWORD", "envpass") };
        let cfg = RequestConfig::new("https://x.com");
        let result = resolve_password(&cfg);
        unsafe { env::remove_var("RUSTCURL_PASSWORD") };
        assert_eq!(result.unwrap(), "envpass");
    }

    #[test]
    fn password_config_takes_priority() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe { env::set_var("RUSTCURL_PASSWORD", "envpass") };
        let cfg = RequestConfig::new("https://x.com").password("cfgpass");
        let result = resolve_password(&cfg);
        unsafe { env::remove_var("RUSTCURL_PASSWORD") };
        assert_eq!(result.unwrap(), "cfgpass");
    }

    #[test]
    fn proxy_from_config() {
        let cfg = RequestConfig::new("https://x.com").proxy("http://myproxy:3128");
        assert_eq!(resolve_proxy(&cfg).unwrap(), "http://myproxy:3128");
    }

    #[test]
    fn proxy_from_env() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("HTTPS_PROXY");
            env::remove_var("HTTP_PROXY");
            env::remove_var("ALL_PROXY");
            env::remove_var("https_proxy");
            env::remove_var("http_proxy");
            env::remove_var("all_proxy");
            env::set_var("HTTPS_PROXY", "http://envproxy:3128");
        };
        let cfg = RequestConfig::new("https://x.com");
        let result = resolve_proxy(&cfg);
        unsafe { env::remove_var("HTTPS_PROXY") };
        assert_eq!(result.unwrap(), "http://envproxy:3128");
    }

    #[test]
    fn proxy_config_takes_priority() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe { env::set_var("HTTPS_PROXY", "http://envproxy:3128") };
        let cfg = RequestConfig::new("https://x.com").proxy("http://cfgproxy:3128");
        let result = resolve_proxy(&cfg);
        unsafe { env::remove_var("HTTPS_PROXY") };
        assert_eq!(result.unwrap(), "http://cfgproxy:3128");
    }

    #[test]
    fn noproxy_from_config() {
        let cfg = RequestConfig::new("https://x.com").noproxy("localhost");
        assert_eq!(resolve_noproxy(&cfg).unwrap(), "localhost");
    }

    #[test]
    fn noproxy_from_env() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("NO_PROXY");
            env::remove_var("no_proxy");
            env::set_var("NO_PROXY", "127.0.0.1");
        };
        let cfg = RequestConfig::new("https://x.com");
        let result = resolve_noproxy(&cfg);
        unsafe { env::remove_var("NO_PROXY") };
        assert_eq!(result.unwrap(), "127.0.0.1");
    }

    #[test]
    fn noproxy_config_takes_priority() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe { env::set_var("NO_PROXY", "envhost") };
        let cfg = RequestConfig::new("https://x.com").noproxy("cfghost");
        let result = resolve_noproxy(&cfg);
        unsafe { env::remove_var("NO_PROXY") };
        assert_eq!(result.unwrap(), "cfghost");
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_get_request() {
        let cfg = RequestConfig::new("https://httpbin.org/get");
        let resp = perform_request(&cfg).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(!resp.headers.is_empty());
        assert!(resp.body_string().contains("httpbin.org"));
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_insecure_request() {
        let cfg = RequestConfig::new("https://httpbin.org/get").insecure(true);
        let resp = perform_request(&cfg).unwrap();
        assert_eq!(resp.status_code, 200);
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_headers_captured() {
        let cfg = RequestConfig::new("https://httpbin.org/get");
        let resp = perform_request(&cfg).unwrap();
        assert!(resp.get_header("content-type").is_some());
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_follow_redirect() {
        let cfg = RequestConfig::new("http://httpbin.org/redirect/1");
        let resp = perform_request(&cfg).unwrap();
        assert_eq!(resp.status_code, 200);
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_post_with_data() {
        let cfg = RequestConfig::new("https://httpbin.org/post")
            .method(Method::Post)
            .data("{\"key\":\"value\"}")
            .header("Content-Type: application/json");
        let resp = perform_request(&cfg).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(resp.body_string().contains("key"));
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_head_request() {
        let cfg = RequestConfig::new("https://httpbin.org/get").head_only(true);
        let resp = perform_request(&cfg).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(resp.body.is_empty());
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_timing() {
        let cfg = RequestConfig::new("https://httpbin.org/get").show_timing(true);
        let resp = perform_request(&cfg).unwrap();
        assert!(resp.timing.is_some());
        let t = resp.timing.unwrap();
        assert!(t.total > Duration::ZERO);
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_compressed() {
        let cfg = RequestConfig::new("https://httpbin.org/gzip").compressed(true);
        let resp = perform_request(&cfg).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(resp.body_string().contains("gzipped"));
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_custom_user_agent() {
        let cfg = RequestConfig::new("https://httpbin.org/user-agent")
            .user_agent("rustcurl-test/0.1");
        let resp = perform_request(&cfg).unwrap();
        assert!(resp.body_string().contains("rustcurl-test/0.1"));
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_output_to_file() {
        let path = "/tmp/rustcurl_test_output.html";
        let cfg = RequestConfig::new("https://httpbin.org/html").output(path);
        let resp = perform_request(&cfg).unwrap();
        assert!(resp.body.is_empty());
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("html"));
        let _ = fs::remove_file(path);
    }

    #[test]
    #[ignore = "requires network access"]
    fn integration_max_redirs() {
        let cfg = RequestConfig::new("https://httpbin.org/redirect/3").max_redirs(5);
        let resp = perform_request(&cfg).unwrap();
        assert_eq!(resp.status_code, 200);
    }
}
