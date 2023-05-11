use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HttpVersion {
    pub major_version: u8,
    pub minor_version: u8,
    pub is_supported: bool,
}

impl HttpVersion {
    const HTTP_V__UNK: HttpVersion = HttpVersion::new(0, 0, false);
    const HTTP_V__0_9: HttpVersion = HttpVersion::new(0, 9, false);
    const HTTP_V__1_0: HttpVersion = HttpVersion::new(1, 0, true);
    const HTTP_V__1_1: HttpVersion = HttpVersion::new(1, 1, false);
    const HTTP_V__2_0: HttpVersion = HttpVersion::new(2, 0, false);
    const HTTP_V__3_0: HttpVersion = HttpVersion::new(3, 0, false);

    const fn new(major_v: u8, minor_v: u8, is_supported: bool) -> Self {
        HttpVersion {
            major_version: major_v,
            minor_version: minor_v,
            is_supported,
        }
    }

    pub fn parse_str(http_v_as_str: &str) -> Result<HttpVersion, String> {
        match http_v_as_str {
            "HTTP/0.9" => Ok(HttpVersion::HTTP_V__0_9),
            "HTTP/1.0" => Ok(HttpVersion::HTTP_V__1_0),
            "HTTP/1.1" => Ok(HttpVersion::HTTP_V__1_1),
            "HTTP/2.0" => Ok(HttpVersion::HTTP_V__2_0),
            "HTTP/3.0" => Ok(HttpVersion::HTTP_V__3_0),
            _ => Err(format!("** Unknown HTTP protocol version: '{}'", http_v_as_str))
        }
    }

    pub fn get_for_version(major_v: u8, minor_v: u8) -> Result<HttpVersion, String> {
        let http_version_as_str = HttpVersion::get_version_format_as_str(major_v, minor_v);

        HttpVersion::parse_str(http_version_as_str.as_str())
    }

    fn get_version_format_as_str(major_v: u8, minor_v: u8) -> String {
        format!("HTTP/{}.{}", major_v, minor_v)
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let http_version_as_str = HttpVersion::get_version_format_as_str(self.major_version, self.minor_version);

        write!(f, "{}", http_version_as_str)
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
