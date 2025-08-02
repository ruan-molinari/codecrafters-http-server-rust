use crate::http::{Header, HeaderMap, Method};

pub struct Request {
    pub method: Method,
    pub target: String,
    pub version: String,
    pub headers: HeaderMap,
    pub body: Option<String>,
}

impl Request {
    pub fn new(buf: Vec<u8>) -> Self {
        let request = String::from_utf8(buf).unwrap();
        let mut lines = request.lines().collect::<Vec<_>>().into_iter();
        let mut status_line = lines.next().unwrap().split_whitespace();

        let mut headers = HeaderMap::new();
        while let Some(header_line) = lines.next() {
            if let Some(header) = Header::from_str(header_line) {
                headers.insert(header.key.clone().to_lowercase(), header);
            }
        }
        let body = String::from(lines.collect::<Vec<&str>>().concat());

        let method = Method::from_str(status_line.next().unwrap()).expect("Invalid HTTP method");

        Request {
            method: method,
            target: status_line.next().unwrap().to_string(),
            version: status_line.next().unwrap().to_string(),
            headers: headers,
            body: Some(body),
        }
    }
}
