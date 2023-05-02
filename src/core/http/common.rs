use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum HttpVersion {
    Unknown(String),    // bad, drop connections in this case
    V0_9(String),       // we don't support this, we'll drop all connections that seem to use v0.9
    V1_0(String),
    V1_1(String),
    V2_0(String),
    V3_0(String),
}

impl HttpVersion {
    pub fn parse(http_v_as_str: &str) -> HttpVersion {
        match http_v_as_str {
            "HTTP/0.9" => HttpVersion::V0_9(String::from(http_v_as_str)),
            "HTTP/1.0" => HttpVersion::V1_0(String::from(http_v_as_str)),
            "HTTP/1.1" => HttpVersion::V1_1(String::from(http_v_as_str)),
            "HTTP/2.0" => HttpVersion::V2_0(String::from(http_v_as_str)),
            "HTTP/3.0" => HttpVersion::V3_0(String::from(http_v_as_str)),
            _ => HttpVersion::Unknown(String::from("HTTP/Unknown"))
        }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Unknown(v) => write!(f, "{}", v),
            HttpVersion::V0_9(v) => write!(f, "{}", v),
            HttpVersion::V1_0(v) => write!(f, "{}", v),
            HttpVersion::V1_1(v) => write!(f, "{}", v),
            HttpVersion::V2_0(v) => write!(f, "{}", v),
            HttpVersion::V3_0(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

impl HttpHeader {
    fn with_int(name: &str, value: i32) -> Self {
        HttpHeader {
            key: name.into(),
            value: value.to_string(),
        }
    }

    fn with_string(name: &str, value: String) -> Self {
        HttpHeader {
            key: name.into(),
            value,
        }
    }

    fn with_str(name: &str, value: &str) -> Self {
        HttpHeader {
            key: name.into(),
            value: value.into(),
        }
    }
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

#[derive(Debug)]
pub struct HttpHeaders {
    pub headers: Vec<HttpHeader>,
}

impl HttpHeaders {
    pub fn new(initial_size: usize) -> HttpHeaders {
        HttpHeaders {
            headers: Vec::with_capacity(initial_size),
        }
    }

    pub fn add_int(mut self, header_name: &str, header_value: i32) -> Self {
        self.headers.push(HttpHeader::with_int(header_name, header_value));
        self
    }

    pub fn add_string(mut self, header_name: &str, header_value: String) -> Self {
        self.headers.push(HttpHeader::with_string(header_name, header_value));
        self
    }

    pub fn add_str(mut self, header_name: &str, header_value: &str) -> Self {
        self.headers.push(HttpHeader::with_str(header_name, header_value));
        self
    }
}

impl Display for HttpHeaders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Add the first header line.
        let mut accumulator = self.headers[0].to_string();

        // Add any remaining headers, one per line.
        for header in &self.headers[1..] {
            accumulator += format!("\n{}", header.to_string()).as_str();
        }

        write!(f, "{}", accumulator)
    }
}

#[derive(Debug)]
pub struct HttpPayload {
    pub value: String,
}

impl Display for HttpPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
