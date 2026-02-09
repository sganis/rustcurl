// src/curl/response.rs

use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Timing {
    pub dns: Duration,
    pub connect: Duration,
    pub tls: Duration,
    pub starttransfer: Duration,
    pub total: Duration,
    pub redirect: Duration,
}

impl fmt::Display for Timing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Timing:")?;
        writeln!(f, "  DNS lookup:    {:>8.3}ms", self.dns.as_secs_f64() * 1000.0)?;
        writeln!(f, "  Connect:       {:>8.3}ms", self.connect.as_secs_f64() * 1000.0)?;
        writeln!(f, "  TLS handshake: {:>8.3}ms", self.tls.as_secs_f64() * 1000.0)?;
        writeln!(f, "  First byte:    {:>8.3}ms", self.starttransfer.as_secs_f64() * 1000.0)?;
        writeln!(f, "  Redirect:      {:>8.3}ms", self.redirect.as_secs_f64() * 1000.0)?;
        write!(f, "  Total:         {:>8.3}ms", self.total.as_secs_f64() * 1000.0)
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u32,
    pub headers: Vec<String>,
    pub body: Vec<u8>,
    pub timing: Option<Timing>,
}

impl Response {
    pub fn body_string(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    pub fn header_map(&self) -> Vec<(String, String)> {
        self.headers
            .iter()
            .filter_map(|h| {
                let (key, val) = h.split_once(':')?;
                Some((key.trim().to_lowercase(), val.trim().to_string()))
            })
            .collect()
    }

    pub fn get_header(&self, name: &str) -> Option<String> {
        let name_lower = name.to_lowercase();
        self.header_map()
            .into_iter()
            .find(|(k, _)| *k == name_lower)
            .map(|(_, v)| v)
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Status: {}", self.status_code)?;
        writeln!(f)?;
        for header in &self.headers {
            writeln!(f, "{header}")?;
        }
        writeln!(f)?;
        write!(f, "{}", self.body_string())?;
        if let Some(ref timing) = self.timing {
            writeln!(f)?;
            writeln!(f)?;
            write!(f, "{timing}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_response(headers: Vec<&str>, body: &[u8]) -> Response {
        Response {
            status_code: 200,
            headers: headers.into_iter().map(String::from).collect(),
            body: body.to_vec(),
            timing: None,
        }
    }

    #[test]
    fn body_valid_utf8() {
        let resp = make_response(vec![], b"Hello, World!");
        assert_eq!(resp.body_string(), "Hello, World!");
    }

    #[test]
    fn body_invalid_utf8() {
        let resp = Response {
            status_code: 200,
            headers: vec![],
            body: vec![0xFF, 0xFE, 0x48, 0x65, 0x6C, 0x6C, 0x6F],
            timing: None,
        };
        assert!(resp.body_string().contains("Hello"));
    }

    #[test]
    fn body_empty() {
        let resp = Response {
            status_code: 204,
            headers: vec![],
            body: vec![],
            timing: None,
        };
        assert_eq!(resp.body_string(), "");
    }

    #[test]
    fn header_map_parsing() {
        let resp = make_response(
            vec![
                "HTTP/1.1 200 OK",
                "Content-Type: text/html",
                "X-Custom: some-value",
            ],
            b"",
        );
        let map = resp.header_map();
        assert!(map.iter().any(|(k, v)| k == "content-type" && v == "text/html"));
        assert!(map.iter().any(|(k, v)| k == "x-custom" && v == "some-value"));
    }

    #[test]
    fn header_map_skips_status_line() {
        let resp = make_response(vec!["HTTP/1.1 200 OK"], b"");
        let map = resp.header_map();
        assert!(map.is_empty() || !map.iter().any(|(k, _)| k == "http/1.1 200 ok"));
    }

    #[test]
    fn get_header_case_insensitive() {
        let resp = make_response(vec!["Content-Type: application/json"], b"");
        assert_eq!(resp.get_header("Content-Type").unwrap(), "application/json");
        assert_eq!(resp.get_header("content-type").unwrap(), "application/json");
        assert_eq!(resp.get_header("CONTENT-TYPE").unwrap(), "application/json");
    }

    #[test]
    fn get_header_missing() {
        let resp = make_response(vec![], b"");
        assert!(resp.get_header("X-Missing").is_none());
    }

    #[test]
    fn display_format() {
        let resp = make_response(vec!["Content-Type: text/plain"], b"hello");
        let output = format!("{resp}");
        assert!(output.contains("Status: 200"));
        assert!(output.contains("Content-Type: text/plain"));
        assert!(output.contains("hello"));
    }

    #[test]
    fn timing_display() {
        let timing = Timing {
            dns: Duration::from_millis(5),
            connect: Duration::from_millis(10),
            tls: Duration::from_millis(20),
            starttransfer: Duration::from_millis(50),
            total: Duration::from_millis(100),
            redirect: Duration::from_millis(0),
        };
        let output = format!("{timing}");
        assert!(output.contains("DNS lookup:"));
        assert!(output.contains("5.000ms"));
        assert!(output.contains("Total:"));
        assert!(output.contains("100.000ms"));
    }

    #[test]
    fn display_with_timing() {
        let resp = Response {
            status_code: 200,
            headers: vec![],
            body: b"ok".to_vec(),
            timing: Some(Timing {
                dns: Duration::from_millis(1),
                connect: Duration::from_millis(2),
                tls: Duration::from_millis(3),
                starttransfer: Duration::from_millis(4),
                total: Duration::from_millis(5),
                redirect: Duration::from_millis(0),
            }),
        };
        let output = format!("{resp}");
        assert!(output.contains("Timing:"));
        assert!(output.contains("Total:"));
    }

    #[test]
    fn display_without_timing() {
        let resp = make_response(vec![], b"body");
        let output = format!("{resp}");
        assert!(!output.contains("Timing:"));
    }
}
