use std::collections::HashMap;

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
