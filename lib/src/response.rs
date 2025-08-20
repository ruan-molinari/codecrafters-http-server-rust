use std::ops::Deref;

use crate::http::{HeaderMap, Status, HTTP_VERSION};

pub struct Response<'r> {
    pub status: Status,
    pub headers: HeaderMap<'r>,
    pub body: Option<&'r str>,
}

impl<'r> Response<'r> {
    pub fn new() -> Self {
        Response {
            status: Status::OK,
            headers: HeaderMap::new(),
            body: None,
        }
    }

    pub fn set_body(&mut self, body: &'r str) {
        self.headers
            .insert("Content-Length", body.len().to_string());
        self.body = Some(body);
    }

    // pub fn new(status: Status, headers: Option<&mut HeaderMap<'r>>, body: Option<&'r str>) -> Self {
    //     let mut headers = headers.unwrap_or(&mut HeaderMap::new());
    //     if let Some(b) = &body {
    //         let header = Header::new("Content-Length", b.len().to_string());
    //         headers.insert(header.name, header.value);
    //     }
    //     Response {
    //         status,
    //         headers: *headers.deref(),
    //         body,
    //     }
    // }

    pub fn to_string(&self) -> String {
        // let mut response = format!("{} {}\r\n", HTTP_VERSION, self.status.as_str());

        let status = self.status.as_str();
        let headers = self.headers.to_string();
        let body = self.body.unwrap_or("");

        format!("{HTTP_VERSION} {status}\r\n{headers}\r\n\r\n{body}")
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
