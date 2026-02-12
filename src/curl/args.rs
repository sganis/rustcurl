// src/curl/args.rs

use super::config::{Method, RequestConfig};

pub fn parse_credentials(input: &str) -> (String, Option<String>) {
    match input.split_once(':') {
        Some((user, pass)) => (user.to_string(), Some(pass.to_string())),
        None => (input.to_string(), None),
    }
}

pub fn print_usage() {
    eprintln!("Usage: rustcurl [OPTIONS] <URL>");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -X, --request <METHOD>   HTTP method (GET, POST, PUT, DELETE, HEAD, PATCH, OPTIONS)");
    eprintln!("  -H, --header <HEADER>    Add header (repeatable), e.g. -H \"Content-Type: application/json\"");
    eprintln!("  -d, --data <DATA>        Request body data (auto-sets POST if no -X given)");
    eprintln!("  -o, --output <FILE>      Write response body to file");
    eprintln!("  -I, --head               Send HEAD request (show headers only)");
    eprintln!("  -s, --silent             Silent mode (only output body)");
    eprintln!("  -A, --user-agent <STR>   Set User-Agent header");
    eprintln!("  -b, --cookie <FILE>      Read cookies from file");
    eprintln!("  -c, --cookie-jar <FILE>  Write cookies to file after request");
    eprintln!("  --negotiate              Enable Kerberos/SPNEGO authentication");
    eprintln!("  --ntlm                   Enable NTLM authentication");
    eprintln!("  -k, --insecure           Ignore SSL certificate verification");
    eprintln!("  --cacert <PATH>          Path to CA certificate bundle");
    eprintln!("  -u, --user <USER:PASS>   Credentials (user:password)");
    eprintln!("  --bearer <TOKEN>         Bearer token authentication");
    eprintln!("  -x, --proxy <URL>        Proxy URL");
    eprintln!("  --proxy-user <USER:PASS> Proxy credentials");
    eprintln!("  --proxy-negotiate        Enable Kerberos/SPNEGO proxy authentication");
    eprintln!("  --proxy-ntlm             Enable NTLM proxy authentication");
    eprintln!("  --proxy-insecure         Skip SSL verification for proxy connection");
    eprintln!("  --proxy-cacert <PATH>    CA certificate for proxy SSL verification");
    eprintln!("  --noproxy <HOSTS>        Comma-separated list of hosts to bypass proxy");
    eprintln!("  --connect-timeout <SECS> Connection timeout in seconds");
    eprintln!("  --max-time <SECS>        Maximum total time in seconds");
    eprintln!("  --max-redirs <N>         Maximum number of redirects");
    eprintln!("  -L, --location           Follow redirects (always enabled)");
    eprintln!("  --ssl-no-revoke          Disable certificate revocation checks");
    eprintln!("  --compressed             Request compressed response");
    eprintln!("  --timing                 Show timing information");
    eprintln!("  --resolve <H:P:A>        Resolve host:port to address (repeatable)");
    eprintln!("  -v, --verbose            Verbose output");
    eprintln!("  -h, --help               Show this help");
    eprintln!();
    eprintln!("Environment variables:");
    eprintln!("  RUSTCURL_USER            Username fallback");
    eprintln!("  RUSTCURL_PASSWORD        Password fallback");
    eprintln!("  HTTPS_PROXY              HTTPS proxy URL");
    eprintln!("  HTTP_PROXY               HTTP proxy URL");
    eprintln!("  ALL_PROXY                Proxy for all protocols");
    eprintln!("  NO_PROXY                 Hosts to bypass proxy");
}

fn parse_method(s: &str) -> Method {
    match s.to_uppercase().as_str() {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        "HEAD" => Method::Head,
        "PATCH" => Method::Patch,
        "OPTIONS" => Method::Options,
        other => Method::Custom(other.to_string()),
    }
}

fn next_arg<'a>(args: &'a [String], i: &mut usize, name: &str) -> Result<&'a str, String> {
    *i += 1;
    args.get(*i)
        .map(|s| s.as_str())
        .ok_or_else(|| format!("{name} requires a value"))
}

fn parse_seconds(s: &str, name: &str) -> Result<std::time::Duration, String> {
    let secs: u64 = s
        .parse()
        .map_err(|_| format!("{name} requires a number of seconds"))?;
    Ok(std::time::Duration::from_secs(secs))
}

fn parse_u32(s: &str, name: &str) -> Result<u32, String> {
    s.parse()
        .map_err(|_| format!("{name} requires a positive integer"))
}

pub fn parse_args(args: &[String]) -> Result<RequestConfig, String> {
    if args.is_empty() {
        return Err("no arguments provided".to_string());
    }

    let mut url = None;
    let mut method = None;
    let mut negotiate = false;
    let mut insecure = false;
    let mut cacert = None;
    let mut username = None;
    let mut password = None;
    let mut proxy = None;
    let mut verbose = false;
    let mut headers: Vec<String> = Vec::new();
    let mut data = None;
    let mut connect_timeout = None;
    let mut max_time = None;
    let mut output = None;
    let mut head_only = false;
    let mut ntlm = false;
    let mut proxy_user = None;
    let mut proxy_password = None;
    let mut noproxy = None;
    let mut cookie = None;
    let mut cookie_jar = None;
    let mut bearer = None;
    let mut compressed = false;
    let mut show_timing = false;
    let mut user_agent = None;
    let mut silent = false;
    let mut max_redirs = None;
    let mut resolve: Vec<String> = Vec::new();
    let mut proxy_negotiate = false;
    let mut proxy_ntlm = false;
    let mut proxy_insecure = false;
    let mut proxy_cacert = None;
    let mut ssl_no_revoke = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "-X" | "--request" => {
                let val = next_arg(args, &mut i, "-X")?;
                method = Some(parse_method(val));
            }
            "-H" | "--header" => {
                let val = next_arg(args, &mut i, "-H")?;
                headers.push(val.to_string());
            }
            "-d" | "--data" => {
                let val = next_arg(args, &mut i, "-d")?;
                data = Some(val.to_string());
            }
            "-o" | "--output" => {
                let val = next_arg(args, &mut i, "-o")?;
                output = Some(val.to_string());
            }
            "-I" | "--head" => head_only = true,
            "-s" | "--silent" => silent = true,
            "-A" | "--user-agent" => {
                let val = next_arg(args, &mut i, "-A")?;
                user_agent = Some(val.to_string());
            }
            "-b" | "--cookie" => {
                let val = next_arg(args, &mut i, "-b")?;
                cookie = Some(val.to_string());
            }
            "-c" | "--cookie-jar" => {
                let val = next_arg(args, &mut i, "-c")?;
                cookie_jar = Some(val.to_string());
            }
            "--negotiate" => negotiate = true,
            "--ntlm" => ntlm = true,
            "-k" | "--insecure" => insecure = true,
            "--cacert" => {
                let val = next_arg(args, &mut i, "--cacert")?;
                cacert = Some(val.to_string());
            }
            "-u" | "--user" => {
                let val = next_arg(args, &mut i, "-u")?;
                let (u, p) = parse_credentials(val);
                username = Some(u);
                password = p;
            }
            "--bearer" => {
                let val = next_arg(args, &mut i, "--bearer")?;
                bearer = Some(val.to_string());
            }
            "-x" | "--proxy" => {
                let val = next_arg(args, &mut i, "-x")?;
                proxy = Some(val.to_string());
            }
            "--proxy-user" => {
                let val = next_arg(args, &mut i, "--proxy-user")?;
                let (u, p) = parse_credentials(val);
                proxy_user = Some(u);
                proxy_password = p;
            }
            "--proxy-negotiate" => proxy_negotiate = true,
            "--proxy-ntlm" => proxy_ntlm = true,
            "-L" | "--location" => {} // follow redirects (always on)
            "--ssl-no-revoke" => ssl_no_revoke = true,
            "--proxy-insecure" => proxy_insecure = true,
            "--proxy-cacert" => {
                let val = next_arg(args, &mut i, "--proxy-cacert")?;
                proxy_cacert = Some(val.to_string());
            }
            "--noproxy" => {
                let val = next_arg(args, &mut i, "--noproxy")?;
                noproxy = Some(val.to_string());
            }
            "--connect-timeout" => {
                let val = next_arg(args, &mut i, "--connect-timeout")?;
                connect_timeout = Some(parse_seconds(val, "--connect-timeout")?);
            }
            "--max-time" => {
                let val = next_arg(args, &mut i, "--max-time")?;
                max_time = Some(parse_seconds(val, "--max-time")?);
            }
            "--max-redirs" => {
                let val = next_arg(args, &mut i, "--max-redirs")?;
                max_redirs = Some(parse_u32(val, "--max-redirs")?);
            }
            "--compressed" => compressed = true,
            "--timing" => show_timing = true,
            "--resolve" => {
                let val = next_arg(args, &mut i, "--resolve")?;
                resolve.push(val.to_string());
            }
            "-v" | "--verbose" => verbose = true,
            arg if arg.starts_with('-') => {
                return Err(format!("unknown option: {arg}"));
            }
            arg => {
                url = Some(arg.to_string());
            }
        }
        i += 1;
    }

    let url = url.ok_or("URL is required")?;

    // Auto-set POST when data provided without explicit method (like curl)
    if data.is_some() && method.is_none() {
        method = Some(Method::Post);
    }

    // -I sets HEAD method
    if head_only && method.is_none() {
        method = Some(Method::Head);
    }

    let mut config = RequestConfig::new(&url)
        .method(method.unwrap_or(Method::Get))
        .negotiate(negotiate)
        .insecure(insecure)
        .verbose(verbose)
        .head_only(head_only)
        .ntlm(ntlm)
        .compressed(compressed)
        .show_timing(show_timing)
        .silent(silent)
        .proxy_negotiate(proxy_negotiate)
        .proxy_ntlm(proxy_ntlm)
        .proxy_insecure(proxy_insecure)
        .ssl_no_revoke(ssl_no_revoke);

    config.headers = headers;
    config.resolve = resolve;

    if let Some(path) = cacert {
        config = config.cacert(&path);
    }
    if let Some(u) = username {
        config = config.username(&u);
    }
    if let Some(p) = password {
        config = config.password(&p);
    }
    if let Some(px) = proxy {
        config = config.proxy(&px);
    }
    if let Some(d) = data {
        config = config.data(&d);
    }
    if let Some(ct) = connect_timeout {
        config = config.connect_timeout(ct);
    }
    if let Some(mt) = max_time {
        config = config.max_time(mt);
    }
    if let Some(o) = output {
        config = config.output(&o);
    }
    if let Some(pu) = proxy_user {
        config = config.proxy_user(&pu);
    }
    if let Some(pp) = proxy_password {
        config = config.proxy_password(&pp);
    }
    if let Some(np) = noproxy {
        config = config.noproxy(&np);
    }
    if let Some(c) = cookie {
        config = config.cookie(&c);
    }
    if let Some(cj) = cookie_jar {
        config = config.cookie_jar(&cj);
    }
    if let Some(b) = bearer {
        config = config.bearer(&b);
    }
    if let Some(ua) = user_agent {
        config = config.user_agent(&ua);
    }
    if let Some(mr) = max_redirs {
        config = config.max_redirs(mr);
    }
    if let Some(pc) = proxy_cacert {
        config = config.proxy_cacert(&pc);
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn args(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn credentials_with_password() {
        let (user, pass) = parse_credentials("admin:secret");
        assert_eq!(user, "admin");
        assert_eq!(pass.unwrap(), "secret");
    }

    #[test]
    fn credentials_password_containing_colon() {
        let (user, pass) = parse_credentials("admin:s:e:c");
        assert_eq!(user, "admin");
        assert_eq!(pass.unwrap(), "s:e:c");
    }

    #[test]
    fn credentials_no_password() {
        let (user, pass) = parse_credentials("admin");
        assert_eq!(user, "admin");
        assert!(pass.is_none());
    }

    #[test]
    fn credentials_empty_password() {
        let (user, pass) = parse_credentials("admin:");
        assert_eq!(user, "admin");
        assert_eq!(pass.unwrap(), "");
    }

    #[test]
    fn url_only() {
        let cfg = parse_args(&args(&["https://example.com"])).unwrap();
        assert_eq!(cfg.url, "https://example.com");
        assert_eq!(cfg.method, Method::Get);
        assert!(!cfg.negotiate);
        assert!(!cfg.insecure);
        assert!(!cfg.verbose);
    }

    #[test]
    fn all_original_flags() {
        let cfg = parse_args(&args(&[
            "--negotiate", "-k", "--cacert", "/tmp/ca.pem", "-u", "admin:pass",
            "-x", "http://proxy:8080", "-v", "https://example.com",
        ]))
        .unwrap();

        assert_eq!(cfg.url, "https://example.com");
        assert!(cfg.negotiate);
        assert!(cfg.insecure);
        assert_eq!(cfg.cacert.as_deref(), Some("/tmp/ca.pem"));
        assert_eq!(cfg.username.as_deref(), Some("admin"));
        assert_eq!(cfg.password.as_deref(), Some("pass"));
        assert_eq!(cfg.proxy.as_deref(), Some("http://proxy:8080"));
        assert!(cfg.verbose);
    }

    #[test]
    fn long_form_flags() {
        let cfg = parse_args(&args(&[
            "--insecure", "--user", "user:pw", "--proxy", "http://p:80",
            "--verbose", "https://test.com",
        ]))
        .unwrap();

        assert!(cfg.insecure);
        assert_eq!(cfg.username.as_deref(), Some("user"));
        assert_eq!(cfg.password.as_deref(), Some("pw"));
        assert_eq!(cfg.proxy.as_deref(), Some("http://p:80"));
        assert!(cfg.verbose);
    }

    #[test]
    fn url_before_flags() {
        let cfg = parse_args(&args(&["https://first.com", "--insecure"])).unwrap();
        assert_eq!(cfg.url, "https://first.com");
        assert!(cfg.insecure);
    }

    #[test]
    fn empty_is_error() {
        assert!(parse_args(&[]).is_err());
    }

    #[test]
    fn no_url_is_error() {
        assert!(parse_args(&args(&["--negotiate"])).is_err());
    }

    #[test]
    fn unknown_flag_is_error() {
        assert!(parse_args(&args(&["--bogus", "https://x.com"])).is_err());
    }

    #[test]
    fn cacert_missing_path_is_error() {
        assert!(parse_args(&args(&["--cacert"])).is_err());
    }

    #[test]
    fn user_missing_value_is_error() {
        assert!(parse_args(&args(&["-u"])).is_err());
    }

    #[test]
    fn proxy_missing_value_is_error() {
        assert!(parse_args(&args(&["-x"])).is_err());
    }

    #[test]
    fn explicit_method() {
        let cfg = parse_args(&args(&["-X", "PUT", "https://x.com"])).unwrap();
        assert_eq!(cfg.method, Method::Put);
    }

    #[test]
    fn custom_method() {
        let cfg = parse_args(&args(&["-X", "PURGE", "https://x.com"])).unwrap();
        assert_eq!(cfg.method, Method::Custom("PURGE".into()));
    }

    #[test]
    fn data_auto_sets_post() {
        let cfg = parse_args(&args(&["-d", "{}", "https://x.com"])).unwrap();
        assert_eq!(cfg.method, Method::Post);
        assert_eq!(cfg.data.as_deref(), Some("{}"));
    }

    #[test]
    fn data_with_explicit_method_keeps_method() {
        let cfg = parse_args(&args(&["-X", "PUT", "-d", "body", "https://x.com"])).unwrap();
        assert_eq!(cfg.method, Method::Put);
        assert_eq!(cfg.data.as_deref(), Some("body"));
    }

    #[test]
    fn head_flag_sets_head_method() {
        let cfg = parse_args(&args(&["-I", "https://x.com"])).unwrap();
        assert_eq!(cfg.method, Method::Head);
        assert!(cfg.head_only);
    }

    #[test]
    fn head_long_form() {
        let cfg = parse_args(&args(&["--head", "https://x.com"])).unwrap();
        assert_eq!(cfg.method, Method::Head);
    }

    #[test]
    fn multiple_headers() {
        let cfg = parse_args(&args(&[
            "-H", "Accept: text/html",
            "-H", "X-Custom: foo",
            "https://x.com",
        ]))
        .unwrap();
        assert_eq!(cfg.headers.len(), 2);
        assert_eq!(cfg.headers[0], "Accept: text/html");
        assert_eq!(cfg.headers[1], "X-Custom: foo");
    }

    #[test]
    fn connect_timeout_and_max_time() {
        let cfg = parse_args(&args(&[
            "--connect-timeout", "10", "--max-time", "30", "https://x.com",
        ]))
        .unwrap();
        assert_eq!(cfg.connect_timeout, Some(Duration::from_secs(10)));
        assert_eq!(cfg.max_time, Some(Duration::from_secs(30)));
    }

    #[test]
    fn connect_timeout_bad_value() {
        assert!(parse_args(&args(&["--connect-timeout", "abc", "https://x.com"])).is_err());
    }

    #[test]
    fn output_flag() {
        let cfg = parse_args(&args(&["-o", "/tmp/out.html", "https://x.com"])).unwrap();
        assert_eq!(cfg.output.as_deref(), Some("/tmp/out.html"));
    }

    #[test]
    fn silent_flag() {
        let cfg = parse_args(&args(&["-s", "https://x.com"])).unwrap();
        assert!(cfg.silent);
    }

    #[test]
    fn ntlm_flag() {
        let cfg = parse_args(&args(&["--ntlm", "-u", "user:pass", "https://x.com"])).unwrap();
        assert!(cfg.ntlm);
    }

    #[test]
    fn proxy_user_flag() {
        let cfg = parse_args(&args(&["--proxy-user", "puser:ppass", "https://x.com"])).unwrap();
        assert_eq!(cfg.proxy_user.as_deref(), Some("puser"));
        assert_eq!(cfg.proxy_password.as_deref(), Some("ppass"));
    }

    #[test]
    fn noproxy_flag() {
        let cfg = parse_args(&args(&["--noproxy", "localhost,127.0.0.1", "https://x.com"])).unwrap();
        assert_eq!(cfg.noproxy.as_deref(), Some("localhost,127.0.0.1"));
    }

    #[test]
    fn cookie_flags() {
        let cfg = parse_args(&args(&[
            "-b", "/tmp/cookies", "-c", "/tmp/jar", "https://x.com",
        ]))
        .unwrap();
        assert_eq!(cfg.cookie.as_deref(), Some("/tmp/cookies"));
        assert_eq!(cfg.cookie_jar.as_deref(), Some("/tmp/jar"));
    }

    #[test]
    fn bearer_flag() {
        let cfg = parse_args(&args(&["--bearer", "tok123", "https://x.com"])).unwrap();
        assert_eq!(cfg.bearer.as_deref(), Some("tok123"));
    }

    #[test]
    fn compressed_flag() {
        let cfg = parse_args(&args(&["--compressed", "https://x.com"])).unwrap();
        assert!(cfg.compressed);
    }

    #[test]
    fn timing_flag() {
        let cfg = parse_args(&args(&["--timing", "https://x.com"])).unwrap();
        assert!(cfg.show_timing);
    }

    #[test]
    fn user_agent_flag() {
        let cfg = parse_args(&args(&["-A", "myagent/1.0", "https://x.com"])).unwrap();
        assert_eq!(cfg.user_agent.as_deref(), Some("myagent/1.0"));
    }

    #[test]
    fn max_redirs_flag() {
        let cfg = parse_args(&args(&["--max-redirs", "3", "https://x.com"])).unwrap();
        assert_eq!(cfg.max_redirs, Some(3));
    }

    #[test]
    fn max_redirs_bad_value() {
        assert!(parse_args(&args(&["--max-redirs", "abc", "https://x.com"])).is_err());
    }

    #[test]
    fn resolve_flag_repeatable() {
        let cfg = parse_args(&args(&[
            "--resolve", "a.com:443:1.1.1.1",
            "--resolve", "b.com:80:2.2.2.2",
            "https://x.com",
        ]))
        .unwrap();
        assert_eq!(cfg.resolve.len(), 2);
        assert_eq!(cfg.resolve[0], "a.com:443:1.1.1.1");
    }

    #[test]
    fn all_new_flags_combined() {
        let cfg = parse_args(&args(&[
            "-X", "POST",
            "-H", "Content-Type: application/json",
            "-d", "{\"a\":1}",
            "-o", "/tmp/out",
            "-I",
            "-s",
            "-A", "agent/1",
            "-b", "/cookies",
            "-c", "/jar",
            "--ntlm",
            "--bearer", "token",
            "--proxy-user", "pu:pp",
            "--noproxy", "localhost",
            "--connect-timeout", "5",
            "--max-time", "60",
            "--max-redirs", "10",
            "--compressed",
            "--timing",
            "--resolve", "h:443:1.2.3.4",
            "https://example.com",
        ]))
        .unwrap();

        assert_eq!(cfg.method, Method::Post);
        assert_eq!(cfg.headers, vec!["Content-Type: application/json"]);
        assert_eq!(cfg.data.as_deref(), Some("{\"a\":1}"));
        assert_eq!(cfg.output.as_deref(), Some("/tmp/out"));
        assert!(cfg.head_only);
        assert!(cfg.silent);
        assert_eq!(cfg.user_agent.as_deref(), Some("agent/1"));
        assert_eq!(cfg.cookie.as_deref(), Some("/cookies"));
        assert_eq!(cfg.cookie_jar.as_deref(), Some("/jar"));
        assert!(cfg.ntlm);
        assert_eq!(cfg.bearer.as_deref(), Some("token"));
        assert_eq!(cfg.proxy_user.as_deref(), Some("pu"));
        assert_eq!(cfg.proxy_password.as_deref(), Some("pp"));
        assert_eq!(cfg.noproxy.as_deref(), Some("localhost"));
        assert_eq!(cfg.connect_timeout, Some(Duration::from_secs(5)));
        assert_eq!(cfg.max_time, Some(Duration::from_secs(60)));
        assert_eq!(cfg.max_redirs, Some(10));
        assert!(cfg.compressed);
        assert!(cfg.show_timing);
        assert_eq!(cfg.resolve, vec!["h:443:1.2.3.4"]);
    }

    #[test]
    fn proxy_negotiate_flag() {
        let cfg = parse_args(&args(&["--proxy-negotiate", "-x", "http://proxy:8080", "https://x.com"])).unwrap();
        assert!(cfg.proxy_negotiate);
    }

    #[test]
    fn proxy_ntlm_flag() {
        let cfg = parse_args(&args(&["--proxy-ntlm", "-x", "http://proxy:8080", "https://x.com"])).unwrap();
        assert!(cfg.proxy_ntlm);
    }

    #[test]
    fn proxy_insecure_flag() {
        let cfg = parse_args(&args(&["--proxy-insecure", "-x", "http://proxy:8080", "https://x.com"])).unwrap();
        assert!(cfg.proxy_insecure);
    }

    #[test]
    fn proxy_cacert_flag() {
        let cfg = parse_args(&args(&["--proxy-cacert", "/corp-ca.pem", "https://x.com"])).unwrap();
        assert_eq!(cfg.proxy_cacert.as_deref(), Some("/corp-ca.pem"));
    }

    #[test]
    fn location_short_flag() {
        let cfg = parse_args(&args(&["-L", "https://x.com"])).unwrap();
        assert_eq!(cfg.url, "https://x.com");
    }

    #[test]
    fn location_long_flag() {
        let cfg = parse_args(&args(&["--location", "https://x.com"])).unwrap();
        assert_eq!(cfg.url, "https://x.com");
    }

    #[test]
    fn ssl_no_revoke_flag() {
        let cfg = parse_args(&args(&["--ssl-no-revoke", "https://x.com"])).unwrap();
        assert!(cfg.ssl_no_revoke);
    }
}
