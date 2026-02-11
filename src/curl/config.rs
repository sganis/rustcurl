// src/curl/config.rs

use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Patch,
    Options,
    Custom(String),
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Patch => "PATCH",
            Method::Options => "OPTIONS",
            Method::Custom(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub url: String,
    pub method: Method,
    pub negotiate: bool,
    pub insecure: bool,
    pub cacert: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
    pub verbose: bool,
    pub headers: Vec<String>,
    pub data: Option<String>,
    pub connect_timeout: Option<Duration>,
    pub max_time: Option<Duration>,
    pub output: Option<String>,
    pub head_only: bool,
    pub ntlm: bool,
    pub proxy_user: Option<String>,
    pub proxy_password: Option<String>,
    pub noproxy: Option<String>,
    pub cookie: Option<String>,
    pub cookie_jar: Option<String>,
    pub bearer: Option<String>,
    pub compressed: bool,
    pub show_timing: bool,
    pub user_agent: Option<String>,
    pub silent: bool,
    pub max_redirs: Option<u32>,
    pub resolve: Vec<String>,
    pub proxy_negotiate: bool,
    pub proxy_ntlm: bool,
    pub proxy_insecure: bool,
    pub proxy_cacert: Option<String>,
    pub ssl_no_revoke: bool,
}

impl RequestConfig {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: Method::Get,
            negotiate: false,
            insecure: false,
            cacert: None,
            username: None,
            password: None,
            proxy: None,
            verbose: false,
            headers: Vec::new(),
            data: None,
            connect_timeout: None,
            max_time: None,
            output: None,
            head_only: false,
            ntlm: false,
            proxy_user: None,
            proxy_password: None,
            noproxy: None,
            cookie: None,
            cookie_jar: None,
            bearer: None,
            compressed: false,
            show_timing: false,
            user_agent: None,
            silent: false,
            max_redirs: None,
            resolve: Vec::new(),
            proxy_negotiate: false,
            proxy_ntlm: false,
            proxy_insecure: false,
            proxy_cacert: None,
            ssl_no_revoke: false,
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn negotiate(mut self, enable: bool) -> Self {
        self.negotiate = enable;
        self
    }

    pub fn insecure(mut self, enable: bool) -> Self {
        self.insecure = enable;
        self
    }

    pub fn cacert(mut self, path: &str) -> Self {
        self.cacert = Some(path.to_string());
        self
    }

    pub fn username(mut self, user: &str) -> Self {
        self.username = Some(user.to_string());
        self
    }

    pub fn password(mut self, pass: &str) -> Self {
        self.password = Some(pass.to_string());
        self
    }

    pub fn proxy(mut self, proxy_url: &str) -> Self {
        self.proxy = Some(proxy_url.to_string());
        self
    }

    pub fn verbose(mut self, enable: bool) -> Self {
        self.verbose = enable;
        self
    }

    #[allow(dead_code)]
    pub fn header(mut self, h: &str) -> Self {
        self.headers.push(h.to_string());
        self
    }

    pub fn data(mut self, d: &str) -> Self {
        self.data = Some(d.to_string());
        self
    }

    pub fn connect_timeout(mut self, d: Duration) -> Self {
        self.connect_timeout = Some(d);
        self
    }

    pub fn max_time(mut self, d: Duration) -> Self {
        self.max_time = Some(d);
        self
    }

    pub fn output(mut self, path: &str) -> Self {
        self.output = Some(path.to_string());
        self
    }

    pub fn head_only(mut self, enable: bool) -> Self {
        self.head_only = enable;
        self
    }

    pub fn ntlm(mut self, enable: bool) -> Self {
        self.ntlm = enable;
        self
    }

    pub fn proxy_user(mut self, user: &str) -> Self {
        self.proxy_user = Some(user.to_string());
        self
    }

    pub fn proxy_password(mut self, pass: &str) -> Self {
        self.proxy_password = Some(pass.to_string());
        self
    }

    pub fn noproxy(mut self, hosts: &str) -> Self {
        self.noproxy = Some(hosts.to_string());
        self
    }

    pub fn cookie(mut self, path: &str) -> Self {
        self.cookie = Some(path.to_string());
        self
    }

    pub fn cookie_jar(mut self, path: &str) -> Self {
        self.cookie_jar = Some(path.to_string());
        self
    }

    pub fn bearer(mut self, token: &str) -> Self {
        self.bearer = Some(token.to_string());
        self
    }

    pub fn compressed(mut self, enable: bool) -> Self {
        self.compressed = enable;
        self
    }

    pub fn show_timing(mut self, enable: bool) -> Self {
        self.show_timing = enable;
        self
    }

    pub fn user_agent(mut self, ua: &str) -> Self {
        self.user_agent = Some(ua.to_string());
        self
    }

    pub fn silent(mut self, enable: bool) -> Self {
        self.silent = enable;
        self
    }

    pub fn max_redirs(mut self, n: u32) -> Self {
        self.max_redirs = Some(n);
        self
    }

    #[allow(dead_code)]
    pub fn add_resolve(mut self, entry: &str) -> Self {
        self.resolve.push(entry.to_string());
        self
    }

    pub fn proxy_negotiate(mut self, enable: bool) -> Self {
        self.proxy_negotiate = enable;
        self
    }

    pub fn proxy_ntlm(mut self, enable: bool) -> Self {
        self.proxy_ntlm = enable;
        self
    }

    pub fn proxy_insecure(mut self, enable: bool) -> Self {
        self.proxy_insecure = enable;
        self
    }

    pub fn proxy_cacert(mut self, path: &str) -> Self {
        self.proxy_cacert = Some(path.to_string());
        self
    }

    pub fn ssl_no_revoke(mut self, enable: bool) -> Self {
        self.ssl_no_revoke = enable;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults() {
        let cfg = RequestConfig::new("https://example.com");
        assert_eq!(cfg.url, "https://example.com");
        assert_eq!(cfg.method, Method::Get);
        assert!(!cfg.negotiate);
        assert!(!cfg.insecure);
        assert!(cfg.cacert.is_none());
        assert!(cfg.username.is_none());
        assert!(cfg.password.is_none());
        assert!(cfg.proxy.is_none());
        assert!(!cfg.verbose);
        assert!(cfg.headers.is_empty());
        assert!(cfg.data.is_none());
        assert!(cfg.connect_timeout.is_none());
        assert!(cfg.max_time.is_none());
        assert!(cfg.output.is_none());
        assert!(!cfg.head_only);
        assert!(!cfg.ntlm);
        assert!(cfg.proxy_user.is_none());
        assert!(cfg.proxy_password.is_none());
        assert!(cfg.noproxy.is_none());
        assert!(cfg.cookie.is_none());
        assert!(cfg.cookie_jar.is_none());
        assert!(cfg.bearer.is_none());
        assert!(!cfg.compressed);
        assert!(!cfg.show_timing);
        assert!(cfg.user_agent.is_none());
        assert!(!cfg.silent);
        assert!(cfg.max_redirs.is_none());
        assert!(cfg.resolve.is_empty());
        assert!(!cfg.proxy_negotiate);
        assert!(!cfg.proxy_ntlm);
        assert!(!cfg.proxy_insecure);
        assert!(cfg.proxy_cacert.is_none());
        assert!(!cfg.ssl_no_revoke);
    }

    #[test]
    fn config_builder_sets_all_fields() {
        let cfg = RequestConfig::new("https://test.com")
            .method(Method::Post)
            .negotiate(true)
            .insecure(true)
            .cacert("/ca.pem")
            .username("admin")
            .password("secret")
            .proxy("http://proxy:8080")
            .verbose(true)
            .header("Content-Type: application/json")
            .data("{\"key\":\"val\"}")
            .connect_timeout(Duration::from_secs(10))
            .max_time(Duration::from_secs(30))
            .output("/tmp/out.html")
            .head_only(true)
            .ntlm(true)
            .proxy_user("puser")
            .proxy_password("ppass")
            .noproxy("localhost,127.0.0.1")
            .cookie("/tmp/cookies")
            .cookie_jar("/tmp/jar")
            .bearer("tok123")
            .compressed(true)
            .show_timing(true)
            .user_agent("rustcurl/0.1")
            .silent(true)
            .max_redirs(5)
            .add_resolve("example.com:443:1.2.3.4")
            .proxy_negotiate(true)
            .proxy_ntlm(true)
            .proxy_insecure(true)
            .proxy_cacert("/proxy-ca.pem")
            .ssl_no_revoke(true);

        assert_eq!(cfg.method, Method::Post);
        assert!(cfg.negotiate);
        assert!(cfg.insecure);
        assert_eq!(cfg.cacert.as_deref(), Some("/ca.pem"));
        assert_eq!(cfg.username.as_deref(), Some("admin"));
        assert_eq!(cfg.password.as_deref(), Some("secret"));
        assert_eq!(cfg.proxy.as_deref(), Some("http://proxy:8080"));
        assert!(cfg.verbose);
        assert_eq!(cfg.headers, vec!["Content-Type: application/json"]);
        assert_eq!(cfg.data.as_deref(), Some("{\"key\":\"val\"}"));
        assert_eq!(cfg.connect_timeout, Some(Duration::from_secs(10)));
        assert_eq!(cfg.max_time, Some(Duration::from_secs(30)));
        assert_eq!(cfg.output.as_deref(), Some("/tmp/out.html"));
        assert!(cfg.head_only);
        assert!(cfg.ntlm);
        assert_eq!(cfg.proxy_user.as_deref(), Some("puser"));
        assert_eq!(cfg.proxy_password.as_deref(), Some("ppass"));
        assert_eq!(cfg.noproxy.as_deref(), Some("localhost,127.0.0.1"));
        assert_eq!(cfg.cookie.as_deref(), Some("/tmp/cookies"));
        assert_eq!(cfg.cookie_jar.as_deref(), Some("/tmp/jar"));
        assert_eq!(cfg.bearer.as_deref(), Some("tok123"));
        assert!(cfg.compressed);
        assert!(cfg.show_timing);
        assert_eq!(cfg.user_agent.as_deref(), Some("rustcurl/0.1"));
        assert!(cfg.silent);
        assert_eq!(cfg.max_redirs, Some(5));
        assert_eq!(cfg.resolve, vec!["example.com:443:1.2.3.4"]);
        assert!(cfg.proxy_negotiate);
        assert!(cfg.proxy_ntlm);
        assert!(cfg.proxy_insecure);
        assert_eq!(cfg.proxy_cacert.as_deref(), Some("/proxy-ca.pem"));
        assert!(cfg.ssl_no_revoke);
    }

    #[test]
    fn config_clone() {
        let cfg = RequestConfig::new("https://a.com").insecure(true);
        let clone = cfg.clone();
        assert_eq!(clone.url, "https://a.com");
        assert!(clone.insecure);
    }

    #[test]
    fn method_display() {
        assert_eq!(Method::Get.to_string(), "GET");
        assert_eq!(Method::Post.to_string(), "POST");
        assert_eq!(Method::Put.to_string(), "PUT");
        assert_eq!(Method::Delete.to_string(), "DELETE");
        assert_eq!(Method::Head.to_string(), "HEAD");
        assert_eq!(Method::Patch.to_string(), "PATCH");
        assert_eq!(Method::Options.to_string(), "OPTIONS");
        assert_eq!(Method::Custom("PURGE".into()).to_string(), "PURGE");
    }

    #[test]
    fn method_as_str() {
        assert_eq!(Method::Get.as_str(), "GET");
        assert_eq!(Method::Custom("TRACE".into()).as_str(), "TRACE");
    }

    #[test]
    fn multiple_headers() {
        let cfg = RequestConfig::new("https://x.com")
            .header("Accept: text/html")
            .header("X-Custom: foo");
        assert_eq!(cfg.headers.len(), 2);
    }

    #[test]
    fn multiple_resolve_entries() {
        let cfg = RequestConfig::new("https://x.com")
            .add_resolve("a.com:443:1.1.1.1")
            .add_resolve("b.com:80:2.2.2.2");
        assert_eq!(cfg.resolve.len(), 2);
    }
}
