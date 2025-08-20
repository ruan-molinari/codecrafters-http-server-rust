use std::{borrow::Cow, collections::HashMap};

use uncased::{Uncased, UncasedStr};

// TODO: create a better way to handle common headers.
// Some common headers have specifications which can be better handled by declaring them.
//
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers
#[derive(Debug, Clone)]
pub struct Header<'h> {
    pub name: Uncased<'h>,
    pub value: Cow<'h, str>,
}

impl<'h> Header<'h> {
    pub fn new<'a: 'h, 'b: 'h, N, V>(name: N, value: V) -> Header<'h>
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'b, str>>,
    {
        Header {
            name: Uncased::new(name),
            value: value.into(),
        }
    }

    pub fn from_str(header: &'h str) -> Option<Self> {
        if let Some((name, value)) = header.split_once(':') {
            Some(Header {
                name: Uncased::from(name.trim()),
                value: Cow::Borrowed(value.trim()),
            })
        } else {
            None
        }
    }
}

impl Into<String> for Header<'_> {
    fn into(self) -> String {
        format!("{}: {}", self.value, self.value)
    }
}

impl ToString for Header<'_> {
    fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}

pub struct HeaderMap<'h> {
    pub headers: HashMap<Uncased<'h>, Cow<'h, str>>,
}

impl<'h> HeaderMap<'h> {
    pub fn new() -> HeaderMap<'h> {
        HeaderMap {
            headers: HashMap::new(),
        }
    }

    pub fn insert<'a: 'h, 'b: 'h, N, V>(&mut self, name: N, value: V)
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'b, str>>,
    {
        self.headers.insert(Uncased::new(name), value.into());
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.headers.get(UncasedStr::new(name)).map(|v| v.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uncased<'h>, &Cow<'h, str>)> {
        self.headers.iter()
    }

    pub fn to_string(&self) -> String {
        self.headers
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value))
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}
