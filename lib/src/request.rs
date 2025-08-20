use std::borrow::Cow;

use crate::http::{Header, HeaderMap, Method};

pub struct Request<'r> {
    pub method: Method,
    pub target: &'r str,
    pub version: &'r str,
    pub headers: HeaderMap<'r>,
    pub body: Option<&'r str>,
}

impl<'h> Request<'h> {
    pub fn from_buf(buf: &'h str) -> Self {
        let raw = Cow::Borrowed(&buf);
        let mut lines = raw.lines();
        let mut status_line = lines.next().unwrap().split_whitespace();

        let mut headers = HeaderMap::new();
        while let Some(header_line) = lines.next() {
            if let Some((name, value)) = header_line.split_once(":") {
                headers.insert(name.trim(), value.trim());
            }
        }

        let body = raw.get(raw.find("\r\n\r\n").unwrap()..);

        let method = Method::from_str(status_line.next().unwrap()).expect("Invalid HTTP method");

        Request {
            method,
            target: status_line.next().unwrap(),
            version: status_line.next().unwrap(),
            headers,
            body,
        }
    }
}
