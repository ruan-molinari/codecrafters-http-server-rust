use std::collections::HashMap;

const HTTP_VERSION: &str = "HTTP/1.1";

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }

    pub fn from_str(method: &str) -> Option<HttpMethod> {
        match method {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            "PATCH" => Some(HttpMethod::PATCH),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            _ => None,
        }
    }
}

pub enum HttpStatus {
    OK,
    NotFound,
    InternalServerError,
    BadRequest,
}

impl HttpStatus {
    pub fn as_str(&self) -> &str {
        match self {
            HttpStatus::OK => "200 OK",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::InternalServerError => "500 Internal Server Error",
            HttpStatus::BadRequest => "400 Bad Request",
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: String,
    pub headers: HeaderMap,
    pub body: Option<String>,
}

impl HttpRequest {
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

        let method =
            HttpMethod::from_str(status_line.next().unwrap()).expect("Invalid HTTP method");

        HttpRequest {
            method: method,
            target: status_line.next().unwrap().to_string(),
            version: status_line.next().unwrap().to_string(),
            headers: headers,
            body: Some(body),
        }
    }
}

pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HeaderMap,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(status: HttpStatus, mut headers: HeaderMap, body: Option<String>) -> Self {
        if let Some(b) = &body {
            headers.insert(
                "content-length".to_string(),
                Header {
                    key: "Content-Length".to_string(),
                    value: b.len().to_string(),
                },
            );
        }
        HttpResponse {
            status,
            headers,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("{} {}\r\n", HTTP_VERSION, self.status.as_str());

        self.headers.iter().for_each(|(_, header)| {
            response.push_str(&header.as_str());
            response.push_str("\r\n");
        });
        response.push_str("\r\n");
        if let Some(body) = &self.body {
            response.push_str(body);
        }
        response
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub key: String,
    pub value: String,
}

pub type HeaderMap = HashMap<String, Header>;

impl Header {
    pub fn new(key: String, value: String) -> Self {
        Header { key, value }
    }

    pub fn as_str(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }

    pub fn from_str(header: &str) -> Option<Self> {
        if let Some((key, value)) = header.split_once(':') {
            Some(Header {
                key: key.trim().to_string(),
                value: value.trim().to_string(),
            })
        } else {
            None
        }
    }
}
