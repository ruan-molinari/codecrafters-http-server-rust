use crate::http::{Header, HeaderMap, Status, HTTP_VERSION};

pub struct Response {
    pub status: Status,
    pub headers: HeaderMap,
    pub body: Option<String>,
}

impl Response {
    pub fn new(status: Status, mut headers: HeaderMap, body: Option<String>) -> Self {
        if let Some(b) = &body {
            headers.insert(
                "content-length".to_string(),
                Header {
                    key: "Content-Length".to_string(),
                    value: b.len().to_string(),
                },
            );
        }
        Response {
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
